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

### P-339 - An ark and a pioneer print life rather than eating, and the spec already says what they are made of

**to** sean - **status** open - **raised** 2026-09-07 - **rewritten** 2026-09-07 - **kind** Sean's
own - **asks** a decision - **shape** an instruction - **into** `releases/first-release.md` -> the
opening list, *What bounds a kind in a territory*, and *Units*

**Your change.** *Lets make both arks and pioneers take no food upkeep. Thematically, pioneers have
the same 3d-printer that an ark has to create new life. The ark has to do more work initially to tune
the life to the planet; the pioneer just copies the blueprint the ark made.*

**Three places say otherwise today**, and the ark's own row already shows what a pioneer's should be.

| Where                             | Now                                                                                             | After                |
| --------------------------------- | ----------------------------------------------------------------------------------------------- | -------------------- |
| *Units*, pioneer's **Upkeep**     | `1 food per turn`                                                                               | empty, as the ark is |
| *What bounds a kind*, **pioneer** | `a capacity of 2, and the food produced here`                                                   | `a capacity of 2`    |
| the opening list                  | *A Pioneer that enters a territory nobody holds must found it, or it perishes for want of food* | replaced, below      |

**How to tell it was carried out**: the pioneer's Upkeep cell is empty, its bound is exactly
`a capacity of 2`, no line contains *perishes for want of food*, and `UnitKind::Pioneer.upkeep()`
returns 0 - the last being the code lane's, filed on promotion.

**It answers `P-338`'s first question rather than adding one.** No upkeep means nothing can ever
reset a number, so **a pioneer must have no number at all** - the argument that already made the ark
durable. `P-337`, a starved pioneer marked unusable, **dissolves outright**: a pioneer cannot starve.

**Two things I told you last turn were wrong, and the spec is what corrects them.**

**There is no loitering hole.** I said removing the upkeep lets a pioneer wait forever.
`spec/unit-types.md` says *An Ark is taken apart on arriving from orbit. A Pioneer is taken apart on
arriving from an adjacent territory.* **Arriving is being taken apart**, so nothing waits. The hunger
was the release's *enforcement* of a rule the specification states outright - so deleting it costs
nothing, and the bullet is replaced by what the spec already says rather than by a new rule:

> A Pioneer is taken apart on arriving in a territory nobody holds, as an Ark is on arriving from
> orbit

**And the seeder may already be there under another name.** `spec/unit-types.md`: *A unit may be
taken apart into what a territory needs to sustain itself: a structure that holds the ground, a
citizen, and a food extractor.* **The unit's own body is the materials.** So the two citizens do not
teleport after all - the pioneer is what carries them, and I read `produce pioneer` and `found by
land` as a gap when they are the two ends of one object being built and unbuilt.

**The blueprint is there too, and it is not carried.** `spec/narrative.md`: *The AI designs life
generally suited to a particular planet*, and *an Ark prints a planet's founding population, with
enough genetic variability for it to be viable.* **Planet-wide and the AI's** - which is exactly your
*the pioneers already have the blueprint, the ark already did that*.

**What is genuinely new in your sequence** is the order: the ark **scans**, then **manufactures a
seeder**, then **secures a territory** - three steps where the spec has one, *taken apart on
arriving*. And a pioneer is manufactured **along with its seeder**, which is one recipe producing
two things rather than a separate build.

**So the decision is whether a seeder is a thing yet, and I recommend not yet.**

**Against, for this release**: a seeder created with its pioneer and consumed with it has **no degree
of freedom**. Every pioneer has exactly one, always, so it can found whenever it likes and the seeder
changes no decision a player makes. It is a kind, a trait, two recipe rows and a container
relationship - a unit holding a thing, which nothing in the release needs - **to model a number that
is always 1.**

**For, and this is when it becomes load-bearing**: *militias on one planet invading another with a
different environment that the life there is better suited to.* **A blueprint tuned to planet X is
wrong on planet Y**, and the seeder is where that tuning lives. With one planet there is one
blueprint and nothing to tell apart. **With two, the seeder is the only place the difference can
sit.**

**My recommendation**: take the upkeep change now, replace the bullet with the spec's own sentence,
and record the seeder as what the second planet needs. **It costs nothing to defer** - the thing that
would carry it, a unit holding a thing, is the same capability whenever it is built.

**What I need from you**: whether that is right, or whether the ark's scan and the seeder should be
distinct steps in this release even with one planet to tune for.

**One consequence either way**: a citizen becomes **the only thing in the release with upkeep**, so
`perish` and `unpaid` still read on any thing and now bite exactly one kind, and `P-338`'s reset half
has a single user. Worth saying, because a rule with one user is easy to fit to that user.

### P-338 - A thing lasts a number of turns, paying its upkeep resets that, and having no number is durable

**to** sean - **status** open - **raised** 2026-09-07 - **rewritten** 2026-09-07 - **kind** Sean's
own - **asks** a decision - **into** `spec/resources.md` or `spec/invariants.md`, and
`releases/first-release.md` -> Traits and Recipes

**The rule.** A thing may carry a number of turns it lasts. **At each turn's end it resets to its maximum if its upkeep was paid, and decrements if it was not. At zero the thing is gone. Having no number is what durable means.**

**The order matters and the first wording did not fix it.** *It decrements at each turn's end,
paying its upkeep resets it* can be read as reset **then** decrement, which leaves every fed citizen
sitting at 1 and kills an unfed one in a single turn - **no slack at any maximum.** Reset **instead
of** decrement is what gives your farm-building turn, so the rule says which.

**One rule, not two.** Food carries a number and has **no upkeep**, so nothing resets it - the number
is a pure lifetime. A citizen carries a number **and** an upkeep, so eating resets it and not eating
spends it. Same trait, same decrement, same recipes; `age` and `spoil` each widen from `food` to
`thing`, which the recipe language already supports.

**The numbers, and the arithmetic is worth checking rather than trusting.**

| Kind        | Number | Why                                                                                     |
| ----------- | ------ | --------------------------------------------------------------------------------------- |
| **food**    | 1      | already declared - made this turn, gone at this turn's end                              |
| **citizen** | 2      | fed resets to 2; one unfed turn leaves 1 and it lives; a second leaves 0 and it is gone |
| **ark**     | none   | upkeep 0, so nothing could ever reset it - it must be durable                           |
| **pioneer** | none   | upkeep 0 under `P-339`, for the same reason                                             |

**Two is what gives your one turn.** *Citizens get a turn before they starve, so we can expand
without logistics as long as we set up a farm this turn.* With a maximum of 1, an unfed citizen
reaches 0 the same turn and there is no farm-building turn at all. **If you meant no slack, the
number is 1 and the sentence about the farm does not hold** - which is why the arithmetic is shown
rather than asserted.

**And `ark` is a check on the rule rather than an entry.** A thing with no upkeep can never reset,
so a number on it would be a countdown to certain death. **Food is exactly that and is meant to be**;
an ark is not. So *no upkeep* and *has a number* is a combination that means mortal-by-design.

**The consequence you found, which I had missed.** A description is a kind and **every stored trait**,
so **citizens at different numbers are different keys.** A territory reads
`{citizen durability:2 ready:yes} -> 6` and `{citizen durability:1 ready:yes} -> 2`. That is the same
splitting `ready` already does and needs no new machinery - but it multiplies citizen entries by the
number of levels, so **keeping the maximum low is not only a game choice.**

**And it makes a hole you deferred yesterday live.** If a territory has five citizens and three food,
three reset and two decrement - **and which two is a competing effect.** `spec/turn.md` requires
*a deterministic mechanic a person wrote and a player can change*, and `docs/notes/spec-backlog.md`
records that nothing supplies one. **You deferred it saying the first real collision is when to
decide. This is that collision.**

**One thing I need, and it has no default.**

**`P-339` answered the other.** An ark and a pioneer take no food upkeep, so nothing could ever
reset a number on either and **both are durable** - the pioneer's row below is settled rather than
open.

**Which citizens eat when there is not enough.** The predecessor fed those who worked before the
   idle, so the idle starved out - that is a rule a person wrote and a player could change, and it
   is recorded in `docs/notes/game-4x-predecessor.md`. **Whether it is yours is your call**, and
   without an answer the split above is undefined.

**What this dissolves**: `P-336`, because `age` then fires on anything with the number, and `P-337`,
because a starving thing keeps working while its number runs down and then perishes - so `usable` is
not needed and the state change is visible as the number falling.

**And it reaches further than this release, which you noted**: a territory whose weather wears things
down, a material durable against one condition and not another. **A number per thing today, a number
per thing per condition later**, keyed the way a deposit keys density by resource. The decrement and
the reset do not change - only how many numbers a thing carries.

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
