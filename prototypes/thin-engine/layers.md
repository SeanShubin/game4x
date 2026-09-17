# Layers

**What may say what, and what stops it being said in the wrong place.**

Sean, 2026-09-16, asking for this: *lets talk about the layers we have and how we are going to make
sure we keep the layers separate and don't put something in the wrong layer.*

**A layer is not a folder and not a format.** It is a question about who may change a thing and what
breaks when they do. Two files in one directory can be different layers, and one layer can be
written in two notations.

## The layers, from the bottom

| Layer         | Where                                 | What only it may say                                                     |
| ------------- | ------------------------------------- | ------------------------------------------------------------------------ |
| **code**      | `src/`                                | what data cannot express                                                 |
| **boundary**  | `data/*/engine.4x`                    | every word the code branches on                                          |
| **structure** | `data/*/schema.4x`                    | relations, their columns, what points at what, which relations are state |
| **ruleset**   | `data/*/rules.4x`, `data/*/things.4x` | this game's rules and this game's categories                             |
| **scenarios** | `data/*/tests/*.4x`                   | one world, one act, and what should be true afterwards                   |
| **harness**   | `data/*/script.4x`, `data/*/setup.4x` | how a scenario is run, which is nothing about the game                   |

**The harness is beside the others rather than under them.** `store`, `test` and `load` are not part
of the game at any level; they are how a scenario is loaded and run. It is the layer that is easiest
to forget, because it does not sit in the stack.

## Two things that are not layers

**Friendly and foundation are one layer in two spellings.** Same facts, one reader each - a person
and this lane. Nothing may be true in one and not the other, and `tests/directories.rs` is what says
so. **Treating them as layers would be the mistake**, because a layer is a place a fact lives and
these are two ways of writing the same place.

**A test is not a layer above the ruleset it exercises.** It is the same distance from the engine as
any other data; what makes it a layer of its own is that it may state a world and nothing else.

## What holds each boundary

**A boundary with nothing on it is a convention, and a convention is what somebody remembers.**

| Boundary            | Held by                                                 | How                                                                                                                                    |
| ------------------- | ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| code / boundary     | `tests/engine.rs`                                       | **both ways**: a `const` in `src/` with no row fails, a row with no `const` fails, and the count is asserted                           |
| code / ruleset      | `tests/isolation.rs`                                    | no noun the game has may appear in a line of `src/` that runs                                                                          |
| structure / ruleset | `Schema::fits`, and the reference check                 | a rule cannot invent a column, and every reference must resolve                                                                        |
| ruleset / scenarios | `a_scenario_states_a_world_and_an_act_and_nothing_else` | a `given` or a `then` holds only state; a `when` holds only a rule's name                                                              |
| scenarios / ruleset | `the_ruleset_states_no_world`                           | no file of the ruleset may hold a row of a state relation                                                                              |
| harness / game      | `run_test`                                              | the script store is checked against `script.4x` alone, sections excluded, and a row that is neither a step nor in a section is refused |

**The two scenario checks need no new data.** `{state relation:...}` already says which relations a
world is made of and `{rule name:...}` says what may be commanded, so the rule is read off what the
structure and the ruleset already declare.

## The one that was open, and what it cost

**Until 2026-09-16 nothing held the ruleset/scenario boundary.** A test's `given` could declare a
category, a rule and a role, and every test still passed - measured, not supposed.

**It had already gone wrong once.** Two tests used `thing id:1`, for `scout` in one and `labor` in
the other, and the friendly-to-foundation translation could not say which. **That was fixed as an
instance**: the categories moved to `things.4x`, which is the ruleset, and the collision went away.
**The class stayed open for three more commits**, because moving the data did not stop the next test
declaring its own.

**Which is the lesson worth keeping.** A layer violation that is fixed by moving the data is fixed
once; a layer violation that is fixed by a check is fixed for every file that comes after.

## How to tell which layer a fact belongs to

**Ask what would have to change with it.**

- *If this changed, would every test change?* Then it is the ruleset or below.
- *If this changed, would one test change?* Then it is a scenario.
- *Could two games disagree about it?* Then it is the ruleset. `scout` is; `relation` is not.
- *Does the engine branch on this word?* Then it is the boundary, and `engine.4x` has to say so.
- *Is it about running a test rather than playing the game?* Then it is the harness.

**A category is the ruleset's, not a scenario's** - Sean, 2026-09-16: *All of the tests share the
same instance of the game rules, so the rules for one test are the same as the rules for another.*
A scenario that declares `scout` is saying what the game is, in a file about one situation.

## What is still unheld

**Nothing separates structure from ruleset in the other direction.** `schema.4x` could declare a
relation only this game needs, and nothing would say so - the check above only stops a *world* from
appearing there. What would catch it is a statement of which relations are the notation's own and
which are this game's, and that statement does not exist yet.

**Nothing stops the harness knowing a game noun.** `script.4x` could declare a relation called
`territory` and be loaded into its own store, and the two would not collide. It has not happened and
there is no check that would notice.

Both are written here rather than built, because a check earns its place by a failure it could
produce and neither has produced one - `docs/process.md`. **This file is where to look when one
does.**
