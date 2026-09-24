# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-547 - Rule 4 says engine types live in *the* adapter, and four crates outside it name `bevy::`

**to** sean · **status** open · **raised** 2026-09-24 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `docs/architecture.md` -> Rules, then `spec/architecture.md`

**You asked that rule 4 be fixed if it is broken. It is broken, and the first question is which
of the two things is wrong** - the rule or the code. That one is yours, because the rule's content
is yours.

## What rule 4 says today

```
4. Engine types live only in the adapter. No `bevy::` anywhere else, including in
   the composition root's own logic - the root may assemble plugins, but it may
   not compute with engine types.
```

## What is measured

```
bevy:: in src/   what docs/architecture.md calls it
planet-bevy       8    view        "The Bevy adapter"
planet-flat      12    view        "The Bevy adapter for the flat projection"
planet-ecs        1    entities    "Game entities, as ECS entities. Contains no rules"
game4x            8    binary      "A composition root and nothing else"
game-globe        3    binding     "Binds the globe to the one game"
```

**Two of the four look like design rather than drift.** `planet-ecs` exists to hold ECS entities,
and `planet-flat` is called an adapter in the same document that says there is one. **Two look
like drift**: `game-globe`, and `game4x/src/inspect.rs`, which defines Bevy systems taking `Res`
and `ResMut` in the composition root - the one case the rule forbids in so many words.

## The three answers

**`R1` - the rule names a layer, not a crate.** *Engine types live only in the adapter layer* -
`planet-bevy`, `planet-flat` and `planet-ecs` - and nowhere above or below it. The two drifts
become defects, and a check is writable today.

**`R2` - the rule names the crates.** The list is the rule, and adding a crate to it is a
decision rather than an edit. Sharpest check; goes stale when a crate is added, which is the
point.

**`R3` - the rule keeps its wording and the code changes.** `planet-ecs` and `planet-flat` fold
into `planet-bevy`, or gain a boundary that keeps `bevy::` out of them. Truest to what is written
and by far the largest.

## What this lane would say

**`R1`.** It is the only one of the three that makes the document stop contradicting itself
without moving any code, and it leaves both real drifts red rather than legalising them.
**`R3` may still be right later** - a second engine is the test of whether the boundary was real,
and that is the doc's own words - but it is a restructuring and not a repair.

**And the ordering matters.** `P-546` puts an architecture check in `tools/spec/`. **A check
written against rule 4 before you answer this would make a rule look held while enforcing the
wrong thing**, which is worse than no check.

### P-548 - Every directory and who writes it, and three cells `CLAUDE.md` never assigns

**to** sean · **status** open · **raised** 2026-09-24 · **kind** measured · **shape** rows · **asks** a decision · **into** `CLAUDE.md` -> Perspectives

**You asked for the full list.** Here it is, built against the filesystem and asserted: every
top-level directory and every tracked root file appears exactly once.

**It asks a decision rather than approval because three cells are this lane's choice**, not
something derivable from what the file already says. They are the bold ones.

| Path                | Who writes it                                  | Where that comes from      |
| ------------------- | ---------------------------------------------- | -------------------------- |
| `spec/`             | Specification, by promotion                    | stated                     |
| `spec/data/`        | Specification, by promotion                    | stated, as `spec/`         |
| `spec/future/`      | Specification, by promotion                    | stated, as `spec/`         |
| `spec/tests/`       | Specification drafts, Sean approves            | stated                     |
| `releases/`         | Specification, by promotion                    | stated                     |
| `docs/`             | Specification                                  | stated                     |
| `docs/notes/`       | Specification                                  | stated, as Claude's        |
| `docs/postmortems/` | Specification                                  | stated, as `docs/`         |
| `docs/prototypes/`  | Specification                                  | stated, as `docs/`         |
| `docs/recipes/`     | Specification                                  | stated, as `docs/`         |
| `docs/theory/`      | Specification                                  | stated, as `docs/`         |
| `decide/`           | **Specification**                              | **nowhere**                |
| `tools/spec/`       | Specification                                  | stated                     |
| `README.md`         | Specification                                  | stated                     |
| `CLAUDE.md`         | Specification, and its columns need Sean       | stated                     |
| `crates/`           | Code                                           | stated                     |
| `web/`              | Code                                           | stated                     |
| `prototypes/`       | Code                                           | stated                     |
| `scenario/`         | Code                                           | stated                     |
| `reports/`          | Code                                           | stated                     |
| `hooks/`            | Code, production support                       | stated                     |
| `scripts/`          | Code, production support                       | stated                     |
| `.github/`          | Code, production support                       | stated, as CI              |
| `tools/anchor/`     | Code, production support                       | stated                     |
| `tools/hooks/`      | Code, production support                       | stated                     |
| `tools/outbox/`     | Code, production support                       | stated                     |
| `tools/pad-tables/` | Code, production support                       | stated                     |
| `Cargo.toml`        | Code                                           | stated, as cargo           |
| `Cargo.lock`        | Code                                           | stated, as cargo           |
| `.gitignore`        | **Code**                                       | **nowhere**                |
| `.gitattributes`    | **Code**                                       | **nowhere**                |
| `lenses/quality/`   | The quality lens                               | stated                     |
| `tools/quality/`    | The quality lens                               | stated                     |
| `lenses/research/`  | The research lens                              | stated                     |
| `tools/research/`   | The research lens                              | stated                     |
| `reviewed/`         | Nobody. The review application, acting as Sean | stated                     |
| `temporary-notes/`  | Sean, and no instance reads it uninvited       | stated                     |
| `pending.md`        | Nobody. Generated from every outbox            | stated                     |
| `target/`           | Nobody. Untracked build output                 | not mentioned, not tracked |
| `lenses/`           | Nobody at its root. Each lens writes its own   | stated, per lens           |
| `tools/`            | Nobody at its root. Each entry is owned        | stated, per entry          |
| `.git/`             | Nobody. Git's own                              | not mentioned, not tracked |
| `.idea/`            | Nobody. Ignored by `.gitignore:17`             | not mentioned, not tracked |

## The three that come from nowhere

**`decide/` has no writer.** `CLAUDE.md` names it four times and says what it is for - *it holds
what waits on a person* - and never says who may write it. **This lane has been writing it all
along**, which is the obvious reading and still a choice nobody approved.

**`.gitignore` and `.gitattributes` are not mentioned at all.** Production support covers
*`hooks/`, `scripts/`, CI, and everything in `tools/` that is not a lane's own*, and a dotfile in
the root is none of those.

**Nothing else needed a guess.** The four `docs/` subdirectories and `spec/data/`, `spec/future/`
follow from their parent, and every `tools/` entry follows from the production-support sentence or
from a lane's name.

## One thing this lane changed rather than asked about

**The Code row named `commands/`, which was deleted on 2026-09-05** - `ddbaed66` moved those files
into `scenario/commands/`, already covered by `scenario/`. **A path that names nothing grants
nothing**, so removing it changes no permission, and `CLAUDE.md` makes paths this lane's to settle
and report. Reported here.

## What it does not do

**It does not say what a lane may write into another's directory, because the answer is nothing.**
The three asymmetric rules under Perspectives already cover that and this table does not restate
them.
