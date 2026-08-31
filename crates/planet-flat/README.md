# planet-flat

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

The Bevy adapter for the flat projection: a window driving the software rasterizer, and the
shader that mirrors it.

## Why it is a separate crate

It was part of [`planet-bevy`](../planet-bevy/README.md), and **the game never used a line
of it.** That crate was two unrelated adapters sharing a name: the globe, which `game4x`
and the Goldberg prototype draw with, and this, which only
[`planet-view`](../../prototypes/planet-view/README.md) uses. Neither binary used the
other's half and both paid for both.

A dependency tree is worth having when it describes what a binary actually contains.

It still names `planet-bevy` for one thing — `window_plugin`, which is how every
composition root here asks for a window with vsync.

## Two ways to draw the same thing

| Path            | Where it runs   | What it is                              |
| --------------- | --------------- | --------------------------------------- |
| `Renderer::Cpu` | `planet-raster` | The reference, with pixel-level tests   |
| `Renderer::Gpu` | `planet.wgsl`   | A direct port of `raster`, per fragment |

`G` switches between them on the same frame, which is what makes them comparable. **When
they disagree, the CPU one is right.**

The shader keeps the per-pixel technique rather than switching to meshes, and that is the
point: no antimeridian split, no clipping at the rim, no instancing for repeating rings,
and no trouble at one or two regions. Fragments do not consult their neighbours, so there
are no seams to get wrong.

## The palette arrives, it is not written down

`planet.wgsl` used to carry the palette as decimals transcribed by hand from
`planet-render/src/palette.rs`. Two copies of six colours, and nothing compared them:
changing a hex value drew a different world on the GPU than on the CPU, silently. They had
already drifted by rounding, and it never showed — the difference survives sRGB to linear
and back to eight bits unchanged, so the sources disagreed while the output agreed.

Both palettes, the background, the border and the two mix strengths now arrive in the
uniform. They are sent as **sRGB rather than linear**, because the shader mixes borders and
tints in sRGB the way the rasterizer does and converts once at the end; sending linear
would change the arithmetic rather than only its source.

## Checking it

The shader has no pixel array to hand back — its output exists only as a rendered frame —
so `planet-view` grew a way to photograph a window:

```
cargo run -p planet-view -- --regions 92 --capture cpu.png
cargo run -p planet-view -- --regions 92 --renderer gpu --shot gpu.png --settle 40
```

The two draw the same palette. Without this the GPU path could not be photographed at all,
and any change to the shader was a change nobody could verify.
