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

## What a territory carries

- Each territory on a planet has a **threat** level, which requires a greater **security**
  level in order to claim
- Territories have natural resource levels
- Two territories are adjacent when they share an edge; territories that meet only at a
  corner are not adjacent

## Open questions
