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

|                                                                                                                                         | What it needs                                                                                                                                                                                                           |
| --------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ~~**A store is a thing a place holds**~~ - **built** as `{bin where:territory-1 what:metal} -> 4`                                       | nothing, and nothing is what it took                                                                                                                                                                                    |
| ~~**A territory holds only so many stores of each resource**~~ - **built** as `{limit held:bin by:capacity}`                            | **nothing, and this entry was wrong about that.** It said the cap had to be per description and that the pool could not say it - both true, and both about the pool. The limit keys on `(where, what)` and always could |
| ~~**A store contributes capacity for its resource**~~ - **built** as `{capacity of:bin for:resource what:resource per:territory} -> 10` | the sum, which `rooming` does                                                                                                                                                                                           |
| ~~**A vehicle carries capacity that travels with it**~~ - **built**, and it was one row                                                 | nothing. Sean reframed it and the reframing is what made it free                                                                                                                                                        |
| ~~**What a place holds beyond its capacity is lost at the turn's end**~~ - **built** as disorder                                        | no comparison after all: `keep` bounds a quantity rather than testing one                                                                                                                                               |
| ~~**A thing that leaves takes what it hauls**~~ - **dissolved, not deferred**                                                           | nothing is inside a transport, so there is nothing to take. Sean, 2026-09-19: *transport would not automatically take what they haul, they would provide the capacity necessary to move the resource they haul*         |

**Storage is done.** All six are built or struck out, and **the one this section said blocked the
rest - the sum - was the smallest part of it.** What actually cost anything was none of the six: it
was finding that a place's room is a sum over *kinds of container* and not one comparison per
capacity row, and that a resource over capacity is disorder rather than a refusal.

**Two entries here were wrong about what a thing needed, and both in the same direction**: they
named a mechanism that would have to be built when one already existed. That is worth remembering
the next time an entry says a thing needs building - **the entry is a claim like any other.**

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

**Four of `spec/turn.md`'s five steps are built.** *Time restores every count* is `refresh`;
*what was not kept in order is lost* is `discard-disorder`; *upkeep is paid* is `upkeep`; and *a
population grows on surplus food or starves for want of it* is `breed` and `perish`, which are one
step in that document and two neighbouring rules here. **Only *nature takes back what is no longer
held* is missing**, and the order has room for it - adding one is a `{part ...}` row and a leaf,
not a change to `end-turn`.

**`{part ... seq:N}` is load-bearing now**, which this said upkeep would be the thing to make it -
and it was, though by a different pairing. It is `upkeep` before `perish` inside
`end-turn`: swap them and everyone dies.

**Two predictions here were wrong and are worth keeping as wrong.** *What it needs from the engine
is one thing: a sweeping `remove`* - it needed `{repeats rule:R}` instead, which is not a sweep but
a repetition, and covers the sweep as a special case. **And it needs a word for upkeep** - it did
not; what a kind owes per turn is the trait it carries and the clause that spends it, which is the
machinery `moving` already had.

**And the zero test never arrived.** *Grows on surplus food or starves for want of it* looked like a
branch on failure - the inhibitor arc `docs/designing-rules.md` measures and `C-75` puts food on the
fatal side of - and it is not one. A state on the citizen makes each half a plain pattern match, and
the section below is the working.

**What survives a zero test is the invariant, which is still worth knowing.** The weighting argument
is about what a rule does and not about when it may fire, so guards only remove firings: **nogain
stays sound and only becomes conservative.** What a zero test costs is reachability and termination.

**One thing this predicted is built and untested.** `upkeep` runs before `discard-disorder`, so
a citizen eats food that is over capacity - Sean's *use stuff over capacity in other recipies to
avoid the waste*, happening automatically. **No test states food in disorder and then ends a turn**,
so the order is asserted by the tree and by nothing that runs.

## Refresh, what is left of it

**Folding the berth pool into the capacity table needs three things.** Measured on 2026-09-19 by
writing berths as a capacity row and reading what it reified into: a family in `for` gives **one
bucket per member** where berths need one shared, it **cross-products with `what`** into a capacity
for scouts carrying metal, and there is **no rate**, so a transport cannot take two berths where a
scout takes one. **The deposit limit folds without any of them**; the pool does not.

**And that is a design boundary rather than a missing feature.** Sean, 2026-09-19: *Territories
contains structures that don't move so I don't want tradeoffs there, but armies do move and the
tradeoffs of what to move where is the whole point of a military simulation.* **Shared capacity is
the army side and per-kind capacity is the structure side**, so a unification that made everything
one or the other would be changing the game rather than tidying the engine.

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

## Comparison, and why the prototype may never need it

**The engine has no operators.** `spec/data/constraint.4x` has 28 rows using four - `at-least`,
`at-maximum`, `exactly` and `one-less` - and the prototype has none of them.

**It may not need any of them, and working out grow-or-starve is what showed it.** Two of the four
turn out to be things thin-engine already says another way, and the other two turn out to be
repetition:

| the spec's operator                                                    | what the prototype says instead                                                                            |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `exactly n:0` on a trait, as in `perish`                               | a literal in a pattern - `{literal clause:clause-3 column:98 value:1}` is how `move` matches a spent scout |
| `at-maximum` on a trait, as in `upkeep`                                | `{assigns ... value:1}`, which is what `refresh` does                                                      |
| *at least one*, anywhere                                               | `remove` refusing when nothing matches                                                                     |
| a varying amount, as in *the minimum of extra-food and total-citizens* | **a rule that fires as many times as it can**                                                              |

**So the operators are not deferred; they may be unnecessary.** That is worth checking before
anyone builds four of them.

## Grow or starve without a zero test, and the one word it costs

**The starving half is built**, on 2026-09-19 - `upkeep`, `perish`, `repeats` and `scope`. `README.md` -> *Hunger, and a rule that fires as many times as it can* is what it
came to. **What is left here is breeding**, and the reserve, and what the two of them would need;
the rest is kept because it is the reasoning the built half rests on.

**The problem it looked like.** *A population grows on surplus food or starves for want of it* reads
as a comparison, and `docs/designing-rules.md` puts a zero test on a counted place - food - on the
fatal side of decidability: *An unbounded place cannot [be zero-tested]. If there is no food, if the
store is empty - these are the cliff, and they are exactly the tests a resource game invites.*

**Sean's move, 2026-09-19**: *Can starvation be represented as the recipe: (citizen, food) ->
(citizen). Can growing be represented as the recipe: (citizen, excess-food) -> citizen. All without
a zero test?*

**Yes, with a state on the citizen.** As written the recipe gives the citizen back unchanged, so it
can fire again on the same one - one citizen eats all the food and nobody dies. With a state, each
rule is a plain pattern match and nothing tests for absence:

```text
upkeep   citizen[hungry:1] + food  ->  citizen[hungry:0]
perish   citizen[hungry:1]         ->  gone
bear     citizen[bearing:1]        ->  citizen[bearing:0] + fertility
breed    fertility + food          ->  citizen
discard  fertility                 ->  gone
```

**And excess is a position in the order, not a comparison.** Breeding runs after upkeep, so whatever
food it finds *is* the surplus. Nothing anywhere says *more than*.

**The mainline spec reaches the same answer**, which is worth knowing before anyone thinks this is a
prototype-only trick. `spec/data/line.4x` and `spec/data/constraint.4x`:

```text
{line block:upkeep seq:3 role:put kind:citizen}
{constraint block:upkeep seq:3 trait:paid compare:at-maximum}
{line block:perish seq:1 role:consume qty:1 kind:citizen}
{constraint block:perish seq:1 trait:paid compare:exactly n:0}
{line block:breed seq:1 role:consume qty:1 kind:fertility}
```

**`paid` is the same state and `fertility` is the piece Sean's version was missing** - without it one
citizen breeds with all the surplus, the same defect the starvation recipe had before its state bit.
**`min(extra-food, total-citizens)` falls out of consuming one of each**, and *at most doubling* is
one fertility per citizen.

**The polarity is inverted here and that is deliberate.** `refresh` restores a trait to **1**, so
`paid` would be restored to *already paid*. `hungry` restores correctly and is then exactly `moving`
and `working` - an allowance the turn restores and acting spends. **The only difference is what
unspent means**: a scout that did not move is fine, a citizen that did not eat is dead.

**A citizen's food reserve is capacity, not a counter on the citizen.** Sean, 2026-09-19, asked
whether starvation needed a richer state - *thematically a citizen has an amount of food in their
body. When they are capped they are not hungry. When they are not capped but not empty they are
hungry. When they are 0 and can't get more at end of turn they starve.*

**It does not change the representation, and a counter would be expensive.** A `fed` level of
`0..cap` needs decrement, and `put` assigns a constant - so going from `fed:2` to `fed:1` is a rule
per level, or arithmetic on trait values. **The reserve belongs in capacity instead**, which is
already built:

```text
{capacity of:citizen for:food what:food per:territory} -> 3
```

Five citizens is room for fifteen food, which is three turns of reserve, **kept orderly so disorder
does not take it**. Run out of production and upkeep eats the reserve down over three turns before
anyone starves. **`hungry` then means exactly one thing: did you eat this turn.**

**The middle state is the food count rather than a citizen state**, which is the number a player
wants to read anyway: it says how many turns are left.

**And a per-individual belly is the wrong scale.** `spec/population.md`: *A citizen is not one
person. It is the smallest group that can sustain reproduction.* The reserve is a settlement's
larder. **What this gives up is deliberate**: some citizens full while others starve in one
territory cannot be said, because which ones went short is not a question the model can ask - which
is *we only lose what we don't have the storage for, without tracking what is stored where*, one
level down.

**What it cost the engine was two words, `{repeats rule:R}` and `{scope rule:R input:I}`**, and both
are built. **`repeats` replaced three things that were each written down as needed** - a sweeping
`remove`, a comparison operator, and arithmetic - because every rule that wanted one of those wanted
the same thing: to happen as many times as it could.

**The paragraph that stood here said this was not a termination proof**, on the grounds that a rule
removing one and adding two stops for neither reason. **The snapshot is what makes it one**: a
firing draws from the world as it was when the repetition began, so a rule that removes one and adds
two still takes one out of a pool that only shrinks. `Malformed::NeverStops` is the other half,
refusing a repetition with nothing to consume.

**And *as many times as it can* includes zero**, which turned out to matter more than the varying
amounts it was added for. It is how a step of the turn does nothing in a world it has no business
in.


## Breeding, built 2026-09-20, and the two tests it still wants

**Sean's loop, 2026-09-19**, of which everything but the food extractor is now the turn:

> citizen works food extractor / food extractor generates enough food for more than one citizen /
> each citizen cosumes 1 food or perishes / **each remaining (citizen, food) produces an additional
> citizen**

**It is what `upkeep` already is, one trait along, and it landed as written:**

```text
breed   remove citizen[hungry:0 bearing:1], remove food  ->  add citizen[hungry:0 bearing:0] x2
```

**Two things the mutation sweep says no test reaches**, both the same shape and both worth a test
rather than a shrug:

- **`breed`'s food binding** - the clause tying the food it consumes to the place it acts in. Both
  two-territory tests have run out of food by the time breeding runs, so nothing states a world
  where breeding happens in two places at once and the surplus must not travel. **`upkeep` has
  exactly that test** - `one-territorys-food-does-not-feed-anothers-citizens` - and breeding has
  no equivalent.
- **Two of `breed`'s literals**, for want of a world where the value they name is what tells two
  citizen rows apart.

**`bearing` is what stops one citizen breeding with the whole surplus**, the same way `hungry` stops
one citizen eating all the food - and it is a capacity rather than an obligation, so it is a gerund.
**Removing the parent and adding two is how it gets its own `bearing` spent** without a `put` beside
a `remove`.

**It goes into the turn after `perish`**, and *excess* is then a position in the order
rather than a comparison: whatever food is left when breeding runs is by construction the surplus.
The increase is `min(food to spare, citizens that can bear)` with nothing computing a minimum.

**What it cost every citizen row is a second trait**, and it cost `upkeep` a clause. An `add`
must name every column, so eating had to say what the fed citizen's `bearing` is - and writing `1`
would have been correct only because the turn restores it last, quietly handing a free breeding to
any citizen that ate with its bearing already spent. **So `upkeep` reads the parent's bearing from
a `require`**, which is the shape `work` uses for a deposit's density, and the coupling fails loudly
instead of silently: two citizen rows differing in `bearing` make the reading ambiguous and `NotOne`
says so.

**Sean, 2026-09-19, on keeping the two apart**: *keeping breeding and hunger separate is the right
call, the apparent connection is coincidental.*


## Orbit as part of a territory, not a territory of its own

**Sean, 2026-09-20**: *I no longer think orbits should be separate territories with adjacencies
like they are in the mainline spec. I think orbits should be part of the territory. So we need a way
to tell if something is in orbit, and the orbital area will have different containment rules than
the surface.*

**The mainline already agrees on the main point, which is worth knowing before anything moves.**
`spec/orbit.md`: *An orbit is not a territory: it has capacity for no extractors, and nothing is
extracted there.* And `spec/console.md`: *A place worked out from another is not open - the orbit
above a territory is named by naming the territory.* **So this is choosing a representation for
something the specification already asserts**, not overturning it. What the spec does treat as a
graph is only crossing: an orbit is next to the territory below it and next to the orbits above that
territory's neighbours, and a unit crosses orbit boundaries or crosses none.

## The containment difference is a split already drawn

**Surface is per-kind capacity and orbit is shared capacity.** Sean, 2026-09-19, on why the berth
pool and the capacity table should not be unified: *Territories contain structures that don't move
so I don't want tradeoffs there, but armies do move and the tradeoffs of what to move where is the
whole point of a military simulation.* That is the two layers, and *Refresh, what is left of it*
above calls it a design boundary rather than a missing feature. **Orbit is what gives the boundary a
name.**

**And `capacity.per` is the column for it.** Sean asked in September whether `per` would always be
`territory` and nothing then could produce a case where it was not. **This is the case.**

## What Sean's answers settle

|                                         |                                                                                                                                                                                                                                                                                                        |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Can a citizen be in orbit?**          | **No.** So citizens never need to know layers exist - `upkeep` scoped per *place* behaves exactly as per *territory*, because orbital places hold no citizens and the repetition fires zero times there                                                                                                |
| **Does orbit match the surface graph?** | **Yes.** So no new adjacency rows: all three moves consult the territory graph that is already there                                                                                                                                                                                                   |
| **Is crossing layers a move?**          | **Undecided, and thematically both.** Sean: *thematically it is a move in the sense of changing position, but also the moves are very different kinds of things, moving between two orbits, moving between two surfaces, and moving between surface and orbit all require very different capabilities* |

**Two and three together mean the graph is one thing and the capability is three.** Surface to
surface and orbit to orbit are the *same* adjacency clause; surface to orbit is the two places
sharing a territory. What differs is which capability the rule demands.

## Where names a place, and what that costs

**Sean is leaning towards places as rows** - `{place id:1 of:territory-1 layer:surface}`, with
things saying `where:place`. **No clause changes**, because every clause already binds one `where`,
and it is what `spec/orbit.md` describes: the orbit is derived from the territory rather than listed
beside it.

**The cost it hides is getting from a place to its territory.** `move` binds `from:place to:place`
and the adjacency clause wants the territories, which is a join - `{reading ...}`, the machinery
`work` uses for a deposit's density and `upkeep` now uses for a parent's bearing. **Two readings,
one per side**, landing in the rule that is already the most complicated.

**The alternative is adjacency between places**: no readings, three times the rows, and an
`adjacency.kind` column to keep the three moves apart. **Rows in the world against readings in the
rule** - and the readings look right, because the graph stays one thing.

## Three steps, because the review has to be one kind of change at a time

**Sean, 2026-09-20**: *this will require just about every test changed. I will want to ensure this is
the only change when I review the tests so I can do so quickly without mixing other kinds of
changes.*

1. **Places exist and nothing else changes.** Every territory gets one surface place; every
   `where:territory-1` becomes `where:surface-1`. No new rule, no capacity change, no orbit, and
   **every test's answer byte-identical to today's**. Per test the diff is two mechanical shapes -
   one added `{place ...}` row per territory, and renamed `where` values - and no others. **It
   cannot be one shape**, because a place depends on a territory and tests state territories.
2. **Orbital places**, and `capacity.per` telling the two layers apart.
3. **The move rules**, and whichever answer step 3 above gets.

**Step two forces an open item to be settled.** *It has capacity for no extractors* is a capacity of
zero, and omitting a capacity currently means unconstrained - see *Refresh, what is left of it*,
where Sean already ruled it should mean none. **It was deferred for want of a user and orbit is the
user**: an orbit saying nothing about extractors would admit any number of them.

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

**Nothing refuses a scoped rule that is nobody's part.** A `{scope ...}` input is the engine's to
fill, so a rule that declares one is world-owned and belongs under the turn - and `offered` decides
what a player may choose by skipping anything a part names, which is a different question that
happens to give the right answer today. **Made a root, `upkeep` would be tried with `where` unbound,
refused, and quietly left off the menu.** The fix is either a check when the world is read or
`offered` skipping a scoped rule outright; **which one is worth writing is not clear yet**, because
the menu a player is shown and the rules the world owns may want to be the same statement.

**`Refused::NotOneToTake` is reachable by no rule.** It refuses a `remove` that several rows answer,
and every clause names every trait it means - which is what avoids it. **A refusal nothing reaches
is not the same as one nothing needs**: it is what makes a clause that says too little fail loudly
rather than pick, and the clause that says too little has not been written yet.
