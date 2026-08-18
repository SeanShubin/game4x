# Vision

[Documentation map](README.md) · [Root README](../README.md)

## What this is

A 4X game — explore, expand, exploit, exterminate — played on the surface of a sphere
divided into discrete regions.

## Constraints

These are the fixed points of the design. Everything else is negotiable.

### Rust

The implementation language. No part of the project is written in another language
without a documented reason.

### Whole numbers only in the game logic

The game logic module uses integers exclusively. No floating point anywhere in it.

The reason is determinism and reproducibility: integer arithmetic gives bit-identical
results on every machine, every compiler version, and every optimization level.
Floating point does not, and a 4X game accumulates state over thousands of turns, so
even a one-bit divergence eventually becomes a different game.

The consequences are real and accepted:

- Ratios are represented as explicit numerator/denominator pairs, or as fixed-point
  values with a documented scale factor, not as `f64`.
- Rounding is a deliberate decision at each site, not a side effect of the machine.
- Anything genuinely continuous — camera position, animation timing, the geometry of a
  region's outline — belongs to the renderer, not the game logic.

Region *geometry* is therefore a rendering concern. The game logic knows only which
region is adjacent to which, as an integer graph. See
[region splitting](theory/region-splitting.md) for where the dividing line falls.

### Modules with a thin composition root

The project is split into multiple modules. A thin composition root module wires them
together and contains no logic of its own. See [architecture](architecture.md).

### Prototypes before features

Each significant aspect of the game gets a prototype demonstrating it in isolation
before it becomes part of the game. See [prototypes](prototypes/README.md).

### Discoverable documentation

Everything is reachable from the root README. See [the documentation map](README.md).

## The planet

The surface of the world is a sphere divided into regions.

- The tessellation approximates a hex grid. **Icosahedral subdivision is rejected** —
  its 12 symmetry points and regular grid seams are visible in play and read as
  artificial.
- It must work for a single region, for hundreds, and for everything between.
- The mechanics see a simple adjacency graph. The renderer uses controlled randomness
  on top of that graph to make coastlines and borders look like real geography.

The theory is in [splitting a sphere into regions](theory/region-splitting.md).

## Open questions

- How many regions does a full-size game use? The tessellation must scale to hundreds,
  but the intended default is undecided.
- Do regions subdivide during play, or is the tessellation fixed at world generation?
