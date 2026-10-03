# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-617 - What a user interface test is written in

**to** sean · **status** open · **raised** 2026-10-03 · **kind** his decision, with the words offered for reading · **shape** text · **asks** approval · **into** `spec/tests/interface/README.md` -> after *What goes here is a test about what the interface shows*

**Sean, 2026-10-03**: *lets go with 1a* - form one with `displayed` removed, after he saw that an
extra item would be caught and asked what `displayed:1` was for.

**Four paragraphs, replacing the two that say the form is unsettled.**

> **An interface test is written in the same form as a rule test.** Rows in, one command, rows out -
> and the `then` is the whole of what the interface shows, so **an item that is not in it is not
> displayed**, and how many items there are is asserted by the rows being all of them.

> **`active` is a column and `displayed` is not.** An item is displayed by being in the `then`, so a
> column saying so could hold only one value. An item that is shown and unusable says `active:0`.

> **Attention is one row naming at most one item**, rather than a column on each - `{attention
> of:new-game}`. *Has the user's attention* is a property of the interface, and a column would let
> two items hold it at once.

> **What this form cannot say is order.** The rows are a multiset, so no test can require *new game*
> above *exit*. **A known limitation rather than an oversight**, and what settles it arrives when a
> test needs it.

## What it replaces

**These two paragraphs go**, both from `spec/tests/interface/README.md`:

```
**What they are written in is not settled.** The friendly form states game rows, and *displayed*,
*active* and *has attention* are not game rows - so the form a test here takes is an open question
and is recorded in docs/notes/spec-backlog.md rather than decided.

**One thing already known about the shape.** *Has the user's attention* is a property of the
interface naming at most one item, not a flag on each item - so a test that models it per item can
assert two items hold it at once.
```

**The second is kept rather than dropped** - it is the third offered paragraph, with its reason
shortened now that the form it warns about is ruled out.

## The measurement that killed `displayed`

**He asked whether an extra menu item would be caught, and it would.** Measured by adding a second
territory to a rule test's `given` and leaving its `then` alone:

```
before   63 tests, 63 as expected, 0 red
after    63 tests, 62 as expected, 1 red
```

**So the `then` is the whole world rather than a subset of it** - which is what makes `displayed` a
column with one possible value, and makes the item count assert itself.

## What is lost, named rather than discovered later

**A test can no longer say *defined and hidden* as distinct from *absent*.** With an exhaustive
`then` those are the same test. **This lane reads that as right** - a menu test should say what the
menu shows, and *defined but hidden* is a fact about the code - but `displayed` was one of the three
properties you named on 2026-10-01, so the loss is stated rather than assumed away.

