# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-562 - you want the old mechanics out of your way, and three of the capabilities queued for you are evidence about them

**to** sean · **status** open · **raised** 2026-09-25 · **rewritten** 2026-09-26, around what you said rather than around the switch · **asks** a decision · **kind** entailed · **into** `releases/first-release.md`

**Sean, 2026-09-26**: *the old mechanics are put somewhere out of this release so that I can focus
on the tests i reviewed.* **Three of the seven capabilities waiting on you are evidence about
those mechanics.**

```
W1  retire R-6, R-7, R-8       you never read them. They assert that mechanics being
                               deleted worked, which stops being worth your attention
                               the moment the deletion is decided - and it is
W2  read them, then retire     the work is done and reported; reading it is an hour
                               and closes the first release properly
W3  leave them open            they go stale when the switch happens and are re-run
                               against the new model, which is `W2` at a later date
```

## The four that are not about the old mechanics, and are unaffected either way

```
R-9   reports browsable, no script      a property of the report's form
R-10  a drawing in either theme         a property of the drawing's form
R-11  the engine's inputs are reachable the set of inputs moves; the claim does not
R-12  the foundation form of a test     generated from reviewed/ - the tests you read
```

**`R-12` is the one that is already about what you want**, and `R-9` and `R-10` are about the
reports rather than about any ruleset. **So this question is about three capabilities, not seven**,
and the other four are worth your eye whatever you answer.

## Why `R-6`, `R-7` and `R-8` are the three

**Their evidence is produced by running the mechanics being replaced.** Re-derived:
`tests/expected_state.rs` writes `scenario/expected/play.4x` by constructing a `game_model::Game`,
and `worked.rs`, which `reports/recipes.md` is built from, does the same. `R-8`'s signatures come
from the same release tables.

**And the scenario is older than the thing replacing it**, which is the question you asked:

```
2026-09-05  scenario/expected/play.4x     366 lines of commands, 133 of expected state
2026-09-14  thin-engine begins
2026-09-21  spec/tests/ and reviewed/ arrive
2026-09-22  the review application
```

**Nine days before thin-engine and sixteen before the review process.** No file in `reviewed/`
mentions the scenario, so the two suites do not overlap at all.

## What this lane would say

**`W1`.** `docs/process.md` makes vetting the thing that gates *finishing* rather than shipping, and
a capability whose subject is being deleted has nothing left to finish. **Reading the three would
tell you that mechanics you are removing worked**, which is true and is not worth an hour of the
one attention this process is built to conserve.

**What `W1` costs, and it is the honest objection**: `R-6` is *the loop can be played through*,
which is the closest thing to a statement that the game works at all. Retiring it unread means
nothing has ever confirmed that by a person's eye - and the new model owes the same observation
before it can claim it. **This lane would file that as a capability of the new release rather than
keep the old one open**, but it is your call whether that is a deferral you accept.

## What is not in question

**Nothing here retires a test you reviewed.** The 54 in `reviewed/` are untouched, and `D-2` is
what makes them the game's.

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
