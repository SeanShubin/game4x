# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-568 - a bin that holds nothing is legal, and two rules about what the data must declare cannot be broken

**to** sean · **status** open · **raised** 2026-09-26 · **asks** a decision · **kind** measured · **into** `spec/logistics.md`, and `spec/planet.md`

**The scenario built a bin for metal and lost all five metal that turn.** `build-bin` succeeded and
`discard-disorder` took the metal with the bin standing there. The missing row was
`{capacity of:bin for:resource what:resource per:place} -> 10`.

**`spec/logistics.md` is not missing the distinction and states it well.** The code lane had the
room-to-stand row and not the how-much-it-holds row, and `spec/data/schema.4x` shows the pair
together in your own example. **What no rule says is that a store must have the second.**

## Why the rule that looks like it covers this cannot be broken

```
spec/logistics.md   "A kind declares one of three things about what it may hold" -
                    no capacity, a limit, or no limit
you, 2026-09-18     "If we omit a capacity, we can default that to mean it may carry
                    none of that thing"
```

**Together, a kind that declares nothing is declaring the first of the three.** So every kind that
could ever exist satisfies the sentence, including one with no rows at all - **and a bin with no
capacity row is not breaking a rule, it is using a default.**

**This is not a useless sentence and that is worth saying.** It defines the three states and tells
a reader that *empty* and *cannot hold* are different, which is load-bearing. **A definition is not
violated, it is used** - so the gap is not that this rule is weak, it is that no rule forbids what
happened.

## The first decision

```
B1  a kind that is a store declares a capacity, and omitting one is a gap rather
    than a declaration. The default stays for kinds that are not stores
B2  building a thing that can hold nothing of what it is for is refused
B3  neither - a bin with no capacity is a legal thing that holds nothing, and what
    catches it is a person reading the scenario
```

**Neither this lane nor the code lane can pick from the data.** An extractor stands in a deposit
and holds nothing, and that is correct - so a check that flagged every kind with no capacity would
be guessing which kinds are meant to be stores.

## The second, found while looking at the first

**`spec/planet.md`**: *For each resource, a territory has capacity for some number of extractors,
and a density that each of them yields.* **The scenario's territory 1 had no energy density at all**
- metal and food on the surface, energy in the orbit - and nothing objected.

```
D1  "some number" may be zero, so a territory may omit a resource entirely
D2  every territory declares all three, and zero is written rather than left out
```

**Measured: adding the density changes nothing observable**, because `deploy` makes two extractors
either way. **So this is about what the sentence means and not what it costs.**

## Why it is yours

**Both are about what the game is**, not about how it is checked. Either answer makes the rule
checkable; this lane will not choose which rule the check should assert.
