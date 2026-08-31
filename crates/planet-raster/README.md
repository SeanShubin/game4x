# planet-raster

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

A software rasterizer for a tessellated sphere: a camera, projections, a font, and one
frame at a time. Knows about pixels; knows nothing about windows, input devices, or any
graphics engine.

## Why it is a separate crate

It was four modules of [`planet-render`](../planet-render/README.md), and **the game never
called any of them.** That crate was two crates wearing one name — world building and the
mesh, which the game draws with, and this, which only
[`planet-view`](../../prototypes/planet-view/README.md) uses. Every binary paid for both:
`game4x` reached about 2,200 lines of rasterizer, camera, font and app through code it
never calls.

A dependency tree is worth having when it describes what a binary actually contains.
Splitting is what makes it an audit rather than a diagram.

It sits *above* `planet-render` rather than beside it, because it draws the same worlds:
the world and the palette are shared, and only the technique differs.

## Public surface

```rust
let mut planet = PlanetView::new(WorldSpec::default(), 1000, 820);

planet.drag(dx, dy);                 // turn the sphere
planet.zoom(x, y, notches);          // zoom about a point
planet.apply(Command::ToggleProjection);
planet.resize(width, height);        // returns true when it changed

let mut pixels = vec![0u32; planet.pixel_count()];
planet.draw(&mut pixels, cursor);    // 0x00RRGGBB per pixel
```

`Command` is named for intent — `ToggleLabels`, `NextSeed`, `MoreRegions(n)` — never for a
key. Which key means what is the adapter's business.

Because `PlanetView` is a plain object, a whole session — drag, zoom, resize, regenerate,
draw — runs in a unit test with no window open, and the assertions are made on actual
pixels.

## Modules

| Module   | Responsibility                                                                 |
| -------- | ------------------------------------------------------------------------------ |
| `app`    | The view as a plain object: state, commands, the readout. The engine boundary. |
| `camera` | Orientation of the sphere, and the two projections                             |
| `raster` | Resolving pixels to regions, then shading, borders, labels, cursors            |
| `font`   | A 5x7 bitmap font, so labels need no text dependency                           |

`camera` is where the interesting geometry lives. The camera is a rotation, not a position,
so panning composes small rotations about the *view* axes — no fixed up-vector to lose, no
gimbal lock, and no special behaviour at the poles.

`raster` does not project polygons. Every pixel asks the sphere which region contains it,
which is why there is no antimeridian split, no polar special case, and no trouble with one
or two regions. Two passes: resolve each pixel to a region and a copy number, then shade.
Borders are found as changes in the resolved buffer rather than from distances, so they
come out an even width at any zoom under either projection. Both passes are parallel via
`std::thread::scope`, no dependency required.

## Its double

[`planet-flat`](../planet-flat/README.md) carries a shader that is a direct port of
`raster`, so the same world can be drawn either way on the same frame. **This is the
reference**: it is the one with pixel-level tests, and when the two disagree it is the one
that is right.
