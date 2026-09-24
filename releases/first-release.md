# Release: First Release

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Releases](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)

## Scope

- A single planet
- Tiny, which is 12 territories
- Each territory is self-contained. No resource and no citizen crosses a territory boundary
- A mobile unit may move across a boundary to start another self-contained territory
- The twelve territories and the thirty adjacencies between them are generated from the
  twelve-faced Goldberg polyhedron rather than stated by hand

### Territory resources

**In** - `spec/planet.md`, *for each resource, a territory has capacity for some number
of extractors, and a density that each of them yields*.

The twelve territories are fixed, each chosen to exercise a different consequence of the rules.
Every territory has capacity for at least one food extractor.

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

1. Start with an ark in orbit
2. Land it, and develop the territory it lands on
3. Reach a second territory, build a Yard there, and launch an ark from it

## Kinds

| Kind          | What it is                                                                         |
| ------------- | ---------------------------------------------------------------------------------- |
| **citizen**   | a person: provides labor, eats, and grows on surplus                               |
| **extractor** | built for one resource, and worked to produce it                                   |
| **yard**      | where an Ark is produced                                                           |
| **store**     | built to hold one resource, and holds nothing else                                 |
| **ark**       | carries a landing, and can invade from orbit                                       |
| **pioneer**   | founds a territory                                                                 |
| **food**      | eaten by citizens; expires                                                         |
| **metal**     | what things are built from; drawn from the planet, and conserved once above ground |
| **energy**    | what moves things; neither conserved nor expiring                                  |
| **labor**     | what working a machine takes; a citizen provides it each turn                      |
| **territory** | a place things are in, which has a density and a capacity per resource             |
| **orbit**     | a place above one territory, which holds units and nothing else                    |
| **deposit**   | what a territory's ground offers of one resource, and how richly                   |
| **adjacency** | two places that share an edge, held by the thing that holds them                   |
| **game**      | every thing is in it, and it is the one thing that is in nothing                   |
| **fertility** | a citizen's capacity to raise one more, spent by raising one and renewed each turn |

## Families

| Family       | Members             |
| ------------ | ------------------- |
| **thing**    | every kind above    |
| **unit**     | ark, pioneer        |
| **resource** | food, metal, energy |
| **place**    | territory, orbit    |

## Where things are

Every thing but the game is in another thing, and this release has two sorts of thing that
give a place room.

| Thing         | Gives room for                | Up to           |
| ------------- | ----------------------------- | --------------- |
| a store       | the resource it was built for | 10              |
| a unit's tank | energy                        | the unit's fuel |

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

| Trait           | Values                                                            | Belongs to |
| --------------- | ----------------------------------------------------------------- | ---------- |
| **id**          | an identity                                                       | each thing |
| **moving**      | a number                                                          | each thing |
| **laboring**    | a number                                                          | each thing |
| **working**     | a number                                                          | each thing |
| **bearing**     | a number                                                          | each thing |
| **resource**    | one of the resources                                              | each thing |
| **strength**    | a number                                                          | the kind   |
| **fuel**        | how much energy its tank holds                                    | the kind   |
| **upkeep**      | food per turn                                                     | the kind   |
| **binding**     | a number: the metal the recipe that makes it consumes             | the kind   |
| **metal in it** | a number: its binding plus the metal in its parts                 | the kind   |
| **density**     | a number                                                          | each thing |
| **capacity**    | a number                                                          | each thing |
| **occupied**    | a number                                                          | each thing |
| **free**        | a number: its capacity less what it holds                         | each thing |
| **control**     | held by a player, or unclaimed: a citizen of that player is there | each thing |
| **from**        | a place                                                           | each thing |
| **to**          | a place                                                           | each thing |
| **keeps**       | the number of turns it will last                                  | each thing |
| **surplus**     | a number: left after every upkeep was paid                        | the kind   |
| **paid**        | a number                                                          | each thing |
| **phase**       | design or play                                                    | each thing |
| **movable**     | a number                                                          | the kind   |

Food is made with `keeps` 1.

## What bounds a kind in a territory

| Kind          | Bounded by                                               |
| ------------- | -------------------------------------------------------- |
| **citizen**   | the food produced here, through upkeep                   |
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
is in one of three states: its source, disorder, or held by something that declares a limit for
it**, and a resource is a raw material. **`labor` and `fertility` are transient**: neither has a
source and nothing declares a limit for either, so both are always in disorder. **What is
constructed is never in disorder** - a unit, a structure or a container, whatever is holding it.
**What is in disorder may be spent the turn it is made and does not survive that turn's end**, and
a raw material returns to its source.

## Units and structures

| Thing         | Strength | Fuel | Upkeep          | Crosses      | Readies               | Movable |
| ------------- | -------- | ---- | --------------- | ------------ | --------------------- | ------- |
| **citizen**   | 1        |      | 1 food per turn |              | bearing 1, laboring 1 |         |
| **extractor** |          |      |                 |              | working 1             |         |
| **yard**      |          |      |                 |              |                       |         |
| **store**     |          |      |                 |              |                       |         |
| **ark**       | 2        |      |                 | orbit border | moving 1              | 1       |
| **pioneer**   | 2        | 2    |                 | border       | moving 1              | 1       |

An Ark can invade land from orbit. Nothing outside this table
readies.

## Recipes

The recipe table has seven columns: **Recipe**, **Owner**, **Role**, **Qty**, **Kind**, **Traits**
and **Where**.

**Owner** is `player` or `world`. **Role** is one of `require`, `limit`, `consume`, `produce` or
`put`: a requirement must be present and is not taken, a limit is a maximum that must not be
exceeded, a consumption is taken, a production is made, and **a put names a thing that is already
there and says what is true of it afterwards - the same thing and not a new one, so what has an
identity keeps it.** A put has no quantity, because nothing is made or taken. **Qty** is a whole
number or an expression. **Kind** is the kind or the family alone. **Traits** are the constraints on
it. **Where** is the place the row is about, and a blank means the one place the recipe acts.

A quantity is a whole number. It is written in the recipe, read from a trait of one of the
ingredients, or read from a trait of a named ingredient.

An ingredient may be given a name, written `$name`, and another ingredient may refer to it. A
recipe that names two things of the same kind must name them, because otherwise a reference has
two candidates.

A blank is not a zero. It says the row has no such number, and a quantity read from one produces
nothing.

**In** - `spec/turn.md`, *ending a turn: everything with upkeep pays it; then a population grows
on surplus food or starves for want of it; what expires expires, and what was not kept in order is
lost; and time restores every count to the number that thing's kind declares*.

The player's recipes fire when the player chooses them. The world's fire when the turn ends, in
that order: `upkeep`, then `bear` and `breed`, then `perish`, then `age`, then `spoil`, then
`stow` and `discard`, then `refresh`. The rows below are in that order.

| Recipe              | Owner  | Role    | Qty                                  | Kind      | Traits                                        | Where          |
| ------------------- | ------ | ------- | ------------------------------------ | --------- | --------------------------------------------- | -------------- |
| **deploy ark**      | player | require | 1                                    | territory |                                               | `$where`       |
|                     |        | consume | 1                                    | ark       |                                               | above `$where` |
|                     |        | produce | 2                                    | citizen   |                                               |                |
|                     |        | produce | 1                                    | extractor | food                                          |                |
|                     |        | produce | 1                                    | extractor | metal                                         |                |
| **move**            | player | require | 1                                    | place     |                                               | `$from`        |
|                     |        | require | 1                                    | place     | joined to `$from` by an edge the unit crosses | `$to`          |
|                     |        | require | 1                                    | unit      | moving at least 1                             | `$from`        |
|                     |        | put     |                                      | unit      | moving one less                               | `$to`          |
|                     |        | consume | 1                                    | energy    |                                               | `$from`        |
| **found by land**   | player | consume | 1                                    | pioneer   |                                               |                |
|                     |        | produce | 2                                    | citizen   |                                               |                |
|                     |        | produce | 1                                    | extractor | food                                          |                |
|                     |        | produce | 1                                    | extractor | metal                                         |                |
| **build extractor** | player | consume | 1                                    | labor     |                                               |                |
|                     |        | consume | 1                                    | metal     |                                               |                |
|                     |        | produce | 1                                    | extractor | `$resource`                                   |                |
| **build store**     | player | consume | 1                                    | labor     |                                               |                |
|                     |        | consume | 1                                    | metal     |                                               |                |
|                     |        | produce | 1                                    | store     | `$resource`                                   |                |
| **build yard**      | player | consume | 1                                    | labor     |                                               |                |
|                     |        | consume | 15                                   | metal     |                                               |                |
|                     |        | produce | 1                                    | yard      |                                               |                |
| **produce pioneer** | player | consume | 3                                    | metal     |                                               |                |
|                     |        | consume | 2                                    | energy    |                                               |                |
|                     |        | consume | 2                                    | citizen   |                                               |                |
|                     |        | produce | 1                                    | pioneer   |                                               |                |
| **launch ark**      | player | require | 1                                    | territory |                                               | `$where`       |
|                     |        | consume | 3                                    | metal     |                                               |                |
|                     |        | consume | 12                                   | energy    |                                               |                |
|                     |        | consume | 2                                    | citizen   |                                               |                |
|                     |        | require | 1                                    | yard      |                                               |                |
|                     |        | produce | 1                                    | ark       |                                               | above `$where` |
| **create labor**    | player | require | 1                                    | citizen   | laboring at least 1                           |                |
|                     |        | put     |                                      | citizen   | laboring one less                             |                |
|                     |        | produce | 1                                    | labor     |                                               |                |
| **work**            | player | require | 1                                    | territory |                                               | `$where`       |
|                     |        | require | 1                                    | extractor | working at least 1                            |                |
|                     |        | put     |                                      | extractor | working one less                              |                |
|                     |        | consume | 1                                    | labor     |                                               |                |
|                     |        | produce | `$where`'s density for that resource | resource  |                                               |                |
| **upkeep**          | world  | require | 1                                    | citizen   |                                               |                |
|                     |        | consume | 1                                    | food      |                                               |                |
|                     |        | put     |                                      | citizen   | paid at its maximum                           |                |
| **bear**            | world  | require | 1                                    | citizen   | bearing at least 1                            |                |
|                     |        | put     |                                      | citizen   | bearing one less                              |                |
|                     |        | produce | 1                                    | fertility |                                               |                |
| **breed**           | world  | consume | 1                                    | fertility |                                               |                |
|                     |        | consume | 1                                    | food      |                                               |                |
|                     |        | produce | 1                                    | citizen   |                                               |                |
| **perish**          | world  | consume | 1                                    | citizen   | paid 0                                        |                |
| **age**             | world  | require | 1                                    | thing     | keeps at least 1                              |                |
|                     |        | put     |                                      | thing     | keeps one less                                |                |
| **spoil**           | world  | consume | 1                                    | thing     | keeps 0                                       |                |
| **stow**            | world  | consume | 1                                    | metal     |                                               |                |
|                     |        | produce | 1                                    | metal     |                                               |                |
| **stow**            | world  | consume | 1                                    | energy    |                                               |                |
|                     |        | produce | 1                                    | energy    |                                               |                |
| **discard**         | world  | consume | 1                                    | metal     |                                               |                |
| **discard**         | world  | consume | 1                                    | energy    |                                               |                |
| **discard**         | world  | consume | 1                                    | labor     |                                               |                |
| **discard**         | world  | consume | 1                                    | fertility |                                               |                |
| **refresh**         | world  | put     |                                      | unit      | moving at its maximum                         |                |
| **refresh**         | world  | put     |                                      | citizen   | laboring at its maximum                       |                |
| **refresh**         | world  | put     |                                      | citizen   | bearing at its maximum                        |                |
| **refresh**         | world  | put     |                                      | extractor | working at its maximum                        |                |
| **renew**           | world  | require | 1                                    | citizen   |                                               |                |
|                     |        | put     |                                      | citizen   | paid 0                                        |                |

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
- **Biomes were delivered before they were cut.** `P-522` puts them out of scope for this
  release, and this capability was observed on 2026-09-03 against a drawing that still exists.
  **The cut is about which rules are built and not about what was seen**, so the vetting stands.

### R-5 - Terrain resolved as finely as it is shown

**to** sean · **status** **vetted** 2026-09-03 · **evidence** 400,000 sub-triangles, blended in parameter space

- **In** - `spec/planet.md`, *a drawing never betrays how it was made. A viewer sees the planet,
  never the process*
- **Vetted when** - at the default camera, no facet, band or flat wash betrays how the surface was
  built, and the finest visible detail is terrain

### R-6 - The loop can be played through

**to** sean - **status** **built** 2026-09-11 - **cited** `faafb5f`, `2f38241`, `53bd58d`, `58c8b4a`, `92786a9`, `d7ed1e8`, `d7e6469`, `3292266`, `5e97b79` - **evidence reported by the code lane and recorded here rather than by the lane that built it**, and **every clause re-run by this lane rather than taken from the report.** `{deploy-ark territory:1}` at `play.4x:19`, `{found-by-land territory:2}` at `:154`, `{launch-ark territory:1}` at `:164`. **The scenario's commands changed by one line on 2026-09-12, and `S-26` says not to do that under you without saying so.** `P-460` gave `move` its second place, so turn 8 reads `{move unit:pioneer from:1 to:2}` in `scenario/commands/play.4x` and the same in `spread.4x`. **`scenario/expected/play.4x` does not change at all**, so what this capability rests on - the loop playing through, and every recipe firing - is untouched and was re-measured rather than assumed. **And its expected state moved again when `P-474` was built, by thirty-four lines and no others.** Every deposit entry carried `room` where it had carried `total-capacity`: `{deposit density:4 resource:food room:0}` in territory 1, where three food extractors stand on ground with capacity for three. **And it moved a third time when `P-475`, `P-476`, `P-477` and `P-478` were built**, again by thirty-four lines and no others. Territory 1's food deposit reads `{deposit capacity:3 density:4 free:0 occupied:3 resource:food}`. **Every clause re-run by this lane rather than taken from the code lane's report**: 34 deposit entries, `capacity - occupied = free` on all 34, `occupied` equal to the extractors standing in that territory on all 34, five deposits with anything on them, and no `room:` anywhere. `cargo test --workspace` is green, where two tests in `crates/game-console/tests/declare.rs` were red on a literal 24 against a count of 26. **Nothing about what the game does moved** - the same 34 deposits by territory, resource and density, and `free` equal to the old `room` everywhere. **Nothing about what the game does moved** - the pioneer that went is the pioneer that went, and the check counted thirty-four deposit lines changed and zero of anything else. **The code lane predicted the shape of that diff before making it** - twenty-nine deposits untouched by any extractor, five not - and the diff agreed, which is why it was willing to reseed the file at all.
`every_recipe_the_release_declares_fires_while_the_scenario_runs` passes, and
`the_committed_scenario_launches_an_ark_and_does_not_finish_the_planet` passes with it. **It does
not win, which is the last clause rather than a shortfall.**

**The fourth clause needed a test that did not exist, and the reason is worth keeping.** Two tests
already covered the halves - ten player recipes and eleven world - and **neither asked whether ten
and eleven are all of them.** A recipe whose `Owner` cell said anything else would have been in
neither population, both would have stayed green, and the clause this capability now turns on would
have been false with nothing saying so. The new test counts the declared set entire, asserts the two
owners partition it, and reads what fired rather than the scenario's text. **Poisoned by dropping
`muster` from what fires: it reports `["muster"]` rather than passing.**

**What it cost, visible in the reseeded data rather than argued.** `P-427` took the two stores out
of founding, so territory 2 has no stores at all, territory 1 has one fewer of each, and ten metal
that used to be kept is lost at a turn's end. `scenario/expected/play.4x` was reseeded under
`P-225`'s protocol and is **unreviewed, and says so**.


- **In** - `spec/control.md`, *a player wins by deploying an Ark to one territory and launching
  an Ark from a different one*
- **Vetted when** - the scenario takes a first territory from orbit, takes a second by land, and
  launches an Ark; and **every recipe in the release fires at least once while it runs**, measured
  by what fired rather than by what the file says. **It does not win, and that is the win condition
  working**: victory takes a launch from a territory other than the one the Ark deployed to, and
  this scenario deploys to territory 1 and launches from territory 1

- **Nothing in the code blocks it, as of 2026-09-05.** Earlier doubts were settled: a
  territory's stores carry, and the output a territory can reach is decidable from the
  territory alone. **What is now in question is not whether it can be played but how much of
  it has to be.**
- **Measured 2026-09-11.** Running `setup.4x`, `{start}` and `play.4x` and asking the model
  gives **twelve claimable territories, two founded, none at maximum output**. It launches an
  Ark at line 164. **These numbers were taken against an older *vetted when*** that asked for a
  fully exploited planet, which the release no longer requires.
- **The gap is not a near miss**, which is the part a summary loses. `tests/fully_exploited.rs:410`
  derives **57 buildings**, which is **114 commands and counts nothing else** - each building is the
  labor that pays for it and the building, read off the predicate at `:404`. **It is a floor**, and
  three things sit on top of it, each read from the release rather than recalled.
- **Ten territories to found, and each wants a founding unit rather than a pioneer.** `found by
  land` consumes 1 pioneer and `deploy ark` consumes 1 ark; `play.4x` uses one of each, territory 1
  by ark and territory 2 by pioneer. **Which one is the player's choice and the release leaves it
  open.**
- **A founding unit costs citizens, and that is the cost that matters.** `produce pioneer` consumes
  **3 metal, 2 energy and 2 citizens**; `launch ark` consumes 3 metal, 12 energy and 2 citizens and
  requires a Yard. **So founding competes with the population rather than costing resources beside
  it** - ten foundings is twenty citizens spent against a target of **144**, summed from the
  per-territory figures `DERIVED_BY_HAND` states. Read as metal and energy alone it looks like a
  cost paid out of production; it is paid out of the goal.
- **Distance costs turns, not only energy.** `moving` is **0 or 1** and `move` requires *moving at
  least 1*, puts the unit back with one less, and consumes 1 energy. **Only `refresh` restores it,
  at a turn's end** - so a unit moves once per turn, and a territory *n* steps from a founded one is
  *n* turns away. `play.4x` spends **10** `{end-turn}`s reaching two founded territories.
- **No total is estimated anywhere here**, against a scenario that is 133 commands and has founded
  two of twelve.

 **Re-run by this lane rather than taken from
  the report**: `the_committed_scenario_launches_an_ark_and_does_not_finish_the_planet` passes on
  `(12, 2, 0)`.
- **One thing this proved that nothing had asserted.** The scenario reaches a second
  settlement and launches anyway from the first, so a launch is not a victory by itself - and
  that is now a check rather than an observation.



### R-7 - Each recipe can be confirmed on its own

**to** sean - **status** **built** 2026-09-08, **and its report moved under you on 2026-09-11** - **cited** `747de8a`, `025eecb`, `fe3dc9b`, `3bab70e`, `2e9a06e`, `dd93bd1`, `ca2309e`, `e42d37c` - **evidence reported by the code lane and recorded here rather than by the lane that built it.** `reports/recipes.md` shows every recipe with a state before, the command that fires it and the state after, in the scenario's notation and generated by running it; `tests/worked.rs` fails if a recipe has no example. **Verified in the report by this lane rather than taken from the report: 24 sections.** **What you read on 2026-09-08 is not what is there now**, so the earlier reading does not carry. The saturating rewrite reached the model: `grow` is gone, so the two examples that showed its expression turning out both ways are now `breed`'s - food the lesser, and citizens the lesser. **The world's recipes are still shown once together on one `{end-turn}`, and that one ending now fires ten rather than six.** Your *vetted when* says *four of them cannot act alone at all*, which was written when there were six; **that number wants your eye while you re-read**, and it is yours to change or leave. `S-88`, `C-84` **One line of the report moved on 2026-09-12 and it is the line that made it ambiguous.** `P-460` made a command bind every place a recipe leaves open, so the scenario's move reads `{move unit:pioneer from:1 to:2}` where it read `{move unit:pioneer territory:2}`. **Every state before and after is byte-identical** - the pioneer that went is the one that always went, and the command now says which. **One of the three things this capability names is the command**, so it moved to be more correct rather than merely changing, and only one of the recipes is affected. This lane first reported `R-7` untouched by checking the Recipes table, **which is not what the capability names**; the code lane measured the report and handed back the diff. `C-101`

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

**to** sean - **status** **built** 2026-09-07 - **cited** `d938c8a`, `79d8f1d`, `14b02d2`, `dd93bd1`, `d617b9a`, `08a77e3`, `d5f565a`, `883901a` - **evidence** a signature per kind computed from the tables, reported by the code lane. **Ready to vet again, 2026-09-08 - `14b02d2`.** `C-71` is fixed: a trait declared of a family now reaches its members, so `fuel` reaches ark and pioneer and `keeps` reaches all sixteen. **It closed a second defect in the half I had called correct** - `thing` is written *every kind above*, a membership rather than a list, so both joins split on commas and missed it, and the world's five recipes named nothing at all. **The conclusion is unchanged and now computed from the right inputs**: no two of the sixteen behave alike, over 120 pairs. `S-78`

**Its report moved under you again on 2026-09-12, and three of its numbers were already wrong.** `P-466` took three columns out of *Units and structures* - `Costs to produce`, `Binding` and `Requires`, each the Recipes table said twice - `P-465` changed eight `Values` cells and two `Movable` cells, and `P-470` gave every kind its trait names in `spec/data/kinds.4x`. **A signature is computed from those tables, so every signature is computed from different inputs than the ones you would have read.**

**And three numbers in this line were stale before today.** The release declares **eighteen** kinds, not sixteen, and eighteen make **153** pairs, not 120 - `fertility` and `force` arrived after 2026-09-08. **The world has eleven recipes, not five**: `P-414` added `muster`, `stand` and `discard`, and the saturating rewrite added the rest. **All three were true when written and have read the same ever since**, which is the failure `docs/notes/nothing-removes.md` is about, found in the item that note was written beside.

**Three stale numbers in one status line is a pattern rather than an accident.** A capability's status is prose about a moment, and a number in it is a second form of a fact the tables hold - `spec/invariants.md` -> *A fact is stated once*. **Nothing re-derives these**, and the check that covers what `docs/` says about the release does not reach what the release says about itself.

**The conclusion survives and was re-derived rather than carried over**: the traits each kind carries, joined to every *(recipe, role)* pair naming it, give **eighteen distinct signatures over eighteen kinds** - so no two behave alike, over 153 pairs. **Its evidence moved three times on 2026-09-12 and then settled.** `P-465`, `P-466` and `P-470` changed the tables a signature was computed from; `P-473` changed it to compute from `spec/data/` instead. **Between the first and the last, `reports/catalog.md` understated seven of the eighteen kinds** - an Ark showed four traits and carries eight - because four cells of the deleted *Of* column described rather than named. **This says what happened rather than what is**: a present tense about another lane's generated file goes stale the moment they regenerate it, which this line did twice in one day.

**Re-derive rather than trust this line: every kind's *Traits* line in `reports/catalog.md` should hold what `spec/data/kinds.4x` states for that kind, plus every trait declared `of:thing`.** When last run it held for all eighteen, an Ark showed eight, and the page said **seven** kinds collide on traits alone - **which is the number the code lane derived by hand from `kinds.4x` before the code produced it from the files.** A prediction made from one source meeting an implementation built from another is the strongest evidence anything got that day.

- **In** - `docs/process.md`, *I insist that the AI make its work verifiable to a human*, applied
  to a kind's behaviour rather than to a scenario's outcome
- **Vetted when** - `reports/catalog.md` gives each kind a **signature**: the traits it carries and
  every *(recipe, role)* pair that names it. **Kinds with the same signature are shown together**,
  and the signature is computed from `spec/data/` rather than written by anyone. I can scan
  the groups, see that two kinds behave alike, and have a name to grep for when I want the detail

### R-9 - I can browse the reports without a script running

**to** sean - **status** **built** 2026-09-07 - **cited** `dc6d341` - **evidence** every reference a link, a diffable sibling for every view, no page carrying a script, two shared stylesheets, and a page plus a sibling for each of the twelve territories. `S-64` built with it

- **In** - `docs/process.md`, *presentations are generated from data*, and *I insist that the AI
  make its work verifiable to a human*
- **Vetted when** - every reference in a report is a link I can follow to the thing it names;
  every generated view has a **diffable sibling** beside it, as `graph.html` has `graph.txt`; and
  **no page needs JavaScript to be read** - a view that filters is a page that was generated, so
  the filter is a URL rather than a click

### R-10 - I can read a generated drawing in the theme I use

**to** sean - **status** **built** 2026-09-12 - **cited** `7b4761f`, `8fd18d9` - **evidence reported by the code lane and recorded here rather than by the lane that built it.** All three clauses hold. **Colour**: every label in `reports/petri.html` declares a fill, counted at 295 of 295. **Names**: every node in the net carries its own. **Parts**: `reports/petri.md` has had one drawing per recipe since it had the whole net - **what was missing was the rest of the clause**, *and it says what each part leaves out*, so a reader of one recipe met a drawing that looked like the whole of that recipe's connections.

**A part now names the recipes that reach the same places**, rather than *everything else*, which is
true and tells a reader nothing - `create labor` leaves out 30 others reaching `citizen` and `labor`,
and names them. **Computed from the arcs**, so a recipe added tomorrow appears in the parts it
touches with nobody maintaining a list. Checked over every part with the count, and **driven both
ways on one case so a pasted list fails where a computed one passes** - `C-99`.

**And the numbers in this line were stale.** It said 62 nodes and 195 arcs; the net is **31 places
and 45 transitions, 76 nodes, joined by 161 arcs** - `P-411`, `P-414`, `P-427` and `P-431` each moved
it and nothing re-counted. **The clause was never about the figure**, which is why the staleness cost
nothing here: it asks whether a reader can read it.


- **In** - `docs/process.md`, *I reject AI responses that do not read clearly and unambiguously to
  a human*, applied to a drawing rather than to prose
- **Vetted when** - every generated drawing is legible in **both** a light and a dark reader,
  because nothing in it declares a colour the theme does not supply. **Every node carries its own
  name**, and I can say what a node is without looking anything up. Where a drawing is too large
  to satisfy that whole, it is shown in parts that do, and it says what each part leaves out

### R-11 - I can reach the engine's inputs from the reports

**to** sean · **status** **built** 2026-09-14 · **cited** `287e67f`, `3b210d8` · **evidence reported by the code lane and recorded here rather than by the lane that built it**, and **the counts re-derived by this lane rather than taken from the report**. **Twenty links over three directories** - twelve in `spec/data/`, seven in `scenario/commands/` and one in `scenario/expected/` - each to a `.txt` twin the pipeline writes into `crates/game4x/dist` and nothing commits, labelled with the `.4x` path the engine actually reads. **The rendering half is measured rather than inferred**, which this lane twice said it could not claim: `spec/data/kinds.4x` and `scenario/expected/play.4x` both answered `200 application/octet-stream`, and `reports/index.html` `text/html`. So GitHub Pages was the cause and the twin is the fix. **And building it found the capability unmet a second way, which the code lane reports against itself.** The inputs section listed `spec/data/` by reading the directory and listed the scenario as a hand-written pair, naming `play.4x` twice - so `setup.4x`, `world.4x`, `biomes.4x`, `nodes.4x`, `forces.4x` and `spread.4x` were reachable from nothing at all. **Half were listed and half were remembered, in the commit that claimed the clause about listing rather than remembering.** Re-derived here: `scenario/commands/` holds seven files and all seven are linked. **It cannot be observed until it is pushed, and that is not a defect.** The twins are made at deploy and nothing commits them, so **nothing is published to follow** - a 404 today reads *not pushed* and never *not built*. **The code lane says plainly that it has not measured a `.txt` rendering on this site**, because none exists there yet; what it measured is that Pages types by extension and serves what it knows - `reports/report.css` as `text/css`, `reports/nogain.md` as `text/markdown`. **`.txt` to `text/plain` is an inference with two neighbours as evidence**, and one fetch after a push settles it.
to need the inputs to the thin engine to be reachable from `reports/index.html` (direct links or
non-canonical generated copies).*

**This invents no rule.** `spec/invariants.md` -> *The game is data* already says the data that runs
the game lives in a data file, and that **the data may be replicated in the presentation layer, and
no replication is canonical**. `R-9` already requires every reference in a report to be a link a
reader can follow. This capability is those three sentences applied to `spec/data/`, which today no
report reaches at all.

**Why it is worth a capability of its own, and it is the thin engine's reason rather than a
convenience.** `P-493` makes complexity in the data a **reading** - *if the data structure explodes
in complexity, or the data itself explodes in complexity, that tells us something needs to be
unified*. A reading nobody can see is not a reading. The inputs are 58 lines today and the instrument
only becomes useful as they grow, so the time to make them visible is before they do.

**Either form satisfies it and the choice is the code lane's**, because it is a presentation
question. A direct link to `spec/data/kinds.4x` costs nothing and shows the canonical bytes; a
generated copy reads better and must then say it is generated and not canonical, as every other
generated view does.

- **In** - `spec/invariants.md`, *The data may be replicated in the presentation layer, and no
  replication is canonical.*
- **Vetted when** - from `reports/index.html` I can reach **every file the engine reads as input**,
  in as many clicks as it takes to reach any other view, without knowing the paths beforehand, and
  **reading it is what following the link does** - it renders in the browser rather than
  downloading. A copy rather than a link **says on the page that it is generated and not
  canonical**, and says which file it came from. **Nothing the engine reads is missing from that
  page**, which is checked by listing the inputs rather than by anybody remembering to add one

  **The rendering clause is Sean's, 2026-09-14**, on finding that
  `seanshubin.github.io/game4x/spec/data/above.4x` downloads: *I want to be able to view these
  pages from the website without downloading them.* **It was always the intent and was not
  written**, which is why the capability could be built and still not deliver it. **It is about the
  published site and not the repository view** - *I don't necessarily need it to be rendered when I
  browse it as source*.

## Out of scope

Whole areas of the specification this release does not touch, so the omission reads as deliberate.
**`spec/` keeps all four**; this is scheduling and not a change to the game.

Sean, 2026-09-20: *I am cutting nature and force from this prototype because I don't think they are
necessary to vet the core game loop: start with an ark -> develop a planet -> launch an ark.*

- **Nature** - `spec/future/force.md` -> Force, *every territory has a force of nature, inherent to it*.
  Nothing resists a claim in this release, so `reclaim`, `renew` and `take` are not built
- **Force** - `spec/future/force.md` -> Producing force, and *gaining and holding ground*. No garrison,
  no muster, and `deploy ark` and `found by land` neither require force nor produce a garrison.
  **`spec/turn.md`'s *nature takes back what is no longer held* goes with it**, so a turn in this
  release has four steps rather than five
- **Biomes** - `spec/planet.md`, *each territory has a biome*. A territory's numbers are stated
  directly in *Territory resources* rather than guided by a biome
- **The rule editor** - `spec/interface.md` -> Surfaces. It is for automating the game to the
  player's preference, and this release has nothing to automate

## Open questions
