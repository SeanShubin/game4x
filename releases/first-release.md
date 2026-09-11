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
7. Build and launch an Ark, which is one act

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
| **deposit**   | what a territory's ground offers of one resource, and how richly                                             |
| **adjacency** | two places that share an edge, held by the thing that holds them                                             |
| **game**      | every thing is in it, and it is the one thing that is in nothing                                             |
| **fertility** | a citizen's capacity to raise one more, spent by raising one and renewed each turn                           |

## Families

| Family       | Members             |
| ------------ | ------------------- |
| **thing**    | every kind above    |
| **unit**     | ark, pioneer        |
| **resource** | food, metal, energy |
| **place**    | territory, orbit    |

## Where things are

Every thing but the game is in another thing, and this release has three sorts of capacity.

| Container                               | Holds                         | Up to                            |
| --------------------------------------- | ----------------------------- | -------------------------------- |
| a territory's total capacity for a kind | that kind                     | its total capacity for that kind |
| a store                                 | the resource it was built for | 10                               |
| a unit's tank                           | energy                        | the unit's fuel                  |

There are twelve territories and twelve orbits. An orbit holds units and nothing else.

**The dump is a data file, so `spec/console.md` governs what it may say** - a map from a
description to a quantity, where a thing appears inside what holds it and nothing states its
container. **Two things are true of this release in particular.**

- **Territory adjacency is stated once, and orbital adjacency is derived from it** - an orbit is
  next to its territory and to the orbits above that territory's neighbours, so stating it would
  be a second copy that can disagree
- **Which store holds which unit of a resource is not recorded in this release**, so a territory's
  amount of a resource is stated against the sum of its stores' capacities

**The check is that the dump reads back into the state it came from.** A count of fields is not,
because a plausible subset passes it.

## Traits

Where a trait admits a closed set of values, its **Values** cell names them, or says where they
are listed.

| Trait              | Of                                      | Values                                    | Stored or derived                                |
| ------------------ | --------------------------------------- | ----------------------------------------- | ------------------------------------------------ |
| **kind**           | every thing                             | one of the kinds                          | stored                                           |
| **id**             | a thing that must be named individually | a number, unique among things of its kind | stored                                           |
| **ready**          | whatever readies                        | yes or no                                 | stored                                           |
| **resource**       | an extractor or a store                 | one of the resources                      | stored                                           |
| **force**          | citizen, garrison, ark, pioneer         | a number                                  | stored                                           |
| **fuel**           | a unit                                  | how much energy its tank holds            | stored                                           |
| **upkeep**         | a thing with upkeep                     | food per turn                             | stored                                           |
| **metal in it**    | whatever is built                       | a number                                  | derived: its binding plus the metal in its parts |
| **density**        | a deposit                               | a number                                  | stored                                           |
| **total capacity** | a deposit                               | a number                                  | stored                                           |
| **control**        | a territory                             | held by a player, or unclaimed            | derived: a citizen of that player is there       |
| **biome**          | a territory                             | one of the biomes                         | stored                                           |
| **nature**         | a territory                             | a number                                  | stored                                           |
| **from**           | an adjacency                            | a place                                   | stored                                           |
| **to**             | an adjacency                            | a place                                   | stored                                           |
| **keeps**          | thing                                   | the number of turns it will last          | stored                                           |
| **surplus**        | food                                    | yes or no                                 | derived: left after every upkeep was paid        |
| **unpaid**         | a thing with upkeep                     | yes or no                                 | derived: its upkeep was not met                  |
| **phase**          | the game                                | design or play                            | stored                                           |
| **movable**        | whatever moves                          | yes or no                                 | stored                                           |
| **spent**          | a citizen                               | yes or no                                 | stored                                           |

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
| **pioneer**   | a capacity of 2                                          |
| **labor**     | the citizens that make it, one each per turn             |
| **fertility** | the citizens that make it, one each per turn             |
| **food**      | the things in it that hold it, and it keeps for one turn |
| **metal**     | the things in it that hold it                            |
| **energy**    | the things in it that hold it                            |

A territory declares **no limit** for a resource. It declares capacity for the things that hold
them - a store holds what it was built to hold, and an extractor holds nothing. **A raw material
is in one of three states: its source, disorder, or a container**, and a resource is a raw
material. **`labor` and `fertility` are transient**: neither has a source and nothing holds
either, so both are always in disorder. **What is constructed is never in disorder** - a unit, a
structure or a container, whatever is holding it. **What is in disorder may be spent the turn it
is made and does not survive that turn's end**, and a raw material returns to its source.

## Units and structures

| Thing         | Force | Fuel | Upkeep          | Costs to produce               | Binding | Crosses      | Requires | Readies | Movable |
| ------------- | ----- | ---- | --------------- | ------------------------------ | ------- | ------------ | -------- | ------- | ------- |
| **citizen**   | 1     |      | 1 food per turn |                                |         |              |          | yes     |         |
| **garrison**  | 0     |      |                 | 1 labor, 1 metal               | 1       |              |          |         |         |
| **extractor** |       |      |                 | 1 labor, 1 metal               | 1       |              |          | yes     |         |
| **yard**      |       |      |                 | 1 labor, 15 metal              | 15      |              |          |         |         |
| **store**     |       |      |                 | 1 labor, 1 metal               | 1       |              |          |         |         |
| **ark**       | 2     | 2    |                 | 3 metal, 12 energy, 2 citizens | 3       | orbit border | a Yard   | yes     | yes     |
| **pioneer**   | 2     | 2    |                 | 3 metal, 6 energy, 2 citizens  | 3       | border       |          | yes     | yes     |

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

**In** - `spec/turn.md`, *ending a turn: everything with upkeep pays it; then a population grows
on surplus food or starves for want of it; what expires expires, and what was not kept in order
is lost; and everything becomes ready again*.

The player's recipes fire when the player chooses them. The world's fire when the turn ends, in
that order: `upkeep`, then `bear`, `breed` and `renew`, then `perish`, then `age`, then `spoil`,
then `stow` and `discard`, then `refresh`. The rows below are in that order.

| Recipe              | Owner  | Role    | Qty                                  | Kind      | Traits                                        | Where                    |
| ------------------- | ------ | ------- | ------------------------------------ | --------- | --------------------------------------------- | ------------------------ |
| **deploy ark**      | player | require | 1                                    | territory |                                               | `$where`                 |
|                     |        | consume | 1                                    | ark       |                                               | the orbit above `$where` |
|                     |        | limit   | 0                                    | garrison  |                                               |                          |
|                     |        | produce | 1                                    | garrison  |                                               |                          |
|                     |        | produce | 2                                    | citizen   |                                               |                          |
|                     |        | produce | 1                                    | extractor | food                                          |                          |
|                     |        | produce | 1                                    | extractor | metal                                         |                          |
|                     |        | produce | 1                                    | store     | food                                          |                          |
|                     |        | produce | 1                                    | store     | metal                                         |                          |
| **move**            | player | require | 1                                    | place     |                                               | `$from`                  |
|                     |        | require | 1                                    | place     | joined to `$from` by an edge the unit crosses | `$to`                    |
|                     |        | consume | 1                                    | unit      | ready                                         | `$from`                  |
|                     |        | consume | 1                                    | energy    |                                               | that unit                |
|                     |        | produce | 1                                    | unit      | not ready                                     | `$to`                    |
| **found by land**   | player | consume | 1                                    | pioneer   |                                               |                          |
|                     |        | limit   | 0                                    | garrison  |                                               |                          |
|                     |        | produce | 1                                    | garrison  |                                               |                          |
|                     |        | produce | 2                                    | citizen   |                                               |                          |
|                     |        | produce | 1                                    | extractor | food                                          |                          |
|                     |        | produce | 1                                    | extractor | metal                                         |                          |
|                     |        | produce | 1                                    | store     | food                                          |                          |
|                     |        | produce | 1                                    | store     | metal                                         |                          |
| **build extractor** | player | consume | 1                                    | labor     |                                               |                          |
|                     |        | consume | 1                                    | metal     |                                               |                          |
|                     |        | produce | 1                                    | extractor | `$resource`                                   |                          |
| **build store**     | player | consume | 1                                    | labor     |                                               |                          |
|                     |        | consume | 1                                    | metal     |                                               |                          |
|                     |        | produce | 1                                    | store     | `$resource`                                   |                          |
| **build yard**      | player | consume | 1                                    | labor     |                                               |                          |
|                     |        | consume | 15                                   | metal     |                                               |                          |
|                     |        | produce | 1                                    | yard      |                                               |                          |
| **produce pioneer** | player | consume | 3                                    | metal     |                                               |                          |
|                     |        | consume | 6                                    | energy    |                                               |                          |
|                     |        | consume | 2                                    | citizen   |                                               |                          |
|                     |        | produce | 1                                    | pioneer   |                                               |                          |
| **launch ark**      | player | require | 1                                    | territory |                                               | `$where`                 |
|                     |        | consume | 3                                    | metal     |                                               |                          |
|                     |        | consume | 12                                   | energy    |                                               |                          |
|                     |        | consume | 2                                    | citizen   |                                               |                          |
|                     |        | require | 1                                    | yard      |                                               |                          |
|                     |        | produce | 1                                    | ark       |                                               | the orbit above `$where` |
| **create labor**    | player | consume | 1                                    | citizen   | ready                                         |                          |
|                     |        | produce | 1                                    | citizen   | not ready                                     |                          |
|                     |        | produce | 1                                    | labor     |                                               |                          |
| **work**            | player | require | 1                                    | territory |                                               | `$where`                 |
|                     |        | consume | 1                                    | labor     |                                               |                          |
|                     |        | consume | 1                                    | extractor | ready                                         |                          |
|                     |        | produce | 1                                    | extractor | not ready                                     |                          |
|                     |        | produce | `$where`'s density for that resource | resource  |                                               |                          |
| **upkeep**          | world  | require | 1                                    | citizen   |                                               |                          |
|                     |        | consume | 1                                    | food      |                                               |                          |
| **bear**            | world  | consume | 1                                    | citizen   | fertile                                       |                          |
|                     |        | produce | 1                                    | citizen   | spent                                         |                          |
|                     |        | produce | 1                                    | fertility |                                               |                          |
| **breed**           | world  | consume | 1                                    | fertility |                                               |                          |
|                     |        | consume | 1                                    | food      |                                               |                          |
|                     |        | produce | 1                                    | citizen   |                                               |                          |
| **renew**           | world  | consume | 1                                    | citizen   | spent                                         |                          |
|                     |        | produce | 1                                    | citizen   | fertile                                       |                          |
| **perish**          | world  | consume | 1                                    | citizen   | whose upkeep is unpaid                        |                          |
| **age**             | world  | consume | 1                                    | thing     | keeps at least 1                              |                          |
|                     |        | produce | 1                                    | thing     | keeps one less                                |                          |
| **spoil**           | world  | consume | 1                                    | thing     | keeps 0                                       |                          |
| **stow**            | world  | consume | 1                                    | metal     |                                               |                          |
|                     |        | produce | 1                                    | metal     |                                               | a store for metal        |
| **stow**            | world  | consume | 1                                    | energy    |                                               |                          |
|                     |        | produce | 1                                    | energy    |                                               | a store for energy       |
| **discard**         | world  | consume | 1                                    | metal     |                                               |                          |
| **discard**         | world  | consume | 1                                    | energy    |                                               |                          |
| **discard**         | world  | consume | 1                                    | labor     |                                               |                          |
| **discard**         | world  | consume | 1                                    | fertility |                                               |                          |
| **refresh**         | world  | consume | 1                                    | thing     | not ready                                     |                          |
|                     |        | produce | 1                                    | thing     | ready                                         |                          |

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

**to** code - **status** open - **cited** `faafb5f`, `2f38241`, `53bd58d`, `58c8b4a`, `92786a9` - **nothing in the code blocks it**, 2026-09-05

- **In** - `spec/control.md`, *a player wins by launching an Ark from a fully exploited planet*
- **Vetted when** - A scenario reaches a fully exploited planet and launches an Ark, on the
  definitions and the machinery of the main scenario - **which I have vetted, and which is what
  makes this one worth trusting.**
- **Nothing in the code blocks it, as of 2026-09-05.** `C-7` was withdrawn on the 31st; `C-11` landed in `05097a6` and a territory's stores carry; `C-9` landed in `ec96bc9` and *fully exploited* is decidable from a territory alone. **What is now in question is not whether it can be played but how much of it has to be** - see the proposal queue.


### R-7 - Each recipe can be confirmed on its own

**to** sean - **status** **built** 2026-09-08, **and its report moved under you on 2026-09-11** - **cited** `747de8a`, `025eecb`, `fe3dc9b`, `3bab70e`, `2e9a06e`, `dd93bd1`, `ca2309e`, `e42d37c` - **evidence reported by the code lane and recorded here rather than by the lane that built it.** `reports/recipes.md` shows every recipe with a state before, the command that fires it and the state after, in the scenario's notation and generated by running it; `tests/worked.rs` fails if a recipe has no example. **Verified in the report by this lane rather than taken from the report: 24 sections.** **What you read on 2026-09-08 is not what is there now**, so the earlier reading does not carry. The saturating rewrite reached the model: `grow` is gone, so the two examples that showed its expression turning out both ways are now `breed`'s - food the lesser, and citizens the lesser. **The world's recipes are still shown once together on one `{end-turn}`, and that one ending now fires ten rather than six.** Your *vetted when* says *four of them cannot act alone at all*, which was written when there were six; **that number wants your eye while you re-read**, and it is yours to change or leave. `S-88`, `C-84`

- **In** - `docs/process.md`, *the definitions and the commands are enough to derive the data dump
  by hand*, applied to one recipe rather than to a whole scenario
- **Vetted when** - `reports/recipes.md` shows, beside each recipe's rule, a state before it fires,
  the command that fires it, and the state after - **in the same notation as the scenario's
  expected data**, holding only what that recipe touches, and generated by running it. I can derive
  the after from the rule and the before by hand, and a recipe whose quantity is an expression
  shows one example for each way the expression turns out. **The world's recipes are shown once,
  together, on `{end-turn}`**, because no command fires one of them alone and four of them cannot
  act alone at all


### R-8 - I can see which kinds behave alike

**to** sean - **status** **built** 2026-09-07 - **cited** `d938c8a`, `79d8f1d`, `14b02d2`, `dd93bd1` - **evidence** a signature per kind computed from the tables, reported by the code lane. **Ready to vet again, 2026-09-08 - `14b02d2`.** `C-71` is fixed: a trait declared of a family now reaches its members, so `fuel` reaches ark and pioneer and `keeps` reaches all sixteen. **It closed a second defect in the half I had called correct** - `thing` is written *every kind above*, a membership rather than a list, so both joins split on commas and missed it, and the world's five recipes named nothing at all. **The conclusion is unchanged and now computed from the right inputs**: no two of the sixteen behave alike, over 120 pairs. `S-78`

- **In** - `docs/process.md`, *I insist that the AI make its work verifiable to a human*, applied
  to a kind's behaviour rather than to a scenario's outcome
- **Vetted when** - `reports/catalog.md` gives each kind a **signature**: the traits it carries and
  every *(recipe, role)* pair that names it. **Kinds with the same signature are shown together**,
  and the signature is computed from the release's tables rather than written by anyone. I can scan
  the groups, see that two kinds behave alike, and have a name to grep for when I want the detail

### R-9 - I can browse the reports without a script running

**to** sean - **status** **built** 2026-09-07 - **cited** `dc6d341` - **evidence** every reference a link, a diffable sibling for every view, no page carrying a script, two shared stylesheets, and a page plus a sibling for each of the twelve territories. `S-64` built with it

- **In** - `docs/process.md`, *presentations are generated from data*, and *I insist that the AI
  make its work verifiable to a human*
- **Vetted when** - every reference in a report is a link I can follow to the thing it names;
  every generated view has a **diffable sibling** beside it, as `graph.html` has `graph.txt`; and
  **no page needs JavaScript to be read** - a view that filters is a page that was generated, so
  the filter is a URL rather than a click
## Open questions
