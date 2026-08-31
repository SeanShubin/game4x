# planet-render

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

Turns a sphere into a screenful of pixels. Knows about cameras, projections and
colours; knows nothing about windows, input devices, or any graphics engine.

## Why it is a separate crate

The engine is the part most likely to be replaced. This project already replaced minifb
with Bevy once, and nothing in here changed — because nothing in here can name an
engine type. The compiler enforces that: every dependency is a crate beneath it -
the geometry, the colouring, the planet model and the terrain. **Not the game.** It
depended on `game-model` for one `use` line until `Biome` moved to `planet-model`,
which is where every rule about a biome is written down anyway.

The other half of the payoff is testing: nothing in here needs a window to be checked.

## What is here, and what left

| Module      | Responsibility                                                                |
| ----------- | ----------------------------------------------------------------------------- |
| `world`     | A tessellation plus its colouring and verification                            |
| `mesh`      | The world as triangles, for an engine to upload                               |
| `realistic` | The other drawing: terrain shaded from the field, at a much finer subdivision |
| `palette`   | Region colours, chosen for grayscale and colour-vision-deficient readability  |

`app`, `camera`, `raster` and `font` are [`planet-raster`](../planet-raster/README.md)
now. **The game never called any of them** - this crate was two crates wearing one name,
world building and the mesh on one side and a software rasterizer on the other, and every
binary paid for both. `game4x` reached about 2,200 lines through code it never calls.

`mesh` and `raster` remain the same layer answering the same question two ways, one crate
apart. `raster` asks, per pixel, which seed is nearest; `mesh` hands over the polygons and
lets the hardware answer it. Neither knows which engine consumes it - there are no engine
types in either file, only numbers.
## Tests

- `a_session_can_be_driven_with_no_engine_at_all` — the boundary, demonstrated.
- `the_whole_world_fits_in_the_first_copy`, `the_far_point_becomes_the_entire_rim`,
  `the_world_repeats_beyond_the_rim` — the fanned projection's defining properties.
- `nowhere_on_the_sphere_is_a_special_place` — centre the view anywhere; the centred
  region takes up about the same screen area. A flat map could never provide this.
- `dragging_forever_never_hits_an_edge` — 4,000 drags across both poles.
- `changing_the_region_count_leaves_the_fixed_solid` — the solid's size is fixed, so
  asking for more regions has to switch to a generated world.
