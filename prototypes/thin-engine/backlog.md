# Backlog

**What is wanted and not built, in the order it would go.** Sean, 2026-09-17: *don't bombard me
with excessive tests, keep it below 5 at a time. Make a backlog if you need to.*

**This is not a plan and nothing here is promised.** It is where a thing goes when it is described
and deferred, so that describing it does not mean building it. An item leaves by being built or by
being struck out with a reason.

## Storage

Sean, 2026-09-17: *We will have structures that can store food/metal/energy. We will also have
vehicles that can do this as well.*

**Most of this is already in `spec/logistics.md`**, which is worth reading before designing any of
it - a place's capacity for a kind is the sum of what is in it that can hold that kind, a place
declares none of its own, and what a place holds beyond its capacity is lost at the turn's end.

|                                                                                                                       | What it needs                                                                                                                                                    |
| --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **A store is a thing a place holds** - `{store where:territory-1 what:metal} -> 4`, one kind keyed by `(where, what)` | nothing; it is a kind like any other                                                                                                                             |
| **A territory holds only so many stores of each resource** - 4 metal, 3 food, 5 energy                                | **a cap per description**, not per relation: the allowance differs by the store's `what`, and the weighted pool cannot say that because a pool is named per kind |
| **A store contributes capacity for its resource** - `{provides kind:store what:metal} -> 10`                          | summing a product over the things present - the first real arithmetic                                                                                            |
| **A vehicle carries capacity that travels with it** - a transport holds 10 metal and 2 fuel                           | the same sum, over a kind that moves                                                                                                                             |
| **What a place holds beyond its capacity is lost at the turn's end**                                                  | the turn, and a comparison - see below                                                                                                                           |
| **A thing that leaves takes what it hauls**, defaulting to a full load                                                | allocation at the moment of leaving, and a command that may name an amount                                                                                       |

**The one that blocks the rest is the sum.** Everything above the line is rows; everything below
needs the engine to add numbers it is currently only comparing.

## The turn

**Nothing in the prototype has an owner.** `spec/data/block.4x` has 10 player blocks and 26 world
ones, and the world's are the whole automatic half of the game: upkeep, breed, perish, age, spoil,
refresh, muster, hold, reclaim, renew, take.

**And it is what readiness is waiting for.** `work` spends a `{working ...}` and nothing puts one
back, so a built extractor starts unready and stays that way. The refresh rule is expressible
today - remove the row, require the extractor, add a `working` whose quantity is read off the
extractor count - and there is nowhere to fire it from.

## Refresh, what is left of it

**A fifth test: refresh where there is nothing to refresh.** The four that exist cover a group
already topped off, which is what makes `refresh-makes-one-entry` come to five rather than refuse -
but none refreshes an empty place, and that is the case `put` exists for over a remove and an add.
Sean capped the set at four and said this is the one to add if another is dropped.

**A per-kind maximum, and the word for it.** Every maximum is 1, so it is a literal in the refresh
rule and nothing checks that a world stating `{scout ... moving:9}` is wrong. Sean deferred both the
word - `readies`, `allows`, `affords` - and whether it unifies with capacity, and deferring costs
nothing while every maximum is 1.

## Comparison

**The engine has no operators.** `spec/data/constraint.4x` has 28 rows using four - `at-least`,
`at-maximum`, `exactly` and `one-less` - and the prototype has none of them. It gets *at least
one* from `take` refusing, and everything else is out of reach: `at-maximum` is every refresh and
`exactly 0` is perish, hold, reclaim and renew.

## Smaller things, each with the reason it is not done

**A constant limit of the plain sort.** `{limit container:territory contained:garrison n:1}` -
one garrison per territory, no weighting. The weighted pool does this with a rate of one, so it
is probably not a separate mechanism; it is here because nothing has needed it yet.

**`energy` as a resource.** Declared nowhere. `spec/data/line.4x` has `move` consuming one energy
at the place it leaves, and the prototype's `move` is free.

**A `draws` row is keyed by its id**, so two rates for one kind and one pool are both legal and
nothing says which wins. It has not happened; a key of `(kind, pool)` would need a relation keyed
by more than its first column, which the schema cannot express without a quantity.

**`move` offers a unit into a place it cannot fit.** It does not - the pool refuses it - but
`offered` finds that out by firing the rule and catching the refusal, which is the expensive way.
Nothing is wrong; it is here because it is where the cost will show up first.
