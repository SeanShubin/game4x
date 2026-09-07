# Questions

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

### P-311 - Containment is a tree in the specification and a column in the data

**to** sean - **status** open - **raised** 2026-09-06 - **kind** gap - **asks** a decision -
**into** `releases/first-release.md` -> the dump's relations

**`spec/logistics.md` -> Containment is precise and none of it is visible in what you read.** Every
thing is in at most one other; **the game is the one thing that is in nothing, so containment is a
tree**; capacity is per kind with a stored **total**, a derived **used** and a derived **available**.

**What the dump has instead.** Eight per-kind relations where the container is a column:
`{store territory:1 resource:food amount:5}`, `{structure territory:1 structure:extractor count:3}`.
Three consequences, and the third is why you cannot see what you asked to see:

- **Things inside containers are counted, not named.** `structure` gives a count, so no individual
  store or extractor has an identity - you cannot point at one and ask what is in it
- **There is no *thing is in thing* relation at all**, so the tree exists in the specification and
  nowhere in the data
- **Capacity is printed for exactly one case.** `{territory-resource territory:1 resource:food
  capacity:3 density:4 built:3}` is a territory's capacity for extractors of a resource. **No store
  prints a capacity, and used and available are printed nowhere**, though the specification says both
  are derived and cannot disagree with what is there

**Two relations would carry it, and the shape of the first is the decision.** Ids are unique among
things of a kind rather than globally - the *Traits* table says so - so a container reference needs
both a kind and a number, and there are two ways to write one.

**A. A uniform column, so the relation is one shape.**

```
{thing kind:game     id:1}
{thing kind:territory id:1 in-kind:game      in-id:1}
{thing kind:store     id:1 in-kind:territory in-id:1}
```

The tree is readable by one rule and a cycle is checkable by one pass. The cost is two new trait
names, and `in-kind`/`in-id` appear nowhere in the game today.

**B. The container's kind is the column, which is what the dump does now.**

```
{thing kind:store id:1 territory:1}
```

Nothing new is introduced and it matches `{store territory:1 ...}`. The cost is that the relation
has a different column per container kind, so it is not one relation - and reading the tree means
knowing every kind that can contain.

**My recommendation is A**, because the tree is the thing you want to see and B cannot be walked
without already knowing the answer. But it adds vocabulary to a release that `P-284` deliberately
keeps closed, so it is yours.

**The second relation follows the first and is not a separate decision.** Capacity per container,
per kind, with the trait value where there is one:

```
{capacity of-kind:territory of-id:1 for:extractor by:resource value:metal total:3 used:3}
{capacity of-kind:territory of-id:1 for:store                            total:6 used:2}
```

`available` is deliberately absent: the specification says it is the total less the used, and
`P-245` says a document that restates another links to it rather than listing it. **Say if you want
it printed anyway** - a number you can check by subtracting is different from one you must.

**What this is for.** An HTML tree in `reports/`, collapsible with `<details>`, generated from these
two relations - so a collapsed container still reads `3/3 extractors` and you can see at a glance
what is full. `S-54` is the build and waits on this.
