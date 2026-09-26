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
