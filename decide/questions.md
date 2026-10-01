# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-604 - Three forms the isolation could take, shown as rows

**to** sean · **status** open · **raised** 2026-10-01 · **revised** 2026-10-01, twice: first after he said the single function is logical rather than architectural, then after he said the choice was not legible · **asks** a decision · **kind** recovered · **into** `spec/invariants.md` -> The game is one function

**You have settled that there is isolation. This asks what form it takes, and the first version asked
it abstractly, which was this lane's error.** Three forms, as rows.

## One - a column on `relation`

```
{relation id:21 name:citizen concern:game}
{relation id:60 name:menu-item concern:interface}
{rule id:1 name:move concern:game}
```

**A clause is refused when its relation's `concern` is not the rule's.** One new column on two
existing relations and nothing else; `move` keeps its five clauses and gains a word.

## Two - the file is the group

```
spec/data/schema.4x            game relations
spec/data/interface-schema.4x  interface relations
spec/data/rules.4x             may declare clauses over the first and a named shared set
spec/data/interface-rules.4x   may declare clauses over the second and the same shared set
```

**No new concept at all - the boundary is where a thing is written**, which is how `crates/` already
works. **The cost is that the engine has to know which file a relation came from**, and today it reads
rows and forgets.

## Three - two runs, and a command is the only crossing

```
the game engine      state: game relations        commands: move, toil, end-turn
the interface engine state: interface relations   commands: select, confirm, hover
                     and it emits a game command rather than touching game state
```

**Nothing is refused because nothing is reachable.** Two invocations of one function over disjoint
state, which is your *different implementations behind interfaces* read literally. **The cost is that
showing a territory's name means the interface holds a copy**, derived by something that is not a
rule.

## What each one costs you to live with

**One** is the cheapest to build and the easiest to weaken - a `concern` is a word somebody can
change on a relation, and nothing says which concern a new relation belongs to.

**Two** makes the boundary visible in the tree and gives the generator a job: a relation's concern
stops being a fact anybody states and becomes a fact about where it lives.

**Three** is the only one where a game rule reading a menu is *not expressible* rather than *refused*
- and the only one that forces a decision about what holds the interface's copy of game state.

**`family` is not one of these.** It groups relations that behave alike so one rule can serve all of
them; this groups relations so a rule cannot leave the group. **Same word, opposite purpose.**
