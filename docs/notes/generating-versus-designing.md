# Generating a Planet, Designing a Territory

**Derived.** Written by Claude from conversation, 2026-08-28. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Biomes](biomes.md) · [Planet appearance](planet-appearance.md)

Sean saw a possible conflict between generating a whole planet and generating a single territory,
and was not sure it was real. It is real in one place only, and that place is not where it looks.

## The two directions, and the third thing actually wanted

**Field first.** Generate elevation, moisture and temperature; biomes fall out of where the fields
cross; each territory gets whatever its ground holds. **Coherent by construction** - nothing can be
inconsistent because nothing was placed. The cost is that you get whatever distribution the noise
happens to give.

**Territory first.** Decide each territory's biome, then synthesise terrain that agrees. **Exactly
the distribution you asked for.** The cost is that coherence is not automatic: nothing stops jungle
bordering ice, or a desert on a coast that should be rainforest.

**Sean wants neither, and the third option is cheaper than both.** *"I want some distribution of
biomes in territories but I don't actually care where they are."* Distribution without placement.

## Distribution without placement: move the cut points, not the biomes

**If the histogram matters and the locations do not, tune the thresholds rather than the terrain.**

Biomes are regions of parameter space
([Dwarf Fortress's actual design](biomes.md)), and which biome a point gets is decided by where the
cut points sit. Want a planet that is predominantly forest and ocean with some desert and grassland?
**Lower the elevation cut so more of the sphere is below sea level; move the moisture cut so more of
the land lands in the wet band.** The fields are untouched.

**Coherence survives because nothing was moved.** Every biome still sits where its parameters put it,
neighbours still differ by a little in every axis, and the picture is still a function of position.
**All that changed is where the lines were drawn**, and the lines are not visible in the world.

The procedure is a fit rather than a design: generate the fields, measure the resulting histogram,
nudge the cut points, measure again. It converges quickly because the mapping from cut point to area
is monotonic - move a threshold one way and that biome always grows.

**And it is the same table the picture uses**, so the tuning is one set of numbers rather than two.
See [procedural planets elsewhere](procedural-planets-elsewhere.md) on the lookup being an image: the
cut points are where the bands sit in that image.

## Plurality settles a question P-100 left open

Sean: *"if a territory is jungle, it just has to be mostly jungle by plurality, not even majority."*

`spec/planet.md` says a territory's biome is **what the terrain gives it**, and
[the proposal that landed it](proposals.md) noted that *commonest by area, sampled at the centre, or
area-weighted* all satisfy the line, with the choice left to implementation. **Plurality is the
choice**, and it is the most robust of the three: a centre sample is a single point and can land in a
sliver, while area-weighted blending produces a number rather than a name.

**It has one consequence worth naming.** A territory that is 40% jungle, 35% grassland and 25% desert
**is jungle**, and its picture shows all three. That is not a defect - it is the picture-model split
P-100 exists to hold - but it makes one release line loose. `releases/first-release.md` vets a biome
on *the biome the realistic drawing shows over that ground*, and under plurality the drawing shows
several. It wants *shows over most of that ground*.

## The rendering licence, which is larger than it looks

Sean: *"the game mechanics don't care which part of a territory a jungle structure is built on, so we
can visually move it wherever we want."*

**That is true and it is worth stating, because an implementer will assume the opposite.** Nothing in
`spec/` gives a position inside a territory. A territory has an id, nodes, a biome and neighbours; a
structure is *in* a territory and nowhere more precise. **So the renderer chooses where things sit,
and should choose where they look right** - a farm on the grassland part, a mine on the rock.

**It generalises past structures.** The same licence covers a territory's id, its ownership mark and
anything else drawn over ground: the model says *in this territory*, and the picture picks the spot.
That is exactly the sticker problem [the arcade research](planet-appearance.md) already documents.

## The one real conflict: a designed scenario is not a world

**This is the place Sean's worry actually bites, and the resolution is to stop trying.**

`releases/first-release.md` has twelve hand-designed territories. Field-first generation cannot
produce them, and it should not be asked to - **they were never meant to be a plausible planet.**
Territory 5 is three food nodes at density 1 beside sixteen nodes at density 8: the richest ground on
the planet with nobody able to work it. That exists to demonstrate that spare labor is
`nodes x (density - 1)`. **No coherent world would produce it, and a generator that did would be
badly tuned.**

So they are two artifacts with two jobs:

|          | A generated planet      | A designed scenario              |
| -------- | ----------------------- | -------------------------------- |
| Built by | fields, then thresholds | `add node` and `set force`       |
| Coherent | by construction         | not attempted                    |
| For      | playing                 | demonstrating a rule, or testing |

**Both are built the same way**, which is what stops this being a fork in the code: the design phase
(P-74) takes commands, and a generator is something that *emits* those commands. A scenario is
hand-written; a planet is generated. **The engine cannot tell the difference and does not need to.**

**And if a scenario ever needs to be plausible too, the answer is search rather than constraint.**
Generate planets until one satisfies the scenario's requirements - a metal-rich territory three moves
from a food-rich one, say. Coherence is preserved because every candidate was generated coherently;
only the accepting is selective. That costs generation time and no design, which is the right way
round.

## What this does not settle

**Whether ocean's no-two-adjacent rule survives threshold tuning.** `spec/planet.md` forbids adjacent
oceans, and the natural way to get *predominantly ocean* is to lower the elevation cut - which
produces adjacent oceans immediately. **Those two pull against each other**, and the more water a
planet is asked for, the harder the restriction is to satisfy. Above about a third of the sphere it
is impossible, since that is the largest set of territories no two of which touch.
[The backlog](spec-backlog.md) records that the restriction lifts when units can cross water; until
then, *predominantly ocean* is not a planet this game can generate.
"""
