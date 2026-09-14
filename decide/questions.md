# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-510 - Pooling cannot be universal, because an orbit holds no energy

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

## This lane's reading, and it is not confident

**`G3`.** It is the only one under which *a resource is in the place* has no exceptions, and the
sentence it changes is a release's rather than the specification's - `spec/orbit.md` forbids
extraction and says nothing about holding. **But it makes an orbit a place things are stored**, and
whether that is a game you want is not something this lane can read off a file.

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
