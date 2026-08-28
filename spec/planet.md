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

- The distance between any two territories is fixed, and can be computed by adjacency
- There can be roads, portals and such that affect traversal, but they do so without
  changing distance

## What a territory carries

- Two territories are adjacent when they share an edge; territories that meet only at a
  corner are not adjacent
- Each territory has an id, unique within its planet, starting at 1
- Each territory has zero or more nodes for each resource, and each node has a density

## Native life

- Each planet has its own native species.

## Presentation

- The planet is presented as a three-dimensional sphere
- The user can rotate the planet such that they can be above any point
- Rotation is bound to the arrow keys
- Dragging with the mouse rotates the planet
- The roll for any point on the planet is fixed
- The user can zoom in and out of the planet
- The user can reset the view to a default
- A territory's id is displayed on the sphere
- The poles are visible on the planet.
- The north and south poles are at the centres of two pentagons, never on a boundary between
  territories

## Open questions
