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

### P-321 - `spec/console.md` states two incompatible forms for a command, nineteen lines apart

**to** sean - **status** open - **raised** 2026-09-06 - **kind** contradiction - **asks** a decision
- **into** `spec/console.md` -> The language, or -> Commands

**Filed the moment it was found**, which is the rule for a contradiction. The code lane found it
starting `S-26` and stopped that half rather than choosing.

**`spec/console.md` -> The language**, line 18: *A command is a verb followed by arguments, one
command to a line.* Then eight examples of that form.

**`spec/console.md` -> Commands**, line 60: *A command is written `{name field:value ...}`. Its name
is the words that open it and its arguments are named.*

**Both are present tense and normative and they are about the same thing.** `land ark 1` has
positional arguments that are not named; `{name field:value ...}` has named ones. **A parser cannot
be built to both.**

**The tree-carrying half only works in the second form.** Line 61 says *a value is a word, a number,
or another command in the same form*, so a command may carry a tree - and `land ark 1` has nowhere
to put one.

**Everything actually written is in the first form.** `scenario/commands/play.4x` opens `land ark 1`,
`create labor 1 1`, `work 1 extractor 1 food`, `end turn`. **So the older statement is what is
built, tested and reviewed by you**, and the newer one is what a reader of the section reaches
second.

**Two ways, and it is a decision because it is your design rather than a wording slip.**

- **Positional, and line 60 goes.** Nothing changes in the scenario, the parser or what you review
  by hand. **What is lost is the tree**: no command can carry another, so `P-215`'s nested command
  has no form - and `C-23` already records that its case has never arisen
- **Named, and line 18 and its eight examples go.** `land ark 1` becomes something like `{land
  kind:ark territory:1}`. **The scenario is rewritten, and so is the file you review most often** -
  199 commands. What is gained is one form for commands and data, and the tree that `P-215` wants

**I have no recommendation and am not going to invent one.** The first costs nothing now and forecloses
a promoted rule; the second costs the artifact you read by hand. **Which of those is worth more is
the design question**, and the code lane declined to answer it for the same reason.

**One fact that may bear on it.** `spec/console.md` -> Commands says the commands are not a list the
document keeps - *they are the recipes whose owner is the player*. **A recipe binds named things**:
the place it acts in, and each ingredient or trait value it names with a `$`. So the named form is
closer to what a command already is underneath, and the positional form is a shorthand for it.
