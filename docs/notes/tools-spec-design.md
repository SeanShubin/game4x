# `tools/spec`, designed

**Derived, 2026-09-02. Part of it is built, 2026-09-11** - see *What is built* at the bottom, which
is the only part of this note that is not a design. Written by Claude after Sean approved porting the
specification lane's editing script to Rust and called the cross-lane dependency justified. Not
binding.


[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## Why it is a redesign and not a translation

`edit.py` is 193 lines whose power is that `transform` is **arbitrary Python written per
promotion**. A Rust tool cannot take a lambda, so it needs a fixed vocabulary of operations.

**That constraint is the whole reason to do it.** Two of `2026-09-01`'s defects were a hand-written
transform doing something the author did not intend - a cell located by counting, a `str.replace`
that silently matched nothing. **A named operation makes both unrepresentable rather than checked.**

## The verbs

Derived from every edit actually made on 2026-09-01, not from imagination. **If an edit needed a verb
that is not here, the verb is added before the edit is made** - which is the point, not the cost.

| Verb                                                          | What it does                                                     |
| ------------------------------------------------------------- | ---------------------------------------------------------------- |
| `promote <id>`                                                | **the one that matters** - see below                             |
| `replace-once <file> <old> <new>`                             | prose, failing unless exactly one match                          |
| `insert-after <file> <anchor> <text>`                         | prose, failing if the text is already present                    |
| `set-cell <file> <section> <row> <column> <value> [--expect]` | a cell, located by header name                                   |
| `set-row` / `insert-rows-after` / `delete-rows`               | table rows, located by a one-column prefix                       |
| `replace-rows <file> <section> <name>`                        | a contiguous block of rows, asserting it is contiguous           |
| `add-column <file> <section> --after <column>`                | every row, or it fails                                           |
| `accept <id> <row>`                                           | a proposal out of the queue and into the ledger                  |
| `reorder <ids...>`                                            | the open queue, asserting the set is unchanged and the bytes are |
| `commit <paths> -m <message> [--claim ...]`                   | with yesterday's two triggers                                    |

**Always, on every operation**: every table's rows are uniform width, no paragraph repeats in an
edited section, and no open proposal addressed `to sean` lacks a verbatim block.

## `promote` is the reason to build it

The quality lens found the class the guards did not cover: **every check compares the edit to what
the script intended, and nothing compares the intent to what Sean approved.** After a promotion the
approved text is retained nowhere, so the queue's one guarantee is unverifiable at the moment it is
asserted.

**`promote <id>` closes that by construction.** It reads the proposal, extracts the text between the
directive and `**Basis**`, applies it to the destination the proposal names, **asserts the approved
text appears once in the destination**, moves the item to the ledger, and commits. **Nothing between
the approval and the file is written by hand**, so there is no intent to diverge.

**It does not replace `S-10`**, which checks promotions from git afterwards. **This makes the common
path safe; that catches the path taken around it**, because a tool cannot enforce that it is used -
three of `2026-09-01`'s eleven defects were commits chained after an ad-hoc script, which is `spec/`
being edited outside the guards precisely when something went wrong.

**Corrected 2026-09-02: this note said quality argued they were alternatives, and quality did not.**
They wrote that there was a real argument against building it yet and that the call was this lane's.
**Strengthening a position nobody held and then beating it is not an argument**, and it is worth
recording because it is a way of being wrong that reads as rigour.

**And the destination is a hand-copy that nothing checks.** A proposal's `**into**` field is
structured; the ledger row's destination is prose **typed by this lane from that field**, and the
field is deleted with the body at promotion. So `promote` should write the ledger row **from the
field** rather than take it as an argument - which closes a transcription step of the same family as
everything else this week.

## The dependency, which Sean has called justified

`tools/outbox` already parses `docs/notes/proposals.md` in 1,022 lines - `Item`, `parse`, `accepted`,
`Landed`. **`tools/spec` depends on it as a path dependency rather than parsing the file twice.**

**It is the first dependency across a column boundary**, and the hazard is not the direction but the
divergence: **two parsers that disagree about what a proposal is would be worse than either.**

**One thing the code lane has to agree to**, and it is small: `outbox`'s parser exposes fields and
the ledger, and `promote` also needs **the proposed text** - the blockquote before `**Basis**`.
Either `outbox` exposes it, or `tools/spec` extracts it and the two disagree about where a proposal's
body ends. **The first is right and it is their call.**

## What it does not change

**`tools/pad-tables` stays where it is and keeps writing the same files.** Sean: *there is no overlap
in how the files are changed - I expect it to work in either order but not concurrently.* **The
constraint is concurrency**, and it already binds on three instances sharing an index.

## The ledger's destination is not the `into` field, measured 2026-09-02

**`S-10` asks that `promote` write the ledger row's destination from the `**into**` field**, so the
thing the check reads was never transcribed. Measured against the promotions that can be checked
reliably, that would make the ledger **less** accurate, not more.

Eight of the day's rows resolve to the commit that really added them. **Seven agree with their
proposal's `into`. One differs, and the ledger is the correct one**: `P-192` said `-> Kinds` and
landed in `Kinds` **and** `Traits`, because declaring two more kinds forced the `kind` trait's values
from *one of the ten* to *one of the twelve*. That consequence was found while promoting, which is
after the field was written.

**So the field is what was intended and the row is what happened, and a promotion that discovers a
consequence makes them differ legitimately.** `promote` should therefore **offer** the `into` field
as the row's default and require the promoter to say when it landed elsewhere - so a difference is
declared rather than typed, and an undeclared difference is the error.

**The other seven rows could not be checked and the reason is worth keeping.** `git log -S` on a
ledger row finds the commit where the padder last rewrote that table, not the commit that added the
row - **every row looks added whenever a column widens.** That is the table hazard `CLAUDE.md`
warns about, arriving in a measurement rather than an edit, and it means any check built on *the
commit that added this row* is reading the wrong parent most of the time. `promote` writing the row
avoids the question entirely.


## What is built, 2026-09-11

**Three verbs and the module under them**, built after this lane made the same class of mistake
three times in one session - a structural markdown edit, hand-rolled, with the boundary or the
newline decided anew each time.

| Verb                            | What it does                                                         |
| ------------------------------- | -------------------------------------------------------------------- |
| `spec show <id>`                | the approved text, from `outbox`'s parser, with its quoting stripped |
| `spec after <id> <file> <line>` | puts it in after that line, then **reads the file back** and asserts |
| `spec land <id> <after-id>`     | a ledger row **from the item's own `into` field**, then removes it   |

**Each is a defect that happened.** The boundary between one item and the next is the next `###`
heading or one of the file's named sections - never a `##`, because a proposal's body uses `##` for
its own sub-headings, and cutting there left sixty-eight lines of two proposals in the `Open`
section **after the same defect had been found and fixed two hours earlier**. Every write goes
through one function that writes `\n`, because the default put six carriage returns into
`spec/invariants.md`. Text is inserted line by line rather than pasted, because a replacement ending
in a newline after an anchor ending in a newline gives a blank line, which had been repaired by hand
after nearly every edit that day.

**The assertion reads the file back from disk.** That is the whole difference between this and what
it replaces: the promotion that failed on 2026-09-11 asserted its destination against the
intermediate file the script had just written, so a truncated sentence passed. `docs/process.md`
now carries the rule - *a check that reads a copy of the population is checking the copy.*

**Verified by breaking rather than by passing.** Debris in the `Open` section, a carriage return in
`spec/control.md`, and the boundary changed back to cutting at any `##` each turn a test red; all
three went green again when undone. The integration tests read `docs/notes/proposals.md` itself
rather than a fixture, and each says what it counted over.

**What is not built**: `replace-once`, `set-cell`, `replace-rows`, `reorder` and `commit`, and
`promote` as one composite. `after` and `land` are the two halves of `promote` that this session
actually needed.

**And one thing blocks the rest**, filed as `S-103`: `Item::proposed_text()` returns an error when a
proposal carries more than one blockquote, and a proposal with two destinations carries two -
`P-425` and `P-431` both did. Until `outbox` exposes the blocks rather than the block, `show` and
`after` handle only single-block proposals, and a two-block promotion is still done by hand. **That
is `outbox`'s call and not this lane's**, which is what this note said in 2026-09-02 and is still
right.

## What was added on 2026-09-12, and what found it

**`spec replacing <id> <file> <old>`**, because **every promotion on 2026-09-11 and 2026-09-12 was a
replacement and there was no verb for it.** Each went through `tools/anchor` with a hand-written
replacement file, and both of the two days' hygiene slips were in that path - a trailing newline kept
twice, leaving a blank line to find by reading the diff.

**`old` is matched with whitespace collapsed on both sides, over a run of any length.** A sentence
offered on one line and wrapped onto two in the file is the case the verb exists for, and the first
implementation could not do it: it matched a run of exactly as many lines as the pattern had.
**A test caught that before the verb was used once.**

**`land` now says so when it empties the queue.** `## Open` followed by nothing reads as *this has
not been looked at*; `*Nothing is open. Everything filed has been decided.*` reads as *there is
nothing here*, and only the second is a claim. **The integration test found this on the live file
after `P-434` landed** - the first defect this lane has had reported to it by a check rather than by
its own re-reading.

**Three of the four verbs now refuse rather than guess**, and every refusal is a defect that
happened: two runs matching, two ledger rows with the same prefix, a block whose range holds another
item, a replacement carrying a carriage return.
