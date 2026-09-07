# When a rule is known and still not applied

2026-09-06. Answers `S-57`'s fourth question, added at Sean's instruction after the eighth case.
Carries the argument for `X-6`. The first three questions are answered in
[the earlier report](2026-09-06-answering-from-memory.md), which stands - the eighth case is a
different shape and does not disturb the classification of the other seven.

**Method.** The eighth case was read as filed. One further case was produced by this lane during the
session and is verifiable in it. The claim about carriers was checked against the tree rather than
asserted: grepping `tools/` for a normalizing comparison helper returns one incidental use in
`tools/outbox/src/lib.rs:286`, for rule names, and nothing general.

## The case, and a ninth beside it

The specification lane proposed *normalize both sides instead of loosening the comparison*, Sean
approved it, that lane promoted it - and then wrote five assertions the same afternoon that each
failed by matching a phrase spanning a line wrap, four of them in one commit. The first attempt to
repair those four used a heredoc and mangled its backslashes, which is **the other rule in the same
paragraph of `CLAUDE.md`**.

**This lane did the same thing, hours later, on the same rule.** Writing the report for `X-3`, its
first attempt used a shell heredoc and failed with `unexpected EOF while looking for matching '`. The
rule - *write a script to a file before running it; never assemble one inside a shell string. A file
has one level of quoting* - had been read that day, in this session, in the file being quoted from.
The fix was not care; it was switching to a tool that writes a file.

So the rule was broken by two lanes within hours of being written, once by the lane that wrote it.

## Why this one is not a memory failure

Nothing needed re-reading. Either lane could have recited the rule. **What did not happen is applying
it at the moment of composing a comparison** - and at that moment the comparison looks correct,
because the fragment is short enough to sit on one line in the writer's head while the file has
wrapped it.

## The answer: some rules fire at doubt, and some fire at confidence

This is the distinction that predicts which rules keep getting broken.

**A rule that fires at a moment of doubt survives as a habit**, because something in the world is the
prompt:

| Rule                                            | What fires it                   |
| ----------------------------------------------- | ------------------------------- |
| Name the population a zero was counted against  | you are about to state a number |
| Re-poison a check when its exception list grows | the list grew                   |
| Do not take another lane's message as true      | a message arrived               |
| Re-read a claim about another lane's column     | you are about to cite one       |

**A rule that fires at a moment of confidence does not**, because nothing prompts it:

| Rule                                           | What would have to fire it                    |
| ---------------------------------------------- | --------------------------------------------- |
| Normalize both sides instead of loosening      | remembering, while the comparison looks right |
| Write a script to a file, never a shell string | remembering, while the string looks right     |

**The failure and the confidence are simultaneous**, which is the same shape as `C-33` and as the
second question in the earlier report. There is no signal because there is no doubt.

## What works, in increasing strength

**The remedy is to move the rule off the actor and onto a carrier** - something in the path of doing
the work that applies it whether or not anyone remembers.

1. **A check.** This repository's answer so far, and it works, after the fact. It has a
   bootstrapping problem on exactly this rule: the rule governs how comparisons are written, a check
   *is* a comparison, so the instrument shares the defect it is meant to catch. Four of the five
   broken assertions were in checks.
2. **A default path on which the rule cannot be broken.** The heredoc failure was not fixed by more
   care; it was fixed by using a tool whose quoting is one level by construction. **The rule became
   unnecessary rather than better remembered.** This is the strongest general form and the cheapest
   when a tool already exists.
3. **One implementation instead of a discipline.** Not *normalize before comparing* but *there is a
   comparison function and it normalizes*. Nothing in `tools/` does this today.

**What does not work is restating the rule.** `CLAUDE.md` gained the normalize rule on 2026-09-06,
in a paragraph that already carried the heredoc rule, and both were broken within hours by the lane
that wrote them. A more emphatic sentence has no more carriers than a plain one.

## The test this yields

For any rule in `docs/process.md` or `CLAUDE.md`, ask **what fires it**. If the answer names
something that happens - a number appearing, a message arriving, a list growing - it can live as a
habit. **If the answer is *remembering*, it needs a carrier**, and writing it down more clearly is
not one.

That is checkable against the existing rules rather than a prediction, and it sorts them: the ones
this repository has had to write down twice are the ones with no carrier.

## What this report does not claim

- **It does not say a check is the wrong remedy** - it is the one this project has, it works, and
  `P-289`'s rule exists because a check caught the loosening. It says a check is weaker than a
  carrier and shares the defect on this particular rule.
- **It does not design the carrier.** Whether an anchor-matching helper belongs in `tools/`, and
  what it looks like, is the code lane's and is not this lane's to specify.
- **The eighth case is unverified**, taken as filed. The ninth is this session and can be read in it.
