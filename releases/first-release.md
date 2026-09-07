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
- The rule editor is not in this release
- A Pioneer that enters a territory nobody holds must found it, or it perishes for want of food

### Territory resources

**In** - `spec/planet.md`, *for each resource, a territory has total capacity for some number
of extractors, and a density that each of them yields*.

The twelve territories are fixed, each chosen to exercise a different consequence of the rules.
Every territory has total capacity for at least one food extractor.

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

| Kind          | What it is                                                                                                   |
| ------------- | ------------------------------------------------------------------------------------------------------------ |
| **citizen**   | a person: provides labor, eats, and grows on surplus                                                         |
| **garrison**  | what holds a territory; a territory has at most one                                                          |
| **extractor** | built for one resource, and worked to produce it                                                             |
| **yard**      | where an Ark is produced                                                                                     |
| **store**     | built to hold one resource, and holds nothing else                                                           |
| **ark**       | carries a landing, and can invade from orbit                                                                 |
| **pioneer**   | founds a territory                                                                                           |
| **food**      | eaten by citizens; expires                                                                                   |
| **metal**     | what things are built from; conserved                                                                        |
| **energy**    | what moves things; neither conserved nor expiring                                                            |
| **labor**     | what working a machine takes; a citizen provides it each turn                                                |
| **territory** | a place things are in, which has a biome, a force of nature, and a density and a total capacity per resource |
| **orbit**     | a place above one territory, which holds units and nothing else                                              |

## Families

| Family       | Members             |
| ------------ | ------------------- |
| **thing**    | every kind above    |
| **unit**     | ark, pioneer        |
| **resource** | food, metal, energy |
| **place**    | territory, orbit    |

## Where things are

Every thing is in another thing, and this release has three sorts of capacity.

| Container                               | Holds                         | Up to                            |
| --------------------------------------- | ----------------------------- | -------------------------------- |
| a territory's total capacity for a kind | that kind                     | its total capacity for that kind |
| a store                                 | the resource it was built for | 10                               |
| a unit's tank                           | energy                        | the unit's fuel                  |

There are twelve territories and twelve orbits. An orbit holds units and nothing else.

## Traits

Where a trait admits a closed set of values, its **Values** cell names them, or says where they
are listed.

| Trait              | Of                                      | Values                                             | Stored or derived                                |
| ------------------ | --------------------------------------- | -------------------------------------------------- | ------------------------------------------------ |
| **kind**           | every thing                             | one of the kinds                                   | stored                                           |
| **id**             | a thing that must be named individually | a number, unique among things of its kind          | stored                                           |
| **ready**          | whatever readies                        | yes or no                                          | stored                                           |
| **resource**       | an extractor or a store                 | one of the resources                               | stored                                           |
| **force**          | citizen, garrison, ark, pioneer         | a number                                           | stored                                           |
| **fuel**           | a unit                                  | how much energy its tank holds                     | stored                                           |
| **upkeep**         | a thing with upkeep                     | food per turn                                      | stored                                           |
| **metal in it**    | whatever is built                       | a number                                           | derived: its binding plus the metal in its parts |
| **density**        | a territory, per resource               | a number                                           | stored                                           |
| **total capacity** | a territory, per kind                   | a number                                           | stored                                           |
| **control**        | a territory                             | held by a player, or unclaimed                     | derived: a citizen of that player is there       |
| **biome**          | a territory                             | one of the biomes                                  | stored                                           |
| **nature**         | a territory                             | a number                                           | stored                                           |
| **adjacency**      | a place                                 | which places it touches, and by which kind of edge | stored                                           |
| **keeps**          | food                                    | the number of turns it will last                   | stored                                           |
| **surplus**        | food                                    | yes or no                                          | derived: left after every upkeep was paid        |
| **unpaid**         | a thing with upkeep                     | yes or no                                          | derived: its upkeep was not met                  |
| **houses**         | a thing that contains things            | whether people live in it                          | stored                                           |
| **phase**          | the game                                | design or play                                     | stored                                           |

Food is made with `keeps` 1. The force nature holds a territory with.

## What bounds a kind in a territory

| Kind          | Bounded by                                               |
| ------------- | -------------------------------------------------------- |
| **citizen**   | the food produced here, through upkeep                   |
| **garrison**  | a capacity of 1                                          |
| **extractor** | a capacity, from *Territory resources*                   |
| **store**     | as many as the extractors of its resource                |
| **yard**      | a capacity of 1                                          |
| **ark**       | a capacity of 2                                          |
| **pioneer**   | a capacity of 2, and the food produced here              |
| **labor**     | the citizens that make it, one each per turn             |
| **food**      | the things in it that hold it, and it keeps for one turn |
| **metal**     | the things in it that hold it                            |
| **energy**    | the things in it that hold it                            |

A territory declares no capacity for a resource. It declares capacity for the things that
hold them - a store holds what it was built to hold, and an extractor holds nothing. **A resource that is in nothing can be used the turn it is made, and is lost when that turn ends -
use it immediately, store it, or lose it.**

## Units and structures

| Thing         | Force | Fuel | A move | Upkeep          | Costs to produce               | Binding | Crosses              | Requires | Readies |
| ------------- | ----- | ---- | ------ | --------------- | ------------------------------ | ------- | -------------------- | -------- | ------- |
| **citizen**   | 1     |      |        | 1 food per turn |                                |         |                      |          | yes     |
| **garrison**  | 0     |      |        |                 | 1 labor, 1 metal               | 1       |                      |          |         |
| **extractor** |       |      |        |                 | 1 labor, 1 metal               | 1       |                      |          | yes     |
| **yard**      |       |      |        |                 | 1 labor, 15 metal              | 15      |                      |          |         |
| **store**     |       |      |        |                 | 1 labor, 1 metal               | 1       |                      |          |         |
| **ark**       | 2     | 2    | 1 fuel |                 | 3 metal, 12 energy, 2 citizens | 3       | orbit border, ascent | a Yard   | yes     |
| **pioneer**   | 2     | 2    | 1 fuel | 1 food per turn | 3 metal, 6 energy, 2 citizens  | 3       | border               |          | yes     |

An Ark can invade land from orbit. Nothing outside this table
readies.

## Recipes

The recipe table has seven columns: **Recipe**, **Auto**, **Role**, **Qty**, **Kind**, **Traits**
and **Where**.

**Auto** is `player` or `world`. **Role** is one of `require`, `limit`, `consume` or `produce`: a
requirement must be present and is not taken, a limit is a maximum that must not be exceeded, a
consumption is taken, and a production is made. **Qty** is a whole number or an expression. **Kind**
is the kind or the family alone. **Traits** are the constraints on it. **Where** is the place the row
is about, and a blank means the one place the recipe acts.

A quantity is a whole number. It is written in the recipe, read from a trait of one of the
ingredients, or read from a trait of a named ingredient.

An ingredient may be given a name, written `$name`, and another ingredient may refer to it. A
recipe that names two things of the same kind must name them, because otherwise a reference has
two candidates.

A blank is not a zero. It says the row has no such number, and a quantity read from one produces
nothing.

Food made this turn survives one ending and is lost at the next.

**In** - `spec/turn.md`, *ending a turn: everything with upkeep pays it; then a population grows
on surplus food or starves for want of it; what expires expires, and what was not kept in order
is lost; and everything becomes ready again*.

The player's recipes fire when the player chooses them. The world's fire when the turn ends, in
that order: `upkeep`, then `grow` and `perish`, then `spoil`, then `age`, then `refresh`. The rows
below are in that order.

| Recipe              | Owner  | Role    | Qty                                                  | Kind      | Traits                                        | Where                    |
| ------------------- | ------ | ------- | ---------------------------------------------------- | --------- | --------------------------------------------- | ------------------------ |
| **deploy ark**      | player | require | 1                                                    | territory |                                               | `$where`                 |
|                     |        | consume | 1                                                    | ark       |                                               | the orbit above `$where` |
|                     |        | limit   | 0                                                    | garrison  |                                               |                          |
|                     |        | produce | 1                                                    | garrison  |                                               |                          |
|                     |        | produce | 2                                                    | citizen   |                                               |                          |
|                     |        | produce | 1                                                    | extractor | food                                          |                          |
|                     |        | produce | 1                                                    | extractor | metal                                         |                          |
|                     |        | produce | 1                                                    | store     | food                                          |                          |
|                     |        | produce | 1                                                    | store     | metal                                         |                          |
| **move**            | player | require | 1                                                    | place     |                                               | `$from`                  |
|                     |        | require | 1                                                    | place     | joined to `$from` by an edge the unit crosses | `$to`                    |
|                     |        | consume | 1                                                    | unit      | ready                                         | `$from`                  |
|                     |        | consume | 1                                                    | energy    |                                               | that unit                |
|                     |        | produce | 1                                                    | unit      | not ready                                     | `$to`                    |
| **found by land**   | player | consume | 1                                                    | pioneer   |                                               |                          |
|                     |        | limit   | 0                                                    | garrison  |                                               |                          |
|                     |        | produce | 1                                                    | garrison  |                                               |                          |
|                     |        | produce | 2                                                    | citizen   |                                               |                          |
|                     |        | produce | 1                                                    | extractor | food                                          |                          |
|                     |        | produce | 1                                                    | extractor | metal                                         |                          |
|                     |        | produce | 1                                                    | store     | food                                          |                          |
|                     |        | produce | 1                                                    | store     | metal                                         |                          |
| **build extractor** | player | consume | 1                                                    | labor     |                                               |                          |
|                     |        | consume | 1                                                    | metal     |                                               |                          |
|                     |        | produce | 1                                                    | extractor | `$resource`                                   |                          |
| **build store**     | player | consume | 1                                                    | labor     |                                               |                          |
|                     |        | consume | 1                                                    | metal     |                                               |                          |
|                     |        | produce | 1                                                    | store     | `$resource`                                   |                          |
| **build yard**      | player | consume | 1                                                    | labor     |                                               |                          |
|                     |        | consume | 15                                                   | metal     |                                               |                          |
|                     |        | produce | 1                                                    | yard      |                                               |                          |
| **produce pioneer** | player | consume | 3                                                    | metal     |                                               |                          |
|                     |        | consume | 6                                                    | energy    |                                               |                          |
|                     |        | consume | 2                                                    | citizen   |                                               |                          |
|                     |        | produce | 1                                                    | pioneer   |                                               |                          |
| **produce ark**     | player | consume | 3                                                    | metal     |                                               |                          |
|                     |        | consume | 12                                                   | energy    |                                               |                          |
|                     |        | consume | 2                                                    | citizen   |                                               |                          |
|                     |        | require | 1                                                    | yard      |                                               |                          |
|                     |        | produce | 1                                                    | ark       |                                               |                          |
| **create labor**    | player | consume | 1                                                    | citizen   | ready                                         |                          |
|                     |        | produce | 1                                                    | citizen   | not ready                                     |                          |
|                     |        | produce | 1                                                    | labor     |                                               |                          |
| **work**            | player | require | 1                                                    | territory |                                               | `$where`                 |
|                     |        | consume | 1                                                    | labor     |                                               |                          |
|                     |        | consume | 1                                                    | extractor | ready                                         |                          |
|                     |        | produce | 1                                                    | extractor | not ready                                     |                          |
|                     |        | produce | `$where`'s density for that resource                 | resource  |                                               |                          |
| **upkeep**          | world  | require | 1                                                    | thing     | with upkeep                                   |                          |
|                     |        | consume | the thing's upkeep                                   | food      |                                               |                          |
| **grow**            | world  | consume | the lesser of the surplus food and the citizens here | food      | surplus                                       |                          |
|                     |        | produce | the lesser of the surplus food and the citizens here | citizen   |                                               |                          |
| **perish**          | world  | consume | 1                                                    | thing     | whose upkeep is unpaid                        |                          |
|                     |        | produce | the thing's metal                                    | metal     |                                               |                          |
| **spoil**           | world  | consume | 1                                                    | food      | keeps 0                                       |                          |
| **age**             | world  | consume | 1                                                    | food      | keeps at least 1                              |                          |
|                     |        | produce | 1                                                    | food      | keeps one less                                |                          |
| **refresh**         | world  | consume | 1                                                    | thing     | not ready                                     |                          |
|                     |        | produce | 1                                                    | thing     | ready                                         |                          |

## Biomes

What a biome is like, so that a territory's numbers can be chosen to suit it. **The numbers here
guide and do not bind; a territory's own are in *Territory resources*. Force of nature is the one
column that binds.** Ocean is not claimable and carries nothing.

| Biome     | Food  | Metal | Energy | Force of nature |
| --------- | ----- | ----- | ------ | --------------- |
| Ocean     | -     | -     | -      | -               |
| Ice       | 1 x 2 | 3 x 5 | 1 x 2  | 1               |
| Desert    | 2 x 4 | 3 x 4 | 5 x 6  | 1               |
| Grassland | 5 x 6 | 2 x 3 | 1 x 3  | 1               |
| Jungle    | 6 x 6 | 1 x 2 | 1 x 2  | 2               |
| Mountain  | 1 x 3 | 5 x 7 | 2 x 3  | 1               |

`5 x 6` is total capacity for five extractors, each yielding six. Every biome except ocean has
total capacity for at least one food extractor at density two or more.

## Controls

- Rotation is bound to the arrow keys, and to dragging
- Zoom is bound to the wheel, and to pinching
- Reset is bound to `R`, and to a control
- The drawing is bound to `T`, and to a control for each drawing.
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

**to** sean · **status** **vetted** 2026-09-03 · **evidence** both drawings exist and are photographed; ids on the practical one only, poles marked, camera shared

- **In** - `spec/planet.md`, *the planet is drawn either practically or realistically, and the
  user can change which*
- **Vetted when** - switching between them moves nothing: the planet is at the same rotation and
  zoom afterwards, and every territory covers the same pixels

### R-2 - Terrain that crosses boundaries

**to** sean · **status** **vetted** 2026-09-03 · **evidence** one continuous field sampled per point; coastlines cross territory boundaries in the photograph

- **In** - `spec/planet.md`, *the terrain of the realistic drawing is continuous*
- **Vetted when** - no line visible in the realistic drawing coincides with a territory boundary,
  and terrain visibly varies within a single territory

### R-3 - A division that cannot be seen

**to** sean · **status** **vetted** 2026-09-03 · **evidence** no seam and no boundary in the realistic drawing; `Drawn.labels` is zero there

- **In** - `spec/planet.md`, *nothing in the terrain reveals how the sphere was divided*
- **Vetted when** - a person who has not seen the tessellation is shown the realistic drawing and
  cannot mark where a five-neighbour territory is, beyond the two at the poles

### R-4 - A biome per territory

**to** sean · **status** **vetted** 2026-09-03 · **evidence** `biomes_of` gives every territory one, and `join_the_land` keeps land connected

- **In** - `spec/planet.md`, *each territory has a biome*, and *a territory's biome is what the
  terrain gives it*
- **Vetted when** - `show territory 5` names a biome, and no other biome covers more of that
  ground in the realistic drawing

### R-5 - Terrain resolved as finely as it is shown

**to** sean · **status** **vetted** 2026-09-03 · **evidence** 400,000 sub-triangles, blended in parameter space

- **In** - `spec/planet.md`, *nothing of how a drawing is made is visible in it*
- **Vetted when** - at the default camera, no facet, band or flat wash betrays how the surface was
  built, and the finest visible detail is terrain

### R-6 - The loop can be played through

**to** code - **status** open - **cited** `faafb5f`, `2f38241` - **nothing in the code blocks it**, 2026-09-05

- **In** - `spec/control.md`, *a player wins by launching an Ark from a fully exploited planet*
- **Vetted when** - A scenario reaches a fully exploited planet and launches an Ark, on the
  definitions and the machinery of the main scenario - **which I have vetted, and which is what
  makes this one worth trusting.**
- **Nothing in the code blocks it, as of 2026-09-05.** `C-7` was withdrawn on the 31st; `C-11` landed in `05097a6` and a territory's stores carry; `C-9` landed in `ec96bc9` and *fully exploited* is decidable from a territory alone. **What is now in question is not whether it can be played but how much of it has to be** - see the proposal queue.

## Open questions
