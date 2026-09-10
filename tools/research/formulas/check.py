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
SYNTAX = {"in", "of", "at", "least"}


def selectable_sets():
    """Every finite set an editor could offer as a menu."""
    return (
        {r[0] for r in DATA["primitives"]} | {"let"}
        | {r[0] for r in DATA["attach_values"] if r[0] != "(blank)"}
        | {r[0] for r in DATA["kinds"]} | {r[0] for r in DATA["families"]}
        | {r[0] for r in DATA["traits"]}
        | {b.lower() for b, _v in DATA["nature_values"]}
        | {"count", "sum", "max", "min"} | {">", ">=", "<", "<=", "="}
        | {"yes", "no", "any", "design", "play", "food", "metal", "energy"}
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
            "selection": "poison",
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
    victim["lines"] = list(victim["lines"]) + [["change", "garrison in t.orbit", "+1", "", "poison"]]
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
    return {
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

    print("Every green above means nothing unless the poison at the top went red.")
    return 0 if poisoned else 1


if __name__ == "__main__":
    sys.exit(main())
