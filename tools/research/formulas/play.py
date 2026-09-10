"""Play the loop, from an ark in orbit to a new ark in orbit.

    python tools/research/formulas/play.py

Every other check on this page reasons *about* the recipes. This one fires them at a state and
reports what happened, which is the only way to answer `R-6` - *the loop can be played through* -
without a person doing it by hand.

**It is not the game.** It is this lane's own evaluator over this lane's own data, deliberately
small: counts per container, the six primitives, and no interface. What it can say is whether a
sequence of firings gets from an ark to a new ark, and what the state was at every step.
"""

import json
import pathlib
import re
import sys

HERE = pathlib.Path(__file__).parent
DATA = json.loads((HERE / "data.json").read_text(encoding="utf-8"))

NATURE = {t: int(n) for t, _b, n in DATA["territory_biomes"]}
RESOURCES = {}
for row in DATA["territories"]:
    for i, res in enumerate(("food", "metal", "energy"), start=1):
        cell = row[i].strip()
        RESOURCES[(row[0], res)] = (0, 0) if cell == "none" else tuple(
            int(x.strip()) for x in cell.split("x"))
FORCE = {k: int(v.replace("<strong>", "").replace("</strong>", ""))
         for k, v, _n in DATA["force_values"]}
BINDING = {"metal": 1, "garrison": 1, "extractor": 1, "store": 1,
           "yard": 15, "ark": 3, "pioneer": 3}


class World:
    """Counts of a kind in a place. A place is a territory id, or ('orbit', id)."""

    def __init__(self):
        self.at = {}
        self.log = []

    def n(self, place, kind, resource=None):
        return self.at.get((place, kind, resource), 0)

    def add(self, place, kind, amount, resource=None):
        key = (place, kind, resource)
        now = self.at.get(key, 0) + amount
        assert now >= 0, f"{kind} in {place} would go to {now}"
        if now:
            self.at[key] = now
        else:
            self.at.pop(key, None)

    def force(self, place):
        """Units always sum. Citizens sum where a garrison organizes them, else max."""
        units = sum(self.n(place, k) * FORCE[k] for k in ("ark", "pioneer"))
        citizens = self.n(place, "citizen")
        if not citizens:
            return units
        organized = self.n(place, "garrison") > 0
        return units + (citizens * FORCE["citizen"] if organized else FORCE["citizen"])

    def metal_equivalent(self):
        return sum(n * BINDING.get(kind, 0) for (_p, kind, _r), n in self.at.items())

    def say(self, text):
        self.log.append(text)


def play(verbose=True):
    w = World()
    w.add(("orbit", "1"), "ark", 1)
    w.say("start: one ark in orbit above territory 1")

    # 1. Deploy the ark. found-colony: a garrison, two citizens, two extractors.
    w.add(("orbit", "1"), "ark", -1)
    for kind, res in (("garrison", None), ("extractor", "food"), ("extractor", "metal")):
        w.add("1", kind, 1, res)
    w.add("1", "citizen", 2)
    w.say("deploy ark onto territory 1: garrison, 2 citizens, a food and a metal extractor")

    def work_turn(t, want):
        """Work extractors for one resource, bounded by capacity, density and labor."""
        cap, den = RESOURCES[(t, want)]
        have = min(w.n(t, "extractor", want), cap, w.n(t, "citizen"))
        w.add(t, want, have * den)
        return have * den

    def turn(t):
        """One turn in one territory: work everything, eat, then grow on the surplus.

        `grow` was missing from the first version of this and the run stopped dead: producing
        a pioneer costs two citizens and a colony starts with exactly two, so the first pioneer
        emptied the colony and nothing could work an extractor afterwards. The population has to
        grow before anything can be spent - which is the loop, and the runner was not playing it.
        """
        for res in ("food", "metal", "energy"):
            if w.n(t, "extractor", res):
                work_turn(t, res)
        eaten = min(w.n(t, "food"), w.n(t, "citizen"))
        w.add(t, "food", -eaten)
        starved = w.n(t, "citizen") - eaten
        if starved > 0:
            w.add(t, "citizen", -starved)   # unsustained: upkeep unmet
        grown = min(w.n(t, "food"), w.n(t, "citizen"))   # surplus food, and citizens
        if grown:
            w.add(t, "food", -grown)
            w.add(t, "citizen", grown)
        w.add(t, "food", -w.n(t, "food"))   # what no store holds is lost at the turn's end
        return grown

    # 2. Build up territory 1 until it can afford a pioneer: 3 metal, 6 energy, 2 citizens.
    w.add("1", "extractor", 1, "energy") if RESOURCES[("1", "energy")][0] else None
    for _ in range(6):
        turn("1")
    w.say(f"six turns on territory 1: {w.n('1','metal')} metal, {w.n('1','energy')} energy")

    # 3. Two pioneers, to breach a jungle. Territory 6 is jungle, nature 2.
    made = 0
    while made < 2 and w.n("1", "metal") >= 3 and w.n("1", "energy") >= 6 and w.n("1", "citizen") >= 2:
        w.add("1", "metal", -3)
        w.add("1", "energy", -6)
        w.add("1", "citizen", -2)
        w.add("1", "pioneer", 1)
        made += 1
        for _ in range(3):
            turn("1")
    w.say(f"produced {made} pioneers on territory 1")

    # 4. Breach the jungle. Both must cross, or neither is enough.
    target = next(t for t, b, _n in DATA["territory_biomes"] if b == "jungle")
    moved = min(made, w.n("1", "pioneer"))
    w.add("1", "pioneer", -moved)
    w.add(target, "pioneer", moved)
    breach = w.force(target) > NATURE[target]
    w.say(f"moved {moved} pioneers into territory {target} (jungle, nature {NATURE[target]}): "
          f"force {w.force(target)} > {NATURE[target]} is {breach}")

    # 5. One deploys and founds; the other leaves.
    if breach:
        w.add(target, "pioneer", -1)
        w.add(target, "garrison", 1)
        w.add(target, "citizen", 2)
        for res in ("food", "metal", "energy"):
            if RESOURCES[(target, res)][0]:
                w.add(target, "extractor", 1, res)
        w.say(f"found by land on {target}: garrison, 2 citizens, extractors for what it has")
        w.add(target, "pioneer", -w.n(target, "pioneer"))
        w.add("1", "pioneer", 1)
        held = w.force(target) >= NATURE[target]
        w.say(f"the other pioneer leaves: force {w.force(target)} >= {NATURE[target]} is {held}")

    # 6. Back on territory 1, build a yard and launch an ark.
    for _ in range(20):
        turn("1")
        if w.n("1", "metal") >= 15 and not w.n("1", "yard"):
            w.add("1", "metal", -15)
            w.add("1", "yard", 1)
            w.say(f"built a yard on territory 1")
        if (w.n("1", "yard") and w.n("1", "metal") >= 3
                and w.n("1", "energy") >= 12 and w.n("1", "citizen") >= 2):
            w.add("1", "metal", -3)
            w.add("1", "energy", -12)
            w.add("1", "citizen", -2)
            # **Read the recipe rather than assume it.** This step used to put an ark in orbit
            # because the runner said so, and the runner could not disagree with the data - it
            # re-implemented the recipes instead of reading them. The specification lane caught
            # it: `P-342` made `launch ark` pay an Ark's cost and produce nothing, and
            # `scenario/commands/play.4x:170` says *puts nothing into orbit, so there is no Ark
            # to move*.
            launch = next(r for r in DATA["player"] if r["name"] == "launch ark")
            makes_ark = any(op == "change" and not str(a).startswith("-") and "ark" in tgt
                            for op, tgt, a, _at, _n in launch["lines"])
            if makes_ark:
                w.add(("orbit", "1"), "ark", 1)
                w.say("launched a new ark into orbit - the loop is closed")
            else:
                w.say("launch ark: paid 3 metal, 12 energy and 2 citizens, and produced NOTHING")
                w.say("  P-342 - the recipe puts nothing into orbit, so no ark comes back")
            break

    if verbose:
        for line in w.log:
            print("  " + line)
    return w


def assert_it_played(w):
    """What the run has to show, or it proves nothing.

    A green run is worth something only if the run could have failed, so the counterfactual is
    part of it: one pioneer must not be able to breach.
    """
    jungle = next(t for t, b, _n in DATA["territory_biomes"] if b == "jungle")
    launch = next(r for r in DATA["player"] if r["name"] == "launch ark")
    makes_ark = any(op == "change" and not str(a).startswith("-") and "ark" in tgt
                    for op, tgt, a, _at, _n in launch["lines"])
    assert w.n(("orbit", "1"), "ark") >= 1 or not makes_ark,         "launch ark produces an ark and none reached orbit"
    assert w.n(jungle, "garrison") == 1, "the jungle was never founded"
    assert w.n(jungle, "citizen") >= 2, "the jungle colony has no citizens"
    assert w.force(jungle) >= NATURE[jungle], "the jungle is held by too little force"
    assert w.n("1", "yard") == 1, "no yard, so no ark could have been launched"
    # The counterfactual: one pioneer alone is 2 against a nature of 2, and 2 is not greater.
    one = World()
    one.add(jungle, "pioneer", 1)
    assert not one.force(jungle) > NATURE[jungle],         "one pioneer breaches a jungle, so the two-pioneer requirement is not doing anything"
    two = World()
    two.add(jungle, "pioneer", 2)
    assert two.force(jungle) > NATURE[jungle], "two pioneers cannot breach, so nothing can"
    # And a garrison is what lets citizens sum: without one they are max(1, 1).
    bare = World()
    bare.add(jungle, "citizen", 2)
    assert not bare.force(jungle) >= NATURE[jungle],         "two citizens hold a jungle with no garrison, so organizing does nothing"
    return jungle


if __name__ == "__main__":
    w = play()
    jungle = assert_it_played(w)
    print()
    arks = w.n(("orbit", "1"), "ark")
    print(f"  territory {jungle} is held, and one pioneer alone could not have taken it")
    if arks:
        print("  the loop closed: an ark is back in orbit")
    else:
        print("  THE LOOP DOES NOT CLOSE. Everything up to the last step works - a colony, two")
        print("  pioneers, a jungle taken and held, a yard - and then `launch ark` pays an Ark's")
        print("  cost and puts nothing into orbit. `P-342` decided that deliberately, and Sean's")
        print("  stated loop ends *launch a new ark into orbit, completing the loop*.")
