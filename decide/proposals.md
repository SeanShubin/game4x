# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-563 - `H3`: the game's rules and kinds move to `spec/data/`, and the engine's primitives stay

**to** sean · **status** open · **raised** 2026-09-26 · **answered** 2026-09-26, `H3` · **asks** approval · **kind** entailed · **shape** an instruction · **into** `spec/data/`, `crates/game-model/data/foundation/`, and `spec/README.md` rule 8

**You said `H3`.** This is the same item asking approval, because an answer to a question is not a
promotion. **It is an instruction rather than text** - what you are approving is a move and the
check that says it happened, not words that land in a file.

## The instruction

**Move `rules.4x` and `schema.4x` from `crates/game-model/data/foundation/` into `spec/data/`, and
leave `engine.4x` where it is.** The engine reads all three by `include_str!`, so the paths in
`crates/game-model/src/foundation.rs` follow the files.

**And empty `spec/data/` of what is there now.** Its eleven files are a rendering generated from
`releases/first-release.md`; they are the old ruleset, and `D-4` deletes them. **They go in the
same move rather than being overwritten in place**, so that nothing is left half of each.

## How to tell it was carried out

```
spec/data/rules.4x, spec/data/schema.4x      exist, byte-identical to what moved
spec/data/                                    holds nothing generated from the release
crates/game-model/data/foundation/engine.4x  still there
crates/game-model/data/foundation/            holds no file naming a game noun
the game                                      still builds and plays from them
```

**The last line is the one that matters** - a move that left the engine unable to read its own
ruleset would satisfy the first four.

## What this does not settle, and it is filed rather than folded in

**Rule 8 says the game's data goes in `spec/data/` and does not say who may write it.** Today the
code lane edits `rules.4x` freely. After this it is in your directory, and `CLAUDE.md` says
`spec/` is written by promotion. **This lane will file that as its own question** rather than
decide it here: whether a rule change is a promotion, or whether `spec/data/` is the exception
rule 8's own *tuning happens in the editor* already hints at.

**It is not urgent and it does not gate the move.** Nothing breaks while it is open; the code lane
carries on as it does today until you say otherwise.

### P-566 - `D-5` says two territories are taken and you said both are developed

**to** sean · **status** open · **raised** 2026-09-26 · **asks** approval · **kind** recovered · **shape** text · **into** `releases/rules-become-data.md` -> `D-5`

**Your words, 2026-09-26**: *ark deploys - develop first territory - develop second territory -
launch from second territory.* **`D-5` as promoted says *taken* where you said *developed***, and
the difference is a scenario that walks to a second territory and launches without building
anything.

**`D-5`'s other clause does not close it.** *Every rule the reviewed tests describe fires at least
once* forces building to happen **somewhere**, not in both places. Your sentence says both.

## The words

**Replacing `D-5`'s *vetted when* line:**

> - **Vetted when** - a main scenario exists over the reviewed ruleset and I have watched it run:
>   an Ark deploys, **that first territory is developed**, a second is taken by land and
>   **developed too**, and an Ark launches from the second. **Every rule the reviewed tests
>   describe fires at least once while it runs**, measured by what fired rather than by what the
>   file says. This is the observation `R-6` was retired without making

## What this does not bring back

**Not the fully exploited planet.** You recalled `R-6`'s old target and it did exist - it asked
for a fully exploited planet until `P-422` changed it on 2026-09-12. **This is the smaller thing
you described**, two territories developed, and it does not restore the old requirement.

### P-565 - rule 3 leans on the difference between `spec/tests/` and `reviewed/` without ever stating it

**to** sean · **status** open · **raised** 2026-09-26 · **answered** 2026-09-26, the code settles it · **asks** approval · **kind** recovered · **shape** text · **into** `spec/README.md` rule 3, after *the rendering is generated from `reviewed/` and never from `spec/tests/`*

**The choice this item offered is withdrawn, because the code answers it.** `review-web.rs` reads
`report::tests_at()`, which is `spec/tests/`, and writes `report::records_at()`, which is
`reviewed/`. **Deleting `spec/tests/` would leave the review application nothing to press `r` on**,
so `T1` was never available and this lane should not have offered it.

```
spec/tests/<name>.4x    the test, written by this lane
       |  you press `r`
reviewed/<name>.4x      a verbatim copy - "I have read this"
```

**`u` deletes the copy and `x` files a note in `reviewed/asked.md`. Three writes and nothing
else.**

## Why it belongs in rule 3 rather than being left to the code

**Rule 3 already leans on the distinction in its own last sentence** - *the rendering is generated
from `reviewed/` and never from `spec/tests/`* - and never says what the two are. **A reader of
`spec/` learns it only by reading `review-web.rs`.**

## The words

**One paragraph, into rule 3, after *so that what the engine runs is derived from what has been
read rather than compared with it*:**

> **A test is written in `spec/tests/`, and a copy of it in `reviewed/` is the record that I have
> read it.** The review application makes that copy and removes it, acting as me; nothing else
> puts a file there. **So the two directories hold the same tests only while I have read every
> one**, and a test nobody has read is in the first and not the second - which is what makes
> generating from `reviewed/` mean something.

## One thing this does not fix, and it is not stale yet

**Rule 3 also says the data is *locked by the scenario test*, and `D-4` deletes that scenario.**
This lane told you promoting `P-564` makes that clause false. **That was overstated**: `spec/`
describes the end state, `D-5` puts a main scenario back, and the clause is true again when it
does. **What is genuinely unsettled is narrower** - `D-5` says you watch the scenario run, and says
nothing about an automated test asserting its expected state, which is what *locked* means today.
**Filed rather than folded in**, because it is about what `D-5` should require and not about these
two directories.
