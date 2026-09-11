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

### P-388 - Two choices between you and a checked no-gain invariant

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, because its first version named the wrong source - **kind** entailed - **asks** a decision - **into** `releases/first-release.md` -> Kinds, and a check for the code lane

**The invariant is a place invariant of the recipe net**: a weighting `w` over the kinds where, at
every recipe, `w · (made - taken) <= 0`. Whether one exists is a linear feasibility question over 73
rows and sixteen kinds. **That is what *decided mechanically, from the rules alone* can be.** Three
of the four pieces exist - see the end. **Two choices are yours.**

## What this lane got wrong, corrected before you choose

**`P-388` first said a deposit is the source and that `work` draws metal from it. Nothing in the
release touches a deposit** - counted over every recipe row. A deposit supplies the *number* `work`
produces, through a trait, and is never taken. **`work` makes resources out of labor and readiness**,
which is where the gain actually is.

**The unpaid input in this game is readiness, and only one recipe makes it.** `refresh` is the sole
recipe that produces a thing marked `ready`, and it is a **world** recipe that fires at the turn's
end. Three player recipes take one: `move`, `create labor`, `work`. **So the turn is the well and the
ready things are the pump**, which is `spec/invariants.md` in its own words - *what is finite is the
gathering, bounded by the finite things that do it, in a finite number of turns.*

## Choice A - who writes the weighting

`spec/invariants.md` says both *the weighting is where it is said what counts* and *decided
mechanically, from the rules alone*, which do not agree about the author.

|       | Option                    | What you do                                                                                                           | What it cannot tell you                                                                                 |
| ----- | ------------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| **1** | **Declared**              | put a **Weight** column on *Kinds*; the check verifies one inequality per recipe and names the one that fails         | nothing - but the numbers are yours to invent, and a wrong guess reads as a broken game                 |
| **2** | **Derived**               | nothing; the check solves for any `w` that works and fails when none does, naming the recipes that force it           | whether the `w` it found is one you would accept - it may weigh labor at nothing and call the game safe |
| **3** | **Derived and published** | nothing; as 2, and the weighting it found is printed in a report you can read                                         | same, but you would see it - a disagreement becomes visible rather than silent                          |
| **4** | **Partly declared**       | give weights only for the kinds you have an opinion about; the solver completes the rest or says no completion exists | nothing this lane can see. **It is 1 and 2 doing the halves each is good at**                           |
| **5** | **Both in full**          | derive to prove one exists, then declare the one the game means, and check the declared one                           | nothing - it answers the question twice and costs two mechanisms                                        |

## Choice B - what the rule exempts as a source

*A source is named, and a named source is not a gain.* **Nothing in the release names one**, and
without an exemption no weighting can exist, because `refresh` makes readiness from nothing.

|       | Option         | What is named                                                                                  | Consequence                                                                                                                                                                      |
| ----- | -------------- | ---------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1** | **The recipe** | `refresh` is declared to draw on a named source; every other recipe must satisfy the weighting | one row of data, and any future world recipe that re-arms must be named too or the check goes quiet about it                                                                     |
| **2** | **The phase**  | the invariant is required of the **player's ten recipes** and not of the world's ten           | no new data at all. Readiness is not restored during a player's phase, so the player's net is already acyclic in it                                                              |
| **3** | **The trait**  | readiness is weighed at nothing, so `refresh` gains nothing by construction                    | **this one does not work**, and is listed so it is not tried: if readiness weighs nothing then `create labor` makes labor out of nothing, and `work` makes resources out of that |

**Option 2 is the one this lane would take if it were choosing, and it is not.** It needs nothing
written, it matches what `X-29` found independently - *the player's recipes are an ordinary Petri net
and the world's are not* - and it is the sublanguage a player will author rules in, which is where an
unenforced invariant would actually hurt. **Option 1 is stronger**: it asks the question of the whole
game rather than half of it, and would catch a world recipe that gains.

## What needs nothing from you

- **Constant arc weights.** 72 of 73 rows carry one. `work`'s *`$where`'s density for that resource*
  is the exception, and `P-376` already resolves it - *one rule with a number per case ... which
  whatever reads it may spell out*
- **No zero tests.** `P-385` deleted the only two `limit` rows. With one, the net is Turing-complete
  and nothing about it is decidable
- **Soft lines.** `P-386` keeps softness on what a rule makes, and a soft line only ever makes less,
  so a weighting that holds where everything is made holds for every partial firing
