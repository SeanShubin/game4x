# planet-bevy

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

The Bevy adapter: a window, input, and presentation. **The only crate in the project that
knows a graphics engine exists.**

## What it does

Two views of the same model, and no logic of its own.

[`globe`](src/globe.rs) draws the world as a solid you can turn in your hands. It takes
the mesh that [`planet-render`](../planet-render/README.md) built out of plain numbers,
hands it to Bevy, points a camera at it, and turns the ball when the pointer moves. This
is what the application shows.

[`gpu`](src/gpu.rs) draws the flat projection, and does the opposite: rather than upload
polygons, it answers "what colour is this pixel" for every pixel, in a fragment shader.
This is what the planet-view prototype shows.

Both draw the same model, and the model cannot tell which one is running. Between them,
the crate does three things:

1. Configures a window with `PresentMode::AutoVsync`.
2. Maps Bevy's input events onto [`planet-render`](../planet-render/README.md)'s plain
   methods — drag, zoom, and a `Command` per key.
3. Uploads geometry, or a pixel buffer, depending on which view is up.

If a second adapter is ever written, nothing below this crate should need to change.
That is the test of whether the boundary is real.

## Why Bevy

The prototype was built on `minifb`, which has **no vsync** — not a setting that was
missed, there are zero references to it in that crate. It blits whenever asked, the
blit lands mid-scanout, and the result tears visibly while panning. Bevy presents
through wgpu with vsync, so frames swap during the vertical blank.

Bevy is pulled in with `default-features = false, features = ["2d", "3d", "ui"]`, which
drops audio. Nothing here uses it, and its wasm backend is a known source of startup
panics on a static host.

## The solid

The mesh arrives as positions, normals, linear colours and indices, and this crate's whole
job is to hand them over. No geometry is computed here.

The ball turns rather than the camera, so that the key light stays put — a terminator that
swung about while you were trying to look at something would be much harder to read. Each
region is drawn as a fan of triangles slightly inset toward its own centre, so the gaps
between panels read as grooves; a darker sphere sits just underneath to fill them.

## The flat projection's presentation path

The rasterizer writes `0x00RRGGBB` per pixel, because that is what the palette and font
code has always used and what its tests assert on. Turning that into a texture is one
pass converting to RGBA bytes. The image uses nearest sampling — it is a pixel buffer
at 1:1, so filtering would only blur it.

Buffers are sized from the window's *logical* resolution rather than its physical one.
On a HiDPI display that means the texture is upscaled by the GPU, which is both correct
and considerably cheaper than rasterizing at 4K.

## Usage

The plugin does not add `DefaultPlugins` — assembling the app is the composition root's
job, so that the wiring stays visible in one place:

```rust
App::new()
    .add_plugins(DefaultPlugins.set(window()))
    .add_plugins(planet_ecs::PlanetEcsPlugin::new(topology))
    .add_plugins(planet_bevy::globe::GlobePlugin::new(spec))
    .run();
```
