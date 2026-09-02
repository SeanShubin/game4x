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

**Sixteen recipes.** There were twenty this morning. `launch` was `move` with a different
destination, `eat` was `upkeep`, `depart` was `perish`, `revert` could never fire, and `land` became
`deploy ark` - while `build extractor` became three. **Every one of those was a collapse rather than
a cut.**

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

**The territory is named and echoed**, so it is used and not consumed. The garrison line is a guard -
*at most 0* - so a second landing on the same territory cannot fire. **Metal balances**: an Ark binds
with 4 and becomes a garrison and three extractors at 1 each.

### move

| territories 1 and 2                  | before | after |
| ------------------------------------ | ------ | ----- |
| territory 1 (`$from`)                | 1      | 1     |
| territory 2 (`$to`, next to `$from`) | 1      | 1     |
| pioneer, in 1, ready                 | 1      | 0     |
| pioneer, in 2, exhausted             | 0      | 1     |
| energy, in that pioneer              | 2      | 1     |

**Both territories are named, echoed and unconsumed**, which is what stops a move destroying where it
came from. **Adjacency is a condition the recipe states** - `next to $from` - read from the planet,
which says which of the things in it are next to which.

### found by land

| territory 2     | before | after |
| --------------- | ------ | ----- |
| pioneer         | 1      | 0     |
| garrison        | 0      | 1     |
| citizen         | 0      | 1     |
| extractor, food | 0      | 1     |

**A Pioneer and no garrison is the whole condition.** A Pioneer is only ever produced where there is
a garrison, so one standing where there is none must have moved there - which is why `arriving` was
deleted. **Metal balances**: a Pioneer binds with 2 and becomes a garrison and an extractor.

**If it does not found, it starves.** Territory 2 has no extractor and therefore no food, so `upkeep`
cannot pay the Pioneer and `perish` takes it at the end of that same turn.

### build food extractor · build metal extractor · build energy extractor

| territory 1               | before | after |
| ------------------------- | ------ | ----- |
| labor                     | 1      | 0     |
| metal                     | 3      | 2     |
| extractor, metal          | 0      | 1     |
| room for metal extractors | 3      | 3     |

Three recipes differing in one cell. **The recipe acts where its ingredients are** - the labor and the
metal are in territory 1, so the extractor appears there and no territory has to be named.

### build yard

| territory 1    | before | after |
| -------------- | ------ | ----- |
| labor          | 1      | 0     |
| metal          | 15     | 0     |
| yard           | 0      | 1     |
| room for yards | 1      | 1     |

**Labor and metal, like everything else a player builds.** Until today it took metal alone, and
fifteen metal assembled itself.

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

**4 is read, not written.** The territory is named so the density has something to be read from.
Three extractors at density 4 need three labor to yield 12.

## The world's, which fire when the turn ends

### upkeep

| territory 1 | before | after |
| ----------- | ------ | ----- |
| citizen     | 2      | 2     |
| pioneer     | 1      | 1     |
| food        | 5      | 2     |

**One recipe feeds everything.** A citizen's upkeep is 1 food and a Pioneer's is 1, so three things
with upkeep take three food. **`eat` was this recipe with a citizen's upkeep assumed rather than
written.**

### grow

| territory 1        | before | after |
| ------------------ | ------ | ----- |
| food, surplus      | 2      | 0     |
| citizen            | 2      | 4     |
| territory (houses) | 1      | 1     |

`surplus` is *left after every upkeep was paid*, so this cannot fire until `upkeep` has. **The
territory is echoed** - growing a citizen does not consume the place they live in.

### spoil

| territory 1   | before | after |
| ------------- | ------ | ----- |
| food, surplus | 2      | 0     |

**Takes surplus, not food**, which is what orders it after upkeep - and it competes with `grow` for
the same surplus, which is the right relationship: surplus either becomes population or rots.

### ready

| territory 1          | before | after |
| -------------------- | ------ | ----- |
| citizen, exhausted   | 2      | 0     |
| citizen, ready       | 0      | 2     |
| extractor, exhausted | 1      | 0     |
| extractor, ready     | 0      | 1     |

Applies to `thing`, the family, so it covers every kind that readies at once.

### perish

| territory 1     | before | after |
| --------------- | ------ | ----- |
| pioneer, unpaid | 1      | 0     |
| metal           | 0      | 2     |

**2 is read, not written** - the thing's metal, which is its binding plus the metal in its parts.
**The wreck is metal in the territory** and is kept only if there is room.

**Not settled.** This recipe also takes an unpaid **citizen**, and a citizen's *Metal in it* is blank
rather than zero. **What a blank yields is not stated** - filed as `P-181`.
