# planet-view

[Prototype notes](../../docs/prototypes/planet-view.md) · [Root README](../../README.md)

The composition root. **This crate contains no logic**, only wiring.

```
scripts/planet-view.ps1                  # or: bash scripts/planet-view.sh
scripts/planet-view.sh --regions 60
scripts/planet-view.ps1 --help
```

## The three files

| File         | Responsibility                                                           |
| ------------ | ------------------------------------------------------------------------ |
| `main.rs`    | Decide headless or windowed, and assemble the app. Four lines of wiring. |
| `options.rs` | Command line parsing. Knows nothing about how the view works.            |
| `capture.rs` | Render one frame to a PNG, with no window and no engine.                 |

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

## Why there are no tests here

Because there is nothing to test. If this crate grows something worth a test, something
has leaked into it that belongs in a layer below. The one exception is `options.rs`,
which has tests for the argument rules — notably that asking for a region count cannot
be silently ignored.
