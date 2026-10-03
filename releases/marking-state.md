# Release: Marking State From A Page

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Releases](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)

## Goal

I can mark a rule test, an interface test or a regression case as approved, pending or denied from
a page I open anywhere, and the terminals see what I said.

## Scope

**The rules are already promoted and this release invents none.** `spec/README.md` rule 3 says what
an approval is about, what a record holds, how two tests are compared, and where a case's verdict
lives; `docs/process.md` says a case's verdict is a reminder and that authorizing a regeneration is
a separate gesture.

**The page is served from GitHub Pages and writes through the GitHub API with Sean's own token**,
which is what keeps `CLAUDE.md`'s rule intact: no lane writes a verdict, and the token is his.

**Interface tests are in scope and there are none.** `spec/tests/interface/` is empty, so the page
shows an empty set rather than a missing one - which is the state it is in rather than a defect.

## Capabilities

### E-1 - The comparison normalizes, and no approval rests on an editable number

**to** sean · **status** **built** 2026-10-01 · **from** `P-606` · **cited** `95e72cf3` · **evidence reported by the code lane and re-derived here.** The canonical order lives in `friendly-notation`, which both crates reach, and `game-console`'s duplicate is deleted - so *one function does that normalizing* is one function. **The second half of the clause is not true yet and that is the point**: 449 of the rows in `reviewed/rule/` are still in the schema's order, because `POST /convert` is run by him through the application and nobody else may rewrite a record. **His first action on this capability is to run it.**

- **In** - `spec/README.md` rule 3, *one function does that normalizing and everything that compares
  delegates to it*
- **Vetted when** - renumbering a `seq:` in `spec/data/schema.4x` clears no approval, and every
  record is in the order `spec/console.md` states. **The 57 convert**, which is not a change of
  approval - Sean, 2026-10-01: *the order of the columns is not significant, so this should not make
  tests different*

### E-2 - I can deny a test, not only approve one

**to** sean · **status** **built** 2026-10-01 · **from** `P-605` · **cited** `a25a5a47` · **evidence reported by the code lane and re-derived here.** `/denied` sits beside `/reviewed` as one gesture with two words, `review_of` returns a fourth mark, and the page has a deny button. **`a_denied_record_is_read_as_denied_and_an_old_one_as_approved` is the check**, which also holds the compatibility rule. Two judgements worth his eye: a denial whose rows no longer match is still *drifted*, and `denied` is its own class rather than `unseen` - showing it unread would ask him to read the one thing a denial says he has already done.

- **In** - `spec/README.md` rule 3, *a record saying `denied` means it is not* bound
- **Vetted when** - I deny a test, the suite stops running it, and the record says so. **The format
  and the reader already exist** - `{verdict state:denied}` and `verdict_of` - so this is a gesture
  and not a mechanism

### E-3 - A case takes a verdict and a regeneration is authorized separately

**to** sean · **status** **built** 2026-10-01 · **from** `P-607`, `P-608` · **cited** `db4717c2`, `c123b840` · **evidence reported by the code lane and re-derived here.** `{verdict case:... state:...}` and `{regenerate case:...}` in `reviewed/cases.4x`, with `a_case_verdict_stands_and_an_authorization_is_a_different_row` holding both halves. **The columns are the ones `P-608` showed him** rather than invented - `d37e657a`. And `world.4x` is excluded by *a case loads it* rather than by a name, which holds over all four suites where a `{when}` predicate would have excluded 129 real cases.

- **In** - `spec/README.md` rule 3, *a case's verdict is a row in `reviewed/cases.4x` and pins no
  behaviour*
- **Vetted when** - I mark a case denied and it stays marked across a run; I authorize a
  regeneration and the case is rewritten without my deleting anything. **165 cases**, and no suite
  is privileged

### E-4 - I can do all of it from a page, from anywhere

**to** sean · **status** **built** 2026-10-01 · **cited** `e2ef89ac` · **evidence reported by the code lane and re-derived here.** The page carries 57 `data-record` attributes, 165 `data-case` entries and no token - asserted by a check of its own. **36 scenario cases have server-rendered approve and deny buttons** and the 57 test cards get theirs from the script, which writes `reviewed/rule/<name>.4x` as `{verdict state:...}` above the body the card already carries. **The owner and repository are read out of the Pages URL**, so the page names neither and a copy served anywhere else offers no writing at all. `Contents: write` and no dispatch. **The round trip through GitHub is his to observe**, which is what makes this a capability only a person can vet, and the check says so rather than implying it covers the trip.

- **In** - `docs/process.md`, *I insist that the AI make its work verifiable to a human*
- **Vetted when** - I open a page away from this machine, mark a rule test and a regression case,
  and a terminal here sees both without my touching git. **A page that lists 222 rows flat is not
  this capability met** - the rule tests, the interface tests and the four regression suites are
  distinguishable without my counting
- **The page writes and does not dispatch.** Sean, 2026-10-01: *I can drop the regenerate feature
  for now, does that make it simpler.* **It does, and the saving is reach rather than code**: a
  verdict is a file write, so the token needs `Contents: write` and not `Actions: write`, and the
  page can commit records and start nothing. **One of the three shapes `P-610` offered, and the
  narrowest.**
- **`{regenerate}` is not dropped, only unoffered.** `E-3` is built and the row still works -
  authorizing a regeneration means writing it where he already is, and the next run spends it.
  **What goes is the button and the workflow job**, not the gesture

- **Shown without a control, and that is three suites rather than two.** Sean, 2026-10-01, asked
  whether `types/` and `primitives/` belong on the page: *let's show them, but these are
  informational only, no vetting capability need be implemented. We can even link to them if it
  helps with comprehensibility.* And asked where `regression/rules/`'s sixteen fall: *I was
  expecting to review 3 things. The tests I was reviewing before. The new user interface tests. And
  the regression tests. Everything else was to be informational only.*
- **So the three he reviews are the rule tests, the interface tests and `regression/scenario/`**, and
  everything else is informational.

  ```
  markable        57 rule tests · 0 interface tests · 36 scenario cases   = 93
  shown only      16 rules · 53 types · 60 primitives                     = 129
  ```

    **He said which suite he meant**, asked nothing and volunteering it: *by regression test I was
  referring to the scenario test that was broken down by turns.* **So `regression/scenario/` is the
  one**, and `rules/` falls in *everything else*.

  **It was an inference for one message and is his words now**, which is worth the distinction: the
  derivation was that *the regression tests* alone could have meant all four suites, and the earlier
  instruction had already made two of them informational, so it could not. **That reasoning was
  sound and it was still a reading** - a record saying *he said so* and one saying *this is what it
  must have meant* are not the same warrant, and only the first survives somebody disagreeing.


- **That is the interface declining to offer a control, not the notation forbidding one.**
  `spec/README.md` rule 3 says *no suite is privileged* and `reviewed/cases.4x` will take a verdict
  for any case, including one of those 113. **A reader comparing the page to the rule would see a
  contradiction and there is none** - one says what may be recorded and the other says what he has
  asked to be built


### E-5 - Nothing but me can write what I said

**to** code · **status** open · **from** `CLAUDE.md`, the record rule

- **In** - `CLAUDE.md`, *the record is the one artifact whose whole value is that nobody judged by
  it can touch it*
- **Vetted when** - a lane asking the page to mark something is refused by the same gate a stranger
  is, and whatever ships keeps it

## E-6 - I can see which category needs me and how much

- **In** - his words above, 2026-10-03
- **Vetted when** - the review entry point is a list of categories, one link each, and **each
  line tells me both how many items wait on me and how much reading that is** - so I can pick the
  category to spend an hour on without opening it. A category with nothing waiting says so and
  does not need opening at all


