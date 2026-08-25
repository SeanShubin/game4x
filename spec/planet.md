# The Planet

**Authored.** Every line here is reviewed by Sean before it lands. Claude may reorganize and
rephrase, reporting every change; new content arrives only by Sean accepting a numbered
proposal from [proposals](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

## Shape

- Planets are spheres divided into territories according to Goldberg polyhedrons
- A consequence of that choice is that only certain territory counts exist: `10T + 2` where
  `T = m^2 + mn + n^2` for whole numbers `m` and `n`, giving 12, 32, 42, 72, 92, 122 and
  upward with nothing in between
- The minimum planet size is therefore 12, a dodecahedron, the smallest Goldberg polyhedron
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

## Open questions
