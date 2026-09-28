# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-584 - A unit test is one part, and an arc belongs to the scenario

**to** sean · **status** open · **raised** 2026-09-28 · **asks** approval · **kind** recovered · **shape** text · **into** `spec/README.md` -> Rules for this directory, rule 3

**Your words, and the measurement that says they describe fifty-four of fifty-six tests already.**
Nothing states how big a test may be, so the two that are not parts are not wrong by any rule - they
are just unreadable, which is what you found.

**Offered as a block at the end of rule 3**, after *prose is the one of the three that can drift
without anything noticing*:

> **A test states one part, and a part is small enough to hold in my head.** The point of the
> directory is that every piece of the game is checked independently, so a test that needs a
> narrative to follow has stopped being one - **and an arc of play belongs to the scenario instead.**
> The scenario is broken into a case per command for exactly this reason: I cannot validate a whole
> playthrough at once, and I can validate any step of one.
>
> **So a test's `when` is one command, or few.** Where a rule needs a world that took several
> commands to build, those commands go in the `given` as state rather than as a story.
>
> **And a test never turns on an absent row.** What is missing cannot be read, so a test whose point
> is that something *could not* happen names what was lacking rather than leaving me to notice which
> of twelve rows is not there.

## What it is measured against

```
commands in {when}   tests
        1             44
        2              8
        3              2
        9              1    an-ark-lands-a-planet-is-developed-and-an-ark-leaves
       21              1    a-second-settlement-launches-the-ark-the-first-could-not
```

**The third paragraph is from your reading rather than from the count.** The first settlement cannot
launch because `place-1` has no energy deposit and `launch` removes an energy - **the title rests on
a row that is not there**, among twelve that are.

## What follows, and it is not in the offered text

**The two are already covered where they belong.** `scenario/main.4x` plays the arc `D-5` describes
and `the_arc_d5_describes_is_the_arc_that_runs` asserts it, over the ruleset you have read - so the
twenty-one-command test is a second statement of what the scenario already says, in the one form you
cannot check.

**Retiring them is your gesture and not this lane's.** If this lands, this lane deletes the two
sources; the review application then shows you two records whose test is gone, and removing those is
yours. **Nothing here creates or deletes a record**, which is the rule that makes the record mean
anything.

**And it changes what I owe you on the Yard.** Both of those tests launch without a yard, so both
were going to need re-reading. **If they are retiring instead, that is two readings you do not
owe** - and the Yard's requirement is already stated by
`an-ark-cannot-launch-where-there-is-no-yard`, which is one command.

