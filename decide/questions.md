# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

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
