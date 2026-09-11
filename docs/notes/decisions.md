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

### P-390 - A parameterised readiness, and what its parameter ranges over

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, four times, the last because Sean settled *once per kind of action* and proposed a parameterised token - **kind** entailed, from `S-93` built - **asks** a decision - **into** `releases/first-release.md` -> Kinds, Traits, Recipes

**Settled by you, 2026-09-11**: *my intention has always been once per kind of action.* Recorded
here because no file says it and the release has never had to answer it.

## `ready_to[work]` needs no new notation, because the release already does this

**A kind parameterised by something is a kind plus a trait**, and `extractor` is the worked example:
one kind, a `resource` trait whose values are *one of the resources*, and `build extractor` binds it
as `produce 1 extractor $resource`. **Your `ready_to[work]` is `readiness` carrying a trait that
names the action** - the same shape, a second time.

**And containment already allows the per-action maximum**, which is the half that could have needed
inventing and does not. `spec/logistics.md`: *what a thing may contain is a maximum per kind, per
family of kinds, **or per kind carrying a particular value of a trait**.* So a citizen may hold one
readiness for `create labor` and one for `bear`, and those are two maxima rather than one shared one.

## What it replaces

| Today                                                                        | With a parameterised readiness                            |
| ---------------------------------------------------------------------------- | --------------------------------------------------------- |
| `create labor`: consume 1 citizen `[ready]`, produce 1 citizen `[not ready]` | consume 1 readiness for that action                       |
| `bear`: consume 1 citizen `[fertile]`, produce 1 citizen `[spent]`           | consume 1 readiness for that action                       |
| `refresh`: 2 rows; `renew`: 2 rows                                           | **one rule**, producing a readiness for each action, soft |
| `ready` and `spent` in *Traits*                                              | one trait naming the action                               |

**Twelve recipe rows become five, two traits become one, one kind is added, and `renew` stops
existing** - it was `refresh` for fecundity and nothing else. **The two habits of naming go with
it**, since nothing is named for a full state or an empty one any more.

**It also removes the list this item was originally about.** `nogain.rs` declares which trait pairs
are a capacity because nothing else does; with readiness a kind and its maxima declared as
containment, the check reads them from the release.

## The one choice left: what the parameter ranges over

**Your examples are not all recipes.** `work` and `breed` are recipes; **`operate` is not**. So you
may already mean a named action rather than a recipe, and the two differ in exactly one way.

|       | The parameter is                                        | New vocabulary                | Two recipes can compete for one thing's action |
| ----- | ------------------------------------------------------- | ----------------------------- | ---------------------------------------------- |
| **1** | **the recipe**                                          | none - the recipe's own name  | **no**, and it can never be said               |
| **2** | **a named action**                                      | a closed list of action names | **yes**, by giving both the same action        |
| **3** | **the recipe, unless a recipe names an action instead** | none until a case needs one   | **yes**, and only where you say so             |

**Option 3 is what you said you wanted from vocabulary** - *I prefer unification over a larger
vocabulary of words* - because it adds no name until two recipes actually have to share one. **It is
also the only one of the three that can express both meanings we were arguing about**: every recipe
naming one action is *a thing acts once a turn*, and every recipe keeping its own is *once per kind
of action*. The choice stops being global and becomes per case.

## What this lane has not checked

**Whether a readiness inside a citizen takes room in the territory.** `spec/logistics.md` says *a
thing that contains things takes up capacity in whatever contains it, so capacity is not conserved.*
Under the trait form this never came up; under the kind form it does, and a territory being asked to
declare room for readiness would be a real cost nobody intends. **One sentence decides it and this
lane has not found the sentence** - it is the first thing to check once you have chosen, and it does
not change which option is right.
