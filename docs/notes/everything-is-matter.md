# Everything Is Matter, and Rust Is the Engine

**Derived.** Written by Claude from conversation, 2026-08-30. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Spec backlog](spec-backlog.md) · [Control without tedium](control-without-tedium.md)

Sean, thinking about how units should be defined: *perhaps a pioneer is actually just a personnel
transport made of metal that stores fuel so it can move. However it could be stripped down for parts
in order to be consumed in transformation formulas.* And separately: *I am leaning towards a more
data driven game where the units and transformations are simply data inputs to rust, and rust is
providing a statically typed engine to run and validate the data.*

Two ideas, and the first is larger than it sounds.

## Conservation makes a unit a container, and that collapses three rules into one

Today a Pioneer **costs** 8 metal: the metal is spent and gone. Under conservation a Pioneer **is** 8
metal, held in a particular arrangement, and stripping it gives the metal back.

**Follow that and a structure is the same kind of thing.** A Yard is not a thing that cost 15 metal,
it is 15 metal arranged as a Yard. So a territory's metal is not its loose pile - it is the loose
pile **plus every structure plus every unit standing there**.

**Which means Sean's three separate limits are one limit.** He has asked for a bound on stored
resources *to make sure the game is finite*, and noticed a bound on units is needed *for the same
reason*, and the backlog wants storage structures with capacity. If everything is matter, all three
are:

> **A territory holds only so much matter, and everything in it is matter.**

One rule, and it closes the finiteness question completely rather than per-category: the total in a
territory is bounded, the number of territories is bounded, so the reachable state space is bounded.
No separate unit cap, no separate store cap, no argument about whether a unit parked on a full
territory counts.

## The asymmetry that falls out, and it is the right one

Conservation cannot apply to everything, and which resources it applies to is not arbitrary:

| Resource   | Persists between turns | Recoverable from what it built |
| ---------- | ---------------------- | ------------------------------ |
| **Metal**  | yes                    | **yes** - it is stuff          |
| **Energy** | yes                    | no - it is spent moving        |
| **Food**   | no                     | no - it is eaten               |

**Metal is the only conserved one**, because it is the only one that remains a thing after being
used. That matches `P-126`'s split for a different reason - that one is about *demand being lumpy or
continuous*, this one is about *whether the resource survives its use* - and the two agree, which is
worth noticing before anybody writes a rule that makes them disagree.

**And the specification already permits this without requiring it.** `spec/invariants.md`:
*conservation is not required; for example, fiat money is created and destroyed by a government, and
that is a model.* What is forbidden is a quantity changing with nothing in the world doing it.
Conservation of metal is a stronger promise than the invariant demands, and nothing has to change to
allow it.

## The question that decides whether it adds depth or drains it

**Is stripping lossless?** If a Pioneer returns all 8 metal, then building one is a decision with no
material cost, and a decision you can fully undo is not much of a decision. If it returns 4, then
conservation is not conservation and the appeal of the idea - one clean rule - is gone.

**The way out is that matter is conserved and time is not.** Building took turns and labour, and
neither comes back. So a lossless strip is materially neutral and remains expensive, because what it
cost was **the turns you were not doing something else**. That preserves the weight of the decision
without a fudge factor, and it is the honest version of what a 50% refund is usually trying to
approximate.

**It also gives stranded units somewhere to go.** A Pioneer that cannot reach anywhere useful is
currently 8 metal thrown away; under conservation it is 8 metal standing in the wrong place, which is
a problem with a solution.

## Rust as the engine, data as the game

Sean's leaning answers the question [the entity-model discussion](game-4x-predecessor.md) left open -
which side is the source of truth. **The data is, and Rust validates it.**

That dissolves the objection that killed the attribute-bag model on its own. The predecessor's
`Thing` was stringly-typed and `requireInt()` threw at runtime; a validating loader checks the same
things **once, at load, against a declared schema**, and everything downstream is typed. You get
*adding a submarine is data* without giving up the compile-time guarantees the model needs.

**There is precedent one level down.** `commands/*.4x` already specifies a *scenario* in data - `add
node`, `set force`, `set biome` - and the console validates each line and reports where it went
wrong. What Sean is describing is the same machinery one level up: data that defines **kinds**
rather than instances.

### The one thing this breaks, and it needs deciding rather than discovering

`spec/invariants.md` says **every change to game state is representable and executable as a console
command**, and `P-115` says a rule's actions are recorded exactly as if typed. Both are about
*state*.

**A unit type is not state, it is rules.** So defining one in data introduces a second kind of thing
that changes, which the one-function invariant does not cover - unless the answer is that **types are
state too**, and defining a Pioneer is a transition like any other.

That second answer is stranger and better. It keeps one function over one state, it makes the
history a complete account of the game *including what the game was*, and it is what would make the
thing moddable. What it costs is that the definitions become part of every saved game, and two games
could disagree about what a Pioneer is - which is either a bug or the whole point, depending on
whether the game is one game or a family of them.

**Nothing needs deciding until transformations are written.** It is recorded here because it will be
easier to answer before there is code than after.
