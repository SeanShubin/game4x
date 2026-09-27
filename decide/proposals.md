# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-578 - `C`, for your reason rather than mine: a biome table properties can grow in

**to** sean · **status** open · **raised** 2026-09-27 · **answered** 2026-09-27, `C` · **asks** approval · **kind** recovered · **shape** an instruction · **into** `spec/data/schema.4x`

**Your reason is a different shape from the one this lane proposed and it is better.** *So that we
can expand upon properties of a biome in one place* needs a **table with one row per biome**. This
lane's `C` had one row per territory, which is where a biome **is** and not what a biome **is** -
and nothing could have been expanded in it.

**You also said the rows-changed argument is valid and not compelling**, and that is right: more
rows come later, so a correction now is cheaper. **It is dropped as a reason and kept only as a
consequence.**

## The shape already exists twice over, as `trait` and `carries`

```
{trait id:1 name:moving}                 the table - one row per trait, and where a
                                         property of a trait would go
{carries kind:scout trait:moving}        the assignment - a join
{reference id:54 column:109 to:trait}    the join points at the table
```

**A biome is that, exactly.** `trait` carries no `{state}` row because it is vocabulary; `carries`
carries none either. **A territory's biome is part of the world**, so the join does carry one, the
way `deposit` does.

## The instruction

**Remove** `P-577`'s row: `{column id:155 relation:territory seq:2 name:biome}`.

**Add** the table, its six rows, and the join:

```
{relation id:56 name:biome}
{column id:155 relation:biome seq:1 name:id}
{column id:156 relation:biome seq:2 name:name}

{biome id:1 name:ocean}      {biome id:4 name:grassland}
{biome id:2 name:ice}        {biome id:5 name:jungle}
{biome id:3 name:desert}     {biome id:6 name:mountain}

{relation id:57 name:terrain}
{column id:157 relation:terrain seq:1 name:of}
{column id:158 relation:terrain seq:2 name:is}
{reference id:77 column:157 to:territory}
{reference id:78 column:158 to:biome}
{state id:22 relation:terrain}
```

**A territory row is `{territory id:1}` again** and its biome is `{terrain of:territory-1
is:grassland}`.

## One name is this lane's choice, said out loud

**`terrain`, from your own sentence** - `spec/planet.md`: *a territory's biome is what the terrain
gives it.* **If you would rather the join were `biome-of` or anything else, say so with the
promotion.** The table's name, `biome`, is yours already.

## What the six rows buy, given your second point

**You said the only reason a biome is in there is the realistic rendering.** So the table starts
with a name and nothing else - **and the point of it is that a colour, a terrain profile or
whatever the drawing needs later goes in as a column**, in one place, rather than being learned by
whatever draws it.

**That is also why `biome` is not state and `terrain` is**: the six do not change while a game
runs, and which territory has which is the world.

## How to tell it was carried out

```
spec/data/schema.4x   territory has one column again; `biome` has two and six rows;
                      `terrain` has two, both referenced, and one {state} row
the suite             the 34 reds are gone, and no test file was edited to make that so
reviewed/             untouched
```

**The middle line is still the check that matters.** If any test had to change, this was not `C`.
