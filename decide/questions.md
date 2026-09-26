# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-562 - three of the seven capabilities waiting on you rest on the ruleset `D-1` replaces, so does the switch wait on your reading?

**to** sean · **status** open · **raised** 2026-09-25 · **rewritten** 2026-09-25, one of the three answers was not available · **asks** a decision · **kind** entailed · **into** `releases/first-release.md`, and how the code lane sequences `D-1`

## What "the switch" is, since this question turns on the word

**One step, not the migration.** `game-console` reaches the rules through `Transition` and
`end_turn`, which today resolve to `crates/game-model/src/rules.rs` - **1,424 lines of Rust that
are the rules.** The switch is the step where that path stops resolving to `rules.rs` and starts
firing the data ruleset through the engine.

**Three things people mean by "moving to thin-engine" have already happened and none of them was
the switch.** The engine's code moved into `crates/game-model` on 2026-09-25, `f633864a`. The
ruleset began shipping with the binary the same day, `c438986a`. The reviewed tests already run,
in `crates/game-model/tests/`.

**What none of that changed is which rules the game plays by.** Measured just now: nothing outside
`game-model` fires the data ruleset, and `rules.rs` is still 1,424 lines and still what a command
reaches. **The switch is the moment the game a player runs starts obeying the data**, and `D-2`
is the same moment seen from the tests - they stop being about the engine alone and start being
about the game.

**Two answers, and neither needs anything from you today.**

```
W1  the switch waits for your word   the code lane builds up to it, says when it is ready,
                                     and throws it when you say. You read R-6, R-7 and R-8
                                     whenever you like before then
W2  the switch does not wait         the three go stale, and are re-run and re-offered
                                     once the new model plays. You read them once, later
```

## What changed since this was filed, and it is the whole of the rewrite

**A third answer offered you `D-2` and `D-3` early, and the release cannot deliver it.** The code
lane checked it against the words rather than against the intent, and it fails:

```
D-1  "the game fires the changed rule"
D-2  "the model the game itself plays on"
D-3  "the game reads its data from the data files at run time"
```

**All three say *the game*, and the game is what `game-console` runs.** So none of the three is
observable until the console is on the new model, which is `D-1` itself - **they do not complete in
an order, they complete together.** This lane confirms the strict reading is the one intended:
`D-2` contrasts it with *today they run against `crates/thin-engine` and against nothing else*, and
under a loose reading `D-2` would already be met and would be saying nothing.

## And checking it made `W1` cost nothing, which is the part worth your eye

**What makes `R-6`, `R-7` and `R-8` stale is one step, not the whole migration.** Their evidence is
`reports/recipes.md` and `scenario/expected/play.4x`, and **both are produced by running
`crates/game-model`** - re-derived here: `worked.rs`, which the recipes report is built from, and
`tests/expected_state.rs`, which writes the scenario's expected state, each construct a
`game_model::Game`.

**So nothing regenerates until `game-console` stops calling `rules.rs` and starts firing the
engine.** Everything before that - a loader that reads the data at run time, the world the engine
has to hold, binding a command to a rule - **touches no generated file at all.** That is the bulk of
the work, and it is invisible to the three.

**`W1` therefore costs no waiting.** It was filed as though it did.

## What this lane would say

**`W1`**, and the code lane independently proposed the same thing. It is `W1` with none of the cost
this item first attributed to it. `C-143` already commits that lane to filing before it throws the
switch, so the difference between the two answers is only whether your reading is a gate on it or a
thing you do afterwards.

## What is not in question

**Nothing here asks you to re-approve `D-1`, `D-2` or `D-3`.** The capabilities are right and this
is about when one step inside them happens.

### P-563 - the game's rules now ship from the one column you do not author, and rule 8 says where they go

**to** sean · **status** open · **raised** 2026-09-26 · **asks** a decision · **kind** entailed · **into** where `rules.4x`, `schema.4x` and `engine.4x` live, and `spec/data/`

**Three answers, and the measurement that separates them is which files name a game noun.**

```
H1  all three move to spec/data/        1,683 lines. engine.4x goes too, and it is
                                        the one file with no game noun in it
H2  all three stay in crates/           cheapest. The game's rules sit in the column
                                        the code lane writes and you do not author
H3  rules.4x and schema.4x move,        1,614 lines move, 69 stay. The split is
    engine.4x stays                     measured rather than judged
```

## Why it is now rather than whenever

**As of `c438986a` the game ships these files.** `crates/game-model/src/foundation.rs` carries
`schema.4x`, `engine.4x` and `rules.4x` with `include_str!` and builds a `Game` from them - so
they stopped being a prototype's fixture and became what a player's build runs. **Every step after
this one is built on that path.**

## The measurement

```
crates/game-model/data/foundation/rules.4x     703 lines   {rule id:1 name:move}       the game
crates/game-model/data/foundation/schema.4x    911 lines   82 lines name a game noun    the game
crates/game-model/data/foundation/engine.4x     69 lines   0 lines name a game noun     the engine
```

**`engine.4x` is the engine's own primitives** and naming no game noun is checked rather than
intended - `docs/architecture.md` rule 12. **That is what makes `H3` a split rather than a
compromise.**

## What `spec/data/` holds today, which is the part that makes room

**Its eleven files are a rendering and not a source.** They are generated from
`releases/first-release.md` by `crates/game-console`, and `C-144` measured that nothing reads them
at run time - deleting a row there changes no game and reddens a test, and the next generator run
puts it back. **When the release's tables stop being the ruleset, that rendering has no subject**,
so the directory is free exactly when the new ruleset needs it.

## What it costs you, stated so you can refuse it

**A rule in `spec/` changes by promotion.** That is the cost of `H1` and `H3` and it is not small -
today the code lane edits `rules.4x` freely, and afterwards a rule change is something you read.
**Rule 8 already draws the line and it may be the line you want**: *tuning happens in the editor
and does not touch the specification, and a tuned value becomes the default only when I say it
does.* So a number being tried costs nothing and a rule being changed costs a reading.

## What this lane would say

**`H3`.** It is what rule 8 says, and the file it leaves behind is the one the architecture already
forbids from naming anything of yours. **`H2` is the honest alternative** rather than a straw man:
the rules are read by a person either way, and if you would rather read them in the pull request
than in the queue, that is a workflow choice and not a violation of anything.

**What this lane will not do is answer it by quoting rule 8 at the code lane.** Where the ruleset
lives changes what the engine reads at run time and what `D-3` is vetted against, and that is
yours.
