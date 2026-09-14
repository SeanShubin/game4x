# Tracked and still lost

**Derived.** Written by the specification lane, 2026-09-13, at Sean's request, after the code lane
rediscovered by measurement a gap that three outbox items had already been carrying. Not binding -
see [the specification](../../spec/README.md) for what was decided.

[Postmortems](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What was supposed to be true

`spec/invariants.md` -> *The game is data* says, and has said since 2026-08-31:

- *Every kind of thing, and every recipe that turns some things into others, is data rather than
  code*
- *The data that runs the game lives in a data file. Not a presentation file such as markdown or
  HTML, and not a programming language file such as Rust*
- *Nothing in the state is special to a kind. Adding a kind adds no field and no case*

## What actually happened

Every figure below is read from the tree or from `git` rather than recalled.

| When       | Commit    | What                                                                                                                                                                                         |
| ---------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-08-31 | `8124c39` | Sean lands *The game is data*. `spec/data/` **does not exist**; `game-model/src` is **2,555** lines                                                                                          |
| 2026-09-04 | `0dae073` | `S-30` raised - *the release's eight data tables have no data file to be generated from*                                                                                                     |
| 2026-09-06 | `0ba023f` | `C-16` - *the invariant has two halves and only one is kept* - closed **acted**, explicitly not dropped: *the gap is unchanged and is not being dropped. `S-30` is the item that carries it* |
| 2026-09-12 | `f30f16c` | The first data file lands. `spec/data/` exists at last, at **58** lines                                                                                                                      |
| 2026-09-12 | `bc7fce8` | `S-30` **withdrawn** - *nothing is lost by this withdrawal* - live half handed to `C-102`                                                                                                    |
| 2026-09-13 | `e43c36b` | `C-114` measures from scratch: recipes read at run time, **zero**. `game-model/src` is **6,231** lines                                                                                       |

**Ten days, three items, no gap in the chain, and the thing was lost anyway.**

## Why nobody noticed

**Each handoff carried a narrower question than the one before it, and every hop was honest.**

| Item    | What it carried                                                                                           |
| ------- | --------------------------------------------------------------------------------------------------------- |
| `C-16`  | kinds, families, traits, **recipes and costs** are hand-written Rust where the invariant says a data file |
| `S-30`  | the release's **eight data tables** have no data file to be generated from                                |
| `C-102` | **which of four remaining tables** is data and which is a relationship                                    |

*Recipes are executed as match arms* is in the first and in neither of the others. It fell out on
2026-09-06, and no later step could recover it, **because each step checked itself only against its
immediate predecessor.** `S-30` genuinely had gone stale. The four data files genuinely had landed.
`C-102` genuinely is the live half **of `S-30`**. Nobody dropped anything, and the thing was dropped.

**And the number being tracked improved while the violation got worse.** What the chain counted was
*tables with a data file*: seven of eight missing became four of eight. Over the same twelve days
the engine went **2,555 -> 6,231 lines**, +3,676, and the data arrived at **58**. The metric moved
toward the goal; the subject of the invariant moved a hundred and forty-four percent away from it.

**A rule about the end state has no reading before the end.** `spec/` is the destination and
`releases/` is what is being built now, so an invariant the code does not satisfy is the normal
case - `CLAUDE.md` records nine of thirteen proposals in one week describing a rule system nothing
could name a condition for yet. **So *not yet built* and *being actively violated* are the same
bytes**, and drift away from an invariant looks exactly like not having arrived at it.

That is the twin of the failure `CLAUDE.md` already names - *the instrument answers a narrower
question than the one asked, and returns a plausible number rather than an error*. Every recorded
instance of that is a **check**. This one is a **chain of outbox items**, where the item is the
instrument and the narrowing happens at the handoff.

## What would have caught it

**Nothing that exists, and this is the section to be honest in.**

- **No check does.** `tools/outbox` reads ids, addressees and statuses; the narrowing is in prose.
  A check would have to compare the scope of two English sentences, which is `P-245`'s wall - *no
  check can ask whether another check's predicate is about its subject*
- **The gate does not.** It is green and was green throughout
- **A person re-measuring did**, which is what happened, at the cost of rediscovering ten days of
  work from scratch

**One thing is mechanical and would have surfaced it**: a closed item that names a still-open item
as carrying what it dropped is a chain, and a chain two hops long is worth a human look. Run over
every outbox as it stands, exactly one such chain exists - `C-16` -> `S-30` -> `C-102` - and it is
this one. **That is a detector rather than a check**: it says where to look and cannot say whether
the successor is narrower.

**It is also fragile, and the fragility was measured while writing this.** Three instruments
written for this postmortem each returned a plausible answer that was wrong: one collected
successor ids per paragraph and missed `S-30`'s, which sits under a heading; one used an `awk`
range whose end pattern matched its own start line and returned nothing at all; one counted any id
mentioned anywhere in a body as a handoff and produced two false edges - `C-30` names `S-30` inside
a table of counts, `C-87` in passing. **Every one was found by deriving the answer a second way,
and none by a check.**

## What did work, and it is not a habit

**The reason reached the invariant.** Sean, 2026-09-13, promoted as `P-493`:

> A thin engine running a data driven game forces inadequacies in the engine and data structure to
> come to light sooner. The design pressure is the whole point. If the code explodes in complexity,
> or the data structure explodes in complexity, or the data itself explodes in complexity, that
> tells us something needs to be unified or redesigned more clearly than anything else could.

**An aspiration has no reading and an instrument has one at every moment.** *The game is data* could
not be violated by a number; *complexity is the reading* can be, and the ratio it reads - 6,231
against 58 - was off the end of its scale with nobody looking. **This is where to look, not a habit
that has caught anything**: it was written from the incident it explains, and by `CLAUDE.md`'s own
rule that means it could not have come out otherwise.
