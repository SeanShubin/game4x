# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-574 - adjacency is stated twice or travel goes one way, and `spec/` already forbids the first

**to** sean · **status** open · **raised** 2026-09-27 · **asks** a decision · **kind** entailed · **into** the schema, and `spec/planet.md` if you choose otherwise

**Your ark-move edit cannot be made without contradicting something.** The code lane filed `C-153`
and restored the scenario unedited.

**What made it visible**: the ark moves from the orbit above territory 1 to the orbit above
territory 2, and the pioneer then settles the territory the ark came from - **two things travelling
opposite ways along one border.** Nothing had ever done that.

## Three sentences, all promoted, and no two of them hold together

```
spec/planet.md:26    two territories are adjacent when they SHARE an edge
spec/planet.md:32    adjacency is a SHARED boundary. Two places are adjacent when they share one
spec/invariants.md   a fact is stated once and every other form of it is derived
```

**Sharing is symmetric.** The data states it directed - `{adjacency id:1 from:territory-1
to:territory-2}` - and `move` matches that direction and no other. **So one row means travel one
way, and two rows state one fact twice.**

## This lane told you the wrong thing an hour ago and is correcting it here

**It offered you *one row derived, or two rows with a check that they agree*. The second is not
available**: two rows is one fact stated twice, which `spec/invariants.md` forbids, and it
contradicts *shared* twice in `spec/planet.md`. **A check that the two rows agree would be a check
that a rule is not being broken, which is not the same as not breaking it.**

## Never exercised, which is why it surfaced now and not months ago

**The code lane measured over everything you have read**: 12 reviewed tests declare an adjacency,
15 moves across them, **15 with the declared direction and 0 against**. The reviewed worlds are
chains and everything walks forward along them. **It corrected its own instrument first** - a
version that could not map a place to its territory reported 13 and 13 - and the count above has
0 unmapped and its three outcomes sum to the total.

## So the decision is where symmetry lives, and both keep one stated fact

```
S1  the schema says the relation is symmetric, and the engine matches either
    direction for any relation so declared. `{symmetric relation:adjacency}`,
    the same shape as `{family relation:unit}` which already exists
S2  a world states one row and something derives the other before the engine
    sees it, the way capacity, occupied and free are three names and two facts
```

**`S1` is a notation feature and `S2` is a derivation step.** `S1` is smaller and general - any
future *shares a* relation gets it free - and it needs the engine to honour a declared property
rather than to know anything about adjacency, **which matters because `docs/architecture.md` rule
11 says the engine names no noun the game has.**

## The third answer the code lane found and deliberately did not take

**A third territory in a chain lets the ark move forward and the pioneer move forward again.** It
contradicts nothing, works today, and **settles the question by avoiding it** - which is why it
was filed rather than quietly done. **It is still available** if you would rather have the
scenario now and the decision later, and this lane would say the decision is worth taking first
because the next world you draw will meet it again.
