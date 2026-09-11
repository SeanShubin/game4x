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

### P-396 - `move` destroys a unit and builds another, and that loses more than a token

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, because Sean asked what the second option solves and the answer is less than this item implied - **kind** entailed, from `P-390` promoted - **asks** a decision - **into** `releases/first-release.md` -> Recipes

**Five of the six blocks go straight through.** `create labor`, `work` and `bear` lose their
*produce ... not ready* row and consume a readiness; `renew` stops existing; `refresh` becomes one
rule. **`move` does not**, and your question is the right one to ask about it.

## What the second option was solving, and it is one symptom of three

**`move` is written as consume-and-produce**: it takes the unit at `$from` and makes one at `$to`.
**So the unit that arrives is a different thing from the one that left**, and the release has to
undo that thing by thing.

| What the departing unit had                                                     | What the arriving one has                                                      | Undone today by                                                                       |
| ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| a spent readiness                                                               | **a fresh one**, since `P-390` says a created thing arrives holding its tokens | the `not ready` trait, which tokens do not have - **this is what option two patches** |
| an **`id`**, and `spec/logistics.md` says a thing carrying one **is one thing** | no id, because the produce row states none                                     | **nothing**                                                                           |
| **energy in its tank**, which `move` spends from                                | no tank contents, because the tank went with the unit                          | **nothing**                                                                           |

**So option two answers the first row and leaves the other two.** That is the honest answer to your
question: **it solves no problem option one does not**, and it stops at the one this lane happened to
notice because `P-390` had just made it visible.

**The second and third are not urgent, which is why nobody has hit them.** No unit exists in the
scenario yet, so no id has been lost and no tank has been emptied by arriving somewhere.

## So the choice is narrower than this item first said

- **A moved thing is not a created thing.** `move` says a thing changes where it is. `spec/logistics.md`
  already carries the idea - *a thing is not located by a trait; **what holds it is what says where it
  is***, and *a thing carrying an `id` is one thing, and anything that holds it holds exactly it*.
  **All three rows above stop existing**, because nothing is destroyed
- **Keep consume-and-produce and patch it.** One patch per row: the produce names the tokens it
  arrives without, then the id it keeps, then what its tank still holds

**The thing that looked like an argument for the second is not one.** A Petri net has no move
primitive - a token moving is an input arc and an output arc, which is exactly consume-and-produce -
so keeping that shape looked like keeping the drawing honest. **But the release already states rules
in one form and grounds them into another**: a family becomes its members, a density becomes its
cases, and `P-390`'s actions ground into tokens. **A move in the notation can ground into two arcs in
the net**, and the net loses nothing.

**This lane is still not recommending, because the first changes the notation and notation is yours.**
What it will now say plainly is that the second is three patches rather than one, and that it was
offered as one because only one of the three had been noticed.
