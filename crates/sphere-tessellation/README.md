# sphere-tessellation

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

Divides the surface of a sphere into hex-dominant regions and reports which of them
touch.

The theory is in [splitting a sphere into regions](../../docs/theory/region-splitting.md).
This crate is that document's pipeline, steps 1 through 5.

## Public surface

```rust
// All randomness is off by default for now: jitter 0, relaxation 0, seed inert.
let tessellation = Tessellation::generate(Params {
    region_count: 32,  // the truncated icosahedron's face count
    jitter: 0.0,       // seed displacement as a fraction of mean spacing
    relaxation: 0,     // Lloyd passes
    seed: 1,
});

// A perfect truncated icosahedron is constructed, not generated.
let solid = Tessellation::soccer_ball();
assert!(solid.verify_truncated_icosahedron().is_perfect());

tessellation.seeds;       // Vec<Vec3>, one per region — rendering only
tessellation.neighbours;  // Vec<Vec<u32>>, sorted — the game logic's whole view

// The polyhedron itself, for drawing it in three dimensions.
let shape = sphere_tessellation::solid(&solid.seeds, &solid.neighbours);
assert_eq!(shape.corners.len(), 60);   // where three regions meet
assert_eq!(shape.cells.len(), 32);     // one polygon per region
assert_eq!(shape.euler_characteristic(), 2);
```

The two outputs are deliberately separated. `neighbours` is integers with no geometry
in it, which is what crosses into the game logic; `seeds` is floating point, and stays
on the rendering side of that line. See
[architecture](../../docs/architecture.md#game-logic).

`jitter` and `relaxation` are the aesthetic controls, and both are **currently zero**
so that the exact geometry can be confirmed first. When they come back: 0.2 to 0.4
jitter looks natural, and how much relaxation is needed depends heavily on the region
count — a few dozen cells need twelve to sixteen passes before every cell is reliably
a pentagon or hexagon, while hundreds settle in three or four. Sweeping the two at 32
regions over sixteen seeds picked jitter 0.20 with 16 passes.

Relaxation is the expensive part: 9 ms at 32 regions, but nearly two seconds at 500.

Note that no generator setting produces a truncated icosahedron. The Fibonacci lattice
is a golden spiral, not the icosahedral arrangement; `soccer_ball()` constructs the
solid directly, and `verify_truncated_icosahedron()` reports every measurable property
of it.

## How it works

| Module        | Responsibility                                                             |
| ------------- | -------------------------------------------------------------------------- |
| `vec3`        | Unit vectors, longitude and latitude, great-circle movement                |
| `rng`         | SplitMix64, hand-written so results are identical on every platform        |
| `lattice`     | Fibonacci lattice, jitter, Lloyd relaxation, nearest-seed queries          |
| `adjacency`   | Which regions share a border, and graph sanity checks                      |
| `icosahedral` | The icosahedron, and the truncated icosahedron with all ninety edges equal |
| `goldberg`    | `GP(m, n)`: the canonical hex-dominant tilings, class I and II             |
| `quality`     | Equal area, six neighbours, compactness, isolated pentagons                |

Adjacency is computed exactly rather than by sampling. Points equidistant from two
seeds form a great circle; every other seed rules out exactly half of it; intersecting
half-circles leaves a single arc. If that arc has positive length the two regions are
neighbours. **Corner contact is not adjacency** — that arc has length zero — which is a
precondition of the four-color guarantee in
[region coloring](../../docs/theory/region-coloring.md#4-the-corner-touching-caveat).

Cost is O(n) per candidate pair, so O(n^3) overall before the distance cutoff prunes
it. Fine into the hundreds. Beyond that the right answer is a Delaunay triangulation via
3D convex hull, which is deferred until something needs it.

## Tests

The interesting ones are topological invariants rather than golden values, because they
catch whole classes of error at once:

- `total_degree_deficit_is_always_twelve` — summing `6 - degree` over every region must
  give exactly 12, for any region count. This is Euler's formula in disguise, and it is
  the twelve mandatory pentagons showing up in the output.
- `the_deficit_holds_across_seeds_jitter_and_counts` — the same invariant over 54
  parameter combinations. An earlier sampling-based adjacency passed the simple cases
  and failed here.
- `a_relaxed_lattice_satisfies_eulers_formula` — `E = 3V - 6` for a sphere triangulation.
- `regions_are_close_to_equal_area_on_the_sphere` - at 20 regions the largest cell is
  107% of the mean and the smallest 94%, a ratio of 1.15. Worth asserting because it is
  what separates "the tessellation is uneven" from "the projection is uneven", and on a
  flat map those look identical.
- `the_voronoi_cells_are_twelve_pentagons_and_twenty_hexagons` and
  `no_two_pentagons_are_adjacent` — the soccer ball, checked against its exactly known
  answer. `Tessellation::soccer_ball()` builds it; `is_soccer_ball()` recognises one.
- `does_thirty_two_regions_make_a_soccer_ball` — records, rather than demands, what the
  generator actually produces at 32 regions. The face counts come out right once
  relaxed; the arrangement almost never does.
- `generation_is_deterministic` — same parameters, same world, bit for bit.
