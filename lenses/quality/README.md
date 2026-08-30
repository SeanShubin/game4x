# Quality

**Derived.** Written by the quality instance. Not binding - a report is an observation about the
code, not a decision about it. Sean decides what is acted on, and the code instance acts.

[Root README](../../README.md) · [Architecture](../../docs/architecture.md) · [Specification](../../spec/README.md)

Code quality reports. This directory is the quality lane's only writable place, and no other lane
writes here - see [the lane table](../../CLAUDE.md#perspectives).

## The rule that makes a report worth reading

**Quality never edits what it reviews.** It reads the tree, runs read-only tools, and writes here.
It does not fix the thing it found, does not reformat, and does not run `cargo fmt`, `cargo fix` or
`clippy --fix` - a review that alters its subject is no longer a review, and the next report would
be measuring its own last one.

The consequence worth stating plainly: **a report is only useful if someone acts on it.** A finding
that is true, well-argued and never acted on is indistinguishable from one that was never written.
So a report names what to do, not merely what is wrong.

## The outbox

**[`outbox.md`](outbox.md) is the only file another perspective has to read.** A report carries the
argument; the outbox carries the claim, its reader and its state. Each item has four things:

| Field      | What it is                                                                                                                                                |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **id**     | `Q-n`, stable, so a commit can cite it and a later report can say what became of it                                                                       |
| **to**     | `sean`, `spec`, `code` - or absent, meaning *not ready, no reader*                                                                                        |
| **status** | `open`, `noted`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` is outstanding; `noted` is *recorded so it is not re-found*, which is terminal |
| one line   | what it is, so a reader can triage without opening the report                                                                                             |

**Research that is not ready is addressed to nobody.** It lives in a dated report, costs no one any
attention, and becomes visible the moment this file gives it a reader. Nothing is held back by
discipline; it is held back by not yet having one.

**A producer's backlog is not Sean's queue.** Items here are addressed to a producer and cost that
producer, not Sean. `CLAUDE.md`'s limit of fifteen is on the *proposal* queue, justified by his
reading time; counting a lens's findings against it was this lens's error and is
[corrected](2026-08-30-two-budgets.md). **This file's length is not a number to report to Sean.**

**If nothing in the outbox is `open`, this lens knows of nothing outstanding.** That is a promise
about the file, not about the tree.

**A producer may decline a finding, and often should.** It says so in the commit that declines it,
citing the id, and the outbox records it. `Q-16` was wrong and was withdrawn after the code lane
refused it - and the refutation produced better output than the finding had. **Check a rejection
before defending it.**

## What a report says

Every finding carries four things, because a finding missing any of them cannot be acted on without
going back to whoever wrote it:

|             |                                                                          |
| ----------- | ------------------------------------------------------------------------ |
| **Where**   | file and line, so it can be found without searching                      |
| **What**    | the defect, in one sentence                                              |
| **Why**     | what it costs - a bug, a trap for the next reader, a rule it breaks      |
| **Whether** | worth fixing now, worth fixing eventually, or noted and deliberately not |

That last column is the one that keeps a report from becoming a wish list. **Most findings should be
"noted and not".** A report where everything matters is a report where nothing does.

## Say it and stop

**State the question, the facts, and what to do. Then stop.** Sean read a sixty-line proposal that
said a generated file may live in the repository root and called it esoteric and vacuous. He was
right, and the specification lane has written a rule against itself about it.

This lens shares the failure mode and should say so. A report is long when the argument is long, not
when the finding is small dressed up. Two tests before filing:

- **Would the finding survive being cut to its Where, What, Why and Whether?** If the rest is
  reasoning nobody has to follow to act, it belongs in a dated report or nowhere.
- **Is a sentence carrying its weight, or is it a phrase that sounds settled?** An aphorism that
  compresses a real finding earns its place. One that decorates a small one costs a reader's trust
  in every other line.

It is the same idea as competing on the value of a finding rather than the count, one step out: a
lens spends a reader's attention by the word as well as by the item.

## What is in scope

The tree, its structure, and whether the code says what the specification says. Concretely: whether
[architecture's rules](../../docs/architecture.md#rules) hold, whether crate boundaries are real,
whether tests assert what they claim, whether names mean one thing, and whether anything in
`crates/` contradicts anything in `spec/`.

**A contradiction with the specification is the highest-value finding**, because neither of the other
two lanes is looking for it: the code instance reads the spec as instructions and the documentation
instance does not read the code.

## What is not

Style the formatter already settles, preferences with no argument behind them, and anything that
would be a design decision rather than an observation. **When a report finds that the specification
itself is wrong or unclear, it says so and stops** - that becomes a proposal in
[the documentation lane](../../docs/notes/proposals.md), not a change here.

## Naming

One file per report, dated: `2026-08-28-crate-boundaries.md`. Reports are records of a moment and go
stale like any note - a superseded one says so at the top rather than being deleted, so a later
reader can tell whether a finding was fixed or merely forgotten.

## Reports

Newest first.

- [Two budgets, counted as one — a correction](2026-08-30-two-budgets.md)
  - 2026-08-30. Why the queue is empty and the counter said fifteen. This lens's error,
    and the one generated document that should replace a remembered command.
- [Who checks the specification is buildable, and where Sean looks](2026-08-30-readiness-and-one-surface.md)
  - 2026-08-30. Two recommendations, both against adding machinery: no readiness lens, and
    no `to sean` address.
- [The workflow, ready to paste](2026-08-30-workflow-to-adopt.md) — **spent**
  - 2026-08-30. The `CLAUDE.md` section and the index tool, written out verbatim for the
    two lanes that own the files they go in.
- [A recommended workflow](2026-08-29-workflow.md)
  - 2026-08-29. **Input to a decision, not a finding.** Seven things move between the
    perspectives, six have a channel, and a blocked question from code has none.
- [Notes on lenses, from the one that exists](2026-08-29-lenses.md)
  - 2026-08-29. **Input to a decision, not a finding.** What three reports suggest about
    several research lenses, and what has to change before there is a second one.
- [Who reads a report](2026-08-29-who-reads-a-report.md)
  - 2026-08-29. Both lanes must read a report, nothing says so, and a known spec
    contradiction is sitting outside the queue that promises to hold it.
- [What the new prototype exposed, and what it did not](2026-08-29-coupling-under-the-game.md)
  - 2026-08-29. `Biome` in the game pulls terrain and rendering up into it, the picture and the
    model disagree about a territory's biome, and a detached globe still links the whole game.
    Finding 1 is withdrawn after the code lane's reply; the correction is in the report.
- [What the response to the first report left behind](2026-08-28-response-to-the-first-report.md)
  - 2026-08-28. Five findings closed and verified; three quotations the P-95 sweep missed.
- [Crate boundaries, duplication, and where Bevy has spread](2026-08-28-crate-boundaries-and-duplication.md)
  - 2026-08-28. Whether Bevy is confined to the adapter, what is duplicated, and four places the
    code and the specification disagree. Findings 1, 2, 3, 4 and 13 are closed; the rest stand.
