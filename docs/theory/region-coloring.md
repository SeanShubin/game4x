# Theory: Coloring Regions

[Theory index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

How to assign colors to regions so that no two neighbors share one, how few colors that
takes, and how to pick colors a human can actually tell apart.

Its companion document is
[splitting a sphere into regions](region-splitting.md), which produces the adjacency
graph this document colors.

## 1. Two different problems share the word "color"

They are usually conflated, and they have nothing to do with each other:

1. **The combinatorial problem.** Given the adjacency graph, assign each region a label
   from a small set so that no edge connects two regions with the same label. What is
   the smallest such set, and how do we find one? This is graph coloring, and it is
   sections 2 through 8.
2. **The perceptual problem.** Given that we need, say, four labels, what four actual
   colors do we show on screen so they are distinguishable — including by a player with
   color vision deficiency, on a cheap monitor, at a glance? This is palette design, and
   it is section 9.

Solving the first perfectly and the second badly produces a map that is provably correct
and unreadable.

Both are needed for the **simplified view** in
[the planet view prototype](../prototypes/planet-view.md), the mode that discards
geography and shows only what the game mechanics see: which region touches which.

## 2. The adjacency graph is planar

The regions tile a sphere, and a sphere is topologically equivalent to a plane — punch a
hole in the sphere in the middle of any region and flatten it out, which is stereographic
projection. So the region adjacency graph can be drawn in the plane with no crossing
edges. It is a **planar graph**.

That single fact is what makes everything below tractable. Planar graphs are far better
behaved than general graphs, and the entire theory of map coloring is really the theory
of coloring planar graphs.

## 3. Four colors always suffice

**The Four Color Theorem.** Every planar graph can be colored with four colors such that
no two adjacent vertices share a color.

Conjectured by Francis Guthrie in 1852, it resisted proof for over a century, and was
finally proved by Appel and Haken in 1976 using a computer to check 1,936 unavoidable
configurations — the first major theorem whose proof could not be checked by hand.
Robertson, Sanders, Seymour and Thomas gave a cleaner proof in 1997 with 633
configurations, and Gonthier produced a fully machine-verified proof in Coq in 2005.

The practical consequence for this project: **we never need more than four colors, no
matter how many regions there are or how they are arranged.** Hundreds of regions, one
region, any tessellation — four is enough. That is a guarantee, not a heuristic.

## 4. The corner-touching caveat

The Four Color Theorem has a precondition that matters directly here. It applies when:

- each region is **connected** (one contiguous piece), and
- adjacency means sharing a **boundary of positive length**, not a single point.

Both hypotheses are easy to violate, and violating them breaks the guarantee outright.
Regions meeting only at a point are not adjacent in the theorem's sense. If you *declare*
corner-touching regions to be adjacent, the resulting graph is no longer planar in the
required way, and the four-color bound evaporates — think of a pie cut into N slices, all
of which meet at the center. Under corner adjacency that is a complete graph, needing N
colors.

Our tessellation avoids the problem by construction. Section 5 of
[region splitting](region-splitting.md#step-5-take-the-dual-to-get-the-voronoi-cells)
defines adjacency as one shared Voronoi edge, which is exactly one Delaunay edge, which
has positive length. Corner contacts are not adjacencies. So:

> **Adjacency is a shared edge, never a shared corner.** This is a game rule, a rendering
> rule, and a precondition of the coloring guarantee, all at once.

The remaining risk is numerical: a Voronoi edge of near-zero length is geometrically
almost a corner touch. It is still a genuine edge and must be treated as one
consistently, or the graph the colorer sees will differ from the graph the game
simulates. The minimum-edge-length metric in
[region splitting](region-splitting.md#7-quality-metrics) exists to keep those rare.

The second hypothesis matters too: a disconnected region — an empire's two separated
provinces treated as one region — breaks the theorem as well. This is the historical
reason real political maps sometimes need five colors. Our regions are always connected.

## 5. When are fewer than four enough?

Four is the ceiling. The actual minimum for a given graph is its **chromatic number**,
and the low end has a clean structure:

**One color** iff there are no edges at all — only the N = 1 world.

**Two colors** iff the graph is **bipartite**, iff it contains no odd-length cycle. This
is decidable in linear time by two-coloring during a breadth-first search and checking
for a contradiction. Our graphs are triangle-rich, so this essentially never applies
beyond trivial cases.

**Three colors** is where it gets interesting. A perfect hexagonal tiling of the *plane*
is 3-colorable — the honeycomb has a clean three-coloring, which is why hex-based board
games so often use three. Two relevant theorems:

- **Grotzsch's theorem**: every triangle-free planar graph is 3-colorable. Our adjacency
  graph is the Delaunay graph, which is full of triangles, so this does not apply.
- **A planar graph is 3-colorable if every face has even degree** (a consequence of the
  same duality that makes plane hex tilings 3-colorable).

On a sphere, the twelve mandatory pentagons of
[section 2 of region splitting](region-splitting.md#2-why-a-perfect-hex-grid-on-a-sphere-is-impossible)
introduce odd faces, and odd faces introduce odd cycles. So a spherical hex-dominant map
is generally **not** 3-colorable, and the twelve topological defects are precisely why.
That is a satisfying connection between the two documents: the same twelve pentagons that
make a perfect hex grid impossible are what push the color count from three to four.

**Four colors** is therefore the expected answer for any realistic planet, and is
guaranteed to be attainable.

## 6. The complexity trap

Given how tidy the four-color guarantee is, the algorithmic picture is surprisingly
uneven:

| Question                          | Difficulty                                                                                                                                |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Is this planar graph 2-colorable? | Linear time. Just check bipartiteness.                                                                                                    |
| Is this planar graph 3-colorable? | **NP-complete.** Even for planar graphs of maximum degree 4.                                                                              |
| Is this planar graph 4-colorable? | Trivially yes — the theorem answers it without looking at the graph.                                                                      |
| *Find* a 4-coloring               | Quadratic time is possible, but the algorithm is a direct implementation of the discharging proof and is notoriously unpleasant to write. |
| *Find* a 5-coloring               | Linear time, and the algorithm is short and easy to get right.                                                                            |

So the awkward part is not proving four suffice, it is *producing* a four-coloring
cheaply. The Robertson-Sanders-Seymour-Thomas quadratic algorithm exists, but nobody
implements it for a few hundred vertices when heuristics find a four-coloring in
milliseconds.

The Five Color Theorem is worth knowing as the safety net. Its proof is constructive and
elementary: every planar graph has a vertex of degree at most 5 (from Euler's formula),
so remove it, color the rest recursively, and put it back — using a Kempe chain argument
to free up a color when all five neighbors differ. It gives a guaranteed, easy,
never-fails fallback.

## 7. Practical algorithms

**Greedy coloring** is the base: order the vertices somehow, then give each the
lowest-numbered color not used by an already-colored neighbor. Quality depends entirely
on the ordering.

- **Welsh-Powell**: order by descending degree. Simple, and a clear improvement over
  arbitrary order.
- **Smallest-last (degeneracy) ordering**: repeatedly remove a minimum-degree vertex,
  then color in reverse removal order. Because every planar graph is **5-degenerate** —
  every subgraph has a vertex of degree at most 5 — this guarantees **at most 6 colors**
  on any planar graph, in linear time, always. A useful hard bound to have in your back
  pocket.
- **DSATUR** (Brelaz, 1979): repeatedly color the uncolored vertex with the highest
  *saturation degree*, the number of distinct colors already among its neighbors, with
  ties broken by degree. It is dramatically better than degree ordering in practice, it
  is exact on bipartite graphs, and on planar graphs it usually lands on four
  immediately.

**Exact search** for small k: DSATUR with backtracking, or an encoding to SAT with one
boolean per (region, color) pair. For a few hundred vertices this is fast, though 3-
colorability is NP-complete so a time budget is still required.

### The recommended ladder

Try increasing color counts and stop at the first success:

1. **k = 2** — bipartiteness check, linear time. Almost always fails; costs nothing.
2. **k = 3** — DSATUR with backtracking under a node or time budget. Succeeds only on
   unusual graphs, but a 3-colored map is noticeably cleaner when it happens.
3. **k = 4** — DSATUR with backtracking. Guaranteed to exist by the Four Color Theorem;
   found quickly in practice.
4. **Fallback** — smallest-last greedy, at most 6 colors, cannot fail.

The fallback exists because a search budget can expire, and the renderer must never be
handed an uncolored map. Log when it fires: with correct adjacency the k = 4 step should
essentially always succeed, so a fallback is evidence of a bug — most likely corner
touches leaking into the adjacency graph, per section 4.

## 8. Stability and determinism

Two properties matter beyond correctness:

**Determinism.** The same adjacency graph must produce the same coloring on every
machine and every run. That means no iteration over hash maps with randomized ordering,
no unseeded randomness in tie-breaking. Region identifiers are integers, so break every
tie by identifier.

**Stability.** If a single region changes — a border shifts, a region is added — a
recoloring that reshuffles the whole map is jarring, because the player is using color as
an identity cue. Prefer an incremental repair: keep the existing coloring, recolor only
vertices that now conflict, and expand outward only as far as conflicts propagate. This
gives up minimality in exchange for visual continuity, which is the right trade for a
debug view. If the tessellation is fixed at world generation, this never comes up.

Properties worth testing directly:

- No edge in the graph connects two regions of the same color. (The definition. Assert
  it on every generated world.)
- The color count is at most 4 for any planar input, and at most 6 for any input at all.
- The same seed produces the same coloring, byte for byte.
- Every region has a color.

## 9. Choosing the actual colors

The graph algorithm returns integers 0 through 3. Turning those into pixels is a separate
design problem with its own theory.

### Perceptual, not numerical, distance

RGB distance does not correspond to perceived difference — `#00FF00` and `#00CC00` look
nearly identical while `#0000FF` and `#0000CC` are further apart than their numbers
suggest. Choose colors in a perceptually uniform space instead, where equal numeric steps
look like equal perceptual steps: **CIELAB**, **CIELCh**, or the more recent **OKLab** and
**OKLCh**, which fix known weaknesses of CIELAB in the blue range. OKLCh is the practical
pick: lightness, chroma, and hue as independent, well-behaved axes.

The rule of thumb is that colors are distinguishable at a glance when they differ
substantially along **more than one** axis. Four hues at identical lightness and chroma
are technically distinct and still easy to confuse in peripheral vision.

### Color vision deficiency

Roughly 8% of men and 0.5% of women of northern European descent have some form of color
vision deficiency, overwhelmingly red-green (deuteranomaly and protanomaly). A palette
that relies on distinguishing red from green fails for a substantial slice of players.

Practical guidance:

- **Vary lightness, not just hue.** Lightness differences survive every form of CVD. If
  four colors have four clearly different lightness values, the palette works in
  grayscale, which means it works for everyone.
- **Prefer the blue-yellow axis** for hue contrast, and be careful along red-green.
- **Verify with a simulator.** Run the palette through deuteranope, protanope, and
  tritanope simulations, and check that all pairs remain separable.
- **Never encode meaning in color alone.** For a debug view whose entire content is the
  colors, that means adding region identifier labels and drawn borders, so the view still
  works when the colors fail.

### Borders carry more than color

Two adjacent regions of different colors read as separate even at low contrast if there is
a border stroke between them. A visible outline — dark or light depending on the palette's
lightness range — reduces how much work the fill colors have to do, and makes near-zero-
length edges (section 4) visible for what they are.

### Two palettes, two purposes

Keep them apart:

- **The mechanics palette** — the 4 to 6 colors from the graph coloring, for the
  simplified view. It should be flat, high-contrast, obviously synthetic, and make no
  attempt to look like terrain. Its entire job is to make adjacency legible.
- **The terrain palette** — greens, blues, browns, driven by the geography that
  [region splitting](region-splitting.md#step-6-render-time-detail) generates. It carries
  meaning about the world, not about the graph.

Using terrain-like colors for the mechanics view invites players to read meaning that is
not there. The mechanics view's colors are arbitrary labels, and they should look like it.

### A workable default

Four colors at distinctly different lightness values, spread around the hue circle and
avoiding a pure red-green opposition — for example a dark blue, a mid orange, a light
yellow-green, and a mid-dark purple. Checked in grayscale first, then under CVD
simulation, then on screen. If the grayscale version is readable, the color version
almost certainly is.

## 10. Open questions

- Does the simplified view color the *regions*, or the *territories* owned by each player?
  Those are different graphs with different requirements — territories are often
  disconnected, which breaks the four-color guarantee per section 4.
- Is minimality actually wanted? A guaranteed-fast 5- or 6-coloring may look better
  (more variety, easier to distinguish neighbors two hops apart) than a minimal
  4-coloring.
- Should the coloring be stable across saves, or recomputed on load? Stable requires
  storing it; recomputed requires the algorithm to be deterministic, which section 8
  demands anyway.

## 11. References

**Graph coloring theory**

- Appel, K. and Haken, W. (1977). "Every Planar Map is Four Colorable." The original
  computer-assisted proof.
- Robertson, Sanders, Seymour and Thomas (1997). "The Four-Colour Theorem." The simplified
  proof, with a quadratic-time 4-coloring algorithm.
- Gonthier, G. (2008). "Formal Proof — The Four-Color Theorem." The Coq-verified proof.
- Grotzsch, H. (1959). Every triangle-free planar graph is 3-colorable.
- Garey, Johnson and Stockmeyer (1976). "Some simplified NP-complete graph problems."
  Planar 3-colorability is NP-complete.
- Heawood, P. J. (1890). "Map colour theorem." The Five Color Theorem, and the flaw in
  Kempe's 1879 attempted proof of four.

**Algorithms**

- Brelaz, D. (1979). "New methods to color the vertices of a graph." DSATUR.
- Welsh, D. J. A. and Powell, M. B. (1967). Degree-ordered greedy coloring.
- Matula and Beck (1983). Smallest-last ordering and graph degeneracy.

**Color perception**

- Ottosson, B. (2020). "A perceptual color space for image processing." OKLab and OKLCh.
- CIE L*a*b* (CIELAB) and CIE L*C*h. The established perceptually uniform spaces.
- Brettel, Vienot and Mollon (1997). "Computerized simulation of color appearance for
  dichromats." The basis of most CVD simulators.
- Okabe, M. and Ito, K. "Color Universal Design." A widely used CVD-safe qualitative
  palette.
- Harrower, M. and Brewer, C. (2003). "ColorBrewer.org." Qualitative palettes designed for
  exactly this problem, on maps.
