# game-model

[Architecture](../../docs/architecture.md) · [Layers](../../docs/layers.md) · [Root README](../../README.md)

The game: a state, a transition, and the one function between them.

`spec/invariants.md`: *a game state and a transition yield a new game state. There is no
other way for state to change.* That is not a description of this crate, it is its entire
shape. `Game::after` is the function; everything else is the state it reads or the
transition it is given.

```rust
pub fn after(&self, transition: &Transition) -> Result<Game, Rejection>
```

The old state is left alone and a new one is returned. A rejection yields no state at all,
so a command that cannot be run changes nothing.

## Two consequences, both easy to erode

- **Designing the world goes through it too.** Which phase a game is in is part of its
  state, so `create planet` and `land ark` are the same kind of thing and take the same
  path. There is no separate constructor that builds a world some other way.
- **A game is exactly its transitions.** Applying the same list to the same start yields
  the same game, always. Nothing is seeded from a clock, nothing reads the environment,
  and there is no floating point.

## What it does not know

It has never heard of a parser, a renderer or an engine. It does not know where a
territory sits on a sphere: adjacency arrives as a graph of integer ids, computed above
and handed in with the transition that creates the planet. Its dependency list is empty.

## Public surface

| Type                                                             | What it is                                                              |
| ---------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `Game`                                                           | The whole state: phase, turn, territories, adjacency, units             |
| `Phase`                                                          | `Design` or `Play`. Part of the state, which is why one function serves |
| `Transition`                                                     | Every way a game state may change, one variant apiece                   |
| `Rejection`                                                      | Why a transition was refused, in the terms of the game                  |
| `Territory`                                                      | A territory and what it carries: nodes, extractors, citizens, stores    |
| `Node`                                                           | One resource node and its density                                       |
| `Extractor`                                                      | A structure worked to draw a resource out of a node                     |
| `cost`                                                           | The release's tuning figures, gathered in one module                    |
| `Unit`                                                           | A unit and where it is                                                  |
| `Location`                                                       | In orbit, or on a territory                                             |
| `TerritoryId`, `UnitId`, `UnitKind`, `StructureKind`, `Resource` | Canonical identity and the closed lists                                 |

The design transitions are the five `spec/console.md` says are available only before
`start`; `Transition::is_design` is what tells them apart, and a test asserts the set is
the one the specification lists.

## Integers only

`docs/architecture.md` rule 3: floating point stops at this boundary. It is enforced
rather than asserted — a test scans this crate's own source and fails if `f32` or `f64`
appears outside a test. Beyond reproducing identically on every machine, that is what
makes resolving territories in any order safe: integer addition is associative, so a sum
does not depend on how the work was split.

## Where the numbers come from

`spec/README.md` rule 7 puts relationships in the specification and values in a release.
The tuning constants here — what a Pioneer costs, what an Ark costs, how many cells a move
spends — are the ones `releases/first-release.md` states. Nothing keeps a constant in Rust
and a figure in a markdown table in step except a test, and that test is
[`game-console/tests/first_release.rs`](../game-console/tests/first_release.rs), which
parses the release document and compares.
