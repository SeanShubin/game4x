# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-578 - `C`: a biome is its own relation, and `P-577`'s column comes out

**to** sean · **status** open · **raised** 2026-09-27 · **answered** 2026-09-27, `C` · **asks** approval · **kind** recovered · **shape** an instruction · **into** `spec/data/schema.4x`

**You said `C`.** `P-577`'s column is removed and a biome becomes a relation keyed by territory.
**No existing row changes**, the 35 reds go, and nothing goes back through the review application.

## The instruction

**Remove** the row `P-577` added:

```
{column id:155 relation:territory seq:2 name:biome}
```

**Add**, in its place:

```
{relation id:56 name:biome}
{column id:155 relation:biome seq:1 name:of}
{column id:156 relation:biome seq:2 name:what}
{reference id:77 column:155 to:territory}
{state id:22 relation:biome}
```

**A territory row goes back to `{territory id:1}`** and a biome reads
`{biome of:territory-1 what:grassland}`.

## Why this shape and not another, each from a row already there

**`of` references a territory** the way `place.of` does - `{reference id:70 column:135 to:territory}`.
**`what` denotes nothing**, the way `place.layer` does: no reference names it, and `grassland` is a
bare word exactly as `surface` is. **That is the argument `P-577` should have made for `C` and made
against it.**

**No `id` and no `quantity`**, the way `carries` and `member` have neither. There is one biome per
territory, nothing counts them, and nothing references one.

**`{state id:22 relation:biome}`** because a biome is part of the game's state, as `place` is.

## How to tell it was carried out

```
spec/data/schema.4x   territory has one column again; `biome` is a relation with two;
                      one {reference} names column 155 and nothing names 156
the suite             the 35 reds are gone, and no test file was edited to make that so
reviewed/             untouched - the whole point of C
a biome row           reads {biome of:territory-1 what:grassland}
```

**The middle line is the check that matters.** If any test had to change, `C` was not what was
done.

## What is unchanged from `P-577`

**Nothing constrains a biome to the six `spec/planet.md` names**, for the reason you already gave:
*a biome earns its place by being shown rather than by being obeyed.* **And `set-biome` still
waits behind `S-200`'s engine work** with the other four rules.
