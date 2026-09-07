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

### P-339 - An ark and a pioneer print life rather than eating, and a seeder is what they carry

**to** sean - **status** open - **raised** 2026-09-07 - **kind** Sean's own - **asks** a decision -
**shape** an instruction - **into** `releases/first-release.md` -> the opening list, *What bounds a
kind in a territory*, *Units*, and *Recipes*

**Your change.** *Lets make both arks and pioneers take no food upkeep. Thematically, pioneers have
the same 3d-printer that an ark has to create new life. The ark has to do more work initially to tune
the life to the planet; the pioneer just copies the blueprint the ark made.*

**Three places say otherwise today**, and the ark's own row already says what a pioneer's should.

| Where                             | Now                                                                                             | After                |
| --------------------------------- | ----------------------------------------------------------------------------------------------- | -------------------- |
| *Units*, pioneer's **Upkeep**     | `1 food per turn`                                                                               | empty, as the ark is |
| *What bounds a kind*, **pioneer** | `a capacity of 2, and the food produced here`                                                   | `a capacity of 2`    |
| the opening list                  | *A Pioneer that enters a territory nobody holds must found it, or it perishes for want of food* | deleted              |

**How to tell it was carried out**: the pioneer's Upkeep cell is empty, its bound is exactly
`a capacity of 2`, no line contains *perishes for want of food*, and `UnitKind::Pioneer.upkeep()`
returns 0 - the last being the code lane's, filed on promotion.

**It answers `P-338`'s first question rather than adding one.** No upkeep means nothing can ever
reset a number, so **a pioneer must have no number at all** - the same argument that made the ark
durable, since a number on either would be a countdown to certain death. `P-337`, a starved pioneer
marked unusable, **dissolves outright** rather than being held: a pioneer can no longer starve.

**But that bullet was doing design work**, and this is what I need decided. It is what forces a
pioneer to found rather than sit. Remove the upkeep and **a pioneer can wait in an unheld territory
forever at no cost**, which the release currently forbids.

**Your seeder is a better answer than a timer, and it fixes something separate.** *Something like a
seeder that has the materials and blueprints to create two citizens, designed by ark and carried by
pioneer.*

**Two citizens already vanish and reappear, and nothing represents them in between.** `produce
pioneer` consumes 2 citizens; `found by land` produces 2 citizens. Same number, different territory,
**no thing carrying them across** - which is the one thing `spec/console.md` says never happens:
*where a thing is, is where it appears.* Today a pioneer is the only unit whose cost teleports.

**A seeder makes that transit a thing.** It is what is spent to found, so a pioneer without one
cannot found and loitering costs the seeder rather than costing nothing. **The pressure becomes a
carried resource instead of hunger**, which is what your theme wanted in the first place.

**Three things it needs before it can be written, and none is mine to settle.**

1. **Who makes it.** *Designed by ark* suggests the ark, which would also be the *more work initially
   to tune the life to the planet* - and would move the 2-citizen cost off `produce pioneer` onto
   whatever the ark does. **Or a yard makes it and the ark only designs the blueprint.**
2. **Whether a unit can hold a thing.** A seeder carried by a pioneer appears *in* the pioneer, and
   nothing in the release has a unit as a container - a unit has `fuel` and that is all. **This is
   the real cost of the idea**, and it is a genuinely new capability rather than a row.
3. **What a spent pioneer is.** After founding it has no seeder. Gone, as today, or an empty unit
   that something can load again.

**If you would rather not pay for 2 yet**, the alternative is to let a pioneer wait: a printer does
not starve, and loitering already costs a unit that could be founding elsewhere. **That needs no new
kind**, and the seeder stays available for when a unit holding a thing is worth building.

**One consequence either way**: a citizen becomes **the only thing in the release with upkeep**. So
`perish` and `unpaid` still read on any thing and now bite exactly one kind, and `P-338`'s reset half
has a single user. Worth saying, because a rule with one user is easy to fit to that user.

**And it points where you are going.** Militias from one planet invading another whose life is better
suited to it - **a blueprint tuned to a planet is a trait of what the seeder carries**, not of the
unit carrying it. Same shape as a number per thing per condition. Neither needs deciding now.

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
