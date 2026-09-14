# Decide

**Everything in this directory is waiting on Sean, and nothing else is.** Open it and there is
nothing to scroll past: no history, no ledger, nothing addressed to another perspective, nothing
already settled.

[The specification](../spec/README.md) · [The release](../releases/first-release.md) · [Root README](../README.md)

## The three things only you can do

| File                                        | What it asks                                                           |
| ------------------------------------------- | ---------------------------------------------------------------------- |
| [`proposals.md`](proposals.md)              | **approve words.** Say *promote P-n*, or say what to change            |
| [`questions.md`](questions.md)              | **answer a question.** No wording can be final until you do            |
| [the release](../releases/first-release.md) | **vet a capability.** Look at the running game and say whether it held |

**Empty means nothing is waiting.** A file here that says nothing is open is the whole report.

## Why the release is a link rather than a file here

**A capability is part of the document that specifies it**, and the code lane builds from that same
document. Moving the five that wait on you would separate a capability from the release it belongs
to, and give the code lane two places to read. **So it stays where it is and this points at it** -
`pending.md` lists which ones by name.

## What is not here, on purpose

**Everything that has closed.** The ledger of accepted proposals, every item that was acted on or
withdrawn, and the reasoning behind answered questions are in `docs/notes/`. They are the record, and
the record is not a queue.

**Everything addressed to an instance rather than to you.** The specification lane's notices to the
code lane and to the lenses stay in [`docs/notes/proposals.md`](../docs/notes/proposals.md), which is
that lane's outbox. **This directory is not an outbox** - it is the one place a person is asked to
act, and an instance never files here to be read by another instance.
