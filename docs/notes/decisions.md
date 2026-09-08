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

### P-357 - A tracked directory belongs to no perspective, and the new check cannot see it

**to** sean - **status** open - **raised** 2026-09-08 - **kind** the code lane's `C-72` -
**asks** a decision - **into** `CLAUDE.md` -> Perspectives

**Only you can approve a change to `CLAUDE.md`, which is why this is an item rather than an edit.**

**Checked rather than relayed**: `notes-to-incorporate-then-remove/` holds **1 tracked files**, and
the string does not appear in `CLAUDE.md` at all.

**Why it matters now rather than when it was created.** `P-352` put a check in `pre-commit` that
refuses a commit whose files span two perspectives' columns. **A directory in no column cannot span
anything**, so a commit mixing it with any lane's work passes. The code lane is carrying it as a
named exception that fails if it is ever placed - so it cannot outlive itself, but it is unguarded
until you say where it sits.

**Three ways, and the third is the one I would take.**

- **Give it to a perspective**, and the check guards it like everything else
- **Name it unowned**, as `pending.md` is - a generated file has no owner and the rule already exists
  for that case. **But this is not generated**, and the rule says a file with any hand-written part
  has an author
- **Delete it.** Its name says it is temporary and says what should happen to it: incorporate, then
  remove. **If it has been incorporated, the directory is finished**; if it has not, that is a
  backlog nobody is reading

**What I cannot tell you** is whether its contents have been incorporated, because that is a
judgement about your notes. **If they have, this needs no rule at all.**

### P-356 - A field may name a kind instead of a thing, and nothing says so

**to** sean - **status** open - **raised** 2026-09-08 - **kind** entailed, from `C-56` -
**asks** a decision - **into** `spec/console.md` -> The language

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

