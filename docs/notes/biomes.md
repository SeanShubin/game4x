# What a Biome Could Mean

**Derived.** Written by Claude from conversation, 2026-08-28. Not binding - options and arguments,
not decisions. See [the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Planet appearance](planet-appearance.md) · [Documentation map](../README.md)

`spec/planet.md` says each territory has a biome and that the terrain gives it. Neither says which
biomes exist, and no rule reads one. Sean asked for **a substantively distinct set**, and then for
what mechanics might apply to it. This is the first half, with the second sketched.

## The fact that bounds the question

**The rules can see exactly two things about a territory.** Grepping every file in `spec/` for what
reads a territory property returns nodes and force of nature, and nothing else:

- **Nodes** - how many, of which resource, at what density (`spec/economy.md`, `spec/structures.md`)
- **Force of nature** - what it takes to hold the ground (`spec/control.md`)

Everything else a territory has - its id, its biome, where it sits - is read by the interface and by
nobody else. **So a biome can only be mechanically distinct along those two axes today.** Anything
more requires the rules to gain something new to read, which is the second half of this note.

## One territory, one biome

Confirmed by Sean, 2026-08-28. `spec/planet.md` says *each territory has a biome*, and the singular
is meant: **exactly one, never a mix.**

**That is a deliberate coarsening, and it is worth seeing as one.** P-97 says the terrain is
continuous and *varies within a single territory* - so a territory whose ground runs from forest into
grassland still has one biome in the model. The picture is continuous; the model is not.

**It is also what makes P-100 a rule rather than a formality.** *A territory's biome is what the
terrain gives it* has to answer a question the picture does not: which biome, when the ground holds
two. Commonest by area, sampled at the centre, or area-weighted - all satisfy the line, and choosing
between them is implementation. What it forbids is picking a biome the ground does not support at
all.

## The test for "substantively distinct"

**A biome should change what a territory is *for*, not how much it yields.** A biome that gives 20%
more food is a multiplier, and a multiplier does not change a decision - you still want the
territory, just slightly more. A biome that gives metal and no food changes what you do with the
ground: you take it for industry and you cannot grow there.

That is the test worth applying to any candidate. **Does knowing this biome change what a player
does with the territory, or only how pleased they are to have it?**

## What Dwarf Fortress actually does

Its biomes are **coordinates, not categories**. Elevation, rainfall, temperature, drainage and a
couple of others are continuous fields; a biome is the name given to a region of that space. Nobody
places a swamp - swamp is what *is* at low drainage and high rainfall.

**Two consequences worth taking.** The list of biomes is a **consequence of where thresholds are
drawn**, so it is tuning rather than design, which is what P-99 leaves open deliberately. And
neighbouring biomes are automatically plausible, because adjacent points in parameter space differ
by a little in every axis - you never get tundra beside jungle unless a field jumps.

The axes available here are settled by
[planet appearance](planet-appearance.md): elevation, moisture and drainage, with **temperature
nearly free** because the sphere has latitude and `spec/planet.md` fixes an axis.

## A candidate set, by role rather than by name

Six roles the current economy can actually distinguish, with a plausible biome for each and the
axis that produces it:

| Role           | Biome     | Falls out of        | Food | Metal | Energy | Force of nature |
| -------------- | --------- | ------------------- | ---- | ----- | ------ | --------------- |
| **Impassable** | Ocean     | lowest elevation    | -    | -     | -      | not claimable   |
| **Population** | Grassland | temperate, moderate | high | low   | low    | low             |
| **Industry**   | Mountain  | highest elevation   | none | high  | low    | mid             |
| **Range**      | Desert    | low moisture        | low  | mid   | high   | low             |
| **Contested**  | Jungle    | hot and wet         | high | low   | low    | **high**        |
| **Marginal**   | Ice       | lowest temperature  | none | low   | low    | low             |

**Six is fewer than either sibling project's twelve, and that is the point.** Visual variety and
mechanical distinctness want different counts: twelve biomes for the eye, six for the rules. Nothing
stops a planet drawing tundra, savanna, marsh and steppe - they simply resolve to one of the roles
above until a rule exists that can tell them apart.

**Jungle is the one that earns its place twice.** It is the only candidate that is *good* and
*dangerous* at once, which makes it the only one that produces a real decision rather than a
preference. Everything else is take-it-when-you-reach-it.

## The decision this forces first: water

**Ocean is the highest-consequence entry, and it is not really a biome question.** It changes
**connectivity**, and connectivity is what the whole game runs on: a Pioneer moves to an adjacent
territory, and the planet is currently a graph where every territory has five or six neighbours and
the greatest distance is three.

Put water in and:

- Some territories become unreachable by land, so **orbit stops being a convenience and becomes the
  only route** to part of the planet. `spec/orbit.md` already allows a unit to land anywhere.
- The greatest-distance figures in `spec/planet.md` stop describing travel. They describe the
  polyhedron, not the reachable graph.
- P-77's win condition - *every territory that can be taken has been taken* - starts doing real
  work, because *can* becomes genuinely restrictive rather than trivially satisfied.

The [open questions](planet-appearance.md) already ask whether an ocean territory is one nobody can
claim or is absent from the game. **Absent is not available**: the tessellation has no way to remove
a cell, and `10T + 2` is exact. So water is unclaimable ground, or there is no water.

## A drift the fixture will hit

`releases/first-release.md` assigns nodes to all twelve territories **by hand**, and `spec/planet.md`
now says a biome is **what the terrain gives it**. If a biome also determines nodes, those two
assignments must agree - and nothing checks that they do.

**It is the same failure P-100 was written to prevent, one level up.** That line stops the model's
biome contradicting the picture's terrain; nothing yet stops the model's *nodes* contradicting the
model's *biome*. A hand-placed metal node in a territory whose terrain says grassland is exactly as
wrong, and exactly as invisible.

## What mechanics could apply, once there are biomes

Today a biome can only touch nodes and force of nature. Beyond that, each of these needs the rules
to gain something they can read - so each is a spec change, not a tuning one:

| Mechanic                           | What it needs                                                                                           |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Biome sets node counts and density | nothing - the rules already read nodes                                                                  |
| Biome sets force of nature         | nothing - already read                                                                                  |
| Moving costs more in some biomes   | a per-territory movement cost; move cost is currently per unit                                          |
| Population grows slower somewhere  | the growth rule to read the territory, which it does not                                                |
| Some structures need some biomes   | a requirement list per structure - the `required` slot from [intermediate steps](intermediate-steps.md) |
| A biome hosts different life       | `spec/planet.md` already says each planet has its own native species; nothing connects them to ground   |

**The last row is the cheapest interesting one.** Native species are already a per-planet fact and
force of nature is already per-territory. Saying *which* species lives where - and that its force is
the territory's - would connect two things that already exist, and would make jungle dangerous for a
stated reason rather than by fiat.
