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
import os
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
        """Sean's rule, read from `force_rule` rather than written again here.

        **The organizers are the garrison and every unit**, because `spec/control.md` says
        coordination is imposed by a structure *or by a military unit, which carries it*. This
        used to organize on a garrison alone, so two pioneers standing with two citizens
        presented 5 where the rule says 6.
        """
        organizers = DATA["force_rule"]["organizers"]
        carried = [FORCE[k] for k in FORCE for _ in range(self.n(place, k))]
        if not carried:
            return 0
        return (sum(carried) if any(self.n(place, k) for k in organizers)
                else max(carried))

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

    def work_turn(t, want, labor):
        """Work extractors for one resource, bounded by capacity, density and labor.

        Returns the labor spent, because a citizen at an extractor is not available to build -
        `spec/economy.md`: *a citizen works at one structure and cannot be in two places at
        once.*
        """
        cap, den = RESOURCES[(t, want)]
        have = min(w.n(t, "extractor", want), cap, labor)
        w.add(t, want, have * den)
        return have

    def build(t, kind, res, metal, labor):
        """One `build` recipe: 1 labor and some metal, and the container must accept it."""
        if labor < 1 or w.n(t, "metal") < metal:
            return 0
        if kind == "store" and w.n(t, "store", res) >= w.n(t, "extractor", res):
            return 0            # as many stores as the extractors of its resource
        if kind == "extractor" and w.n(t, "extractor", res) >= RESOURCES[(t, res)][0]:
            return 0            # the deposit's total capacity
        w.add(t, "metal", -metal)
        w.add(t, kind, 1, res)
        return 1

    def turn(t, plan=()):
        """One turn in one territory: work everything, eat, then grow on the surplus.

        `grow` was missing from the first version of this and the run stopped dead: producing
        a pioneer costs two citizens and a colony starts with exactly two, so the first pioneer
        emptied the colony and nothing could work an extractor afterwards. The population has to
        grow before anything can be spent - which is the loop, and the runner was not playing it.
        """
        # **Food, then a reserve, then metal, then the plan, then energy with the rest.**
        # A citizen at an extractor is not building, so working everything owned leaves nothing
        # to grow with - which is what the trace showed before this order existed.
        labor = w.n(t, "citizen")
        labor -= work_turn(t, "food", labor)
        reserve = min(len(plan), 2)
        labor -= work_turn(t, "metal", max(0, labor - reserve))
        for kind, res, metal in plan:
            labor -= build(t, kind, res, metal, labor)
        labor -= work_turn(t, "energy", labor)
        eaten = min(w.n(t, "food"), w.n(t, "citizen"))
        w.add(t, "food", -eaten)
        starved = w.n(t, "citizen") - eaten
        if starved > 0:
            w.add(t, "citizen", -starved)   # unsustained: upkeep unmet
        grown = min(w.n(t, "food"), w.n(t, "citizen"))   # surplus food, and citizens
        if grown:
            w.add(t, "food", -grown)
            w.add(t, "citizen", grown)
        # **`end-of-turn losses`, played rather than approximated.** Food and labor go whole;
        # metal and energy are cut to what the territory's stores hold, and a territory may
        # have one store per extractor of that resource. **A colony builds none**, so until it
        # buys some it keeps nothing - which the runner used to hide by bounding food alone and
        # letting 23 metal pile up on a territory with nowhere to put it.
        w.add(t, "food", -w.n(t, "food"))
        w.add(t, "labor", -w.n(t, "labor"))
        for res in ("metal", "energy"):
            over = w.n(t, res) - DATA["store_capacity"] * w.n(t, "store", res)
            if over > 0:
                w.add(t, res, -over)
        return grown

    # 2. Build up territory 1 until it can afford a pioneer: 3 metal, 6 energy, 2 citizens.
    #
    # **A colony is delivered with no store**, so until it builds one every scrap of metal it
    # mines is gone when the turn ends - `end-of-turn losses`. That makes the opening forced,
    # and it is the thing the runner used to hide: it let six turns accumulate 23 metal on a
    # territory whose stores hold nothing.
    #
    # The plan is therefore stores first, then the energy extractor Sean's gate needs - *you
    # can't really start expanding until you start exploiting energy in a region* - then more
    # extractors while there is labor for them.
    # **Food extractors first, because the population is the only thing that makes more
    # labor**, then a metal store so anything mined survives the turn, then the energy pair -
    # an extractor is useless without a store to keep what it makes.
    PLAN = [("extractor", "food", 1), ("store", "metal", 1), ("extractor", "food", 1),
            ("extractor", "energy", 1), ("store", "energy", 1), ("extractor", "metal", 1),
            ("store", "energy", 1), ("store", "metal", 1), ("extractor", "energy", 1)]
    turns = 0
    while turns < 30 and not (w.n("1", "metal") >= 3 and w.n("1", "energy") >= 6
                              and w.n("1", "citizen") >= 4):
        turn("1", PLAN)
        turns += 1
        if os.environ.get("TRACE"):   # a per-turn trace, for reading the policy
            print(f"  t{turns}: cit={w.n('1','citizen')} "
                  f"Xf={w.n('1','extractor','food')} Xm={w.n('1','extractor','metal')} "
                  f"Xe={w.n('1','extractor','energy')} "
                  f"Sm={w.n('1','store','metal')} Se={w.n('1','store','energy')} "
                  f"m={w.n('1','metal')} e={w.n('1','energy')}")
    w.say(f"{turns} turns building territory 1: "
          f"{w.n('1','citizen')} citizens, {w.n('1','store','metal')} metal stores and "
          f"{w.n('1','store','energy')} energy stores, "
          f"{w.n('1','metal')} metal and {w.n('1','energy')} energy kept")

    # 3. Two pioneers, to breach a jungle. Territory 6 is jungle, nature 2.
    made = 0
    while made < 2 and w.n("1", "metal") >= 3 and w.n("1", "energy") >= 6 and w.n("1", "citizen") >= 2:
        w.add("1", "metal", -3)
        w.add("1", "energy", -6)
        w.add("1", "citizen", -2)
        w.add("1", "pioneer", 1)
        made += 1
        for _ in range(3):
            turn("1", PLAN)
    w.say(f"produced {made} pioneers on territory 1")

    # 4. Load the tanks, then breach. Sean, 2026-09-09: a tank is loaded from a region, one
    #    move per fuel cell, two cells per pioneer - so expansion gates on having exploited
    #    energy somewhere first. `found-colony` builds a food and a metal extractor and no
    #    energy one, so the gate is real and was invisible while `move` spent from the ground.
    target = next(t for t, b, _n in DATA["territory_biomes"] if b == "jungle")
    # **`refuel`, not `load`.** `spec/units.md`: fuel moves freely between a *controlled*
    # territory that has it and anything there that can hold it - so nothing the player does
    # fills a bin, and `spec/invariants.md` forbids an action whose first step is always taken.
    FUEL = 2
    cells = 0
    for _ in range(made):
        take = min(FUEL, w.n("1", "energy"))
        w.add("1", "energy", -take)
        cells += take
    w.say(f"refuel filled {cells} cells into {made} pioneers on controlled ground "
          f"(the bin holds {FUEL}; energy left in territory 1: {w.n('1','energy')})")
    hops = 1  # territory 6 is one hop from territory 1
    if cells < made * hops:
        w.say(f"NOT ENOUGH FUEL: {made} pioneers need {made * hops} cells to make {hops} hop(s)")
        return w
    cells -= made * hops
    moved = min(made, w.n("1", "pioneer"))
    w.add("1", "pioneer", -moved)
    w.add(target, "pioneer", moved)
    w.say(f"each pioneer spent 1 cell to cross; {cells} cells left between them")
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
