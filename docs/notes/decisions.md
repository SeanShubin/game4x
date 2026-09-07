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

### P-338 - A thing may last a number of turns, and having no number is what durable means

**to** sean - **status** open - **raised** 2026-09-07 - **rewritten** 2026-09-07 - **kind** Sean's
own - **asks** a decision - **into** `spec/resources.md` -> the expires rule, or
`spec/invariants.md`; and `releases/first-release.md` -> Recipes

**Your idea, and it explains why `P-336` exists.** The specification has a **boolean** and the
release has a **number**, and neither the model nor the two documents ever met.

- **`spec/resources.md`** has a table with an **expires** column - yes for food, no for metal and
  energy - and *a resource that expires is lost when the turn ends, whether it was used or not*
- **`releases/first-release.md`** declares `keeps` - *the number of turns it will last* - and `age`,
  which turns one food into a food that keeps one less
- **The model implements the boolean**: `end_of_turn_losses` discards all food at every ending

**The rule, in your terms**: a thing may carry a number of turns it lasts. **It decrements at each
turn's end, and at zero the thing is gone. Having no number is what durable means.**

**It needs no language construct, and the mechanism is already used four times.** You asked whether
this wants generics, and whether an inheritance hierarchy is the shape. **It is neither** - the
recipe language already dispatches on **a family and a trait**.

A recipe's **Kind** column takes *the kind or the family alone*, `thing` is the family meaning *every
kind above*, and **Traits** constrains it. So `upkeep` is already **require 1 `thing`, with upkeep**,
and `perish`, `refresh` and `move` name families too. **`upkeep` is your pattern working today**: it
acts on anything carrying the trait and does nothing to anything that lacks it.

**So *do nothing if it is not present* needs no rule.** A recipe constrained on a trait does not
match a thing without it. That falls out rather than being handled.

**And `age` becomes a one-cell change.** `| age | world | consume | 1 | **food** | keeps at least 1 |`
becomes `thing`. `spoil` - *consume 1 food, keeps 0* - becomes `thing` as well. **Two cells, and the
whole mechanism is expressed.**

**Why there is no word for the category, which is what you were reaching for.** An inheritance
hierarchy would need a `Perishable` above `Food`. **A trait needs no category** - a kind gains the
behaviour by gaining the trait, and *durable* is then a statement about an absence rather than a
type. So the word you could not find is one the design does not need.

**Two things I am still not deciding.**

**1. What the trait is called.** `keeps` is declared and free, but you said *things* rather than
resources and `keeps` reads oddly on a unit.

**2. Where the rule lives.** Replacing the boolean puts it in `spec/resources.md`; **a rule about any
thing does not belong in the resources file**, and moving it to `spec/invariants.md` makes it
something every other document obeys, which is a bigger claim than you may have meant.

**`P-337` is a third concept and is not folded in here.** *Durable while upkeep is met* is a
condition rather than a count, and whether it should become a count is the question that item now
carries.

### P-336 - `age` is declared, fires in no state, and `keeps` is not implemented

**to** sean - **status** open - **raised** 2026-09-07 - **kind** contradiction - **asks** a decision
- **into** `releases/first-release.md` -> Recipes, and Traits

**The code lane's `C-61`, found by trying to build `R-7`'s example for it.** Verified rather than
taken.

**The release declares two things the model does not have.** Food has a trait `keeps` - *the number
of turns it will last* - and `age` is *consume 1 food, keeps at least 1* producing food that keeps
one less.

**`Trait::Keeps` does not exist**, and `end_of_turn_losses` discards **all** food at every turn's
end, with a comment saying so: *food keeps for one turn, so what is here at the end was made this
turn and expires now.* So `spoil` and `age` are one discard in the model, **and `age` fires in no
state that can be reached.**

**Which means `R-7` cannot show it.** An example would have to be drawn rather than generated - the
exact thing `P-330` says makes an artifact able to lie, and `R-7` found it by trying.

- **Implement `keeps`.** Food carries a number of turns, `age` decrements it, `spoil` takes what
  reaches zero. **The release already describes this** and the model would be catching up
- **Drop `age` and `keeps`**, and say food lasts one turn. **The model already behaves this way**,
  `spoil` alone expresses it, and one recipe and one trait leave the release
- **Keep both and mark `age` unreachable**, which is what is true today and what nothing says

**I have no recommendation.** The first is a game rule you may want; the second is what is built. **I
cannot tell from the specification which you intended**, because `spec/turn.md` says *what expires
expires* without saying whether food ages first.

### P-337 - `perish` says a thing is consumed and its metal produced, and the model does neither

**to** sean - **status** open - **raised** 2026-09-07 - **kind** contradiction - **asks** a decision
- **into** `releases/first-release.md` -> Recipes, and Traits

**The code lane's `C-62`, from the same attempt to build an example.** Verified.

**The release:** `perish` consumes *1 thing whose upkeep is unpaid* and produces *the thing's metal*.

**The model removes an unpaid citizen** - which matches - **and marks an unpaid unit
`usable = false`, in place, producing nothing.** Two places set it, at `game.rs:915` and `:965`.

**And `usable` is not a declared trait** - it appears **zero times** in the release. So a starved
pioneer reads `{pioneer fuel:2 id:1 ready:yes}` **before and after**, identically. **A reader
deriving that turn's end by hand would conclude the pioneer was fine.**

**That is worse than a rule the artifacts disagree about.** It is a state change no artifact can
show, in the file you vet, which is what `docs/process.md` says the four artifacts exist to prevent.

- **A unit perishes like anything else** - consumed, its metal produced. **The release already says
  this** and the model would be catching up. What it costs is that a starved unit is gone rather
  than recoverable
- **A unit is disabled rather than consumed**, and `usable` becomes a declared trait so the state
  change is visible. **The release gains a rule** distinguishing a thing that perishes from a unit
  that stops working
- **Something else**, if being unusable was meant to be temporary - nothing says whether a starved
  unit recovers, and I could not find it in `spec/units.md` either

**My recommendation is the second**, weakly. A unit is expensive and a wreck that can be recovered is
a more interesting rule than one that evaporates - but **that is a guess about the game you want**,
and the first is what the release says today.
