# game4x

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

The application. **A composition root and nothing else** - it decides what to build and
wires the layers together, and contains no logic of its own.

This is also the crate that gets published: the same `main.rs` compiles natively and to
WASM, and the WASM build is what
[seanshubin.github.io/game4x](https://seanshubin.github.io/game4x/) serves.

## What it wires

```text
  command-language  +  game-model            the game, pure
           \              /
            game-console                     the only door into it
                  |
             game-front                      the console and browser, no engine
                  |
  sphere-tessellation  +  graph-coloring     the geometry, pure
           \                  /
            planet-render                    a view model, no engine
                  |
             planet-bevy                     window, input, presentation
                  |
                game4x                       wiring, and nothing else
```

Three plugins, in the order they appear in `main`:

1. `DefaultPlugins`, with a window this crate describes. Assembling the app is the
   composition root's job, so the window lives here rather than in the engine adapter.
2. `planet_ecs::PlanetEcsPlugin` - game entities and the turn. No rendering, no rules.
3. `planet_bevy::globe::GlobePlugin` - the solid, a camera, and the pointer.

Plus, on the desktop only, a thread reading stdin for
[`game-front`](../game-front/README.md)'s terminal shell. The engine's event loop never
returns, so a console that shares the main thread with it would never get a line.

It is the only place that knows both that Bevy exists and that the game exists at the
same time.

## Which world it opens on

**The one the game has.** `main` asks [`game-front`](../game-front/README.md) how many
territories there are, and that answer comes from a console already open on the release's
planet - built out of `commands/setup.4x`, the only way a world can be built, by running
commands. The globe then follows: it watches a generation counter and rebuilds when the
game moves.

There is one constant left, `UNDESIGNED`, and it is the fallback for a game designed no
further than `create planet` - ninety-two rather than twelve, so a blank world is
obviously a blank one. Ninety-two is `GP(3,0)`: twelve pentagons at the icosahedron's
vertices and eighty hexagons between them, **constructed rather than searched for**.

Region counts that are not Goldberg numbers still work - they just have no perfect answer
to construct, so generation falls back to relaxation and picks up extra 5-7 defect pairs.
See [region splitting](../../docs/theory/region-splitting.md) for which counts are
Goldberg numbers, and [the note on in-between counts](../../docs/notes/in-between-counts.md)
for what that costs and what does not fix it.

## Running it

```
scripts/game4x.ps1        # natively; or bash scripts/game4x.sh
scripts/web.ps1           # as WASM on localhost:8080; or bash scripts/web.sh
```

Turning the world is a drag with a mouse, a finger on a touch screen, or the arrow keys.
Zooming is the wheel or a pinch. `R` puts the view back.

`1`–`5` start a new game on a planet of that size, smallest to largest, and so do the
buttons along the bottom of the game surface and `/new <size>` typed at the console. All
three type the same line, so none of them is a second way to change anything.

On the web, the keyboard belongs to the canvas. Leaving the console gives it back, because
winit listens for keys there and focus otherwise stays on the text field.

## The three surfaces

`spec/interface.md` asks for the game, the console and the data browser, all reachable,
in every build. Only the first of the three is drawn by the engine.

| Surface     | Desktop          | Web                                        |
| ----------- | ---------------- | ------------------------------------------ |
| The game    | the window       | the canvas                                 |
| The console | stdin and stdout | a text field and a transcript, on the page |
| The browser | `/browser`       | a panel, reached by its own button         |

Why they are not drawn by the engine, and what that fixed, is in
[game-front](../game-front/README.md).

## The web build

[`index.html`](index.html) is the Trunk entry point. It sits next to this crate's
`Cargo.toml` because Trunk resolves the cargo manifest from its working directory, and the
workspace root is a virtual manifest with no root package.

Three things in that file are worth knowing about:

- **Build provenance.** CI rewrites the `__BUILD_COMMIT__` and `__BUILD_TIME__` placeholder
  meta tags and writes a matching `build-info.json`. The served page therefore always
  reveals which commit is live, *even when the wasm fails to boot*. A local `trunk build`
  leaves the placeholders, which the page reports as `(local dev)`.
- **A diagnostics overlay**, registered before the wasm module. Rust panics cross the JS
  boundary as errors or rejected promises; both are caught and painted onto the page along
  with which GPU backends the browser exposes. That last fact is what separates "this
  browser cannot run it" from "the build is broken", and without it a failed startup is a
  silent black screen.
- **The console, the data browser and the size controls**, as ordinary elements: a `<pre>`
  that scrolls and selects, an `<input>` that iOS will raise a soft keyboard for, and
  `<button>`s big enough to tap. They call into the wasm through `game-front`'s web shell, which is the
  only thing crossing that boundary and passes nothing but strings.

Bevy is pulled in with `default-features = false`, dropping audio - nothing here uses it,
and its wasm backend is a known source of startup panics on a static host.

## Publishing

Every push to `master` runs [the pipeline](../../.github/workflows/pipeline.yml), which
gates on lint plus the geometry tests, deploys the WASM bundle it just built, and only then
runs the fuller verification. A failure in that last stage reds the run but never undoes a
deploy that already happened.
