# Architecture

[Documentation map](README.md) · [Root README](../README.md)

## Shape

The code is a Cargo workspace. Each module is its own crate, so the dependency rules
below are enforced by the compiler rather than by discipline.

```
composition root  (binary)
        |
        +--> rendering  (library)
        |         |
        +---------+--> game logic  (library)
                            |
                            +--> supporting crates (library)
```

Dependencies point in one direction only, downward. Nothing below depends on anything
above it.

## The modules

### Composition root

A binary crate. It constructs the concrete implementations, wires them to each other,
and starts the program. That is all it does.

It is the only place in the codebase allowed to know about every other module at once.
Because it holds no logic, it needs no tests of its own — if it is large enough to be
worth testing, something has leaked into it that belongs elsewhere.

### Game logic

A library crate. The rules of the game: turns, regions, adjacency, resources, combat,
victory conditions.

- **Integers only.** No floating point. See [vision](vision.md#whole-numbers-only-in-the-game-logic).
- No knowledge of rendering, windowing, input devices, or file formats.
- No knowledge of the *geometry* of a region. A region is an identifier; adjacency is
  an edge in a graph. Where a region sits on the sphere is the renderer's problem.
- Deterministic: same inputs and same seed produce the same game, everywhere.

### Rendering

A library crate. Turns game state into pixels.

- Owns all geometry: sphere coordinates, region outlines, projections, the camera.
- Owns all floating point.
- Owns the controlled randomness that makes borders look geographic rather than
  computed. That randomness is seeded from the game state so it is stable across
  frames and across sessions, but it never feeds back into the game logic.
- Reads game state; never mutates it.

### Supporting crates

Added as needed to keep the above clean. Candidates so far:

- **Spherical tessellation** — generating the region graph and its geometry. Runs at
  world generation time, produces an integer adjacency graph for the game logic and a
  geometric description for the renderer. See
  [region splitting](theory/region-splitting.md).
- **Graph coloring** — assigning few colors so no two adjacent regions match. Pure
  graph algorithm, no game or rendering knowledge. See
  [region coloring](theory/region-coloring.md).

A crate earns its existence by having a boundary someone else could reasonably want to
cross. "This file got long" is not a boundary.

## Rules

1. **Dependencies point one way.** If two modules need each other, the shared part
   belongs in a third module beneath both.
2. **Only the composition root sees the whole graph.** Everything else depends on
   interfaces and on the modules below it.
3. **Floating point lives above the game logic.** The game logic boundary is where
   `f32` and `f64` stop.
4. **Each crate has a `README.md`** describing its purpose and its public surface,
   linked from this document.

## Open questions

- Does rendering depend on the game logic crate directly, or on a trait-defined view
  of it so prototypes can supply fake state? Leaning toward the latter.
- Where does input handling live — its own crate, or part of the composition root?
