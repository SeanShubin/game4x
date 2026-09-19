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

|                                                                                                              | What it needs                                                                                                                                                                                                           |
| ------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ~~**A store is a thing a place holds**~~ - **built** as `{bin where:territory-1 what:metal} -> 4`            | nothing, and nothing is what it took                                                                                                                                                                                    |
| ~~**A territory holds only so many stores of each resource**~~ - **built** as `{limit held:bin by:capacity}` | **nothing, and this entry was wrong about that.** It said the cap had to be per description and that the pool could not say it - both true, and both about the pool. The limit keys on `(where, what)` and always could |
| **A store contributes capacity for its resource** - `{provides kind:store what:metal} -> 10`                 | summing a product over the things present - the first real arithmetic                                                                                                                                                   |
| **A vehicle carries capacity that travels with it** - a transport holds 10 metal and 2 fuel                  | the same sum, over a kind that moves                                                                                                                                                                                    |
| **What a place holds beyond its capacity is lost at the turn's end**                                         | the turn, and a comparison - see below                                                                                                                                                                                  |
| **A thing that leaves takes what it hauls**, defaulting to a full load                                       | allocation at the moment of leaving, and a command that may name an amount                                                                                                                                              |

**The one that blocks the rest is the sum.** Everything above the line is rows; everything below
needs the engine to add numbers it is currently only comparing.

**Two rows are struck out and the first four words of this section are why**: *most of this is
already in `spec/logistics.md`* was true, and so was the part nobody checked - the mechanism was
already in `schema.4x`. **The entry that said what was needed had named the wrong instrument**, and
it took looking at `held_within_what_holds_it` to find out. That is worth remembering the next time
an entry here says a thing needs building.

**The thing is a `bin` and not a `store`.** `store` is the engine's own word - `src/store.rs`, and
a bare word in five of the eight modules - so `tests/isolation.rs` cannot hold *no game noun
appears in code that runs* for it. `spec/logistics.md` draws the same line: *A bin is a thing. What
holds a kind is a store for that kind.*

## The turn

**Nothing in the prototype has an owner.** `spec/data/block.4x` has 10 player blocks and 26 world
ones, and the world's are the whole automatic half of the game: upkeep, breed, perish, age, spoil,
refresh, muster, hold, reclaim, renew, take.

**`end-turn` exists and restores every count**, so the last of `spec/turn.md`'s five steps is
built and the other four are not: everything with upkeep pays it, a population grows or starves,
what expires expires, and nature takes back what is no longer held. **Each is a rule the order has
room for**, which is what the tree buys - adding one is a `{part ...}` row and a leaf, not a change
to `end-turn`.

**Two of the four need something the engine has not got.** *Everything with upkeep pays it* is a
sweeping `remove`, and `remove` takes one match where `put` sweeps. *Grows on surplus food or
starves for want of it* is a branch on failure, which is a zero test on a counted place - the
inhibitor arc `docs/designing-rules.md` measures, and `C-75` puts food on the fatal side of.

**What survives a zero test is the invariant, which is worth knowing before deciding.** The
weighting argument is about what a rule does and not about when it may fire, so guards only remove
firings: **nogain stays sound and only becomes conservative.** What a zero test costs is
reachability and termination.

## Refresh, what is left of it

**Omitting a capacity means unconstrained, where Sean said it should mean none.** *If we omit a
capacity, we can default that to mean it may carry none of that thing* - and
`nothing_holds_more_than_there_is_room_for` walks the capacity rows, so a pairing nobody states is
one nobody asks about. **Found by the mutation sweep**, which deleted a capacity row from a test
and found nothing failed, while that test's own prose claimed the row was what made the world
legal.

**The fix is not a small one, and that is the interesting part.** To make omission mean *none*,
the check has to walk the things that are *present* rather than the capacities that are *stated* -
and then every kind in the world needs a capacity or nothing of it may exist. **That cannot happen
while `limit` and the berth pool still govern deposits and units**: scouts would be refused for
having no capacity row, because the mechanism that admits them is a different one. **So the
default Sean specified arrives with the unification and not before it**, which is an argument for
doing the unification sooner rather than a defect in the table.

**The order a composite states is not depended on by anything yet.** Both of `end-turn`'s
`{part ... seq:N}` values can be changed and nothing fails - refreshing `moving` and refreshing
`working` do not touch each other, so swapping them leaves the same world. **`tests/mutation.rs`
records it as not load-bearing**, which is the honest state: the column exists and the tree prints
an order that nothing enforces because nothing needs it enforced. **The four unbuilt steps of
`spec/turn.md` are what will need it** - upkeep must be paid before a population grows on what is
left - and until one of them lands, a test asserting the order would be a test asserting a
coincidence.

**A part cannot be handed its parent's argument.** `{argument ...}` carries a constant, so a
composite that took a `where` and passed it down has no way to say so. **Nothing needs one**:
`end-turn` takes no arguments because time does not visit one place. It is here because it is the
first thing a second composite is likely to want.

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
nothing says which wins. It has not happened, and **the reason it was deferred has gone**: this
said a key of `(kind, pool)` needs a relation keyed by more than its first column, *which the
schema cannot express without a quantity* - and the key rule changed on 2026-09-18, so a relation
with no `id` is keyed by every column but the quantity and the attributes. Dropping the `id` is
now the whole change. **Nothing edited this line when the rule moved under it**, which is the
staleness `CLAUDE.md` describes and the reason the sentence names its rule.

**`assigns` has an `id` it does not need.** With one row left its id is read by nothing, which
`tests/mutation.rs` now records. **Keyed by `(clause, input, value)` instead, it could not state
two assignments of one input on one clause** - which is the rule rather than a restriction, and
the key rule of 2026-09-18 made that key expressible. It is one row's worth of change and nothing
needs it yet.

**A family is silently not a family when the schema is read from the friendly rows.**
`Schema::of` resolves `{family relation:26}` and `{member kind:28 family:26}` through the id-to-name
map, so on the friendly side - where those rows already say `unit` and `scout` - both lookups miss
and `families` comes back empty. **`UnlikeShape` then checks nothing and passes**, which is a green
that means *no families were found* rather than *every member has the shape*. Harmless today:
`tests/directories.rs` is the only caller that hands it friendly rows, and it wants the schema for
the quantity column alone. **It is here because it is the same defect the `carries` check hit** -
`by id or by name` is the pattern the translator uses everywhere, and these two places do not.
**Read from the code, not run**: `named` is keyed by the relation row's `id`, which is numeric in
both notations, and the friendly file writes `{family relation:unit}`. No test would show it,
because a check that finds nothing to check passes.

**`move` offers a unit into a place it cannot fit.** It does not - the pool refuses it - but
`offered` finds that out by firing the rule and catching the refusal, which is the expensive way.
Nothing is wrong; it is here because it is where the cost will show up first.
