# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-550 - A check that pins the present state cannot report a gap against what should be

**to** sean · **status** open · **raised** 2026-09-24 · **kind** measured · **shape** text · **asks** approval · **into** `CLAUDE.md` -> What done means

**Two failures this week were the same sentence twice**, and the code lane found the sentence.
**It has already caught three cases it did not come from**, which is what `CLAUDE.md`'s own test
asks of a habit before it becomes a rule.

## What lands

After *a check whose subject is behaviour reads the outcome, not the input*:

> **A check that pins the present state cannot report a gap against what should be.** A test
> asserting the exact words a program produces is the strongest possible statement about what it
> does and **says nothing about what it ought to do** - so it runs green while a document the code
> is meant to obey goes unkept. **What tells the two apart is what the assertion names**: the
> output, or the rule the output owes.
>
> **Two instances in one week and they are one sentence.** `docs/architecture.md`'s rule 4 -
> engine types only in the adapter - was written down and held by nothing. `P-542` renamed the
> planet sizes and the code said `tiny` for three days, **held by a test asserting the old
> names**. Neither was found by a check failing; both were found by somebody re-deriving a claim.
>
> **So a rule a document states gets a check that asks the rule, over every case it covers.**
> `every_refusal_over_a_closed_set_says_what_was_expected` is the shape: four closed sets rather
> than the one an item named, each asserted to say what was expected, the count asserted, and the
> fifth excluded by name with its reason. **Fixing the case an item happened to name would have
> left three others breaking the same rule.**

## Why it is a rule and not a place to look

**`CLAUDE.md` says a habit earns its place by a case it caught, not a case it explains.** This one
was written off two incidents and then **found three more that neither incident mentioned** -
biome, resource and unit all refused without saying what was expected, and no item had named any
of them.

**And it is checkable in the one way that matters.** The code lane drove it rather than arguing it:
reverting the message to what it said that morning turns the new test red with *`spec/console.md`
asks for what was expected* - the exact state that had been green for three days.

## Whose it is

**The sentence is the code lane's.** This lane had the two instances and drew the wrong boundary
around them: it excluded the incident from the class because a dead variant sat beside the live
one, and *a branch that never runs returns no answer at all* is true and was not the point. **The
test that ran every commit and was wrong by a rule is the instance**, and they said so.


