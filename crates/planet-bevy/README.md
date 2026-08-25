# planet-bevy

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

The Bevy adapter: a window, input, and vsync presentation. **The only crate in the
project that knows a graphics engine exists.**

## What it does

Three things, and no logic of its own:

1. Configures a window with `PresentMode::AutoVsync`.
2. Maps Bevy's input events onto [`planet-render`](../planet-render/README.md)'s plain
   methods — drag, zoom, and a `Command` per key.
3. Draws into a pixel buffer each frame and uploads it as a texture on a sprite.

If a second adapter is ever written, nothing below this crate should need to change.
That is the test of whether the boundary is real.

## Why Bevy

The prototype was built on `minifb`, which has **no vsync** — not a setting that was
missed, there are zero references to it in that crate. It blits whenever asked, the
blit lands mid-scanout, and the result tears visibly while panning. Bevy presents
through wgpu with vsync, so frames swap during the vertical blank.

Bevy is pulled in with `default-features = false, features = ["2d"]`, which drops 3D,
UI and audio.

## The presentation path

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
    .add_plugins(DefaultPlugins.set(planet_bevy::window_plugin(width, height)))
    .add_plugins(planet_bevy::PlanetViewPlugin::new(spec))
    .run();
```
