# goldberg-view

[Prototypes](../../docs/prototypes/README.md) · [Architecture](../../docs/architecture.md) · [Root README](../../README.md)

**The question.** `spec/planet.md` allows only certain territory counts — `10T + 2` for
`T = m² + mn + n²` — and the game has picked five of them. Which counts actually read as a
planet you could play on? Twelve is five neighbours everywhere and looks like a die; two
hundred and twelve is a great many small cells. That is a judgement about a picture, so the
way to reach it is to look at all ten in turn without rebuilding anything in between.

**The answer.** Not yet recorded. When it is, it belongs here and in the
[prototype index](../../docs/prototypes/README.md).

## Running it

```
scripts/goldberg-view.ps1        # or: bash scripts/goldberg-view.sh
```

`[` and `]` step through the ten solids. Drag, a finger or the arrows turn the world; the
wheel or a pinch zooms; `R` puts the view back. The digits do nothing here — there is no
game to start.

## Abstract, deliberately

This shows the **practical drawing** only: flat colours, a groove at every boundary, an id
on every face. `spec/planet.md` says that drawing exists to *make adjacency legible*, and
adjacency is the entire subject. The realistic drawing would bury the thing being compared
under terrain.

## What it borrows, and what it refuses

| Borrowed                                                            | Why                                                                |
| ------------------------------------------------------------------- | ------------------------------------------------------------------ |
| [`planet-render`](../../crates/planet-render/README.md)             | Builds the mesh. The geometry is the subject                       |
| [`planet-bevy`](../../crates/planet-bevy/README.md)                 | The sphere, the camera, and turning it                             |
| [`sphere-tessellation`](../../crates/sphere-tessellation/README.md) | Which `(m, n)` arrangements exist, and how many regions each gives |

It refuses everything that plays: no game, no console, no biome, no terrain. That is why it
asks for `GlobePlugin::detached` rather than `GlobePlugin::new`.

**A prototype does not have to touch the game code — but the fact that it *could* is what
keeps the boundaries honest.** A prototype about polyhedra that had to link the command
language in order to draw a sphere would mean the layering was a diagram rather than a fact.
Building this one found two places where it was not yet a fact:

- `Planet` held one of the game's five named sizes, so the globe could draw only counts the
  *game* has. It holds a region count now. The renderer never cared what a planet was
  called.
- Three systems in `globe` reached into `game-front`. They are added only when the globe is
  asked to follow a game, so a detached one links none of the front end.

## The list is derived, not written down

`sphere-tessellation` already knows which arrangements exist and how many regions each
gives, so the ten counts come from the same place the solids are built from. A table typed
in here could disagree with the geometry; this cannot. Counts that more than one
arrangement produces appear once — the question is about the shape of a planet, and two
routes to the same face count are one planet to look at.

## Tests

- `the_ten_smallest_are_the_counts_the_specification_names` — every one is `10T + 2`, they
  strictly increase, and the first five are 12, 32, 42, 72, 92.
- `every_planet_size_the_game_has_is_one_of_them` — the prototype covers what the game does
  as well as what it does not.
