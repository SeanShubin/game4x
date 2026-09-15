# thin-engine

**The question**: can a thin engine run the game from data?

That is `C-114`'s open half. `C-114` is Sean's reason, and it is a reason rather than a
preference: *a thin engine running a data driven game forces inadequacies in the engine and data
structure to come to light sooner*. **A thin engine is an instrument, and the three explosions -
of the code, of the data structure, of the data - are its three readings.**

`C-123` is what the instrument was pointed at first, from the other side. The main tree's engine
has a struct per game noun and a method per rule, and `spec/data/` describes to a reader what the
code already knows; nine of twenty-six recipes could be run from rows and seventeen could not.
**This asks the opposite thing: let the engine know nothing, and see whether the data can say
everything.**

Built to `S-136`. **Nothing here is a decision** - it is research, and if its answer implies one,
that reaches Sean as a proposal through the specification lane, not from this directory.

## The answer

**Yes for one mechanic, and the cost is visible.** `move` runs from data with no `move` in the
code. Measured on 2026-09-14:

|                                             |                                                              |
| ------------------------------------------- | ------------------------------------------------------------ |
| Code that runs, in `src/`                   | **232 lines** - notation 77, store 65, engine 87, `lib.rs` 3 |
| The same engine in `crates/game-model/src/` | **1662 lines**, of which `rules.rs` is 636                   |
| Rows of data                                | **11** - six of world, five of rule                          |
| Game nouns in code that runs                | **0**, checked against a list read out of `data/`            |
| Tests                                       | **15**, all passing                                          |

**The two numbers are not comparable and the table says so by being read carefully.** 232 lines run
one mechanic and 1662 run twenty-six, so the honest reading is not *seven times smaller*. It is
that **the 232 does not grow when a mechanic is added** - a mechanic is rows - and that is the
claim the next concept tests rather than one this one has established.

**What is established is narrower and is the part that was in doubt**: a rule can be *stated* as
rows - what it needs, what it drops, what it adds - and executed without the engine knowing what
any of it means. The engine's whole vocabulary is four words, `rule`, `needs`, `drops` and `adds`,
and they are about how a rule is written rather than about what the game is.

## The three readings, which is what `C-114` asked for

| Does it explode?       | At one mechanic                                                                                                                                                                                                     |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **The code**           | **No.** 232 lines, and three modules with one job each                                                                                                                                                              |
| **The data structure** | **No, and this is the one that surprised.** There is one structure - a row - and rules are written in it too. `{needs rule:move relation:at thing:$it place:$from}` is the same shape as `{at thing:scout place:1}` |
| **The data**           | **Unknown, and honestly so.** Eleven rows is too few to read anything off                                                                                                                                           |

**A reading at one mechanic is not a reading.** The instrument is built and it has been taken
once; what it is for is the second and third mechanic, when the code either stays at 232 lines or
does not. **Recorded as a reading that has been set up rather than one that has been taken.**

**And one thing is answered that the question did not ask.** The main tree's `move` is a method
whose failure is a `Rejection` variant; here a refusal is *the world does not have this row*, named
back to the caller as the row itself. `{adjacent from:1 to:3}` **is** the error message. Every
refusal this engine has is one of five, none of them about the game.

## The two tests, which are the whole of it

Three territories, two adjacencies, one vehicle - `data/world.4x`. One rule - `data/rules.4x`.

- `the_scout_moves_to_a_place_that_is_adjacent` - `{move it:scout from:1 to:2}` succeeds, and the
  scout is at 2 and nowhere else
- `the_scout_does_not_move_to_a_place_that_is_not_adjacent` - `{move it:scout from:1 to:3}` is
  refused, naming `{adjacent from:1 to:3}`

**No resources, no turns, no capacity, no combat**, because neither test needs one. Every absence
below is a concept not yet added rather than a thing left undone.

## Why the notation is re-implemented, which looks like a defect and is not

`crates/command-language` already parses this notation and **this crate does not use it**.

Sean, in `S-136`: *I don't want any dependencies or assumptions creeping in from existing code,
even the notation.* **A borrowed parser is a borrowed decision about what a line may say** - and
the notation is the thing most likely to have to change here, because what the data must express
is exactly what is under test. `src/notation.rs` is 77 lines and answers to this directory alone.

The isolation is checked rather than promised, by `tests/moving.rs`:

- the `[dependencies]` table is empty, so there is no path dependency on any crate
- `std::fs`, `include_str!` and `env!` appear in no line of `src/` that runs, so **the engine
  reads no file at all** - the tests read `data/`, and nothing reaches outside this directory
- its own `[workspace]` in `Cargo.toml`, so it is not a member of the root one and a half-built
  intermediate state cannot redden a gate every lane commits against

## The three things that look like defects and are the design

**Territories are stated and read by nothing.** `{territory id:1}` is in `data/world.4x` and no
rule mentions it, so *not adjacent* and *no such place* refuse identically - moving to territory 9
gives the same error as moving to 3. **That is the first concept to add**, and it is first because
it is the smallest one that needs the engine to check a row it was not handed.

**Adjacency is one-directional.** `{adjacent from:1 to:2}` does not let the scout go back. Making
it symmetric is either a second row per pair, which the data can do today, or a property of a
relation, which is a concept the engine does not have. **Nothing here needs it, so nothing here
decides it.**

**A hole nothing bound is refused rather than matched.** `$name` is substitution and not search: a
hole that matched anything would have to answer *which one, when several match*, and that is the
question a query language exists to answer. The day a rule needs one it arrives as a concept with
a name.

## What comes next, in order

Each is one concept, and each has a test that fails before it:

1. **A place must exist** - distinguishes *not adjacent* from *no such place*
2. **A second rule** - the first real test of whether the engine grows when the game does; the
   claim in the table above is unproven until a rule is added and `src/` does not change
3. **A number** - everything is a string today, and the first rule that counts anything forces
   the question `src/notation.rs` records as deliberately open
4. **A turn** - rules that fire without a command, which is where `block.4x`'s firing order lives
   in the main tree and lives nowhere here

**Concepts 2 and 3 are where this is most likely to fail**, and saying so now is the point of
writing the order down: if the engine has to grow to take a second rule, the answer above is worth
less than it reads.

## Running it

```
cd prototypes/thin-engine
cargo test
```

It is not in the workspace, so the root `cargo test` does not reach it. **Every check here was
poisoned before being trusted** - a game noun added to `src/`, a file read added to `src/`, and
`{adjacent from:1 to:3}` added to the world - and all three failed the run they should have.
