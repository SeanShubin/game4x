# Theory: Splitting a Sphere into Regions

[Theory index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

How to divide the surface of a sphere into discrete regions that look and behave like a
hex grid, for any count from one region to several hundred.

Its companion document is [coloring regions](region-coloring.md), which takes the
adjacency graph produced here and assigns colors to it.

## 1. What we actually need

| Requirement | Why |
| --- | --- |
| Mostly six neighbors per region | A hex grid has no diagonal ambiguity: every neighbor is a full edge away, movement costs are uniform, and there is no "does the corner count?" question |
| Roughly equal area | Region area is a game resource; wildly uneven regions are wildly uneven play |
| Roughly equal shape (compact, not slivered) | Slivers look wrong and behave wrong |
| No visible global symmetry or seams | Symmetry reads as artificial; the player should not be able to find the "corners of the world" |
| Works from N = 1 to N in the hundreds | Prototypes and small scenarios use tiny worlds |
| Deterministic from a seed | The same seed gives the same world on every machine |
| Produces a plain adjacency graph | That graph is all the game logic ever sees |

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

Why it is rejected:

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

Note what is *not* rejected: the twelve pentagons themselves. Section 2 proves those are
unavoidable in any hex-dominant sphere tiling, including ours. What we reject is placing
them at symmetric, predictable locations. Our approach produces twelve defects too —
scattered, irregular, and individually unremarkable.

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

| Jitter fraction | Result |
| --- | --- |
| 0.0 | Near-regular grid; visibly computed |
| 0.2 - 0.4 | Irregular but still hexagon-dominant and compact — **the intended range** |
| above 0.6 | Slivers, degenerate cells, wildly unequal areas |

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
- `k = 2` to `4` — **the intended range**: compact cells, still organic
- `k` large — approaches a regular hexagonal arrangement, and the irregularity we paid
  for is gone again

The pair (jitter fraction, relaxation count) is therefore the entire aesthetic control
surface for the planet's shape.

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

| N | Behavior |
| --- | --- |
| 1 | The single region is the entire sphere. No borders, no adjacency, no triangulation. Special-cased. |
| 2 | Two caps separated by one great circle. The adjacency graph is a single edge, and the shared border is a closed loop rather than an arc with two endpoints, so the usual DCEL invariants do not hold. Special-cased. |
| 3 | Three lunes meeting at two antipodal points. Also degenerate for a hull-based approach. Special-cased. |
| 4 to about 12 | The convex hull works, but cells are large spherical polygons with few neighbors each. Not hex-like, and unavoidably so. |
| about 20 and up | Hexagon-dominant behavior emerges. |
| hundreds | The intended range. |

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

## 8. The 2D view and wraparound

The 2D view is a projection of the same sphere, so the only real choice is which
distortion to accept:

- **Equirectangular** (longitude to x, latitude to y). Trivial to implement, wraps
  naturally in x with period `2*pi`, and badly stretches the poles — a polar region
  becomes a wide band.
- **Mercator.** Conformal, so region shapes stay recognizable, but area distortion grows
  without bound toward the poles, and the poles themselves sit at infinity.
- **Equal-area cylindrical** (Lambert). Areas are correct, shapes are squashed near the
  poles. The best match for a game in which region area is a resource.

The horizontal wraparound described in
[the planet view prototype](../prototypes/planet-view.md) falls straight out of any
cylindrical projection: the projection is periodic in longitude, so drawing the map at
x-offsets of `..., -2*pi, 0, +2*pi, ...` tiles seamlessly. The dimmed duplicates are the
copies at nonzero offsets. Vertical wraparound does not exist — the top and bottom edges
of the projection are the poles, which are points rather than edges.

## 9. Open questions

- Which cylindrical projection for the 2D view? Equal-area is the principled choice;
  equirectangular is easier and may be good enough.
- Are the twelve pentagons worth detecting and deliberately relocating (into oceans,
  say), or does jitter already hide them well enough?
- Should relaxation be area-weighted, so that deliberately non-uniform region density —
  dense continents, sparse oceans — becomes achievable?
- Is a fixed tessellation enough, or will regions need to subdivide during play?

## 10. References

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
