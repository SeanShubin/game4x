# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-589 - `P-588` stated two things the notation cannot express, and both shapes are yours

**to** sean · **status** open · **raised** 2026-09-29 · **asks** a decision · **kind** measured · **from** `P-588`, `S-221`

**The `planet` relation is in `spec/data/schema.4x`** - one relation, an id, a `{state}` row - which
unblocks the code lane's half. **Neither of `P-588`'s two sentences can be written as rows yet**, and
both want a shape rather than a wording.

## One: the star's deposit cannot name the planet

```
{reference id:22 column:51 to:place}   deposit.where references `place`
no column in schema.4x                 carries two references - measured, zero
no reference in schema.4x              targets a family - measured, zero
```

**So *the star is a deposit on the planet* has nowhere to be a row.** Four shapes, and none is
obviously right:

- **A column may reference more than one relation.** `where` becomes *a place or a planet*. Changes
  what the notation can say, so it is `spec/invariants.md` -> The data is a normalized relational
  model rather than a data file
- **A reference may target a family**, and a family gains `place` and `planet`. Families are over
  **kinds** today and these are relations, so it widens what a family is
- **A planet is a place**, with `place.of` admitting a planet - which moves the same problem one
  level up rather than solving it
- **A second relation for a planet's deposit.** Nothing in the notation changes and *the sun acts as
  a deposit* stops being literally true, which was your own sentence

## Two: *its territories are in it* has no row either

**A `territory` gains an `of` column**, which is `P-577` exactly: a mandatory column over 380
existing rows, 126 of them in `reviewed/`, and 34 tests red until every row carries it. **That is a
dated failure and it is three days old.**

**Or a join relation**, the way `terrain` joins a territory to its biome - `{in of:territory-1
is:planet-1}` - which states it once per territory without touching the territory rows. **`P-578` is
the precedent and it was yours.**

## What this lane would say

**The join for the second**, because `P-578` already chose that shape for the same problem and
nothing argues the two should differ. **No lean on the first**: it is a change to what the data model
can express, and every one of the four costs something in a different currency.

**Nothing is blocked on the answer.** The `planet` relation exists, `S-219`'s verdict does not depend
on where the sun's row sits, and the scenario's two orbits are wrong today whichever shape lands.
