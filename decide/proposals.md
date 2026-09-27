# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-580 - A generated suite per type of thing that is data, and the line is the transition

**to** sean · **status** open · **raised** 2026-09-27 · **asks** approval · **kind** recovered · **shape** text · **into** `docs/process.md` -> How I know the application is right

**The four artifacts are already there and the suites are them.** Three of the four name a type of
thing that is data, and the fourth - the data dump - is what a case's `{then}` holds rather than a
suite of its own. **One type is below all four** and is the only part of this that is new: the words
a rule delegates to, where a change is a change to the code rather than to the game.

**Offered as a block after the paragraph beginning** *When I change my mind, I delete the expected
data*:

> **Every type of thing that is data has a generated suite of its own**, and there are four: the
> commands a scenario ran, the transformations over the things, the definitions of the things, and
> the words the engine implements. **The first three are the artifacts above**; the data dump is
> not a fourth suite, because it is what every case already ends with. **The fourth is below all of
> them** - a word a rule delegates to, where what moved is the code rather than the game.
>
> **A suite is one directory and deleting it is the same gesture at a coarser grain.** Deleting asks
> to be shown; **committing the diff is the acceptance.** So deleting the wrong one costs a question
> rather than an approval - the suite is written back unchanged, and nothing is accepted that I have
> not read a diff of.
>
> **The pattern reaches exactly as far as a transition.** A local command makes none, so there is
> nothing for a case to state about one - and the day a local command has a case is the day the game
> reads the interface.

## What this does not say, so that the release can

**Not the names.** `regression/scenario/`, `regression/rules/`, `regression/types/` and
`regression/primitives/` are how the four are spelled and where they sit, and they belong in the
release with the work. **What is here is the count and the boundary**, which outlive it.

## One sentence in it is a restatement of yours rather than yours

**Deleting asks to be shown; committing the diff is the acceptance.** That is this lane's wording
from today's conversation for a thing your own paragraph already implies - *what I review is the
diff in version control*. **It is the sentence to strike if you would rather the document did not
say it**, because the rest stands without it.

