# What the proposal limit has actually done

**Derived.** Written by Claude, 2026-09-06, from a question Sean asked: *what does the limit
actually do?* Not binding. The decision it fed is `P-301`.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## The measurement

Across every commit that has touched [`proposals.md`](proposals.md), counting the `### P-n`
headings inside the `## Open` section:

|                             |                                           |
| --------------------------- | ----------------------------------------- |
| Commits examined            | 448                                       |
| Peak open queue             | **12** - 2026-08-25, and again 2026-09-06 |
| Commits at or above 15      | **0**                                     |
| Commits at or above 12      | 4                                         |
| Commits with an empty queue | 135, a little under a third               |

Maximum depth per day:

```
2026-08-25  12    2026-09-01   7
2026-08-26   8    2026-09-02   7
2026-08-28   7    2026-09-03   6
2026-08-29   5    2026-09-04   8
2026-08-30   3    2026-09-05   5
2026-08-31   5    2026-09-06  12
```

## How to re-run it

For each commit in `git log --format=%H --reverse -- docs/notes/proposals.md`, take
`git show <hash>:docs/notes/proposals.md`, slice from `## Open` to the next top-level heading that
is not `## Open`, and count lines matching `^### P-\d+ `. The two peaks are the first day, when the
queue was seeded, and the day Sean redefined the four instances.

**One thing the method gets right and a simpler one would not.** Counting `### P-` across the whole
file would count the ledger and the withdrawn items too, which is the wrong population - the limit
is about what is *open*. The slice is what makes the number mean what the limit means.

## What it says

**The limit has never bound.** In 448 commits it has never stopped a proposal being filed, so the
number fifteen has never been tested. Ten would have produced the same history.

**What held items back was judgement below the number.** On 2026-09-06 this lane held `Q-59` and the
Traits-table rule at twelve, citing queue length. The cap did not bind; awareness of a budget did.

**So an unreached limit is not evidence the limit is unnecessary**, and it is not evidence it is
well-calibrated either - nothing here distinguishes those. What the measurement rules out is the
*stated* justification: fifteen was written as a bound on Sean's reading, and his reading has never
come near it.

Sean's conclusion, 2026-09-06: keep a number and change what it is for. It is a tripwire on the
specification instance - if the queue reaches fifteen, proposals are being filed faster than
decisions are being made, which means guessing at design, and the remedy already written in
`CLAUDE.md` is to ask one question instead. **Its job is never to be reached.**

## What it does not cover

**Contradictory proposals**, which a count does nothing about. The instrument for those is the
re-read trigger on a section that has taken a second proposal, which fires on 38 sections as of this
date. Sean identified this as a separate problem before the measurement was run, and it is.
