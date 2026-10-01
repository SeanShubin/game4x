# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-606 - Column order is not behaviour, one function normalizes, and that decides the order for us

**to** sean · **status** open · **raised** 2026-10-01 · **answered** 2026-10-01, twice: column order is not significant, and the comparison delegates to a normalize function · **asks** approval · **kind** recovered · **shape** text · **into** `spec/README.md` -> rule 3

**Your two answers together settle the order, which neither of them does alone.** This replaces the
ambiguous phrase in `P-600`.

**Replacing *entries coalesced to one per description, traits and entries in the order this
specification already gives them, whitespace not significant because the braces say where a row
begins and ends*:**

>    entries coalesced to one per description, **and the columns in a row normalized**, because the
>    order they are written in is not significant; whitespace not significant either, because the
>    braces say where a row begins and ends. **One function does that normalizing and everything
>    that compares delegates to it** - so no comparison can disagree with another about whether two
>    tests say the same thing.
>
>    **The order that function puts columns in must not depend on anything editable.** `id` first,
>    then every other trait alphabetically, then `occupied`, `free` and `capacity` last - **the order
>    this specification already states for an entry**, which depends on names. **An order taken from
>    the schema's `seq:` would mean renumbering those cleared every approval I have given**, and
>    renumbering is a tidy-up nobody thinks twice about.

## Why your two answers together pick the order

**Alone, *order is not significant* permits any deterministic order.** Alone, *one normalize function
that comparison delegates to* permits any order too.

**Together they do not**, because the record is that function's output - `P-605`: *the rows,
canonical*. So the record's order **is** normalize's order, and if normalize derives it from `seq:`
then renumbering `seq:` changes every record and clears every verdict. **A name-derived order cannot
do that**, and `spec/console.md` already states one.

## What it costs, and your own ruling covers it

```
records whose rows would be reordered    57 of 57
rows not in alphabetical trait order     449, with 51 alphabetical by coincidence
```

**Every record changes, and that is not a change of approval by your own words.** *The order of the
columns is not significant, so this should not make tests different* - so rewriting a record into a
different column order does not change what you approved, exactly as moving one did not. **A lane may
convert them**, on the same ruling that let a lane move them.

**And the code lane's cost stands**: the canonical ordering lives in
`crates/game-console/src/containment.rs`, which `crates/game-model` cannot reach because the
dependency runs the other way, so it moves to `friendly-notation` - a refactor of a 712-line module.
**That is the price of the trap not being there.**

*Nothing is open. Everything filed has been decided.*

