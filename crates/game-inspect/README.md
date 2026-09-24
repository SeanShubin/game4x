# game-inspect

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

The remote control: put the camera at stated angles, choose a drawing, run commands, wait for
the world to settle, then write a PNG and a text dump and quit.

## Why there is a remote control at all

The globe is drawn by a graphics engine into a window, and neither a test nor an agent working
on this code can see a window. Everything below the engine is testable without one — that is
what the layering is for — but **the picture is exactly the part that is not**, and half of
`spec/planet.md` is about the picture: *the terrain of the realistic drawing is continuous*,
*nothing in the terrain reveals how the sphere was divided*, *the two drawings share the camera
and nothing else*. Those are claims about pixels.

So the application can be asked a question and made to answer with a file. What comes back is
evidence rather than an opinion.

## It drives the shipped binary

Every line from `--run` goes through the one console, exactly as typing it would. Nothing here
reaches into the model, and **nothing here is compiled differently from what ships** — the same
binary plays and poses. A harness that ran a special path would be evidence about the harness.

## Why it is a separate crate

It was `crates/game4x/src/inspect.rs` until `S-160`.

`docs/architecture.md` rule 4 says the composition root **may assemble plugins, but it may not
compute with engine types**, and rule 6 says it again from the other side: an algorithm never
names `Entity`, `Query`, `Commands` or `Res`. This code takes `Res<Errand>` and writes through
`ResMut<Orbit>`, which is computing with them.

**Nothing was red.** The rule had been written down and unheld for as long as it had existed,
and what found it was the specification lane measuring rule 4 for `P-545` rather than any check.

**A crate boundary rather than a rewrite**, because the paragraph above is what this crate rests
on: moving the code makes it an engine adapter — the layer that is allowed to name those types —
and a crate the binary depends on is still in the binary. Rewriting it to avoid `Res` would have
made the harness a special path, which is the one thing it must not be.

`options.rs` came with it. It names no engine type and could have stayed, but it is the type
this plugin takes, so leaving it behind would have made the root depend on the plugin for a
type while the plugin depended on the root for nothing. The root is `main.rs` alone now, which
is what assembling means.

## What is here

| File         | What it is                                                                   |
| ------------ | ---------------------------------------------------------------------------- |
| `lib.rs`     | `InspectPlugin`: the systems that place the camera, settle, capture and quit |
| `options.rs` | What the application was asked to do, parsed from the command line           |

`scripts/shot.sh` and `scripts/shot.ps1` are how it is usually run.
