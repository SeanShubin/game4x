# Recipes, before and after

**One section per recipe** in [the first release](../../releases/first-release.md), all in
this file, each showing a state the recipe can fire in and the state after it fires. **The tables are the data and this is a
rendering of them**; where the two disagree, the release is right and a line here is a defect.

[Documentation map](../README.md) · [Root README](../../README.md)

## What a section shows, and why

Every section shows **the whole territory, not only the rows the recipe touches.** An unchanged line is
evidence: a recipe that does the right thing *and something else* is the failure you cannot see from
inputs and outputs alone.

**The release's table has seven columns now** - `Recipe`, `Auto`, `Role`, `Qty`, `Kind`, `Traits`
and `Where` - and `Role` is where a row says whether a thing is required, limited, consumed or
produced. These sections show what a recipe does to a territory, so they are unchanged by that except
where the old shape was being explained.

Quantities that are read rather than written say where they were read from, so a lookup is visible
rather than magic. **A named ingredient is shown by its name** - `$where`, `$from`, `$to` - because
naming the place is how a recipe says where it acts.

**Territory 1 is the setting throughout**, because it is the landing site and everything works there:
food, metal and energy each **3 extractors at density 4**, force of nature 1. Room: citizens 8,
garrison 1, yard 1, arks 2, pioneers 2, labor 8, and 20 each of food, metal and energy.

**Seventeen recipes.** There were twenty two mornings ago. `launch` was `move` with a different
destination, `eat` was `upkeep`, `depart` was `perish`, `revert` could never fire, and `land` became
`deploy ark` - while `build extractor` became three and `age` arrived with spoilage. **Every deletion
was a collapse rather than a cut.**

## The player's

### deploy ark

| territory 1 (`$where`) | before | after |
| ---------------------- | ------ | ----- |
| territory              | 1      | 1     |
| ark                    | 1      | 0     |
| garrison               | 0      | 1     |
| citizen                | 0      | 2     |
| extractor, food        | 0      | 1     |
| extractor, metal       | 0      | 1     |

**The territory is `require`d**, which is how a row says the recipe uses a thing without taking it -
there is no second row echoing it back. The garrison line is a `limit` of 0, so a second landing on
the same territory cannot fire.

**Two citizens, because two things have to be operated at once.** One works the garrison so the
territory's force of nature does not overrun the landing, and one works the food extractor so it does
not starve. **A third would have nothing to do**: no energy extractor is deployed, because it can be
built later out of what the first two produce.

**One of each extractor however much room there is.** Territory 1 has space for three of each and
gets one, so the landing is the same everywhere - the difference between a rich territory and a poor
one shows up in what is built afterwards, not in what arrives.

**Metal balances**: an Ark binds with 3 and becomes a garrison and two extractors at 1 each. Where a
territory has no room for one of them - territory 6 has no metal - that extractor is not built and
its metal is wasted, and `spec/interface.md` requires the interface to say so before the landing is
taken.

### move

| territories 1 and 2                  | before | after |
| ------------------------------------ | ------ | ----- |
| territory 1 (`$from`)                | 1      | 1     |
| territory 2 (`$to`, next to `$from`) | 1      | 1     |
| pioneer, in 1, ready                 | 1      | 0     |
| pioneer, in 2, exhausted             | 0      | 1     |
| energy, in that pioneer              | 2      | 1     |

**The recipe is over `unit`, and the Pioneer here is one** - an Ark crossing a boundary by land is
the same recipe with a different unit in it.

**Both territories are named and `require`d rather than consumed**, which is what stops a move
destroying where it came from. **Adjacency is a condition the recipe states** - `next to $from` - read from the planet,
which says which of the things in it are next to which.

### found by land

| territory 2      | before | after |
| ---------------- | ------ | ----- |
| pioneer          | 1      | 0     |
| garrison         | 0      | 1     |
| citizen          | 0      | 2     |
| extractor, food  | 0      | 1     |
| extractor, metal | 0      | 1     |

**A Pioneer and no garrison is the whole condition.** A Pioneer is only ever produced where there is
a garrison, so one standing where there is none must have moved there - which is why `arriving` was
deleted.

**A Pioneer deploys exactly what an Ark does**, which is why both bind with 3: a garrison and two
extractors at 1 each. Territory 2 has room for two metal extractors at density 4, so both are built
here.

**If it does not found, it starves.** Territory 2 has no extractor until the moment it founds, so a
Pioneer that arrives and does nothing cannot be paid by `upkeep`, and `perish` takes it at the end of
that same turn.

### build food extractor · build metal extractor · build energy extractor

| territory 1               | before | after |
| ------------------------- | ------ | ----- |
| labor                     | 1      | 0     |
| metal                     | 3      | 2     |
| extractor, metal          | 0      | 1     |
| room for metal extractors | 3      | 3     |

Three recipes differing in one cell. **Every row's `Where` is blank, which means the one place the
recipe acts** - so the labor and the metal are in one territory and the extractor appears there,
without any territory having to be named. That blank is what `P-190` added, and what `P-183` died
asking for.

### build yard

| territory 1    | before | after |
| -------------- | ------ | ----- |
| labor          | 1      | 0     |
| metal          | 15     | 0     |
| yard           | 0      | 1     |
| room for yards | 1      | 1     |

**Labor and metal, like everything else a player builds.** Until recently it took metal alone, and
fifteen metal assembled itself.

### produce pioneer

| territory 1 | before | after |
| ----------- | ------ | ----- |
| metal       | 3      | 0     |
| energy      | 6      | 0     |
| citizen     | 2      | 0     |
| pioneer     | 0      | 1     |

**Two citizens leave and two arrive.** A Pioneer carries the people who will operate what it deploys,
so founding a territory does not conjure a population out of metal - it moves one.

**No garrison is required.** One is usually standing there, because a garrison is the first thing a
landing deploys - but that is a coincidence of geography rather than a condition, and the recipe no
longer pretends otherwise.

### produce ark

| territory 1 | before | after |
| ----------- | ------ | ----- |
| metal       | 3      | 0     |
| energy      | 12     | 0     |
| citizen     | 2      | 0     |
| yard        | 1      | 1     |
| ark         | 0      | 1     |

**The same metal and the same citizens as a Pioneer, and twice the energy.** The two deploy the same
things, so they must bind with the same metal; what the extra energy buys is a Yard to build it in
and the ability to invade land from orbit.

### create labor

| territory 1        | before | after |
| ------------------ | ------ | ----- |
| citizen, ready     | 1      | 0     |
| citizen, exhausted | 0      | 1     |
| labor              | 0      | 1     |

**Named for what it makes.** It was `spend readiness`, which named what it costs - the only recipe in
the release named for its input. **The labor comes from a citizen here and need not always**;
widening it is a later decision, and the name already survives it.

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
territory is `require`d** - growing a citizen does not consume the place they live in.

### perish

| territory 1     | before | after |
| --------------- | ------ | ----- |
| pioneer, unpaid | 1      | 0     |
| metal           | 0      | 3     |

**3 is read, not written** - the thing's metal, which is its binding plus the metal in its parts.
**The wreck is metal in the territory** and is kept only if there is room.

**It also takes an unpaid citizen, and yields nothing for one.** A citizen's *Metal in it* is blank,
and `P-181` settled what that means: **a blank is not a zero.** It says the row has no such number,
and a quantity read from one produces nothing.

### spoil

| territory 1   | before | after |
| ------------- | ------ | ----- |
| food, keeps 0 | 2      | 0     |

**Takes food that has run out of turns**, not surplus. Until spoilage was written this took every
surplus food every turn, which is a spoilage rate of zero - the harshest setting there is, arrived at
by not choosing one.

### age

| territory 1   | before | after |
| ------------- | ------ | ----- |
| food, keeps 1 | 3      | 0     |
| food, keeps 0 | 0      | 3     |

**The number that decrements.** Food is made with `keeps` 1, so it survives one ending and is lost at
the next - one turn more than it used to last.

**Order is what makes it exactly one turn.** `spoil` runs first and takes what is already at zero,
then `age` moves this turn's food down to zero, then `ready` restores what was spent. Food made this
turn is therefore never taken by the ending that immediately follows it.

**Preservation is a technology, and a technology here is a recipe you have.** `age` being present from
the start is what *we already know how to keep food for one turn* means, and a civilisation that
later keeps it longer has a different recipe rather than a different rule.

### ready

| territory 1          | before | after |
| -------------------- | ------ | ----- |
| citizen, exhausted   | 2      | 0     |
| citizen, ready       | 0      | 2     |
| extractor, exhausted | 1      | 0     |
| extractor, ready     | 0      | 1     |

Applies to `thing`, the family, so it covers every kind that readies at once.
