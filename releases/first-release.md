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
- A Pioneer that leaves a territory you control must found the one it enters, or it perishes

### Territory nodes

**In** - `spec/planet.md`, *each territory has zero or more nodes for each resource, and each
node has a density*.

The twelve territories are fixed, each chosen to exercise a different consequence of the rules.
Every territory has at least one food node.

| Territory | Food  | Metal | Energy | What it exercises                     |
| --------- | ----- | ----- | ------ | ------------------------------------- |
| 1         | 3 x 4 | 3 x 4 | 3 x 4  | The landing site. Everything works    |
| 2         | 2 x 6 | 2 x 4 | 2 x 4  | Few dense food nodes                  |
| 3         | 6 x 2 | 2 x 4 | 2 x 4  | Many thin food nodes, same food total |
| 4         | 1 x 2 | 4 x 5 | 4 x 5  | The minimum a territory can be        |
| 5         | 3 x 1 | 8 x 8 | 8 x 8  | Food density 1                        |
| 6         | 4 x 4 | none  | 4 x 5  | No metal                              |
| 7         | 4 x 4 | 4 x 5 | none   | No energy                             |
| 8         | 6 x 6 | 1 x 2 | 1 x 2  | Population without industry           |
| 9         | 2 x 3 | 6 x 8 | 1 x 2  | Rich metal, too few hands to work it  |
| 10        | 3 x 3 | 1 x 3 | 6 x 8  | An energy depot                       |
| 11        | 5 x 6 | 5 x 6 | 5 x 6  | The prize                             |
| 12        | 2 x 2 | 8 x 8 | 8 x 8  | Rich nodes, almost no workers         |

## The loop

1. No presence on the planet
2. Land the ark on a territory from orbit, founding it
3. Work the food extractor to grow the population
4. Build extractors to work the metal and energy nodes
5. Produce pioneers and spread across the planet by land
6. Build a Yard
7. Produce an Ark
8. Launch the Ark into orbit

## Units and structures

### Citizen

- force 1

### Ark

- force 2
- energy cells: 2
- a move costs 1 cell
- can invade land from orbit

### Garrison

- force 1
- multiplier 1
- allows create pioneer

### Create Ark

- cost to produce: 12 metal, 12 energy
- required to produce: a Yard

### Founding

- produces garrison, citizen, food extractor

### Pioneer

- force 2
- energy cells: 2
- a move costs 1 cell
- maintenance: 1 food per turn

### Create Pioneer

- cost to produce: 8 metal, 1 citizen, 6 energy

### Yard

- cost to produce: 15 metal

### Extractor

- cost to produce: 1 labor, and nothing else

## Capabilities

*Not yet filled in. Each step above needs its **In / Reduced / Out** entries against the
spec, and a **vetted when** line. See the shape in [releases](README.md).*

## Open questions
