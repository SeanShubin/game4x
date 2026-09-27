# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-574 - a boundary is shared and a crossing has a direction, and a generated world states both

**to** sean · **status** open · **raised** 2026-09-27 · **answered** 2026-09-27, directed and `W1` · **asks** approval · **kind** entailed · **shape** text · **into** `spec/planet.md`, replacing the *Adjacency is a shared boundary* paragraph

**You said**: leave one-way adjacency open, and for this release check that no **generated** world
is one-way.

**Your geometry sentence survives and the fix is smaller than this item first assumed.** The
boundary is shared - that is symmetric and about the planet. **What the data states is a crossing
of it**, and that is what can have a direction. `spec/planet.md` already separates them one
paragraph later: *to cross is to pass through a shared boundary. A unit crosses some and not
others.*

## The words

**Replacing *Adjacency is a shared boundary. Two places are adjacent when they share one. Edge,
border and boundary name that shared thing, and none of them means anything the others do not.*:**

> **A boundary is shared and a crossing has a direction.** Two places that share a boundary are
> adjacent, and that is symmetric. **Edge, border and boundary name that shared thing**, and none
> of them means anything the others do not. **What the data states is a crossing of it** - that a
> unit may pass from one place to the other - so a boundary may in principle be crossable one way
> and not the other.
>
> **Every generated world states both crossings of every boundary**, and a check says so. **A world
> written by hand may state one**, which is how a test declares the crossing it uses and no more.

## Why the second paragraph names *generated* rather than every world

```
directed rows across everything you have read, plus the scenario:  19
rows with no reverse:                                              17
files declaring an adjacency:                                      13, of which 12 are
                                                                   tests you have approved
```

**Checking every world would change twelve approved tests** and their `reviewed/` copies would stop
matching, so you would re-read all twelve. **And they are right as they stand**: a test about
moving forward along a chain declares the crossing it uses and no more, which is *the model is a
minimal expression of intent* applied to a world.

## What follows and is not in the words

**The check is the code lane's** and `spec/README.md` rule 9 makes it part of stating the rule:
*a boundary stated here is one the build keeps, and the check that fails when it stops being kept
is part of stating it.* **Filed to them when this lands.**

**`scenario/main.4x` gains the reverse crossing**, because your ark-move edit needs it - the ark
crosses one way and the pioneer comes back the other. That is the code lane's file and the check
will not cover it, so it is named here rather than left to be noticed.

**One thing this does not fix**: the relation is called `adjacency` and states a crossing, which
are now two different words for two different things. **Renaming it is a data change reaching
every world and every rule**, and nothing breaks while it waits.
