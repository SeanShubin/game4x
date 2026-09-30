# Decide

**Everything in this directory is waiting on Sean, and nothing else is.** Open it and there is
nothing to scroll past: no history, no ledger, nothing addressed to another perspective, nothing
already settled.

[The specification](../spec/README.md) · [The release](../releases/first-release.md) · [Root README](../README.md)

## The five things only you can do

| Where it is                                  | What it asks                                                           |
| -------------------------------------------- | ---------------------------------------------------------------------- |
| [`proposals.md`](proposals.md)               | **approve words.** Say *promote P-n*, or say what to change            |
| [`questions.md`](questions.md)               | **answer a question.** No wording can be final until you do            |
| [the release](../releases/first-release.md)  | **vet a capability.** Look at the running game and say whether it held |
| the review application, `scripts/review.ps1` | **read a test.** A test you have not read constrains nothing           |
| [`regression/`](../regression/)              | **accept a case.** Deleting one says *I accept what it does now*       |

**The last two are why `attention.md` exists.** Neither is an item with an addressee, so
`pending.md` could never see one - a test waiting on a reading is `spec/tests/` against
`reviewed/`, and a stale case is only knowable by regenerating. **Four of the five are derived and
the fifth is named**, which the section below says.

**Empty means nothing is waiting.** A file here that says nothing is open is the whole report.


## `attention.md` is the one file, and it is generated

**Everything waiting on you is listed in [`attention.md`](attention.md)**, so that nothing has to
be remembered or looked for. It is generated and rewritten on every commit, which is what makes it
trustworthy rather than tidy: a thing that waits on me and is not in it is a defect in whatever
writes it.

**Four of the five kinds are derivable and one is not.** An open proposal, an open question and a
capability marked `built` are items with an addressee, and a test awaiting a reading is a file
comparison - present in `spec/tests/` and absent from `reviewed/`, or present in both and
different. **A stale regression case is only knowable by generating**, so `attention.md` says the
suite is what reports one rather than pretending to.

**A capability stays in the release that specifies it** and is listed here by name. Moving it would
separate a capability from the document the code lane builds from, and give that lane two places to
read.

## What is not here, on purpose

**Everything that has closed.** The ledger of accepted proposals, every item that was acted on or
withdrawn, and the reasoning behind answered questions are in `docs/notes/`. They are the record, and
the record is not a queue.

**Everything addressed to an instance rather than to you.** The specification lane's notices to the
code lane and to the lenses stay in [`docs/notes/proposals.md`](../docs/notes/proposals.md), which is
that lane's outbox. **This directory is not an outbox** - it is the one place a person is asked to
act, and an instance never files here to be read by another instance.
