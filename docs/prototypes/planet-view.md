# Prototype: Planet View

[Prototypes](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## The question

Can we divide the surface of a sphere into hex-like regions, scale that from one region
to hundreds, and show it so that it reads as a world rather than as a computed grid?

The theory behind the answer lives in
[splitting a sphere into regions](../theory/region-splitting.md) and
[coloring regions](../theory/region-coloring.md).

## Status

**Built.** Abstract geometry only: regions, borders, adjacency and colors, with no
terrain and no render-time noise.

**All randomness is currently off, and the default world is an exact truncated
icosahedron** — a real soccer ball, 12 pentagons and 20 hexagons. The first step is
confirming the exact geometry before building anything irregular on top of it.

The readout states the verification outright:

```
PERFECT: 12 pentagons 20 hexagons 90 borders, 2 exact lengths
border pentagon-hexagon 0.457458 rad, hexagon-hexagon 0.315102 rad
spread within a kind: borders 3.8e-15  seed angles 2.2e-16
```

### Which region counts are supported

| Count                                                                                 | What you get                                                                                                                                         |
| ------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| A Goldberg count of class I or II — 12, 32, 42, 92, 122, 162, 252, 272, 362, 482, 492 | The canonical solid. Twelve pentagons, none touching, no cell with four or seven neighbours.                                                         |
| A Goldberg count of class III — 72, 132, 192, 212, 282, 312, 372, 392, 432            | **Falls back to relaxation.** Class III solids are chiral and are not built yet; see [`goldberg`](../../crates/sphere-tessellation/src/goldberg.rs). |
| Anything else — 100, 137, 200, …                                                      | Falls back to relaxation. Never fails, but the guarantees go.                                                                                        |

The fallback degrades gracefully at first and then not so gracefully. Measured with the
default settings:

| asked | pentagons | cells that are neither pentagon nor hexagon |
| ----- | --------- | ------------------------------------------- |
| 20    | 12        | 0, but the pentagons cluster                |
| 100   | 15        | 3                                           |
| 137   | 14        | 2                                           |
| 200   | 36        | **24**                                      |

So a non-canonical count always gives *a* world, but at 200 regions a fifth of the map is
malformed. Pick a canonical count.

Region count is set with `--regions N`, or with `-` and `+` at runtime (hold shift
for steps of ten). Because the solid's size is fixed at 32, any of `--regions`,
`--seed`, `--jitter` or `--relax` switches to a generated world automatically, and
combining one with `--soccer` is rejected rather than silently ignored.

Press `S`, or pass `--generated`, for a generated world instead. With jitter and
relaxation both at zero that is a bare Fibonacci lattice, which at 32 regions gives
`4:4 5:8 6:16 7:4` — deliberately **not** a truncated icosahedron, because a golden
spiral is not the icosahedral arrangement. See
[the soccer ball](../theory/region-splitting.md#the-soccer-ball).

```
scripts/planet-view.ps1                        # or: bash scripts/planet-view.sh
scripts/planet-view.sh --regions 60 --seed 7 --jitter 0.35
scripts/planet-view.sh --soccer                # the truncated icosahedron, as a reference
scripts/planet-view.ps1 --help
```

The scripts are in [`scripts/`](../../scripts/README.md) and pass arguments straight
through; `cargo run --release -p planet-view` does the same thing by hand.

| Control     | Effect                                                       |
| ----------- | ------------------------------------------------------------ |
| drag        | turn the sphere, in any direction, forever                   |
| wheel       | zoom about the cursor                                        |
| `P`         | fan the ball out flat, or fold it back into a globe          |
| `S`         | a real soccer ball, or back to a generated world             |
| `L` `B` `D` | toggle labels, borders, duplicate dimming                    |
| `R`         | new seed                                                     |
| `-` `+`     | fewer or more regions, regenerated live; hold shift for tens |
| `0`         | reset the view                                               |
| `Esc`       | quit                                                         |

`--capture FILE.png` renders a single frame headlessly and exits, which is how the view
gets checked without a human at the window.

## The view

Hold a ball. Fan it out flat and you see all of it at once: the point facing you is
undistorted at the centre, everything else stretches outward, and the point behind the
ball becomes the entire rim. To look elsewhere, fold it up, turn it, fan it out again.

That is the whole interaction model, and the projection is the azimuthal equidistant one
centred on the view direction. Two modes, on `P`:

- **Fanned** (default) — the whole world in one disc. Past the rim the projection covers
  the sphere again, in rings, so the plane fills with dimmed repeats and there is no
  background and no edge anywhere.
- **Globe** — the ball held together, as the eye would see it. One hemisphere, no
  repeats, space around it.

Each region is drawn at full strength exactly once, inside the first disc. Every repeat
is dimmed, and the cursor is replicated dimmed onto each of them.

### Why not a flat map

An earlier version of this prototype *was* a flat equirectangular map that wrapped
horizontally and vertically. It was replaced, because a flat map of a sphere cannot wrap
by translation — the sphere is simply connected and has no nontrivial covering space, so
any flat map must fold, and the folds land on the poles. Walking north over a pole puts
you on the opposite meridian heading south, which a flat map has to draw as a mirror.

That mirror is honest, but it makes two arbitrary places special, permanently smears
polar regions, and reads as broken. Fanning out has no poles at all: the projection's
axis is wherever you happen to be looking. The argument in full is in
[wrapping](../theory/region-splitting.md#10-wrapping).

### What is still distorted

The rim, badly — the far point is smeared around an entire circle. The difference from a
cylindrical projection is that the rim is a property of the current view rather than of
the world, and is one turn of the ball away from being the centre. See
[distortion](../theory/region-splitting.md#9-distortion).

## What has been learned

- **The regions really are equal-sized.** Measured on the sphere: at 20 regions the
  largest is 107% of the mean and the smallest 94%, a ratio of 1.15. Everything a player
  might read as uneven is projection, not tessellation.
- **Rasterizing beats projecting polygons.** Every pixel independently asks the sphere
  which region it lands in. That removes the antimeridian split, the polar special case,
  and the degenerate low region counts all at once — those are artifacts of building
  polygons, and this builds none. It costs one nearest-seed search per pixel, so it is a
  prototype technique rather than a shipping one.
- **Borders belong in a second pass.** Detecting them as changes in the resolved
  region-id buffer, rather than from distances to seeds, makes them an even width at any
  zoom under either projection with no per-projection maths.
- **Four colors, every time.** The exact search has never needed the greedy fallback,
  which matches the Four Color Theorem and confirms the adjacency graph is really planar.
- **Sampling adjacency does not work.** The first version sampled each candidate pair's
  bisector at 512 points and silently dropped short borders — enough to break Euler's
  formula by 120 regions. The constraint is exactly solvable instead.
- **Swapping the graphics engine cost one crate and four lines.** The prototype was
  built on `minifb`, which has no vsync and therefore tore while panning. Moving to
  Bevy changed nothing in the model, the camera or the rasterizer — they cannot name an
  engine type, so they could not be affected. The whole migration was a new adapter
  crate plus the composition root's plugin assembly. That is the separation in
  [architecture](../architecture.md) paying for itself.
- **The exact solid verifies to floating point precision.** All 60 pentagon-hexagon
  borders are the same length and all 30 hexagon-hexagon borders are the same length,
  with a spread within each kind of about 1e-15. Two exact lengths, differing by 31%.
- **Asking for 32 regions does not give you a soccer ball.** It reliably gives the right
  *census* once relaxed — twelve pentagons and twenty hexagons, forced by the degree
  deficit — but almost never the soccer ball *arrangement*, because isolating all twelve
  pentagons is a much stronger condition. Measured at 2 of 72 parameter combinations.
  See [the soccer ball](../theory/region-splitting.md#the-soccer-ball).

## How it is put together

Four crates and a thin root, so that the engine stays swappable:

| Crate                                                               | Responsibility                                                                 |
| ------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| [`sphere-tessellation`](../../crates/sphere-tessellation/README.md) | The sphere, the adjacency graph, the reference solid. No dependencies.         |
| [`graph-coloring`](../../crates/graph-coloring/README.md)           | Few-colour assignment. No dependencies, no geometry.                           |
| [`planet-render`](../../crates/planet-render/README.md)             | Camera, projections, software rasterizer, application state. **No engine.**    |
| [`planet-bevy`](../../crates/planet-bevy/README.md)                 | Window, input, vsync presentation. The only crate that knows an engine exists. |
| [`planet-view`](../../prototypes/planet-view/README.md)             | Composition root. Wiring only.                                                 |

The rasterizer is still a CPU loop — Bevy is used for the window, the input and the
vsync'd presentation of a pixel buffer, not to draw the sphere. Drawing regions on the
GPU is an open question, not a decision that has been made.

Because `planet-render` has no engine in it, a whole session can be driven in a unit
test with no window open, and `--capture` renders a PNG without starting Bevy at all.

## Constraints

| Constraint                                        | Source                                                                                           |
| ------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| No icosahedral subdivision                        | [vision](../vision.md#the-planet); the twelve symmetry points and grid seams are visible in play |
| The world is a sphere, not a torus                | [vision](../vision.md#the-world-is-a-sphere-not-a-torus)                                         |
| Approximates a hex grid                           | Uniform movement costs, no diagonal ambiguity                                                    |
| Works from 1 region to hundreds                   | Prototypes and small scenarios need tiny worlds                                                  |
| Same seed gives the same world everywhere         | [vision](../vision.md#whole-numbers-only-in-the-game-logic)                                      |
| Adjacency is a shared edge, never a shared corner | [region coloring](../theory/region-coloring.md#4-the-corner-touching-caveat)                     |
| Nowhere on the sphere is a special place          | It is a planet; the poles are not landmarks                                                      |

## What this prototype fakes

- **No game rules.** Regions carry no ownership, resources, or units.
- **No terrain and no border noise.** Abstract geometry only, for now.
- **No persistence.** The world is regenerated from a seed each run.
- **No input beyond camera control**, view toggles, and a hover highlight.

## Success criteria

- [x] Region counts of 1, 2, 3, 7, 20, 100 and 500 all render without crashing or
      visible artifacts.
- [x] The neighbour-count histogram spikes at 6, with a total degree deficit of exactly
      twelve — the twelve pentagons, observed rather than assumed.
- [x] Every region is visible at full strength in one glance, from any orientation.
- [x] Turning never reaches an edge, a fold, or a mirror.
- [x] No location on the sphere is special: the region at the centre of the view takes
      up about the same screen area wherever you point it.
- [x] The simplified view uses at most four colors, no two adjacent regions share one,
      and it stays readable in grayscale.
- [ ] A player cannot locate the twelve pentagons by eye, and cannot find any symmetry
      axis or grid seam. *Needs a human judgement that has not been made yet.*

## Open questions

- Terrain, border noise, and the controlled randomness that makes the world look
  geographic rather than computed — none of it exists yet.
- At what region count does the per-pixel nearest-seed search stop being fast enough,
  and what replaces it? 500 regions currently costs about 16 ms a frame.
- Does the game want a whole-world overview that is not centred on the player?
- Should the globe mode shade the sphere, or stay flat like the fanned mode?
