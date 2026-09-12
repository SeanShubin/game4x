# The rules as a Petri net

**Generated. Do not edit.** Read out of the release's *Recipes* table by `crates/game-console/src/petri.rs`.

`S-87`, and the formalism is the research lens's: `lenses/research/2026-09-08-simple-finite-and-decidable.md` establishes that a recipe network **is** a Petri net rather than resembling one. Four of the release's five roles are arc kinds and nothing had to be invented to get from one to the other.

**The fifth is `put`, and this drawing is a projection rather than the whole of the release.** `P-421`: a put *names a thing that is already there and says what is true of it afterwards - the same thing and not a new one, so what has an identity keeps it*. A token in a plain Petri net has no identity, so what is drawn below is the count a put names - an arc into or out of `citizen laboring`, and the like - which is faithful about the state and silent about the thing whose state it is.

The release states **31 blocks of recipe rows** under **21 names**. **30 of the blocks are drawn** and **1 are not**, as **45 transitions**.

**30 blocks became 45 transitions**, which is that spelling out. The cases are the ones *Territory resources* offers - a `(resource, density)` pair a territory actually has - rather than a range or the biome table's numbers, which the release says guide and do not bind. A density no territory offers would be a transition for a planet that does not exist.

**Blocks outnumber names because a rule whose subject is a family is a rule for each of them** - `P-373`. `stow` and `discard` are each stated once per kind, and in a net they really are separate transitions: `discard` metal takes metal and `discard` labor takes labor. A repeated name is labelled with the kind it acts on, so two of them are never one node on the page. 21 names, 31 blocks.

**What is missing is counted rather than mentioned.** A diagram that quietly left something out would be a picture of a game that is not this one, and nothing on the page would say so - so the arithmetic above is printed whether or not it has anything to report.

**What that costs the drawing is measured rather than assumed.** Asked against the rows of the one recipe left out rather than against the list of kinds, because what a left-out recipe costs is only whatever it was the sole way into: **nothing**. Every place they touch is reached by some other recipe, so the drawing is short of those transitions and of no part of the state.

**4 kinds the release declares appear nowhere in the drawn net**: `orbit`, `deposit`, `adjacency`, `game`. **This is not the exclusion's doing** - no recipe names any of them, so they would be missing from a drawing with nothing left out. They are where things are and how places relate, rather than things a recipe moves.

**31 places and 161 arcs** between them: 74 `consume`, 43 `produce`, 44 `require`.

**6 of those places are room rather than a count** - `P-374`. What a container stores is the room left, not the total: used capacity is what is there, total capacity is the two added, and nothing records the total so nothing can disagree with it. Making a thing takes one of the room and destroying it gives one back. Room is stored, so room is state, so it is drawn - a diagram showing the count and hiding the room would be leaving out half of what containment is.

**0 of these arcs are zero tests, and that number used to be two.** Reachability in a plain Petri net is decidable and an inhibitor arc makes the net Turing-complete; the release had two, both `limit 0 garrison`. **`P-385` deleted both rows.** Sean, 2026-09-11: *repeated deployments are player choice, safe because they are not capable of causing an infinite resource glitch* - so a second landing on a colony you already hold is allowed, and nothing in the rules refuses it. The `limit` role is still one of the four the release names and no row carries it.

A **place** is a circle - somewhere a kind can be, which is a container and a kind together, because energy in a tank is not energy in a territory and the first of those is bounded while the second has no limit. A place named *room for* something holds the room left in its container rather than the things themselves. A **transition** is a bar: one recipe. An arc into a bar is `consume`, or `require` when it is dotted and the thing is not taken; an arc out of a bar is `produce`.

## The whole net

The drawing is on the page beside this file; what follows is the same net in the form a diff can show.

**It is too large to read whole, and saying so is half of what `R-10` asks.** 31 places and 45 transitions is 76 nodes, joined by 161 arcs - past the point where the eye follows one transition out of the bundle. **So the parts below are the drawing that can be read**: one per recipe, each with its own arcs written out, **and each saying which other recipes reach the same places**. A part that did not say that would read as the whole of a recipe's connections rather than as a part.

## What is not drawn

| Recipe | Why not drawn                                                                    |
| ------ | -------------------------------------------------------------------------------- |
| stand  | its produce row is `that unit's strength`, which is a state rather than a number |

**Computed from the release, not listed.** `S-87` named five, taken from the research lens's own re-encoding rather than from `releases/first-release.md`, and two of that five - `refuel` and `end-of-turn losses` - are not recipes in the release at all. Deriving the set from the table is what keeps this page from inheriting that.

## Places

| Place                            | Container     | Kind              |
| -------------------------------- | ------------- | ----------------- |
| territory in the game            | the game      | territory         |
| ark in an orbit                  | an orbit      | ark               |
| garrison                         | a territory   | garrison          |
| room for garrison                | a territory   | garrison          |
| citizen                          | a territory   | citizen           |
| extractor                        | a territory   | extractor         |
| room for extractor               | a territory   | extractor         |
| place in the game                | the game      | place             |
| unit                             | a territory   | unit              |
| unit moving                      | a territory   | unit moving       |
| energy in a unit's tank          | a unit's tank | energy            |
| room for energy in a unit's tank | a unit's tank | energy            |
| pioneer                          | a territory   | pioneer           |
| room for pioneer                 | a territory   | pioneer           |
| labor                            | a territory   | labor             |
| metal                            | a territory   | metal             |
| store                            | a territory   | store             |
| room for store                   | a territory   | store             |
| yard                             | a territory   | yard              |
| room for yard                    | a territory   | yard              |
| energy                           | a territory   | energy            |
| citizen laboring                 | a territory   | citizen laboring  |
| extractor working                | a territory   | extractor working |
| food                             | a territory   | food              |
| citizen bearing                  | a territory   | citizen bearing   |
| fertility                        | a territory   | fertility         |
| thing                            | a territory   | thing             |
| thing keeps                      | a territory   | thing keeps       |
| citizen defending                | a territory   | citizen defending |
| force                            | a territory   | force             |
| unit defending                   | a territory   | unit defending    |

## The incidence matrix

Places down, transitions across. `-n` is taken, `+n` is made, `rn` is required and not taken, `0!` is the zero test. **This is what the checks operate on.**

| Place                            | deploy ark | move   | found by land | build extractor | build store | build yard | produce pioneer | launch ark | create labor | work (energy x2) | work (energy x4) | work (energy x5) | work (energy x6) | work (energy x8) | work (food x1) | work (food x2) | work (food x3) | work (food x4) | work (food x6) | work (metal x2) | work (metal x3) | work (metal x4) | work (metal x5) | work (metal x6) | work (metal x8) | upkeep | bear | breed | perish | age | spoil | stow (metal) | stow (energy) | discard (metal) | discard (energy) | discard (labor) | discard (fertility) | refresh (unit moving) | refresh (citizen laboring) | refresh (citizen bearing) | refresh (extractor working) | muster | refresh (citizen defending) | refresh (unit defending) | discard (force) |
| -------------------------------- | ---------- | ------ | ------------- | --------------- | ----------- | ---------- | --------------- | ---------- | ------------ | ---------------- | ---------------- | ---------------- | ---------------- | ---------------- | -------------- | -------------- | -------------- | -------------- | -------------- | --------------- | --------------- | --------------- | --------------- | --------------- | --------------- | ------ | ---- | ----- | ------ | --- | ----- | ------------ | ------------- | --------------- | ---------------- | --------------- | ------------------- | --------------------- | -------------------------- | ------------------------- | --------------------------- | ------ | --------------------------- | ------------------------ | --------------- |
| territory in the game            | r1         |        |               |                 |             |            |                 | r1         |              | r1               | r1               | r1               | r1               | r1               | r1             | r1             | r1             | r1             | r1             | r1              | r1              | r1              | r1              | r1              | r1              |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| ark in an orbit                  | -1         |        |               |                 |             |            |                 | +1         |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| garrison                         | +1         |        | +1            |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             | r1     |                             |                          |                 |
| room for garrison                | -1         |        | -1            |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| citizen                          | +2         |        | +2            |                 |             |            | -2              | -2         | r1           |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 | r1     | r1   | +1    | -1     |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             | r1     |                             |                          |                 |
| extractor                        | +1, +1     |        | +1, +1        | +1              |             |            |                 |            |              | r1               | r1               | r1               | r1               | r1               | r1             | r1             | r1             | r1             | r1             | r1              | r1              | r1              | r1              | r1              | r1              |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| room for extractor               | -1, -1     |        | -1, -1        | -1              |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| place in the game                |            | r1, r1 |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| unit                             |            | r1     |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| unit moving                      |            | -1     |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     | +1                    |                            |                           |                             |        |                             |                          |                 |
| energy in a unit's tank          |            | -1     |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| room for energy in a unit's tank |            | +1     |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| pioneer                          |            |        | -1            |                 |             |            | +1              |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| room for pioneer                 |            |        | +1            |                 |             |            | -1              |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| labor                            |            |        |               | -1              | -1          | -1         |                 |            | +1           | -1               | -1               | -1               | -1               | -1               | -1             | -1             | -1             | -1             | -1             | -1              | -1              | -1              | -1              | -1              | -1              |        |      |       |        |     |       |              |               |                 |                  | -1              |                     |                       |                            |                           |                             |        |                             |                          |                 |
| metal                            |            |        |               | -1              | -1          | -15        | -3              | -3         |              |                  |                  |                  |                  |                  |                |                |                |                |                | +2              | +3              | +4              | +5              | +6              | +8              |        |      |       |        |     |       | -1, +1       |               | -1              |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| store                            |            |        |               |                 | +1          |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| room for store                   |            |        |               |                 | -1          |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| yard                             |            |        |               |                 |             | +1         |                 | r1         |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| room for yard                    |            |        |               |                 |             | -1         |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| energy                           |            |        |               |                 |             |            | -6              | -12        |              | +2               | +4               | +5               | +6               | +8               |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              | -1, +1        |                 | -1               |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| citizen laboring                 |            |        |               |                 |             |            |                 |            | -1           |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       | +1                         |                           |                             |        |                             |                          |                 |
| extractor working                |            |        |               |                 |             |            |                 |            |              | -1               | -1               | -1               | -1               | -1               | -1             | -1             | -1             | -1             | -1             | -1              | -1              | -1              | -1              | -1              | -1              |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           | +1                          |        |                             |                          |                 |
| food                             |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  | +1             | +2             | +3             | +4             | +6             |                 |                 |                 |                 |                 |                 | -1     |      | -1    |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| citizen bearing                  |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        | -1   |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            | +1                        |                             |        |                             |                          |                 |
| fertility                        |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        | +1   | -1    |        |     |       |              |               |                 |                  |                 | -1                  |                       |                            |                           |                             |        |                             |                          |                 |
| thing                            |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        | r1  | -1    |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| thing keeps                      |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        | -1  |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             |                          |                 |
| citizen defending                |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             | -1     | +1                          |                          |                 |
| force                            |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             | +1     |                             |                          | -1              |
| unit defending                   |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |      |       |        |     |       |              |               |                 |                  |                 |                     |                       |                            |                           |                             |        |                             | +1                       |                 |

## One recipe at a time

The net above answers how the rules connect and cannot answer what one of them does - at this many arcs the eye cannot follow a single transition out of the bundle. Each is drawn on its own on the page, and written out here.

### deploy ark

- needs 1 territory in the game, and does not take it
- takes 1 ark in an orbit
- makes 1 garrison
- takes 1 room for garrison
- makes 2 citizen
- makes 1 extractor - food
- takes 1 room for extractor - food
- makes 1 extractor - metal
- takes 1 room for extractor - metal

Leaves out 26 other recipes that reach these same places: bear, breed, build extractor, create labor, found by land, launch ark, muster, perish, produce pioneer, upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### move

- needs 1 place in the game, and does not take it
- needs 1 place in the game, and does not take it - joined to `$from` by an edge the unit crosses
- needs 1 unit, and does not take it - moving at least 1
- takes 1 unit moving - moving one less
- takes 1 energy in a unit's tank
- makes 1 room for energy in a unit's tank

Leaves out 1 other recipes that reach these same places: refresh (unit moving).

### found by land

- takes 1 pioneer
- makes 1 room for pioneer
- makes 1 garrison
- takes 1 room for garrison
- makes 2 citizen
- makes 1 extractor - food
- takes 1 room for extractor - food
- makes 1 extractor - metal
- takes 1 room for extractor - metal

Leaves out 26 other recipes that reach these same places: bear, breed, build extractor, create labor, deploy ark, launch ark, muster, perish, produce pioneer, upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### build extractor

- takes 1 labor
- takes 1 metal
- makes 1 extractor - `$resource`
- takes 1 room for extractor - `$resource`

Leaves out 26 other recipes that reach these same places: build store, build yard, create labor, deploy ark, discard (labor), discard (metal), found by land, launch ark, produce pioneer, stow (metal), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### build store

- takes 1 labor
- takes 1 metal
- makes 1 store - `$resource`
- takes 1 room for store - `$resource`

Leaves out 24 other recipes that reach these same places: build extractor, build yard, create labor, discard (labor), discard (metal), launch ark, produce pioneer, stow (metal), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### build yard

- takes 1 labor
- takes 15 metal
- makes 1 yard
- takes 1 room for yard

Leaves out 24 other recipes that reach these same places: build extractor, build store, create labor, discard (labor), discard (metal), launch ark, produce pioneer, stow (metal), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### produce pioneer

- takes 3 metal
- takes 6 energy
- takes 2 citizen
- makes 1 pioneer
- takes 1 room for pioneer

Leaves out 27 other recipes that reach these same places: bear, breed, build extractor, build store, build yard, create labor, deploy ark, discard (energy), discard (metal), found by land, launch ark, muster, perish, stow (energy), stow (metal), upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### launch ark

- needs 1 territory in the game, and does not take it
- takes 3 metal
- takes 12 energy
- takes 2 citizen
- needs 1 yard, and does not take it
- makes 1 ark in an orbit

Leaves out 32 other recipes that reach these same places: bear, breed, build extractor, build store, build yard, create labor, deploy ark, discard (energy), discard (metal), found by land, muster, perish, produce pioneer, stow (energy), stow (metal), upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### create labor

- needs 1 citizen, and does not take it - laboring at least 1
- takes 1 citizen laboring - laboring one less
- makes 1 labor

Leaves out 30 other recipes that reach these same places: bear, breed, build extractor, build store, build yard, deploy ark, discard (labor), found by land, launch ark, muster, perish, produce pioneer, refresh (citizen laboring), upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (energy x2)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 2 energy

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (energy), discard (labor), found by land, launch ark, produce pioneer, refresh (extractor working), stow (energy), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (energy x4)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 4 energy

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (energy), discard (labor), found by land, launch ark, produce pioneer, refresh (extractor working), stow (energy), work (energy x2), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (energy x5)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 5 energy

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (energy), discard (labor), found by land, launch ark, produce pioneer, refresh (extractor working), stow (energy), work (energy x2), work (energy x4), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (energy x6)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 6 energy

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (energy), discard (labor), found by land, launch ark, produce pioneer, refresh (extractor working), stow (energy), work (energy x2), work (energy x4), work (energy x5), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (energy x8)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 8 energy

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (energy), discard (labor), found by land, launch ark, produce pioneer, refresh (extractor working), stow (energy), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (food x1)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 1 food

Leaves out 26 other recipes that reach these same places: breed, build extractor, build store, build yard, create labor, deploy ark, discard (labor), found by land, launch ark, refresh (extractor working), upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (food x2)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 2 food

Leaves out 26 other recipes that reach these same places: breed, build extractor, build store, build yard, create labor, deploy ark, discard (labor), found by land, launch ark, refresh (extractor working), upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (food x3)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 3 food

Leaves out 26 other recipes that reach these same places: breed, build extractor, build store, build yard, create labor, deploy ark, discard (labor), found by land, launch ark, refresh (extractor working), upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (food x4)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 4 food

Leaves out 26 other recipes that reach these same places: breed, build extractor, build store, build yard, create labor, deploy ark, discard (labor), found by land, launch ark, refresh (extractor working), upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (food x6)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 6 food

Leaves out 26 other recipes that reach these same places: breed, build extractor, build store, build yard, create labor, deploy ark, discard (labor), found by land, launch ark, refresh (extractor working), upkeep, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (metal x2)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 2 metal

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (labor), discard (metal), found by land, launch ark, produce pioneer, refresh (extractor working), stow (metal), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (metal x3)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 3 metal

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (labor), discard (metal), found by land, launch ark, produce pioneer, refresh (extractor working), stow (metal), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### work (metal x4)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 4 metal

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (labor), discard (metal), found by land, launch ark, produce pioneer, refresh (extractor working), stow (metal), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x5), work (metal x6), work (metal x8).

### work (metal x5)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 5 metal

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (labor), discard (metal), found by land, launch ark, produce pioneer, refresh (extractor working), stow (metal), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x6), work (metal x8).

### work (metal x6)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 6 metal

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (labor), discard (metal), found by land, launch ark, produce pioneer, refresh (extractor working), stow (metal), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x8).

### work (metal x8)

- needs 1 territory in the game, and does not take it
- needs 1 extractor, and does not take it - working at least 1
- takes 1 extractor working - working one less
- takes 1 labor
- makes 8 metal

Leaves out 27 other recipes that reach these same places: build extractor, build store, build yard, create labor, deploy ark, discard (labor), discard (metal), found by land, launch ark, produce pioneer, refresh (extractor working), stow (metal), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6).

### upkeep

- needs 1 citizen, and does not take it
- takes 1 food

Leaves out 14 other recipes that reach these same places: bear, breed, create labor, deploy ark, found by land, launch ark, muster, perish, produce pioneer, work (food x1), work (food x2), work (food x3), work (food x4), work (food x6).

### bear

- needs 1 citizen, and does not take it - bearing at least 1
- takes 1 citizen bearing - bearing one less
- makes 1 fertility

Leaves out 11 other recipes that reach these same places: breed, create labor, deploy ark, discard (fertility), found by land, launch ark, muster, perish, produce pioneer, refresh (citizen bearing), upkeep.

### breed

- takes 1 fertility
- takes 1 food
- makes 1 citizen

Leaves out 15 other recipes that reach these same places: bear, create labor, deploy ark, discard (fertility), found by land, launch ark, muster, perish, produce pioneer, upkeep, work (food x1), work (food x2), work (food x3), work (food x4), work (food x6).

### perish

- takes 1 citizen - whose upkeep is unpaid

Leaves out 9 other recipes that reach these same places: bear, breed, create labor, deploy ark, found by land, launch ark, muster, produce pioneer, upkeep.

### age

- needs 1 thing, and does not take it - keeps at least 1
- takes 1 thing keeps - keeps one less

Leaves out 1 other recipes that reach these same places: spoil.

### spoil

- takes 1 thing - keeps 0

Leaves out 1 other recipes that reach these same places: age.

### stow (metal)

- takes 1 metal
- makes 1 metal

Leaves out 12 other recipes that reach these same places: build extractor, build store, build yard, discard (metal), launch ark, produce pioneer, work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### stow (energy)

- takes 1 energy
- makes 1 energy

Leaves out 8 other recipes that reach these same places: discard (energy), launch ark, produce pioneer, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8).

### discard (metal)

- takes 1 metal

Leaves out 12 other recipes that reach these same places: build extractor, build store, build yard, launch ark, produce pioneer, stow (metal), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### discard (energy)

- takes 1 energy

Leaves out 8 other recipes that reach these same places: launch ark, produce pioneer, stow (energy), work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8).

### discard (labor)

- takes 1 labor

Leaves out 20 other recipes that reach these same places: build extractor, build store, build yard, create labor, work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### discard (fertility)

- takes 1 fertility

Leaves out 2 other recipes that reach these same places: bear, breed.

### refresh (unit moving)

- makes 1 unit moving - moving at its maximum

Leaves out 1 other recipes that reach these same places: move.

### refresh (citizen laboring)

- makes 1 citizen laboring - laboring at its maximum

Leaves out 1 other recipes that reach these same places: create labor.

### refresh (citizen bearing)

- makes 1 citizen bearing - bearing at its maximum

Leaves out 1 other recipes that reach these same places: bear.

### refresh (extractor working)

- makes 1 extractor working - working at its maximum

Leaves out 16 other recipes that reach these same places: work (energy x2), work (energy x4), work (energy x5), work (energy x6), work (energy x8), work (food x1), work (food x2), work (food x3), work (food x4), work (food x6), work (metal x2), work (metal x3), work (metal x4), work (metal x5), work (metal x6), work (metal x8).

### muster

- needs 1 garrison, and does not take it
- needs 1 citizen, and does not take it - defending at least 1
- takes 1 citizen defending - defending one less
- makes 1 force

Leaves out 11 other recipes that reach these same places: bear, breed, create labor, deploy ark, discard (force), found by land, launch ark, perish, produce pioneer, refresh (citizen defending), upkeep.

### refresh (citizen defending)

- makes 1 citizen defending - defending at its maximum

Leaves out 1 other recipes that reach these same places: muster.

### refresh (unit defending)

- makes 1 unit defending - defending at its maximum

Leaves out nothing a reader of this part needs: no other recipe touches any of these places.

### discard (force)

- takes 1 force

Leaves out 1 other recipes that reach these same places: muster.

