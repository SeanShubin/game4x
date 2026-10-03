# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-618 - One link per category, each saying how much attention it needs

**to** sean · **status** open · **raised** 2026-10-03 · **kind** his requirement, with the words offered for reading · **shape** text · **asks** approval · **into** `releases/marking-state.md` -> a new `E-6`, after `E-5`

**Sean, 2026-10-03**: *I am expecting to see a link for each category of tests, each summarizing
with enough information for me to know which needs attention and how much attention it needs.*

**Today there is one page and one tally**, and the tally mixes kinds: `228 rows · 63 tests · 63 as
expected · 0 red · 63 reviewed · 0 to read`. **Sixty-three is the rule tests**; the 167 case rows on
the same page are not in any of those numbers, and the one interface test is in none of them at all.

## What the numbers would say today, which is the argument

```
                   items   to read   rows     attention
rule tests            63         0      995   nothing waiting
interface tests        1         1        3   minutes
regression cases     166       166    1,511   an evening, and it has never been started
  primitives          60        60       60   one row each
  rules               16        16      268   seventeen rows each
  scenario            37        37      841   twenty-three rows each
  types               53        53      342   six rows each
```

**`reviewed/cases.4x` does not exist**, so **no case has a verdict and none ever has.** The page
shows 167 case rows and says nothing about that.

The capability offered, for `releases/marking-state.md`:

> ## E-6 - I can see which category needs me and how much
>
> - **In** - his words above, 2026-10-03
> - **Vetted when** - the review entry point is a list of categories, one link each, and **each
>   line tells me both how many items wait on me and how much reading that is** - so I can pick the
>   category to spend an hour on without opening it. A category with nothing waiting says so and
>   does not need opening at all

## Why *how much* is a different number from *how many*

**Sixty primitives cases are one row each and thirty-seven scenario cases are twenty-three.** A count
puts the first ahead; the reading puts the second ahead by a factor of fourteen. **So a line carrying
only a count tells you which category has the most items and not which will take the evening** -
which is the half of your sentence a count cannot answer.

## What this lane did not decide

**What the second number is.** Rows is what is measurable today and may not be what reading costs -
a 23-row scenario case you have been walked through once may be quicker than a 3-row interface test
in a form you have never seen. **The offered words say *how much reading that is* rather than naming
a unit**, so the code lane picks one and you find out by using it.

