# What `S-47` will need looking at, 2026-09-06

**Held, addressed to nobody.** Written before the work it is about exists, so it is not a finding and
nothing acts on it. It is here because **a request that lives only in a message dies with the
session that received it**, which is the rule this lens has spent a week applying to other people's
work.

[Quality](README.md) · [Outbox](outbox.md) · [The last sweep](2026-09-05-sweep.md)

## Why there is no sweep yet

Sean's arrangement is that the code lane signals when its list is empty and the sweep starts then, so
that nothing is reported about work that was going to disappear anyway. On 2026-09-06 the code lane
**declined to signal, and said why**: the gate is green across the workspace for the first time since
`P-257`, and `S-47` is a third built.

**Stopping short was the right call and it is worth recording that it was made on evidence.** Three
of their last six edits needed a second attempt, and a poison they had applied to the working tree
was inert against a check that reads `git show <commit>:<file>` - a green run they came close to
reading as evidence, an hour after filing `C-33` about exactly that. **A lane that reports its own
error rate is doing something no check does for it.**

## What `S-47` changes, as the code lane describes it

Five promotions that change what a data file *is*: containment as a map from a description to a
quantity, ordered so that the same state is the same bytes; every word in a data file being a kind, a
trait, or one of a trait's values; and `Unit.location` moving into containment, because a thing is
not located by a trait.

## The four they asked for a second pair of eyes on

Recorded in their words rather than this lens's, so that the sweep answers what was asked rather than
what was remembered.

1. **Whether the map form's ordering is actually total.** *Same state, same bytes* is a claim about
   every pair of states, and an ordering that is total on the cases a test builds is not the same
   thing.
2. **Whether `read` and `write` round-trip every state**, rather than the ones a test happens to
   build. This is `CLAUDE.md`'s *check the rule over every case, and assert how many cases there
   were*, aimed by the person who will have written the check.
3. **Whether moving `Unit` into `children` left any place that still assumes a flat list.** A
   representation change that compiles is the case where the compiler is not the check.
4. **Whether the three rules that become unrepresentable really are.** Their own framing, and the
   sharpest of the four: *I will have written that claim and it is exactly the kind that reads as
   true.*

**The fourth is the one this lens should spend most on.** An unrepresentability claim is a statement
about everything that cannot be written down, so it has the shape `Q-41` had - a claim of zero, which
proves something only against a population that is not also zero. What it needs is the population:
what a reader would have written before, and what the form now refuses.

## What the sweep will do with them

Re-measure against the baseline in [the last sweep](2026-09-05-sweep.md) rather than starting from
nothing, run the specification lane's target - *which of our checks would still pass if its subject
were deleted* - over whatever is new, and answer the four above by reading rather than by pattern,
because every detector this lens has written this week needed the reading anyway.
