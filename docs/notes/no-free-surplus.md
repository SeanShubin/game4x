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

- ~~**A state for matter, not only a quantity.**~~ **Withdrawn** - see the section below. Sean's
  second version needs no such state, because metal from a wreck is ordinary metal facing the
  ordinary end-of-turn question.
- **A recipe that refines.** Scrap plus labour yields usable metal, and it is an ordinary row.
- **`P-126` would want one word.** *Taken apart to get it back* does not say **in what state**, and
  under this it comes back as scrap. It is not wrong today, because scrap does not exist; it becomes
  wrong the day it does.

### And it makes destruction interesting rather than only costly

**A destroyed vehicle leaves something.** Whoever holds the ground afterwards has scrap worth
recovering, so a battlefield is worth taking for a reason nobody had to invent. **That falls out of
the model rather than being a mechanic bolted on**, which is the same test `docs/notes/spec-backlog.md`
applied to supply routes getting cheaper when you clear them.

## Running it down: salvage, and the state that turns out not to be needed

Sean worked the model through to mechanics and changed his mind halfway, in the right direction.

**First**: a destroyed vehicle renders no usable metal - it is there, disorganised, and gameplay-wise
gone. Salvage is a deliberate operation, so salvaged metal *is* usable that turn but unusable if not
stored.

**Then**: *perhaps the metal from a destroyed vehicle should stay around for the turn, and metal does
not expire until end turn happens and there is no one to expend the labor to put it in a bin, or no
bin.*

**The second is better and it deletes a concept.** Under the first, scrap is a second state of matter
and every recipe has to know which it is taking. Under the second there is **only metal**, and the
rule is uniform:

> Metal is usable when it arrives, from wherever it came. At the end of the turn it is lost unless
> somebody spent the labor to store it, and there is somewhere to put it.

**So the section above was wrong to say a trait is needed.** `{metal, usable}` against `{metal, scrap}`
was the cost of the first version. The second needs no trait at all - **disorganisation is not a state
of the metal, it is what happens to metal nobody organised**, which is what Sean said in the first
place about metal lying around.

**And salvage becomes an ordinary ability rather than a special case.** A unit with *salvage* turns a
wreck into metal in the usual way; the metal then faces the same end-of-turn question as any other.

## What it collides with, and both need deciding rather than discovering

**Storing costs labor, and `P-126` does not say so.** That proposal landed as *metal and energy remain
where they are*, bounded by capacity - **unconditionally**. This adds a price: labor, every turn, for
everything kept. It is a real addition and not a clarification.

**It also threatens the arithmetic that made the release winnable.** `P-126` was promoted on a
measurement: territory 1 raises 12 metal a turn against a Yard's 15, so two turns buys one. **If
keeping 12 metal costs labor, territory 1 has nine hands and fewer of them are producing** - and how
many fewer depends entirely on a rate nobody has chosen. **At one labor per unit stored the release is
unwinnable again**; at one labor per bin it is barely affected. **The rate is the whole question**, and
the measurement should be re-run against whatever it turns out to be.

## The garrison, where conservation bites harder than expected

Sean: *a garrison can be created from metal, either from a military unit and a metal transport truck,
or a pioneer that when disassembled yields one metal. Either way it takes a citizen to provide labor.*

**As an account of founding this is fine and rather good** - founding is already the recipe that makes
a garrison, and *the pioneer is taken apart and a citizen does the work* is a theme for a rule that
exists rather than a new rule.

**As a second route it contradicts the specification.** `spec/control.md`: *a garrison is not built. A
territory gains one by being founded, and **gains one no other way**.* A military unit plus a truck
yielding a garrison is another way. The units do not exist, so nothing is wrong today - **but that
line has to move before they do**, and it is the kind of thing that gets discovered by an
implementer.

**And the arithmetic exposes something conservation demands that nobody has provided.** A Pioneer is
8 metal. If metal is conserved, founding must account for all 8 - and founding produces a garrison, a
citizen and a food extractor, **none of which has ever had a metal content**, because none of them is
built from metal. Sean's *yields one metal* implies the garrison is worth 7, but nothing says so.

> **Conservation forces every kind to have a metal content, including kinds that are never built.**

**Too strong - corrected below.** Conservation requires the books to balance, not that every kind
holds metal.

That is a bill the release has not been shown. It is not an argument against conservation - it is the
work conservation entails, and it is better seen now than when a recipe fails to balance.

## Storage at the node, which rescues the arithmetic

Sean: *storage for metal coming out of a node is thematically cheaper, so perhaps the same labor can
pay for both extraction and storage... perhaps storage is a thing we can add to nodes to collect with
no additional labor cost, and external storage would require labor to put the extractor output
there.*

**This answers the threat the section above raised, and answers it without a number.** The worry was
that charging labor to keep metal would undo `P-126`'s winnability measurement - territory 1 has nine
hands and raises twelve metal, and if keeping the twelve costs hands the sums change.

**Under storage at the node it does not.** Working a node already spends the labor; the output lands
where it was made and stays. **Territory 1's three metal nodes cost three labor and yield twelve, and
nothing further is owed** - the measurement stands as promoted.

**And labor is charged exactly where the theme says it should be**: picking scattered metal off the
ground and loading it into a truck is work; catching what an extractor produces in a bin beside the
extractor is not. **The free case is the one where nothing had to be gathered.**

**What it costs to state.** A node gains a capacity, and there is a second, general storage that costs
labor to fill. Two numbers rather than one, and they are release tuning rather than rules.

## Purging *founded* and *founding*

Sean: *this is more evidence I need to purge the "founded" and "founding" concepts.*

**Measured before agreeing: seven lines in five files.** `spec/console.md` (a territory is *taken and
founded*), `spec/control.md` (the garrison line), `spec/population.md` and `spec/unit-types.md` (three
lines about a *founding unit*), and two in the release. `spec/narrative.md` uses *founding population*
in the fiction, which is prose and not a mechanic.

**The purge is smaller than it looks, because the pieces already have homes:**

| What it was doing               | Where it goes                                          |
| ------------------------------- | ------------------------------------------------------ |
| `founded` as a stored flag      | derived: force present is at least the force of nature |
| *founding* as a special act     | a recipe like any other                                |
| *a founding unit* as a category | a unit that appears in that recipe's inputs            |
| *gains one no other way*        | already covered - see below                            |

**The line that looked load-bearing is not.** `spec/control.md` says a garrison is *not built* and a
territory *gains one no other way*, which was the thing a second route would contradict. **But the
constraint that matters is stated separately in the same file**: *a territory has at most one
garrison, because it represents the organisation of the whole territory.* **That is what stops
garrison-spam**, and it survives the purge untouched. The *no other way* clause was doing nothing the
*at most one* clause was not already doing.

**So the purge removes a special case rather than a rule**, which is the good kind, and it follows
from `P-130` and `P-134` rather than being a new idea: if everything is a recipe, nothing is *not
built*.

## Correction: conservation does not force a metal content on everything

The section above claimed **conservation forces every kind to have a metal content, including kinds
that are never built**. Sean: *I don't think it follows that everything must have metal content.*
**He is right and the claim was too strong.**

**Conservation requires the books to balance, not that every kind holds metal.** A citizen is not made
of metal and can hold none; what conservation demands is that the metal a recipe takes in is
accounted for by what it puts out - and *output* includes loose metal.

**His own figures balance.** A Pioneer is 8 metal, and *a pioneer that when disassembled yields one
metal* gives: garrison 7, extractor 0, citizen 0, and **1 metal loose** - which then faces the same
end-of-turn question as any other loose metal. **Eight in, eight out**, and the one left over is
exactly the salvage he described.

**So the bill is real but smaller.** Kinds that metal goes *into* need a content; kinds it does not
need none. And the balance is checkable when a recipe is written, which is what `P-130` makes
possible.
