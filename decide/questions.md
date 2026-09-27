# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-577 - a biome is a label and everything else in the schema is a thing in a place

**to** sean · **status** open · **raised** 2026-09-27 · **asks** a decision · **kind** measured · **into** `spec/data/schema.4x`

**Carved out of `P-575` while carrying it out.** The other four rules need no decision from you;
`set-biome` does, because **the schema has no shape for a label.**

**`spec/planet.md` names all six** - *the biomes are ocean, ice, desert, grassland, jungle and
mountain* - so nothing here invents content. **What is missing is how a territory carries one.**

## Why the obvious pattern does not fit

**`resource` is the closest thing and it is the wrong shape.** `{member kind:metal family:resource}`
makes `metal` a relation, and a metal is a **counted thing standing in a place**:
`{metal where:place-1 quantity:3}`. **A biome is none of that** - there is one per territory, it is
not in a place, there is no quantity of it, and nothing spends it.

**`primitive` is the engine's own vocabulary** - `add`, `clause`, `quantity` - not the game's, so it
is not that either. **`trait` is an allowance with a count**, which a biome is not.

## Three shapes

```
B1  one relation, the biome as a word    {terrain territory:1 what:grassland}, and
                                         `grassland` denotes nothing - the first value in
                                         the schema that is only itself
B2  six member relations of a family     {member kind:grassland family:biome}, matching
                                         resource exactly, and each carries a territory
                                         rather than a place and a quantity
B3  a column on territory                {territory id:1 biome:grassland}, which is the
                                         smallest and makes a biome a fact about a
                                         territory rather than a row of its own
```

**`B3` is smallest and `B2` is most like what is there.** `B1` introduces a value that denotes
nothing, which is new in this schema and is the thing `P-573` and the `argument` machinery both
work hard to avoid.

## What decides it, and it is your own sentence

**`spec/planet.md`**: *a biome earns its place by being shown rather than by being obeyed.* **No rule
reads it, ever** - it is what the realistic drawing is made of. **So the cheapest shape that a
drawing can read is the right one**, and this lane would say `B3` on that ground alone.

**Nothing waits on this.** The other four rules and `create planet` do not need a biome, and the
game plays without one.
