# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

### P-450 - `P-448` answered one of `C-98`'s three cells, and `traits.4x` waits on the other two

**to** sean - **status** open - **raised** 2026-09-12 - **kind** entailed, from the code lane's `C-98` - **asks** a decision - **into** `spec/console.md` -> The language, and `spec/data/`

**The code lane has `kinds.4x` and is stopped on the next file.** `C-98` named **three** cells that
hold several values where the notation gives a key one. **`P-448` answered the first** - a kind
declares its family, so `Members` is gone. **Two are left, and both are in *Traits*.**

## The first: what a trait is *of*

**Read from the table, every trait's `Of` column.** Most name one thing and four do not:

```
{trait name:laboring of:citizen}                        one, and fine today
{trait name:defending of:citizen}   ... of:unit         two - and "a unit" is a family, not a kind
{trait name:strength of:citizen}    ... of:garrison     ... of:ark   ... of:pioneer
{trait name:resource of:extractor}  ... of:store
```

**`strength` is the one worth looking at**, because the family vocabulary already shortens it. Its
four are citizen, garrison, **and `unit`** - so it is three lines rather than four, and would be two
if there were a family for the things that muster.

**And a line per pair is the same count whichever file holds it.** `{trait name:strength of:citizen}`
in `traits.4x`, or `{kind name:citizen trait:strength}` in `kinds.4x`, is the same number of lines
saying the same thing. **`P-448` chose the second shape for families** - the member declares what it
belongs to - **and consistency says the same here.**

**What pulls the other way**: a trait's other facts - what it admits, whether it is stored - want to
be on the trait's own line, and putting its domain in `kinds.4x` splits one declaration across two
files.

## The second: what a trait *admits*

**Three sorts, and only the third has no home:**

```
{trait name:laboring admits:0-or-1}          a closed pair, in every readiness trait
{trait name:resource admits:resource}        a family - the values ARE kinds
{trait name:biome admits:???}                six words that are not kinds at all
```

**`resource` solves itself** - *one of the resources* names the family, and `P-448` already declares
families. **`biome` does not.** Ice, desert, grassland, jungle, mountain and ocean are not kinds and
belong to no family; they are words that exist only as this trait's values.

**The same principle would say the value declares its trait:**

```
{value name:ice of:biome}
{value name:desert of:biome}
```

**Which is `P-448`'s shape a third time** - a kind declares its family, a value declares its trait -
and it is how an enum works in the language your `Any` analogy came from.

## And a third question the code lane raised that is not `C-98`

**`Biomes` and `Units and structures` declare facts about kinds rather than vocabulary**, and
`P-443` settled the vocabulary case only. **Under `P-448`'s shape they are not new files at all:**

```
{kind name:ark family:unit strength:2 binding:3 crosses:orbit-border requires:yard}
```

**Every column of *Units and structures* is a trait the ark has**, so the table folds into
`kinds.4x` the way `family` did rather than becoming `units.4x`. **`Biomes` does not fold**, because
a biome is a value rather than a kind - it would be `{value name:jungle of:biome nature:2 ...}`,
which is **a value carrying traits**, and nothing says whether that is allowed.

## What this lane is not doing

**Not choosing**, for `C-49`'s reason and because two of the three have a candidate rather than an
answer. **What it will say** is that all three have the same shape available - *the member declares
what it belongs to* - and that taking it everywhere is the unification `P-446` would approve of,
**while the split of a trait's declaration across two files is the cost that might make it a lie.**

**Nothing is blocked meanwhile.** `families.4x` is fully specified by `P-448` and is the next file
either way.

