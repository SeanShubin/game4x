# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-512 - *Where things are* still says a tank holds fuel, and one row of it changes the game

**to** sean · **status** open · **raised** 2026-09-14 · **kind** entailed · **shape** an instruction · **asks** approval · **into** `releases/first-release.md` -> Where things are

**The code lane filed `C-125` and cannot proceed past it.** `P-509` and the release now disagree
about whether a unit's tank holds anything, and the disagreement is load-bearing rather than
verbal.

```
spec/logistics.md      The things in it that can hold that kind contribute capacity and hold nothing

releases/first-release.md -> Where things are
                       | Container     | Holds  | Up to             |
                       | a unit's tank | energy | the unit's fuel   |
```

## What lands

**The section's opening sentence becomes:**

```
Every thing but the game is in another thing, and this release has two sorts of thing that give a
place room.
```

**And the table becomes:**

| Thing         | Gives room for                | Up to           |
| ------------- | ----------------------------- | --------------- |
| a store       | the resource it was built for | 10              |
| a unit's tank | energy                        | the unit's fuel |

**The territory's row goes.** A place does not give itself room - under `P-509` its capacity is the
sum of what is in it, so *a territory holds that kind up to its free capacity for that kind* is the
rule stated as if it were a container.

## Why the heading changes and the row does not

**The row was right and the column was wrong.** *A unit's tank, energy, the unit's fuel* is a true
statement about **capacity** and a false one about holding. **Renaming the column is the whole of the
correction**, and it is why this is an instruction rather than rows: the words offered describe a
table that does not exist yet, so none of them lands as written.

## The thing that is not bookkeeping, and it is yours

**A unit can no longer move out of a place that has no energy.** `P-511` made `move` consume its
energy from `$from`; if a tank holds nothing, the place must have it. **Today a unit moves on fuel it
carries and is refused with `NoCells` when its own tank is empty.**

**It is nearly equivalent and not quite.** A unit hauls energy when it leaves, and that energy joins
the new place - so a unit can still cross an empty territory by bringing fuel, and chain moves on
what it brought. **What changes is that the fuel it brought is the place's**, so anything else
standing there may spend it.

**That is a real change to the game and this lane is not deciding it.** It is the last clause of
`P-509` arriving somewhere visible.

## And it moves something you have already reviewed

**The containment tree stops drawing energy inside a unit.** `P-485` and `S-128` put a pioneer's fuel
there - `{pioneer ...} -> 1` over `{energy} -> 2` - and under pooling that energy is the territory's.
**`scenario/expected/play.4x` changes**, which is the file `R-6` rests on and `P-225`'s protocol says
is reseeded and unreviewed until you read it.

**The code lane has built nothing on this** and says so; it has the reading above and is waiting.

## How to tell it was carried out

**`releases/first-release.md` -> Where things are has two rows and no `Holds` column.** Its heading
row reads `| Thing | Gives room for | Up to |`, the store and the tank rows are present with their
cells unchanged, and **the territory row is gone**.

**And the section says `two sorts of thing that give a place room`**, where it said `three sorts of
capacity`. Three became two by the territory leaving, which is the check that the right row went.

