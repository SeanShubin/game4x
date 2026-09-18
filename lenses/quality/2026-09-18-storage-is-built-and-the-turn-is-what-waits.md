# Storage is three-quarters built, and the turn is what the rest is waiting for

**Derived.** 2026-09-18, reading `prototypes/thin-engine/backlog.md` against the tree after Sean
asked which concept comes next. Not binding.

[Quality](README.md) · [The backlog](../../prototypes/thin-engine/backlog.md) · [Logistics](../../spec/logistics.md)

**How every number here was produced** is stated where it is used, so each can be re-run rather
than trusted. Nothing here was read from a previous report.

## The short version

**The backlog's Storage block names the sum as what blocks the rest, and the sum was implemented in
the commit that wrote that sentence.** Two of its four hard rows are rows-only today. What the
other two are actually waiting on is **the turn**, and so is the readiness already in the schema -
which makes the turn the concept with three things behind it rather than one.

## The sum is done, and the sentence that says otherwise shipped with it

**Where.** `prototypes/thin-engine/backlog.md` line 28: *The one that blocks the rest is the sum.
Everything above the line is rows; everything below needs the engine to add numbers it is currently
only comparing.*

**What.** It is not true at `HEAD`, and was not true in the commit that wrote it. `f5687d24` added
`backlog.md` (64 lines) and `src/schema.rs` (+135) in one commit, and the 135 lines are the sum:
`counted`, at `prototypes/thin-engine/src/schema.rs:754-786`, accumulates
`how_many.parse::<i64>() * rate` per place, for providers and consumers alike. A weighted sum over
the things present is the whole mechanism.

**The lane knew.** `850388e1`'s own message says it: *a provider either is the place or is in one,
so `{provides kind:store what:metal} -> 10` gives a place capacity as the sum of what is in it that
provides - `spec/logistics.md`'s sentence, **with no further engine***. The doc comment at
`schema.rs:697-699` says the same thing and names a store as the example. **Only the backlog line
did not move**, and it is the line a reader consults to decide what to do next.

**Why it costs something.** It is the ordinary way an item goes stale: nothing edited it, the world
moved under it. A reader planning from that line concludes storage needs engine work first, and
defers the concept that is cheapest.

## The store shape reintroduces what `6b642e4f` removed, and row three does not work with row one

**Where.** `backlog.md`, the Storage table, rows one and three.

**What.** Row one offers `{store where:territory-1 what:metal} -> 4` - *one kind keyed by
`(where, what)`* - and calls it *a kind like any other*. **It is not.** Every kind in
`data/friendly/schema.4x` is its own relation keyed by `where` with a quantity: `scout`,
`transport`, `metal`, `food`, `labor`. `6b642e4f` is the commit that made them so, on Sean's
observation that a deposit, an extractor and a food in one territory were *specified differently* -
and it deleted `residency` and `thing`, which are exactly the shape row one proposes.

**And the two rows collide.** `provides` is keyed by `(kind, what)` where `kind` references a
**relation** - `schema.4x`, `{reference id:48 column:90 to:relation}`. So
`{provides kind:store what:metal}` says *every store row provides metal room*, food stores
included. The design cannot distinguish them, because the distinguishing fact lives in a column
rather than in the relation.

**Row two is the same cause seen as a difficulty.** It asks for *a cap per description, not per
relation*, because the allowance differs by the store's `what`. Make `store-metal` and `store-food`
separate relations - the established convention - and the allowance differs **per relation**, which
is what the existing mechanism already does. The requirement dissolves rather than being met.

**Whether.** Worth knowing before the rows are written, and cheap then. Not worth a change to
anything today.

## The pool refuses; the spec loses at the turn's end

**Where.** `prototypes/thin-engine/src/engine.rs:243`, and `spec/logistics.md`, *Containment*.

**What.** `engine.rs:243` runs `schema::check` over the world **after** a rule fires and maps a
failure to `Refused::Broke`. So a supply exceeded **refuses the rule**. `spec/logistics.md` says
something else about resources: *at the turn's end what the place holds beyond that capacity is
lost*. The metal is produced and then lost; the `work` that produced it is not refused.

**Why this is about ordering rather than a defect.** Refusal is right for berths - a unit that will
not fit does not move - and `provides`/`consumes` was built for berths. It is wrong for resources,
and nothing in the prototype yet distinguishes a supply that refuses from one that spills. **So the
storage rows that look free are free only for capacity that refuses**, and the row the backlog
already marks *the turn, and a comparison* is where the spec's actual semantics live.

**This makes the dependency run the other way from the backlog's order.** Storage is listed first
and the turn second; the half of storage that is not already built is behind the turn.

## What the turn is, in engine terms, and why it is the largest untested thing

**Every rule in the prototype fires because a `{when}` line names it with its inputs**, across all
seventeen tests - `data/friendly/tests/*.4x`, each `{when}` block, counted by reading them.

**`spec/data/block.4x` has 36 blocks: 10 `owner:player` and 26 `owner:world`** - counted with
`grep -o "owner:[a-z]*" | sort | uniq -c`, summing to the file's 36 rows. The three rules built so
far - `move`, `build-extractor`, `work` - are all player blocks. **So the prototype has tested
"can data describe a rule?" three times and "can data describe a rule that fires by itself over
every match?" not once**, and that second question covers 26 of the 36 blocks.

**Readiness is the smallest case of it and is currently a dead mechanism.** `working` appears in
`data/friendly/rules.4x` exactly once, at line 81, as a `remove` clause of `work`. **Nothing adds
one.** Six of the seventeen tests hand-place a `working` in `given` for that reason, and a built
extractor is worked as many times as its given allows and then never again. `{limit held:working
by:extractor}` already caps it, so the missing half is not a comparison - it is a place to fire
from.

**The refresh rule itself needs no new data mechanism**, which the backlog says and this lens
checked: remove the `working` rows, require the extractor, add a `working` whose quantity is read
off the extractor count - `{reading ...}` already does that, in `rules.4x`, for `work`'s density.

## On the operators, a correction to the backlog's framing

**`spec/data/constraint.4x` has 28 rows and four operators** - 7 `at-least`, 8 `at-maximum`, 6
`exactly`, 7 `one-less`, summing to 28, from `grep -oE "compare:[a-z-]+" | sort | uniq -c`.

**Half are already expressible.** `at-least n:1` followed by `one-less` is require-then-remove,
which is precisely what `work` does with `working` - 14 of the 28 rows are that pair. What is out
of reach is `at-maximum` (8) and `exactly` (6), and `exactly 0` is a require-absent the engine has
no role for. **The backlog's *the prototype has none of them* is the pessimistic half of a true
sentence**, and the optimistic half changes what building the turn would cost.

## Corrected 2026-09-18: most of them are not comparisons, and the difference is one role

**The section above reads `constraint.4x` without reading the recipe rows the constraints attach
to, which is this lens making the mistake it named in the same report.** The `compare` column is
doing two jobs, and counting it as one produced a cost estimate that is too high.

**15 of the 28 sit on `put` rows** - every one of the 8 `at-maximum`
(`grep -E "at its maximum" releases/first-release.md` is 8 rows and all 8 are `put`) and every one
of the 7 `one less` (lines 229, 257, 261, 268, 275, 291, 294, all `put`). **A put writes a value,
so those are not tests.** *Moving at its maximum* is what the row assigns, not something it checks.

**The 6 `exactly n:0` are mixed**, which is the tell: `hold` has `require nature met 0` and
`renew` has `put nature met 0`, the same `compare` cell on both sides of the distinction.

**So the engine needs one comparison and one role, not four operators.** The comparison is
`at-least n:1` on require rows, 7 of them, which require-then-remove already gives. The role is
`put`, **the only one of the release's four the prototype has not built** - counted across the
recipe table, 92 role rows are 32 `consume`, 22 `require`, 21 `produce`, 17 `put` and **0 `limit`**,
and the prototype's require/remove/add cover 75 of the 92.

**The practical difference.** Refresh was placed behind *comparison* in the backlog's ordering and
in the section above. It is not behind it at all: a put writes the maximum, and writing 1 where 1
already stands is a no-op, so *tops off whatever has less* needs no operator to notice the *less*.

## Whether

**Worth acting on now, as a choice of what to build rather than a repair.** The three stale lines
cost nothing until somebody plans from them, and the plan is what was being asked for. No change to
code is proposed here.

**This lens's recommendation, which is a recommendation and not a finding**: the turn, at its
narrowest - a rule that fires over every match with no command naming it - with refresh of
`working` as its one worked example. It ends a dead mechanism, it is the first reading the
instrument has taken of the automatic half of the game, and the two storage rows that are not
already free are behind it.
