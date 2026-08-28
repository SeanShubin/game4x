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

It draws **the game and nothing else**. The console and the data browser used to be Bevy
`Text` nodes in this crate, and are now elements on a page or lines on a terminal — see
[game-front](../game-front/README.md) for the four things that fixed at once.

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

## Turning it, whatever you are holding

A mouse drag, a held arrow key and a finger all arrive at the same `Orbit::drag`, so they
cannot mean different things. `R` puts the view back. That matters more than it sounds: winit routes a touch to
`TouchInput` and never to `MouseInput` or `CursorMoved` — its web backend tests
`pointer_type != "touch"` before raising a pointer event — so without a path of its own a
tablet reaches none of the mouse or keyboard systems, and the planet cannot be turned at
all. `spec/interface.md` does not allow that: nothing is available in one build and not
another, and only *how* the user acts on it follows the platform.

Two fingers pinch to zoom. The angle between them is computed nowhere, deliberately:
`spec/planet.md` fixes the roll for any point on the planet and says nothing the user does
changes it, and a twist is the gesture that would reach roll. A test twists a pair through
a full turn and asserts the view does not move.

Touch is read from the `TouchInput` messages rather than from Bevy's `Touches` resource,
which only refreshes `previous_position` on frames that carried an event — with a finger
held still, `Touch::delta` keeps reporting the last movement and the world would drift on.

## Following the game

The one `Session` lives outside the engine, in [`game-front`](../game-front/README.md). On
the web it is not even on the same call stack, because the page calls into it, so it cannot
hand over the new state when it changes. `globe` watches a generation counter instead and
rebuilds when the number it last saw is not the number it sees now.

The number keys choose a planet size, and they do it **by typing the line** — `/new <size>`
at the one console, exactly as a person would, and exactly what the buttons on the page
type. They used to write `Planet::size` directly, which let the view hold a world the model
did not have; a view that can do that is not a projection of the model. Going through the
console means a key, a button and a typed line take the same path, and the globe learns
about the result the same way in all three cases: by watching the counter.

`/new <size>` rather than `create planet <size>`, because the second is available only
before `start` and the shipped build opens on a game already under way — so every size key
would have been refused, correctly and uselessly.

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
