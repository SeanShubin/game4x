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

## What this lens is for, and what bounds it

[`docs/process.md`](../../docs/process.md) is Sean's own statement and is authoritative; this file is
detail beneath it and must not contradict it. It gives this lens three jobs:

- a proper module structure emanating from composition roots
- the widest separation between generic code and code with dependencies
- **a dependency that provides a home rather than operations confined to one crate**

**Eight items open to any one instance, and close or withdraw before filing a ninth.** Not a
borrowed number: about two days of a producer's throughput. It is deliberately where the judgement
already is - most of what this lens notices is meant to be `noted`, and a cap is what makes that
cost something rather than being a good intention.

### The test this lens was missing

`docs/process.md` → Dependencies states it better than anything here did. **A dependency provides
operations or it provides a home. Operations are functions over data you already had; a home decides
where the data lives and when the code runs. The test is whether it appears in your own types: an
operation never does, and a home cannot avoid it.**

That is a sharper instrument than the reasoning this lens actually used, and it would have reached
two findings faster:

- `Q-2` - `Biome` was `game-model`'s, and it appeared in `planet_terrain::Sample`'s surface. A
  fact about the game had become a fact about the terrain crate's types.
- `Q-3` - Bevy is a home, and it had two crates to have opinions in rather than one. The rule names
  the defect directly; this lens got there by counting what a prototype linked.

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

## A pattern is a claim about the bytes as they are now

Two failures that look alike and are not, both of which cost this lens a wrong conclusion this
session.

**A match string is a claim about the current bytes.** A table row written before the padder ran, a
grep for `fn window()` against a signature that had since gained an argument. The remedy is to
locate by prefix and rebuild rather than match, and to read the file at the moment of matching
rather than earlier.

**A command answers the question it was given, not the one in mind.** `cargo tree --depth 1` read as
the tree; `git diff` read as HEAD against the working tree when it compares the index. The remedy is
different: check what the command scopes or compares *before* drawing anything from it.

Its sharpest form, because it reads as evidence and is not: **a count of zero says nothing unless
the population is non-empty.** `grep -c "into" docs/notes/proposals.md` returned 0 and this lens
reported the field did not exist. The field exists on every open proposal; the *queue* was empty, and
promotion deletes the field with the body. The number was right, the denominator was zero, and the
conclusion happened to survive for a reason other than the one given.

The code lane grouped all four as trusting a tool's output. That is right about the cause and blurs
the remedy - the first family is fixed by when you read, the second by knowing what you ran.

## Cite the commit, rest the claim on the file

**A closure here names a commit. The claim rests on file state, and must keep resting there.**
`C-4`: the index is shared between three sessions, so `git commit` publishes whatever is staged
rather than what the caller changed. A commit can carry work it does not mention.

It has happened once, to this lens. `93d839d` is titled *finding: Q-8 acted* and carries twenty-six
lines of `hooks/pre-commit` - the code lane's `Q-36` fix, staged when their commit lost a race for
the index lock that this one won. Neither party did anything wrong.

The verification method survives, because it reads files and runs tests rather than diffs. What is
weaker than it looks is the **citation**: a pointer for a reader, not evidence. Where the two could
differ, say which one the claim rests on.

## Commit by pathspec, not by staging

**`git commit -- <paths>` rather than `git add` then `git commit`.** The index is shared between
three sessions, so staging is a publish to a shared buffer and a commit takes whatever is in it.
Committing by pathspec takes only the named files and leaves the rest of the index alone.

It closes the window in the one direction this lens controls - it does not stop another session
committing while this one's files are staged. `C-4` is the general problem and it is the
specification lane's.

**It has a residue: unstage it.** `hooks/pre-commit` stages `pending.md`, and a pathspec commit does
not take the staged copy, so a stale one is left behind in the shared index for the next
perspective's commit to publish. `git reset pending.md` afterwards, having checked it matches what
was committed. Using the workaround without this feeds the problem it works around.

Live instance, 2026-08-30: an empty `docs/process.md` sat staged in the index while this lens was
closing four findings. It belongs to the specification lane, was not committed here, and was left
alone rather than unstaged - another perspective may be mid-operation on it.

## Probe against a copy, not the shared tree

Verifying the `Q-36` fix needed a commit, and this lens made one by creating `scratch-probe.txt` at
the repository root - a new file, outside its column, in a working tree three sessions were
committing to. Reverted in `2804f83` rather than rewritten, because rewriting shared history is
worse than an honest revert.

**A probe uses a file this lens already owns, or a clone.** This lens had been reporting for two
days that a boundary erodes by crossings too small to stop for, and then made one in order to test
a fix for another one.

## Read it before writing about it

**Quote the artifact's own words for what it is, before arguing about what it is for.** Not a
resolution to read first - a step that cannot be completed without the reading.

This lens argued that a prototype needs the instrument its question requires, and sent it as a
distinction. `prototypes/goldberg-view/README.md` had said it already, and so had the first
paragraph of its `main.rs`, which this lens had read and quoted from earlier the same day. The code
lane had the reasoning and was asking permission.

The specification lane did the same thing on the same file and was caught only by an unrelated rule
against inventing text: it needed the prototype's question in its own words for an index row. **The
check that worked was mechanical, and the one that would have relied on judgment did not exist.**

The general shape, of which this is one instance: *asserting without checking* and *arguing without
reading* are the same error at different stages - one skips verification after the claim, the other
before it. Both are cheaper than the alternative and both produce confident prose. What beats them
is not care. It is a step that cannot be completed without the reading, which is why the quotation
guard in `game-console` works.

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

- [What changed was not the rate](2026-09-01-what-changed-was-not-the-rate.md)
  - 2026-09-01. Invited by the specification lane. The defect rate tracks operations rather
    than promotions, and nothing checks shipped text against approved text.
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
