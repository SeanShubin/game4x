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

### P-485 - nothing ever puts energy in a unit's tank, and two views disagree about whether there is any

**to** sean · **status** open · **raised** 2026-09-13 · **kind** contradiction, found by asking whether a pioneer can be built, moved and found with in one turn · **shape** an instruction · **asks** a decision · **into** `releases/first-release.md` -> Recipes, or `spec/logistics.md`

**Your question has an answer and it stops at energy.** Producing, moving and founding in one turn
is allowed by everything except the one thing nothing states.

## What the release says about a unit's energy

```
Where things are    a unit's tank | energy | the unit's fuel
Traits              fuel | how much energy its tank holds | the kind
move                consume 1 energy ... from `that unit`
produce pioneer     consume 3 metal, 6 energy, 2 citizen ... produce 1 pioneer
```

**One recipe row in the whole release takes energy out of a unit and no row anywhere puts any
in.** `stow` fills a *store*, not a tank. `produce pioneer` consumes six energy from the territory
and says nothing about the pioneer it makes.

## The two views, on the turn the pioneer appears

```
the state        {pioneer id:1 defending:1 moving:1} -> 1        nothing inside it
the entity view  id 1 | pioneer | territory 1 | fuel 2 | ready
```

**And one turn later, after the move, the entity view says `fuel 1`.**

**Those cannot both be right.** `spec/logistics.md` says a thing appears inside what holds it, so a
pioneer holding two energy would show two energy nested under it - and the state shows an empty
pioneer. **Meanwhile `fuel` is `kept:kind`**, so it is one number for every pioneer that ever
exists and cannot go from 2 to 1 for one of them.

## What is actually missing

**A rule for how a unit gets energy**, which is one of three things:

- **`A` - a unit is created with a full tank**, and `produce pioneer`'s six energy includes it. Then
  the state must show that energy inside the pioneer, and the dump is currently wrong
- **`B` - a unit is created empty and something fills it**, and that recipe does not exist yet. Then
  a fresh pioneer cannot move, and the scenario's turn-8 move worked for a reason the release does
  not contain
- **`C` - a tank is not containment at all** but a per-unit count like `moving`, in which case
  `fuel` is `kept:thing` and not `kept:kind`, and *Where things are* should not list it as a
  container

**This lane leans `C`.** It is the only one where nothing has to be added: `fuel` becomes the
maximum on the kind's line - which `P-459` already made the meaning of a number there - and a
per-unit count that `move` lowers by one, exactly like `moving`. **`A` and `B` both require a
recipe or a rule that does not exist.**

## What it does to your question

**Under `A` or `C`, yes**: build, move and found in one turn. `spec/turn.md` says a thing created
during a turn begins at its full count and may act at once; **`found by land` names no action at
all**, so it is not blocked by the `moving 0` the move leaves behind.

**Under `B`, no** - and the scenario has been passing on behaviour nothing specifies.
*Nothing is open. Everything filed has been decided.*
