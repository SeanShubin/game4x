# Making the Planet Look Like a World

**Derived.** Written by Claude, 2026-08-28. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

A survey, not a measurement. It reads the prior research in
`D:\keep\github\sean\seans-arcade` against what this repository has already built and
specified, and reports what carries over, what does not, and what is missing. Nothing here was
run or timed; every number quoted comes from the document it is attributed to.

## The question

Sean's words, this session: *render planets such that they look realistic with varied biomes,
yet still divide along the Goldberg polyhedron grid.*

Those are two demands that pull in opposite directions, and saying so plainly is most of the
work.

## Why the two demands pull apart

**Realistic terrain is continuous.** A coastline does not care where a boundary is. Rainfall
falls off gradually. A mountain range runs across whatever the map is divided into. Nothing in
geography is aware of a cell.

**The Goldberg grid is discrete, and it is the game.** A territory is claimed, held, worked and
counted as one thing. [`spec/planet.md`](../../spec/planet.md) fixes the tessellation exactly:
twelve pentagons, five or six neighbours each, adjacency by shared edge. The player has to be
able to see the cell they are about to claim.

Three obvious resolutions, and why each fails:

| Resolution                             | Why it fails                                                                                                                                                 |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| One flat colour per territory          | Reads as a board, not a world. This is what the prototype does today, and deliberately - `palette.rs` says outright that the colours are labels, not terrain |
| Continuous terrain, grid hidden        | The player cannot see what they are playing on                                                                                                               |
| Continuous terrain, grid drawn over it | The grid looks pasted on, because nothing in the picture makes the line fall where it falls                                                                  |

## The resolution: one field, two readers

Generate a **continuous field over the sphere** and let both the picture and the model read it,
rather than generating a picture and a model separately and hoping they agree.

- The **renderer** reads the field per pixel. It draws terrain that flows across boundaries,
  because the field does not know boundaries exist.
- The **model** reads the same field per territory - aggregated over the cell, or sampled at its
  seed - and gets one discrete answer per territory, which is what claiming and working a
  territory needs.

They cannot disagree, because one is derived from the other rather than authored beside it. The
grid then reads as a *survey* laid over a world, which is what it is, rather than as the world's
own structure.

### Half of the tension is already dissolved elsewhere

While this note was being written, another session filed [P-96](proposals.md) from Sean's own
words: **there are two drawings, practical and realistic, and the player toggles between them.**

That changes the shape of the problem, and for the better. The first row of the table above -
one flat colour per territory - is not a failed attempt at realism at all; it is the practical
drawing doing its job, and `docs/theory/region-coloring.md` already argues it should look
obviously synthetic precisely so that nobody reads terrain into it.

So the two demands never have to be met by one picture. Everything below is about the realistic
drawing. The field is still shared - both drawings sit on the same tessellation and, if terrain
turns out to be a fact of the model, on the same fields - but only one of them has to look like
a world.

## What the arcade research already answers

| Document                                                                                       | What it contributes here                                                                      |
| ---------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `docs/research/biome-variety-from-simple-parts.md`                                             | How many biomes each noise axis actually buys, and what to do instead of adding a fourth      |
| `docs/research/procedural-materials-and-stickers.md`                                           | Why a spatial function has no seams, and where its limit is                                   |
| `docs/research/algorithmic-rendering-theory.md`                                                | The primitive list: fBm, domain warping, Voronoi, smoothstep, biome weight maps               |
| `examples/biome_blender.rs`, `examples/noise_visualizer.rs`, `examples/normal_map_lighting.rs` | Working code for exactly the three-axis lookup, the noise variants, and lighting from normals |

### Biome count comes from axis count, not from an enumeration

The arcade note's central finding: with N axes and K bands each you get at most `K^N` biomes,
most combinations are nonsense, and the returns fall off hard. Its recommendation is **three
noise layers** - elevation, moisture, drainage - for fifteen to eighteen base biomes, then
**overlay systems** for perceived variety, because a new axis costs a whole new material and an
overlay costs a palette shift.

Drainage is the interesting one, and it is Dwarf Fortress's insight rather than an invention: it
only does work where moisture is already high. High rain plus low drainage is swamp; high rain
plus high drainage is forest. It does not subdivide deserts at all.

The deeper point in that note is worth restating, because it is the difference between a world
and a lookup table: **parameter interaction, not enumeration.** Do not define biomes and place
them. Define fields, and let biomes fall out of where the fields cross. Boundaries then form
along gradients on their own.

`biome_blender.rs` already implements that three-axis lookup, with twelve biomes and
configurable blend widths. It is the closest thing to a starting point that exists.

### The sphere makes temperature a real axis

The arcade note discounts temperature - *"in a top-down fantasy world without latitude, mostly
redundant with elevation"*. That objection is about a flat world with no axis, and it does not
apply here. This planet has a fixed axis: [`spec/planet.md`](../../spec/planet.md) → Presentation
says the roll at every point is fixed and the poles are visible, at the centres of two pentagons.

So latitude exists, and temperature is a genuinely independent axis rather than a second name for
elevation. That buys ice caps, a temperate band and a hot equator - the single strongest cue that
a picture of a ball is a picture of a *planet* - for one cosine and no noise layer at all.

### A spatial function is the right fit, and for a reason already written down

The arcade materials note argues that a material should be a function of position, because two
adjacent pixels have adjacent inputs and therefore produce continuous output - across tiles,
across faces, across parts. Continuity is automatic; there is nothing to make seamless.

On a sphere the payoff is larger than it was on a flat tile map. Evaluating noise on the **3D
unit direction** of each pixel means there is no UV map, no antimeridian and no pole singularity,
because none of those are properties of the sphere - they are properties of flattening it.

That is the same argument, arrived at independently, that this repository already made about
geometry. [`docs/prototypes/planet-view.md`](../prototypes/planet-view.md) records *"rasterizing
beats projecting polygons ... that removes the antimeridian split, the polar special case, and the
degenerate low region counts all at once"*. Two decisions made for different purposes, agreeing:
**do not flatten the sphere, ask each pixel where it is.**

The practical consequence is that terrain needs no new architecture. The rasterizer already
computes a unit direction per pixel in order to find the nearest seed. Terrain is another function
of that direction, evaluated at the same moment.

### Where the arcade research does not carry over

- **Autotile blob patterns, the 47-tile sets, Wang tiles.** All of it assumes a square grid of
  fixed-size tiles. Nothing on this sphere is a square grid, and pixels are being coloured by
  function rather than stamped from tiles. Skip the whole line.
- **Pseudo-3D from a fixed overhead angle** - height parallax, offset drop shadows, layered
  canopies. These fake depth that this project has for real. The one part that still earns its
  place is **normal-mapped relief**: a normal derived from the elevation field, lit by a fixed
  sun, is what makes mountains read as mountains rather than as brown paint.
- **Stickers.** Not terrain. But the spec requires a territory's id drawn on the sphere, and
  ownership has to show somehow; a bounded shape projected onto a surface and composited over the
  base material is exactly that problem, already solved and worth keeping in reserve.

## What the arcade research does not cover

### Noise alone does not make continents

fBm over a sphere gives blobs of high ground - convincing at a glance, wrong on inspection,
because real elevation is not isotropic. Mountains run in *chains*, and they run along the edges
of plates.

The standard fix is a second, much coarser Voronoi tessellation of the sphere - a dozen or two
plates - with a per-plate elevation bias and ridged noise concentrated at the plate boundaries.
That is what turns noise into geography: continents with shapes, ranges with directions, and
oceans that are not merely the low parts.

One trap specific to this project: that is a *second* spherical Voronoi tessellation sitting on
top of the territory one. If the plate count is near the territory count, or shares its seeding,
the two will visibly coincide and every plate boundary will be a territory boundary. Different
seeding, and far fewer plates than territories.

### The terrain must not know about the tessellation, and vice versa

Generate every field from position on the sphere alone - never from cell index, cell centre, or
the icosahedral subdivision that produced them.

This is not tidiness. The last unchecked success criterion in
[`docs/prototypes/planet-view.md`](../prototypes/planet-view.md) is *"a player cannot locate the
twelve pentagons by eye, and cannot find any symmetry axis or grid seam"*. Terrain derived from
the tessellation would carry icosahedral symmetry into the picture and hand the player exactly
that. Terrain derived from position carries none - and in fact helps, because a continent
straddling several cells is the best available camouflage for a regular grid.

### Making the boundary look like it belongs there

[`docs/theory/region-splitting.md`](../theory/region-splitting.md) → Step 6 already specifies
border noise - subdivide each border edge, displace the midpoints with seeded fractal noise - and
already names the constraint that matters: **seed the noise from the unordered pair of region
ids**, or the two sides displace differently and the map cracks along every border.

Terrain adds one thing that section could not know about. Bias the displacement along the local
elevation gradient, and boundaries drift onto ridge lines and coastlines - the places real borders
actually sit. The topology is untouched: who neighbours whom is decided by the tessellation, and
how wiggly the line is drawn remains, in that section's words, purely a rendering decision.

## What `asset-creator` contributes, surveyed 2026-08-28

A second sibling repository, `D:\keep\github\sean\asset-creator`, was read after this note was
first written. It has never been implemented - its world editor is marked *future / out-of-tree* -
but the design is worked out in detail, and it answers a question the arcade research does not:
**how two biomes meet.**

| Source                           | What it gives us                                                      |
| -------------------------------- | --------------------------------------------------------------------- |
| `docs/future/world-editor.md`    | A biome lookup, and blending in parameter space                       |
| `docs/future/noise-functions.md` | Ridged multifractal, turbulence, domain warp, Voronoi, with formulas  |
| `docs/csg-normals.md`            | Two ways of computing normals that fail, and why                      |
| `docs/future/surface-editor.md`  | Shader architecture for materials, shared between objects and terrain |

`asset-manager` has nothing relevant. It is a file browser and bundler for asset libraries.

### Two projects reached the same three axes independently

The arcade's research recommends **elevation, moisture, drainage** and stops there. `world-editor.md`
uses **elevation, moisture, drainage** with per-layer seed offsets and shared fBm parameters. Neither
cites the other.

**That is worth more than either document alone.** The arcade's case is an argument from what a
fourth axis buys; asset-creator's is a design someone worked through to a lookup table. Agreement
between an argument and an artefact is the strongest form this kind of evidence takes.

### The finding this note was missing: blend in parameter space

The arcade's `biome_blender.rs` has *configurable blend widths* and this note recorded that without
saying what they configure. `world-editor.md` says:

> For each pixel, sample the biome function at multiple jittered offsets within a `blend_width`
> radius **in parameter space** (elevation/moisture/drainage). Average the resulting colours.

**The jitter is in the parameters, not on the screen.** Blurring colours in screen space smudges a
boundary and looks like a blur. Jittering the *inputs* asks "what biome would be here if the ground
were slightly wetter?" and averages the answers - so a forest edge dissolves into grassland the way a
real one does, in patches, because that is what the lookup returns for nearby parameter values.

Defaults: `blend_width` 0.06 of the parameter range, `blend_samples` 8. Zero width or one sample
gives hard edges. Offsets are placed at angles `i/n * TAU` with radii `blend_width * fract(i *
0.618034)`, golden-ratio spacing so eight samples spread evenly rather than clumping.

**It transfers to a sphere unchanged, and that is the point.** Parameter space is dimensionless -
three numbers between zero and one. Nothing about the technique knows whether those numbers came
from a flat map or from a pixel's 3D unit direction. The whole adaptation is *where the noise is
sampled*, which [this note already argues](#) must be the 3D direction.

### Micro-variation is a second, cheaper layer

Each biome carries a **base colour and a detail colour**, blended by a separate high-frequency noise
field - `detail_strength` 0.3, `detail_freq` 20. So variation within a biome costs one extra noise
lookup and a colour lerp, not a second material. That is the same economy the arcade note argues for
overlays, arrived at from the other direction.

### Normals, and two ways of getting them wrong

Sean asked about normals and bump mapping. `world-editor.md` derives the normal from the elevation
field directly:

```
dh/dx = (elevation[x+1] - elevation[x-1]) / (2 * step)
dh/dy = (elevation[y-1] - elevation[y+1]) / (2 * step)
normal = normalize(-dh/dx * height_scale, -dh/dy * height_scale, 1.0)
```

**No mesh is involved.** The elevation field is already being evaluated to choose the biome, so a
central difference on it costs two more samples per axis and yields relief lighting for free. On a
sphere the two tangent directions replace x and y; the field is the same.

`csg-normals.md` records two approaches that fail, and both failures are about **meshes** rather
than fields:

- **Triangle cross products give black triangles.** Dual contouring does not guarantee consistent
  winding, so some face normals point inward.
- **Per-vertex SDF gradients give crinkly edges.** Shared vertices average their gradients across a
  sharp edge, so flat faces come out wavy.

Its fix - flat face normals oriented by the SDF gradient, with unshared vertices - **is not needed
here**, because a field-derived normal has no vertices to share and no winding to get wrong. The
value of that document is the warning: **if the realistic drawing ever moves from a per-pixel field
to a generated mesh, both failures are waiting.**

### The noise primitives, with the two that matter most

`noise-functions.md` gives formulas for fBm, ridged multifractal, turbulence, domain warp, marble and
Voronoi. Two are worth naming against what this note already argues:

- **Ridged multifractal** - `(1 - abs(noise))` per octave, each weighted by the last - concentrates
  detail at ridge peaks. This note argues for ridged noise at plate boundaries to make mountain
  chains; this is the formula.
- **Domain warp** - feeding fBm's output back into its own input, twice, with magic constants to
  break symmetry between passes. **This is a second answer to hiding the tessellation**: warping the
  domain destroys any alignment between a noise field and the grid it is sampled over, without
  touching the grid.

## The risk the sphere introduces that a flat world did not

Latitude is real here, and the poles are pinned to the centres of two pentagons by
[`spec/planet.md`](../../spec/planet.md) → Presentation. Temperature by latitude therefore makes
**the same two territories polar on every planet ever generated**, and both of them
five-neighbour cells.

That is a permanent, learnable regularity in a game whose worlds are otherwise generated.

**Settled on 2026-08-28: it is acceptable.** Sean's call, made after P-98 landed and with the
consequence stated - the same two five-neighbour territories are polar on every planet, and a
player who notices has found two of the twelve pentagons for good.

The reason it costs little: **real planets have ice caps**, so a player who learns where the cold
is has learned something true rather than something about the tessellation. The regularity is
legible as geography, which is exactly the disguise the rest of the terrain work is for. And it
is bounded - two territories, both already placed by the spec, and P-98 permits those two and
nothing more.

**Recorded so it is not re-raised.** It is the kind of finding that looks new every time someone
works out that latitude and a fixed axis imply fixed ice caps.

## What this implies for the spec

Two lines are filed as proposals; see [P-97 and P-98](proposals.md). Both build on P-96 rather
than restating it, and both hold regardless of how the question below is answered.

Nothing is filed for *the boundary stays visible over the terrain*, even though this note leans
on it throughout. P-96 already holds that ground: its first open question asks what carries over
between the two drawings, and ids, poles and borders are exactly what is at stake there. Filing a
second proposal against the same question would put two answers to it in the queue.

### The question this note was written around, answered

**Is a territory's terrain a fact of the game model, or only of the picture?**

**Answered by Sean on 2026-08-28: it is a fact of the model.** Each territory has a biome, and
no rule uses it yet. Filed as [P-99](proposals.md), with [P-100](proposals.md) for the line that
keeps the model's biome and the drawn terrain from being two independent things.

Two consequences follow, and neither is filed:

- **Biomes belong under *What a territory carries*,** beside the id and the resource nodes,
  rather than under Presentation. The realistic drawing then illustrates a fact rather than
  inventing one.
- **The first release describes a world with no geography in it.**
  [It](../../releases/first-release.md) gives all twelve territories identical nodes - 6 food at
  6, 4 metal at 8, 5 energy at 7. That is consistent with the spec as it stands, because no rule
  connects a biome to a node. It stops being consistent the moment one does, and that is the
  first place to look when a rule finally reads a biome.

## Open questions

- Ice caps at two fixed pentagons: acceptable, or does temperature need decoupling from latitude?
- **Which biomes exist?** Answered above that a territory *has* one; the list is still open, and
  this note argues it should fall out of where the fields cross rather than be enumerated first.
- How many plates, and are plates visible to the player at all?
- Water: is a territory the field calls ocean simply a territory nobody can claim, or is it absent
  from the game entirely? The tessellation has no way to remove a cell.
- The rasterizer is a CPU loop already costing about 16 ms a frame at 500 regions
  ([planet-view](../prototypes/planet-view.md)). Three noise fields per pixel is far more work
  than one nearest-seed search. At what point does that force the move to the GPU which the same
  document lists as an open question?
