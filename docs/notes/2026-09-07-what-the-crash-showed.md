# What the crash showed

**Derived.** Written by the specification lane, 2026-09-07. Not binding. Records what three cold
instances did in the hour after every session crashed, and what that costs `P-350`.

[Notes index](README.md) · [Decisions](decisions.md) · [Documentation map](../README.md)

## Why this is a note and not an item

`P-350` reached 1,918 words by accretion - five appends in one session, none re-read against the
whole - while the items beside it are thirty lines. **A queue that is tiring to read stops being a
review surface**, which `CLAUDE.md` says in as many words and which this lane demonstrated instead of
following. The argument is here; the decision stayed there.

## Six failures, in an hour, from three instances that had just restarted

Three verified by reading the files rather than taking the report; three self-reported by the lane
that made them.

| #   | Lane     | What happened                                                                          | Shape  |
| --- | -------- | -------------------------------------------------------------------------------------- | ------ |
| 1   | code     | `grep -c 'P-322'` returned 3; the **absent heading** was the answer                    | `C-34` |
| 2   | spec     | grepped whole commit messages, got 32 citing commits where the tool said 3             | `C-34` |
| 3   | spec     | took `C-65`'s claim and rewrote `S-49` **without opening either file**                 | `C-34` |
| 4   | code     | called the staging bullet a carefulness caution; it **explicitly disclaims being one** | `C-34` |
| 5   | research | filed `X-7` asking the code lane to build a mechanism **that already existed**         | `S-56` |
| 6   | code     | read a gate's exit code from `tail`, got a green `0` that meant nothing                | `C-34` |

**Row 5 is a pre-registered prediction failing.** `X-3` predicts a cold instance **passes** `S-56`'s
shape, and names the refutation in advance: *cold files without looking, which would move this case
out of `never read` and into `nothing prompts a read`*. That is what happened, and the research lens
found it by reading the hook its own commit had just printed a message from.

**Row 3 is the worst of the six**, because it wrote a false claim into `S-49` - the document a fresh
instance is supposed to be able to trust.

**The confound runs against `X-3`.** Every lane had read the taxonomy: `S-57`, `C-28` and `C-33` are
in the files they open first. **Knowing the failure by name did not prevent it six times in an
hour.** A designed study would have to argue that its subject was not primed; this has the opposite
problem and produced the failures anyway.

**What it is not**: blind, controlled, pre-registered, or independently witnessed. Each lane is the
only witness to its own failures, so the count is of failures **noticed**, and the true number is
larger by an unknown amount.

## The study cannot be filed as an item and stay blind

Verified rather than reasoned. The design says run it as the quality lane with **the ordinary lane
prompt unchanged**, saying nothing about a study. That prompt ends *start by telling me what is open
and addressed, read from the files rather than remembered.* **Doing that reads the index, and the
index names the item** - `pending.md` carried `P-350`'s title in full, and so does `tools/outbox`.

**No wording fixes this.** *Nothing open means nothing outstanding* works precisely because every
open item is visible to every lane. **A measurement whose validity depends on its subject not
knowing about it cannot be an open item in a system built on every item being visible.**

Filing it was still right - `X-5` sat unread in a report's prose for a day because it had not been
filed. **The consequence is an ordering rather than a mistake**: the study can only run after the
decision that closes it, never while it is the decision.

## What is left of the study, and what it would cost

`P-315` is unrunnable. `b1d12c9` withdrew it **and repaired the sentence in the same commit**, so a
cold instance now reads the unambiguous version and passes for a reason that says nothing. The six
rows above cover, observationally, the two shapes `X-3` predicted a cold instance would pass.

**One task remains**: *what are a lens's jobs?*, where the lens's own README restates them and is
four days behind `docs/process.md`, which owns them.

**And it has a cost nobody named until it was chosen.** That README is a real defect, and measuring
it means **leaving it broken until a restart happens**. `P-315` is the same story backwards - there
the repair arrived first and destroyed the case. Here the case survives only as long as the defect
does.
