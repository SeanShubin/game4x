# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

### P-315 - Nobody may create a lens's directory, and one is now promised in the process document

**to** sean - **status** open - **raised** 2026-09-06 - **kind** governance - **asks** a decision -
**into** `CLAUDE.md` -> Starting a new lens

**The code lane declined `S-52` and was right to.** I asked it to create `lenses/research/README.md`
and `lenses/research/outbox.md`. `CLAUDE.md` -> Perspectives says **a producer never writes into a
lens's directory**, and that rule is absolute rather than scoped to findings. **The same forbids
me**: this lane is the other producer.

**So the answer to *who creates it* is currently nobody**, and `docs/process.md` -> *Starting the
instances* now names `lenses/research/outbox.md` in a prompt, which makes it a file the document
promises and the rules forbid anyone to make.

**`CLAUDE.md` -> Starting a new lens says what to create and never says who.** *To start one, create
`lenses/<name>/README.md` and `lenses/<name>/outbox.md`, and tell it:* - an imperative with no
subject.

**Two answers, and this is a change to who may write where, so it is yours and only yours.**

- **The lens creates its own.** You start the instance, it reads its prompt, and its first act is to
  make the directory it is told is its own. Nothing is relaxed - a lens writing `lenses/<name>/` is
  exactly its column. **My recommendation**, and it needs only a subject added to that sentence
- **You create it**, which is what the sentence reads as today, and which is one more thing that
  needs you before anything can start

**Why it is filed rather than settled.** `CLAUDE.md` says an approval for that file comes from you
directly, and this is squarely what it means: the columns and who may write in them. **A relayed
approval cannot be told from an invented one**, so this lane will not act on anything but your own
words here.

**`S-52` is withdrawn in the same commit**, naming this item as what now carries the gap - which is
`P-305`'s rule, promoted this afternoon, used for the first time.
