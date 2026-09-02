# How this lane fails

**Derived, 2026-09-01, corrected 2026-09-02.** Written by Claude at Sean's request after a day of
**forty** promotions - the first version said twenty-eight, a count taken mid-afternoon and never
re-taken. Not binding.

**Five causes, and the fifth was found by the quality lens rather than by this lane.** Four are
below; the fifth is at the end, and it is the one that matters.

**The instances are not counted here.** Three counts in this document were wrong - twenty-eight
promotions where the ledger had forty, an inventory short by one, and eleven instances where its own
sections sum to twelve or thirteen depending on whether the opposite case counts. **All three were
found by a lens and none by re-reading.** The sections say what they say and a reader may add
them up; **a number in prose that nothing computes is the thing this note is about.**

**And the guards cannot move where most of them land.** Quality's second pass: the six guards
address two of the four causes, which is five instances. **The other seven are in the two that had
only a stated rule** - so the next defect was more likely than not to be in the same two places, and
that was knowable without waiting for more data. **Both now have a trigger**, which is the change
that answers it.

**And it was not the rate.** Quality measured it rather than judging it: **2026-08-26 was 47
promotions in 1 commit with 0 self-repairs; 2026-09-01 was 40 promotions in 43 commits with 5.** The
variable is the number of operations, not the number of decisions - and one of the four causes below,
a commit chained after a failing script, **can only happen once per operation.** Forty-three chances
rather than one.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What went wrong, by cause

### A check that confirms presence, not correctness

**Two defects, both in this lane's own editing tool, both committed.**

- **A cell was written into the wrong column.** Promoting `P-176` set `unpaid`'s **Values** to *a
  thing with upkeep*, because the script located the cell by counting columns and counted wrong.
  The check asked whether the string was present. **It was - in the neighbouring column.**
- **A paragraph was duplicated.** Promoting `P-166` inserted the `$name` sentence, a patch to skip
  the already-applied edit used `str.replace` on a string that did not match, and the insert ran
  again. **`str.replace` with no match is a no-op rather than an error**, which `CLAUDE.md` warns
  about in as many words. The check asked whether the sentence was present, not whether it was
  present **once**.

- **A third, found by quality and missing from the first version of this note.** `02587e8` cleared
  an owner cell left on a continuation row: rebuilding `work` blanked the Recipe cell of the row that
  used to be first and not the Owner cell beside it. **Reported at the time and not counted here**,
  which matters because the note's confidence rests on the count being complete and **the count was
  assembled by the instance being reviewed.** `f1feecf` is a fourth candidate of the
  incomplete-promotion shape and is not verified.

**All three are the same failure the whole day was about** - a green check that proves nothing - one
level below where it was being discussed.

### A commit outside the assertion boundary

**Three times**, a commit was chained after a script that could fail: `python - <<PY ... PY && git
commit`. The script failed an assertion, the commit ran anyway, and **a partial edit shipped under a
message claiming the whole**. Once the message asserted something false - *the word bin now appears
nowhere* - when it appeared three times.

### A claim made before the check that would refute it

**Three times.** *The word bin appears nowhere.* *No recipe makes a non-founding Pioneer perish* -
`upkeep` and `perish` do, and Sean asked. *A Yard could be lost to overflow* - it could not, and Sean
asked. **Each was reasoning presented as observation**, and each cost him a turn to correct.

### A question filed where text was required

**Three times** - `P-166`, `P-168`, `P-181` were filed as findings with options and no promotable
text. Sean said *promote* and there was nothing to copy. **The queue's contract is *here is the text,
approve it***, and a finding is not that.

**And once, the opposite**: `P-176` named an open question and promotion deleted it, breaking the rule
added to `CLAUDE.md` two hours earlier - **the first promotion after the rule.**

## What is fixed, and how it is checked

`edit.py` now has, each with a case that fails without it:

| Guard                                        | The defect it would have caught    |
| -------------------------------------------- | ---------------------------------- |
| `exactly_once=`                              | the duplicated paragraph           |
| `must_not_contain=`                          | a deletion that did not happen     |
| ragged-table check inside `apply`            | a row left the wrong width         |
| `set_cell(..., column, expect=)`             | the wrong-column write, twice over |
| `set_row` refusing a prefix with a bar in it | a prefix that padding breaks       |
| `section()` asserting it spans one heading   | a slice that covered two tables    |
| `commit()` inside the script                 | the three chained commits          |

**Six cases were run against the new version and all six fail where they used to pass.** That is the
standard `CLAUDE.md` asks for - a check that would have failed before - applied to the tool rather
than to the game.

## What had no tool, and now has one

**These two were left as stated rules, and the quality lens pointed out why that was not enough.**
**Nothing in this repository has ever been held by a stated rule.** What worked was always a step
that could not be completed without the check - the quotation guard, the hook refusing while an
outbox is unstaged, a trigger rather than a duty. **This lane made that argument itself, about
`goldberg-view`, and then wrote two duties.**

**Both now run inside `commit`, so a commit cannot be made without them.**

| Trigger                                                               | What it refuses                                            |
| --------------------------------------------------------------------- | ---------------------------------------------------------- |
| a message matching *appears nowhere*, *appears n times* and their kin | the commit, unless `claims=` is passed and the grep agrees |
| an open proposal addressed `to sean` with no `> ` block               | the commit, naming the proposal                            |

**Three cases were run: a message claiming something unchecked, a claim the tree contradicts, and a
proposal filed without text.** All three refuse. **A fourth case is worth recording because it
failed the other way** - a claim this lane expected to be caught was simply true, and the test was
wrong rather than the tool. **Which is the same defect as the three counts above**, in the check
rather than in the prose.

## The thing that has no home

**`edit.py` is a scratchpad file.** It makes every edit to `spec/` and `releases/`, and it is not in
the repository - not versioned, not reviewable, and gone when the session ends. **Two of today's
defects were in it**, and neither could have been caught by a review, because there is nothing to
review.

`CLAUDE.md` gives `tools/` to the code lane. **A tool the specification lane uses to edit the
specification has nowhere in its own column to live**, which is a gap in the columns rather than an
oversight - filed as a proposal.

## The fifth cause, which this lane could not see

**Every guard added above checks the edit against what the script intended. Nothing checks what the
script intended against what Sean approved.** `P-176` would have passed all six: the script did
exactly what it meant to, and it meant the wrong column.

**And after a promotion nothing can check it at all.** The Accepted ledger keeps a one-line row and
**the approved text is retained nowhere**. So the one guarantee the queue exists to provide -
*approved text is byte-identical to shipped text* - **becomes unverifiable at the moment it is
asserted.** That is why all eleven defects were caught by a person: the property that matters had no
check to be careless with.

**It is buildable, and only from git.** For a commit promoting `P-n`, take `P-n`'s text from the
**parent** commit's proposals file and assert it appears once in the destination the ledger names.
Same shape as `quotations.rs` and `first_release.rs`, both built after a hand-check missed something
twice. Filed to the code lane as `S-10`.

**There is a real argument against building it**, and quality made it: `P-182` may put the tool where
it can be reviewed, and a reviewable tool may not need a check downstream of it. **Those are
alternatives rather than a sequence**, and choosing costs less than building both.

## What a lane cannot see about itself

**This note was short by one and wrong in its first sentence**, and both were found by a lens rather
than by re-reading. The count was taken early and stated as though final; the inventory was assembled
by the instance whose work it inventories. **Neither is a mistake of care** - they are what
self-assessment is, and the reason a lens reads everything and owes nobody a defence.
