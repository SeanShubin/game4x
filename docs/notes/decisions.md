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

### P-385 - Does deploying onto a colony you already hold give it two more citizens

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, because its first version asked a question you had already settled - **kind** entailed, from `X-12` and `C-87` - **asks** a decision - **into** `releases/first-release.md` -> Recipes

**Your rule settles the mechanism and this is what it leaves.** *Costs are hard, effects are soft* is
`P-386`. Applied to `deploy ark` and `found by land`, `produce 1 garrison` is an effect, the
garrison's capacity of 1 stops it, and the command does not fail.

**So the `limit 0 garrison` row has no work left to do.** It is a hard gate standing in front of a
line that the capacity already bounds - and the two are not the same test. `spec/invariants.md`:
*soft means what holds it will not take another - never there is one already. Those differ wherever
a capacity is more than one, and agree only by accident where it is one.* **A garrison's capacity is
one, so this is the accident**, and the row reads as though it were the rule when the capacity is.

**Removing it changes what a second deployment does, and that is the question.** Both recipes make
seven things. Six of them are bounded and stay soft on their own: two extractors and two stores go
in only where there is capacity, and the garrison does not go in at all. **`produce 2 citizen` is
bounded by nothing** - *What bounds a kind in a territory* gives citizens *the food produced here,
through upkeep*, which is not a capacity to be short of.

| Deploying an Ark onto ground you already hold | Today, with the hard gate       | With the row removed     |
| --------------------------------------------- | ------------------------------- | ------------------------ |
| the ark                                       | **not spent** - nothing happens | **spent**                |
| garrison, extractors, stores                  | none                            | only where there is room |
| citizens                                      | none                            | **+2, always**           |

**Two ways, and the choice is which of them the game is.**

- **The row goes.** An Ark can be unloaded into an existing colony, and what it adds is two citizens
  plus whatever there is room for. The two recipes become identical apart from what is spent, so a
  shared sub-recipe can be extracted - `X-12`, and the first recipe that calls a recipe, which is
  what makes `C-75`'s acyclicity check stop being vacuous
- **The row stays and becomes a cost.** Deploying onto held ground is refused, and the refusal is
  written as something the rule takes rather than as a limit - which is the only shape that is hard
  under `P-386`

**Nobody is asking you to approve an extraction**, and the second question from `X-12` still stands
on its own: your sketch of the founding recipe **omits both stores**, which the release produces.

**What this lane got wrong.** The first version of this asked whether the garrison gate was hard or
soft. **You had settled that and the answer was general** - it is about costs and effects, not about
garrisons - and no file said so, which is `P-386`. The question that was actually open is the one
above, and it is one line rather than a table of readings.
