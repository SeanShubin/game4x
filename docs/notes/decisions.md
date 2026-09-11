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

### P-405 - Where a token lives, now that the map form cannot tell two citizens apart

**to** sean - **status** open - **raised** 2026-09-11 - **kind** entailed, from the code lane's `C-90` - **asks** a decision - **into** `spec/console.md` -> The language, and `spec/turn.md` -> Order of operations

**The code lane found this while building `P-399` and filed it before committing the work that
found it.** This lane checked it, and it is worse in two ways it did not report.

## What it found

**`P-399` moved readiness out of a thing's description and into its contents.** A citizen that has
spent its labor token and one that has not are both `{citizen}`, differing only in what they hold.
**`spec/console.md` keys the map on the description**, so *fourteen citizens, six of which have
spent a token* has no written form. The state is reachable by playing - one `create labor` in a
territory of two citizens makes it, and it lasts until the turn ends. **`containment::tree` refuses
rather than writing a file that is wrong about six of fourteen.**

## The first thing it did not report: the rule's own example is now stale

**`spec/console.md` says**: *a description is a kind and **every stored trait that thing has***; a
derived trait is never part of one, and **no trait may be left out** - `{citizen ready:yes} -> 8`
and `{citizen ready:no} -> 6`, never `{citizen} -> 14`.

**`ready` is the trait `P-399` deleted.** So the sentence that exists to forbid `{citizen} -> 14`
now illustrates itself with a trait the game does not have - **and `{citizen} -> 14` is exactly what
the form must now write.** The rule did not merely lose a case; it lost the case it was written
from.

## The second: what the dump writes today is ambiguous, and it is in a file you read

`scenario/expected/play.4x`, territory 1:

> `{citizen} -> 8`
> `{readiness for:bearing} -> 1`
> `{readiness for:labor} -> 1`

**Is that one token each, or one token between eight?** `spec/console.md` says *where a thing is, is
where it appears* and nothing about how a quantity nested under a grouped entry is read. **It must
mean one each, because one between eight would make the eight non-identical** - which the form
cannot say. **So the file is correct only under a convention nobody has written**, and a reader
deriving the dump by hand has no way to know it.

## Four ways, and the fourth is this lane's

|       |                                                                                                                                                      | Costs                                                                                                                  |
| ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **1** | **A citizen carries an `id`** - `spec/console.md` already says a thing with one has a description no other shares                                    | twelve territories of individually named citizens, and a dump that grows with the population                           |
| **2** | **The map keys on a description *and its contents***                                                                                                 | a change to the notation, and every nested entry becomes part of a key                                                 |
| **3** | **Readiness goes back to being a stored trait**                                                                                                      | undoes `P-399`'s shape, and the trait model could not express *two labor a turn*, which was your reason for leaving it |
| **4** | **A token is held by the place, not by the interchangeable thing** - `{readiness for:labor} -> 8` beside `{citizen} -> 8`, and *six spent* is `-> 2` | `P-390` says a **kind** declares how many a **thing** holds; the bound becomes one per thing that can take that action |

**Why this lane offers a fourth.** The first three are all about the **writing** - name the things,
change the key, or move the fact back into the key. **The fourth asks where the fact belongs**, and
answers that a citizen with no `id` is not a thing you can say anything about individually. Your own
rule is the argument: *there is never a quantity of a thing with an `id` - it is one thing.* **The
contrapositive is that a quantity of fourteen is fourteen interchangeable things, and which of them
spent a token is not a fact about the game.**

**It is also the only one of the four that writes the state `C-90` says has no form**, without an
id and without touching the notation. **A unit is unaffected either way**, because a unit carries an
`id` and can hold its own.

**This lane is not recommending**, because three of the four change what you decided this morning
and the fourth changes where `P-390` puts a token. **Whichever you take, `spec/console.md`'s example
needs a trait that exists**, and that is a promotion rather than a decision - it follows this.

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
