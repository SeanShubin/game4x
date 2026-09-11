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

### P-385 - Is `limit 0 garrison` hard or soft, and does founding twice do it twice

**to** sean - **status** open - **raised** 2026-09-11 - **kind** entailed, from `X-12` and `C-87` - **asks** a decision - **into** `releases/first-release.md` -> Recipes

**`deploy ark` and `found by land` are the same recipe apart from what is spent.** Counted from the
release rather than recalled, and re-counted by this lane today: nine rows and eight, **seven
identical** - `limit 0 garrison`, `produce 1 garrison`, `produce 2 citizen`, two extractors and two
stores. `deploy ark` spends an ark from the orbit above; `found by land` spends a pioneer. Nothing
else differs.

**So *what a new colony starts with* is two edits, and one of them can be forgotten.** You intend
players to edit recipes inside the game, which makes a duplicated block a rule somebody changes once
and sees take effect half the time.

**The blocker is not the duplication. It is one word in the row both copies share.**

Today `limit 0 garrison` is **hard**: it gates the whole recipe, so deploying onto ground that
already has a garrison does nothing at all. **Made soft** - the way `create-if-missing` works - the
garrison line is skipped when one is already there and **the other six rows still fire**: deploying
onto an existing colony gives it two more citizens, two more extractors and two more stores.

| The gate             | Deploying onto an existing colony              | What the other six rows mean                                  |
| -------------------- | ---------------------------------------------- | ------------------------------------------------------------- |
| **hard**, as written | nothing happens                                | they are one indivisible founding                             |
| **soft**             | it grows by 2 citizens, 2 extractors, 2 stores | each is a separate question, and they are not the same answer |

**The table as written hides that there is a question.** One hard gate in front of seven rows reads
as though the seven were considered together; splitting it is what shows they were never considered
individually. **This is why nothing can be extracted first** - a shared sub-recipe freezes whichever
answer it is built on, in the one place two recipes would then both read from.

**Two things ride on it, and neither is a reason to hurry.** A `found-colony` sub-recipe would be the
first recipe that calls a recipe, which is the population `C-75` correctly refused to wire an
acyclicity check over - the check stops being green over nothing the moment it lands. And once
players edit recipes, the engine has to reject a bad one at edit time with a reason, which makes
that check editor validation rather than elegance.

**One more question, flagged rather than reconciled, because answering it is also yours.** Your
sketch of the founding recipe **omits both stores**, which the release produces. Dropping them is a
change to the game and not a refactor, so nothing has been reconciled toward either version.

**Nobody is asking you to approve an extraction.** The research lens says the duplication is worth
acting on and deliberately did not decide this; the code lane says it has built nothing and why.
**This item exists because the question was sitting in a lens's outbox addressed to a producer, where
nothing was carrying it to you** - which is the hop this lane owes and had not made.

