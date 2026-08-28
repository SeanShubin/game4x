# How Other Projects Generate Planets

**Derived.** Written by Claude from conversation, 2026-08-28. Not binding - a survey of published
work read against this repository. See [the specification](../../spec/README.md) for what was
actually decided.

[Notes index](README.md) · [Planet appearance](planet-appearance.md) · [Depth on a sphere](depth-on-a-sphere.md)

Sean pointed at a Pistol Shrimp dev diary and asked what else applies. Two sources had real
technical content; the rest were tools and asset packs. **Sean also scoped it: the spec is not ready
for multiple planets**, so this note separates what applies to one planet from what only matters once
there are several.

## What was read

| Source                                                                                                                            | What it is                                                               |
| --------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| [Pistol Shrimp, *Dev Diary: Procedural Planet Art*](https://pistolshrimpgames.com/2024/01/dev-diary-procedural-planet-art/)       | A shipping team's account of making planets that read well at two scales |
| [Toni Sagristà, *Procedural generation of planetary surfaces*](https://tonisagrista.com/blog/2021/procedural-planetary-surfaces/) | The Gaia Sky author, with formulas and parameters                        |

## Three things confirmed independently

Worth recording as corroboration rather than as news, because agreement between unrelated projects is
what makes a technique safe to rely on:

**Sample noise in 3D on the sphere's surface.** Sagristà converts spherical coordinates to cartesian
and samples there, precisely so that there is no seam and no pole artefact. This is what
[planet appearance](planet-appearance.md) already argues from the arcade research, and what
[the prototype](../prototypes/planet-view.md) already does for finding the nearest seed. **Three
independent routes to the same answer.**

**Water gets full specular and land gets none.** Sagristà assigns full specular to heights at or
below zero. [The depth note](depth-on-a-sphere.md) reaches the same place from the argument that it
separates water from land at any zoom.

**A texture that reads well far away does not read well close up.** Pistol Shrimp: *"The planet
texture which looked good in Orbit did not look good on Planetside."* The depth note argues the same
thing from first principles - far out it is a planet and the limb carries everything, close in it is
terrain. **A shipping team hit this and had to restyle.** It is the finding here most likely to cost
real time if ignored.

## The one technique that would change a decision: the lookup is an image

Sagristà does not use threshold ranges. **The biome map is a texture**: humidity on x, elevation on
y, and the colour is whatever the image holds at that point. Smooth or stepped is a property of the
image, not of the code.

**That is a second solution to the problem `asset-creator` solved by jittering.**
[Planet appearance](planet-appearance.md) records blending in parameter space - sample the biome
function at jittered offsets and average. A lookup image gets the same smooth transition from one
texture fetch and bilinear filtering, instead of eight lookups and an average.

**The two are not interchangeable, and the difference is which layer needs the answer.** A blended
*colour* is all the picture wants. But `spec/planet.md` says a territory has **one biome**, and a
model needs a name rather than a blend. So:

- **The picture** wants the smooth read - a lookup image, filtered.
- **The model** wants the discrete read - which cell of the table the territory's terrain lands in.

**They can be the same table read two ways**, which is worth noticing before someone builds two.

## What applies to a single planet

- **Layered fBm with lacunarity 2.0 and persistence 0.5** as the starting parameters. Sagristà gives
  these as defaults; `asset-creator` exposes the same two knobs. A concrete place to start rather
  than a search.
- **Normals from elevation gradients in two directions.** Identical to `asset-creator`'s central
  difference and to what [the depth note](depth-on-a-sphere.md) argues for. **Three sources, one
  formula.**
- **A cloud layer is the same noise with the z scale stretched**, which streaks it directionally. One
  extra field, no new machinery, and it would sit above the terrain without touching it.
- **Crater stamping.** Pistol Shrimp stamps craters onto a surface as discrete marks. That is the
  *sticker* problem the arcade research already documents - a bounded shape projected onto a surface
  - and it is the same machinery a territory id or an ownership mark would need.

## What waits for multiple planets

**Sean's scoping is right and these do not apply yet**, but they are the cheap ways to make planets
differ once there are several:

- **A planet type, assigned before anything is generated.** Pistol Shrimp assigns rocky, gem and
  others, and the type selects the parameters. **Distinctness comes from the type, not from the
  noise** - two planets with the same type and different seeds look like siblings, which is what you
  want.
- **Hue-shifting in HSL.** Sagristà shifts hue for variety. It costs a colour transform and no new
  material, and it is the same economy the arcade research argues for overlays.
- **Lighting as a character axis.** Pistol Shrimp: *"Even the same type of planet being lit
  differently gives the experience unique character."* Since [the depth note](depth-on-a-sphere.md)
  argues the sun should be fixed per planet so shadows can be baked, **the sun's direction and colour
  are then per-planet constants** - free variety from a decision already worth making.

## What did not apply

The rest of what surfaced was tools, shaders and asset packs rather than technique. And one
performance note is worth discounting deliberately: Sagristà's 2024 edit reports moving generation to
the GPU made it *"almost instantaneous."* **That is a solution to a problem this project may not
have** - generation happens once per planet here, not per frame, and `docs/architecture.md` keeps the
model free of the engine. A slow generator that runs once is not the same problem as a slow shader.
