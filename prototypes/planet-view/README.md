# planet-view

[Prototype notes](../../docs/prototypes/planet-view.md) · [Root README](../../README.md)

The composition root. **This crate contains no logic**, only wiring.

```
scripts/planet-view.ps1                  # or: bash scripts/planet-view.sh
scripts/planet-view.sh --regions 60
scripts/planet-view.ps1 --help
```

## The three files

| File            | Responsibility                                                           |
| --------------- | ------------------------------------------------------------------------ |
| `main.rs`       | Decide headless or windowed, and assemble the app. Four lines of wiring. |
| `options.rs`    | Command line parsing. Knows nothing about how the view works.            |
| `capture.rs`    | Render one frame to a PNG, with no window and no engine.                 |
| `photograph.rs` | Screenshot what the *engine* drew, for the path that has no pixel array. |

The windowed branch in full:

```rust
App::new()
    .add_plugins(DefaultPlugins.set(planet_bevy::window_plugin(width, height)))
    .add_plugins(planet_bevy::PlanetViewPlugin::new(options.spec()))
    .run();
```

Everything it assembles lives elsewhere:
[`planet-render`](../../crates/planet-render/README.md) for the view itself,
[`planet-bevy`](../../crates/planet-bevy/README.md) for the engine.

## Headless capture

```
scripts/planet-view.sh --capture frame.png --zoom 0.3 --turn-up 30
```

Renders one frame and exits, printing frame time, region count and colour count. It
touches no engine code at all — that path exists only because the rendering crate has
no engine in it.

## Photographing the shader

`--capture` draws through `PlanetView::draw`, the CPU rasterizer. **The shader has no
pixel array to hand back** — its output exists only as a rendered frame — so seeing it
means opening a window, letting it draw, and screenshotting the result:

```
cargo run -p planet-view -- --regions 92 --renderer gpu --shot gpu.png --settle 40
```

`--renderer gpu|cpu` picks which path draws; `--shot` names the file; `--settle` is how
many frames to let pass before the shutter, because the first frame has no world in it.

This exists because of a specific thing that could not otherwise be checked. The palette
lived twice — as hex in `planet-render/src/palette.rs`, and as decimals transcribed by
hand into `planet.wgsl` — so changing one drew a different world on the GPU than on the
CPU and nothing said so. Deleting the copy was a change nobody could verify, because
nothing could photograph the thing being changed.

Now both can be photographed and compared, and they draw the same palette:

```
cargo run -p planet-view -- --regions 92 --capture cpu.png
cargo run -p planet-view -- --regions 92 --renderer gpu --shot gpu.png --settle 40
```

A prototype may hold a harness like this in its composition root; `docs/prototypes/README.md`
allows a shortcut the game may not, provided the document says which and why. The why is
that a picture is the only evidence about a picture.

## Why there are no tests here

Because there is nothing to test. If this crate grows something worth a test, something
has leaked into it that belongs in a layer below. The one exception is `options.rs`,
which has tests for the argument rules — notably that asking for a region count cannot
be silently ignored.
