# Bootstrapping the game from a small kernel

**Research, 2026-09-09. Addressed to nobody.** Not a finding and not work for anyone. Sean said he
intends this *once I have a well defined, working game*, and explicitly not now - so this records
what today's decisions already buy it, and what would make it hard, while the reasons are fresh.

[Research](README.md) · [The recipe report](formulas.html) · [Outbox](outbox.md)

Sean, 2026-09-09: *once I have a well defined, working game, I am going to want to bootstrap the
whole thing from a very small set of rules, and incrementally build out the whole game from there.*

## The nearest neighbour, and why its kernel is five times bigger

Forth is the closest thing to what he describes: a small kernel of primitives, and everything above
it defined in Forth itself with colon definitions. [seedForth's kernel is about thirty
primitives](https://github.com/uho/preForth) plus a compiler and an interpreter.

**This game has six**, and the difference is not that it is more elegant. Forth's thirty include
parsing its own input, searching its own dictionary and compiling its own words - **its metasystem
is inside its kernel**. This game's metasystem is outside: a console parses, a renderer displays, a
checker analyses, and none of that is written in recipes. So six primitives is the size of the
*domain* kernel, and a bootstrap would add whatever is needed to define things from within, which
is not zero.

## What today already buys it

- **The primitive set is minimal by a test rather than by taste.** `X-11` fixed the criterion -
  Felleisen's eliminability, and Nebel's compilation schemes - and every primitive has since been
  argued against it. `change` absorbed two; `require` survived because a read arc is not a change;
  `each` was added because `xE` cannot be expanded when it is written. **A kernel chosen this way
  is the thing a bootstrap needs to be able to trust.**
- **`call` is the defining word.** A recipe defined in terms of other recipes is the whole mechanism
  of building out, and it already exists, already has one real call site, and already has its
  decidability caveat: acyclic is free, recursive is not.
- **Dependency ordering already has a form.** A recipe that needs a kind to exist can say
  `require 1 {kind name:ark}`. Depth is free; what is forbidden is a cycle.
- **The incremental rule is exactly what a build needs.** Checks 1 and 6 read one recipe on its own,
  so a recipe checked once stays checked; check 2 is a property of the whole matrix and is re-run.
  **Building out incrementally costs one whole-net check per addition and nothing else.**

## Three things that would make it hard

**The kernel has to be complete before the tower is tall.** Discovering at recipe two hundred that a
seventh primitive is needed means everything built on the kernel may have to be revisited. That
makes `X-11`'s question - *is this the least expressive complete set?* - load-bearing rather than
aesthetic. It was worth answering carefully today for a reason that only appears later.

**Diagnostics degrade with depth, and this has already been seen at depth one.** The design-time
discussion hit it: a category can say *that command belongs to the design phase*, and a guard on
`game.phase` can only say *no game with phase design*. That is the same failure at the shallowest
possible depth. In a bootstrapped system an error surfaces at the primitive that failed rather than
at the definition a person wrote, and **retrofitting good messages onto a deep tower is expensive
where designing for them is cheap.** Worth deciding what a failure carries with it *before* there
are two hundred recipes rather than after.

**Nothing can be defined from within yet, and check 6 says so precisely.** Neither `kind` nor
`recipe` is one of the sixteen kinds, so a definition has nowhere in the state to be. Running
`define ark` through the checks refuses it in two lines - *nothing says a game may hold a kind*,
*nothing says a game may hold a recipe*.

## The shortest path, when the time comes

**Two kinds and two declarations.** `kind` and `recipe` become kinds; the game declares it may hold
each, with no limit. Check 6 then passes on a definition recipe, and the rest follows from what is
already there: `change` creates the definition carrying its traits in one transition, which is
`spec/invariants.md`'s *a definition arrives in one transition*; `require` orders the dependencies;
`call` builds on what is defined.

**And one condition, which is weaker than a phase.** The set of recipes that could ever exist has to
be finite and knowable in advance. Definitions written by a person always satisfy it. Definitions
made by a recipe satisfy it if they come from a fixed catalogue - a reconfigurable net, where
boundedness stays decidable - and do not if a recipe composes an arbitrary new one out of the state,
which is a net rewriting system and Turing powerful.

## What is deliberately not being done

No item is filed and nothing is built. This is a note that will be found when it is wanted, which is
what unaddressed research is for. **If any of it turns out to bear on a decision before then it
becomes an item, and until then it costs nobody any attention.**
