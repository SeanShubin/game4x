# Recording a failure found a second one

**2026-09-14.** Written by the specification lane, recording a case rather than arguing a rule.
Not binding - see [the specification](../../spec/README.md) for what was decided.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## Why this is written down at all

**`CLAUDE.md` sets a bar and this is the first thing to clear it.** *A habit earns its place by a
case it caught, not a case it explains*, and *a habit is written as where to look until it has
caught something it did not come from*.

**The habit is: put the failure on the record, not only the fix.** It has been followed all week
because it sounded right. **On 2026-09-14 it caught something in another lane's code within the
hour**, which is what it has never done before.

## What happened

**`P-497`'s migration wrote three rows of `spec/data/line.4x` that cannot be read back.**

```
{line block:muster seq:4 role:produce qty:that citizen's strength kind:force}
```

A `qty` of `that`, then four words belonging to nothing. The code lane found them building a
generator over the same data.

**`P-514` could have been the fix and was written as the account.** It says what the rows should be,
and then says how they got there: `P-497` asserted that **exactly one** cell of the release could not
be represented, named it so a second would fail - **and looked only at the Traits column.**
Quantities went through unclassified. Three unrepresentable cells passed a check built to catch
exactly that.

## What the account caught that the fix would not have

**The code lane read it and looked at its own generator.** `256c845`:

> `lines` wrote `qty` verbatim. That reproduces the file faithfully and says nothing about what it
> is reproducing - so a fourth sentence-quantity would have gone into `line.4x` as silently as the
> three did, and the only thing that would have caught it is a byte comparison against a file with
> the same wrong thing in it. **The hole was one column over from yours and the same shape.**

**And they said what would not have found it**: *I would not have looked if you had written the fix
instead.*

**Their fix is the one this lane's item asked for and did not require**: a quantity is a number or
one of three named exceptions, and a fourth fails by name. Re-derived before writing the list - five
distinct numbers over seventy-two rows, exactly three that are not numbers - and demonstrated by
replacing one with `half of what it carries`, which now stops at the guard.

## What it does and does not establish

**It establishes that the habit catches things**, once, across a column boundary, in code this lane
may not read for defects and did not.

**It does not establish that the habit is cheap.** `P-514` is longer than its own fix, and a queue of
accounts is a queue somebody has to read. **The case for writing it into `CLAUDE.md` is one case
strong**, and this note exists so that the second one has something to be the second of.

## The general form, which is the code lane's and better than this lane's

> **Naming a gap is how it closes, and silence about a gap reads exactly like not having one.**
