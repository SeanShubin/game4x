# A citation that resolves and is wrong

**Derived.** Written by Claude, 2026-09-12, from two instances in one evening, one in each producing
lane. Not binding - see [the specification](../../spec/README.md) for what was decided.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

**Both were written from confidence rather than read from a source**, and both sat in sentences
where every other fact had been measured.

## The two

| Lane | What it said                                                   | What was true                            | Corrected by |
| ---- | -------------------------------------------------------------- | ---------------------------------------- | ------------ |
| spec | a test run happened on `9e8d7e1`'s tree                        | no such object has ever existed          | `81cce0e`    |
| code | `P-478` took the Traits table from 24 rows to 26, in 24 places | `P-476` did; `P-478` moved no row at all | `2a0afca`    |

**The spec lane's was one citation and the code lane's was twenty-four**, across ten files, all the
same wrong id. One came from typing a plausible short hash; the other from building three
promotions in one pass, where the id that reached the comment was the last one typed rather than
the one that caused the change.

## What the checks do and do not do

**`tools/spec` and `tools/outbox` both fail when a `**cited**` field names no commit.** That catches
the first: a hash that never existed cannot resolve. **Nothing catches the second, because `P-478`
resolves** - it is a real proposal with a real ledger row, and the row is about renaming words.

**So the carrier checks that a citation resolves, not that it resolves to the thing it is about.**
That is `P-245`'s wall said about citations: no check can ask whether another check's predicate is
about its subject, and *is this the proposal that caused this change* is exactly that question.

## Why the second is the more expensive one

**A hash that has stopped existing looks exactly like one that still does**, which is why
`CLAUDE.md` forbids amending - and it is checkable, so the check is the answer.

**A wrong id sends a reader somewhere.** They open `P-478`'s ledger row, find a rename, and either
conclude the comment is stale or conclude they have misread the code. **One wrong citation is one
reader sent to the wrong commit; twenty-four is a reader sent there from wherever they enter.**
That was the code lane's own reason for checking all twenty-four rather than the one pointed at.

## What actually caught both

**A reader re-deriving a claim they had been handed.** Neither was found by a check, and neither
would have been - the spec lane's was found by running `git cat-file -t` on its own sentence after
writing it, and the code lane's by the spec lane reading one comment and asking which proposal
moved the rows.

**Two of the evening's corrections were also inferences that arrived beside measurements.** The
code lane's first thought about a third stale literal was *an unreached branch*, and it was an
unreached **line** - close enough to feel confirmed, different enough to fix wrongly. It said so
rather than acting on it, which is the habit `CLAUDE.md` records as *a measurement travels with an
explanation of itself, which is not measured*.

## Sources

`81cce0e` and `2a0afca`, and the messages between the two sessions on the evening of 2026-09-12.
The counts here were re-derived rather than taken: after `2a0afca` there are no `P-478` references
in `crates/`, `prototypes/` or `web/`, and ten files under `crates/` cite `P-476`.
