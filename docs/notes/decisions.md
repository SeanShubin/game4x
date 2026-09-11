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

### P-380 - Nothing removes `labor` or `fertility`, and neither is a resource

**to** sean - **status** open - **cited** `985adaa`, `6495f1c` - **raised** 2026-09-10 - **rewritten** 2026-09-11, when Sean corrected the rule it argued from - **kind** entailed, from `C-83` - **asks** a decision - **into** `releases/first-release.md` -> Recipes, then What bounds a kind in a territory

**The code lane found this by working it rather than imagining it.** A territory with two citizens
and no food: `upkeep` fires nothing so both are unpaid, `bear` turns both spent and leaves two
`fertility`, `breed` cannot fire for want of food, `renew` makes them fertile, and `perish` takes
both. **The territory ends the turn with no citizens and two fertility**, and next turn `breed`
fires twice and two citizens appear from nobody. `game-model` has a test named for the rule that
forbids it - *a population of none never grows however much food there is*.

**Checking it found `labor` has the same hole**, and nobody had raised it.

| Kind        | Made by        | Removed by                        | A resource |
| ----------- | -------------- | --------------------------------- | ---------- |
| `fertility` | `bear`         | `breed`, and nothing else         | no         |
| `labor`     | `create labor` | what spends it, and nothing else  | no         |
| `food`      | `work`, `grow` | `upkeep`, `breed`, `age`, `spoil` | yes        |
| `metal`     | `work`         | what builds, and `discard`        | yes        |
| `energy`    | `work`         | what moves, and `discard`         | yes        |

**Yesterday this asked you to approve two `discard` rows, and your correction removes its grounds.**
It argued that `P-369`'s sweep already reaches both, because *what a territory holds directly is in
disorder*. **That is the sentence you have called wrong**, and `P-381` replaces it. On the corrected
rule **only a resource is ever in disorder, and neither of these is a resource** - so nothing sweeps
them, and `discard` is the sweep for resources rather than a general one.

**So what removes them is open, and no wording can be final until you settle it.** Three ways, and
they need not have the same answer for both kinds.

- **`discard` takes them**, as it takes metal and energy - the two rows drafted yesterday, unchanged.
  They need a ground the corrected rule does not give: either both are matter in disorder despite not
  being in the family of three, or what decides it is whether anything can hold a kind rather than
  whether it is a resource
- **They are made with `keeps` 1**, so `age` and `spoil` take them as they take food. **This needs no
  new rule** - `keeps` is a trait the release already has and food already uses. It says they expire
  rather than that they are loose, which is a different idea about the same kinds
- **Nothing sweeps them, and the recipes stop leaving a remainder.** A token that cannot outlive the
  step that made it needs no sweep. The largest of the three, and the only one that changes how a
  turn is built

**One reading is ruled out rather than passed over**, because it had to be. The code lane offered
*the accumulation is intended, a territory that starves banking its capacity to recover*. **Then a
dead population no longer stays dead**, and `spec/control.md` no longer says when a player has lost -
it says *no citizens and nothing that converts into a citizen*, and leftover fertility is something
that converts into a citizen.

**Whichever you choose, one row in the release is already false.** *What bounds a kind in a
territory* says `labor` is bounded by *the citizens that make it, one each per turn* - true only if
something removes what is left when the turn ends, and nothing does. **`fertility` has no row there
at all.** The first two ways make the labor row true as written and give fertility the same words;
the third needs different words for both.

