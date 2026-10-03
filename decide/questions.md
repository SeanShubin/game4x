# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-617 - What a user interface test is written in

**to** sean · **status** open · **raised** 2026-10-03 · **kind** the open question `spec/tests/interface/README.md` names · **asks** a decision · **into** `spec/tests/interface/README.md`

**You asked for a test showing the starting menu of *new game* and *exit*.** This lane cannot write
it yet: `spec/tests/interface/README.md` says **what they are written in is not settled**, and the
form is an idea rather than wording. **So here is that one test, three ways.**

**All three honour the one thing you have already settled**: *has the user's attention* is a
property of at most one item in the whole interface, so none of them puts it on an item.

## One - the rule form unchanged, counts and all

```
{test name:the-starting-menu-offers-new-game-and-exit}

{given}
{saves} -> 0

{when}
{open-menu}

{then}
{item name:new-game displayed:1 active:1} -> 1
{item name:exit displayed:1 active:1} -> 1
{attention of:new-game}
```

**Nothing new to learn**, and `tools/anchor`, the padder, the drift check and the foundation
generator all work on it as they stand. **What it cannot say is order** - the rows are a multiset,
so this test passes with *exit* drawn above *new game*.

## Two - the same, with an id carrying the order

```
{test name:the-starting-menu-offers-new-game-and-exit}

{given}
{saves} -> 0

{when}
{open-menu}

{then}
{item id:1 name:new-game displayed:1 active:1}
{item id:2 name:exit displayed:1 active:1}
{attention of:new-game}
```

**`id` already means position in `spec/console.md`'s canonical order**, so this reuses a thing that
exists rather than adding one. **The `-> 1` goes**, because an item is one thing and a count of it
says nothing. **The cost is that every menu test states positions**, so inserting *load* above
*exit* renumbers the rows below it.

## Three - a drawn menu, compared as text

```
{test name:the-starting-menu-offers-new-game-and-exit}

{given}
{saves} -> 0

{when}
{open-menu}

{then-drawn}
> new game
  exit
```

**It reads like the thing it describes**, order and attention both visible - `>` is the item with
attention. **The cost is that it is a second format**, so the padder, the drift check, the
foundation generator and the review page each need to learn it, and a test cannot say *displayed
but not active* without inventing more marks.

## What this lane would pick, and why it is not this lane's to pick

**Two.** It says order, it adds nothing to the notation, and it keeps every tool you already have.
**Three reads better and costs a format**; one is cheapest and cannot express the thing you are most
likely to get wrong about a menu.

**But the form is the shape of every interface test you will ever read**, and reading them is the
whole of your work here - so it is a decision rather than an approval, and this lane will not make
it quietly.

## Two things it leaves for later, deliberately

**`{saves} -> 0` and `{open-menu}` are placeholders in all three.** What a save game is, as data, and
whether *continue* means the most recent save or a session in memory, are both unsettled - your own
list from 2026-10-01. **The test above needs neither answered**, because a starting menu with no
saves is the one case that does not depend on them.

**And the status bar is not in it.** You said it is always present and its text is checkable; that
is a second test and a second question about form.
