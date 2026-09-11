# The rules as a Petri net

**Generated. Do not edit.** Read out of the release's *Recipes* table by `crates/game-console/src/petri.rs`.

`S-87`, and the formalism is the research lens's: `lenses/research/2026-09-08-simple-finite-and-decidable.md` establishes that a recipe network **is** a Petri net rather than resembling one. The release's four roles are the four arc kinds and nothing had to be invented to get from one to the other.

The release declares **16 recipes**. **12 are drawn** and **4 are not**, because a Petri net arc carries a constant weight and those recipes have amounts that depend on the state when they fire.

**That is the whole of what is missing, and it is counted rather than mentioned.** A diagram that quietly left them out would be a picture of a game that is not this one, and nothing on the page would say so.

**15 places and 49 arcs** between them: 20 `consume`, 2 `limit`, 22 `produce`, 5 `require`.

**2 of those arcs are zero tests**, and they are why this is worth drawing rather than tabulating. Reachability in a plain Petri net is decidable; an inhibitor arc makes the net Turing-complete. Both of this release's are `limit 0 garrison`, and a garrison is bounded by a capacity of 1 - a zero test on a bounded place costs nothing, which is `C-75` and what `X-9` asks the specification to adopt.

A **place** is a circle - somewhere a kind can be, which is a container and a kind together, because energy in a tank is not energy in a territory. A **transition** is a bar: one recipe. An arc into a bar is `consume`, or `require` when it is dotted and the thing is not taken; an arc out of a bar is `produce`. An **inhibitor arc** has a hollow head and fires only when its place is empty.

## The whole net

The drawing is on the page beside this file; what follows is the same net in the form a diff can show.

## What is not drawn

| Recipe | Why not drawn                                                                                                    |
| ------ | ---------------------------------------------------------------------------------------------------------------- |
| work   | its produce row is ``$where`'s density for that resource`, which is a state rather than a number                 |
| upkeep | its consume row is `the thing's upkeep`, which is a state rather than a number                                   |
| grow   | its consume row is `the lesser of the surplus food and the citizens here`, which is a state rather than a number |
| perish | its produce row is `the thing's metal`, which is a state rather than a number                                    |

**Computed from the release, not listed.** `S-87` named five, taken from the research lens's own re-encoding rather than from `releases/first-release.md`, and two of that five - `refuel` and `end-of-turn losses` - are not recipes in the release at all. Deriving the set from the table is what keeps this page from inheriting that.

## Places

| Place                   | Container     | Kind      |
| ----------------------- | ------------- | --------- |
| territory in the game   | the game      | territory |
| ark in an orbit         | an orbit      | ark       |
| garrison                | a territory   | garrison  |
| citizen                 | a territory   | citizen   |
| extractor               | a territory   | extractor |
| store                   | a territory   | store     |
| place in the game       | the game      | place     |
| unit                    | a territory   | unit      |
| energy in a unit's tank | a unit's tank | energy    |
| pioneer                 | a territory   | pioneer   |
| labor                   | a territory   | labor     |
| metal                   | a territory   | metal     |
| yard                    | a territory   | yard      |
| energy                  | a territory   | energy    |
| thing                   | a territory   | thing     |

## The incidence matrix

Places down, transitions across. `-n` is taken, `+n` is made, `rn` is required and not taken, `0!` is the zero test. **This is what the checks operate on.**

| Place                   | deploy ark | move   | found by land | build extractor | build store | build yard | produce pioneer | launch ark | create labor | age    | spoil | refresh |
| ----------------------- | ---------- | ------ | ------------- | --------------- | ----------- | ---------- | --------------- | ---------- | ------------ | ------ | ----- | ------- |
| territory in the game   | r1         |        |               |                 |             |            |                 | r1         |              |        |       |         |
| ark in an orbit         | -1         |        |               |                 |             |            |                 | +1         |              |        |       |         |
| garrison                | 0!, +1     |        | 0!, +1        |                 |             |            |                 |            |              |        |       |         |
| citizen                 | +2         |        | +2            |                 |             |            | -2              | -2         | -1, +1       |        |       |         |
| extractor               | +1, +1     |        | +1, +1        | +1              |             |            |                 |            |              |        |       |         |
| store                   | +1, +1     |        | +1, +1        |                 | +1          |            |                 |            |              |        |       |         |
| place in the game       |            | r1, r1 |               |                 |             |            |                 |            |              |        |       |         |
| unit                    |            | -1, +1 |               |                 |             |            |                 |            |              |        |       |         |
| energy in a unit's tank |            | -1     |               |                 |             |            |                 |            |              |        |       |         |
| pioneer                 |            |        | -1            |                 |             |            | +1              |            |              |        |       |         |
| labor                   |            |        |               | -1              | -1          | -1         |                 |            | +1           |        |       |         |
| metal                   |            |        |               | -1              | -1          | -15        | -3              | -3         |              |        |       |         |
| yard                    |            |        |               |                 |             | +1         |                 | r1         |              |        |       |         |
| energy                  |            |        |               |                 |             |            | -6              | -12        |              |        |       |         |
| thing                   |            |        |               |                 |             |            |                 |            |              | -1, +1 | -1    | -1, +1  |

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

### age

### spoil

### refresh

