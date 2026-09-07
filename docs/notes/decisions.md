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

### P-340 - `spec/unit-types.md` takes a unit apart on arriving, and both recipes only do it where nobody holds

**to** sean - **status** open - **raised** 2026-09-07 - **kind** contradiction, found while drafting
`P-339` - **asks** a decision - **into** `spec/unit-types.md` -> *Units that become structures*

**The specification is unconditional.** `spec/unit-types.md:16`:

- *An Ark is taken apart on arriving from orbit. A Pioneer is taken apart on arriving from an
  adjacent territory*

**Both recipes are gated.** `deploy ark` and `found by land` each carry `limit 0 garrison`, so
neither fires where a garrison already stands. **Arriving in a territory you hold takes nothing
apart.**

**Which means a pioneer can be moved through your own ground, and the sentence says it cannot.** Read
literally, a pioneer that steps into a territory you hold is consumed there - so it could never cross
one to reach a frontier, and with 12 territories and 30 edges most frontiers are not adjacent to
where a pioneer is built.

**This is not something `P-339` creates.** Both halves are already in the tree; deleting the bullet
about starving neither causes nor hides it. I found it drafting the bullet's replacement, and stopped
because **writing the condition into the release would have been me deciding this.**

**Your rule picks an answer, and I am not applying it for you.** *Nothing we don't need, but don't
make a design choice that is going to be troublesome later.* **Unconditional is the troublesome
one** - it forbids traversal now, and forbids it harder when there are many planets and a pioneer
built on one is carried toward another. The gated reading is also **what is built and what the
scenario exercises**, so choosing it changes no code.

**Three ways, and the middle is what the code does today.**

1. **Unconditional stays** - and `deploy ark` and `found by land` lose their `limit 0 garrison`, so
   arriving anywhere consumes the unit. Simplest sentence, and it makes a pioneer unable to travel
2. **The sentence gains the condition** - *taken apart on arriving where nobody holds the ground*.
   Matches both recipes, matches the scenario, changes nothing built
3. **The player chooses** - arriving is arriving, and taking a unit apart is a separate command.
   **More than we need**, but it is the version that never has to be revisited, because a unit that
   can arrive without being consumed is already what a seeder would need

**I recommend 2**, and note that 3 is where 1 and 2 both end up if a unit ever needs to arrive and
stay.

**Whichever you pick, the release needs nothing.** The recipes already encode 2. This is the
specification catching up with what was built, which is the direction that is usually wrong - so it
is worth your eye rather than mine.

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
