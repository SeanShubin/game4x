# Colonising By Rule

**Derived.** Written by Claude from conversation, 2026-08-31. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Control without tedium](control-without-tedium.md) · [The game as tables](the-game-as-tables.md)

Sean's first worked example of the rule system: *I am going to create a pioneer, then create a set of
rules for that pioneer, then every new pioneer is going to use that set of rules by default, causing
me to explore and colonize the entire planet by setting only two rules.*

Two rules, and the interesting part is the problem he spotted in them.

## The two rules

- **`{pioneer}`** - go to a territory that can be claimed, and claim it
- **`{territory, founded}`** - produce a pioneer while there is somewhere left to claim

**Neither mentions the other, and neither names a territory or a unit.** That is what makes two rules
enough for a planet.

## "Every new pioneer uses it by default" costs nothing

This is the part that would be expected to need machinery and does not. **A rule does not attach to an
instance; it matches a pattern.** *For each thing matching `{pioneer}`* applies to a pioneer built ten
turns from now, because that pioneer carries the same trait. Nobody attaches anything, nothing is
inherited, and there is no default to configure.

It falls out of `P-134` - the state is things with traits - and of the predecessor's `Thing.isPartOf`,
where a partial thing is a query. **The generality is in the pattern rather than in a mechanism.**

## The contention Sean spotted, and what actually causes it

> We don't want them both creating one, but that means the territories have to know what each other
> are doing, or we have to make order matter somehow.

**Neither is needed. The condition is simply wrong.**

*Produce a pioneer while there is somewhere left to claim* conditions on **opportunity**. Territory A
fires and builds one - and there is still somewhere left to claim, because the pioneer has not
arrived. So B fires too. **Evaluating one at a time does not help**, since the opportunity is still
there after A acts.

The condition has to count **commitments**:

> Produce a pioneer while the places left to claim outnumber the pioneers already on their way.

Now A fires, and B evaluates against a world holding one pioneer and one unclaimed territory. The gap
is zero, so B does nothing. **No territory needs to know what another is doing - it needs to see what
exists, and the shared state is the coordination.**

**This generalises past pioneers.** Any rule that *creates* something must condition on the gap
between what is wanted and what is already accounted for, never on the want alone. It is the standard
cause of duplicated work in anything autonomous, and it is a property of the condition rather than of
the scheduler.

## The gap has to be local, which is the refinement Sean's version needs

A global count is the right *shape* and the wrong *scope*, because a pioneer cannot go far. `P-131`
gives it two cells at one per move, and `releases/first-release.md` says a Pioneer that leaves a
territory you control must found the one it enters. **So it can cross at most one territory you hold
and then found**: reach two, and only the last step may be onto unclaimed ground.

So a territory that is nowhere near the frontier builds a pioneer that can never arrive. Counted
globally: two territories unclaimed, both beside `A`, `B` far away and idle - `A` builds, the gap is
still 1, `B` builds a pioneer that dies. **The count has to be over what this territory can reach**:

> Produce a pioneer while the territories *I can reach* that are unclaimed outnumber the pioneers
> already heading for them.

Which is the same rule with adjacency in it, and adjacency is a derived relation
([the pairs the tessellation generates](complexity-against-the-predecessor.md)).

## Order still matters, for who and not how many

With the gap condition, order decides **which** territory builds the pioneer and never **how many**.
That question is already settled: `crates/game-model/src/game.rs:264` picks the lowest-numbered
candidate *so the choice is data-derived rather than an accident of iteration order*, citing
`docs/architecture.md` rule 9, which forbids depending on execution order at all.

**So the tie-break exists, is principled, and predates the rule system.**

## What this costs the derived-kinds decision

`P-132` recorded that two rows need things that are not kinds - *node, unworked* and *food, surplus* -
each a comparison between two counts, and said *each appears exactly once*, which was the measure of
what choosing derived kinds over comparisons was worth.

**This is a third use, and it changes that arithmetic** - though see the section below, which
removes the need for it in this particular rule.** *Unclaimed territories I can reach, minus
pioneers heading for them* is the same construct again, and it is not a corner of the release - it is
the condition on which the whole of automation turns. The predecessor needed exactly this and wrote
it as `less-than {gatherer resource:food} {node resource:food}`.

**Two rows was a reason to take the cheap answer. A rule system built on it is a reason to take the
right one**, and which is which is worth re-asking now rather than after the first rules are written.

## What is still unanswered

- **How a pioneer chooses among several claimable territories.** *Go to a territory that can be
  claimed* names a set, not a member. This is the priority-list shape - jungle, then grassland, then
  forest - and it is the same construct that both decides and chooses.
- **Whether a rule can see another territory's contents at all.** The gap condition needs to count
  pioneers that are not here. Nothing crosses a territory boundary in this release, but that is a rule
  about *resources moving*, not about *what can be looked at*, and the distinction has never been
  drawn.

## Defaults, and the parallel answer that is already written down

Sean: *I was thinking of solving this with defaults - territories resolve in the order they were
claimed, and searching for a territory is breadth first starting north and going clockwise... a
consequence is that we can't run everything in parallel. I would like to come up with a solution that
allows me to run everything in parallel.*

**The defaults are right and the consequence does not follow.** `docs/layers.md` section 5.3 already
answers this, and lists his case almost word for word:

> **Two armies claim one region.** Do not let the first mover win. Gather all claims, then resolve
> over the set: highest strength, tie-broken by owner id, then by intent index.

**Gather, resolve, apply.** Gather is read-only and embarrassingly parallel. Resolve is a pure
function of the *multiset* of proposals, with tie-breaks from data. Apply writes each piece of state
from exactly one source. **Nothing about it is sequential**, and it is what the document calls *the
answer to everything that looks stubbornly sequential*.

### The defaults are not what prevents parallelism - they are what makes it deterministic

A resolve step has to pick between competing proposals **without asking who went first**, which means
it needs an ordering that comes from the data. *Territories resolve in the order they were claimed*
is exactly that: claim order is in the history, so it is data-derived, and `docs/architecture.md`
rule 9 asks for precisely this - *canonicalise the result by sorting on a data-derived key rather
than ordering the work.*

**So the two ideas compose rather than conflict.** Without a default tie-break, parallel resolution is
non-deterministic. With one, it is a pure function. Sean's defaults are the missing half of the
parallel answer, not the price of giving it up.

### And it makes the rules simpler than the gap condition does

The section above proposed conditioning on the gap - *unclaimed territories I can reach, minus
pioneers already heading for them*. **Gather-and-resolve is better, and the reason is where the
coordination sits.**

A gap condition puts coordination **in every rule**: each one must count what others have already
committed. Gather-and-resolve puts it **in one place**, and the rules get shorter:

- **The rule becomes**: *I am beside unclaimed ground - I propose a pioneer aimed at territory `X`.*
  No counting of anybody else's pioneers.
- **Resolve becomes**: at most one proposal per target survives; ties go to the territory claimed
  earliest.

**Naming the target turns contention into a duplicate**, which is a much simpler thing to resolve than
an allocation. And it is something the rule has to say anyway, because a pioneer has to go somewhere.

### The reason not to defer it, which is about the rules and not the engine

Sean: *perhaps not in first release.* **The cost of deferring is not the engine, it is the rules
written meanwhile.** Sequential evaluation needs gap conditions in every creating rule; gather and
resolve needs none of them. Defer, and the first rules are written with counting in them and have to
be rewritten without it later.

**And the machinery is not obviously more work.** Sequential is *evaluate, apply, repeat until nothing
fires*; gather-resolve-apply is *evaluate all, resolve, apply, repeat until nothing fires*. The loop
is the same shape with one pure function inserted.

### Player-changeable defaults are `P-113` one level down

Sean wants the defaults discoverable and changeable: *that way all the rules are player discoverable
and player changable.* **That is the promoted invariant applied to the tie-break itself** - `nothing
plays itself; every behaviour that acts on a player's behalf is a rule some person wrote`. A hidden
tie-break is exactly the auto-scouting complaint in miniature: something decided on the player's
behalf that they cannot see or change.

So *breadth-first from north, clockwise* should be a rule in the same list as the others, not a
constant in the engine - and `game.rs:264`'s lowest-numbered pick becomes a default rule rather than a
hardcoded choice.
