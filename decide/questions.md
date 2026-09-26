# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-562 - three of the seven capabilities waiting on you rest on the ruleset `D-1` replaces, so do you look before it starts?

**to** sean · **status** open · **raised** 2026-09-25 · **asks** a decision · **kind** entailed · **into** `releases/first-release.md`, and how the code lane sequences `D-1`

**The three answers, so you can correct rather than design.**

```
V1  the code lane vets nothing and waits      you read R-6, R-7, R-8 first; D-1 starts after
V2  the code lane starts now                  the three go stale under you; they are re-run
                                              and re-offered once the new model plays
V3  the code lane starts on D-2 and D-3       the tests and the data move first, the ruleset
                                              last, and you read the three whenever you like
```

## Why it is only three, re-derived here rather than taken from `C-143`

**The code lane measured that the two rulesets share seven recipe names of twenty-one** and said
plainly that a name comparison cannot say whether they are the same game. **What follows without
settling that** is which capabilities are derived from the ruleset at all:

```
R-6   the scenario plays through        scenario/expected/play.4x, from the old model   STALE
R-7   each recipe confirmed alone       reports/recipes.md, generated from the release  STALE
R-8   which kinds behave alike          reports/catalog.md, the same release tables     STALE
R-11  the engine's inputs are reachable the claim holds; the SET of inputs changes      RE-LOOK
R-9   reports browsable, no script      a property of the report's form                 SAFE
R-10  a drawing in either theme         a property of the drawing's form                SAFE
R-12  the foundation form of a test     generated from reviewed/, the NEW ruleset       SAFE
```

**`R-12` is the one that gets stronger**, because `reviewed/` is what the new model is built to
obey.

## What this lane would say, and it is weak

**`V3`.** `D-2` and `D-3` are the two that do not touch `crates/game-model`'s rules, so they can be
built while the three sit unread, and **nothing is wasted whichever way you then go**. It is weak
because it assumes the code lane can order them that way, and that is its call rather than this
lane's.

**`V1` costs you a session now** and preserves work already done and reported. **`V2` costs the
code lane a second run of three evidence lines** and costs you nothing, and `R-7` shows what that
looks like - it has already been re-reported twice under you, on 2026-09-11 and 2026-09-12.

## What is not in question

**Nothing here asks you to re-approve `D-1`.** The capability is right and this is about the order
it is built in.
