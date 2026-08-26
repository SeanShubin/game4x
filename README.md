# Game 4X

A 4X game written in Rust. 4X is the genre known as **e**xplore, **e**xpand, **e**xploit, **e**xterminate.

The game itself is not built yet. What exists is the design documentation, the world
generator, and something to look at.

## Look at it

**[seanshubin.github.io/game4x](https://seanshubin.github.io/game4x/)** - the current
world, in a browser. Every push to `master` republishes it; the page reports which commit
it is running.

The same thing, natively:

```
scripts/game4x.ps1             # or: bash scripts/game4x.sh
```

A Goldberg polyhedron: twelve pentagons at the icosahedron's vertices and hexagons
everywhere else, coloured so no two neighbours match. Drag to turn the world, wheel to
zoom, arrow keys to turn.

There is also an earlier prototype that flattens the sphere instead of turning it:

```
scripts/planet-view.ps1        # or: bash scripts/planet-view.sh
```

A sphere divided into regions, fanned out flat so you can see all of it at once. Drag
to turn the world, wheel to zoom, `P` to fold it back into a globe, `Esc` to quit. See
[the planet view prototype](docs/prototypes/planet-view.md).

## Start here

| Document                                | What it covers                                                                           |
| --------------------------------------- | ---------------------------------------------------------------------------------------- |
| [Specification](spec/README.md)         | What the game **is**, stated normatively. If it is not there, it is not decided          |
| [Vision](docs/vision.md)                | What the game is, the design constraints, and the non-negotiables                        |
| [Architecture](docs/architecture.md)    | Module boundaries, the composition root, dependency rules                                |
| [Layers](docs/layers.md)                | `(old world, events) -> new world`: what must be reproducible, and how it stays parallel |
| [Prototypes](docs/prototypes/README.md) | Standalone programs demonstrating one aspect of the game each                            |
| [Theory](docs/theory/README.md)         | Background research the design leans on                                                  |
| [Documentation map](docs/README.md)     | Every document in the repo, and the rules for adding one                                 |
| [Scripts](scripts/README.md)            | How to run each prototype                                                                |
| [Notes](docs/notes/README.md)           | Derived records of analysis. Not binding                                                 |

## The short version

- The programming language is Rust.
- The game is organized into multiple modules.
- A thin composition root module ties everything together.
- The game logic module uses **whole numbers only** — no floating point.
- A separate module renders the game.
- Other modules exist as needed to keep the code clean.
- Many prototypes demonstrate individual aspects of the game in isolation.
- Documentation stays organized: **everything is discoverable from this file.**

## Theory documents

The two pieces of background research the planet view depends on:

- [Splitting a sphere into regions](docs/theory/region-splitting.md) — why a perfect
  hex grid on a sphere is impossible, which tessellations exist, and which one this
  project uses.
- [Coloring regions](docs/theory/region-coloring.md) — the minimum number of colors
  needed so no two adjacent regions match, and how to pick colors humans can actually
  tell apart.

## Prototypes

- [Planet view](docs/prototypes/planet-view.md) — a sphere divided into regions, fanned
  out flat so the whole world is visible at once, colored so no two neighbours match.
  **Built.**

## Repository layout

```
README.md              you are here — the entry point to everything
Cargo.toml             workspace root
docs/
  README.md            documentation map and documentation rules
  vision.md            what the game is
  architecture.md      how the code is organized
  prototypes/          one document per prototype
  theory/              background research, independent of implementation
scripts/
  README.md            how to run each prototype
crates/
  sphere-tessellation/ dividing the sphere, the adjacency graph, and the solid
  graph-coloring/      few-color assignment, no geometry
  planet-model/        the model: old world + intents -> new world, integers only
  planet-ecs/          regions as ECS entities; systems that gather and apply
  planet-render/       camera, software rasterizer and mesh; no graphics engine
  planet-bevy/         the Bevy adapter: window, input, vsync, the solid in 3D
  game4x/              the application: composition root, and the page that is published
prototypes/
  planet-view/         composition root; wiring only
.github/workflows/     one pipeline: gate, deploy to Pages, then verify
hooks/                 opt-in git hooks; see scripts/README.md
```

Documents under `docs/` say *why*. Each crate's own `README.md` says *how*, and is
linked from [architecture.md](docs/architecture.md).
