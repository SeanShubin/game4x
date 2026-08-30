# Architecture

[Documentation map](README.md) · [Root README](../README.md)

## Shape

The code is a Cargo workspace. Each module is its own crate, so the dependency rules
below are enforced by the compiler rather than by discipline.

```
composition root  (binary)
        |
        +--> engine adapter  (library)
        |            |
        +--> rendering  (library)
        |         |
        +---------+--> game logic  (library)
                            |
                            +--> supporting crates (library)
```

Dependencies point in one direction only, downward. Nothing below depends on anything
above it.

## The layers

The single most important line in this project runs between **the model** and **the
graphics engine**. Everything else is detail.

The reasoning behind that line — what has to be reproducible, why transcendental
functions are allowed on one side and not the other, and how the simulation stays
parallel without becoming unpredictable — is in
[layers: intent to pixels](layers.md). This document covers the crate graph; that one
covers why it is shaped this way.

| Layer             | Knows about                  | Does not know about             |
| ----------------- | ---------------------------- | ------------------------------- |
| Supporting crates | Spheres, graphs, integers    | Pixels, windows, engines        |
| Rendering         | Pixels, cameras, projections | Windows, input devices, engines |
| Engine adapter    | Bevy, windows, input, vsync  | How anything actually works     |
| Composition root  | All of the above, briefly    | Nothing else; it holds no logic |

That line has already paid for itself once. The prototype was built on `minifb` and
then moved to Bevy; the model, the camera and the rasterizer did not change a line, and
the whole migration was contained in one new crate plus four lines of wiring.

## The modules

### Composition root

A binary crate. It constructs the concrete implementations, wires them to each other,
and starts the program. That is all it does.

It is the only place in the codebase allowed to know about every other module at once.
Because it holds no logic, it needs no tests of its own — if it is large enough to be
worth testing, something has leaked into it that belongs elsewhere.

Concretely, the planet view's root is three small files: option parsing, a headless
capture path, and a `main` whose entire windowed branch is four lines of plugin
assembly.

### Engine adapter

A library crate, one per engine. It maps the engine's events onto the rendering layer's
plain methods and presents the pixels that come back. It owns no logic.

This is where an engine's opinions are allowed to land, and nowhere else. If a second
adapter is ever written, nothing below it should need to change — that is the test of
whether the boundary is real.

### Rendering

A library crate. Turns game state into pixels.

- Owns all geometry: sphere coordinates, region outlines, projections, the camera.
- Owns all floating point.
- Owns the controlled randomness that makes borders look geographic rather than
  computed. That randomness is seeded from the game state so it is stable across frames
  and across sessions, but it never feeds back into the game logic.
- Reads game state; never mutates it.
- **Knows nothing about any engine.** It draws into a plain slice of pixels, which is
  why a whole session can be driven and asserted on in tests with no window open.

### Game logic

A library crate. The rules of the game: turns, regions, adjacency, resources, combat,
victory conditions.

Its whole job is one function — `(world state, intent array) -> next world state` — and
that function must be **reproducible** and **confluent**: same answer on any machine,
and same answer regardless of the order operations resolve in. See
[layers](layers.md#5-confluence-how-order-is-made-irrelevant).

- **Integers only.** No floating point. See [vision](vision.md#whole-numbers-only-in-the-game-logic).
  Beyond cross-machine reproducibility, this is what makes parallel reduction safe:
  integer addition is associative, floating point addition is not.
- No knowledge of rendering, windowing, input devices, or file formats.
- No knowledge of the *geometry* of a region. A region is an identifier; adjacency is
  an edge in a graph. Where a region sits on the sphere is the renderer's problem.
- Deterministic: same inputs and same seed produce the same game, everywhere.

### Supporting crates

Added as needed to keep the above clean. A crate earns its existence by having a
boundary someone else could reasonably want to cross. "This file got long" is not a
boundary.

## What exists today

| Crate                                                                   | Kind       | Depends on                                                                                                   | What it holds                                                                   |
| ----------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------- |
| [`crates/sphere-tessellation`](../crates/sphere-tessellation/README.md) | algorithm  | **none**                                                                                                     | Seeds, relaxation, the adjacency graph, and the solid's corners                 |
| [`crates/graph-coloring`](../crates/graph-coloring/README.md)           | algorithm  | **none**                                                                                                     | Few-color assignment; no geometry                                               |
| [`crates/planet-model`](../crates/planet-model/README.md)               | model      | **none**                                                                                                     | `(old world, intents) -> new world`; integers enforced by test                  |
| [`crates/planet-terrain`](../crates/planet-terrain/README.md)           | model      | `game-model`, `sphere-tessellation`                                                                          | One noise field over the sphere, and the biome a territory's ground gives it    |
| [`crates/planet-ecs`](../crates/planet-ecs/README.md)                   | entities   | `bevy`, `planet-model`                                                                                       | Regions as entities; systems gather and apply, and hold no rules                |
| [`crates/planet-render`](../crates/planet-render/README.md)             | view model | `game-model`, `graph-coloring`, `planet-model`, `planet-terrain`, `sphere-tessellation`                      | Camera, projections, software rasterizer, and the mesh                          |
| [`crates/planet-bevy`](../crates/planet-bevy/README.md)                 | view       | `bevy`, `game-front`, `planet-ecs`, `planet-model`, `planet-render`, `planet-terrain`, `sphere-tessellation` | Window, input, vsync presentation, and the solid in 3D                          |
| [`crates/command-language`](../crates/command-language/README.md)       | algorithm  | **none**                                                                                                     | Grammar, positions and typed syntax trees; a test scans it for game nouns       |
| [`crates/game-model`](../crates/game-model/README.md)                   | model      | **none**                                                                                                     | The game state and the one function over it; no words, engine or floating point |
| [`crates/game-console`](../crates/game-console/README.md)               | binding    | `command-language`, `game-model`, `planet-model`, `planet-terrain`, `sphere-tessellation`                    | The only place a word meets a rule                                              |
| [`crates/game-front`](../crates/game-front/README.md)                   | front end  | `game-console`, `game-model`, `wasm-bindgen` on web                                                          | The three surfaces of this release, and the shell that hosts them               |
| [`crates/game4x`](../crates/game4x/README.md)                           | binary     | `bevy`, `game-console`, `game-front`, `planet-bevy`, `planet-ecs`, `planet-render`                           | Composition root; the binary that is built, run and published                   |
| [`prototypes/planet-view`](../prototypes/planet-view/README.md)         | binary     | `bevy`, `planet-bevy`, `planet-ecs`, `planet-render`, `png`                                                  | Composition root for the flat projection                                        |
| [`prototypes/goldberg-view`](../prototypes/goldberg-view/README.md)     | binary     | `bevy`, `planet-bevy`, `planet-render`, `sphere-tessellation`                                                | Composition root for comparing solids side by side                              |

The integers-only rule is now exercised: `planet-model` has a test that scans its own
source and fails if `f32` or `f64` appears in code. The algorithm and view-model crates
sit on the geometry side of that line and use floating point freely.

Worth noting what the two model crates depend on: **nothing**. No math crate, no random
number generator, no parallelism crate. That is deliberate — the generator is
hand-written so that results are identical on every platform, which is what makes
"same seed, same world" a guarantee rather than a hope.

## Rules

1. **Dependencies point one way.** If two modules need each other, the shared part
   belongs in a third module beneath both.
2. **Only the composition root sees the whole graph.** Everything else depends on
   interfaces and on the modules below it.
3. **Floating point lives above the game logic.** The game logic boundary is where
   `f32` and `f64` stop.
4. **Engine types live only in the adapter.** No `bevy::` anywhere else, including in
   the composition root's own logic — the root may assemble plugins, but it may not
   compute with engine types.
5. **Each crate has a `README.md`** describing its purpose and its public surface,
   linked from this document.
6. **Entities and algorithms are different kinds of thing.** Every game entity is an ECS
   entity — there is no second way of holding game state. Every algorithm is a pure
   function over plain data that never names `Entity`, `Query`, `Commands` or `Res`.
   Systems are glue between the two and contain no rules of their own. See
   [layers](layers.md#6-entities-and-algorithms-are-different-kinds-of-thing).
7. **Algorithm crates do not depend on Bevy.** That is what makes rule 6 enforced by the
   compiler rather than by discipline, and `cargo tree` is the audit.
8. **`Entity` is never identity.** Bevy reuses entity ids and does not keep them stable
   across runs or saves. Canonical identity is the model's own integer id. See
   [layers](layers.md#what-ecs-demands-in-return).
9. **Nothing in the model may depend on execution order.** Not on system order, not on
   query iteration order, not on hash iteration. Where a sequence is genuinely needed,
   canonicalise the *result* by sorting on a data-derived key rather than ordering the
   work.

## Open questions

- Does rendering depend on the game logic crate directly, or on a trait-defined view of
  it so prototypes can supply fake state? Leaning toward the latter.
- The rasterizer is a CPU loop over pixels. Bevy brings a GPU with it; at some point
  the sensible thing is to draw regions on the GPU instead, at which point "rendering"
  and "engine adapter" may need to merge or be re-cut.
- Where does input handling live once there is a game to control — the adapter, or a
  crate of its own beneath it?
