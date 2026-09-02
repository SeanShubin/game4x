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
rather than magic - `the territory's density for that resource` is a number this page had to fetch.

**Territory 1 is the setting throughout**, because it is the landing site and everything works there:
food, metal and energy each **3 extractors at density 4**, force of nature 1. Room comes from
[what a territory has room for](../../releases/first-release.md): citizens 8, garrison 1, yard 1,
arks 2, pioneers 2, labor 8, and 20 each of food, metal and energy.

**A page ending in a defect line is not a failed page.** Finding those is what the exercise is for,
and each one is filed rather than only noted here.

## The player's, in the order the release lists them

### deploy ark

| territory 1 | before | after    |
| ----------- | ------ | -------- |
| ark         | 1      | 0        |
| garrison    | 0      | 1        |
| citizen     | 0      | 1        |
| extractor   | 0      | 1 (food) |
| metal       | 0      | 0        |

**Defect.** An Ark holds 12 metal. What it becomes holds 2 - a garrison and an extractor at one
each. **Ten metal leaves the game**, and `spec/resources.md` says metal is conserved and what it was
made into can be taken apart to get it back. Filed as `P-165`.

### move

| territory 1 → 2      | before  | after       |
| -------------------- | ------- | ----------- |
| pioneer, in 1        | 1 ready | 0           |
| pioneer, in 2        | 0       | 1 exhausted |
| energy, in that unit | 2       | 1           |

**Defect.** Nothing in the recipe says territories 1 and 2 are adjacent, and since `P-158` removed
the Scope column nothing says what `here` and `there` are. A source and a destination are parameters
of a recipe, and the table has no column for one. Filed as `P-166`.

**And a second one.** `found by land` needs `pioneer, arriving`, and **no recipe produces it.** A
Pioneer that moves in is not marked as having arrived. Filed with the above.

### found by land

| territory 2 | before     | after    |
| ----------- | ---------- | -------- |
| pioneer     | 1 arriving | 0        |
| garrison    | 0          | 1        |
| citizen     | 0          | 1        |
| extractor   | 0          | 1 (food) |

**Defect.** A Pioneer holds 8 metal and what it becomes holds 2. **Six metal leaves the game**, the
same way `deploy ark` loses ten. One filing covers both.

### build extractor

| territory 1               | before | after |
| ------------------------- | ------ | ----- |
| labor                     | 1      | 0     |
| metal                     | 3      | 3     |
| extractor                 | 0      | 1 (?) |
| room for metal extractors | 3      | 3     |

**Two defects, and the page cannot be completed without them.** *Units and structures* says an
extractor costs **1 labor and 1 metal**; the recipe takes labor only, so the two tables in one file
disagree. And the recipe does not say **which resource** the extractor is for, though `work` reads
that trait to know what it produces. Filed as `P-167`.

### build yard

| territory 1    | before | after |
| -------------- | ------ | ----- |
| metal          | 15     | 0     |
| yard           | 0      | 1     |
| room for yards | 1      | 1     |

Agrees with *Units and structures*: 15 metal. **Nothing checks the room**, which is general rather
than particular to this recipe - see the note at the end.

### produce pioneer

| territory 1 | before | after |
| ----------- | ------ | ----- |
| metal       | 8      | 0     |
| energy      | 6      | 0     |
| citizen     | 2      | 1     |
| garrison    | 1      | 1     |
| pioneer     | 0      | 1     |

The garrison is echoed, so it survives. Metal is conserved exactly: 8 in, and a Pioneer holds 8.

### produce ark

| territory 1 | before | after |
| ----------- | ------ | ----- |
| metal       | 12     | 0     |
| energy      | 12     | 0     |
| yard        | 1      | 1     |
| ark         | 0      | 1     |

Conserved exactly: 12 in, and an Ark holds 12.

### spend readiness

| territory 1        | before | after |
| ------------------ | ------ | ----- |
| citizen, ready     | 1      | 0     |
| citizen, exhausted | 0      | 1     |
| labor              | 0      | 1     |

### work

| territory 1          | before    | after     |
| -------------------- | --------- | --------- |
| labor                | 1         | 0         |
| extractor, ready     | 1 (metal) | 0         |
| extractor, exhausted | 0         | 1 (metal) |
| metal                | 0         | 4         |
| density for metal    | 4         | 4         |

**4 is read, not written** - the territory's density for metal. Three extractors at density 4 need
three labor to yield 12, which is the arithmetic `P-156` rests on.

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

`surplus` is what is left after everything ate, so this fires after `eat`. **Three surplus food
becomes three citizens**, which is the growth rate `P-156`'s citizen room of 8 has to hold.

### depart

| territory 1    | before | after |
| -------------- | ------ | ----- |
| citizen, unfed | 1      | 0     |
| citizen, fed   | 2      | 2     |

### spoil

| territory 1 | before | after |
| ----------- | ------ | ----- |
| food        | 3      | 0     |

Fires once per food, everywhere, so all uneaten food goes. It runs after `grow`, which is why
surplus becomes citizens rather than being lost.

### ready

| territory 1          | before | after |
| -------------------- | ------ | ----- |
| citizen, exhausted   | 2      | 0     |
| citizen, ready       | 0      | 2     |
| extractor, exhausted | 1      | 0     |
| extractor, ready     | 0      | 1     |

Applies to `thing`, the family, so it covers every kind that readies at once.

### upkeep

| territory 1 | before | after |
| ----------- | ------ | ----- |
| pioneer     | 1      | 1     |
| food        | 3      | 2     |

**1 is read, not written** - the unit's upkeep. A unit with upkeep 2 eats two, which is the bug
`P-142` fixed by making it a lookup.

### perish

| territory 1     | before | after |
| --------------- | ------ | ----- |
| pioneer, unpaid | 1      | 0     |
| metal           | 0      | 8     |

**8 is read, not written** - the unit's metal. The wreck is metal in the territory and is kept only
if there is room for it, which is what makes salvage a thing a player could be given rather than
something the engine knows.

### revert

| territory 4     | before | after |
| --------------- | ------ | ----- |
| citizen         | 1      | 0     |
| force           | 0      | 0     |
| force of nature | 1      | 1     |

Fires once per citizen while the force is short. Control is derived, so nothing writes it: **the
population is what nature takes, and control follows it out.**

## One defect that belongs to no single page

**Nothing checks room.** `build yard`, `produce pioneer`, `produce ark`, `build extractor`, `grow`
and `deploy ark` all create things, and not one of them asks whether there is room. `spec/turn.md`
says what a territory can keep is bounded and anything above the bound is lost when the turn ends -
**so the bound is enforced at the end of the turn and never at the moment of creation.**

That may be intended: build what you like, lose what will not fit. It may also mean a player can
build a second Yard where room is 1 and lose one of them at random. **Nothing says which**, and it is
the one question these seventeen pages raise that none of them contains. Filed as `P-168`.
