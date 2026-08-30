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

Every biome except ocean has at least one food node of density two or more.

## Controls

- Rotation is bound to the arrow keys, and to dragging
- Zoom is bound to the wheel, and to pinching
- Reset is bound to `R`, and to a control
- The three surfaces in this release are reached by `F1`, `F2` and `F3`, by buttons on the page,
  and by `/game`, `/console` and `/browser` typed at the console
- Choosing a planet size abandons the current game and starts one on a planet of that size. It
  is bound to `1` through `5`, to a control for each size, and to `/new <size>`

## Capabilities

Each capability is an item addressed to the code lane, in the same shape every outbox uses, so
`tools/outbox --to code` lists what is still to build beside what a lens has found. **Status is
`open` until the *vetted when* line is observed, then `vetted`.**

**The code lane does not mark its own.** It reports the evidence `to spec` and this file records
it - the same shape as a lens never editing what it reviews, and what keeps the record of what has
been delivered separate from the account of whoever built it.

### R-1 - Two drawings

**to** code · **status** open

- **In** - `spec/planet.md`, *the planet is drawn either practically or realistically, and the
  user can change which*
- **Vetted when** - switching between them moves nothing: the planet is at the same rotation and
  zoom afterwards, and every territory covers the same pixels

### R-2 - Terrain that crosses boundaries

**to** code · **status** open

- **In** - `spec/planet.md`, *the terrain of the realistic drawing is continuous*
- **Vetted when** - no line visible in the realistic drawing coincides with a territory boundary,
  and terrain visibly varies within a single territory

### R-3 - A division that cannot be seen

**to** code · **status** open

- **In** - `spec/planet.md`, *nothing in the terrain reveals how the sphere was divided*
- **Vetted when** - a person who has not seen the tessellation is shown the realistic drawing and
  cannot mark where a five-neighbour territory is, beyond the two at the poles

### R-4 - A biome per territory

**to** code · **status** open

- **In** - `spec/planet.md`, *each territory has a biome*, and *a territory's biome is what the
  terrain gives it*
- **Vetted when** - `show territory 5` names a biome, and no other biome covers more of that
  ground in the realistic drawing

### R-5 - Terrain resolved as finely as it is shown

**to** code · **status** open

- **In** - `spec/planet.md`, *nothing of how a drawing is made is visible in it*
- **Vetted when** - at the default camera, no facet, band or flat wash betrays how the surface was
  built, and the finest visible detail is terrain

### R-6 - The loop can be played through

**to** code · **status** open

- **In** - `spec/control.md`, *a player wins by launching an Ark from a fully exploited planet*
- **Vetted when** - starting from a single Ark in orbit over the twelve designed territories, a
  person playing entirely by hand reaches a fully exploited planet and launches an Ark

## Open questions
