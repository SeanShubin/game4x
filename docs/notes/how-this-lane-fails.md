# How this lane fails

**Derived, 2026-09-01.** Written by Claude at Sean's request, after a day of twenty-eight promotions
and a visible number of mistakes. Not binding. **The point is the causes, not the count** - four
causes produced eleven defects, and three of the four have a fix.

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

**Both are the same failure the whole day was about** - a green check that proves nothing - one level
below where it was being discussed.

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

## What no tool fixes

**Claiming before checking is a habit.** The rule is: *if a sentence says the file contains something,
read the file in the same breath.* Every one of the three cost a round-trip, and every one was
avoidable by a grep.

**Filing a question instead of text is a habit too.** The rule is: **a proposal always carries the
text it would promote, even when this lane would rather ask.** Where the choice is genuinely open, the
text goes in with the alternative named beneath it.

## The thing that has no home

**`edit.py` is a scratchpad file.** It makes every edit to `spec/` and `releases/`, and it is not in
the repository - not versioned, not reviewable, and gone when the session ends. **Two of today's
defects were in it**, and neither could have been caught by a review, because there is nothing to
review.

`CLAUDE.md` gives `tools/` to the code lane. **A tool the specification lane uses to edit the
specification has nowhere in its own column to live**, which is a gap in the columns rather than an
oversight - filed as a proposal.
