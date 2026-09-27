# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-575 - the rule notation is built for playing and world-making is a different operation, measured four ways

**to** sean · **status** open · **raised** 2026-09-27 · **stopped** 2026-09-27, a second time and for a better reason · **asks** a decision · **kind** measured · **into** `spec/console.md`, and how a world comes to exist

**This lane started writing the five rules and stopped four times.** Each time the notation could
not say the thing. **They are not four difficulties; they are one fact seen four ways.**

```
1  no rule has ever added a row to a relation carrying an `id`
   what rules add to: ark bin citizen energy extractor labor pioneer resource unit
   every one keyed by `where` with a quantity. territory, place and adjacency all
   carry ids and no rule touches them

2  no rule takes a number
   every input is `of:` a relation - place, resource, unit, founder, trait, relation
   `set-resource <territory> <resource> <extractors> <density>` needs two numbers

3  there is no `phase`
   so "before play begins" cannot be said, and no design command can be distinguished
   from a play command

4  there is no `biome`
   `P-541` brought it back to the data and nothing carried it into this schema
```

**A rule changes a world that exists. World-making makes identified things with literal values,
which is the one thing the notation does not do.**

## And your own artifacts already say how a world arrives

**Every world is stated, never built.** All fifty-four reviewed tests state theirs in `{given}`;
`scenario/main.4x` states its own. **Nothing anywhere has ever made a world by firing rules** - the
thing `N1` asks for has no precedent in the model because the model does not work that way.

## So `spec/console.md` contains the contradiction, and it is one sentence

> A game state changes only by a transition, which phase a game is in is part of its state, and
> **designing is therefore made of the same rules as playing.**

**That sentence is what `N1` was faithful to.** Against it stands everything above, and the fact
that no phase exists to be part of any state.

## The decision

```
N2   a world is produced, not played into being. `generate-planet` emits a stated
     world - rows - and design commands stop existing. The sentence above changes.
     Nothing in the notation grows
N5   the notation grows: numeric inputs, ids that come from the command, a phase
     relation and a biome relation. Then designing really is made of the same rules
     as playing, and `create planet` is a macro writing them, which is your framing
```

**This lane would say `N2`**, because four extensions to buy a thing you only do once per game is
a large price, and because the model already behaves that way everywhere. **`N5` is what your macro
framing needs to be true**, and it is the honest version of it - the macro was right about the
mechanism and it has nothing to expand into yet.

**What is not in question**: the ten mechanical renames, and that the old model goes.
