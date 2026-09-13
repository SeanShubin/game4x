# Two conventions on one page

**Derived.** Written by Claude, 2026-09-12, from one defect in `X-32` found by the code lane
building it and corrected by the research lane at `943c87f`. Not binding - see
[the specification](../../spec/README.md) for what was decided.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

**This repository has written down two classes of this kind and this is a third.** The first is
about a **reader** - *the instrument answers a narrower question than the one asked, and returns a
plausible answer*. The second is about a **text** - *an artifact says more than its author meant,
because quoting a thing and doing it are the same bytes*. This one is about a **join**: each half
is correct, the defect exists only in the combination, and no amount of re-reading either half
finds it.

## What happened

`X-32` specifies a hex torus whose sizes are `N = 3k²`, and gives a generator for the wrapping.
**The size formula was written in one coordinate convention and the generator in the other**, on
one page, in the two places a reader has to put them together.

Taken into the other's norm the generator gives `7k²`. At `k = 2` that is **28 cells where 12 were
wanted** - which is what the code lane got when it built the thing.

## Why reading it twice would not have found it

**Hexagonal lattices have two conventional norms**, `a² - ab + b²` and `a² + ab + b²`, and **they
represent the same integers.** Re-derived here rather than taken: over `0 ≤ a,b < 40` both produce
the same 123 values up to 400.

So the size list, the ratio table against a sphere, the colouring argument and the ten rungs are
**all correct under either convention**, and the list a reader would check is identical either way.
**There was no false sentence anywhere in the item.**

**What disagreed was an integer produced twice** - once by the formula and once by the generator -
and nothing produces it twice until somebody builds it.

## The near-miss, which is worth as much as the catch

The code lane's own observation, and the part of this most worth keeping:

> If I had built in the 120-degree convention I would have got 12, drawn a correct picture, and
> shipped an item whose generator and size formula still could not be combined.

**The defect would have survived the only process that found it.** The catch depended on which of
two equally reasonable conventions the builder happened to pick - so the rate at which this class
is found by building is about one in two, and the picture that hides it is a correct picture.

## What this means for a check

**Nothing mechanises it, for the reason `P-245` names.** A check would have to know that two
expressions on one page are in different conventions, which is the question being asked. What is
available is the same repair as every other case in this repository: **produce the answer a second
way and find that the two differ.**

**The difference here is that neither producer could see it from inside.** The code lane found a
wrong answer by building; the research lane found the cause by re-deriving. Neither noticed it was
a class rather than an incident, because each had only its own half of it.

## Sources

`943c87f` (the research lane's correction, re-derived by determinant), `fe227c0` (the prototype
built and the workspace joined), `5c0a3b6` (`X-32` closed), and the messages between the three
sessions on the evening of 2026-09-12. The 123-value equality above was computed for this note.

**Both of this note's load-bearing numbers were then derived a second time by the lane the note
is about.** The research lane recomputed the 123 values - identical sets rather than equal
counts - and checked the near-miss it had not made, getting **12** from the generator at
`k = 2` in the 120-degree convention. So the claim that a correct picture was one coin-flip
away rests on two independent derivations rather than on the builder's account of what it
nearly did.
