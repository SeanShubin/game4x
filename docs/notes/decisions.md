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

### P-388 - Is the weighting declared or solved for, and what names a source

**to** sean - **status** open - **raised** 2026-09-11 - **kind** entailed - **asks** a decision - **into** `releases/first-release.md` -> Kinds, and a check for the code lane

**You asked for what it takes to enforce *nothing comes back round with more* in production.** Three
of the four pieces exist. **This is the one that does not, and it is a choice rather than work.**

**What the invariant is, mechanically.** `spec/invariants.md` describes a **place invariant** of the
recipe net: a weighting `w` over the kinds where, at every recipe, `w · (made - taken) <= 0`. Whether
such a `w` exists is a linear feasibility question over a matrix with one row per recipe and one
column per kind - **73 rows and sixteen kinds today.** That is what *decided mechanically, from the
rules alone* can be.

**The two sentences that describe the weighting do not agree about who writes it.**

| The sentence                                    | What it implies                                                                           |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------- |
| *The weighting is where it is said what counts* | **you declare it** - a number per kind, carrying which kinds the game treats as wealth    |
| *decided mechanically, from the rules alone*    | **the check derives it** - it solves for any `w` that works, and reports that none exists |

**They are not opposites and the order matters.** Solving first is strictly more informative: **if
no weighting exists, no declaration can rescue it**, and the solver names the cycle that gains.
Declaring first says what you meant, and a solver that finds *some* `w` may find one that weighs
labor at nothing and calls the game safe for a reason you would not accept.

- **Declared.** A **Weight** column on *Kinds*. The check verifies one inequality per recipe and
  names the recipe that fails. Simplest to build, and the numbers are yours to invent
- **Derived.** No new data. The check solves for `w` and fails when the rules admit none, reporting
  the recipes that force it. Nothing to invent, and it cannot tell you the answer is one you dislike
- **Both, in that order.** Derive to prove one exists, declare to say which one is the game's, and
  check the declared one. Two mechanisms and one question answered twice

**A second thing has to be named whichever you choose**, because the rule exempts it: *a source is
named, and a named source is not a gain.* **Nothing in the release names one.** `deposit` is *what a
territory's ground offers of one resource*, and `work` produces from it - so a deposit is the obvious
candidate and the release never says so. **Without the exemption every weighting fails at `work`**,
which makes metal out of a deposit and would read as a gain.

**What does not need you.** The three remaining pieces are done or are the code lane's:

- **Constant arc weights.** 72 of the 73 rows carry one. The exception is `work`'s *`$where`'s
  density for that resource*, and `P-376` already permits and resolves it - *one rule with a number
  per case ... which whatever reads it may spell out.* Unfolding it is reading, not a new rule
- **No zero tests.** `P-385` deleted the only two `limit` rows, so nothing in the release asks
  whether a place is empty. That matters more than it looks: with such a test the net is
  Turing-complete and nothing about it is decidable
- **Soft lines.** `P-386` keeps softness on what a rule makes, and a soft line only ever makes
  **less** - so a weighting that holds where everything is made holds for every partial firing. The
  matrix is unchanged by softness, which is why that bullet was worth landing first

