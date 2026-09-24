# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-549 - Does launching an Ark leave one in orbit? Four statements, and two say no

**to** sean · **status** open · **raised** 2026-09-24 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `releases/first-release.md` -> Recipes, and `spec/data/line.4x`

**This decides whether there is an Ark to move**, which is what you asked about. Nothing else
between you and that test is a rule question.

## The four statements

```
releases/first-release.md   launch ark ... produce | 1 | ark | above $where
spec/data/line.4x:33        {line block:launch-ark seq:6 role:produce qty:1 kind:ark
                                  place-above:where}
scenario/commands/play.4x   "P-342 made launching one recipe that pays an Ark's cost at a
      :170                   Yard and puts nothing into orbit, so there is no Ark to move"
the ledger                  "P-342, produce ark becomes launch ark, and stops producing
                             anything"
```

**The first two agree and the last two agree, and the pairs contradict each other.**

## What this lane measured and what it did not

**Measured**: those four texts, as they sit on disk today. And `scenario/expected/play.4x`
contains no Ark at all - though that file has not been reseeded since `P-512`, so it is weak
evidence either way.

**Not measured**: which is intended, or which changed last. **A search for the release row's
history matched the padder rewriting column widths rather than the rule**, so the log cannot
attribute it, and this lane will not guess from a ledger title.

## The two answers

**`K1` - launching leaves an Ark in orbit.** The release and the data already say so, and the
scenario's comment is stale and load-bearing. **Then there is an Ark to move and your test is a
rule that exists** - `spec/orbit.md` says an orbit boundary includes one *between two orbits* and
that a unit crossing orbit boundaries may cross any of them, so an Ark travels in orbit above the
surface.

**`K2` - launching produces nothing**, as `P-342` said. Then the release row and the data line
both go, **and nothing in the first release can move an Ark at all** - it exists only long enough
to be consumed by deploying. The thing that moves across the planet is the pioneer.

## What this lane would say

**`K1`, and the reason is your own win condition.** `spec/control.md`: *a player wins by deploying
an Ark to one territory and launching an Ark from a different one.* **Under `K2` the Ark that is
launched goes nowhere and is never seen again**, which makes launching a payment rather than an
event. Under `K1` the winning Ark is in orbit at the end, which is a thing you can look at.

**Either way the scenario's comment is wrong and says so as fact**, in a file the code lane owns.
Filed to them as `S-165`.
