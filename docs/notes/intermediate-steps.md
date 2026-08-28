# When an Intermediate Step Earns Its Place

**Derived.** Written by Claude from conversation, 2026-08-26. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Sean asked how to decide between *build the thing directly* and *build a prerequisite, then the
thing* - and how to stay lean in the first release without painting the design into a corner.

## Half of it is already decided

[`spec/invariants.md`](../../spec/invariants.md) says:

> No action has an intermediate step that is always taken. Where one would, the action is defined
> to reach the outcome directly.

That disposes of every intermediate the player would never decline. If building a Yard is simply
what you do before building an Ark, and no sane player would do otherwise, the Yard is not a step -
it is ceremony with a cost attached. **So the question is never "is there an intermediate?" It is
"does the intermediate carry a decision?"**

## The test

An intermediate step earns its place when a player can **get it wrong**. Four ways that can
happen, and a step needs at least one:

| Test            | The question                                             | Wrong looks like                              |
| --------------- | -------------------------------------------------------- | --------------------------------------------- |
| **Timing**      | Is there a bad moment to build it?                       | Built too early, starving something urgent    |
| **Placement**   | Is there a bad territory to build it in?                 | Built where it cannot be defended or supplied |
| **Exclusivity** | Does building it mean not building something else?       | Spent the metal on the wrong thing            |
| **Branching**   | Does it open one path among several you cannot all have? | Committed to a line that does not pay off     |

If none apply, delete the step. If one applies, it is a real decision and worth the rule.

**When you delete a step, fold its cost into the thing it gated.** Removing an intermediate should
remove the clicking, not the price. A game where the Ark costs `Yard + metal` and one where it
costs `metal + metal` differ in how many decisions the player makes, not in how hard the goal is.
Dropping the cost as well is a separate decision and should be made separately.

## The mechanism that keeps the door open

The specification already generalises this, which is why depth can be added later without a new
rule. [`spec/logistics.md`](../../spec/logistics.md) says:

> A cost may be made of anything the player controls: resources, citizens, units or structures.

So *build a research lab, then the thing* is not a different mechanic from *build the thing*. It
is the same mechanic with a different cost list. **A tech tree is data, not machinery.**

There are two shapes and they are worth keeping distinct, because they behave differently:

| Shape        | Meaning                              | What happens to it                      |
| ------------ | ------------------------------------ | --------------------------------------- |
| **Cost**     | Spent to produce the thing           | Consumed - gone afterwards              |
| **Required** | Must be present to produce the thing | Persists - one gate, then free for ever |

Sean began drafting exactly this distinction and stopped mid-line.
[`releases/first-release.md`](../../releases/first-release.md) reads:

```
### Create Pioneer
- cost to produce: 1 metal, 1 citizen
- required to produce
```

The second line has no content. **Every producible thing having both a `cost` list and a
`required` list is the whole anti-corner mechanism.** Leave `required` empty in the first release
and everything is buildable directly; fill it in later and you have prerequisites, research labs
or an arbitrarily deep tech tree - with no rule anywhere changing, only entries in a list.

`Required` also composes cleanly with
[the infrastructure invariant](../../spec/invariants.md): a structure costs nothing to keep, so a
requirement is a one-time gate rather than an ongoing drag.

## Applying the test to what exists now

**The Yard passes**, on timing and exclusivity. Metal spent on a Yard is metal not spent on
Pioneers, and that is the central decision of the whole game: *when do I stop expanding and start
leaving?* Too early and the planet is under-exploited; too late and the turns are wasted. It fails
placement - territories are identical and self-contained in the first release - and branching,
since there is nothing else to build. One test is enough.

**The garrison requirement is currently vacuous, and this is the clearest example of the
invariant biting.** `spec/structures.md` says a garrison *"is what allows units to be produced."*
But founding a territory produces a garrison, and a territory that loses its force is lost
entirely - so **every territory a player controls has a garrison, always.** The requirement can
never fail to be met, which makes it a step always taken.

It is not wrong, it is simply not yet doing work. It would begin to if garrisons could be
destroyed without the territory falling, or if a territory could be held without one. Until then
the honest reading is that a Pioneer can be built in any territory you control.

## What this suggests for the first release

- Everything buildable directly, `required` lists empty - except the Yard, which is the one
  intermediate that carries a decision
- Keep the `cost` / `required` split in the data even while `required` is empty, because that is
  what makes later depth a content change rather than a redesign
- Add a prerequisite only when one of the four tests answers yes, and say which one in the
  proposal that adds it
