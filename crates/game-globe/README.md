# game-globe

[Architecture](../../docs/architecture.md) · [Root README](../../README.md)

Binds the globe to the one game: the counter it watches, and the keys that type a line.

## Why it is a separate crate

`planet-bevy` used to name `game-front`, so everything that drew a globe linked the console,
the grammar and the game with it. **A prototype comparing Goldberg solids was linking the
command language in order to draw a sphere.**

`GlobePlugin::detached` was the first attempt: a globe that added no system reaching the
front end. It worked as far as it went and could not go far enough, because *the code was
still in the crate* — a dependency is a fact about a manifest, not about which systems get
scheduled. `cargo tree -p goldberg-view` still contained the command language, and one of
those systems, `reset_view`, called into the front end from the unconditional list anyway.

So the direction is inverted. `planet-bevy` knows nothing about a game; this crate names
both it and `game-front` and joins them, which is what a binding is. There is no flag and no
second constructor any more — a prototype gets a globe, and the game gets a globe plus this.

The check is now something a compiler does:

```
cargo tree -p goldberg-view | grep -E 'game-front|game-console|command-language'
```

Empty, and it will stay empty by construction rather than by care.

## What is here, and what stayed

| Here                                     | In `planet-bevy`               |
| ---------------------------------------- | ------------------------------ |
| Watching the game's generation counter   | Drawing whatever `Planet` says |
| `1`–`5` typing `/new <size>`             | —                              |
| A control on the page asking for a reset | The `R` key                    |
| A control asking for the other drawing   | The `T` key                    |

The two rows with an entry on both sides were one system each, and the split is the point:
a sphere with no edge to bump into needs a way back whether or not anything is being played
on it, so the key belongs to any globe. The *control* is how a tablet reaches the same
place — `spec/interface.md` does not allow a capability to need a key the platform may lack
— and a control lives on a page that has a game behind it.

## Why a counter, four times over

The one `Session` lives outside the engine, and on the web it is not even on the same call
stack, because the page calls into it. It cannot hand anything over when something changes.
So every system here watches a number that only goes up, and acts when the number it last
saw is not the number it sees now.

## One plugin, so two things cannot disagree

`FollowsTheGamePlugin` sets `FollowsTheGame(true)` in the same breath as adding the systems
it describes. That resource is what puts the game's bindings in the readout, and for one
release the systems and the advertisement of them were two separate things to remember — so
a detached globe listed five keys that started no game, on the first screen of the newest
prototype.
