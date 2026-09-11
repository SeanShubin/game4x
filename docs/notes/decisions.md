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

### P-407 - Is a trait that never varies stored, and does it belong in a description

**to** sean - **status** open - **raised** 2026-09-11 - **kind** entailed, from the code lane's `C-92` - **asks** a decision - **into** `releases/first-release.md` -> Traits, or `spec/console.md` -> The language

**The code lane found this reading the same sentence that produced `P-405`, and it predates `P-399`
so neither lane can blame today.** This lane checked every claim against the files.

**`spec/console.md`**: a description is a kind and **every stored trait that thing has**; *no trait
may be left out*.

**The release declares `force` **stored**, of citizen, garrison, ark and pioneer.** The data file
writes **`{garrison force:0}`** and **`{citizen}`**. **Three of the four kinds that have a force do
not carry one, and the one that does is the one whose force is zero.**

## What checking it added

**All four forces are constants.** *Units and structures* gives one number per kind - citizen 1,
garrison 0, ark 2, pioneer 2 - and nothing varies them. What varies is a **territory's** force,
which `spec/control.md` computes from what is standing there. **So there is no citizen anywhere with
a force of anything but 1.**

**That inverts which line is at fault.** If a trait that never varies is not stored, then
`{citizen}` is right and **`{garrison force:0}` is the defect** - one kind writing a fact about its
kind into a description of one of its instances. **Three lines are not missing a word; one line has
a word too many.**

## The two ways

- **`force` is not stored.** It is a fact about the kind, the way a store's capacity of ten is -
  which is the line `S-58` already drew, with `catalog.md` saying which bounds belong to a kind and
  which to each one. **The *Traits* table loses a word and `{garrison force:0}` becomes
  `{garrison}`**
- **`force` is stored and three kinds are missing it.** Every citizen writes `force:1`, every ark
  `force:2`. **The dump grows a word per thing that never differs**, and the model grows a field to
  hold it

## Why this and `P-405` are one question asked twice

**`P-405` asks where a fact about **one** interchangeable thing can live**, when the map groups
identical things and readiness is no longer part of the key. **This asks whether a fact about a
**kind** belongs in the description of an instance at all.** Both are the map form not
distinguishing what is true of a kind from what is true of a thing - **and answering them
separately risks two mechanisms for one distinction.**

**This lane is not recommending either**, but it will say that the arithmetic favours the first:
`{citizen}` appears in every territory and `{garrison force:0}` in one, so the cheaper repair is
also the one that removes a word rather than adding thousands.
