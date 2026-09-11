# The rules as a Petri net

**Generated. Do not edit.** Read out of the release's *Recipes* table by `crates/game-console/src/petri.rs`.

`S-87`, and the formalism is the research lens's: `lenses/research/2026-09-08-simple-finite-and-decidable.md` establishes that a recipe network **is** a Petri net rather than resembling one. The release's four roles are the four arc kinds and nothing had to be invented to get from one to the other.

The release states **24 blocks of recipe rows** under **20 names**. **23 are drawn** and **1 are not**, because a Petri net arc carries a constant weight and those recipes have amounts that depend on the state when they fire.

**The two numbers differ because a rule whose subject is a family is a rule for each of them** - `P-373`. `stow` and `discard` are each stated once per kind, and in a net they really are separate transitions: `discard` metal takes metal and `discard` labor takes labor. So 24 blocks are drawn as 24 nodes, and a repeated name is labelled with the kind it acts on so that two of them are never one node on the page.

**That is the whole of what is missing, and it is counted rather than mentioned.** A diagram that quietly left them out would be a picture of a game that is not this one, and nothing on the page would say so.

**What that costs the drawing is measured rather than assumed.** Asked against the rows of the one recipe left out rather than against the list of kinds, because what a left-out recipe costs is only whatever it was the sole way into: **`resource`**, which nothing drawn names. A reader looking at the picture for resource would conclude the game has none.

**4 kinds the release declares appear nowhere in the drawn net**: `orbit`, `deposit`, `adjacency`, `game`. **This is not the exclusion's doing** - no recipe names any of them, so they would be missing from a drawing with nothing left out. They are where things are and how places relate, rather than things a recipe moves.

**23 places and 82 arcs** between them: 46 `consume`, 30 `produce`, 6 `require`.

**6 of those places are room rather than a count** - `P-374`. What a container stores is the room left, not the total: used capacity is what is there, total capacity is the two added, and nothing records the total so nothing can disagree with it. Making a thing takes one of the room and destroying it gives one back. Room is stored, so room is state, so it is drawn - a diagram showing the count and hiding the room would be leaving out half of what containment is.

**0 of these arcs are zero tests, and that number used to be two.** Reachability in a plain Petri net is decidable and an inhibitor arc makes the net Turing-complete; the release had two, both `limit 0 garrison`. `P-374` removed them without meaning to: *there is no garrison here* is *the garrison's room is untouched*, which is an ordinary requirement on an ordinary place. **That translation is exact only because a garrison's capacity is one** - at two, *there is none* and *there is room for one* are different claims - and the reader is refused rather than approximated if a `limit 0` ever appears somewhere with more room than that.

A **place** is a circle - somewhere a kind can be, which is a container and a kind together, because energy in a tank is not energy in a territory and the first of those is bounded while the second has no limit. A place named *room for* something holds the room left in its container rather than the things themselves. A **transition** is a bar: one recipe. An arc into a bar is `consume`, or `require` when it is dotted and the thing is not taken; an arc out of a bar is `produce`.

## The whole net

The drawing is on the page beside this file; what follows is the same net in the form a diff can show.

## What is not drawn

| Recipe | Why not drawn                                                                                    |
| ------ | ------------------------------------------------------------------------------------------------ |
| work   | its produce row is ``$where`'s density for that resource`, which is a state rather than a number |

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

| Place                            | deploy ark | move   | found by land | build extractor | build store | build yard | produce pioneer | launch ark | create labor | upkeep | bear   | breed | renew  | perish | age    | spoil | stow (metal) | stow (energy) | discard (metal) | discard (energy) | discard (labor) | discard (fertility) | refresh |
| -------------------------------- | ---------- | ------ | ------------- | --------------- | ----------- | ---------- | --------------- | ---------- | ------------ | ------ | ------ | ----- | ------ | ------ | ------ | ----- | ------------ | ------------- | --------------- | ---------------- | --------------- | ------------------- | ------- |
| territory in the game            | r1         |        |               |                 |             |            |                 | r1         |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| ark in an orbit                  | -1         |        |               |                 |             |            |                 | +1         |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| garrison                         | +1         |        | +1            |                 |             |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for garrison                | -1         |        | -1            |                 |             |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| citizen                          | +2         |        | +2            |                 |             |            | -2              | -2         | -1, +1       | r1     | -1, +1 | +1    | -1, +1 | -1     |        |       |              |               |                 |                  |                 |                     |         |
| extractor                        | +1, +1     |        | +1, +1        | +1              |             |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for extractor               | -1, -1     |        | -1, -1        | -1              |             |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| store                            | +1, +1     |        | +1, +1        |                 | +1          |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for store                   | -1, -1     |        | -1, -1        |                 | -1          |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| place in the game                |            | r1, r1 |               |                 |             |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| unit                             |            | -1, +1 |               |                 |             |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| energy in a unit's tank          |            | -1     |               |                 |             |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for energy in a unit's tank |            | +1     |               |                 |             |            |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| pioneer                          |            |        | -1            |                 |             |            | +1              |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for pioneer                 |            |        | +1            |                 |             |            | -1              |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| labor                            |            |        |               | -1              | -1          | -1         |                 |            | +1           |        |        |       |        |        |        |       |              |               |                 |                  | -1              |                     |         |
| metal                            |            |        |               | -1              | -1          | -15        | -3              | -3         |              |        |        |       |        |        |        |       | -1, +1       |               | -1              |                  |                 |                     |         |
| yard                             |            |        |               |                 |             | +1         |                 | r1         |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| room for yard                    |            |        |               |                 |             | -1         |                 |            |              |        |        |       |        |        |        |       |              |               |                 |                  |                 |                     |         |
| energy                           |            |        |               |                 |             |            | -6              | -12        |              |        |        |       |        |        |        |       |              | -1, +1        |                 | -1               |                 |                     |         |
| food                             |            |        |               |                 |             |            |                 |            |              | -1     |        | -1    |        |        |        |       |              |               |                 |                  |                 |                     |         |
| fertility                        |            |        |               |                 |             |            |                 |            |              |        | +1     | -1    |        |        |        |       |              |               |                 |                  |                 | -1                  |         |
| thing                            |            |        |               |                 |             |            |                 |            |              |        |        |       |        |        | -1, +1 | -1    |              |               |                 |                  |                 |                     | -1, +1  |

## One recipe at a time

The net above answers how the rules connect and cannot answer what one of them does - at this many arcs the eye cannot follow a single transition out of the bundle. Each is drawn on its own on the page.

### deploy ark

### move

### found by land

### build extractor

### build store

### build yard

### produce pioneer

### launch ark

### create labor

### upkeep

### bear

### breed

### renew

### perish

### age

### spoil

### stow (metal)

### stow (energy)

### discard (metal)

### discard (energy)

### discard (labor)

### discard (fertility)

### refresh

