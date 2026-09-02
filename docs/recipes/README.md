# Recipes, before and after

**Reviewed.** One page per recipe in [the first release](../../releases/first-release.md), showing a
state the recipe can fire in and the state after it fires. **The tables are the data and this is a
rendering of them**; where the two disagree, the release is right and a line here is a defect.

[Documentation map](../README.md) · [Root README](../../README.md)

## What a page shows, and why

Every page shows **the whole territory, not only the rows the recipe touches.** An unchanged line is
evidence: a recipe that does the right thing *and something else* is the failure you cannot see from
inputs and outputs alone.

Quantities that are read rather than written say where they were read from, so a lookup is visible
rather than magic. **A named ingredient is shown by its name** - `$where`, `$from`, `$to` - because
naming the place is how a recipe says where it acts.

**Territory 1 is the setting throughout**, because it is the landing site and everything works there:
food, metal and energy each **3 extractors at density 4**, force of nature 1. Room: citizens 8,
garrison 1, yard 1, arks 2, pioneers 2, labor 8, and 20 each of food, metal and energy.

**A page ending in a defect line is not a failed page.** Finding those is the exercise, and each one
is filed rather than only noted here.

## The player's

### deploy ark

| territory 1 (`$where`) | before | after |
| ---------------------- | ------ | ----- |
| territory              | 1      | 1     |
| ark                    | 1      | 0     |
| garrison               | 0      | 1     |
| citizen                | 0      | 1     |
| extractor, food        | 0      | 1     |
| extractor, metal       | 0      | 1     |
| extractor, energy      | 0      | 1     |

**The territory is named and echoed**, so it is used and not consumed. The garrison line is a guard:
*at most 0* before, so a second landing on the same territory cannot fire. **Metal balances** - an Ark
binds with 4 and becomes a garrison and three extractors at 1 each.

**Note, not a defect.** The Ark's tank held energy and the Ark is consumed. Energy is neither
conserved nor expiring, so no rule is broken - but the player loses it, and `P-168`'s line says an
action that wastes something says so.

### move

| territories 1 and 2                  | before | after |
| ------------------------------------ | ------ | ----- |
| territory 1 (`$from`)                | 1      | 1     |
| territory 2 (`$to`, next to `$from`) | 1      | 1     |
| pioneer, in 1, ready                 | 1      | 0     |
| pioneer, in 2, exhausted, arriving   | 0      | 1     |
| energy, in that pioneer              | 2      | 1     |

**Both territories are named, echoed and unconsumed**, which is what stops a move from destroying
where it came from. **Adjacency is a condition the recipe states** - `next to $from` - read from the
planet, which says which of the things in it are next to which.

### found by land

| territory 2       | before | after |
| ----------------- | ------ | ----- |
| pioneer, arriving | 1      | 0     |
| garrison          | 0      | 1     |
| citizen           | 0      | 1     |
| extractor, food   | 0      | 1     |

**No territory is named and none is needed**: the Pioneer is in one, and `arriving` says it just got
there. **Metal balances** - a Pioneer binds with 2 and becomes a garrison and an extractor.

### build food extractor · build metal extractor · build energy extractor

| territory 1               | before | after |
| ------------------------- | ------ | ----- |
| labor                     | 1      | 0     |
| metal                     | 3      | 2     |
| extractor, metal          | 0      | 1     |
| room for metal extractors | 3      | 3     |

Three recipes differing in one cell. **The recipe acts where its ingredients are** - the labor and the
metal are in territory 1, so the extractor appears there, and no territory has to be named.

### build yard

| territory 1    | before | after |
| -------------- | ------ | ----- |
| metal          | 15     | 0     |
| yard           | 0      | 1     |
| room for yards | 1      | 1     |

**Defect.** Every other thing a player builds costs **labor and metal**; a Yard costs metal alone.
**Fifteen metal assembles itself.** Filed as `P-174`.

### produce pioneer

| territory 1 | before | after |
| ----------- | ------ | ----- |
| metal       | 2      | 0     |
| energy      | 6      | 0     |
| citizen     | 2      | 1     |
| garrison    | 1      | 1     |
| pioneer     | 0      | 1     |

The garrison is echoed and survives. **The citizen is not** - producing a Pioneer spends a person.

### produce ark

| territory 1 | before | after |
| ----------- | ------ | ----- |
| metal       | 4      | 0     |
| energy      | 12     | 0     |
| yard        | 1      | 1     |
| ark         | 0      | 1     |

**Metal balances and energy carries the price**, which is why an Ark costs 4 rather than 12: only
metal has to equal what the thing becomes.

### spend readiness

| territory 1        | before | after |
| ------------------ | ------ | ----- |
| citizen, ready     | 1      | 0     |
| citizen, exhausted | 0      | 1     |
| labor              | 0      | 1     |

### work

| territory 1 (`$where`)       | before | after |
| ---------------------------- | ------ | ----- |
| territory                    | 1      | 1     |
| labor                        | 1      | 0     |
| extractor, metal, ready      | 1      | 0     |
| extractor, metal, exhausted  | 0      | 1     |
| metal                        | 0      | 4     |
| `$where`'s density for metal | 4      | 4     |

**4 is read, not written.** The territory is named so that the density has something to be read from,
which is the whole of `P-173`. Three extractors at density 4 need three labor to yield 12.

## The world's, which fire when the turn ends

### eat

| territory 1 | before | after |
| ----------- | ------ | ----- |
| citizen     | 2      | 2     |
| food        | 5      | 3     |

Fires once per citizen. The citizen is echoed and the food is not.

### grow

| territory 1        | before | after |
| ------------------ | ------ | ----- |
| food, surplus      | 3      | 0     |
| citizen            | 2      | 5     |
| territory (houses) | 1      | 1     |

`surplus` is *left after everything ate*, so this cannot fire until `eat` has. **The territory is
echoed** - growing a citizen does not consume the place they live in.

### depart

| territory 1    | before | after |
| -------------- | ------ | ----- |
| citizen, unfed | 1      | 0     |
| citizen, fed   | 2      | 2     |

### spoil

| territory 1   | before | after |
| ------------- | ------ | ----- |
| food, surplus | 3      | 0     |

**Takes surplus, not food**, which is what orders it after eating - and it competes with `grow` for
the same surplus, which is the right relationship: surplus either becomes population or rots.

### ready

| territory 1          | before | after |
| -------------------- | ------ | ----- |
| citizen, exhausted   | 2      | 0     |
| citizen, ready       | 0      | 2     |
| extractor, exhausted | 1      | 0     |
| extractor, ready     | 0      | 1     |
| pioneer, arriving    | 1      | 1     |

**Defect.** *Traits* says `arriving` is *stored, cleared at end turn*, and **nothing clears it.**
`ready` restores readiness and says nothing about arriving, so a Pioneer that moved once is arriving
for ever. Filed as `P-175`.

### upkeep

| territory 1 | before | after |
| ----------- | ------ | ----- |
| pioneer     | 1      | 1     |
| food        | 3      | 2     |

**1 is read, not written** - the unit's upkeep.

**Not settled.** `upkeep` and `eat` both draw on food and nothing says which goes first. **A unit may
be unpaid because a citizen ate, or a citizen unfed because a unit was paid**, and this page cannot
show a definite after-state where food is short. In [the backlog](../notes/spec-backlog.md).

### perish

| territory 1     | before | after |
| --------------- | ------ | ----- |
| pioneer, unpaid | 1      | 0     |
| metal           | 0      | 2     |

**2 is read, not written** - the unit's metal, which under `P-170` is its binding plus its parts.
**The wreck is metal in the territory** and is kept only if there is room.

### revert

| territory 4     | before | after |
| --------------- | ------ | ----- |
| territory       | 1      | 1     |
| citizen         | 1      | 0     |
| force           | 0      | 0     |
| force of nature | 1      | 1     |

Fires once per citizen while the force is short. **Control is derived, so nothing writes it**: the
population is what nature takes, and control follows it out.

**Not settled.** The garrison, the extractors and whatever is stored are not mentioned. **They stay,
in a territory nobody holds.** In [the backlog](../notes/spec-backlog.md).
