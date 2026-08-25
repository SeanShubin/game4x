# graph-coloring

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

Assigns colors to a graph so that no two adjacent vertices share one, using as few as
practical.

The theory is in [coloring regions](../../docs/theory/region-coloring.md). This crate is
the combinatorial half of that document; the perceptual half — which actual colors to
put on screen — belongs to whatever is doing the rendering.

This crate knows nothing about spheres, geometry, or the game. It takes neighbour lists
and returns colors.

## Public surface

```rust
let coloring = color_graph(&neighbours);   // neighbours: &[Vec<u32>]

coloring.colors;       // Vec<u8>, one color index per vertex
coloring.color_count;  // how many distinct colors were used
coloring.method;       // Trivial | Exact(n) | GreedyFallback

find_conflict(&neighbours, &coloring.colors);  // Option<(usize, usize)>, None when valid
```

## How it works

A region adjacency graph on a sphere is planar, so four colors always suffice — the
Four Color Theorem. But the algorithmic picture is lopsided: deciding 3-colorability is
NP-complete, 4-colorability is trivially *true* yet awkward to *find*, and 5-coloring is
easy. So rather than implement the quadratic algorithm from the proof, this climbs a
ladder and stops at the first success:

1. **No edges** — one color, immediately.
2. **k = 2, then 3, then 4** — exact backtracking search under a step budget.
3. **Greedy fallback** — smallest-last ordering, which cannot fail and cannot exceed six
   colors on a planar graph, because planar graphs are 5-degenerate.

The search picks vertices by DSATUR — always the vertex whose neighbours already use the
most distinct colors — so it fails fast where it is going to fail. A vertex may only ever
introduce the *next* unused color, which discards the factorial symmetry between
equivalent color permutations and is what makes the search finish instantly at these
sizes.

**The fallback is a bug detector.** On a genuinely planar input the k = 4 step always
succeeds, so `Method::GreedyFallback` means something upstream is wrong — most likely
corner contacts leaking into the adjacency graph, which breaks the planarity the
theorem depends on. The planet view prints it in capitals for that reason.

Everything is deterministic: ties break by vertex index, never by hash order.

## Tests

- Known chromatic numbers: a path needs 2, an odd cycle needs 3, an even cycle needs 2,
  `K4` needs 4.
- `a_sphere_tessellation_needs_at_most_four_colors` — real tessellations from 4 to 137
  regions, asserting no conflicts, at most four colors, **and** that the fallback never
  fired.
- `the_greedy_fallback_always_produces_a_valid_coloring` — the safety net is itself safe.
