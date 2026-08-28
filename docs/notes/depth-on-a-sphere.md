# Making a Planet Look Like It Has Depth

**Derived.** Written by Claude from conversation, 2026-08-28. Not binding - options and arguments,
not decisions. See [the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Planet appearance](planet-appearance.md) · [Biomes](biomes.md)

Sean asked what makes a rendered planet read as having depth, mentioned bump mapping and StarCraft,
and asked what he has not thought of. This sorts the techniques by what they buy **here**, on a
sphere seen whole, which is a different problem from terrain seen from inside it.

## What I could not verify

Sean mentioned a recent innovation in **Crimson Desert**. I do not have reliable knowledge of a
specific rendering technique from it and have not invented one. If it matters, it wants looking up
rather than recalling.

## The insight that changes which techniques are worth it

**At planet scale, bump mapping's usual weakness does not apply.**

The standard objection to normal and bump mapping is the **silhouette**: you perturb the shading but
not the geometry, so a bumpy wall still has a straight edge, and the illusion collapses at the
outline. Every technique above bump mapping in cost - parallax, relief, displacement - exists mostly
to fix that.

**A planet's silhouette is a circle.** Mountains do not break a planet's horizon seen from space;
Everest is a thousandth of Earth's radius. So the thing the expensive techniques buy is a thing this
project **should not have**. Normal mapping is not a compromise here - it is correct, and displacement
would be actively wrong.

That reverses the usual ordering. On a sphere, the cheapest technique is also the most accurate one.

## What actually makes a sphere read as solid

In rough order of effect per unit of work:

**1. The light direction, before any technique at all.** A sphere lit from behind the camera looks
flat no matter how good the shading is - every normal faces the light equally. A sphere lit from the
**side** shows relief along the terminator, where grazing light turns a two-percent slope into a
visible shadow. **Choosing where the sun is buys more than choosing how the surface is shaded.**

**2. Normal mapping from the elevation field.** The field already exists for biomes, and
[`asset-creator`](planet-appearance.md) gives the central-difference formula. Two extra samples per
axis per pixel. **The cheapest large win, and the one Sean already had in mind.**

**3. The limb.** The edge of a planet is where a viewer decides whether it is a sphere or a disc.
Real planets darken toward the limb, and an atmosphere brightens and softens it. This is a function
of the angle between the surface normal and the view direction - which the rasterizer already has,
because it computed that direction to find the nearest seed. **Nearly free, and specific to spheres
in a way that generic terrain advice never covers.**

**4. Ambient occlusion, approximated.** Darken where the height field is locally concave. Valleys
get dim, ridges stay bright, and the eye reads depth that no single normal conveys. A curvature
estimate is a second difference of a field already being sampled.

**5. Self-shadowing.** Terrain casting shadows across other terrain is the strongest depth cue there
is, and the most expensive - it needs marching along the light direction through the height field.
See the baking note below, which makes it affordable.

**6. Specular on water only.** Ocean glints; land does not. One test on the biome, and it separates
water from land instantly at any zoom, in a way colour alone does not.

## What Sean may not have considered

### Zoom changes which technique matters

`spec/planet.md` says the user can zoom in and out. **Those are two different rendering problems.**

Far out it is a **planet**: the limb, the terminator, the atmosphere and the overall shape carry
everything, and per-pixel terrain detail is below a pixel. Close in it is **terrain**: normals,
shadows and surface texture carry everything, and the limb is off-screen.

**A technique that is essential at one zoom is invisible at the other.** That is a reason to
implement them in the order above rather than all at once, and a reason not to judge any of them at
a single zoom level.

### The StarCraft lesson is about baking, not about art

StarCraft's units were **modelled in 3D and pre-rendered to sprites**. The striking thing was not the
modelling - it was that expensive lighting could be computed once, offline, because the viewing angle
was fixed.

**Here the camera is free, so that trick does not transfer directly. But the sun can be fixed.**
Nothing in the spec says a planet rotates or that light moves. If the sun is fixed relative to the
planet, then **shadows and ambient occlusion never change**, and both can be computed once at
generation time and stored with the terrain.

That converts the two most expensive items on the list into a one-off cost. **Rotating the camera
does not move the shadows, because the camera is not the light.**

Worth stating what it forbids: a day/night cycle, or a sun that moves with the seasons. Neither is
in the spec, and a strategy map arguably wants neither - a permanently-lit hemisphere is a game
board, not a simulation.

### Relief comes from the terrain generator, not the shader

A shader can only reveal what the height field contains. **fBm alone gives blobs**, and blobs shaded
beautifully still look like blobs. [Planet appearance](planet-appearance.md) already argues for a
coarse plate layer with ridged noise at the boundaries, so ranges run in chains.

**The ordering matters:** ridged noise plus flat shading looks more like a planet than smooth noise
plus every technique on this page. Depth is a property of the field first and the lighting second.

### The two drawings mean this can be expensive

`spec/planet.md` says the practical and realistic drawings **share the camera and nothing else**. So
the realistic one is free to cost far more per pixel than the practical one, and a player who needs
speed can switch. **Nothing has to be fast enough to be the only mode**, which is a larger licence
than it first appears - the rasterizer's ~16 ms a frame at 500 regions is a budget for the practical
drawing, not a ceiling for both.

## What makes detail read as high quality

Sean asked for planets that look like they carry a lot of high-resolution, realistic detail. That is
a different problem from depth, and **more octaves is not the answer to it.**

### Procedural detail is unlimited in resolution and poor in variety

Fractal noise can be zoomed into for ever, which sounds like infinite detail and is the exact reason
it reads as fake. **fBm looks the same at every scale** - that self-similarity is what the eye picks
up as *computer-generated*, and adding octaves adds more of the same tell.

Real terrain has **different structure at different scales**: plates, then ranges along their edges,
then valleys cut into the ranges, then scree at the foot of the valleys. Each of those is a
*different process*, not the same process at a smaller frequency. **The quality comes from using a
different generator per scale**, which is the argument [planet appearance](planet-appearance.md)
already makes for a coarse plate layer, extended downward.

### Erosion is the single largest quality jump, and nobody we read mentioned it

Neither published source, neither sibling repository, and nothing in this repository names it.
**Hydraulic erosion is what separates terrain that looks generated from terrain that looks real.**

Noise produces hills that have no reason to be where they are. Erosion produces **drainage
networks** - water runs downhill, collects, carves, and deposits - and the eye reads the result as
landscape because every feature is a consequence of every other. Valleys are V-shaped near ridges and
U-shaped where they fill. Ridgelines become continuous rather than lumpy. Sediment fans out where
slope drops. **None of that can be added by another octave**; it is a relationship between places,
and noise has no relationships.

### And erosion is where the *function of position* model breaks

This matters more than the technique. [Planet appearance](planet-appearance.md) records a strong
argument: **a material is a function of position, so continuity is automatic** - adjacent pixels have
adjacent inputs and there is nothing to make seamless. That is why 3D noise on the sphere has no
seam and no pole.

**Erosion is not a function of position.** It is a simulation with state, iterated: water has to
arrive from uphill before it can carve downhill, so a pixel's value depends on a whole catchment
rather than on its own coordinates. It cannot be evaluated per pixel and it cannot be evaluated
independently.

So it has to be **baked** - computed once over a grid or a mesh at generation time and stored. The
trade is exact:

|                         | Pure function of position | Baked simulation      |
| ----------------------- | ------------------------- | --------------------- |
| Zoom                    | unlimited                 | fixed resolution      |
| Storage                 | none                      | a field per planet    |
| Seams                   | none by construction      | whatever the grid has |
| Erosion, flow, sediment | impossible                | natural               |

**The answer is both, and the crossover is the design decision.** Bake erosion coarsely - enough for
ranges, valleys and river courses - and add pure procedural detail on top of it for everything finer
than the baked resolution. That is what planet renderers generally do, and it is the same shape as
the baking argument for shadows above: **compute what has memory, evaluate what does not.**

### Coherence beats density

A planet whose rivers run downhill into seas looks more real than one with twice the noise. **The eye
reads quality from features that relate to each other correctly**, not from feature count. That is
why erosion buys more than octaves, why plate-driven ranges buy more than louder fBm, and why the
biome lookup should be driven by the *same* elevation field that drives the shading - a desert on a
mountaintop is a coherence failure that no amount of resolution hides.

### Detail that is smaller than a pixel must be removed, not drawn

**A procedural planet that shimmers looks cheap however detailed it is.** At the zoom where a whole
planet is on screen, most of the terrain detail is below one pixel, and sampling a high-frequency
field at those points produces noise that crawls as the camera turns.

The fix is the procedural equivalent of a mipmap: **drop octaves as the pixel footprint grows**, so
the highest frequency drawn is always near the sampling limit. It costs nothing - it is fewer noise
evaluations, not more - and skipping it is the most common reason procedural work looks worse in
motion than in screenshots.

**And it interacts with the zoom split above.** Far out, the high octaves are dropped and the limb,
terminator and colour carry the image. Close in, they come back and the limb is gone. The two
regimes hand off to each other, which is a reason to build the octave-dropping early rather than
bolting it on when the shimmer is noticed.

## What this does not settle

Which of the two render paths the realistic drawing uses - the CPU rasterizer or the Bevy 3D path -
is the code lane's, and the spec deliberately constrains neither. `spec/planet.md` asks only for
*a three-dimensional sphere*; a textured sphere, a polyhedron and a raymarched field all satisfy it.
