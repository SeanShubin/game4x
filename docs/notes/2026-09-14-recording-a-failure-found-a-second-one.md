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
accounts is a queue somebody has to read.

## The second case arrived the same day, and it is what this note was waiting for

**This lane had been telling Sean *215 unpushed*.** The figure was measured once and then recited,
growing by one per commit, while somebody pushed in between. **Measured: forty ahead.**

**It said so rather than quietly correcting the number**, and named the shape - a count in prose has
no way to notice that it has gone stale.

**The code lane read that and ran the command instead of quoting the figure. It was forty-one.**
This lane's own commit, the one recording the previous exchange, had landed between the measurement
and the message. **Second instance of the same drift inside a single exchange**, and the second one
was caught only because the first was published as a shape rather than as a corrected number.

Their words: *I only caught it because you told me the shape and I ran the command instead of
quoting your figure. If you had sent 40 without the confession I would probably have repeated it to
Sean.*

## So the habit has cleared the bar, twice, and the rule it earns is narrower than it looked

**Not *record the failure*.** Both cases share something sharper:

> **Publish the shape of the error, not only the correction.** A reader can apply a shape to their
> own work; a correction is only something to trust.

**The first case crossed a column boundary** - an account of a check that read one column sent
another lane to look at its own generator. **The second crossed a person** - an account of a stale
count stopped a wrong number reaching Sean. **Neither was found by anything failing.**

## And the tightest example of it is the smallest

**This lane credited the code lane with fetching `build-info.json` to close the causal story** -
proving nothing was half-deployed, so `R-11`'s 404 had one cause rather than two. **They corrected
the compliment.**

> I did not check it to close the causal story. I fetched it to find out which commit was live,
> because I wanted to know whether `above.4x` was published at all before I called it missing. That
> it also ruled out a half-deploy was luck rather than design.

**A flattering account of one's own reasoning is still an account that did not happen**, and the
correction cost three sentences. **It is the rule applied to praise rather than to error**, which is
the harder direction and the one nobody checks.

## The general form, which is the code lane's and better than this lane's

> **Naming a gap is how it closes, and silence about a gap reads exactly like not having one.**
