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

### P-338 - A thing lasts a number of turns, paying its upkeep resets that, and having no number is durable

**to** sean - **status** open - **raised** 2026-09-07 - **rewritten** 2026-09-07 - **kind** Sean's
own - **asks** a decision - **into** `spec/resources.md` or `spec/invariants.md`, and
`releases/first-release.md` -> Traits and Recipes

**Your reset is the piece that makes it one mechanism rather than three.** With it, spoilage and
starvation are the same rule and `P-336` and `P-337` both dissolve into this.

**The rule.** A thing may carry a number of turns it lasts. **It decrements at each turn's end.
Paying its upkeep resets it to its maximum. At zero the thing is gone. Having no number is what
durable means.**

**Why that is one rule and not two.** Food carries a number and has **no upkeep**, so nothing ever
resets it - the number is a pure lifetime and food goes when it runs out. A citizen carries a number
**and** an upkeep, so eating resets it and not eating spends it. **Same trait, same decrement, same
recipe** - the difference is entirely whether the thing has an upkeep to pay.

**So `spoil` and starvation stop being separate ideas.** `age` is the decrement over any thing with
the number; `spoil` is the removal at zero. Both are already recipes and both become **one cell
wider** - `food` becomes `thing`, which the recipe language already supports.

**What this dissolves.**

- **`P-336`** - `age` is unimplemented and fires in no state. Under this it fires on anything with
  the number, which is what the release always meant
- **`P-337`** - a starved pioneer is marked unusable. Under this it **keeps working while its number
  runs down and then perishes**, so `usable` is not needed and the state change is visible as the
  number falling. **A citizen that cannot eat still works until it is gone**, which is what you
  described

**Three things I need from you, and the first is the only one with no default.**

**1. The maximum for each kind.** Food is 1 today and the release says so. **A citizen and a pioneer
need numbers and I will not invent them** - they set how much slack a player has, which is the game
rather than the mechanism.

**2. What it is called.** You said *durability* thematically. `keeps` is declared, free, and reads
oddly on a citizen; *durability* reads well on both and is a new word in a release `P-284` keeps
closed.

**3. Where the rule lives.** `spec/resources.md` holds the boolean it replaces, but **this is about
any thing, not a resource** - so `spec/invariants.md` may be the honest home, which is a larger
claim.

**On gravitating rather than simulating.** You want a human to last longer without food than without
water without building that. **The shape admits it without redesign**: a number per thing today, and
a number **per thing per upkeep resource** later, keyed the way a deposit keys density by resource.
**Nothing about the decrement or the reset changes** - only how many numbers a thing carries. So the
cheap version is not a dead end, which is the property you asked for.

### P-336 - `age` is declared, fires in no state, and `keeps` is not implemented

**to** sean - **status** open - **raised** 2026-09-07 - **kind** contradiction - **asks** a decision
- **into** `releases/first-release.md` -> Recipes, and Traits

**Held under `P-338`, 2026-09-07.** Your thematic model - a thing lasts a number of turns, and paying its upkeep resets that - makes this one case of a single rule rather than a question of its own. **It dissolves if `P-338` lands** and comes back only if it does not.

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

**Held under `P-338`, 2026-09-07.** Your thematic model - a thing lasts a number of turns, and paying its upkeep resets that - makes this one case of a single rule rather than a question of its own. **It dissolves if `P-338` lands** and comes back only if it does not.

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
