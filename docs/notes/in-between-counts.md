# Clumping at in-between counts

**Derived.** Written by Claude from conversation on **2026-08-26**. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Sean asked whether region counts between the Goldberg numbers can still get twelve
well-spaced pentagons, and then whether the construction could avoid clumping in the first
place rather than repairing it afterward. This records what was measured, including two
approaches that looked promising and were not.

## The question, stated precisely

A sphere tiling needs a total angular defect of exactly `+720°`. A pentagon supplies
`+60°`, a hexagon nothing, and a heptagon `-60°`. Twelve pentagons therefore satisfy
curvature exactly, and a heptagon can only ever appear alongside a thirteenth pentagon
paying it back. **Heptagons are never required by curvature.**

Nor are they required for spacing. In fullerene terms, a tiling where no two pentagons
touch satisfies the Isolated Pentagon Rule, and IPR fullerenes are known to exist at 60
vertices and at every even vertex count from 70 up. Converting to faces via `F = V/2 + 2`:

> Twelve isolated pentagons, all hexagons, no heptagons - available at `F = 32` and at
> every count from 37 upward.

The smallest is `C60`, 32 faces, which is our truncated icosahedron.

There is one genuine exception. Face counts **33, 34, 35 and 36** have no IPR fullerene -
the corresponding vertex counts 62 to 68 fall in the gap between `C60` and `C70`. At those
four sizes two pentagons must touch, however the tiling is built.

## What we actually produce, and why

At a Goldberg count the generator constructs the answer and relaxes nothing.
`canonical_seeds` covers every Goldberg count, not only 32, so 42, 92, 162, 252 and 492
each come back with twelve pentagons, none touching, in zero relaxation passes.

In the gaps there is no construction, so generation falls back to a golden spiral plus
Lloyd relaxation, and the result carries extra 5-7 pairs:

| regions | pentagons | heptagons | touching pairs |
| ------- | --------- | --------- | -------------- |
| 100     | 15        | 3         | 0              |
| 150     | 14        | 2         | 0              |
| 200     | 36        | 24        | 2              |
| 300     | 46        | 34        | 0              |

Note the last column. **The pentagons do not clump** - they are spread out at almost every
count. There are simply more of them than the twelve that curvature demands, each paired
off with a heptagon.

That structure is what defect interaction predicts: like-signed disclinations repel, so
pentagons spread; opposite-signed ones attract, so a spare pentagon pairs with a heptagon.
The spreading is not *caused* by the heptagons. The heptagons are leftovers that the
spreading then tidies up.

## Three things that do not fix it

Each was measured, not reasoned about.

**Seeding from the nearest Goldberg solid and inserting the shortfall.** Worse, decisively.
At 100 regions it gave 26 pentagons, 14 heptagons and 2 touching pairs, against the spiral
start's 15, 3 and 0. Inserting points into a *perfect* lattice tears it locally, and each
insertion spawns a defect cluster that relaxation spreads but cannot remove.

**Shake-and-resettle (annealing), and best-of-K restarts.** Both looked like clear wins in
an isolated harness - roughly halving the defect count - and both were **confounded by
compute**. The annealed path was given 34 relaxation passes against plain relaxation's 24.
Matched fairly inside the real generator, annealing lost: 100 regions went from 3 heptagons
and 0 touching pairs to 6 and 2. The change was written, measured, and reverted.

**Relaxing harder.** Relaxation is a geometric optimizer and defect count is a topological
property. There is no gradient pointing toward "one fewer heptagon", because the thing to
be changed is not a position. Every parameter on this machine is the wrong parameter.

## What would work

Invert the order: build the graph first, embed it second.

That is exactly what the Goldberg path already does, and what
[`topology.rs`](../../crates/sphere-tessellation/src/topology.rs) already does for class I,
where adjacency is derived from `(m, n)` by Eisenstein arithmetic and never measured. The
arbitrary-`N` equivalent is to generate a fullerene graph with twelve pentagons
combinatorially, then relax positions with the **topology held fixed** - geometry serving a
topology already decided, rather than topology emerging from wherever geometry happens to
land.

A cheaper partial route, for repairing an existing tiling rather than constructing a new
one: a single edge flip does not annihilate a 5-7 pair, it **moves** it. Flipping decrements
two vertex degrees and increments two others, so fixing one pair typically converts two
neighbouring hexagons into a fresh one nearby. Dislocations only vanish when two meet with
opposing orientation, which makes repair a migration loop rather than a pass. Our extra
constraint is that adjacency here *is* the Delaunay dual of the seeds, so after each flip
the seeds must be nudged until the new triangulation is Delaunay again, or the topology
desynchronises from what the renderer draws.

Neither is built.

## The practical reading

Goldberg counts below 500 are 12, 32, 42, 72, 92, 122, 132, 162, 192, 212, 252, 272, 282,
312, 362, 372, 392, 432, 482 and 492. The largest gap is 50 and the mean is about 25, so
world sizes chosen from that list are never far from any number you might have wanted, and
every one of them is perfect by construction.

Nothing here is a decision. Whether the game offers only Goldberg counts is Sean's to say.
