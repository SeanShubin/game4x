# The Planet

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

## Shape

- Planets are spheres divided into territories according to Goldberg polyhedrons
- A consequence of that choice is that only certain territory counts exist: `10T + 2` where
  `T = m^2 + mn + n^2` for whole numbers `m` and `n`, giving 12, 32, 42, 72, 92, 122 and
  upward with nothing in between
- Another consequence of Goldberg polyhedrons is that every territory has five or six
  neighbours, and exactly twelve have five
- The smallest Goldberg polyhedron is the dodecahedron, at 12 territories, so 12 is the
  smallest planet size
- There are 5 planet sizes, corresponding to the 5 smallest Goldberg polyhedrons
  - tiny: 12
  - small: 32
  - medium: 42
  - large: 72
  - huge: 92

## Distance

- Two territories are adjacent when they share an edge; territories that meet only at a
  corner are not adjacent
- The distance between any two territories is fixed, and can be computed by adjacency
- There can be roads, portals and such that affect traversal, but they do so without
  changing distance

## What a territory carries

- Each territory has an id, unique within its planet, starting at 1
- For each resource, a territory has total capacity for some number of extractors, and a density that
  each of them yields.
- Each territory has a biome.
- A territory's biome is what the terrain gives it. It is not chosen independently of the
  surface the territory covers.
- The biomes are ocean, ice, desert, grassland, jungle and mountain.
- No territory can be claimed whose biome is ocean.
- Oceans never isolate land from land. Every territory that is not ocean can be reached from
  every other without crossing one.
- Neither rule yields to the other. A planet whose terrain would isolate land is not a planet
  this game presents: the terrain is changed until it does not, and the biome is still what the
  terrain gives.

## Native life

- Each planet has its own native species.

## Presentation

- The planet is presented as a three-dimensional sphere
- The user can rotate the planet such that they can be above any point
- The roll for any point on the planet is fixed, and nothing the user does changes it
- The user can zoom in and out of the planet
- The user can reset the view to a default
- A territory's id is displayed on the sphere **in the practical drawing**
- The poles are visible **in the practical drawing**
- The north and south poles are at the centres of two pentagons, never on a boundary between
  territories
- The planet is drawn either practically or realistically, and the user can change which
- The practical drawing makes adjacency legible. Its colours are flat and make no attempt to
  look like terrain
- The realistic drawing shows the world: terrain
- A drawing never betrays how it was made. A viewer sees the planet, never the process.
- Where two biomes meet, the ground between them is mixed rather than switched. A biome has a
  margin, not a border.
- The two drawings share the camera and nothing else. Rotating, zooming and resetting behave the
  same in both
- The terrain of the realistic drawing is continuous. It varies within a single territory and it
  runs across boundaries, because it is a property of the place and not of the division.
- Nothing in the terrain reveals how the sphere was divided. Apart from the two territories at
  the poles, which are placed already, the terrain has no axis of symmetry, no seam and no
  repeating feature that would let a player find a five-neighbour territory from the terrain
  alone.

## Open questions
