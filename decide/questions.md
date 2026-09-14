# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-510 - Pooling cannot be universal, and the choice is now two

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `P-509`, before it is promoted

**`P-509` says a resource is in the place and the things in it hold nothing.** Two lines of the
release say that cannot be true everywhere, and which way it gives is yours.

```
releases/first-release.md:94   An orbit holds units and nothing else.
releases/first-release.md      | move | consume | 1 | energy | | that unit |
```

**A move burns energy the unit carries.** An ark sitting in an orbit is in a place that can hold no
energy, so **there is nothing to pool with** - and if it holds nothing itself, it can never move
again.

## The three ways, and the third is the one that keeps `P-509` whole

**`G1` - a unit holds fuel; everything else pools.**

```
{orbit id:4}
  {ark}  {energy} -> 2

{territory id:1}
  {metal} -> 25              pooled: no transport holds any of it
  {transport resource:metal} -> 2
```

Cargo behaves as `P-509` says and fuel does not. **Two rules for storage**, which is the thing you
said you would rather over-specify than have.

**`G2` - a thing pools what its place can hold, and holds what its place cannot.**

```
{territory id:1}                {orbit id:4}
  {energy} -> 5                   {ark}  {energy} -> 2
  {ark}                           

```

One rule with a condition. **An ark holds fuel in orbit and holds none in a territory**, so the same
thing reads differently in two places and *when does a unit hold something* has no short answer.

**`G3` - an orbit can hold resources, and pooling is universal.**

```
{orbit id:4}
  {energy} -> 2
  {ark} -> 1
```

`releases/first-release.md:94` becomes *an orbit holds units and the resources they carry*, and
**`P-509` needs no exception at all**. The cost is a change to what an orbit is, which is a rule
rather than a shape - and `spec/orbit.md` says only that an orbit *has capacity for no extractors,
and nothing is extracted there*, which does not forbid it.

## The difference, in the notation, and it is two states and one command

**They agree everywhere a place can hold the kind.** In a territory the two are the same bytes, so
every example so far has failed to separate them. **An orbit is where they part.**

## One ark in orbit, carrying two energy

```
G2                          G3
{orbit id:4}                {orbit id:4}
  {ark}                       {ark} -> 1
    {energy} -> 2             {energy} -> 2
```

**Under `G2` the energy is inside the ark; under `G3` it is in the orbit** and the ark's tank is what
gave the orbit the room.

## Two arks, one with two energy and one with one

```
G2                          G3
{orbit id:4}                {orbit id:4}
  {ark}                       {ark} -> 2
    {energy} -> 1             {energy} -> 3
  {ark}
    {energy} -> 2
```

**`G2` has two entries and `G3` has one.** Under `G2` they share a description and are told apart
only by their contents, which is the whole of what `P-507` could not write and `P-508` needed
positions for. **Under `G3` there is nothing to tell apart** - two arks, three energy, and a
capacity of four.

## And the command that cannot be written

```
G2   {move unit:???  to:5 haul-energy:2}    which ark? both are `{ark}`
G3   {move unit:ark  to:5 haul-energy:2}    either; they are the same
```

**That is the cost of `G2` in one line.** Not that orbits are odd, but that **the problem this week
was spent on comes back inside them.**

## The same ark, before and after landing

```
G2                                    G3
{orbit id:4}                          {orbit id:4}
  {ark}                                 {ark} -> 1
    {energy} -> 2                       {energy} -> 2

{move unit:ark to:1 haul-energy:2}

{territory id:1}                      {territory id:1}
  {energy} -> 7                         {energy} -> 7
  {ark} -> 1                            {ark} -> 1
```

**Both end in the same place.** Under `G2` the ark's contents emptied into the pool on arrival and
under `G3` nothing happened, because they were pooled already. **`G2` is a thing that changes shape
depending on where it stands; `G3` is a thing that never holds anything.**

## `G1` is eliminated, 2026-09-14, by Sean's own constraint

**Sean**: *I don't want the gas tank to be special in mechanics, only in defaults.*

**`G1` is exactly that specialness.** Fuel held and cargo pooled is two mechanics, chosen by which
resource it is. **Withdrawn**, and not on taste - it is the one thing he ruled out by name.

**`G2` and `G3` both survive it**, because neither is about the resource: `G2` branches on the
place, `G3` branches on nothing. **A gas tank under either is a store like any other, and the only
thing left that is fuel-shaped is a default** - *haul most* topping one off because you were picking
up.

## `G3` needs no capacity declared, which this lane did not see the first time

**Under pooling a place's capacity is the sum of what is in it that can hold the kind.** An orbit
holding an ark whose fuel store is 2 **already has an energy capacity of 2**, derived, with nothing
declared:

```
{orbit id:4}
  {energy} -> 2
  {ark} -> 1        its fuel store is what gives the orbit the room
```

**An empty orbit holds nothing because it can hold nothing** - capacity 0, no rule needed. So the
release's *an orbit holds units and nothing else* becomes *an orbit holds units, and what they can
hold*, and **nothing anywhere declares an orbit a store.**

## Sean's 500 / 500 / 10 case, which `P-509` already gets right

```
{territory id:1}                       {territory id:2}
  {metal} -> 500                         {metal} -> 0
  {store resource:metal} -> 50           {store resource:metal} -> 50
  {transport resource:metal} -> 1
```

```
{move unit:transport to:2 haul-metal:10}
```

**Ten, and only ten, however much room is at either end.** `P-509` bounds a haul by *its own capacity
for that kind* and by nothing else, so the destination having 500 free changes nothing. **No option
here affects it** - it is `P-509`'s rule rather than this choice.

## One consequence of pooling worth seeing before you choose

**Capacity can walk away.** A territory with 500 fixed storage and a transport standing in it has a
capacity of 510. If it holds 505 and the transport leaves carrying nothing:

```
{territory id:1}
  {metal} -> 505      capacity is now 500, so five is lost at the turn's end
```

**That is correct rather than a bug** - the five had nowhere to be - and it is exactly what the
waste check is for. **It is also a thing a player can do to themselves by accident**, which is the
argument for warning on it.

## This lane's reading, and it is not confident

**`G3`, and more strongly than before.** It is the only one under which *a resource is in the place*
has no exceptions; the sentence it changes is a release's rather than the specification's; and its
capacity turns out to be derived rather than declared, so **it adds no rule at all** - it removes
one.

**`G2` is the live alternative** and its cost is now clearer: an ark in orbit holds its own contents,
so everything `P-507` and `P-508` were built to solve - describing a container by what it holds,
telling two alike things apart - **comes back, in orbits only.** One place where the rules differ is
the whole of what `G2` buys over `G3`.

## And one smaller thing `P-509` left open, which the same choice settles

**`spec/logistics.md`: *a rule may ask whether something is absent only where what would hold it
declares a limit for it*** - and a territory declares no limit for a resource.

**Under `P-509` the limit stops being about the container and becomes about the place**: what a place
holds beyond the capacity in it is lost, and that capacity is a number the place has. **So the
territory does declare a limit** - a derived one - and the sentence is satisfied rather than
strained. `free metal of {territory id:1}` then means what it says.

**That holds under all three**, so it is settled by `P-509` landing rather than by this choice. It is
written here because `P-509` names it as open and this is the answer to it.
*Nothing is open.*
