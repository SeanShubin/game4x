# The rules as a Petri net

**Generated. Do not edit.** Read out of the release's *Recipes* table by `crates/game-console/src/petri.rs`.

`S-87`, and the formalism is the research lens's: `lenses/research/2026-09-08-simple-finite-and-decidable.md` establishes that a recipe network **is** a Petri net rather than resembling one. The release's four roles are the four arc kinds and nothing had to be invented to get from one to the other.

The release states **24 blocks of recipe rows** under **20 names**. **24 of the blocks are drawn** and **0 are not**, as **39 transitions**.

**Nothing is left out.** A Petri net arc carries a constant weight, and until `P-376` the release had one row that did not: `work` produces *`$where`'s density for that resource*. That rule is not a rule that measures what is present - it is **one rule with a number per case**, and the specification says whatever reads it may spell the cases out. So it is drawn as its cases rather than counted as missing, and this page no longer has a list of what a reader cannot see.

**24 blocks became 39 transitions**, which is that spelling out. The cases are the ones *Territory resources* offers - a `(resource, density)` pair a territory actually has - rather than a range or the biome table's numbers, which the release says guide and do not bind. A density no territory offers would be a transition for a planet that does not exist.

**Blocks outnumber names because a rule whose subject is a family is a rule for each of them** - `P-373`. `stow` and `discard` are each stated once per kind, and in a net they really are separate transitions: `discard` metal takes metal and `discard` labor takes labor. A repeated name is labelled with the kind it acts on, so two of them are never one node on the page. 20 names, 24 blocks.

**What is missing is counted rather than mentioned.** A diagram that quietly left something out would be a picture of a game that is not this one, and nothing on the page would say so - so the arithmetic above is printed whether or not it has anything to report.

**4 kinds the release declares appear nowhere in the drawn net**: `orbit`, `deposit`, `adjacency`, `game`. **This is not the exclusion's doing** - no recipe names any of them, so they would be missing from a drawing with nothing left out. They are where things are and how places relate, rather than things a recipe moves.

**23 places and 194 arcs** between them: 94 `consume`, 78 `produce`, 22 `require`.

**6 of those places are room rather than a count** - `P-374`. What a container stores is the room left, not the total: used capacity is what is there, total capacity is the two added, and nothing records the total so nothing can disagree with it. Making a thing takes one of the room and destroying it gives one back. Room is stored, so room is state, so it is drawn - a diagram showing the count and hiding the room would be leaving out half of what containment is.

**0 of these arcs are zero tests, and that number used to be two.** Reachability in a plain Petri net is decidable and an inhibitor arc makes the net Turing-complete; the release had two, both `limit 0 garrison`. **`P-385` deleted both rows.** Sean, 2026-09-11: *repeated deployments are player choice, safe because they are not capable of causing an infinite resource glitch* - so a second landing on a colony you already hold is allowed, and nothing in the rules refuses it. The `limit` role is still one of the four the release names and no row carries it.

A **place** is a circle - somewhere a kind can be, which is a container and a kind together, because energy in a tank is not energy in a territory and the first of those is bounded while the second has no limit. A place named *room for* something holds the room left in its container rather than the things themselves. A **transition** is a bar: one recipe. An arc into a bar is `consume`, or `require` when it is dotted and the thing is not taken; an arc out of a bar is `produce`.

## The whole net

The drawing is on the page beside this file; what follows is the same net in the form a diff can show.

## What is not drawn

| Recipe | Why not drawn |
| ------ | ------------- |

**Computed from the release, not listed.** `S-87` named five, taken from the research lens's own re-encoding rather than from `releases/first-release.md`, and two of that five - `refuel` and `end-of-turn losses` - are not recipes in the release at all. Deriving the set from the table is what keeps this page from inheriting that.

## Places

| Place                            | Container     | Kind      |
| -------------------------------- | ------------- | --------- |
| territory in the game            | the game      | territory |
| ark in an orbit                  | an orbit      | ark       |
| garrison                         | a territory   | garrison  |
| room for garrison                | a territory   | garrison  |
| citizen                          | a territory   | citizen   |
| extractor                        | a territory   | extractor |
| room for extractor               | a territory   | extractor |
| store                            | a territory   | store     |
| room for store                   | a territory   | store     |
| place in the game                | the game      | place     |
| unit                             | a territory   | unit      |
| energy in a unit's tank          | a unit's tank | energy    |
| room for energy in a unit's tank | a unit's tank | energy    |
| pioneer                          | a territory   | pioneer   |
| room for pioneer                 | a territory   | pioneer   |
| labor                            | a territory   | labor     |
| metal                            | a territory   | metal     |
| yard                             | a territory   | yard      |
| room for yard                    | a territory   | yard      |
| energy                           | a territory   | energy    |
| food                             | a territory   | food      |
| fertility                        | a territory   | fertility |
| thing                            | a territory   | thing     |

## The incidence matrix

Places down, transitions across. `-n` is taken, `+n` is made, `rn` is required and not taken, `0!` is the zero test. **This is what the checks operate on.**

| Place                            | deploy ark | move   | found by land | build extractor | build store | build yard | produce pioneer | launch ark | create labor | work (energy x2) | work (energy x4) | work (energy x5) | work (energy x6) | work (energy x8) | work (food x1) | work (food x2) | work (food x3) | work (food x4) | work (food x6) | work (metal x2) | work (metal x3) | work (metal x4) | work (metal x5) | work (metal x6) | work (metal x8) | upkeep | bear   | breed | renew  | perish | age    | spoil | stow (metal) | stow (energy) | discard (metal) | discard (energy) | discard (labor) | discard (fertility) | refresh |
| -------------------------------- | ---------- | ------ | ------------- | --------------- | ----------- | ---------- | --------------- | ---------- | ------------ | ---------------- | ---------------- | ---------------- | ---------------- | ---------------- | -------------- | -------------- | -------------- | -------------- | -------------- | --------------- | --------------- | --------------- | --------------- | --------------- | --------------- | ------ | ------ | ----- | ------ | ------ | ------ | ----- | ------------ | ------------- | --------------- | ---------------- | --------------- | ------------------- | ------- |
| territory in the game            | r1         |        |               |                 |             |            |                 | r1         |              | r1               | r1               | r1               | r1               | r1               | r1             | r1             | r1             | r1             | r1             | r1              | r1              | r1              | r1              | r1              | r1              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| ark in an orbit                  | -1         |        |               |                 |             |            |                 | +1         |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| garrison                         | +1         |        | +1            |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for garrison                | -1         |        | -1            |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| citizen                          | +2         |        | +2            |                 |             |            | -2              | -2         | -1, +1       |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 | r1     | -1, +1 | +1    | -1, +1 | -1     |        |       |              |               |                 |                  |                 |                     |         |
| extractor                        | +1, +1     |        | +1, +1        | +1              |             |            |                 |            |              | -1, +1           | -1, +1           | -1, +1           | -1, +1           | -1, +1           | -1, +1         | -1, +1         | -1, +1         | -1, +1         | -1, +1         | -1, +1          | -1, +1          | -1, +1          | -1, +1          | -1, +1          | -1, +1          |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for extractor               | -1, -1     |        | -1, -1        | -1              |             |            |                 |            |              | +1, -1           | +1, -1           | +1, -1           | +1, -1           | +1, -1           | +1, -1         | +1, -1         | +1, -1         | +1, -1         | +1, -1         | +1, -1          | +1, -1          | +1, -1          | +1, -1          | +1, -1          | +1, -1          |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| store                            | +1, +1     |        | +1, +1        |                 | +1          |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for store                   | -1, -1     |        | -1, -1        |                 | -1          |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| place in the game                |            | r1, r1 |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| unit                             |            | -1, +1 |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| energy in a unit's tank          |            | -1     |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for energy in a unit's tank |            | +1     |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| pioneer                          |            |        | -1            |                 |             |            | +1              |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for pioneer                 |            |        | +1            |                 |             |            | -1              |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| labor                            |            |        |               | -1              | -1          | -1         |                 |            | +1           | -1               | -1               | -1               | -1               | -1               | -1             | -1             | -1             | -1             | -1             | -1              | -1              | -1              | -1              | -1              | -1              |        |        |       |        |        |        |       |              |               |                 |                  | -1              |                     |         |
| metal                            |            |        |               | -1              | -1          | -15        | -3              | -3         |              |                  |                  |                  |                  |                  |                |                |                |                |                | +2              | +3              | +4              | +5              | +6              | +8              |        |        |       |        |        |        |       | -1, +1       |               | -1              |                  |                 |                     |         |
| yard                             |            |        |               |                 |             | +1         |                 | r1         |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for yard                    |            |        |               |                 |             | -1         |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| energy                           |            |        |               |                 |             |            | -6              | -12        |              | +2               | +4               | +5               | +6               | +8               |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        |        |       |              | -1, +1        |                 | -1               |                 |                     |         |
| food                             |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  | +1             | +2             | +3             | +4             | +6             |                 |                 |                 |                 |                 |                 | -1     |        | -1    |        |        |        |       |              |               |                 |                  |                 |                     |         |
| fertility                        |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        | +1     | -1    |        |        |        |       |              |               |                 |                  |                 | -1                  |         |
| thing                            |            |        |               |                 |             |            |                 |            |              |                  |                  |                  |                  |                  |                |                |                |                |                |                 |                 |                 |                 |                 |                 |        |        |       |        |        | -1, +1 | -1    |              |               |                 |                  |                 |                     | -1, +1  |

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
- makes 1 store - food
- takes 1 room for store - food
- makes 1 store - metal
- takes 1 room for store - metal

### move

- needs 1 place in the game, and does not take it
- needs 1 place in the game, and does not take it - joined to `$from` by an edge the unit crosses
- takes 1 unit - ready
- takes 1 energy in a unit's tank
- makes 1 room for energy in a unit's tank
- makes 1 unit - not ready

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
- makes 1 store - food
- takes 1 room for store - food
- makes 1 store - metal
- takes 1 room for store - metal

### build extractor

- takes 1 labor
- takes 1 metal
- makes 1 extractor - `$resource`
- takes 1 room for extractor - `$resource`

### build store

- takes 1 labor
- takes 1 metal
- makes 1 store - `$resource`
- takes 1 room for store - `$resource`

### build yard

- takes 1 labor
- takes 15 metal
- makes 1 yard
- takes 1 room for yard

### produce pioneer

- takes 3 metal
- takes 6 energy
- takes 2 citizen
- makes 1 pioneer
- takes 1 room for pioneer

### launch ark

- needs 1 territory in the game, and does not take it
- takes 3 metal
- takes 12 energy
- takes 2 citizen
- needs 1 yard, and does not take it
- makes 1 ark in an orbit

### create labor

- takes 1 citizen - ready
- makes 1 citizen - not ready
- makes 1 labor

### work (energy x2)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 2 energy

### work (energy x4)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 4 energy

### work (energy x5)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 5 energy

### work (energy x6)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 6 energy

### work (energy x8)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 8 energy

### work (food x1)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 1 food

### work (food x2)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 2 food

### work (food x3)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 3 food

### work (food x4)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 4 food

### work (food x6)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 6 food

### work (metal x2)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 2 metal

### work (metal x3)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 3 metal

### work (metal x4)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 4 metal

### work (metal x5)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 5 metal

### work (metal x6)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 6 metal

### work (metal x8)

- needs 1 territory in the game, and does not take it
- takes 1 labor
- takes 1 extractor - ready
- makes 1 room for extractor - ready
- makes 1 extractor - not ready
- takes 1 room for extractor - not ready
- makes 8 metal

### upkeep

- needs 1 citizen, and does not take it
- takes 1 food

### bear

- takes 1 citizen - fertile
- makes 1 citizen - spent
- makes 1 fertility

### breed

- takes 1 fertility
- takes 1 food
- makes 1 citizen

### renew

- takes 1 citizen - spent
- makes 1 citizen - fertile

### perish

- takes 1 citizen - whose upkeep is unpaid

### age

- takes 1 thing - keeps at least 1
- makes 1 thing - keeps one less

### spoil

- takes 1 thing - keeps 0

### stow (metal)

- takes 1 metal
- makes 1 metal

### stow (energy)

- takes 1 energy
- makes 1 energy

### discard (metal)

- takes 1 metal

### discard (energy)

- takes 1 energy

### discard (labor)

- takes 1 labor

### discard (fertility)

- takes 1 fertility

### refresh

- takes 1 thing - not ready
- makes 1 thing - ready

