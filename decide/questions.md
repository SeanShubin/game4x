# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-588 - The sun is a deposit and it is per planet, and the data has no planet to hang it on

**to** sean · **status** open · **raised** 2026-09-28 · **asks** a decision · **kind** measured · **from** `C-166`

**Your two sentences are both right and together they need a shape that is not there.** *The sun acts
as a deposit for energy*, and *the sun is per planet, not per territory*.

```
deposit.where          names a place, and a place is one layer of one territory
capacity.per           takes `place` (88 rows) and `territory` (2). There is no `planet`
the schema             has no `planet` relation at all - a planet is the set of its territories
```

**So a per-planet density has nowhere to be stated once.** Stated per orbit it is stated as many
times as there are territories, and `spec/invariants.md` says a fact is stated once.

## And the scenario already shows the cost of that, measured

```
scenario/main.4x   2 orbit places: place-2 above territory-1, place-4 above territory-2
                   {deposit where:place-2 what:energy density:3}      the sun, above the first
                   place-4                                             nothing at all
```

**The same star gives three above one territory and nothing above the other.** An ark in territory-2's
orbit cannot gather. **Nobody stated that; it fell out of stating a planet's fact in one place.**

## Three shapes, and the first is the only one that states it once

- **A `planet` enters the data**, and a deposit may name it - or `per:planet` joins `per:place` and
  `per:territory`. **The sun is then one row for the whole game**, and `gather` reads it wherever the
  ark is. Costs a relation the model has done without so far
- **The sun is stated per orbit and the repetition is accepted.** Twelve rows saying one thing, which
  contradicts *a fact is stated once* - and is what the scenario does today by accident rather than
  by choice
- **The sun is not a deposit.** `gather` stops requiring one and takes its density from somewhere
  else - which undoes the sentence you just gave me, so it is here to be visibly refused rather than
  because this lane means it

**What this lane will not do is pick.** The first adds a noun to the data model, and
`spec/invariants.md` is where nouns are decided - so it is yours whichever way it goes. **`S-219` is
not at stake**: a weighting that holds with the star charged to the planet holds with it charged to
the star.
*Nothing is open. Everything filed has been decided.*
