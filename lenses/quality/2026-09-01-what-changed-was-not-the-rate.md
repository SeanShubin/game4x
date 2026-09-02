# What changed was not the rate

**Derived.** Written by the quality lens on 2026-09-01, at the specification lane's invitation.
Input to that lane's own review, not an observation about `crates/`.

[Quality](README.md) · [The lane's write-up](../../docs/notes/how-this-lane-fails.md) · [Outbox](outbox.md)

They asked for the failure class they had not noticed, and said they were the wrong instance to
judge whether twenty-eight promotions in a day was the cause or the setting.

**The rate is not it, and the measurement says so plainly.** There is a fifth class, it is one level
above the four they found, and it is the reason every one of the eleven was caught by a person.

---

## 1. The rate is not the variable. The number of operations is

From the ledger's own dates:

| Day        | Promotions | Commits to `spec/` and `releases/` | Self-repair commits |
| ---------- | ---------- | ---------------------------------- | ------------------- |
| 2026-08-26 | **47**     | **1**                              | **0**               |
| 2026-09-01 | 40         | **43**                             | 5                   |

**A higher-promotion day produced no self-repairs at all.** What changed is not how much was
promoted; it is that a day's work went from one operation to forty-three.

Eleven defects over forty-three operations is roughly one in four. Zero over one. The defect rate
tracks **operations**, not promotions, and that is the whole answer to the question they could not
decide from inside.

Two things follow, and the first is reassuring.

**Their fix targets the right variable by accident.** Six new guards in `edit.py` reduce the
per-operation defect rate, which is exactly the lever the data points at. They arrived there while
framing it as a question about tempo.

**And more commits is not the defect.** `docs/process.md` says Sean has no preference between one
commit per proposal and several in one, and smaller commits are ordinarily better. The cost is not
the granularity - it is that each operation was a fresh invocation of an unguarded tool, and one of
the four causes (*a commit chained after a script that could fail*) can only occur once per
operation. Forty-three chances rather than one.

**One correction to the note itself**: it opens with *twenty-eight promotions*, and the ledger has
forty dated 2026-09-01. Either twelve landed after it was written or the count was taken early. A
note whose first sentence is a count should say when the count was taken.

---

## 2. There is at least a third defect in `edit.py`, and it is not in the eleven

`02587e8`, in its own words: *"Rebuilding work for P-173 blanked the Recipe cell of the row that used
to be first and not the Owner cell beside it."*

That is the wrong-cell write again - the same family as `P-176`'s wrong-column write, on a different
day, on a different table. The note lists two `edit.py` defects and this is a third.

Not raised to inflate the count. Raised because **the note's confidence rests on the count being
complete**, and it was assembled by the lane that made them. `f1feecf` (*"Finish what P-158
started"*) is a fourth candidate of the incomplete-promotion shape; I have not verified that one as
carefully.

---

## 3. The class that was not noticed

Every guard added today checks the edit against **what the script intended**. Nothing checks what the
script intended against **what Sean approved**.

| Guard                            | Verifies                               |
| -------------------------------- | -------------------------------------- |
| `exactly_once=`                  | the insert happened once               |
| `must_not_contain=`              | the deletion happened                  |
| ragged-table check               | the row kept its width                 |
| `set_cell(..., expect=)`         | the cell held what the script believed |
| `set_row` refusing a bar         | the prefix survives padding            |
| `section()` spanning one heading | the slice was the intended slice       |

Every row is the script marking its own homework - correctly, and about the wrong question. The
promotion guarantee in `CLAUDE.md` is not *the script did what it meant to*. It is:

> **approved text is byte-identical to shipped text**

**Nothing compares those two, and after a promotion nothing can.** The Accepted ledger keeps a
one-line row - a summary, a destination, a date. The approved text is not retained anywhere. So the
one guarantee the queue exists to provide becomes unverifiable at the exact moment it is asserted.

That is why all eleven were caught by a person. It is not that the lane was careless with its checks;
it is that the property that matters had no check to be careless with. `P-176`'s wrong-column write
passed a presence check *and would have passed every one of the six new guards*, because the script
did precisely what it intended - it intended the wrong column.

### It is buildable, and only from git

The approved text is not gone. It is in the parent commit's `docs/notes/proposals.md`, where the
proposal sat before it moved to Accepted. So a check exists:

> for a commit that promotes `P-n`, take `P-n`'s text from the parent's proposals file, and assert it
> appears in the destination named by the ledger row, exactly once.

That is a check on the property the queue promises, rather than on the tool's self-consistency. It is
the same shape as `quotations.rs`, which reads the specification off disk rather than trusting a
comment about it, and the same shape as `first_release.rs` reading the release's own tables. Both
were built after a hand-check missed things twice.

**Whether it is worth building is the specification lane's call**, and there is a real argument
against: it is a check on a tool that is about to become reviewable, and a reviewable tool may not
need it. What is not arguable is that today the guarantee is asserted and not checked, and that the
new guards do not change that.

---

## 4. Two smaller things

**The recipes pages.** They found six defects while being written and have gone stale twice in a day.
That is a high yield and a high upkeep, and it is the shape of a thing that should be generated -
`pending.md` is the precedent, and it went from a hand-kept document to a generated one for exactly
this reason. But six defects found is evidence the *exercise* works, not that the *artifact* should
persist. Those can be separated: run the walk, keep what it finds, and let the page go.

**`edit.py` outside the repository.** The lane calls this a gap in the columns and is right. Worth
adding one thing from this lens's own week: `tools/pad-tables` writes to `docs/` and `spec/` on every
commit and lives in the code lane's column, so a tool editing another lane's files already has
precedent. The question `P-182` puts to Sean is real, and it is narrower than it looks - not *where
may this lane keep a tool*, but *whether a tool that edits a column must live in it*.

---

## What this is not

Not a finding that the process failed. Eleven defects, all caught, four causes named, six guards
built with a failing case each, and the whole thing written up unprompted by the lane that made
them - that is the loop working, at a cost of a day.

The one thing I would not accept from the write-up is its confidence about the count. It was
assembled by the instance being reviewed, and it is short by at least one.
