# Release: First Release

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Releases](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)

## Scope

- A single planet
- Tiny, which is 12 territories
- Each territory is self-contained. No resource and no citizen crosses a territory boundary
- A mobile unit may move across a boundary, usually to conquer and start another self-contained
  territory
- Every territory has a force of nature of 1
- The rule editor is not in this release
- A Pioneer that leaves a territory you control must found the one it enters, or it perishes

### Territory resources

**In** - `spec/planet.md`, *for each resource, a territory has room for some number of
extractors, and a density that each of them yields*.

The twelve territories are fixed, each chosen to exercise a different consequence of the rules.
Every territory has room for at least one food extractor.

| Territory | Food  | Metal | Energy | What it exercises                          |
| --------- | ----- | ----- | ------ | ------------------------------------------ |
| 1         | 3 x 4 | 3 x 4 | 3 x 4  | The landing site. Everything works         |
| 2         | 2 x 6 | 2 x 4 | 2 x 4  | Few dense food extractors                  |
| 3         | 6 x 2 | 2 x 4 | 2 x 4  | Many thin food extractors, same food total |
| 4         | 1 x 2 | 4 x 5 | 4 x 5  | The minimum a territory can be             |
| 5         | 3 x 1 | 8 x 8 | 8 x 8  | Food density 1                             |
| 6         | 4 x 4 | none  | 4 x 5  | No metal                                   |
| 7         | 4 x 4 | 4 x 5 | none   | No energy                                  |
| 8         | 6 x 6 | 1 x 2 | 1 x 2  | Population without industry                |
| 9         | 2 x 3 | 6 x 8 | 1 x 2  | Rich metal, too few hands to work it       |
| 10        | 3 x 3 | 1 x 3 | 6 x 8  | An energy depot                            |
| 11        | 5 x 6 | 5 x 6 | 5 x 6  | The prize                                  |
| 12        | 2 x 2 | 8 x 8 | 8 x 8  | Rich extractors, almost no workers         |

## The loop

1. No presence on the planet
2. Land the ark on a territory from orbit, claiming it
3. Work the food extractor to grow the population
4. Build extractors for metal and energy
5. Produce pioneers and spread across the planet by land
6. Build a Yard
7. Produce an Ark
8. Launch the Ark into orbit

## Kinds

| Kind          | What it is                                                    |
| ------------- | ------------------------------------------------------------- |
| **citizen**   | a person: provides labor, eats, and grows on surplus          |
| **garrison**  | what holds a territory; a territory has at most one           |
| **extractor** | built for one resource, and worked to produce it              |
| **yard**      | where an Ark is produced                                      |
| **ark**       | carries a landing, and can invade from orbit                  |
| **pioneer**   | founds a territory                                            |
| **food**      | eaten by citizens; expires                                    |
| **metal**     | what things are built from; conserved                         |
| **energy**    | what moves things; neither conserved nor expiring             |
| **labor**     | what working a machine takes; a citizen provides it each turn |

## Families

| Family       | Members             |
| ------------ | ------------------- |
| **thing**    | every kind above    |
| **unit**     | ark, pioneer        |
| **resource** | food, metal, energy |

## Where things are

Every thing is in a bin, and this release has three sorts.

| Bin                           | Holds                         | Up to                           |
| ----------------------------- | ----------------------------- | ------------------------------- |
| a territory's room for a kind | that kind                     | what the territory has room for |
| an extractor's catch          | the resource it was built for | the territory's density for it  |
| a unit's tank                 | energy                        | the unit's fuel                 |

There are twelve territories and one orbit. Orbit holds units and nothing else.

## Traits

| Trait               | Of                              | Values                            | Stored or derived                  |
| ------------------- | ------------------------------- | --------------------------------- | ---------------------------------- |
| **kind**            | every thing                     | one of the ten                    | stored                             |
| **place**           | every thing                     | the bin it is in                  | stored                             |
| **readiness**       | whatever readies                | ready, exhausted                  | stored                             |
| **force**           | citizen, garrison, ark, pioneer | a number                          | stored                             |
| **fuel**            | a unit                          | how much energy its tank holds    | stored                             |
| **upkeep**          | a unit                          | food per turn                     | stored                             |
| **metal in it**     | whatever is built               | a number                          | stored                             |
| **resource**        | an extractor                    | food, metal or energy             | stored                             |
| **density**         | a territory, per resource       | a number                          | stored                             |
| **room**            | a territory, per kind           | a number                          | stored                             |
| **control**         | a territory                     | claimed by a player, or unclaimed | stored                             |
| **biome**           | a territory                     | one of the six                    | stored                             |
| **force of nature** | a territory                     | a number                          | stored                             |
| **adjacency**       | a territory                     | which territories touch it        | stored                             |
| **arriving**        | a pioneer                       | yes or no                         | stored, cleared at end turn        |
| **surplus**         | food                            | yes or no                         | derived: left after everything ate |
| **unfed**           | a citizen                       | yes or no                         | derived: it did not eat            |
| **unpaid**          | a unit                          | yes or no                         | derived: its upkeep was not met    |
| **houses**          | a thing that contains things    | whether people live in it         | stored                             |

## What a territory has room for

| Kind          | Room                                                     |
| ------------- | -------------------------------------------------------- |
| **citizen**   | 8                                                        |
| **garrison**  | 1                                                        |
| **extractor** | what the *Territory resources* table gives, per resource |
| **yard**      | 1                                                        |
| **ark**       | 2                                                        |
| **pioneer**   | 2                                                        |
| **labor**     | 8                                                        |
| **food**      | 20                                                       |
| **metal**     | 20                                                       |
| **energy**    | 20                                                       |

What an extractor holds is additional: a territory holds this much, and each extractor holds one
cycle of what it makes.

## Units and structures

| Thing         | Force | Fuel | A move | Upkeep          | Costs to produce             | Metal in it | Requires   | Readies |
| ------------- | ----- | ---- | ------ | --------------- | ---------------------------- | ----------- | ---------- | ------- |
| **citizen**   | 1     |      |        |                 |                              |             |            | yes     |
| **garrison**  | 1     |      |        |                 | 1 labor, 1 metal             | 1           |            |         |
| **extractor** |       |      |        |                 | 1 labor, 1 metal             | 1           |            | yes     |
| **yard**      |       |      |        |                 | 15 metal                     | 15          |            |         |
| **ark**       | 2     | 2    | 1 fuel |                 | 12 metal, 12 energy          | 12          | a Yard     | yes     |
| **pioneer**   | 2     | 2    | 1 fuel | 1 food per turn | 8 metal, 6 energy, 1 citizen | 8           | a garrison | yes     |

A garrison's multiplier is 1. An Ark can invade land from orbit. Nothing outside this table
readies.

## Recipes

A quantity is a whole number. It is written in the recipe, read from a trait of one of the
ingredients, or read from a trait of the place named by the recipe's scope.

An ingredient is consumed exactly when the same thing, with the same traits, does not appear
among the results.

| Recipe              | Owner  | Role | Thing                                      | Qty                                       | Bound    |
| ------------------- | ------ | ---- | ------------------------------------------ | ----------------------------------------- | -------- |
| **deploy ark**      | player | in   | ark, here                                  | 1                                         | at least |
|                     |        | in   | garrison                                   | 0                                         | at most  |
|                     |        | out  | garrison                                   | 1                                         |          |
|                     |        | out  | citizen                                    | 1                                         |          |
|                     |        | out  | extractor, food                            | 1                                         |          |
| **move**            | player | in   | unit, here, ready                          | 1                                         | at least |
|                     |        | in   | energy, in that unit                       | 1                                         | at least |
|                     |        | out  | unit, there, exhausted                     | 1                                         |          |
| **found by land**   | player | in   | pioneer, arriving                          | 1                                         | at least |
|                     |        | in   | garrison                                   | 0                                         | at most  |
|                     |        | out  | garrison                                   | 1                                         |          |
|                     |        | out  | citizen                                    | 1                                         |          |
|                     |        | out  | extractor, food                            | 1                                         |          |
| **build extractor** | player | in   | labor                                      | 1                                         | at least |
|                     |        | out  | extractor                                  | 1                                         |          |
| **build yard**      | player | in   | metal                                      | 15                                        | at least |
|                     |        | out  | yard                                       | 1                                         |          |
| **produce pioneer** | player | in   | metal                                      | 8                                         | at least |
|                     |        | in   | energy                                     | 6                                         | at least |
|                     |        | in   | citizen                                    | 1                                         | at least |
|                     |        | in   | garrison                                   | 1                                         | at least |
|                     |        | out  | pioneer                                    | 1                                         |          |
|                     |        | out  | garrison                                   | 1                                         |          |
| **produce ark**     | player | in   | metal                                      | 12                                        | at least |
|                     |        | in   | energy                                     | 12                                        | at least |
|                     |        | in   | yard                                       | 1                                         | at least |
|                     |        | out  | ark                                        | 1                                         |          |
|                     |        | out  | yard                                       | 1                                         |          |
| **spend readiness** | player | in   | citizen, ready                             | 1                                         | at least |
|                     |        | out  | citizen, exhausted                         | 1                                         |          |
|                     |        | out  | labor                                      | 1                                         |          |
| **work**            | player | in   | labor                                      | 1                                         | at least |
|                     |        | in   | extractor, ready                           | 1                                         | at least |
|                     |        | out  | extractor, exhausted                       | 1                                         |          |
|                     |        | out  | resource                                   | the territory's density for that resource |          |
| **eat**             | world  | in   | citizen                                    | 1                                         | at least |
|                     |        | in   | food                                       | 1                                         | at least |
|                     |        | out  | citizen                                    | 1                                         |          |
| **grow**            | world  | in   | food, surplus                              | 1                                         | at least |
|                     |        | in   | thing, houses                              | 1                                         | at least |
|                     |        | out  | citizen                                    | 1                                         |          |
|                     |        | out  | thing, houses                              | 1                                         |          |
| **depart**          | world  | in   | citizen, unfed                             | 1                                         | at least |
| **spoil**           | world  | in   | food                                       | 1                                         | at least |
| **ready**           | world  | in   | thing, exhausted                           | 1                                         | at least |
|                     |        | out  | thing, ready                               | 1                                         |          |
| **upkeep**          | world  | in   | unit with upkeep                           | 1                                         | at least |
|                     |        | in   | food                                       | the unit's upkeep                         | at least |
|                     |        | out  | unit with upkeep                           | 1                                         |          |
| **perish**          | world  | in   | unit whose upkeep is unpaid                | 1                                         | at least |
|                     |        | out  | metal                                      | the unit's metal                          |          |
| **revert**          | world  | in   | territory, force below its force of nature | 1                                         | at least |
|                     |        | out  | territory, unclaimed                       | 1                                         |          |

## Biomes

What each biome gives a territory. Ocean is not claimable and carries nothing.

| Biome     | Food  | Metal | Energy | Force of nature |
| --------- | ----- | ----- | ------ | --------------- |
| Ocean     | -     | -     | -      | -               |
| Ice       | 1 x 2 | 2 x 3 | 1 x 2  | 1               |
| Desert    | 2 x 4 | 3 x 4 | 5 x 6  | 1               |
| Grassland | 5 x 6 | 1 x 3 | 1 x 3  | 1               |
| Jungle    | 4 x 6 | 1 x 2 | 1 x 2  | 1               |
| Mountain  | 1 x 3 | 5 x 7 | 2 x 3  | 1               |

`5 x 6` is room for five extractors, each yielding six. Every biome except ocean has room for at
least one food extractor at density two or more.

## Controls

- Rotation is bound to the arrow keys, and to dragging
- Zoom is bound to the wheel, and to pinching
- Reset is bound to `R`, and to a control
- The three surfaces in this release are reached by `F1`, `F2` and `F3`, by buttons on the page,
  and by `/game`, `/console` and `/browser` typed at the console
- Choosing a planet size abandons the current game and starts one on a planet of that size. It
  is bound to `1` through `5`, to a control for each size, and to `/new <size>`

## Capabilities

Each capability is an item in the same shape every outbox uses, so it appears in `pending.md` and in
`tools/outbox` beside what a lens has found. It moves through three states and changes hands once:

- **`open`, addressed `to code`** - not built yet
- **`built`, addressed `to sean`** - the code lane says it is done, and nobody has looked
- **`vetted`** - a person has observed the *vetted when* line and it held

**The code lane does not mark its own.** It reports the evidence and this lane records it, which is
what `docs/process.md` requires in Sean's own words - *so that the account of what has been
delivered is not kept by whoever built it*. It touches neither `built` nor `vetted`. **Five of the six below are vetted by a person looking**,
at a drawing or at a whole game played through, so `built` is where they will wait and Sean is the
only one who can move them.

### R-1 - Two drawings

**to** sean · **status** **built** 2026-08-30 · **evidence** both drawings exist and are photographed; ids on the practical one only, poles marked, camera shared

- **In** - `spec/planet.md`, *the planet is drawn either practically or realistically, and the
  user can change which*
- **Vetted when** - switching between them moves nothing: the planet is at the same rotation and
  zoom afterwards, and every territory covers the same pixels

### R-2 - Terrain that crosses boundaries

**to** sean · **status** **built** 2026-08-30 · **evidence** one continuous field sampled per point; coastlines cross territory boundaries in the photograph

- **In** - `spec/planet.md`, *the terrain of the realistic drawing is continuous*
- **Vetted when** - no line visible in the realistic drawing coincides with a territory boundary,
  and terrain visibly varies within a single territory

### R-3 - A division that cannot be seen

**to** sean · **status** **built** 2026-08-30 · **evidence** no seam and no boundary in the realistic drawing; `Drawn.labels` is zero there

- **In** - `spec/planet.md`, *nothing in the terrain reveals how the sphere was divided*
- **Vetted when** - a person who has not seen the tessellation is shown the realistic drawing and
  cannot mark where a five-neighbour territory is, beyond the two at the poles

### R-4 - A biome per territory

**to** sean · **status** **built** 2026-08-30 · **evidence** `biomes_of` gives every territory one, and `join_the_land` keeps land connected

- **In** - `spec/planet.md`, *each territory has a biome*, and *a territory's biome is what the
  terrain gives it*
- **Vetted when** - `show territory 5` names a biome, and no other biome covers more of that
  ground in the realistic drawing

### R-5 - Terrain resolved as finely as it is shown

**to** sean · **status** **built** 2026-08-30 · **evidence** 400,000 sub-triangles, blended in parameter space

- **In** - `spec/planet.md`, *nothing of how a drawing is made is visible in it*
- **Vetted when** - at the default camera, no facet, band or flat wash betrays how the surface was
  built, and the finest visible detail is terrain

### R-6 - The loop can be played through

**to** code · **status** open · **cited** `faafb5f`, `2f38241` · blocked by `C-7`, see `P-125` · **blocked by** `C-11`

- **In** - `spec/control.md`, *a player wins by launching an Ark from a fully exploited planet*
- **Vetted when** - starting from a single Ark in orbit over the twelve designed territories, a
  person playing entirely by hand reaches a fully exploited planet and launches an Ark
- **Unreachable today, and not for the reason it was.** `P-126` and `P-138` removed the wall `C-7` and `C-8` described, and the code has not followed: `crates/game-model/src/game.rs:705` still empties a territory's stores at the end of every turn, under a comment quoting the rule `P-126` replaced. **A play-through run now would hit the old wall and prove nothing about the game as specified.** `C-11` records the divergence and argues for not repairing it yet, since `P-134` removes the very shapes it lives in.

## Open questions
