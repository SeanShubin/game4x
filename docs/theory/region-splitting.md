# Theory: Splitting a Sphere into Regions

[Theory index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

How to divide the surface of a sphere into discrete regions that look and behave like a
hex grid, for any count from one region to several hundred.

Its companion document is [coloring regions](region-coloring.md), which takes the
adjacency graph produced here and assigns colors to it.

This document describes the chosen pipeline in depth.
[Comparing region schemes](../notes/region-schemes.md) is the wider survey it was picked
out of — it measures every known scheme against each other, covers four this document does
not, and reaches a different conclusion under a different set of goals.

## 1. What we actually need

| Requirement                                 | Why                                                                                                                                                     |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Mostly six neighbors per region             | A hex grid has no diagonal ambiguity: every neighbor is a full edge away, movement costs are uniform, and there is no "does the corner count?" question |
| Roughly equal area                          | Region area is a game resource; wildly uneven regions are wildly uneven play                                                                            |
| Roughly equal shape (compact, not slivered) | Slivers look wrong and behave wrong                                                                                                                     |
| No visible global symmetry or seams         | Symmetry reads as artificial; the player should not be able to find the "corners of the world"                                                          |
| Works from N = 1 to N in the hundreds       | Prototypes and small scenarios use tiny worlds                                                                                                          |
| Deterministic from a seed                   | The same seed gives the same world on every machine                                                                                                     |
| Produces a plain adjacency graph            | That graph is all the game logic ever sees                                                                                                              |

Explicitly rejected: **icosahedral subdivision**. Section 4.1 covers what that is and
what exactly is being rejected, because part of it turns out to be unavoidable.

## 2. Why a perfect hex grid on a sphere is impossible

This is not an engineering limitation. It is a theorem, and understanding it explains
every design choice that follows.

Take any convex polyhedron whose faces are the regions. Euler's formula relates its
vertices, edges, and faces:

```
V - E + F = 2
```

Assume the ideal hex grid: every vertex is a corner of exactly three regions
(trivalent, like a honeycomb), and every face is a pentagon or hexagon. Let `F5` be the
pentagon count and `F6` the hexagon count.

Counting edge-ends around vertices, `3V = 2E`, so `V = 2E/3`.
Counting edge-ends around faces, `2E = 5*F5 + 6*F6`.
And `F = F5 + F6`.

Substituting into Euler's formula:

```
2E/3 - E + F = 2
     F - E/3 = 2
     3F - E  = 6
     6F - 2E = 12
6(F5 + F6) - (5*F5 + 6*F6) = 12
                        F5 = 12
```

**Exactly twelve pentagons. Always.** The hexagon count `F6` cancels out entirely — it
can be anything at all, including zero (that is the dodecahedron), and the answer is
still twelve.

The geometric reading is Gauss-Bonnet: a sphere carries a total curvature of `4*pi` that
has to go somewhere. A flat hexagon in a honeycomb contributes none. Each pentagon
contributes an angular defect of `pi/3`, and `12 * pi/3 = 4*pi`. The twelve pentagons
are the sphere's curvature made discrete.

So the design question is never "how do I avoid the defects?" It is **"where do I put
the twelve defects so the player never notices them?"**

## 3. What the game logic gets

Before choosing an algorithm, note where the boundary falls. Whatever tessellation we
pick produces two artifacts:

- **An adjacency graph** — regions as integer identifiers, adjacency as edges. This is
  the *only* thing the game logic sees. It contains no coordinates and no floating
  point, consistent with
  [the integers-only rule](../vision.md#whole-numbers-only-in-the-game-logic).
- **A geometric description** — region outlines as points on a unit sphere, used by the
  renderer and by nothing else.

Generation runs once, at world creation, and may use floating point freely. The
adjacency graph is then frozen. That is the seam: floating point on the geometry side,
integers on the mechanics side.

This is also why the theory here is worth getting right *once* — everything downstream
in the game rules is insulated from it.

## 4. The families of approaches

### 4.1 Geodesic and Goldberg polyhedra (rejected)

Start with an icosahedron, subdivide each triangular face into a triangular grid, and
project the vertices onto the sphere. That is a **geodesic polyhedron**. Its dual — the
polyhedron whose faces correspond to the geodesic's vertices — is a **Goldberg
polyhedron**, and Goldberg polyhedra are hexagons plus exactly twelve pentagons,
inheriting the icosahedron's twelve original vertices. A soccer ball is the smallest
interesting one.

They are classified by two integers `(m, n)`: class I is `(m, 0)`, class II is `(m, m)`,
and class III is everything else (chiral). The region count is `10*(m^2 + m*n + n^2) + 2`.

**This section records the original rejection. It has since been reversed** — see
[vision](../vision.md#icosahedral-subdivision-rejected-then-adopted) and
[the Goldberg section](#goldberg-polyhedra-the-canonical-answer) below. The objections
are kept because three of the four turned out to be about appearance rather than
topology, which is a mistake worth not repeating.

Why it was rejected:

- **The twelve pentagons sit at icosahedral symmetry points.** They are arranged with
  perfect five-fold symmetry, so once a player notices one, the other eleven are
  predictable.
- **Grid seams.** Each of the twenty triangular faces is an internally regular grid, so
  the sphere shows twenty patches of straight rows meeting along visible boundaries.
- **Coarse quantization of N.** Region counts jump 12, 42, 92, 162, 252, 362, 492, ...
  You cannot ask for 100 regions.
- **Distortion is systematic rather than random.** Cells near the pentagons are
  consistently smaller and more distorted than cells at the face centers, in a pattern
  that repeats twenty times.

### The soccer ball

The truncated icosahedron — `GP(1, 1)`, a soccer ball — is the smallest Goldberg
polyhedron with both face kinds and the most familiar object in the whole subject:

|                  |                                                              |
| ---------------- | ------------------------------------------------------------ |
| Faces            | 32: **12 pentagons and 20 hexagons**                         |
| Vertices         | 60                                                           |
| Edges            | 90                                                           |
| Euler            | 60 - 90 + 32 = 2                                             |
| Defining feature | no two pentagons touch; every pentagon is ringed by hexagons |

It is the clearest demonstration that twelve pentagons are mandatory rather than a quirk
of any particular pipeline, and it makes a far better test fixture than a generated
world because its answer is known exactly. The planet view can render it on demand, and
`sphere-tessellation` builds it in `icosahedral`.

**It is currently the default**, as a deliberate first step: confirm the exact geometry,
with all randomness switched off, before building anything irregular on top of it. The
prototype reports a full verification — face counts, border count, pentagon isolation,
and the fact that there are exactly two distinct border lengths with no variation within
either.

That does not change the design decision. It is still rejected as a *world*, for the
reasons above: perfectly symmetric, so once a player finds one pentagon the other eleven
are predictable. It is a starting point to build away from, not a destination.

#### Does asking for 32 regions naturally produce one?

Half of it, and the half that comes free is instructive. Measured across 72 combinations
of jitter, relaxation and seed:

| Outcome                                   | Frequency |
| ----------------------------------------- | --------- |
| Exactly 12 pentagons and 20 hexagons      | 34 of 72  |
| An actual soccer ball, pentagons isolated | 2 of 72   |

The **counts** are forced, not lucky. The degree deficit is always exactly 12, so the
moment every cell happens to be a pentagon or hexagon, twelve and twenty is the only
arithmetic available. What decides whether that happens is relaxation: raw or lightly
relaxed lattices produce squares and heptagons too (a typical unrelaxed run gives
`4:4 5:8 6:16 7:4`, which still sums to a deficit of 12), and by about twelve Lloyd
passes the squares and heptagons have annealed away.

The **arrangement** is not forced at all, and that is the difference between the right
face counts and a soccer ball. Isolating all twelve pentagons is a much stronger
condition, and Lloyd relaxation starting from a golden-spiral lattice almost always
settles into a different local minimum — one with the right census but pentagons sitting
next to each other. It does find the icosahedral arrangement occasionally, under heavy
relaxation, which is unsurprising given that the truncated icosahedron is the optimal
arrangement of 32 points on a sphere; the basin is just narrow.

Which is the useful lesson for the game: the twelve defects are guaranteed, and where
they land is not. That is exactly the property the design wants.

Note what is *not* rejected: the twelve pentagons themselves. Section 2 proves those are
unavoidable in any hex-dominant sphere tiling, including ours. What we reject is placing
them at symmetric, predictable locations. Our approach produces twelve defects too —
scattered, irregular, and individually unremarkable.

### Goldberg polyhedra: the canonical answer

A Goldberg polyhedron `GP(m, n)` places twelve pentagons at the icosahedron's vertices
and hexagons everywhere else. The pair says how to walk from one pentagon to the next
through the lattice: `m` steps one way, turn 60 degrees, `n` more.

| Class | Condition              | Handedness                                            |
| ----- | ---------------------- | ----------------------------------------------------- |
| I     | `n = 0`                | achiral                                               |
| II    | `m = n`                | achiral — the soccer ball is `GP(1,1)`                |
| III   | both nonzero, `m != n` | **chiral**; `GP(m,n)` and `GP(n,m)` are mirror images |

With `T = m^2 + mn + n^2` there are `10T + 2` faces, always exactly twelve of them
pentagons. Only certain counts are therefore available — 12, 32, 42, 72, 92, 122, 132,
162, 192, 212, 252, 272, 282, 312, 362, 372, 392, 432, 482, 492 below five hundred — and
the largest gap between them is 50.

**One count can admit two solids.** 492 faces is both `GP(7,0)` and `GP(5,3)`, because
`49 = 7^2 = 5^2 + 5*3 + 3^2`. They are different shapes with the same number of regions,
so "one region count, one canonical shape" is not a rule that survives contact with
arithmetic. Beyond the Goldberg counts there is no canonical shape at all, and genuinely
distinct arrangements are the norm rather than the exception.

Measured, with subdivision along the sphere:

| regions | GP    | pentagons | touching           | area ratio | compactness |
| ------- | ----- | --------- | ------------------ | ---------- | ----------- |
| 12      | (1,0) | 12        | n/a, all pentagons | 1.00       | 0.99        |
| 32      | (1,1) | 12        | 0                  | 1.54       | 0.93        |
| 42      | (2,0) | 12        | 0                  | 1.13       | 0.92        |
| 92      | (3,0) | 12        | 0                  | 1.17       | 0.91        |
| 122     | (2,2) | 12        | 0                  | 1.17       | 0.91        |
| 162     | (4,0) | 12        | 0                  | 1.15       | 0.91        |
| 272     | (3,3) | 12        | 0                  | 1.18       | 0.91        |
| 492     | (7,0) | 12        | 0                  | 1.14       | 0.91        |

### 4.2 Cube sphere and quad sphere

Project a cube's six faces onto the sphere. Indexing is simple, addressing is cheap, and
it is the standard choice in planet renderers. But it gives a *quad* grid, not a hex
grid, with eight highly visible corner singularities and four-way adjacency ambiguity at
every vertex. Wrong shape for this game.

### 4.3 HEALPix

Twelve equal-area base quads, subdivided recursively. Its selling point is exactly equal
cell areas plus an efficient hierarchical index, which is why astronomy uses it for
all-sky maps. It is still quads, still has visible structure, and still quantizes N to
`12 * 4^k`. Worth knowing about because exact equal area is genuinely hard to get any
other way.

### 4.4 Spherical Voronoi tessellation of a point set (chosen)

Scatter N points on the sphere, and let each region be the set of surface points closest
to one of them. This is a **spherical Voronoi diagram**; the points are called seeds or
generators.

Three properties make it the right choice:

- **N is exactly what you asked for.** One seed, one region. Any N, no quantization.
- **Voronoi cells of a well-spread point set are hexagon-dominant by default.** This is
  not a coincidence: the hexagonal lattice is the optimal circle packing of the plane,
  and any locally-even point distribution approximates it. The twelve mandatory
  pentagons appear on their own, wherever the point set happens to put them.
- **Randomness is a dial, not a fixed property.** Perfectly spread seeds give a
  near-regular grid; jittered seeds give organic irregularity. Same algorithm, one
  parameter.

The costs are that generation is a real computation rather than a lookup, and that
regions are addressed by index rather than by coordinate. Both are acceptable —
generation runs once, and the game logic wants indices anyway.

## 5. The chosen pipeline

### Step 1: Seed points, the Fibonacci sphere lattice

We need N points spread as evenly as possible over the sphere. The **Fibonacci lattice**
(also called the golden spiral) does this in closed form, in O(N), for any N:

```
golden ratio  phi = (1 + sqrt(5)) / 2

for i in 0 .. N-1:
    z     = 1 - (2*i + 1) / N        # equal-area bands in z
    r     = sqrt(1 - z*z)
    theta = 2*pi * i / phi           # golden-angle increment
    point = (r*cos(theta), r*sin(theta), z)
```

Two ideas are doing the work. Sampling `z` uniformly produces equal-area horizontal
bands, because a sphere's surface area is uniform in `z` — that is Archimedes'
hat-box theorem. Advancing the angle by the golden angle keeps the points from falling
into rows, because the golden ratio is the irrational number worst approximated by
rationals, so the spiral never almost-repeats.

The result has excellent local uniformity and no symmetry group. Alternatives with
marginally better theoretical spread exist — solving the **Thomson problem** (minimize
mutual electrostatic repulsion) or using Rakhmanov-Saff-Zhou spiral points — but they
cost iteration for a small gain, and we are about to perturb the points deliberately
anyway.

### Step 2: Jitter, the controlled randomness

Displace each seed by a random tangential offset drawn from a seeded PRNG, with
magnitude bounded by a fraction of the mean seed spacing, which is about `2/sqrt(N)`
radians.

The jitter fraction is the single knob controlling how natural the map looks:

| Jitter fraction | Result                                                                    |
| --------------- | ------------------------------------------------------------------------- |
| 0.0             | Near-regular grid; visibly computed                                       |
| 0.2 - 0.4       | Irregular but still hexagon-dominant and compact — **the intended range** |
| above 0.6       | Slivers, degenerate cells, wildly unequal areas                           |

The PRNG is seeded from the world seed, so the jitter is reproducible.

### Step 3: Relaxation with Lloyd's algorithm

Jitter buys irregularity at the cost of area variance. Lloyd's algorithm buys some of
the evenness back without restoring the symmetry:

```
repeat k times:
    compute the Voronoi cells of the current seeds
    move each seed to the centroid of its own cell, renormalized onto the sphere
```

The fixed point is a **centroidal Voronoi tessellation** (CVT), in which every seed is
its own cell's centroid. Convergence is monotone but slow, which is convenient here — we
do not want the fixed point, we want to stop partway.

- `k = 0` — raw jitter, high area variance
- `k = 2` to `4` — compact cells, still organic, **once there are enough of them**
- `k` large — approaches a regular hexagonal arrangement, and the irregularity we paid
  for is gone again

The pair (jitter fraction, relaxation count) is therefore the entire aesthetic control
surface for the planet's shape.

**How much relaxation is enough depends strongly on the region count**, and the small
end needs far more than the two-to-four above suggests. At 32 regions, three passes
still leaves squares and heptagons in the mix - a typical run gives `4:1 5:11 6:19 7:1`
- and it takes about twelve to sixteen passes before every cell is reliably a pentagon
or a hexagon. Larger counts settle much sooner, because a golden-spiral lattice is
already locally well behaved when there are hundreds of points and only globally
awkward when there are a few dozen.

**The current defaults switch all of this off:** jitter 0 and relaxation 0, so
generation is entirely deterministic and the seed does nothing. That is a deliberate
starting point rather than a tuned setting — the project is confirming the exact
geometry first and will turn the randomness back on afterwards.

When it does, the settings to return to are **jitter 0.20 with 16 relaxation passes**,
found by sweeping the two against each other at 32 regions across sixteen seeds and
taking the combination that always produced clean topology without collapsing onto the
icosahedral arrangement. Relaxation is not free — it costs
`iterations x samples x regions`, which is 9 ms at 32 regions but nearly two seconds at
500.

### Step 4: Delaunay triangulation via 3D convex hull

We need the Voronoi diagram, and the standard route is through its dual, the Delaunay
triangulation. On a sphere there is an elegant shortcut:

> For points on a sphere, the **convex hull** of those points, taken as a 3D polyhedron,
> *is* the Delaunay triangulation of the sphere's surface.

So one call to a 3D convex hull routine gives the triangulation. The hull's triangular
faces are the Delaunay triangles and the hull's edges are the Delaunay edges. Cost is
O(N log N).

Equivalent alternatives, if a hull implementation is inconvenient: stereographic
projection to the plane followed by an ordinary planar Delaunay triangulation (correct,
but needs care near the projection pole), or a direct spherical incremental algorithm.

### Step 5: Take the dual to get the Voronoi cells

The Voronoi diagram is the dual of the Delaunay triangulation, and constructing it is
mechanical:

- **Each Delaunay triangle** contributes one Voronoi vertex: the circumcenter of that
  triangle on the sphere, which is the normalized center equidistant from its three
  points.
- **Each seed's region** is the polygon formed by the circumcenters of all triangles
  incident to that seed, taken in rotational order.
- **Each Delaunay edge** is an adjacency between two regions, and corresponds to exactly
  one shared Voronoi edge.

That last point is the important one: **the Delaunay edge set is the adjacency graph the
game logic wants.** Adjacency is defined by a shared edge of positive length, never by a
shared corner — which matters directly for
[region coloring](region-coloring.md#4-the-corner-touching-caveat).

Store the result as a half-edge structure (a DCEL) so that "walk the boundary of this
region" and "list this region's neighbors in order" are both cheap.

### Step 6: Render-time detail

The Voronoi polygon is the *true* border. The renderer draws something noisier:
subdivide each border edge and displace the midpoints with seeded fractal noise
(midpoint displacement), so that coastlines and frontiers look eroded rather than
straight.

One constraint: a border edge is shared by two regions, so both must displace it
identically. Derive the noise seed from the unordered pair of region identifiers, not
from the region doing the drawing. Otherwise the map develops cracks along every border.

None of this touches the adjacency graph. Two regions are neighbors or they are not; how
wiggly the line between them is drawn is purely a rendering decision.

## 6. Small and degenerate cases

The requirement is one region to hundreds, and the low end needs explicit handling:

| N               | Behavior                                                                                                                                                                                                             |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1               | The single region is the entire sphere. No borders, no adjacency, no triangulation. Special-cased.                                                                                                                   |
| 2               | Two caps separated by one great circle. The adjacency graph is a single edge, and the shared border is a closed loop rather than an arc with two endpoints, so the usual DCEL invariants do not hold. Special-cased. |
| 3               | Three lunes meeting at two antipodal points. Also degenerate for a hull-based approach. Special-cased.                                                                                                               |
| 4 to about 12   | The convex hull works, but cells are large spherical polygons with few neighbors each. Not hex-like, and unavoidably so.                                                                                             |
| about 20 and up | Hexagon-dominant behavior emerges.                                                                                                                                                                                   |
| hundreds        | The intended range.                                                                                                                                                                                                  |

Additionally, the convex hull of coplanar or near-coplanar points is degenerate, so the
seed generator must never produce them. The Fibonacci lattice never does for N >= 4, but
a jitter magnitude large enough to collapse points onto each other would.

## 7. Quality metrics

The pipeline has knobs, so it needs measurements. Each of these is cheap to compute and
worth asserting in tests:

- **Neighbor-count histogram.** Expect a strong spike at 6, a total degree-5 deficit
  accounting for exactly twelve pentagons, and a small tail at 5 and 7. A large count of
  4s or 8s means jitter is too high or relaxation too low.
- **Area ratio**, `max_area / min_area`. Target below 2.0 in the intended parameter
  range.
- **Compactness** per region, the isoperimetric quotient `4*pi*A / P^2`: it is 1.0 for a
  circle and about 0.907 for a regular hexagon. Anything below roughly 0.6 is a sliver.
- **Minimum edge length** relative to the mean. Near-zero-length edges are numerically
  fragile and visually indistinguishable from a corner touch.
- **Graph invariants.** The adjacency graph must be connected and planar and satisfy
  `V - E + F = 2`. This is a strong end-to-end correctness check: a bug in the dual
  construction almost always breaks Euler's formula.
- **Determinism.** Same seed and same N give a byte-identical adjacency graph, on every
  platform.

## 8. Showing it flat: fanning the ball out

Hold a ball. Fan it out flat in front of you and you can see all of it at once: the
point facing you sits at the centre undistorted, everything else is pushed outward and
stretched, and the point directly behind the ball has nowhere to go but the entire rim.
To look somewhere else, fold the ball up, turn it, and fan it out again.

That is the **azimuthal equidistant projection centred on the view direction**, and it
is what the planet view uses. Distance from the centre of the screen is exactly angular
distance on the sphere, so the radial scale is truthful everywhere and only the
tangential scale stretches:

| Angle from centre | Tangential stretch                             |
| ----------------- | ---------------------------------------------- |
| 0 degrees         | 1.00, exact                                    |
| 30 degrees        | 1.05                                           |
| 60 degrees        | 1.21                                           |
| 90 degrees        | 1.57                                           |
| 150 degrees       | 4.53                                           |
| 180 degrees       | infinite — the far point becomes the whole rim |

The stretch factor is `theta / sin(theta)`, and the important word is *centred*: the
projection is rebuilt around wherever you are looking, so nothing on the sphere is
permanently distorted. Turn the ball and the distortion turns with it.

### Why not a cylindrical projection

The obvious alternatives — equirectangular, Mercator, Lambert equal-area — are all
built from a fixed polar axis. That has three consequences, and they are why this
project does not use one:

- **The poles are permanently smeared.** Distortion is a function of latitude alone, so
  a polar region is stretched in every view, forever. Equirectangular stretches it by
  `1 / cos(latitude)`, which is 2x at 60 degrees and unbounded at the pole itself.
- **Two arbitrary places become special.** Nothing about the game makes the poles
  different from anywhere else, but the map insists they are.
- **The map has folds.** Section 10.

An equal-area cylindrical projection fixes the *area* complaint and makes the *shape*
complaint worse: preserving area while stretching east-west by `1 / cos(latitude)`
requires squashing north-south by `cos(latitude)`, so the anisotropy at the pole goes as
`1 / cos^2`. Fanning out sidesteps the trade rather than picking a side of it.

## 9. Distortion

### It is unavoidable

No projection of a sphere onto a plane preserves distance. This is not a matter of
finding a better formula: it is Gauss's *Theorema Egregium*, which says Gaussian
curvature is intrinsic, preserved by any distance-preserving map. A sphere of radius
`R` has curvature `1/R^2`; a plane has curvature `0`. No distance-preserving map between
them exists, anywhere, even locally.

Every projection therefore distorts something. The only decision is what to sacrifice,
and where.

### The regions themselves are not the problem

Worth separating from the projection, because on a flat map they look the same. Measured
over the sphere, with the default jitter and three relaxation passes:

| Regions | Largest      | Smallest    | Ratio |
| ------- | ------------ | ----------- | ----- |
| 20      | 104% of mean | 94% of mean | 1.10  |
| 60      | 105%         | 94%         | 1.12  |
| 150     | 105%         | 94%         | 1.12  |

(Measured with the current zeroed defaults. The figures are a little tighter than a
jittered world's, which is expected — jitter buys irregularity by spending evenness.)

So the tessellation is doing its job: the regions really are near enough the same size.
Any unevenness the player sees is the projection, not the world. `region_areas` in the
tessellation crate is the measurement, and it is asserted in a test.

### Why a region's sides are different lengths

Distinct from the projection, and worth separating out, because it is a property of the
tessellation rather than of the view. Three reasons, only one of which is a choice.

**A sphere cannot be tiled with regular polygons of one kind.** Section 2 already rules
out all-hexagons. Even setting the twelve pentagons aside, regular hexagons tile only a
flat plane; on a sphere something has to give, and edge length is one of the things
that gives.

**Our soccer ball is a Voronoi one, not the Archimedean one.** The Archimedean truncated
icosahedron famously has all 90 edges equal. Ours does not, and the reason is exact: in
the Archimedean solid the two kinds of face centre sit at *different distances from the
centre of the solid*, but a spherical Voronoi diagram sees only directions - every seed
lies on the same sphere - so it cannot reproduce that difference. Measured:

| Border kind         | Count | Length     | Variation within the kind |
| ------------------- | ----- | ---------- | ------------------------- |
| pentagon to hexagon | 60    | 0.4575 rad | none, to 9 decimal places |
| hexagon to hexagon  | 30    | 0.3151 rad | none, to 9 decimal places |

Within each kind every border is identical - the solid really is that symmetric - but
the two kinds differ by **31%**. So on the reference soccer ball each pentagon has five
equal sides, and each hexagon has three long sides and three short ones, alternating.
Making all ninety equal would need a *power diagram*, a weighted Voronoi giving the
pentagon seeds a different weight from the hexagon seeds. Worth knowing about; not worth
building for a reference solid.

**In a generated world the seeds are irregular on purpose.** This is the controlled
randomness the design asks for, so cells are not regular polygons and their sides vary
continuously. Longest side over shortest side, within a single region, at 32 regions:

| Relaxation   | Median    | Mean | Worst | Degrees             |
| ------------ | --------- | ---- | ----- | ------------------- |
| 0            | 8.28x     | 44x  | 307x  | `4:4 5:8 6:16 7:4`  |
| 3            | 4.69x     | 11x  | 55x   | `4:1 5:11 6:19 7:1` |
| 8            | 2.61x     | 5.0x | 21x   | `5:12 6:20`         |
| 16 (default) | **1.73x** | 7.1x | 67x   | `5:12 6:20`         |
| 40           | 1.59x     | 34x  | 514x  | `5:12 6:20`         |

The median is the honest answer to "how uneven does a region look": at the default,
typical sides are within about 1.7x of each other, which reads as an irregular but
believable cell.

The mean and the worst case tell a different story, and it is worth understanding
because it never goes away. **Some borders are very nearly zero length.** A Voronoi
vertex normally joins three cells; occasionally four cells come close to meeting at one
point, and the two diagonal ones are then left sharing a sliver of an edge instead of
touching at a corner. Those slivers are the 67x and 514x outliers, and more relaxation
does not remove them - it tightens the typical cell while making near-four-way vertices
somewhat more likely.

Those slivers are why
[the quality metrics](#7-quality-metrics) include a minimum-edge-length check, and why
adjacency has to be computed exactly rather than sampled: a sampling method drops
exactly these borders, and dropping one silently turns a shared edge into a corner
contact, which is the distinction the
[four-colour guarantee](region-coloring.md#4-the-corner-touching-caveat) rests on.

### Where the distortion goes

Under a cylindrical projection, distortion is bolted to the polar axis: it is a function
of latitude alone, identical at every longitude, and no amount of panning moves it.

Under the fanned projection it is radial from wherever you are looking. The centre of
the screen is exact, the rim is smeared, and turning the sphere moves both. No location
on the sphere is inherently distorted, which is the property that matters for a game
where every region is supposed to be equivalent.

The cost is that the rim is *badly* distorted — worse than any cylindrical projection is
at its poles, since the far point is smeared around an entire circle. The difference is
that the rim is a property of the current view rather than of the world, and it is one
turn of the ball away from being the centre.

## 10. Wrapping

### A flat map of a sphere cannot wrap by translation

The intuitive thing to want is a hex grid that repeats in both directions, so panning
never ends and every region is identical. That is a **torus**: a flat surface, zero
curvature, Euler characteristic 0, tiled perfectly by hexagons with no defects and no
distortion at all.

A sphere cannot do it, and the reason is topological rather than technical. A flat map
that wraps purely by translation is a quotient of the plane by a lattice — the plane
covers it. But **the sphere is simply connected, so it has no nontrivial covering
space**: no torus, and no plane, can cover a sphere. Any flat map of a sphere must
therefore *fold*, and the folds land on the poles.

Concretely: walk north from longitude 0, latitude 80 degrees. Twenty degrees later you
are at the pole. Keep walking straight and twenty degrees after that you are at latitude
80 degrees, **longitude 180**, heading south. From the north pole every direction is
south. On a flat map with the pole along the top edge, that is: leave the top at one
longitude, re-enter at the opposite one, upside down. A glide reflection — a mirror.

That mirror is not an artifact of a badly chosen projection. It is the only honest way to
draw walking over a pole. A pure vertical translation would mean stepping off the north
pole and arriving at the south pole, which is a teleport, not a surface.

### The repeating rings

Fanning out wraps too, but radially, and without folding.

The whole sphere fits inside the disc of radius `pi`. Keep going outward and the
projection simply covers the sphere again — a geodesic that passes the far point carries
on and comes back. So the plane beyond the first disc fills with further copies of the
world, in rings, alternating between upright and turned inside out:

```
radius     0 .. pi     pi .. 2pi    2pi .. 3pi
contents   the world   the world    the world
drawn      full        dimmed       dimmed
```

This gives the property the flat map was reaching for. There is no background and no
edge anywhere; every pixel shows some part of the world; each region appears at full
strength exactly once, inside the first disc, with every further appearance dimmed. It
is the same "render once, dim the duplicates" rule, arrived at without a fold.

### Panning is rotation

Because the projection is centred on the view direction, moving the view means turning
the sphere rather than sliding a plane. Dragging composes small rotations about the
*view* axes, so there is no fixed up-vector to lose, no gimbal lock, and no special
behaviour at the poles — a pole is a point that slides past like any other. You can drag
in any direction forever and never reach an edge, because there is no edge to reach.

## 11. Open questions

- Should there be a whole-world overview that is not centred on the player, for
  planning? Fanning out already shows every region at once, but the rim is a poor place
  to read anything.
- How far out should the repeating rings be drawn before they stop informing and start
  distracting?
- Are the twelve pentagons worth detecting and deliberately relocating (into oceans,
  say), or does jitter already hide them well enough?
- Should relaxation be area-weighted, so that deliberately non-uniform region density —
  dense continents, sparse oceans — becomes achievable?
- Is a fixed tessellation enough, or will regions need to subdivide during play?

## 12. References

**The impossibility result**

- Euler's polyhedron formula, and its corollary that any trivalent polyhedron made of
  pentagons and hexagons has exactly twelve pentagons — the fullerene and Goldberg
  result.
- Gauss-Bonnet theorem: the total curvature of a sphere is `4*pi`, distributed as
  angular defect at the vertices. Descartes' theorem on total angular defect is the
  discrete form.

**Tessellations**

- Goldberg, M. (1937). "A class of multi-symmetric polyhedra." The Goldberg polyhedra.
- Gorski et al. (2005). "HEALPix: A Framework for High-Resolution Discretization and
  Fast Analysis of Data Distributed on the Sphere."

**Point distributions**

- Gonzalez, A. (2010). "Measurement of Areas on a Sphere Using Fibonacci and
  Latitude-Longitude Lattices."
- Rakhmanov, Saff and Zhou (1994). "Minimal discrete energy on the sphere." Spiral
  points and the Thomson problem.
- The Thomson problem: minimal-energy configurations of N points on a sphere.

**Voronoi and Delaunay**

- Lloyd, S. (1982). "Least squares quantization in PCM." Lloyd's algorithm.
- Du, Faber and Gunzburger (1999). "Centroidal Voronoi Tessellations: Applications and
  Algorithms."
- Brown, K. Q. (1979). Voronoi diagrams from convex hulls — the duality result that
  turns the spherical case into a hull computation.
- Renka, R. J. (1997). "Algorithm 772: STRIPACK — Delaunay Triangulation and Voronoi
  Diagram on the Surface of a Sphere."
