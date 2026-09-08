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

### P-356 - A field may name a kind instead of a thing, and nothing says so

**to** sean - **status** open - **raised** 2026-09-08 - **kind** entailed, from `C-56` - **asks** a
decision - **into** `spec/console.md` -> The language

**`spec/console.md` says how a field that refers to a **thing** is written, and says nothing about a
field whose value is a **kind**.** The release uses both:

| Field           | Its value | What it does                     | Governed? |
| --------------- | --------- | -------------------------------- | --------- |
| `territory:1`   | an `id`   | refers to one particular thing   | **yes**   |
| `resource:food` | a kind    | says which kind, not which thing | **no**    |
| `unit:ark`      | a kind    | says which kind, not which thing | **no**    |

**The cost is already paid once.** The code lane could not tell whether `unit:ark` was legal, took
the form that preserved behaviour, and filed the question - which is `C-56`, open since the 6th.
**Nothing was wrong; the document just did not answer.**

**Two ways, and I have no evidence favouring either**, so I am not recommending.

- **Say the second shape exists** - a field may name a kind, and then its value is one of that
  kind's names rather than an `id`
- **Say when each applies** - which is more, and would have to answer why `unit:ark` names a family
  where `resource:food` names a trait

**Not urgent and nothing waits on it.** `C-56` closes either way, and the code lane has been told so.

