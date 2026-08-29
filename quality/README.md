# Quality

**Derived.** Written by the quality instance. Not binding - a report is an observation about the
code, not a decision about it. Sean decides what is acted on, and the code instance acts.

[Root README](../README.md) · [Architecture](../docs/architecture.md) · [Specification](../spec/README.md)

Code quality reports. This directory is the quality lane's only writable place, and no other lane
writes here - see [the lane table](../CLAUDE.md#three-instances).

## The rule that makes a report worth reading

**Quality never edits what it reviews.** It reads the tree, runs read-only tools, and writes here.
It does not fix the thing it found, does not reformat, and does not run `cargo fmt`, `cargo fix` or
`clippy --fix` - a review that alters its subject is no longer a review, and the next report would
be measuring its own last one.

The consequence worth stating plainly: **a report is only useful if someone acts on it.** A finding
that is true, well-argued and never acted on is indistinguishable from one that was never written.
So a report names what to do, not merely what is wrong.

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

## What is in scope

The tree, its structure, and whether the code says what the specification says. Concretely: whether
[architecture's rules](../docs/architecture.md#rules) hold, whether crate boundaries are real,
whether tests assert what they claim, whether names mean one thing, and whether anything in
`crates/` contradicts anything in `spec/`.

**A contradiction with the specification is the highest-value finding**, because neither of the other
two lanes is looking for it: the code instance reads the spec as instructions and the documentation
instance does not read the code.

## What is not

Style the formatter already settles, preferences with no argument behind them, and anything that
would be a design decision rather than an observation. **When a report finds that the specification
itself is wrong or unclear, it says so and stops** - that becomes a proposal in
[the documentation lane](../docs/notes/proposals.md), not a change here.

## Naming

One file per report, dated: `2026-08-28-crate-boundaries.md`. Reports are records of a moment and go
stale like any note - a superseded one says so at the top rather than being deleted, so a later
reader can tell whether a finding was fixed or merely forgotten.

## Reports

Newest first.

- [What the new prototype exposed, and what it did not](2026-08-29-coupling-under-the-game.md)
  - 2026-08-29. `Biome` in the game pulls terrain and rendering up into it, the picture and the
    model disagree about a territory's biome, and a detached globe still links the whole game.
- [What the response to the first report left behind](2026-08-28-response-to-the-first-report.md)
  - 2026-08-28. Five findings closed and verified; three quotations the P-95 sweep missed.
- [Crate boundaries, duplication, and where Bevy has spread](2026-08-28-crate-boundaries-and-duplication.md)
  - 2026-08-28. Whether Bevy is confined to the adapter, what is duplicated, and four places the
    code and the specification disagree. Findings 1, 2, 3, 4 and 13 are closed; the rest stand.
