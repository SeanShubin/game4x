# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-574 - adjacency stays directed, and the check that keeps worlds two-way needs a scope

**to** sean · **status** open · **raised** 2026-09-27 · **answered** 2026-09-27, directed · **asks** a decision · **kind** entailed · **into** `spec/planet.md`, and a check in `releases/rules-become-data.md`

**You said**: *I am thinking i want to leave one-way adjacency open, but for the current release
have check in place so that we never generated a world without two way adjacency.*

**That dissolves the contradiction rather than choosing a side.** If adjacency is directed then two
rows are **two facts** - passable one way, passable the other - so `a fact is stated once` is not
broken and nothing is duplicated. **Both of the answers this item offered are withdrawn**; neither
was needed.

## One sentence of `spec/` now says the wrong thing

**`spec/planet.md`**: *Adjacency is a shared boundary. Two places are adjacent when they share one.*
**Under your answer that is no longer what adjacency means** - it is how the worlds you generate
happen to be built. **A cliff you descend and cannot climb would satisfy the model and contradict
that sentence.**

**This lane is not proposing the replacement yet**, because the scope below changes what it should
say.

## The scope decision, and the measurement makes it concrete

```
directed adjacency rows in everything you have read, plus the scenario:  19
rows with no reverse:                                                    17
files declaring an adjacency:                                            13
```

```
W1  the check is on generated worlds only - what `generate-planet` produces.
    Costs nothing today: 0 files change, and a hand-written test world stays
    free to be one-way, which is correct if one-way is legal
W2  the check is on every world, tests included. 13 files gain rows - and 12
    of them are tests you have approved, so the copies in `reviewed/` stop
    matching and you re-read all twelve
```

**Your words say *generated*, which is `W1`.** This lane reads it that way and is asking because
`W2` costs twelve re-readings and nothing said which you meant.

## What is true either way

**The scenario gains the reverse row.** Your ark-move edit needs it - the ark crosses one way and
the pioneer comes back the other - and `scenario/main.4x` is one world whichever scope you pick.

**And the reviewed tests are right as they stand under `W1`.** A test about moving forward along a
chain declares the boundary it uses and no more, which is *the model is a minimal expression of
intent* applied to a world.
