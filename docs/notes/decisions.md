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

### P-341 - How an Ark leaves, and whether building one is the same act as launching it

**to** sean - **status** open - **raised** 2026-09-07 - **kind** Sean's own - **asks** a decision -
**into** `spec/structures.md` -> Yard, and `releases/first-release.md` -> Recipes

**Your words.** *I was envisioning a structure capable of launching an ark, possibly the same one
that builds it. Possibly building and launching are the same act.*

**Nothing says how an Ark leaves, and the scenario proves it.** `{launch-ark}` is the last command
in the main scenario and fires no recipe - `crates/game-console/src/fired.rs:70`, *no recipe in it
names an orbit*. **So the ark is still there afterwards**: `scenario/expected/play.4x:48` reads
`{ark fuel:1 id:1 ready:yes}` in the finished state.

**That is why `R-6` cannot be vetted.** Its evidence is *a scenario reaches a fully exploited planet
and launches an Ark*, and the launch currently changes nothing. **`R-6` is the open capability this
decides**, and I named `R-9` for it an hour ago, which was wrong.

**Two ways, and they differ in whether an Ark you build ever stands on the ground.**

**A - two acts.** A Yard produces an Ark; a separate `launch ark` requires a Yard and consumes it.

- The Ark exists, can move, can ascend, and could deploy on another territory of this planet
- **The scenario contradicts it as written**: the yard is in territory 1 (line 148), the ark is
  produced there (164), then **moved to territory 2** (170) and launched (182) - and territory 2 has
  no yard. So promoting A makes the scenario wrong and the code lane has to change it

**B - one act.** A Yard builds and launches in the same breath: 3 metal, 12 energy, 2 citizens, and
the Ark is gone.

- **No Ark ever stands on this planet**, so lines 164, 170 and 182 collapse to one command, and
  `{ark fuel:1 id:1 ready:yes}` leaves the expected state
- The Yard requirement is automatic rather than a rule anyone can violate
- **What it costs**: an Ark built here can never deploy here. Today that route exists - ascend, then
  `deploy ark` on unclaimed ground for 2 extractors and 2 stores, against a pioneer's 1 extractor for
  half the energy. **Nothing in the scenario uses it**, so it is a possibility being closed rather
  than a behaviour being removed

**I recommend B**, by your own two tests. *Nothing we don't need*: it deletes a unit lifecycle the
game never exercises. *Not troublesome later*: an Ark's destination becomes an argument to the one
command when there are many planets, which is where it would have to live under A too.

**What I need is the choice.** A or B - and if B, whether the recipe is called `launch ark`, since
that is the act the player means and `produce ark` would no longer describe what happens.

**Either way `spec/structures.md` gains a line**, because *A Yard produces Arks* is all it says today
and a release may not invent the rest.

### P-336 - `age` is declared, fires in no state, and `keeps` is not implemented

**to** sean - **status** open - **raised** 2026-09-07 - **kind** contradiction - **asks** a decision
- **into** `releases/first-release.md` -> Recipes, and Traits

**Held under `P-338`, 2026-09-07**, which is now in [proposals.md](proposals.md) asking approval. Your thematic model - a thing lasts a number of turns, and paying its upkeep resets that - makes this one case of a single rule rather than a question of its own. **It dissolves if `P-338` lands** and comes back only if it does not.

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

**Dissolved by `P-339`, 2026-09-07.** A pioneer takes no food upkeep, so it cannot starve and there is no state for `usable` to mark. **It returns only if `P-339` does not land.** Formerly held under `P-338`: Your thematic model - a thing lasts a number of turns, and paying its upkeep resets that - makes this one case of a single rule rather than a question of its own. **It dissolves if `P-338` lands** and comes back only if it does not.

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
