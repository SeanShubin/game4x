# planet-terrain

[Architecture](../../docs/architecture.md) · [Planet appearance](../../docs/notes/planet-appearance.md) · [Root README](../../README.md)

One continuous field over the sphere, and the biome that falls out of it.

`docs/notes/planet-appearance.md`: *generate a continuous field over the sphere and let both
the picture and the model read it, rather than generating a picture and a model separately
and hoping they agree.*

- The **model** reads it once per territory, at that territory's seed, and gets one discrete
  answer — which is what claiming and working a territory needs.
- The **renderer** will read it per point, and gets terrain that flows across boundaries.

They cannot disagree, because one is derived from the other rather than authored beside it.
That is what `spec/planet.md` means by *a territory's biome is what the terrain gives it. It
is not chosen independently of the surface the territory covers.*

## It does not know the tessellation exists

Nothing here takes a territory, a neighbour list or a cell boundary. `sample` takes a
direction and a seed and nothing else — which is what makes *the terrain is continuous … it
runs across boundaries* true **by construction rather than by care**: there is no boundary
in scope to run up against.

The one regularity that survives is latitude, and it is deliberate. A fixed axis makes the
same two territories polar on every planet, both five-neighbour cells. The note records
Sean's decision that this is acceptable: real planets have ice caps, so a player who learns
where the cold is has learned something true about the world rather than something about the
tessellation.

## Axes

| Axis        | Where it comes from                                                 |
| ----------- | ------------------------------------------------------------------- |
| Elevation   | Broad fractal noise for continents, with ridges added inland        |
| Moisture    | Fractal noise on an unrelated offset                                |
| Drainage    | Fractal noise, and it only does work where moisture is already high |
| Temperature | Latitude, less a lapse rate for height. No noise layer at all       |

Drainage is Dwarf Fortress's insight rather than an invention: high rain and low drainage is
swamp, high rain and high drainage is forest, and it does not subdivide a desert at all.
Temperature is free on a sphere with a fixed axis — one cosine — and it is the single
strongest cue that a picture of a ball is a picture of a *planet*.

Biomes are not enumerated and placed. `biome_of` is the crossing of those four fields
written down, read as the questions a place answers about itself, most decisive first: is it
underwater, is it frozen, is it above the trees, is it dry, and only then how the wet drains
away.

## Two things the measurements decided

Both constants were set against four thousand points rather than chosen and hoped for.

- **Summing octaves narrows a field.** Each octave is an independent average, so the total
  piles up in the middle and the tails thin out — raw fractal noise ran 0.37 to 0.67 between
  the tenth and ninetieth percentiles, which would have left a planet with almost no desert
  and almost no swamp, because both live in a tail. `spread` puts the tails back.
- **Ridges are added only to land**, by a weight that grows from nothing at the shoreline.
  Adding them everywhere pushes the sea floor into the air; adding them with a hard cutoff
  at sea level puts a cliff along every coast. A smooth weight does neither and keeps the
  field continuous.

## Reproducibility

Floating point lives here, above the game logic, which
[architecture](../../docs/architecture.md) rule 3 allows. The biome that comes out is an
integer fact and that is what the model stores — the same arrangement adjacency already has,
where the geometry is computed above and handed in with the transition.

Only addition, subtraction and multiplication over `f64`, with no transcendental functions
and no library random number generator. The lattice is hashed rather than tabled, so the
field is infinite and stateless and two calls for the same point anywhere in the program
agree with nothing carried between them.

That is what keeps `history` a save file: the biomes are **not written into the history**.
They are recomputed from `create planet <size>`, so the same commands rebuild the same world
on somebody else's machine.

## Tests

- `the_field_is_continuous` — two points a thousandth of a radian apart cannot differ much.
  A discontinuity would be order 1; the bound is loose because a ridge is legitimately steep.
- `the_terrain_varies_inside_one_territory` — sampled at an **equatorial** territory, because
  a pole is entirely ice by design and is the one place where uniform ground is the correct
  answer and proves nothing.
- `the_biomes_that_exist_are_the_biomes_that_occur` — every biome the model can hold occurs
  somewhere, or the list is longer than the crossing of the fields justifies.
- `most_of_the_planet_is_ocean`, `it_is_cold_at_the_poles_and_warm_at_the_equator` — the two
  things that make a ball read as a planet.
- `the_same_seed_gives_the_same_field` and `a_different_seed_gives_a_different_field` — the
  seed does something, and it does the same thing twice.
