# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-577 - `B3`: a biome is a column on a territory, the way a layer is a column on a place

**to** sean · **status** open · **raised** 2026-09-27 · **answered** 2026-09-27, `B3` · **asks** approval · **kind** entailed · **shape** an instruction · **into** `spec/data/schema.4x`

**You chose `B3`, and it turns out not to be a new shape at all.** `place` already has one:

```
{column id:135 relation:place seq:2 name:of}      {reference id:70 column:135 to:territory}
{column id:136 relation:place seq:3 name:layer}   no reference row at all
```

**`of` denotes a territory and `layer` denotes nothing** - `surface` and `orbit` are bare words in
a column, and no `{reference}` row names column 136. **A biome is that**, and `spec/planet.md`
names the six: *ocean, ice, desert, grassland, jungle and mountain.*

## The instruction

**Give `territory` a second column and no reference.** It has one today,
`{column id:38 relation:territory seq:1 name:id}`.

```
{column id:155 relation:territory seq:2 name:biome}
```

**And nothing else** - no relation, no family, no members, no reference row.

## How to tell it was carried out

```
spec/data/schema.4x   territory has two columns, biome is the second, and no {reference}
                      names the new column
a territory row       reads {territory id:1 biome:grassland}
the suite             still green, because no rule reads a biome
```

## What it does not do, and you already said why

**Nothing constrains a biome to the six.** `layer` is unconstrained the same way, and **no rule
ever reads a biome** - *a biome earns its place by being shown rather than by being obeyed*. **A
check that the six are the six is available and is not part of this**; if you want one it is a
sentence in `spec/planet.md` and a check in the code lane's column.

**And `set-biome` is not written here.** It rejoins the other four rules in `S-200`'s queue behind
the engine work, now that its shape is settled.
