# No Free Surplus

**Derived.** Written by Claude from conversation, 2026-08-31. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Prevent, detect, bound](prevent-detect-bound.md) · [Everything is matter](everything-is-matter.md)

Sean set out his intent for rules and asked whether it is internally consistent. **Almost. One line
forbids something the game already needs**, and the weaker version he offered as a fallback turns out
to be what the game already does.

## The intent, as stated

- There is a **ready** and an **exhausted** state
- Things come into existence via recipes, and **arrive ready**
- Acting **exhausts** the actor
- So a thing made this turn can be used this turn
- **No perpetual motion**, enforced by design rather than mechanic: the chain of created things must
  not form a cycle
- Or, weaker: cycles are allowed if some **single-directional state change** prevents infinite regress
- The principle underneath: *you can't somehow end up with all of the same resources you started with
  plus more*

**The first four are already the specification.** `spec/turn.md`: *anything that can be used is ready
or exhausted; using it exhausts it, and a thing created during a turn begins ready and may be used at
once.*

## The inconsistency: no cycles would forbid the economy

**The game's core loop is a cycle.** Labor works an extractor and yields food; food feeds citizens;
citizens provide labor. `labor -> food -> citizen -> labor`, and it closes.

So *the chain of created things must not form a cycle* would rule out the thing the game is made of.
**The stronger version is not the safe choice - it is the impossible one.**

**And a correction this note owes.** An earlier argument in
[control without tedium](control-without-tedium.md) held that a turn terminates because every action
exhausts something, so the set of things that can act only shrinks. **That is wrong once a recipe
makes a ready thing** - the set can grow, and shrinking was doing the work in that proof.

## What actually stops it: every cycle passes through an exhaustion

The `labor -> food -> citizen -> labor` cycle does not close **within a turn**. Getting labor out of a
citizen exhausts that citizen, and nothing inside the turn makes it ready again. **The cycle closes
only across the turn boundary**, where readiness is restored deliberately and once.

**So Sean's weaker version is not a fallback. It is a description of the game as built.** Ready to
exhausted is the single direction, it never reverses inside a turn, and the turn boundary is the one
controlled reset. That is why a turn ends rather than running for ever - not because the usable set
shrinks, but because **readiness is spent and cannot be re-made.**

**Which gives the rule to write, if it is ever written:**

> Every cycle among recipes must spend readiness somewhere along it.

A cycle that spends readiness runs at most as many times per turn as there is readiness to spend. A
cycle that spends none runs for ever.

## The other half, which is a different guarantee

Termination is not the same as **no free surplus**, and Sean's last sentence is about the second.

A recipe reading *10 metal -> 11 metal* spends no readiness, terminates fine on its inputs, and is a
money press. Nothing above forbids it. **What forbids it is conservation** - `P-126` makes metal
conserved, so what a thing was made from is exactly what taking it apart returns. **Build-and-strip
cannot profit, because there is nothing to profit from.**

So the two guarantees are separate and both are needed:

| Guarantee                    | What enforces it                                                    |
| ---------------------------- | ------------------------------------------------------------------- |
| A turn ends                  | every cycle spends readiness, which only the turn boundary restores |
| Nothing is made from nothing | conserved resources return exactly what went in                     |

**And a third thing is doing quiet work.** Extractors do create metal that was not there - the economy
grows, and that is intended. It is not free because it is **metered**: `spec/economy.md` says a
planet's resources never run out and what is finite is the rate. **A node is the outside of the
system**, and every cycle that seems to gain is a cycle drawing on one.

## Where this leaves the design rule

**Sean's instinct to forbid it by design rather than by mechanic is right**, and it is
[tier one](prevent-detect-bound.md): a perpetual motion machine that cannot be written beats one that
is detected. What changes is the test applied when a recipe is added:

- **Not** *does this create a cycle* - the economy is one.
- **But** *does this cycle spend readiness*, and *does it return more of something than it consumed*.

Both are checkable when the recipe is written, which is what `P-130` makes possible: recipes are data,
so the graph over them can be walked before anything runs.
