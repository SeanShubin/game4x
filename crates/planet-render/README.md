# planet-render

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

Turns a sphere into a screenful of pixels. Knows about cameras, projections and
colours; knows nothing about windows, input devices, or any graphics engine.

## Why it is a separate crate

The engine is the part most likely to be replaced. This project already replaced minifb
with Bevy once, and nothing in here changed — because nothing in here can name an
engine type. The compiler enforces that: the only dependencies are the two model
crates.

The other half of the payoff is testing. Because [`PlanetView`](src/app.rs) is a plain
object, a whole session — drag, zoom, resize, regenerate, draw — runs in a unit test
with no window open, and the assertions are made on actual pixels.

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

[`Command`](src/app.rs) is named for intent — `ToggleLabels`, `NextSeed`,
`MoreRegions(n)` — never for a key. Which key means what is the adapter's business.

## Modules

| Module    | Responsibility                                                                 |
| --------- | ------------------------------------------------------------------------------ |
| `app`     | The view as a plain object: state, commands, the readout. The engine boundary. |
| `world`   | A tessellation plus its colouring and verification                             |
| `camera`  | Orientation of the sphere, and the two projections                             |
| `raster`  | Resolving pixels to regions, then shading, borders, labels, cursors            |
| `palette` | Region colours, chosen for grayscale and colour-vision-deficient readability   |
| `font`    | A 5x7 bitmap font, so labels need no text dependency                           |

`camera` is where the interesting geometry lives. The camera is a rotation, not a
position, so panning composes small rotations about the *view* axes — no fixed
up-vector to lose, no gimbal lock, and no special behaviour at the poles.

`raster` does not project polygons. Every pixel asks the sphere which region contains
it, which is why there is no antimeridian split, no polar special case, and no trouble
with one or two regions. Two passes: resolve each pixel to a region and a copy number,
then shade. Borders are found as changes in the resolved buffer rather than from
distances, so they come out an even width at any zoom under either projection. Both
passes are parallel via `std::thread::scope`, no dependency required.

## Tests

- `a_session_can_be_driven_with_no_engine_at_all` — the boundary, demonstrated.
- `the_whole_world_fits_in_the_first_copy`, `the_far_point_becomes_the_entire_rim`,
  `the_world_repeats_beyond_the_rim` — the fanned projection's defining properties.
- `nowhere_on_the_sphere_is_a_special_place` — centre the view anywhere; the centred
  region takes up about the same screen area. A flat map could never provide this.
- `dragging_forever_never_hits_an_edge` — 4,000 drags across both poles.
- `changing_the_region_count_leaves_the_fixed_solid` — the solid's size is fixed, so
  asking for more regions has to switch to a generated world.
