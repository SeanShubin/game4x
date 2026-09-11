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

### P-380 - Nothing removes `labor` or `fertility`, and neither is a raw material

**to** sean - **status** open - **cited** `985adaa`, `6495f1c` - **raised** 2026-09-10 - **rewritten** 2026-09-11, twice, as Sean's two corrections removed what it argued from - **kind** entailed, from `C-83` - **asks** a decision - **into** `releases/first-release.md` -> Recipes, then What bounds a kind in a territory

**The code lane found this by working it rather than imagining it.** A territory with two citizens
and no food: `upkeep` fires nothing so both are unpaid, `bear` turns both spent and leaves two
`fertility`, `breed` cannot fire for want of food, `renew` makes them fertile, and `perish` takes
both. **The territory ends the turn with no citizens and two fertility**, and next turn `breed`
fires twice and two citizens appear from nobody. `game-model` has a test named for the rule that
forbids it - *a population of none never grows however much food there is*.

**Checking it found `labor` has the same hole**, and nobody had raised it.

| Kind        | Made by        | Removed by                        | Comes out of a source |
| ----------- | -------------- | --------------------------------- | --------------------- |
| `fertility` | `bear`         | `breed`, and nothing else         | no                    |
| `labor`     | `create labor` | what spends it, and nothing else  | no                    |
| `food`      | `work`, `grow` | `upkeep`, `breed`, `age`, `spoil` | yes                   |
| `metal`     | `work`         | what builds, and `discard`        | yes                   |
| `energy`    | `work`         | what moves, and `discard`         | yes                   |

**Yesterday this asked you to approve two `discard` rows, and both of your corrections since have
cut the ground out from under them.** It argued that `P-369`'s sweep already reaches both kinds,
because *what a territory holds directly is in disorder* - the sentence `P-381` now replaces. **On
your principle the three states belong to a raw material, and neither of these is one**: nothing
extracts either from a source, and no deposit offers either. So `discard` cannot take them without
saying they are raw materials, which they are not.

**So what removes them is open, and no wording can be final until you settle it.** Two ways, and
they need not have the same answer for both kinds.

- **They are made with `keeps` 1**, so `age` and `spoil` take them as they take food. **This needs
  no new rule and no new state** - `keeps` is a trait the release already has, and `C-61` settled
  that `age` fires before `spoil` in one ending, so a thing made with `keeps` 1 lives exactly the
  turn it was made. `bear` makes a fertility, `breed` may spend it, and what is left never sees the
  next turn
- **Nothing sweeps them, and the recipes stop leaving a remainder.** A token that cannot outlive
  the step that made it needs nothing to remove it. The larger of the two, and the only one that
  changes how a turn is built

**One reading is ruled out rather than passed over**, because it had to be. The code lane offered
*the accumulation is intended, a territory that starves banking its capacity to recover*. **Then a
dead population no longer stays dead**, and `spec/control.md` no longer says when a player has lost -
it says *no citizens and nothing that converts into a citizen*, and leftover fertility is something
that converts into a citizen.

**Whichever you choose, one row in the release is already false.** *What bounds a kind in a
territory* says `labor` is bounded by *the citizens that make it, one each per turn* - true only if
something removes what is left when the turn ends, and nothing does. **`fertility` has no row there
at all.** The first way makes the labor row true as written and gives fertility the same words; the
second needs different words for both.
