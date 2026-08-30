# planet-presentation

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

How the planet is shown and steered, with no engine underneath it. Where the camera is,
what a hand on the glass is asking for, which ink an id needs to stay legible, what the
heads-up line says.

## Why it is a separate crate

It was a set of items inside `planet-bevy`, and every test covering it was pure and none
of them ran before deploy.

The pipeline gates the published page on a list of engine-free crates, and leaves the
engine-facing ones to a job that runs *after* the page goes out. So a regression in the
pinch floor, or in the ink that keeps ids legible on dark panels, would have been reported
by a green deployment followed by a red notification — in that order. One of the tests
here guards a bug that had already shipped once.

Widening the gate to compile the engine would have made the gate slow. Moving the policy
out from under the engine makes it fast and correct: **a rule that can be checked without
an engine should not sit where checking it needs one.** That is the same argument
[`planet-render`](../planet-render/README.md) makes one layer down, applied to the layer
above it.

## The line between here and the engine

| Here                                | There, in `planet-bevy`               |
| ----------------------------------- | ------------------------------------- |
| What a step of drag means           | What a `Vec2` is                      |
| What two fingers are asking for     | Which system reads the touch messages |
| Where the pitch and zoom limits are | When those systems run                |
| Which ink an id needs               | What a `Color` is                     |
| What the heads-up line says         | Which entity the text is written into |

`planet-bevy` wraps [`Orbit`](src/lib.rs) and [`Fingers`](src/lib.rs) in newtypes so Bevy
can store them as resources, and converts a `Vec2` into a [`Step`](src/lib.rs) at that
seam and nowhere else. Nothing here names an engine type, and the compiler is what says
so: the dependencies are `planet-render` and `sphere-tessellation`.

## What is deliberately not here

**Rotations.** `globe_transform` and `upright` compose quaternions, which is engine
arithmetic, and they stay in `planet-bevy` — whose tests the gate now runs too, in debug,
reusing the build clippy already paid for.

**The drawing.** Which drawing is on screen is a `Drawing` resource in `planet-bevy`.
[`summary`](src/lib.rs) is handed the *name* of the drawing a key would switch to, or
`None` when no game is being followed, so the heads-up line lists a binding only when the
binding exists. A detached globe advertised five keys that started no game for one
release, which is why that is a parameter and not a flag.
