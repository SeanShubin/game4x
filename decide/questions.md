# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-570 - may a territory omit a resource, or does it write the zero?

**to** sean · **status** open · **raised** 2026-09-26 · **asks** a decision · **kind** measured · **into** `spec/planet.md`

**Split out of `P-568`**, which now asks approval for the container unification and cannot also
ask a decision.

**`spec/planet.md`**: *For each resource, a territory has capacity for some number of extractors,
and a density that each of them yields.* **The main scenario's territory 1 had no energy density
at all** - metal and food on the surface, energy in the orbit - and nothing objected.

```
D1  "some number" may be zero, so a territory may omit a resource entirely
D2  every territory declares all three, and a zero is written rather than left out
```

**Measured: adding the density changes nothing observable**, because `deploy` makes two extractors
either way. **So this is about what the sentence means and not what it costs.**

## Why it is the same shape as the bug `P-568` fixes

**`D1` makes the sentence unfalsifiable.** If *some number* may be zero and a missing row means
zero, then every territory satisfies it, including one that declares nothing at all - **which is
exactly how a bin came to be built that could hold nothing.**

**`D2` makes it checkable**: three resources, three rows, and a territory short of one is a gap
a tool can name.

**This lane is not choosing**, because `D1` may be what you want - a territory with no energy is a
fact about that territory, and writing `-> 0` to say so is ceremony. The question is whether the
ceremony buys the check.
