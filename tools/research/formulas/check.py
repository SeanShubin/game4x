"""The checks, run against `data.json`.

    python tools/research/formulas/check.py

1. **Declared conservation.** The specification says metal is *conserved*. Metal is conserved
   only if the metal bound inside built things counts, so the weights are the Binding column
   of *Units and structures*. This asks whether that weighting is a P-invariant: does every
   recipe leave the total unchanged? It is exact - it cannot raise a false alarm.

2. **Structural unboundedness.** Is there a non-negative firing vector `x`, not all zero, with
   `C.x >= 0` - a set of transitions that, fired in some ratio, ends with more than it began?
   Decided by linear programming over exact rationals. It ignores guards, so it is conservative:
   it can flag a loop the guards prevent and cannot miss one.

3. **A cap.** Applies recipes to a state and reports the first count to pass a declared bound.
   A backstop for bugs in 1 and 2, never the mechanism.

**Every check is poisoned before it is believed** - see `self_test`. A checker that cannot be
made to fail on demand is a checker whose green means nothing, and every one here is green on
purpose in the ordinary case.
"""

import json
import pathlib
import re
import sys
from fractions import Fraction

HERE = pathlib.Path(__file__).parent
DATA = json.loads((HERE / "data.json").read_text(encoding="utf-8"))

# The Binding column of *Units and structures*: the metal locked inside a built thing.
# These are the weights of the claimed P-invariant, copied from the release, not invented.
METAL_WEIGHT = {
    "metal": 1,
    "garrison": 1,
    "extractor": 1,
    "store": 1,
    "yard": 15,
    "ark": 3,
    "pioneer": 3,
}

# `each` has no effect of its own: it says how many transitions this recipe is a source of,
# and the lines below it are the transition. `spec/invariants.md` - *a rule is a source of
# transitions, not a kind of one* - so the body is what checks 1 and 2 must weigh, and the
# quantifier is the count of firings rather than part of one. Weighing the whole would make a
# recipe's net effect depend on the size of the planet, which is not a fixed vector at all.
# `each` and `some` are the same field with different quantifiers - the world's recipes fire
# for every member of a set, the player's are offered for one. Neither has an effect of its
# own: they say how many transitions the recipe is a source of, and the lines below them are
# the transition. So checks 1 and 2 weigh the body, which is `spec/invariants.md`'s own
# sentence - a rule is a source of transitions, not a kind of one.
NON_EFFECT_OPS = {"let", "require", "each", "some"}

# What the Kinds table declares. metal is "conserved"; energy is "neither conserved nor
# expiring"; food "expires"; a citizen grows on surplus. So a loop that gains energy, food,
# citizens or labor is the game, and a loop that gains metal is a bug. Check 2 uses these
# rather than a separate list of intended loops - one declaration, two checks.
DECLARED_FREE = {"energy", "food", "citizen", "labor"}

# **Extraction is a declared source and check 1 was wrong to test against its absence.**
# `work` on a metal extractor mines up to 8 metal out of the ground, so "metal is conserved"
# cannot mean globally conserved - no arrangement of the other recipes could make it true.
# What the specification can mean is: conserved OUTSIDE extraction. Sean's objection is what
# found this; the check was testing a stronger claim than the words can carry.
DECLARED_SOURCES = {"work"}

# There is no declared sink. `disorder` was one for an hour, because it deleted a unit and
# returned nothing where `perish` returns a thing's metal. Merging them removed the exception:
# one recipe cannot disagree with itself.
DECLARED_SINKS = set()

# A family in a target hides a kind. `resource[extractor.resource]` collapses to the family
# `resource`, which has no metal weight, so working a metal extractor scored zero. A check
# that cannot see the game's only metal source is a check aimed at the wrong subject.
FAMILIES = {"resource": ("food", "metal", "energy"), "unit": ("ark", "pioneer")}

# Every family the data declares, including `place` and `thing`. Kept apart from FAMILIES
# above, which grounds transitions - expanding `place` there would ground every recipe over
# territory and orbit, which is a different question from what may hold what.
ALL_FAMILIES = {row[0]: tuple(k.strip() for k in row[1].split(",")) for row in DATA["families"]}


def called_names(data):
    """Recipes that are only ever reached through a call.

    A called recipe is not a transition. Counting `found-colony` as one lets it fire on its
    own, with no ark and no pioneer spent - which reports a metal source that no player can
    reach. Found by Sean pushing back on check 1, and it was overstating.
    """
    called = set()
    for f in data["player"] + data["world"]:
        for op, target, _a, _at, _n in f["lines"]:
            if op == "call":
                called.add(target.split("(")[0].strip())
    return called


def place_of(target):
    """The kind a target names, ignoring where it is.

    Aggregating over locations is the right over-approximation for a structural question:
    a loop that moves metal between territories is not a loop that creates metal.
    """
    t = target.split(" in ")[0].strip()
    t = t.split("[")[0].strip()
    return t


def amount_of(raw):
    """A signed integer amount, or None when the recipe's amount depends on the state.

    Amounts carry their own sign since 2026-09-09: `create` and `consume` are one `change`,
    and which direction it goes is in the number rather than in the operator.
    """
    try:
        return int(str(raw).strip())
    except (TypeError, ValueError):
        return None


def sign_of(raw):
    """The direction of a change, for the amounts an integer cannot be read from."""
    return -1 if str(raw).strip().startswith("-") else 1


def effects(recipe, recipes_by_name, seen=(), bounded=False, binding=None):
    """Net effect per kind, or None if any amount is state-dependent.

    Calls are inlined. A cycle would be an unbounded decomposition, which the acyclicity
    rule forbids, so meeting one is a defect rather than a case to handle.
    """
    net = {}
    for op, target, raw, _attach, _note in recipe["lines"]:
        if op in NON_EFFECT_OPS:
            continue
        if op == "set":
            continue
        if op == "call":
            name = target.split("(")[0].strip()
            if name in seen:
                raise ValueError("recipe call cycle at %r" % name)
            sub = recipes_by_name.get(name)
            if sub is None:
                return None
            inner = effects(sub, recipes_by_name, seen + (name,), bounded, binding)
            if inner is None:
                return None
            for k, v in inner.items():
                net[k] = net.get(k, 0) + v
            continue
        n = amount_of(raw)
        k = place_of(target)
        # **Grounding, and it is the same operation the menu needs.** A target naming a
        # family stands for one transition per kind in it. Leaving it a family is what made
        # the check blind to mining: `resource` has no metal weight, so working a metal
        # extractor scored zero. Found by Sean asking what a check that cannot see its own
        # subject is worth.
        if binding and k in binding:
            k = binding[k]
        if n is None:
            if not bounded:
                return None
            b = DATA.get("bounds", {}).get(recipe["name"].split("(")[0].strip(), {}).get(k)
            if b is None:
                return None
            n = sign_of(raw) * b[0]
        net[k] = net.get(k, 0) + n
    return net


def families_in(recipe):
    """Which families a recipe's targets name, in a stable order."""
    seen = []
    for op, target, _a, _at, _n in recipe["lines"]:
        if op in NON_EFFECT_OPS or op in ("set", "call"):
            continue
        k = place_of(target)
        if k in FAMILIES and k not in seen:
            seen.append(k)
    return seen


def ground(name, recipe):
    """One (name, binding) per instantiation of the families this recipe mentions.

    A recipe naming no family grounds to itself, so this is the identity in the ordinary
    case and only the three that name one expand.
    """
    fams = families_in(recipe)
    if not fams:
        return [(name, None)]
    out = [(name, {})]
    for fam in fams:
        nxt = []
        for nm, b in out:
            for kind in FAMILIES[fam]:
                nxt.append((f"{nm}[{kind}]", dict(b, **{fam: kind})))
        out = nxt
    return out


def gather():
    by_name = {}
    for f in DATA["player"] + DATA["world"]:
        by_name[f["name"].split("(")[0].strip()] = f
    return by_name


def check_conservation():
    """Check 1. Metal conserved OUTSIDE declared extraction. Exact: a violation is a violation."""
    by_name = gather()
    only_called = called_names(DATA)
    analysed, skipped, bad = [], [], []
    for name, f in sorted(by_name.items()):
        if name in DECLARED_SOURCES or name in DECLARED_SINKS:
            continue
        # The same fix check 2 needed: a recipe that is only ever called is not fired on its
        # own, so weighing it alone reports a source no player can reach. `found-colony`
        # delivers 3 and its callers each spend a unit worth 3; only the pair is a real event.
        if name in only_called:
            continue
        # **Ground the families first.** Check 2 learned this when `work` produced the family
        # `resource`, which carries no metal weight, and the game's only metal source scored
        # zero. Check 1 had the same hole and nothing exposed it until `disorder` destroyed a
        # `unit`: a unit has no weight, an ark and a pioneer weigh 3, and the recipe scored 0.
        # A family hiding a kind, the fourth time this repository has recorded it.
        for gname, binding in ground(name, f):
            net = effects(f, by_name, binding=binding)
            if net is None:
                if gname not in skipped:
                    skipped.append(gname)
                continue
            total = sum(METAL_WEIGHT.get(k, 0) * v for k, v in net.items())
            analysed.append(gname)
            if total != 0:
                parts = [
                    f"{'+' if v > 0 else ''}{v} {k} (x{METAL_WEIGHT[k]})"
                    for k, v in sorted(net.items())
                    if METAL_WEIGHT.get(k)
                ]
                bad.append((gname, total, ", ".join(parts)))
    return analysed, skipped, bad


def simplex(A, b, c):
    """Maximize c.x subject to A.x <= b, x >= 0, with b >= 0 so the origin is feasible.

    Exact rationals and Bland's rule, because this problem is massively degenerate - most
    of `b` is zero - and Bland's rule is what makes termination certain rather than likely.
    Returns (optimum, x).
    """
    m, n = len(A), len(c)
    T = [[Fraction(A[i][j]) for j in range(n)] + [Fraction(1) if k == i else Fraction(0) for k in range(m)] + [Fraction(b[i])] for i in range(m)]
    T.append([Fraction(-x) for x in c] + [Fraction(0)] * m + [Fraction(0)])
    basis = list(range(n, n + m))
    while True:
        piv_col = -1
        for j in range(n + m):
            if T[-1][j] < 0:
                piv_col = j
                break
        if piv_col < 0:
            break
        piv_row, best = -1, None
        for i in range(m):
            if T[i][piv_col] > 0:
                ratio = T[i][-1] / T[i][piv_col]
                if best is None or ratio < best or (ratio == best and basis[i] < basis[piv_row]):
                    best, piv_row = ratio, i
        if piv_row < 0:
            return None, None  # unbounded objective; cannot happen with x <= 1
        pv = T[piv_row][piv_col]
        T[piv_row] = [v / pv for v in T[piv_row]]
        for i in range(m + 1):
            if i != piv_row and T[i][piv_col] != 0:
                f = T[i][piv_col]
                T[i] = [T[i][j] - f * T[piv_row][j] for j in range(n + m + 1)]
        basis[piv_row] = piv_col
    x = [Fraction(0)] * n
    for i in range(m):
        if basis[i] < n:
            x[basis[i]] = T[i][-1]
    return T[-1][-1], x


def check_unbounded(extra=None, exclude_sources=False):
    """Check 2. Conservative: it ignores guards, so a hit is a candidate and a miss is a proof."""
    by_name = gather()
    if extra:
        by_name = dict(by_name, **extra)
    only_called = called_names(DATA)
    names, nets, skipped = [], [], []
    for name, f in sorted(by_name.items()):
        if name in only_called and not extra:
            continue  # inlined into its callers; not a transition of its own
        if exclude_sources and name in DECLARED_SOURCES:
            continue
        for gname, binding in ground(name, f):
            net = effects(f, by_name, bounded=True, binding=binding)
            if net is None:
                if name not in skipped:
                    skipped.append(name)
                continue
            names.append(gname)
            nets.append(net)
    kinds = sorted({k for net in nets for k in net})
    hidden = sorted({k for k in kinds if k in FAMILIES})
    # maximise sum(x) subject to -C.x <= 0 and x <= 1
    A, b = [], []
    for k in kinds:
        A.append([-net.get(k, 0) for net in nets])
        b.append(0)
    for j in range(len(names)):
        A.append([1 if i == j else 0 for i in range(len(names))])
        b.append(1)
    opt, x = simplex(A, b, [1] * len(names))
    witness = []
    if opt and opt > 0:
        for name, net, v in zip(names, nets, x):
            # A transition whose net effect is entirely zero rides along for free: the LP is
            # indifferent to it, so it lands in the answer at whatever value fits. Naming it
            # costs a reader time and tells them nothing, so the witness carries only
            # transitions that actually move something.
            if v > 0 and any(net.values()):
                witness.append((name, v))
    gain = {}
    for k in kinds:
        g = sum(net.get(k, 0) * v for net, v in zip(nets, x or []))
        if g > 0:
            gain[k] = g
    # Split the gain by what the specification declares. A gain in a declared-free kind is
    # the game; a gain in metal-equivalent is the bug; a gain in a kind nobody declared is a
    # question for whoever added it.
    metal_gain = sum(METAL_WEIGHT.get(k, 0) * v for k, v in gain.items())
    free = {k: v for k, v in gain.items() if k in DECLARED_FREE}
    undeclared = {k: v for k, v in gain.items()
                  if k not in DECLARED_FREE and k not in METAL_WEIGHT}
    return names, skipped, witness, gain, metal_gain, free, undeclared, hidden


def check_attachment():
    """Check 4. An attachment is only observable where the line can actually fail.

    A `create` fails when its container will not hold another. So an attachment on a create
    whose target has **no capacity** decides nothing: hard and soft behave identically and
    always will. Naming the question does not make it one.

    Returns (meaningful, unobservable, no_capacity_kinds).
    """
    bounded = {}
    for container, holds, upto, _note in DATA.get("capacities", []):
        base = holds.split("[")[0].strip()
        bounded[base] = (container, upto)
    meaningful, unobservable = [], []
    for f in DATA["player"] + DATA["world"]:
        for op, target, _amt, attach, _n in f["lines"]:
            if op != "change" or sign_of(_amt) < 0 or not attach:
                continue
            k = place_of(target)
            row = (f["name"], target, attach)
            if k in bounded and "unbounded" not in bounded[k][1]:
                meaningful.append(row + (bounded[k][1],))
            else:
                unobservable.append(row + (bounded.get(k, ("", "no capacity declared"))[1],))
    return meaningful, unobservable


RESOURCE_COLUMN = {"food": 1, "metal": 2, "energy": 3}


def check_placement():
    """Check 5. Which territories would refuse a create, per line.

    **Aimed at the resource the line actually names.** An earlier version of this report said
    territory 7 mattered because it has no energy - and no recipe here creates an energy
    extractor, so it never did. Listing territories that lack *any* resource answers a wider
    question than the one asked, which is the failure this repository keeps recording.
    """
    out = []
    for f in DATA["player"] + DATA["world"]:
        for op, target, _amt, attach, _n in f["lines"]:
            if op != "change" or sign_of(_amt) < 0 or "[" not in target:
                continue
            kind = target.split("[")[0].strip()
            res = target.split("[", 1)[1].split("]")[0].strip()
            col = RESOURCE_COLUMN.get(res)
            if kind != "extractor" or col is None:
                continue
            blocked = [row[0] for row in DATA["territories"]
                       if row[col].strip().lower() == "none"]
            out.append((f["name"], target, attach, blocked))
    return out


def container_of(target):
    """The name a target gives for what will hold the thing, or None if it names none."""
    return target.split(" in ", 1)[1].strip() if " in " in str(target) else None


def check_containment():
    """Check 6. Every positive change names a container that declares a bound for that kind.

    `spec/logistics.md`: every thing is in the game, directly or through what contains it, and
    a kind that declares no capacity contains nothing and never can. So a recipe that creates
    a thing somewhere nothing allows it is a defect, and one that creates it nowhere at all is
    a worse one.

    Negative changes are not tested: they act on something that already exists, so where it is
    is a fact about the state rather than a claim by the recipe.
    """
    allowed = {(c, k) for c, k, _b, _s in DATA["containment_declared"]}
    names = DATA["container_kind"]
    examined, homeless, undeclared = 0, [], []
    for f in DATA["player"] + DATA["world"] + DATA["creation"]:
        for op, target, amount, _attach, _note in f["lines"]:
            if op != "change" or sign_of(amount) < 0:
                continue
            if "." in str(target) and " in " not in str(target):
                continue  # a numeric trait, not a thing being put anywhere
            examined += 1
            where = container_of(target)
            if where is None:
                homeless.append((f["name"], target))
                continue
            container = names.get(where)
            if container is None:
                homeless.append((f["name"], f"{target} - no kind known for {where!r}"))
                continue
            kind = place_of(target)
            if (container, kind) not in allowed:
                undeclared.append((f["name"], target, container, kind))
    return examined, homeless, undeclared


RELEASE = pathlib.Path(__file__).parents[3] / "releases" / "first-release.md"


def release_traits():
    """The release's *Traits* table, parsed from the document itself."""
    rows, inside = {}, False
    for line in RELEASE.read_text(encoding="utf-8").splitlines():
        if line.startswith("| Trait "):
            inside = True
            continue
        if inside:
            if not line.startswith("|"):
                break
            cells = [c.strip() for c in line.strip("|").split("|")]
            if set(cells[0]) <= set("- "):
                continue
            name = cells[0].strip("*").strip().replace(" ", "-")
            rows[name] = "derived" if cells[3].startswith("derived") else "stored"
    return rows


def check_drift():
    """Check 7. Where the copy and the release disagree. Reports, never fails."""
    theirs = release_traits()
    assert theirs, "parsed no traits from the release, so this check knows nothing"
    ours = {r[0]: r[2] for r in DATA["traits"]}
    only_ours = sorted(set(ours) - set(theirs))
    only_theirs = sorted(set(theirs) - set(ours))
    differ = sorted(k for k in set(ours) & set(theirs) if ours[k] != theirs[k])
    return len(theirs), only_ours, only_theirs, differ


def check_unreached():
    """Check 8. Declared (container, kind) pairs that no recipe reaches.

    Families are grounded before comparing, so `move` putting a `unit` in a place counts as
    reaching both `ark` and `pioneer` rather than neither. Without that this reported a
    pioneer-in-orbit gap that was an artefact of the check rather than a fact about the game.
    """
    names = DATA["container_kind"]
    used = set()
    for f in DATA["player"] + DATA["world"] + DATA["creation"]:
        for op, target, amount, _at, _n in f["lines"]:
            # Only a POSITIVE change reaches a container. Counting a consume as well is how
            # this check first missed that nothing fills a unit's tank: `move` spends energy
            # from it, which looked like use, and no recipe ever puts any in.
            if op != "change" or sign_of(amount) < 0:
                continue
            where = container_of(target)
            if where is None:
                continue
            container = names.get(where)
            if container is None:
                continue
            kind = place_of(target)
            # Both sides expand, and by the data's own families rather than the grounding map
            # above - that one has no `place`, so a unit moved into a place reached neither an
            # orbit nor a territory and the check reported two gaps it had invented.
            for c in ALL_FAMILIES.get(container, (container,)):
                for k in ALL_FAMILIES.get(kind, (kind,)):
                    used.add((c, k))
    declared = [(c, k, b) for c, k, b, _s in DATA["containment_declared"]]
    assert declared, "no declarations, so this check knows nothing"
    # "unreached" now means nothing ever puts one there, which is the question worth asking.
    unreached = []
    for c, k, b in declared:
        # A declaration about a family is reached when any of its kinds is.
        pairs = {(cc, kk) for cc in ALL_FAMILIES.get(c, (c,))
                 for kk in ALL_FAMILIES.get(k, (k,))}
        if not (pairs & used):
            unreached.append((c, k, b))
    return len(declared), unreached


MINIMAL_CONTROL = 2  # two citizens, each eating one food per turn


def check_minimal_control():
    """Check 9. Which territories can feed the two citizens minimal control needs.

    Reads the *Territory resources* row rather than any prose about it: `capacity x density`,
    and it is the density that decides, because one citizen's labor works one extractor.
    """
    can, cannot = [], []
    for row in DATA["territories"]:
        tid, food = row[0], row[1].strip()
        if food == "none":
            cannot.append((tid, "no food at all"))
            continue
        capacity, density = (int(p.strip()) for p in food.split("x"))
        assert capacity >= 1, (tid, food)
        if density >= MINIMAL_CONTROL:
            can.append((tid, density))
        else:
            cannot.append((tid, f"food density {density}, and two citizens eat {MINIMAL_CONTROL}"))
    assert can or cannot, "no territories, so this check knows nothing"

    # The other half: force. A citizen carries 1 and a garrison carries 0, so minimal control
    # is worth exactly MINIMAL_CONTROL. Nature is per biome, and no territory has a biome
    # stated - `X-19` - so the only answerable question is against the worst of them.
    force = {k: v for k, v, _n in DATA["force_values"]}
    per_citizen = int(force["citizen"])
    assert int(force["garrison"].strip("<strong>/")) == 0, force["garrison"]
    worst = max(int(v.strip("<strong>/")) for _b, v in DATA["nature_values"] if v.strip("<strong>/").isdigit())
    return can, cannot, MINIMAL_CONTROL * per_citizen, worst


# The shape of a form rather than anything typed into it.
# The notation's own words. An editor offers the expression *form* and these come with it;
# `available` joined on 2026-09-10 with `available <kind> of x`, which `spec/logistics.md`
# defines and `refuel` is the first recipe to need.
SYNTAX = {"in", "of", "at", "least", "available"}


def selectable_sets():
    """Every finite set an editor could offer as a menu."""
    return (
        {r[0] for r in DATA["primitives"]} | {"let"}
        | {r[0] for r in DATA["attach_values"] if r[0] != "(blank)"}
        | {r[0] for r in DATA["kinds"]} | {r[0] for r in DATA["families"]}
        | {r[0] for r in DATA["traits"]}
        | {b.lower() for b, _v in DATA["nature_values"]}
        | {"count", "sum", "max", "min"} | {">", ">=", "<", "<=", "="}
        # Every value a trait may take, declared once so a menu can offer it.
        | {v.strip() for r in DATA["value_sets"] for v in r[1].split(",")}
        | {"yes", "no", "any", "design", "play", "food", "metal", "energy"}
        | {"holder", "border"}  # `holder of x`, and the border a unit crosses
        | SYNTAX
        | {f["name"].split("(")[0].strip()
           for s in ("player", "world", "creation") for f in DATA[s]}
    )


def check_editor():
    """Check 10. Could an editor build every recipe, with names the only thing typed?

    Sean's test, and it is a question about how finite the game is. Every token in every line
    is matched against the sets an editor would offer; a name a recipe binds counts, because a
    name is the one thing he conceded has to be typed. What is left over is what could not be
    chosen from any menu.
    """
    import re

    offered = selectable_sets()
    per, tokens, ok = {}, 0, 0
    for sec in ("player", "world", "creation"):
        for f in DATA[sec]:
            names = (set(re.findall(r"[a-z]\w*", f["name"].split("(")[1].rstrip(")")))
                     if "(" in f["name"] else set())
            for op, target, *_ in f["lines"]:
                if op in ("each", "some") and ":" in str(target):
                    names |= set(re.findall(r"[a-z]\w*", str(target).split(":")[0]))
                if op == "let" and "=" in str(target):
                    names |= set(re.findall(r"[a-z]\w*", str(target).split("=")[0]))
            names |= {"t", "unit", "thing", "citizen", "extractor", "ark", "pioneer", "yard"}
            for op, target, amount, attach, _n in f["lines"]:
                for field in (op, str(target), str(amount), str(attach)):
                    for tok in re.findall(r"[A-Za-z][\w-]*|\d+|[<>=]+", field):
                        tokens += 1
                        if (tok in offered or tok.lower() in offered
                                or tok in names or tok.isdigit()):
                            ok += 1
                        else:
                            per.setdefault(f["name"], set()).add(tok)
    n = sum(len(DATA[s]) for s in ("player", "world", "creation"))
    assert tokens, "no tokens, so this check knows nothing"
    return tokens, ok, n, per


def force_in(present):
    """The force a territory presents, from `force_rule` in the data rather than from here.

    `spec/control.md` -> *Coordination*: coordination is imposed by a structure such as a
    garrison, or by a military unit, which carries it. So the rule is one condition and two
    aggregators, and stating it once is what lets a poison move it. Check 11 computed it five
    times by hand before 2026-09-10, which is five places for it to drift.
    """
    force = {k: int(v.replace("<strong>", "").replace("</strong>", ""))
             for k, v, _n in DATA["force_values"]}
    carried = [force[k] for k, n in present.items() if k in force for _ in range(n)]
    if not carried:
        return 0
    organized = any(present.get(k, 0) for k in DATA["force_rule"]["organizers"])
    return sum(carried) if organized else max(carried)


def check_capture():
    """Check 11. The sequence Sean described, arithmetic checked at every step.

    *Once two pioneers breach the region, one of them can deploy and make a garrison with two
    citizens, at which point the other pioneer can leave.* Every step names what is standing
    there and asks `force_in`; the aggregator is never chosen here.

    **Breaching needs greater than nature and staying needs equal**, which is the two
    comparisons `unsustained_causes` declares - so the operator per step comes from that table
    rather than from a literal, and a change to the declared cause moves this check.
    """
    hardest = max(int(n) for _t, _b, n in DATA["territory_biomes"])
    jungles = [t for t, b, _n in DATA["territory_biomes"] if b == "jungle"]

    # The declared causes, so the operators are read rather than typed. A cause fires when the
    # force falls short, so what holds is its negation: `not <=` is `>`, and `not <` is `>=`.
    causes = " ".join(row[1] for row in DATA["unsustained_causes"])
    assert "&le;" in causes and "&lt;" in causes, "the declared causes name neither comparison"
    ENTER, STAY = ">", ">="

    scenes = [
        ("two pioneers breach", {"pioneer": 2}, ENTER),
        ("one pioneer alone breaches", {"pioneer": 1}, ENTER),
        ("after one deploys, with the other still there",
         {"pioneer": 1, "garrison": 1, "citizen": 2}, STAY),
        ("the other pioneer leaves; garrison organizes", {"garrison": 1, "citizen": 2}, STAY),
        ("the same two citizens with no garrison", {"citizen": 2}, STAY),
    ]
    steps = []
    for what, present, op in scenes:
        have = force_in(present)
        steps.append((what, have, op, hardest,
                      have > hardest if op == ENTER else have >= hardest))

    assert len(steps) == len(scenes), "a scene was dropped"
    return jungles, hardest, steps


def check_yard():
    """Check 14. Where can a yard be built, and how long does it take?

    **This is `X-21`'s arithmetic redone after `X-21` was refuted.** The old answer was *six of
    twelve*, and it rested on nothing storing, so a yard's metal had to be mined in one turn.
    **Metal carries**: `end-of-turn losses` keeps as much as the territory's stores hold, and a
    territory may have **as many stores of a resource as it has extractors of it**.

    Nothing here is typed. The cost comes from the recipe, the store's capacity from the data,
    and each territory's metal from the `n x d` cell. **Citizens are not modelled** - the
    per-turn figure assumes every extractor is worked, which needs one citizen's labor each, and
    that is what the note beside each row carries.
    """
    yard = next(r for r in DATA["player"] if r["name"] == "build yard")
    cost = -sum(int(a) for op, tgt, a, _at, _n in yard["lines"]
                if op == "change" and "metal" in tgt and str(a).startswith("-"))
    assert cost > 0, "read no metal cost from build yard, so this check knows nothing"
    cap = DATA["store_capacity"]

    rows = []
    for tid, _food, metal, _energy, _note in DATA["territories"]:
        if "x" not in metal:
            rows.append((tid, 0, 0, 0, None))
            continue
        n, d = (int(x.strip()) for x in metal.split("x"))
        store, per_turn = cap * n, n * d
        turns = None
        if store + per_turn >= cost:
            held, turns = 0, 1
            while held + per_turn < cost:
                held = min(held + per_turn, store)   # what a turn's end keeps
                turns += 1
                assert turns <= 99, "accumulation did not converge"
        rows.append((tid, per_turn, store, store + per_turn, turns))

    assert len(rows) == len(DATA["territories"]), "a territory was dropped"
    return cost, cap, rows


CRATE = RELEASE.parent.parent / "crates" / "game-model" / "src"


def check_behaviours():
    """Check 15. Behaviours the running game has, and which recipe names each one.

    **The specification lane's generalisation of `X-21`.** Four were found by hand in one day -
    storing, greater force to enter, equal force to maintain, and what losing a territory does -
    and four is enough to suspect more.

    **The anchor is a line of the code, not a line number.** Each row names text that must occur
    **exactly once** in its file; the check goes red when a behaviour is rewritten or removed,
    and stays quiet when something above it moves. A line number would cry wolf on every edit
    and an absent anchor would pass silently, which is `CLAUDE.md`'s own warning about a check
    that outlives its example.

    **What this check cannot do is decide whether a rule ought to be a recipe.** The
    classification is data - `code_behaviours` in `data.json` - and this only refuses to keep
    agreeing about code that has changed under it.
    """
    known = set(release_recipes())
    assert known, "read no recipes from the release, so this check knows nothing"

    rows, missing = [], []
    for src, anchor, what, recipes, note in DATA["code_behaviours"]:
        text = (CRATE / src).read_text(encoding="utf-8")
        # **The game, not its tests.** `game.rs:1618` reimplements the end of a turn backwards
        # to prove the settling order cannot matter, so three anchors occur twice - and a
        # behaviour that lived only in a test would be a rule nothing runs.
        body = text.split("#[cfg(test)]")[0]
        assert len(body) < len(text) or "#[cfg(test)]" not in text
        hits = [i for i, line in enumerate(body.splitlines(), 1) if anchor in line]
        if len(hits) != 1:
            missing.append((src, anchor, len(hits)))
            continue
        if recipes:
            for r in recipes:
                assert r in known, f"{r} is not one of the release's recipes"
        rows.append((f"{src}:{hits[0]}", what, recipes, note))

    assert rows, "no behaviour anchored, so this check knows nothing"
    return rows, missing


READS_A_MARKING = ("count {", "sum ", "max ", "available ")


def lets_of(recipe):
    """Every `let` binding in a recipe, so an arc's weight can be resolved through it."""
    out = {}
    for op, tgt, _a, _at, _n in recipe["lines"]:
        if op == "let" and "=" in str(tgt):
            name, expr = (p.strip() for p in str(tgt).split("=", 1))
            out[name] = expr
    return out


def arc_kind(amount, lets):
    """`ordinary`, `trait` or `marking` - the Petri-net arc kind of one weight.

    **Resolving through the `let` matters.** `grow`'s weights are `-n` and `+n`, and its `let` is
    `min(count {food surplus:yes in t}, count {citizen in t})`, so they read two markings. A
    classifier that stopped at the weight scored them ordinary, which is the whole reason this
    reads the bindings.
    """
    a = str(amount).strip()
    for name, expr in lets.items():
        if re.search(r"[+-]?" + chr(92) + "b" + re.escape(name) + chr(92) + "b", a):
            a = a.replace(name, expr)
    if any(w in a for w in READS_A_MARKING):
        return "marking"
    if re.fullmatch(r"[+-]?[0-9]+", a):
        return "ordinary"
    return "trait"


def check_arcs():
    """Check 16. Every arc by its Petri-net kind, and the two unfoldings of the coloured net.

    **Places are the declared `(container, kind)` pairs, transitions are recipes, colours are
    families, and grounding is unfolding** - so this check is the arc half of a correspondence
    the rest of the report names. Two things it asserts that nothing else does:

    - **Every arc whose weight reads a marking is a reset or transfer arc**, which an ordinary
      net has none of. All of them are in **world** recipes today, so the sublanguage a player
      writes in is an ordinary coloured net - which is what `spec/invariants.md` needs when it
      says a player's rules always finish.
    - **`soft` is a second unfolding.** A soft arc is skipped rather than failing, so a recipe
      carrying three of them is eight transitions. `ground()` never sees them, because check 2
      ignores guards by design.
    """
    per, kinds, by_group = {}, {}, {}
    by_colour = by_both = 0
    unfold = []
    for grp in ("player", "world", "creation"):
        for f in DATA[grp]:
            lets = lets_of(f)
            for i, (op, _tgt, amt, attach, _n) in enumerate(f["lines"]):
                if op != "change":
                    continue
                k = arc_kind(amt, lets)
                per[f"{f['name']}|{i}"] = k
                kinds[k] = kinds.get(k, 0) + 1
                if k == "marking":
                    by_group.setdefault(k, []).append((grp, f["name"], i + 1))
            if grp == "creation":
                continue
            colours = len(ground(f["name"].split("(")[0].strip(), f))
            soft = sum(1 for _o, _t, _a, at, _n in f["lines"]
                       if str(at).strip().lower() == "soft")
            by_colour += colours
            by_both += colours * 2 ** soft
            if colours > 1 or soft:
                unfold.append((f["name"], sorted(families_in(f)), colours, soft,
                               colours * 2 ** soft))
    assert kinds, "no arcs read, so this check knows nothing"
    assert by_both >= by_colour, "softness cannot shrink the net"
    return kinds, by_group.get("marking", []), unfold, by_colour, by_both, per


def release_recipes():
    """The release's *Recipes* table: {recipe: {(role, kind): qty}}."""
    out, current, inside = {}, None, False
    for line in RELEASE.read_text(encoding="utf-8").splitlines():
        if line.startswith("| Recipe "):
            inside = True
            continue
        if not inside:
            continue
        if not line.startswith("|"):
            break
        cells = [c.strip() for c in line.strip("|").split("|")]
        # A continuation row has an EMPTY first cell and carries the recipe above it. The
        # first version of this test read an empty cell as a separator, so it kept only each
        # recipe's first row and reported every other one as added - a check that cried wolf
        # about eighteen rows, of which one was real.
        if len(cells) < 6:
            continue
        if cells[0] and set(cells[0]) <= set("-"):
            continue
        name = cells[0].strip("*").strip()
        if name:
            current = name
            out.setdefault(current, {})
        if current is None:
            continue
        role, qty, kind = cells[2], cells[3], cells[4].strip("`")
        if role in ("require", "consume", "produce", "limit") and kind:
            out[current][(role, kind)] = qty
    return out


def ours_by_effect():
    """This lane's recipes as {recipe: {(role, kind): qty}}, in the release's vocabulary."""
    out = {}
    for f in DATA["player"] + DATA["world"]:
        name = f["name"].split("(")[0].strip()
        rows = {}
        for op, target, amount, _at, _n in f["lines"]:
            kind = place_of(target)
            if op == "change":
                role = "consume" if sign_of(amount) < 0 else "produce"
                rows[(role, kind)] = str(amount).lstrip("+-")
            elif op == "require":
                inner = re.search(r"\{(\w[\w-]*)", str(target))
                if inner:
                    rows[("require", inner.group(1))] = str(amount)
        out[name] = rows
    return out


def check_recipe_drift():
    """Check 12. Where this lane's recipes and the release's disagree, by (role, kind)."""
    theirs, ours = release_recipes(), ours_by_effect()
    assert theirs, "parsed no recipes from the release, so this check knows nothing"
    rows = []
    for name in sorted(set(theirs) | set(ours)):
        a, b = theirs.get(name), ours.get(name)
        if a is None:
            rows.append((name, "ONLY HERE", ""))
            continue
        if b is None:
            rows.append((name, "ONLY IN THE RELEASE", ""))
            continue
        for key in sorted(set(a) | set(b)):
            if key not in b:
                rows.append((name, "dropped", f"{key[0]} {a[key]} {key[1]}"))
            elif key not in a:
                rows.append((name, "ADDED", f"{key[0]} {b[key]} {key[1]}"))
    return len(theirs), rows


SPEC = RELEASE.parent.parent / "spec"


def check_kinds_defined():
    """Check 13. Which kinds the release has that `spec/` never defines.

    `CLAUDE.md`: *a release spec never invents a rule; if a release needs one the spec lacks,
    propose it into the spec first.* A kind is a rule of that sort.

    **Searched by concept rather than by word.** Each kind declares the terms that would show
    the idea is in `spec/` at all - `store` also looks for storage, silo, warehouse, granary
    and stockpile - because a bare word search returns a plausible zero and invites no
    question. The terms are in `data.json` so the search can be judged rather than trusted.
    """
    files = sorted(SPEC.glob("*.md"))
    lines = [(p.name, i + 1, l) for p in files
             for i, l in enumerate(p.read_text(encoding="utf-8").split(chr(10)))]
    text = " ".join(l for _f, _i, l in lines).lower()
    assert text and len(files) > 1, "read no specification, so this check knows nothing"
    kinds = [k for k, _w in DATA["kinds"]]
    named, concept_only, absent = [], [], []
    for kind in kinds:
        terms = DATA["spec_concepts"][kind]
        if re.search(r"\b" + re.escape(kind) + r"\b", text):
            named.append(kind)
        elif any(term in text for term in terms[1:]):
            # **Every match, with where it is.** Reporting only the first hides a false
            # positive behind it, which is what `node` did: `spec/interface.md` uses it for a
            # tree node in the rule editor, and the real hit is in `spec/logistics.md`.
            where = [(term, f, i, l.strip()) for term in terms[1:]
                     for f, i, l in lines if re.search(rf"\b{re.escape(term)}\b", l.lower())]
            concept_only.append((kind, where))
        else:
            absent.append(kind)
    assert len(named) + len(concept_only) + len(absent) == len(kinds)
    return len(kinds), named, concept_only, absent


def check_cap(cap=1000):
    """Check 3. A backstop: apply every recipe once and report anything past the cap."""
    by_name = gather()
    state, breached = {}, []
    for name, f in sorted(by_name.items()):
        net = effects(f, by_name)
        if net is None:
            continue
        for k, v in net.items():
            state[k] = state.get(k, 0) + max(0, v)
            if state[k] > cap:
                breached.append((name, k, state[k]))
    return cap, breached


def self_test():
    """Poison every check before believing any of it.

    The poison is aimed OUTSIDE the region each check already covers where it can be: the
    conservation poison creates metal from nothing, which no real recipe does, and the
    unboundedness poison is a recipe with no inputs at all.
    """
    ok = True
    poison = {
        "POISON-free-metal": {
            "name": "POISON-free-metal",
            "selector": "poison",
            "lines": [["change", "metal in t", "+1", "", "poison"]],
        }
    }
    w = check_unbounded(extra=poison)[2]
    if not any(n == "POISON-free-metal" for n, _ in w):
        print("  POISON FAILED: check 2 did not flag a recipe that creates metal from nothing")
        ok = False
    else:
        print("  poison ok: check 2 flags a recipe with no inputs")

    saved = METAL_WEIGHT.get("citizen")
    METAL_WEIGHT["citizen"] = 1
    _, _, bad = check_conservation()
    if not bad:
        print("  POISON FAILED: check 1 stayed green with citizens weighted as metal")
        ok = False
    else:
        print(f"  poison ok: check 1 goes red on a wrong weighting ({len(bad)} recipes)")
    if saved is None:
        del METAL_WEIGHT["citizen"]
    else:
        METAL_WEIGHT["citizen"] = saved

    # The poison aimed at the thing Sean just changed: put the two stores back and check 1
    # must go red for exactly that reason. A check that stayed green here would have been
    # agreeing with the fix rather than measuring it.
    fc = next(f for f in DATA["player"] if f["name"].startswith("found-colony"))
    fc["lines"] += [["change", "store[food] in t", "+1", "", "poison"],
                    ["change", "store[metal] in t", "+1", "", "poison"]]
    _, _, bad2 = check_conservation()
    names2 = {n for n, _t, _p in bad2}
    if {"deploy ark", "found by land"} <= names2:
        print("  poison ok: putting the two stores back turns deploy ark and found by land red")
    else:
        print("  POISON FAILED: the two stores went back and check 1 stayed green")
        ok = False
    fc["lines"] = [l for l in fc["lines"] if l[4] != "poison"]

    _, breached = check_cap(cap=0)
    if not breached:
        print("  POISON FAILED: check 3 stayed green with a cap of 0")
        ok = False
    else:
        print("  poison ok: check 3 goes red at a cap of 0")
    # Check 6 poison: put a garrison in an orbit, which nothing declares.
    saved = [list(r) for r in DATA["containment_declared"]]
    fc = dict(DATA["player"][0])
    DATA["player"] = [dict(f) for f in DATA["player"]]
    victim = DATA["player"][0]
    victim["lines"] = list(victim["lines"]) + [
        ["change", "garrison in {orbit below:t}", "+1", "", "poison"]]
    _e, _h, bad6 = check_containment()
    victim["lines"] = victim["lines"][:-1]
    if not any(k == "garrison" and c == "orbit" for _n, _t, c, k in bad6):
        print("  POISON FAILED: check 6 did not flag a garrison put in an orbit")
        ok = False
    else:
        print("  poison ok: check 6 flags a thing put where nothing declares it")
    DATA["containment_declared"] = saved

    # Check 6 against X-20: take back the four rows Sean's answer added and it must go red.
    # This is the evidence that the check has teeth - it would have caught the contradiction
    # that this lane's own model carried for a day.
    saved = list(DATA["containment_declared"])
    DATA["containment_declared"] = [
        r for r in saved
        # `resource` goes too: it is the row that lets `work` put what it mines in a
        # territory, and X-20 was about resources rather than about three named kinds.
        if not (r[0] == "territory"
                and r[1] in ("food", "metal", "energy", "labor", "resource"))
    ]
    _e2, _h2, bad20 = check_containment()
    DATA["containment_declared"] = saved
    caught = {k for _n, _t, c, k in bad20 if c == "territory"}
    # `food` is absent on purpose: no recipe puts food in a territory by name - `work` makes
    # the family `resource`, and that is the row that carries it.
    if not {"metal", "labor", "resource"} <= caught:
        print(f"  POISON FAILED: check 6 would not have caught X-20 (caught {sorted(caught)})")
        ok = False
    else:
        print(f"  poison ok: check 6 goes red on X-20's state, over {len(bad20)} lines")

    # Check 7 poison: claim a stored trait is derived and it must notice.
    _t = {r[0]: r[2] for r in DATA["traits"]}
    _row = next(r for r in DATA["traits"] if r[0] == "ready")
    _row[2] = "derived"
    if "ready" not in check_drift()[3]:
        print("  POISON FAILED: check 7 did not notice a copy that disagrees with the release")
        ok = False
    else:
        print("  poison ok: check 7 notices a copy that disagrees with the release")
    _row[2] = _t["ready"]

    # Check 8 poison: declare a pairing nothing could reach, and it must be named.
    _saved8 = list(DATA["containment_declared"])
    DATA["containment_declared"] = _saved8 + [["store", "yard", "1", "poison"]]
    if not any(c == "store" and k == "yard" for c, k, _b in check_unreached()[1]):
        print("  POISON FAILED: check 8 did not name a declaration nothing reaches")
        ok = False
    else:
        print("  poison ok: check 8 names a declaration nothing reaches")
    DATA["containment_declared"] = _saved8

    # Check 9 poison: two citizens eating three would leave more territories behind.
    _m = MINIMAL_CONTROL
    globals()["MINIMAL_CONTROL"] = 3
    _harder = len(check_minimal_control()[1])
    globals()["MINIMAL_CONTROL"] = _m
    if _harder <= len(check_minimal_control()[1]):
        print("  POISON FAILED: check 9 is not reading the density it claims to")
        ok = False
    else:
        print(f"  poison ok: check 9 excludes {_harder} territories when a citizen eats more")

    # Check 10 poison: an unknown word in a target must show up as something typed.
    _v = DATA["player"][0]
    _saved10 = [list(l) for l in _v["lines"]]
    _v["lines"] = _saved10 + [["change", "flibbertigibbet in t", "+1", "", "poison"]]
    if "flibbertigibbet" not in "".join(
            ",".join(s) for s in check_editor()[3].values()):
        print("  POISON FAILED: check 10 did not notice a word no menu could offer")
        ok = False
    else:
        print("  poison ok: check 10 notices a word no menu could offer")
    _v["lines"] = _saved10

    # Check 11 poison: take the organizers away and the garrison stops organizing, so two
    # citizens present `max(1, 1)` rather than 2 and a jungle can no longer be held. This
    # replaced a poison that discarded its own result - `if check_capture()[2][4][4]: pass` -
    # and so could never have failed.
    _org = DATA["force_rule"]["organizers"]
    DATA["force_rule"]["organizers"] = []
    if check_capture()[2][3][4]:
        print("  POISON FAILED: check 11 holds a jungle with nothing organizing the citizens")
        ok = False
    else:
        print("  poison ok: check 11 stops holding when nothing organizes force")
    DATA["force_rule"]["organizers"] = _org
    _c = next(r for r in DATA["force_values"] if r[0] == "citizen")
    _cw = _c[1]
    _c[1] = "0"
    if check_capture()[2][3][4]:
        print("  POISON FAILED: check 11 holds a jungle with citizens worth nothing")
        ok = False
    else:
        print("  poison ok: check 11 fails to hold when a citizen carries no force")
    _c[1] = _cw

    # Check 12 poison: put back the ark that launch ark must not produce.
    _la = next(r for r in DATA["player"] if r["name"] == "launch ark")
    _saved12 = [list(l) for l in _la["lines"]]
    # A produced **yard**, which nothing makes. This used to add a produced ark, and `P-362`
    # landed that row on 2026-09-10 - so the poison agreed with the release and stopped
    # detecting anything. A poison can go blind by being overtaken.
    _la["lines"] = _saved12 + [["change", "yard in t", "+1", "", "poison"]]
    if not any(w == "ADDED" and n == "launch ark" for n, w, _d in check_recipe_drift()[1]):
        print("  POISON FAILED: check 12 did not notice a produced ark the release does not have")
        ok = False
    else:
        print("  poison ok: check 12 notices a row the release does not have")
    _la["lines"] = _saved12

    # Check 13 poison: a kind spec/ has never heard of must come back absent.
    _k = DATA["kinds"]
    DATA["kinds"] = _k + [["flibbertigibbet", "poison"]]
    DATA["spec_concepts"]["flibbertigibbet"] = ["flibbertigibbet"]
    if "flibbertigibbet" not in check_kinds_defined()[3]:
        print("  POISON FAILED: check 13 found a kind spec/ has never heard of")
        ok = False
    else:
        print("  poison ok: check 13 reports a kind spec/ does not define")
    DATA["kinds"] = _k
    del DATA["spec_concepts"]["flibbertigibbet"]

    # Check 14 poison: take the stores away and the answer must fall back to what could be
    # mined in a single turn - which is the six the page reported while `X-21` stood.
    _cap = DATA["store_capacity"]
    DATA["store_capacity"] = 0
    _fell = sum(1 for _t, _p, _s, _r, turns in check_yard()[2] if turns is not None)
    DATA["store_capacity"] = _cap
    _now = sum(1 for _t, _p, _s, _r, turns in check_yard()[2] if turns is not None)
    if _fell >= _now:
        print("  POISON FAILED: check 14 builds as many yards with no store as with one")
        ok = False
    else:
        print(f"  poison ok: check 14 falls from {_now} territories to {_fell} with no stores")

    # Check 15 poison: an anchor that is not in the code must be reported, not passed over.
    _saved15 = [list(r) for r in DATA["code_behaviours"]]
    DATA["code_behaviours"] = _saved15 + [
        ["game.rs", "fn nothing_of_the_sort(", "poison", None, ""]]
    if not check_behaviours()[1]:
        print("  POISON FAILED: check 15 anchored a behaviour the code does not have")
        ok = False
    else:
        print("  poison ok: check 15 reports an anchor that is no longer in the code")
    DATA["code_behaviours"] = _saved15

    # Check 16 poison, one per claim. Make a marking-reading weight constant and the count of
    # reset arcs must fall; add a soft arc and the second unfolding must grow.
    _eot = next(f for f in DATA["world"] if f["name"] == "end-of-turn losses")
    _was16 = [list(l) for l in _eot["lines"]]
    _before = check_arcs()[0].get("marking", 0)
    for _l in _eot["lines"]:
        if "count {" in str(_l[2]):
            _l[2] = "-1"
    if check_arcs()[0].get("marking", 0) >= _before:
        print("  POISON FAILED: check 16 counts a reset arc that is now a constant")
        ok = False
    else:
        print("  poison ok: check 16 stops counting a reset arc when the weight is a constant")
    _eot["lines"] = _was16

    _wk = next(f for f in DATA["player"] if f["name"] == "work")
    _wasw = [list(l) for l in _wk["lines"]]
    _b4 = check_arcs()[4]
    _wk["lines"] = [list(l) for l in _wk["lines"]]
    _wk["lines"][-1][3] = "soft"
    if check_arcs()[4] <= _b4:
        print("  POISON FAILED: check 16 did not notice a soft arc doubling the transitions")
        ok = False
    else:
        print("  poison ok: check 16 grows the unfolded net when an arc becomes soft")
    _wk["lines"] = _wasw

    return ok


def as_dict():
    """The three checks' results, for the renderer.

    The report shows what this returns rather than a transcription of it, so the page and
    the checker cannot disagree.
    """
    analysed, skipped, bad = check_conservation()
    names, skipped2, witness, gain, metal_gain, free, undeclared, hidden = check_unbounded()
    n2, _s2, w2, g2, mg2, f2, u2, _h2 = check_unbounded(exclude_sources=True)
    cap, breached = check_cap()
    c6_examined, c6_homeless, c6_undeclared = check_containment()
    # How many lines the check flags in the state the release still describes - the evidence
    # that it has X-20's teeth, recorded rather than left only in the poison output.
    _saved = list(DATA["containment_declared"])
    DATA["containment_declared"] = [
        r for r in _saved
        if not (r[0] == "territory"
                and r[1] in ("food", "metal", "energy", "labor", "resource"))
    ]
    x20_lines = len(check_containment()[2])
    DATA["containment_declared"] = _saved
    cost14, cap14, rows14 = check_yard()
    rows15, missing15 = check_behaviours()
    kinds16, marking16, unfold16, colour16, both16, per16 = check_arcs()
    return {
        "arcs": {"kinds": kinds16, "marking": marking16, "unfold": unfold16,
                 "by_colour": colour16, "by_both": both16, "per_line": per16},
        "behaviours": {"rows": [[a, b, c, d] for a, b, c, d in rows15],
                       "missing": [[a, b, c] for a, b, c in missing15]},
        "yard": {"cost": cost14, "store": cap14,
                 "rows": [[a, b, c, d, e] for a, b, c, d, e in rows14]},
        "conservation": {
            "analysed": len(analysed), "skipped": skipped,
            "weights": dict(METAL_WEIGHT),
            "violations": [[n, t, parts] for n, t, parts in bad],
        },
        "unbounded": {
            "analysed": len(names), "skipped": skipped2,
            "witness": [[n, str(v)] for n, v in witness],
            "gain": {k: str(v) for k, v in sorted(gain.items())},
            "metal_gain": str(metal_gain),
            "free": sorted(free), "undeclared": sorted(undeclared),
            "hidden": hidden,
            # The verdict needs the breakdown, not just the metal figure. Without it the
            # renderer had a hand-typed "clean" beside a checker saying UNBOUNDED - true,
            # for the reason the prose gave, and not derived from anything.
            "without_sources": {"transitions": len(n2),
                                "witness": [[n, str(v)] for n, v in w2],
                                "gain": {k: str(v) for k, v in sorted(g2.items())},
                                "metal_gain": str(mg2),
                                "free": sorted(f2), "undeclared": sorted(u2)},
        },
        "cap": {"cap": cap, "breaches": [[n, k, c] for n, k, c in breached]},
        "editor": {"tokens": check_editor()[0], "selectable": check_editor()[1],
                   "recipes": check_editor()[2], "clean": check_editor()[2] - len(check_editor()[3]),
                   "typed": {k: sorted(v) for k, v in check_editor()[3].items()}},
        "unreached": {"declared": check_unreached()[0],
                      "pairs": [list(r) for r in check_unreached()[1]]},
        "containment": {"examined": c6_examined,
                        "declared": len(DATA["containment_declared"]),
                        "homeless": [[n, t] for n, t in c6_homeless],
                        "undeclared": [list(r) for r in c6_undeclared],
                        "x20_lines": x20_lines},
        "placement": [[n, tg, at, b] for n, tg, at, b in check_placement()],
        "attachment": {"meaningful": [list(r) for r in check_attachment()[0]],
                       "unobservable": [list(r) for r in check_attachment()[1]]},
        "poison": ["check 2 flags a recipe with no inputs",
                   "check 1 goes red on a wrong weighting",
                   "check 3 goes red at a cap of 0"],
    }


def main():
    if "--json" in sys.argv:
        json.dump(as_dict(), sys.stdout, indent=2)
        return 0
    print("Poisoning the checks before trusting them")
    poisoned = self_test()
    print()

    analysed, skipped, bad = check_conservation()
    print("CHECK 1 - metal conserved OUTSIDE extraction, weighted by Binding")
    print(f"  declared sources, excluded: {', '.join(sorted(DECLARED_SOURCES))}")
    if DECLARED_SINKS:
        print(f"  declared sinks, excluded: {', '.join(sorted(DECLARED_SINKS))}")
    else:
        print("  no declared sinks - every recipe that destroys a thing leaves its metal behind")
    print(f"  {len(analysed)} recipes analysed, {len(skipped)} skipped for state-dependent amounts")
    if skipped:
        print(f"  skipped: {', '.join(skipped)}")
    if bad:
        for name, total, parts in bad:
            print(f"  VIOLATION  {name}: net {total:+d} metal-equivalent   [{parts}]")
    else:
        print("  no violation")
    print()

    names, skipped2, witness, gain, metal_gain, free, undeclared, hidden = check_unbounded()
    n2, _s2, w2, g2, mg2, _f2, _u2, _h2 = check_unbounded(exclude_sources=True)
    print("CHECK 2 - structurally unbounded")
    print(f"  {len(names)} transitions in the matrix, {len(skipped2)} skipped for state-dependent amounts")
    if witness:
        print("  UNBOUNDED. A witness, as a firing ratio:")
        for n, v in witness:
            print(f"    {v}  x  {n}")
        print(f"  net gain: {', '.join(f'{k} +{v}' for k, v in sorted(gain.items()))}")
        print(f"  metal-equivalent gain: {metal_gain}   <- the number that matters")
        if free:
            print(f"  declared free, so not a defect: {', '.join(sorted(free))}")
        if undeclared:
            print(f"  UNDECLARED - nobody said whether these may grow: {', '.join(sorted(undeclared))}")
    else:
        print("  no non-negative firing vector gains anything - structurally bounded")
    if hidden:
        print(f"  A FAMILY STILL HIDES A KIND: {', '.join(hidden)}")
    print()

    print(f"CHECK 2b - the same, with the declared sources removed ({', '.join(sorted(DECLARED_SOURCES))})")
    print(f"  {len(n2)} transitions, and this is the question worth asking")
    if w2:
        print("  UNBOUNDED WITHOUT MINING. Witness:")
        for n, v in w2:
            print(f"    {v}  x  {n}")
        print(f"  metal-equivalent gain: {mg2}   <- a metal source that is not extraction")
    else:
        print("  no loop gains metal without mining")
    print()

    print("CHECK 5 - which territories would refuse a create, per line")
    for name, target, attach, blocked in check_placement():
        where = ("none - every territory has room" if not blocked
                 else "territory " + ", ".join(blocked))
        verdict = "" if not blocked else ("  <- HARD would refuse founding there"
                                          if attach.upper() != "SOFT" else "  (soft, so it skips)")
        print(f"  {name:16} {target:24} [{attach or '-'}]  blocked on: {where}{verdict}")
    print()

    meaningful, unobservable = check_attachment()
    print("CHECK 4 - is an attachment observable?")
    print(f"  {len(meaningful)} create(s) whose container can refuse, so hard and soft differ:")
    for name, target, attach, upto in meaningful:
        print(f"    {name:16} {target:28} [{attach}]  capacity {upto}")
    if unobservable:
        print(f"  {len(unobservable)} where the container never refuses, so the attachment decides nothing:")
        for name, target, attach, upto in unobservable:
            print(f"    {name:16} {target:28} [{attach}]  {upto}")
    print()

    cap, breached = check_cap()
    print(f"CHECK 3 - cap of {cap}")
    print("  " + (f"{len(breached)} breach(es)" if breached else "no breach applying each recipe once"))
    print()
    examined, homeless, undeclared = check_containment()
    print()
    print("CHECK 6 - is everything created put somewhere that declares it?")
    print(f"  {examined} positive changes examined, over "
          f"{len(DATA['containment_declared'])} declared (container, kind) pairs")
    assert examined, "check 6 examined nothing, so its green means nothing"
    for name, target in homeless:
        print(f"  NO CONTAINER: {name}: {target}")
    for name, target, container, kind in undeclared:
        print(f"  UNDECLARED: {name}: {target} - nothing says a {container} may hold a {kind}")
    if not homeless and not undeclared:
        print("  every one lands somewhere that declares it")

    n_rel, only_ours, only_theirs, differ = check_drift()
    print()
    print("CHECK 7 - where this lane's copy differs from the release")
    print(f"  {n_rel} traits parsed from releases/first-release.md, {len(DATA['traits'])} copied here")
    for k in differ:
        ours = {r[0]: r[2] for r in DATA["traits"]}
        print(f"  DIFFERS: {k} - release says {release_traits()[k]}, this copy says {ours[k]}")
    for k in only_ours:
        print(f"  ONLY HERE: {k}")
    for k in only_theirs:
        print(f"  ONLY IN THE RELEASE: {k}")
    if not (differ or only_ours or only_theirs):
        print("  the copy matches, so nothing here has diverged yet")
    print("  divergence is allowed and is the reason for copying - this reports it, never fails it")

    n_decl, unreached = check_unreached()
    print()
    print("CHECK 8 - is every declaration reachable? (the reverse of check 6)")
    print(f"  {n_decl - len(unreached)} of {n_decl} declared pairs are reached by some recipe")
    for c, k, b in unreached:
        print(f"  UNREACHED: a {c} may hold a {k} (bound {b}) - no recipe puts one there")
    if not unreached:
        print("  every declaration is reachable")

    can, cannot, held, worst = check_minimal_control()
    print()
    print("CHECK 9 - which territories can sustain minimal control?")
    print(f"  minimal control is 1 garrison, 1 food extractor, {MINIMAL_CONTROL} citizens")
    print(f"  {len(can)} of {len(can) + len(cannot)} territories can feed it from one worked extractor")
    for tid, why in cannot:
        print(f"  CANNOT: territory {tid} - {why}")
    print(f"  force: {MINIMAL_CONTROL} citizens carry {held}; the worst force of nature is {worst}")
    if held < worst:
        print(f"  SHORT by {worst - held} against the worst biome")
    elif held == worst:
        print("  exactly sufficient, with no margin - and no recipe compares the two")

    tok, ok, n_rec, per = check_editor()
    print()
    print("CHECK 10 - could an editor build these by choosing, with only names typed?")
    print(f"  {ok} of {tok} tokens are selectable from a finite set ({100*ok//tok}%)")
    print(f"  {n_rec - len(per)} of {n_rec} recipes could be built with nothing typed but names")
    for name in sorted(per):
        print(f"  TYPED: {name} - {', '.join(sorted(per[name]))}")

    jungles, hardest, steps = check_capture()
    print()
    print("CHECK 11 - does the jungle capture sequence work?")
    print(f"  jungles are territories {', '.join(jungles)}, force of nature {hardest}")
    for what, have, op, need, ok in steps:
        print(f"  {'yes' if ok else 'NO ':<4} {what:<44} {have} {op} {need}")

    n_rel_r, drift = check_recipe_drift()
    print()
    print("CHECK 12 - where this lane's recipes differ from the release's, row by row")
    print(f"  {n_rel_r} recipes parsed from releases/first-release.md")
    for name, what, detail in drift:
        print(f"  {what:<20} {name:<18} {detail}")
    if not drift:
        print("  no recipe differs from the release")
    print("  divergence is allowed - this reports it, never fails it")

    n_kinds13, named13, concept13, absent13 = check_kinds_defined()
    print()
    print("CHECK 13 - which kinds does the release have that spec/ never defines?")
    print(f"  {len(named13)} of {n_kinds13} kinds are named in spec/ by their own word")
    for kind, where in concept13:
        print(f"  CONCEPT ONLY: {kind} - spec/ never names the kind. Every match:")
        for term, f, i, line in where:
            print(f"      {f}:{i}  \"{term}\"  {line[:74]}")
    for kind in absent13:
        print(f"  ABSENT: {kind} - no trace of the concept in spec/ at all")

    cost14, cap14, rows14 = check_yard()
    print()
    print("CHECK 14 - where can a yard be built, now that metal carries?")
    print(f"  a yard costs {cost14} metal; a store holds {cap14}, and a territory may have one "
          f"per extractor of that resource")
    for tid, per_turn, store, reach, turns in rows14:
        verdict = f"{turns} turn(s)" if turns else "NEVER"
        print(f"  territory {tid:<3} {per_turn:>3}/turn  store {store:>3}  reach {reach:>3}  {verdict}")
    can = [t for t, _p, _s, _r, n in rows14 if n]
    one = [t for t, _p, _s, _r, n in rows14 if n == 1]
    print(f"  {len(can)} of {len(rows14)} territories can build a yard; "
          f"{len(one)} of them in a single turn, which was the whole answer while X-21 stood")

    rows15, missing15 = check_behaviours()
    print()
    print("CHECK 15 - behaviours the running game has, and which recipe names each")
    for where, what, recipes, _note in rows15:
        named = ", ".join(recipes) if recipes else "** NO RECIPE **"
        print(f"  {where:<22} {named:<24} {what}")
    for src, anchor, n in missing15:
        print(f"  ANCHOR GONE: {src} has {n} occurrences of {anchor!r} - reclassify it")
    unnamed = [r for r in rows15 if not r[2]]
    print(f"  {len(rows15)} behaviours anchored in the code; "
          f"**{len(unnamed)} are named by no recipe in the release's sixteen**")

    kinds16, marking16, unfold16, colour16, both16, _per16 = check_arcs()
    print()
    print("CHECK 16 - every arc by its Petri-net kind, and the two unfoldings")
    for k, v in sorted(kinds16.items(), key=lambda kv: -kv[1]):
        print(f"  {v:4}  {k}")
    for grp, name, i in marking16:
        print(f"    reads a marking: {grp}/{name} line {i}")
    groups = sorted({g for g, _n, _i in marking16})
    print(f"  every marking-reading arc is in: {', '.join(groups)}")
    for name, fams, colours, soft, total in unfold16:
        print(f"    {name:<22} colours {colours:<3} soft {soft:<3} -> {total:<4} "
              f"({', '.join(fams) or 'no family'})")
    print(f"  {colour16} transitions by colour, {both16} by colour and softness together")

    print("Every green above means nothing unless the poison at the top went red.")
    return 0 if poisoned else 1


if __name__ == "__main__":
    sys.exit(main())
