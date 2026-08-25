# Theory: Comparing Region Schemes

**Derived.** Written by Claude from conversation, 2026-08-24. Not binding - the
decisions it reaches are ratified in [spec/planet.md](../../spec/planet.md).

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

A survey of every known way to divide a sphere into hex-like regions, measured against each
other, for a game that needs an *abstraction* rather than a simulation.

Its companion document is [splitting a sphere into regions](../theory/region-splitting.md), which
describes the chosen pipeline in depth. This document is the wider survey that pipeline was
picked out of, plus four schemes that document does not cover. The decision it reaches
is recorded in [section 10](#10-where-the-analysis-landed).

## 1. What the abstraction has to do

Two questions, and nothing else.

**Are these two regions adjacent?** and **how far apart are they?** Everything
downstream - movement, borders, supply range, weapon coverage - is built from those.
There is no terrain simulation to serve.

| Requirement                          | Why                                                                                                                  |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| Adjacency is unambiguous             | Every rule about borders and movement rests on it. Corner-touching must never be a question                          |
| Distance behaves the same everywhere | A weapon's range is a number on a card. It has to mean the same thing wherever you build                             |
| Five or six neighbours per region    | A hex grid has no diagonal ambiguity and uniform movement cost. Section 3 shows this forces exactly twelve pentagons |
| Region count comes in usable sizes   | Planet size is a game lever, and it needs enough rungs                                                               |
| Cheap, deterministic generation      | Many planets per game, generated from a seed                                                                         |

A flat hex grid satisfies all of these, which is why it is the reference: adjacency is
six full edges, and distance is closed form in cube coordinates,
`d = (|dq| + |dr| + |ds|) / 2`.

**The planet is rendered as a 3D sphere and never projected to a plane.** That decision
is recorded in [section 10](#10-where-the-analysis-landed), and it retires a requirement this document
originally carried - that the tessellation also be drawable flat without distortion.
Sections 6 and 8 were written to serve that requirement. They are kept, marked as
rejected, because the reasoning is what makes the decision legible.

## 2. The one law

Take any tiling of the sphere in which three regions meet at each corner. Define a region's
**defect** as `6 - (its neighbor count)`. Then:

```
sum over regions of (6 - degree) = 12
```

Twelve units, always, at every size, for every scheme. A pentagon is one unit, a square two,
a triangle three. [Region splitting section 2](../theory/region-splitting.md#2-why-a-perfect-hex-grid-on-a-sphere-is-impossible)
derives this; it is Gauss-Bonnet made discrete, with each unit worth `pi/3` of the sphere's
`4*pi`.

Two corollaries do most of the work below, and neither appears in that derivation.

### 2.1 Corners cost extra

The count of twelve assumes every corner is trivalent. When corners are shared by more
regions the budget inflates, and you pay for defect you did not ask for:

```
sum of (6 - degree) = 12 + 2 * sum over corners of (degree - 3)
```

One four-way corner costs two extra units. This is the trap that breaks several otherwise
tidy constructions, and it is not merely an accounting problem: a four-way corner is exactly
the case where "adjacent" stops being well defined, which requirement one forbids outright.
See [the corner-touching caveat](../theory/region-coloring.md#4-the-corner-touching-caveat).

### 2.2 In a row-based scheme, defect is a second difference

Build the sphere as latitude rows of `n[0], n[1], ... n[R]` cells, each row wrapping, each
cell owning a longitude arc. Two adjacent rows of size `a` and `b` share exactly `a + b`
borders, so a cell's average degree in row `r` is `4 + (n[r-1] + n[r+1]) / n[r]`, and:

```
defect of row r = 2*n[r] - n[r-1] - n[r+1] = -(second difference of n)
```

Zero exactly when the row counts are **linear in the row index**. The row-count profile *is*
the curvature distribution function. Since `n` must rise from a pole and fall back, it has to
bend somewhere, and the bend is where the twelve go.

## 3. Every scheme is a partition of twelve

That reframes the whole design space. The twelve units cannot be avoided and cannot be
created without paying the corner penalty. The only decision is how to break them into lumps
and where to put the lumps.

| Partition | Shape it takes                         | Realized by                                |
| --------- | -------------------------------------- | ------------------------------------------ |
| 12 x 1    | twelve pentagons                       | icosahedral (Goldberg), D6 barrel, Voronoi |
| 6 x 2     | six squares                            | octahedral subdivision                     |
| 4 x 3     | four triangles, or four folded corners | tetrahedral subdivision, pillow rectangle  |
| 3 x 4     | three two-sided cells                  | the `333` flat orbifold                    |
| 5 + 4 + 3 | one each of degree 1, 2, 3             | the `632` flat orbifold                    |

Fewer, fatter lumps mean sharper local distortion. They also mean more of the world is
exactly flat, which is [section 8](#8-putting-it-on-a-screen-superseded) and the reason the
ordering reverses.

Naive latitude-longitude — every row the same length, poles as single cells — is not on this
table. Its poles are corners shared by `W` regions, so 2.1 applies and it grows cells of
unbounded degree as the grid refines. It is the one option that is simply wrong rather than a
trade-off. Section 9 measures how wrong.

## 4. Latitude row profiles

Rows of varying length, offset so the grid is hex-like. Choosing the profile is the entire
design, because by 2.2 the cells sit at degree six wherever the profile is linear and the
curvature lands wherever it bends.

### 4.1 Step by one: 1, 2, 3, ... k, ... 3, 2, 1

The obvious profile, and it fails on the corner penalty. The rows of two beside each pole
create four-way corners, so the measured budget comes to **16, not 12**, and the pole cells
have two neighbors each. Rejected: it violates requirement one before any question of
quality arises.

### 4.2 Step by six: 1, 6, 12, 18, ... 6k ... 18, 12, 6, 1

Cap each pole with a single hexagon and grow by six. Every corner is now trivalent and the
budget is exactly twelve, at every size:

| k   | cells | defect | degrees      |
| --- | ----- | ------ | ------------ |
| 2   | 26    | 12     | `5:12 6:14`  |
| 3   | 56    | 12     | `5:12 6:44`  |
| 4   | 98    | 12     | `5:12 6:86`  |
| 6   | 218   | 12     | `5:12 6:206` |

Twelve pentagons, everything else a hexagon, at `6k^2 + 2` cells — 26, 56, 98, 152, 218, 296,
386, 488. All twelve land in the single row where the profile turns.

Why it comes out this clean: rings of 1, 6, 12, 18 around a hexagon **are** a flat hex grid.
This construction is two flat hexagonal discs of radius `k`, glued along their `6k`-cell rims
with the southern disc rotated so its six rim corners interleave with the northern six. Six
plus six is the twelve. That fact matters again in section 8.

Call this the **D6 barrel**. It is a genuine sibling of Goldberg — same theorem, hexagonal
barrel symmetry instead of icosahedral, `6k^2 + 2` instead of `10T + 2` — and it demonstrates
constructively that the twelve pentagons are not an icosahedral artifact.

### 4.3 Why the bend cannot be smoothed away

Spreading the turn over several rows to hide the belt immediately buys heptagons. Measured:

| Profile                        | degrees               |
| ------------------------------ | --------------------- |
| single turn, k = 4             | `5:12 6:86`           |
| turn spread over three rows    | `5:16 6:218 7:4`      |
| smooth `cos(latitude)` profile | `5:40 6:488 7:22 9:2` |

The budget stays twelve either way. You have simply started paying it in matched pairs of
positive and negative defect instead of in twelve clean pentagons.

## 5. Seed-solid subdivision

Put a triangular lattice on each face of a deltahedron, project onto the sphere, take the
dual. Every seed vertex of degree `d` becomes a cell with `d` neighbors — defect `6 - d` —
and every other cell is a hexagon. A deltahedron's degrees always satisfy `sum(6 - d) = 12`,
so the budget takes care of itself.

```
N = (faces of seed / 2) * T + 2        where T = a^2 + a*b + b^2
```

| Seed        | Faces | Defects                | N         | Counts available             |
| ----------- | ----- | ---------------------- | --------- | ---------------------------- |
| Tetrahedron | 4     | 4 triangles (degree 3) | `2T + 2`  | 4, 8, 14, 20, 26, 38, ...    |
| Octahedron  | 8     | 6 squares (degree 4)   | `4T + 2`  | 6, 18, 30, 38, 50, 74, ...   |
| Icosahedron | 20    | 12 pentagons           | `10T + 2` | 12, 32, 42, 72, 92, 122, ... |

The icosahedral case is the **Goldberg polyhedron**, covered in depth in
[region splitting section 4.1](../theory/region-splitting.md#41-geodesic-and-goldberg-polyhedra-rejected).
It is the optimum of the entire partition table: twelve units broken into the smallest pieces
allowed, placed at the twelve points that are as far from each other as twelve points on a
sphere can get. Nothing beats it on uniformity because nothing is permitted to.

The other two seeds are worth recording precisely because they are worse in a predictable,
measurable way — they are the `4 x 3` and `6 x 2` rows made concrete — and because they fill
gaps in the count ladder. The cost across all three is that `N` is quantized; the largest gap
between Goldberg counts below 500 is fifty.

## 6. Flat folds: the rectangle that is a sphere (rejected)

**Rejected** by [section 10](#10-where-the-analysis-landed): its entire advantage is being drawable
flat, and the planet is not drawn flat. It also fails the five-or-six rule outright.
Kept because it is the sharpest illustration of what concentrating the twelve costs, and
because the classification in 6.3 is the reason no flat option could have worked.

A `W x H` array, wrapping left-right, with each open end zipped shut. It is the only
scheme here that is genuinely flat.

### 6.1 The construction

Hex-offset rows on a plain rectangular array, columns wrapping by translation — that is a
cylinder. Then zip each open end shut the way you seal the end of a toothpaste tube:

```
cell (0,   c)  is also adjacent to  (0,   -c mod W)  and  (0,   -c-1 mod W)
cell (H-1, c)  is also adjacent to  (H-1, -c mod W)  and  (H-1, -c-1 mod W)

columns   0   1   2   3   4   5   6   7        (W = 8)
the zip   ^   |___|___|___|   ^   |   |        1<->7   2<->6   3<->5
          |_______|___|_______|___|___|        cols 0 and 4 fold onto themselves
```

Six modular additions per cell. No geometry, no floating point, no hull, no relaxation.
Verified against an independent edge-level construction that glues the `2W` boundary edges
pairwise; the two agree exactly and the adjacency is symmetric.

| W   | H   | N   | defect | degrees         |
| --- | --- | --- | ------ | --------------- |
| 8   | 5   | 40  | 12     | `4:4 5:4 6:32`  |
| 9   | 4   | 36  | 12     | `4:4 5:4 6:28`  |
| 11  | 6   | 66  | 12     | `4:4 5:4 6:58`  |
| 37  | 3   | 111 | 12     | `4:4 5:4 6:103` |
| 20  | 10  | 200 | 12     | `4:4 5:4 6:192` |

Twelve every time. That constant is the proof of topology: a torus reads zero, `RP^2` reads
six, a sphere reads twelve. The census is always four cells with four neighbors, four with
five, the rest hexagons, independent of `W` and `H`.

Width need not be even, and `H = 1` works, so **`N = W x H` reaches essentially any count**.
What quantizes is the aspect ratio, not `N`: a round pillow wants `W = 2H`, so a prime `N`
forces a sausage.

### 6.2 Why the distance is exact

Zipping both ends of a cylinder gives a pillowcase — a sphere with four cone points of angle
`pi`. Four points at three units each is the `4 x 3` row of the partition table. But a cone
angle of `pi` is `2*pi/2`, an **orbifold** point of order two, and the pillowcase orbifold has
Euler characteristic `2 - 4*(1/2) = 0`. It is **flat**, and its universal cover is the
Euclidean plane.

Concretely, the pillow sphere is a hex-grid *torus* of `W x 2H` cells folded by a 180-degree
rotation. So distance lifts:

1. Lift both cells to the covering torus.
2. Take the minimum over the two-element orbit of the target.
3. Each torus distance is itself a minimum over nine translates of the flat cube-coordinate
   hex distance.

Constant time, exact. Checked against brute-force BFS on the quotient at `6x3`, `8x4`,
`10x5`, `7x4` — identical at every pair, with the rotation fixed-point-free on cells.

### 6.3 There are exactly three flat hex spheres

A flat hex grid closes into a sphere only when every cone angle is `2*pi/n`. A seed vertex of
degree `d` gives a cone angle of `60d` degrees, so `n = 6/d`, and only `d = 1, 2, 3` work. The
Euclidean orbifolds with sphere topology are `2222`, `333`, `442` and `632`; `442` needs
four-fold symmetry a hex lattice does not have.

So there are exactly three, and `2222` splits the twelve most evenly of the three. **The
pillow is the best flat option that exists.**

The tetrahedron is the same orbifold, differing only in where the cone points sit: on cell
centers (four triangular cells) instead of on folded edges (four plus four). Octahedra and
icosahedra have cone angles of 240 and 300 degrees, neither of the form `2*pi/n` — which is
exactly why Goldberg has no closed-form distance and no flat drawing.

### 6.4 What it costs

Each of the four corner cells has one hexagon side **folded onto itself**. That is a region
bordering itself: real, not a bug, but something the rules have to survive — a border with one
owner instead of two, and a self-loop in the adjacency graph that
[region coloring](../theory/region-coloring.md) is not currently written to expect.

## 7. Point sets, and the quad branch

**Spherical Voronoi over a Fibonacci lattice** is the chosen pipeline and is covered in
[region splitting sections 4.4 and 5](../theory/region-splitting.md#44-spherical-voronoi-tessellation-of-a-point-set-chosen).
Its place in this survey: it is the only scheme with *no quantization whatsoever*, it lands in
the `12 x 1` partition with the twelve scattered rather than placed, and it has neither a
distance formula nor a flat drawing — only search and projection.

**Cube sphere** and **HEALPix** give up hexagons entirely and are covered in
[sections 4.2 and 4.3](../theory/region-splitting.md#42-cube-sphere-and-quad-sphere) there. One thing
worth adding: HEALPix is the row-profile idea of section 4 taken to its logical end. Its polar
caps grow linearly — 4, 8, 12, and so on — which is exactly the condition from 2.2, with
latitudes chosen so the bands come out equal-area. Push a row-profile scheme toward equal area
and you converge on it.

## 8. Putting it on a screen (superseded)

**This section's premise is dead.** It assumes the world is drawn on a flat screen, and
[section 10](#10-where-the-analysis-landed) chose a 3D sphere instead, where there is no projection and
therefore no projection distortion. The analysis below is still correct; it simply no
longer decides anything. It is kept because it explains why the flat schemes existed at
all, and because the reversal it describes is the single most useful thing in this
document.

Here the ordering reverses, and it reverses for the same reason it existed.

Any polyhedron can be cut open and laid flat; a net always exists and is undistorted inside
each face. That is not the interesting property. The interesting property is whether **the
plane covers the world** — whether you can pan in any direction forever, never hit a cut, and
have every cell sit at the center of a locally perfect hex neighborhood.

That is exactly the flat-orbifold condition of 6.3. A flat orbifold is precisely a world whose
universal cover is the Euclidean plane, so it can be drawn on a flat screen with zero
distortion anywhere. Every other scheme must be either cut, showing seams, or projected,
distorting everywhere.

| Scheme      | Flat layout                                          | Distortion in the layout       | Seams          | Pan forever |
| ----------- | ---------------------------------------------------- | ------------------------------ | -------------- | ----------- |
| Pillow      | one `W x H` rectangle — the storage *is* the picture | none                           | 2 mirror edges | yes         |
| Tetrahedral | one rhombus or triangle                              | none                           | 3              | yes         |
| D6 barrel   | two hexagonal discs side by side                     | none within a hemisphere       | 1 equator      | no          |
| Octahedral  | one rhombic net of 8 triangles                       | none within a face             | 6              | no          |
| Goldberg    | icosahedral net, 10 rhombi, jagged                   | none within a face             | about 20       | no          |
| Voronoi     | no net exists                                        | projection distorts everywhere | —              | no          |

So the `4 x 3` partition, which measures *worst* on metric fidelity, is the only one that is
*perfect* on display: the world is literally a 2D array of hexes, drawn as itself, with the
entire error budget compressed into four points a player can be told about in one sentence.
Goldberg, which measures best on fidelity, has no flat drawing at all — its curvature is
smeared thinly everywhere, so every part of every flat view of it is slightly wrong.

The barrel reads better on screen than its numbers suggest. Two undistorted hexagonal discs
side by side is a layout a person parses instantly, the same convention as a two-hemisphere
world map, with one seam to label and twelve honest pentagons along it.

The pillow has one display wrinkle. The plane tiles by copies of the rectangle that alternate
180-degree rotations, so panning never hits an edge, but crossing a pole seam flips the world.
At region-count scale you would not pan across it: the whole world is one screen of hexes, and
the seam is a labeled edge on a static board rather than a scrolling artifact.

## 9. Measured

Every scheme built as a graph and measured identically, at comparable `N`.

The metric that matters for an abstraction is how hex-like the *distance* is. In a flat hex
grid the ring at distance `k` from any cell holds exactly `6k` cells. **Ring error** is the
deviation from that, averaged over every cell as origin. A flat hex torus is included as a
control to calibrate the scale.

| Scheme                          | N   | Defect | Diameter | Ring error, mean | Ring error, worst | Degrees           |
| ------------------------------- | --- | ------ | -------- | ---------------- | ----------------- | ----------------- |
| *control: flat hex torus 22x22* | 484 | 0      | 16       | 0.0%             | 0.0%              | `6:484`           |
| Goldberg GP(7,0)                | 492 | 12     | 21       | 7.2%             | **27.1%**         | `5:12 6:480`      |
| D6 barrel, k = 9                | 488 | 12     | 18       | 6.5%             | 31.0%             | `5:12 6:476`      |
| Octahedral, m = 11              | 486 | 12     | 22       | 6.7%             | 33.3%             | `4:6 6:480`       |
| Pillow 32x16                    | 512 | 12     | 23       | 7.5%             | 48.1%             | `4:4 5:4 6:504`   |
| Tetrahedral, m = 16             | 514 | 12     | 21       | 6.4%             | 50.0%             | `3:4 6:510`       |
| Naive lat/long 23x21            | 485 | 12     | 22       | 11.1%            | 283.3%            | `5:46 6:437 23:2` |

Two things fall straight out.

**The mean is nearly the same everywhere** — six to seven and a half percent, for every scheme
that gets its corners right. That is the sphere's curvature, and it is not negotiable.

**The worst case tracks the partition exactly.** `12 x 1` gives 27%. `12 x 1` clustered in a
belt gives 31%. `6 x 2` gives 33%. `4 x 3` gives 48 to 50%. That ordering is not a measurement
artifact; it is the partition table of section 3 read back out of the graph. Concentrating the
defect concentrates the distortion, in direct proportion.

Naive lat/long is off the scale for the reason 2.1 predicted: its two pole cells have
twenty-three neighbors each.

The same ordering holds at a smaller size — Goldberg GP(4,0) at 162 reads 16.7% worst, the
barrel at 152 reads 29.2%, octahedral at 146 reads 33.3%, pillow 18x9 at 162 reads 46.7%,
tetrahedral at 164 reads 50.0%.

### How these were measured

Each construction is built directly as an adjacency graph, with no geometry involved. Degree
censuses and Euler defects are exact integer counts. Ring error runs a breadth-first search
from every cell and compares `|{y : d(x,y) = k}|` against `6k` for `k` up to 40% of the
eccentricity. The flat torus control reading exactly `0.0%` is what validates the measure.
These figures are **not yet asserted anywhere in the code** — see open questions.

## 10. Where the analysis landed

**Goldberg polyhedra, rendered as a 3D sphere.** This is the conclusion the analysis
reaches, not a decision this document is entitled to make. It becomes real only when it is
written into [spec/planet.md](../../spec/planet.md). Four parts, taken together:

| Decision                                                              | What it settles                                                                                                                         |
| --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| The planet is rendered as a **3D sphere**, never projected to a plane | There is no projection, so there is no projection distortion, so flat-drawability stops being a requirement at all                      |
| The tessellation is a **Goldberg polyhedron** `GP(m,n)`               | Twelve pentagons at icosahedral vertices, hexagons elsewhere. Best measured uniformity of anything in this survey                       |
| Render geometry is **decoupled** from the tessellation                | Draw a smooth sphere; resolve regions per fragment. Polygon count becomes independent of region count                                   |
| Distance is a **precomputed all-pairs table**                         | One BFS per region at generation, one byte per pair. Every scheme answers "how far" in a lookup, so no scheme needed a distance formula |

### Why Goldberg wins once the sphere is 3D

Every criterion that favoured an alternative was a flat-screen criterion. Removing the
plane removes all of them at once, and what is left is one-sided:

|                                  | Goldberg            | Best alternative                              |
| -------------------------------- | ------------------- | --------------------------------------------- |
| Worst-case ring error at N ~ 490 | **27.1%**           | barrel 31.0%                                  |
| Isotropy on a rotating globe     | **no axis at all**  | barrel has a visible equatorial pentagon belt |
| Region counts below 500          | **20**              | barrel 8                                      |
| Five or six neighbours           | yes                 | barrel yes, pillow no                         |
| Already built and tested         | **yes, to 1e-15**   | none                                          |
| Constant eccentricity            | no, `3m` is the max | barrel `2k` exactly                           |

Only the last line favours anything else, and it is answered by quoting the *maximum*
eccentricity as the global-strike range, which is exact.

On a 3D globe the barrel's pentagon belt inverts from asset to liability: twelve
pentagons scattered are hard to find, twelve in a ring around the equator are not.

### Pentagons come in antipodal pairs

A property of the chosen family worth recording, because game rules can lean on it.
The twelve pentagons sit at the icosahedron's twelve vertices, which are six antipodal
pairs, so this holds for every `GP(m,n)` regardless of class. Verified for `GP(1,0)`
through `GP(5,0)`:

- Every cell has a geometric antipode (achiral classes I and II; chiral class III solids
  are not centrally symmetric, so this is expected to fail there).
- A pentagon's antipode is always a pentagon.
- **A pentagon's unique farthest region is its own antipodal twin, at exactly `3m`.**
  From a hexagon the farthest set is not unique - several regions tie.

### What was rejected, and what each was for

| Scheme                               | Was good at                                                                   | Rejected because                                                                                                                 |
| ------------------------------------ | ----------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Pillow rectangle (section 6)         | the only genuinely flat option; exact O(1) distance; any `N`                  | its whole value was being drawn flat, and it fails the five-or-six rule with four degree-4 regions                               |
| D6 barrel (section 4.2)              | flat per hemisphere; constant eccentricity `2k`                               | flatness is moot in 3D, and its equatorial pentagon belt is visible on a rotating globe                                          |
| Row profiles (section 4)             | fill **every** diameter with no gaps, constant eccentricity, twelve pentagons | same as the barrel: the layout advantage does not survive 3D rendering                                                           |
| Tetrahedral / octahedral (section 5) | coarser partitions of the twelve, extra count rungs                           | strictly worse uniformity, and they break the five-or-six rule                                                                   |
| Voronoi point set (section 7)        | exactly `N`, no quantization at all                                           | needs a hull plus relaxation per planet, has no coordinate structure, and its defect census must be verified rather than assumed |
| Cube sphere, HEALPix (section 8)     | trivial addressing; exactly equal area                                        | quads, so adjacency is ambiguous at every corner                                                                                 |

### What this costs

`N` is quantized to `10T + 2` where `T = m^2 + mn + n^2` - twenty counts below 500,
largest gap 50. Nine of those twenty are chiral class III (72, 132, 192, 212, 282, 312,
372, 392, 432) and are not built yet, which currently leaves eleven usable sizes.

## 11. Open questions

- Class III chirality is unimplemented, costing nine of the twenty available region
  counts. Worth building, or are eleven sizes enough?
- What cube-map resolution is needed for crisp borders at typical zoom, and where does
  the bake belong - `planet-render`, or a new crate between it and `sphere-tessellation`?
- Should the section 9 measurements become tests in `sphere-tessellation`, so ring error
  and eccentricity are asserted rather than recorded? They were produced by throwaway
  scripts.
- Six pentagon axes are now a known structural feature. Do the rules use them
  deliberately, hide them, or ignore them?
- [Region splitting](../theory/region-splitting.md) still presents the Voronoi pipeline as chosen.
  It needs reconciling with this decision.

## 12. References

Beyond those in [region splitting section 12](../theory/region-splitting.md#12-references):

- Conway, J. H., Burgiel, H., Goodman-Strauss, C. (2008). *The Symmetries of Things.* The
  orbifold notation used in section 6.3, and the classification of Euclidean 2-orbifolds.
- Thurston, W. P. *The Geometry and Topology of Three-Manifolds*, chapter 13. Orbifold Euler
  characteristic, and why characteristic zero means the universal cover is the plane.
- Peirce, C. S. (1879), and Guyou, E. (1887). The quincuncial and hemisphere-in-a-square
  projections — the conformal maps that realize the section 6 identification geometrically.
