# Vision

[Documentation map](README.md) · [Root README](../README.md)

## What this is

A 4X space strategy game — explore, expand, exploit, exterminate — played on the surface of
spheres divided into discrete regions.

### Inspirations

Master of Orion, Stars!, Factorio, Distant Worlds, and Sins of a Solar Empire.

### Theme

Each player represents a sentient AI with highly advanced technology. The player can 3D
print life, and endeavors to create life suitable for the environment.

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

- The tessellation approximates a hex grid, and the way it does that is a **Goldberg
  polyhedron** — twelve pentagons at icosahedral vertices, hexagons everywhere else.
  This reverses an earlier decision; see below.
- It must work for a single region, for hundreds, and for everything between.
- The mechanics see a simple adjacency graph. The renderer uses controlled randomness
  on top of that graph to make coastlines and borders look like real geography.

### Icosahedral subdivision: rejected, then adopted

This project originally rejected icosahedral subdivision. That decision has been
reversed, and the reasoning is worth keeping because three of the four original
objections turned out to belong to a different layer.

The original objections, re-examined:

| Objection                                               | Verdict                                                                                                                                                                                                             |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| The twelve pentagons sit at predictable symmetry points | **Weakened.** Twelve pentagons are mandatory on *any* sphere tiling. Letting them fall where they may clusters them, which measurably reads as lumpy; evenly spaced is what a soccer ball is, and what looks right. |
| Twenty patches of straight rows, with visible seams     | **Appearance, not mechanics.** See below.                                                                                                                                                                           |
| Coarse quantization of the region count                 | **Stands.** This is the real cost, and it is paid with preset world sizes.                                                                                                                                          |
| Distortion is systematic rather than random             | **Was real, now fixed.** Subdividing along the sphere rather than through it brought the area ratio at 492 regions from 1.73 down to 1.14.                                                                          |

The error in the original reasoning was mixing up two layers. This document already says
that *"the mechanics see a simple adjacency graph"* and that *"the renderer uses
controlled randomness on top of that graph to make coastlines and borders look like real
geography"* — but the rejection judged the mechanical structure by how it would look.
Symmetry that a renderer is going to disguise is not a reason to reject a topology.

Worth noting *when* that objection is at its strongest: right now, with the renderer
drawing flat colours and geometrically exact borders and no terrain at all. The very
thing that would hide the symmetry is the part not yet built.

What Goldberg polyhedra buy, measured across every buildable size:

- **Perfect topology.** No cell ever has four or seven neighbours. Exactly twelve
  pentagons, never touching each other. No search, no tuning, no luck.
- **Compactness at the ceiling.** 0.906 to 0.99, against 0.907 for a regular hexagon.
- **Uniform movement.** Six equidistant neighbours means cost is isotropic, which is how
  a hex grid approximates distance without computing it. The pentagons are the only
  defect and they thin out with size: 37% of cells at 32 regions, 7% at 162, 2.4% at 492.
- **Integer generation.** This is the largest and least obvious benefit. A `GP(m, n)`
  adjacency graph is *combinatorial* — derivable from `(m, n)` by lattice arithmetic
  rather than measured from geometry. That removes the last floating point from the
  model and dissolves the reproducibility problem described in
  [layers](layers.md#the-one-place-this-is-not-theoretical) entirely.

### The world is a sphere, not a torus

Considered and rejected: making the world a torus. A torus is flat, so it takes a
perfect hex grid with no pentagons, wraps by pure translation in both directions, and
has no projection distortion at all — every region identical in size and shape. It would
delete the twelve pentagons, the poles, and every projection question in one move.

It is rejected because the world is a planet. The cost of that choice is the twelve
pentagons and the fact that no flat map of the world can wrap by translation; both are
consequences of the sphere's topology rather than of any implementation, and both are
worked through in [region splitting](theory/region-splitting.md).

The theory is in [splitting a sphere into regions](theory/region-splitting.md).

## Open questions

- How many regions does a full-size game use? The tessellation must scale to hundreds,
  but the intended default is undecided.
- Do regions subdivide during play, or is the tessellation fixed at world generation?
