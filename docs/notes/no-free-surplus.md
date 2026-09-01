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

## The refinement: matter is conserved, order is not

Sean: *the planet is an open system, more metal and energy exists than can be practically extracted,
and food gets its energy from the sun. Metal can come out of the ground, be made into a vehicle, and
that vehicle can be destroyed. The metal becomes unavailable but the reason matters. Metal in a
storage bin is organized so it is easy to access. Metal from a destroyed vehicle must be organized to
be useful.*

**This is the first law and the second law, and it settles a question this note left open twice.**

### Conservation was the wrong half on its own

`P-126` says metal is conserved: *what it was made into can be taken apart to get it back.* Twice
this lane asked whether stripping is therefore **lossless**, and answered *matter is conserved and
time is not* - true, and philosophy rather than mechanism.

**Sean's version is mechanical.** Taking a vehicle apart returns all of the metal and returns it
**disorganised**. Nothing is destroyed and nothing is free: what was spent is the *arrangement*, and
arrangement is what makes metal usable. **A destroyed vehicle is a pile, and a pile is not a
stockpile.**

So build-and-strip has a price, and it is not a fudge factor: **the price is re-organising the
scrap**, which costs labour like anything else.

### A second arrow, alongside ready-to-exhausted

The note above found one single-directional change: **ready becomes exhausted and only the turn
boundary reverses it.** This adds a second:

> **Organised becomes disorganised for free. The reverse always costs.**

**That is what forbids perpetual motion, and it is stronger than forbidding cycles.** Any cycle that
returns matter returns it degraded, so going round again costs the work of re-refining. **The cycle is
allowed and it simply does not pay** - which is a better rule than one that has to enumerate what may
not be built.

### And it is already half-written

Sean said, before he had the frame: *if you have a bunch of metal lying around it gets lost, but if
you have storage containers to keep it organized you have more usable metal in the same area.*
**That is this rule, and `P-129` already carries it** - *capacity is organisation, not room*, and a
container occupies capacity of one kind and provides it of another.

So loose metal being lost is not a separate mechanic to be added. **It is disorder winning where
nothing is spent to hold it back.**

### The open system is the node

*More metal and energy exists than can be practically extracted, and food gets its energy from the
sun.* `spec/economy.md` already says it from the other side: *a planet's resources never run out;
what is finite is the rate at which they can be extracted.*

**So a node is where the system is open.** Every cycle that appears to gain is drawing on one, and the
rate is what stops it running away. Nothing else in the game is a source.

### What it would cost to adopt

- **A state for matter, not only a quantity.** `{metal}` becomes `{metal, usable}` and
  `{metal, scrap}` - a trait, which `P-134` already makes free.
- **A recipe that refines.** Scrap plus labour yields usable metal, and it is an ordinary row.
- **`P-126` would want one word.** *Taken apart to get it back* does not say **in what state**, and
  under this it comes back as scrap. It is not wrong today, because scrap does not exist; it becomes
  wrong the day it does.

### And it makes destruction interesting rather than only costly

**A destroyed vehicle leaves something.** Whoever holds the ground afterwards has scrap worth
recovering, so a battlefield is worth taking for a reason nobody had to invent. **That falls out of
the model rather than being a mechanic bolted on**, which is the same test `docs/notes/spec-backlog.md`
applied to supply routes getting cheaper when you clear them.
