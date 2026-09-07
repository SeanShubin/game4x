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

**to** sean - **status** open - **raised** 2026-09-07 - **kind** Sean's own - **asks** a decision -
**into** `spec/resources.md` -> the expires rule, or `spec/invariants.md`

**Your idea, and it explains why `P-336` exists.** The specification has a **boolean** and the
release has a **number**, and neither the model nor the two documents ever met.

- **`spec/resources.md`** has a table with an **expires** column - yes for food, no for metal and
  energy - and *a resource that expires is lost when the turn ends, whether it was used or not*
- **`releases/first-release.md`** declares `keeps` - *the number of turns it will last* - and `age`,
  which turns one food into a food that keeps one less
- **The model implements the boolean**: `end_of_turn_losses` discards all food at every ending

**So the release already made the move you are describing and nothing followed it.** That is `P-336`,
and this is the rule underneath it.

**The rule, in your terms**: a thing may carry a number of turns it lasts. **It decrements at each
turn's end, and at zero the thing is gone. Having no number is what durable means.**

**Three things I am not deciding, and they are why this asks rather than offers.**

**1. What it is called.** `keeps` is declared and used - *food keeps for one turn* - so generalising
it costs no new word. But you said *things*, not resources, and `keeps` reads oddly on a unit.
**Nothing else in the release wants the name**, so it is free either way.

**2. Where it lives.** The boolean is in `spec/resources.md` and would be replaced there. **But a
rule about any thing does not belong in the resources file**, and moving it to `spec/invariants.md`
makes it something every other document obeys. **The second is a bigger claim than you may have
meant.**

**3. Whether it answers `P-337`.** You said the two seem related and I can see the shape - a starved
unit is *on its way out* the way expiring food is - but **a countdown on a disabled unit is a game
rule I would be inventing.** If a starved unit is meant to be recoverable, a number is the wrong
tool; if it is meant to decay, this is exactly the tool. **That is yours and I have not assumed it
either way.**

**What it would cost, so the size is visible.** The `expires` column in `spec/resources.md` becomes a
number or goes; `keeps` widens from *food* to *a thing*; `age` becomes implementable and `R-7` can
show it; and the model gains a per-thing counter where it has a blanket discard. **`spec/turn.md`'s
*what expires expires* still reads correctly** and needs nothing.

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
