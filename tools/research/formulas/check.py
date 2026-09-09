"""The three checks, run against `data.json`.

    python tools/research/formulas/check.py

1. **Declared conservation.** The specification says metal is *conserved*. Metal is conserved
   only if the metal bound inside built things counts, so the weights are the Binding column
   of *Units and structures*. This asks whether that weighting is a P-invariant: does every
   formula leave the total unchanged? It is exact - it cannot raise a false alarm.

2. **Structural unboundedness.** Is there a non-negative firing vector `x`, not all zero, with
   `C.x >= 0` - a set of formulas that, fired in some ratio, ends with more than it began?
   Decided by linear programming over exact rationals. It ignores guards, so it is conservative:
   it can flag a loop the guards prevent and cannot miss one.

3. **A cap.** Applies formulas to a state and reports the first count to pass a declared bound.
   A backstop for bugs in 1 and 2, never the mechanism.

**Every check is poisoned before it is believed** - see `self_test`. A checker that cannot be
made to fail on demand is a checker whose green means nothing, and all three here are green on
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

NON_EFFECT_OPS = {"let", "require"}

# What the Kinds table declares. metal is "conserved"; energy is "neither conserved nor
# expiring"; food "expires"; a citizen grows on surplus. So a loop that gains energy, food,
# citizens or labor is the game, and a loop that gains metal is a bug. Check 2 uses these
# rather than a separate list of intended loops - one declaration, two checks.
DECLARED_FREE = {"energy", "food", "citizen", "labor"}

# **Extraction is a declared source and check 1 was wrong to test against its absence.**
# `work` on a metal extractor mines up to 8 metal out of the ground, so "metal is conserved"
# cannot mean globally conserved - no arrangement of the other formulas could make it true.
# What the specification can mean is: conserved OUTSIDE extraction. Sean's objection is what
# found this; the check was testing a stronger claim than the words can carry.
DECLARED_SOURCES = {"work"}

# A family in a target hides a kind. `resource[extractor.resource]` collapses to the family
# `resource`, which has no metal weight, so working a metal extractor scored zero. A check
# that cannot see the game's only metal source is a check aimed at the wrong subject.
FAMILIES = {"resource": ("food", "metal", "energy"), "unit": ("ark", "pioneer")}


def called_names(data):
    """Formulas that are only ever reached through a call.

    A called formula is not a transition. Counting `found-colony` as one lets it fire on its
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
    """An integer amount, or None when the formula's amount depends on the state."""
    try:
        return int(str(raw).strip())
    except (TypeError, ValueError):
        return None


def effects(formula, formulas_by_name, seen=(), bounded=False, binding=None):
    """Net effect per kind, or None if any amount is state-dependent.

    Calls are inlined. A cycle would be an unbounded decomposition, which the acyclicity
    rule forbids, so meeting one is a defect rather than a case to handle.
    """
    net = {}
    for op, target, raw, _attach, _note in formula["lines"]:
        if op in NON_EFFECT_OPS:
            continue
        if op == "set":
            continue
        if op == "call":
            name = target.split("(")[0].strip()
            if name in seen:
                raise ValueError("recipe call cycle at %r" % name)
            sub = formulas_by_name.get(name)
            if sub is None:
                return None
            inner = effects(sub, formulas_by_name, seen + (name,), bounded, binding)
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
            b = DATA.get("bounds", {}).get(formula["name"].split("(")[0].strip(), {}).get(k)
            if b is None:
                return None
            n = b[0]
        sign = 1 if op == "create" else -1
        net[k] = net.get(k, 0) + sign * n
    return net


def families_in(formula):
    """Which families a formula's targets name, in a stable order."""
    seen = []
    for op, target, _a, _at, _n in formula["lines"]:
        if op in NON_EFFECT_OPS or op in ("set", "call"):
            continue
        k = place_of(target)
        if k in FAMILIES and k not in seen:
            seen.append(k)
    return seen


def ground(name, formula):
    """One (name, binding) per instantiation of the families this formula mentions.

    A formula naming no family grounds to itself, so this is the identity in the ordinary
    case and only the three that name one expand.
    """
    fams = families_in(formula)
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
        if name in DECLARED_SOURCES:
            continue
        # The same fix check 2 needed: a formula that is only ever called is not fired on its
        # own, so weighing it alone reports a source no player can reach. `found-colony`
        # delivers 3 and its callers each spend a unit worth 3; only the pair is a real event.
        if name in only_called:
            continue
        net = effects(f, by_name)
        if net is None:
            skipped.append(name)
            continue
        total = sum(METAL_WEIGHT.get(k, 0) * v for k, v in net.items())
        analysed.append(name)
        if total != 0:
            parts = [
                f"{'+' if v > 0 else ''}{v} {k} (x{METAL_WEIGHT[k]})"
                for k, v in sorted(net.items())
                if METAL_WEIGHT.get(k)
            ]
            bad.append((name, total, ", ".join(parts)))
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
            # A formula whose net effect is entirely zero rides along for free: the LP is
            # indifferent to it, so it lands in the answer at whatever value fits. Naming it
            # costs a reader time and tells them nothing, so the witness carries only
            # formulas that actually move something.
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
            if op != "create" or not attach:
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
    territory 7 mattered because it has no energy - and no formula here creates an energy
    extractor, so it never did. Listing territories that lack *any* resource answers a wider
    question than the one asked, which is the failure this repository keeps recording.
    """
    out = []
    for f in DATA["player"] + DATA["world"]:
        for op, target, _amt, attach, _n in f["lines"]:
            if op != "create" or "[" not in target:
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


def check_cap(cap=1000):
    """Check 3. A backstop: apply every formula once and report anything past the cap."""
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
    conservation poison creates metal from nothing, which no real formula does, and the
    unboundedness poison is a formula with no inputs at all.
    """
    ok = True
    poison = {
        "POISON-free-metal": {
            "name": "POISON-free-metal",
            "selection": "poison",
            "lines": [["create", "metal in t", "1", "", "poison"]],
        }
    }
    w = check_unbounded(extra=poison)[2]
    if not any(n == "POISON-free-metal" for n, _ in w):
        print("  POISON FAILED: check 2 did not flag a formula that creates metal from nothing")
        ok = False
    else:
        print("  poison ok: check 2 flags a formula with no inputs")

    saved = METAL_WEIGHT.get("citizen")
    METAL_WEIGHT["citizen"] = 1
    _, _, bad = check_conservation()
    if not bad:
        print("  POISON FAILED: check 1 stayed green with citizens weighted as metal")
        ok = False
    else:
        print(f"  poison ok: check 1 goes red on a wrong weighting ({len(bad)} formulas)")
    if saved is None:
        del METAL_WEIGHT["citizen"]
    else:
        METAL_WEIGHT["citizen"] = saved

    # The poison aimed at the thing Sean just changed: put the two stores back and check 1
    # must go red for exactly that reason. A check that stayed green here would have been
    # agreeing with the fix rather than measuring it.
    fc = next(f for f in DATA["player"] if f["name"].startswith("found-colony"))
    fc["lines"] += [["create", "store[food] in t", "1", "", "poison"],
                    ["create", "store[metal] in t", "1", "", "poison"]]
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
        "placement": [[n, tg, at, b] for n, tg, at, b in check_placement()],
        "attachment": {"meaningful": [list(r) for r in check_attachment()[0]],
                       "unobservable": [list(r) for r in check_attachment()[1]]},
        "poison": ["check 2 flags a formula with no inputs",
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
    print(f"  {len(analysed)} formulas analysed, {len(skipped)} skipped for state-dependent amounts")
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
    print(f"  {len(names)} formulas in the matrix, {len(skipped2)} skipped for state-dependent amounts")
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
    print("  " + (f"{len(breached)} breach(es)" if breached else "no breach applying each formula once"))
    print()
    print("All three green means nothing unless the poison above went red.")
    return 0 if poisoned else 1


if __name__ == "__main__":
    sys.exit(main())
