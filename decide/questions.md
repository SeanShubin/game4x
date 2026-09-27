# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-578 - `P-577` reddened 35 tests, and the shape this lane rejected costs nothing

**to** sean · **status** open · **raised** 2026-09-27 · **asks** a decision · **kind** measured · **into** `spec/data/schema.4x`, and whether `P-577` stands

**The biome column made 380 existing rows illegal.** A column is not optional:
`Schema::fits` compares the key sets with `sorted != given` - **exact equality** - and `Game::of`
validates every row before any rule runs. **So `{territory id:1}` stopped being a legal row the
moment the column landed**, and *no rule reads a biome* is true and is not what refuses them.

```
reviewed/                 126 territory rows   0 carrying a biome
spec/tests/               126                  0
data/foundation/tests/    126                  0
scenario/                   2                  0
                          380                  0
```

**Re-derived here, not taken from the code lane's report.** 35 tests are red.

## Two errors of this lane's in one proposal, and the first is the one to learn from

**The `layer` argument measured the shape and was read as covering the arrival.** *436 of 436 place
rows carry a layer* - **because the column has been there since places had rows**. *380 of 380
territory rows carry no biome* - **because the column arrived today**. **The analogy was true about
the shape and said nothing about the migration**, and this lane offered it as if it had said both.

**And it rejected the cheap shape on a ground `layer` itself refutes.** `P-577` dismissed a biome
relation because `grassland` would be *the first value in the schema that is only itself*. **`layer`
is already exactly that** - no `{reference}` names column 136. **So the objection was false when it
was written**, and the file this lane was quoting was the refutation.

## The decision

```
A  every territory row states a biome. 380 rows: 126 in `reviewed/`, which no
   instance may write, so 54 tests come back through the review application and
   you read them again. And which biome each territory is, is your content
C  a biome is its own relation keyed by territory - {biome territory:1
   what:grassland}. Changes no existing row, reddens nothing, and `P-577` is
   reversed. The column goes
```

**This lane says `C`**, and would have said it in `P-577` if it had costed the migration instead of
admiring the analogy. **`B` is not available** - a column that may be absent is the null you ruled
out on 2026-09-26.

**What `C` costs**: one relation and two columns rather than one column, and a biome is a row
rather than a field. **What `A` costs**: you re-reading 54 tests, and deciding twelve biomes.

**The gate stays red until you answer**, and the code lane committed the regeneration deliberately
so the red is the true state rather than a stale file hiding it.
