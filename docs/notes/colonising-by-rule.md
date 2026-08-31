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

**This is a third use, and it changes that arithmetic.** *Unclaimed territories I can reach, minus
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
