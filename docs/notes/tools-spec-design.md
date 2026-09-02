# `tools/spec`, designed

**Derived, 2026-09-02.** Written by Claude after Sean approved porting the specification lane's
editing script to Rust and called the cross-lane dependency justified. Not binding, and **nothing is
built** - `P-182` has to land first, since `tools/spec/` is not yet anyone's to write.

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
