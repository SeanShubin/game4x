# Prototype: Planet View

[Prototypes](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## The question

Can we divide the surface of a sphere into hex-like regions, scale that from one region
to hundreds, and render it so that it reads as a world rather than as a computed grid?

The theory behind the answer lives in
[splitting a sphere into regions](../theory/region-splitting.md) and
[coloring regions](../theory/region-coloring.md). This document covers what gets built.

**Answer: not yet determined.**

## Scope

Three views of the same tessellated sphere, switchable at runtime.

### 3D view

The sphere itself, rendered as regions.

- Rotate and zoom.
- Region borders visible.
- Controlled randomness applied to borders and terrain so the world looks geographic
  rather than computed. See
  [render-time detail](../theory/region-splitting.md#step-6-render-time-detail).

### 2D view

A cylindrical projection of the same sphere.

- Pan and zoom.
- **Wraps around horizontally.** The projection is periodic in longitude, so the map is
  drawn repeatedly at horizontal offsets of one full world width.
  - Each region is rendered normally exactly once.
  - Every duplicate of an already-rendered region is dimmed.
  - The cursor is replicated over the duplicates it is hovering, also dimmed.
- No vertical wraparound. The top and bottom of the projection are the poles, which are
  points, not edges.

The projection choice is open — see
[the 2D view and wraparound](../theory/region-splitting.md#8-the-2d-view-and-wraparound).

### Simplified view

Everything the game mechanics see, and nothing else.

- Regions as abstract shapes, adjacency as the only relationship shown.
- Colored so that no two adjacent regions share a color, using as few colors as
  practical — four suffice for any planet, and that is a theorem, not a heuristic. See
  [coloring regions](../theory/region-coloring.md).
- Region identifiers visible, so the view remains readable without relying on color
  alone.
- Deliberately flat and synthetic-looking. It must not resemble terrain; its colors are
  arbitrary labels and should look like it.

This view is the debugging tool for everything else. When the 3D view looks wrong, the
simplified view says whether the problem is in the tessellation or in the rendering.

## Constraints

| Constraint | Source |
| --- | --- |
| No icosahedral subdivision | [vision](../vision.md#the-planet); the twelve symmetry points and grid seams are visible in play |
| Approximates a hex grid | Uniform movement costs, no diagonal ambiguity |
| Works from 1 region to hundreds | Prototypes and small scenarios need tiny worlds |
| Same seed gives the same world everywhere | [vision](../vision.md#whole-numbers-only-in-the-game-logic) |
| Adjacency is a shared edge, never a shared corner | [region coloring](../theory/region-coloring.md#4-the-corner-touching-caveat) |

## What this prototype fakes

- **No game rules.** Regions carry no ownership, resources, or units. Generated terrain
  values only.
- **No persistence.** The world is regenerated from a seed each run.
- **No input beyond camera control** and view switching, plus a hover highlight to prove
  that picking works across the wraparound duplicates.

## Controls to expose

The generator's parameters need to be adjustable at runtime, because the whole point is
to find values that look right:

- Region count N
- Jitter fraction and relaxation iteration count — the two aesthetic knobs from
  [the pipeline](../theory/region-splitting.md#5-the-chosen-pipeline)
- World seed
- View mode
- Border wobble amount

## Success criteria

- Region counts of 1, 2, 3, 7, 20, 100, and 500 all render without crashing or visible
  artifacts. The degenerate low end is enumerated in
  [small and degenerate cases](../theory/region-splitting.md#6-small-and-degenerate-cases).
- The neighbor-count histogram spikes at 6, with exactly twelve degree-5 defects.
- A player cannot locate the twelve pentagons by eye, and cannot find any symmetry axis
  or grid seam.
- The 2D view's wraparound is seamless; the region under the cursor is unambiguous even
  when several dimmed duplicates are visible.
- The simplified view is colored with at most four colors, with no two adjacent regions
  sharing one, and remains readable in grayscale.

## Open questions

- Which rendering stack? The choice has to support both a 3D sphere and a 2D projected
  view without two separate renderers.
- Is the simplified view a third render path, or the 2D view with a different palette and
  no terrain? The latter is less code, but the simplified view arguably wants a graph
  layout rather than a geographic one.
- Does the prototype generate terrain, or only regions? Terrain is what makes the 3D view
  convincing, but it is also a separate problem.
