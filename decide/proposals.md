# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-511 - What pooling does to moving: who hauls, where the fuel comes from, and the end of `refuel`

**to** sean · **status** open · **raised** 2026-09-14 · **kind** recovered · **shape** rows · **asks** approval · **into** `spec/logistics.md` -> Containment, and `releases/first-release.md` -> Recipes

**The code lane filed `C-124` saying it cannot proceed past three questions, and it is right about
all three.** Two of them are defects `P-509` created and this lane did not notice. **The first you
have already answered**, in the message that produced `P-509`, and it was never written down.

## One - given by whom

**Your words, 2026-09-14**: *we default to them hauling as much as they can... Haul most (you were
picking up), haul none (you were dropping off), or user specifies number to haul.*

> **What a thing hauls is the player's to say.** A command that moves a thing may give an amount for
> each kind; what it does not name it hauls as much of as it can carry. **The default is to fill**,
> because a thing that has come to fetch something is the common case and a thing that has come to
> deliver says so.

`P-509` wrote *is given an amount*, which is passive and leaves three different games open. This
says which.

## Two - `move` takes its energy from the place, not from the unit

| Recipe   | Owner  | Role    | Qty | Kind   | Traits | Where   |
| -------- | ------ | ------- | --- | ------ | ------ | ------- |
| **move** | player | consume | 1   | energy |        | `$from` |

**Replacing `that unit` in the `Where` cell.** Nothing else in the block moves.

**A unit holds nothing under `P-509`**, so *consume 1 energy from that unit* names a place with
nothing in it. **`$from` is where the energy is**, and for an ark in orbit that is the orbit -
which has room for exactly the fuel its units carry, so the ark's own tank is what put the energy
there.

## Three - `refuel` is deleted

**Its four rows go and nothing replaces them.**

```
| **refuel**          | player | require | 1 | territory |                        | `$where` |
|                     |        | require | 1 | unit      | free energy at least 1 | `$where` |
|                     |        | consume | 1 | energy    |                        |          |
|                     |        | produce | 1 | energy    |                        | that unit |
```

**It moves an energy into a unit and there is nowhere to move it to.** The energy is the place's
before and after; a tank contributes capacity and holds nothing. **And its qualifier is now always
true**: a unit that holds nothing has all of its free capacity, so `free energy at least 1` never
refuses.

**`C-112` already said no command fires it.** This is the stronger finding and it retires the recipe
rather than asking what would fire it.

## What this does not settle, and the code lane did not ask it

**How energy reaches an orbit.** `launch ark` produces an ark in the orbit above, and produces no
energy there - so an ark begins with an empty orbit around it and cannot move until something hauls
energy up. **The loop still closes**, because `deploy ark` costs no energy. **This lane checked that
and is naming it rather than fixing it**, since whether an ark should launch fuelled is a rule you
have not been asked about.

