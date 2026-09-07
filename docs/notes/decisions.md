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

### P-331 - `total capacity` is stored and is written nowhere, so the round trip cannot close

**to** sean - **status** open - **raised** 2026-09-07 - **kind** contradiction - **asks** a decision
- **into** `releases/first-release.md` -> Where things are, or `spec/console.md` -> The language

**The code lane's `C-53`, confirmed independently by the quality lens, and the error in it is mine.**
`P-322` told you *`total capacity` is untouched and needs nothing - `spec/logistics.md` makes it a
fact about containment keyed by kind, so it is computed from what a thing holds rather than written.*

**`spec/logistics.md` says the opposite**, and I have now read it rather than remembered it:

> That maximum is its **total capacity** for that kind, and **it is stored**. **Used capacity** is
> how many of that kind it holds, and **available capacity** is the total less the used; both are
> derived.

**Total is stored. Used and available are the derived pair.** I described a stored trait as derived,
and the description made an absence sound like a rule being obeyed.

**The exhibit, measured rather than argued.** `capacity` appears **zero times** in
`scenario/expected/play.4x`. The release gives territory 3 as **6 x 2** for food - total capacity
six, density two - and the file says:

```
{territory biome:grassland id:3 nature:1} -> 1
  {deposit density:2 resource:food} -> 1
```

**Two is there and six is nowhere.** Territory 3 has built no extractors, so nothing it holds implies
six either. **A reader with the data file alone cannot say what that ground offers.**

**So `P-320`'s check cannot pass as written**: *the check is that the dump reads back into the state
it came from.* It cannot, and the same fix that gave `density` a home is available.

- **`total capacity` joins the deposit.** `{deposit resource:food density:2 capacity:6} -> 1`. **One
  trait moves and nothing else changes** - a deposit already exists, is already written, and is
  already keyed by resource. **My recommendation**
- **Capacity per kind is its own thing**, because a territory's capacity for stores or garrisons is
  not about a resource and would not fit on a deposit. **More correct and more work**, and it is the
  general case `spec/logistics.md` actually describes - *per kind, per family, or per kind carrying
  a trait value*
- **Narrow the round trip** and say the file states what things contain rather than all of the
  state. **Cheapest, and it gives up the property `P-320` was promoted for**

**The first is not the general case and I want to be plain about that.** It closes the round trip for
the capacities that exist today, all of which are per resource. **The second is what the rule
describes**, and choosing the first is choosing to meet the rule where the game currently stands
rather than where it is written.

**Nothing is blocked and nothing was guessed.** The code built exactly what you approved, and its
containment module says in its own words that total capacity is written nowhere.
