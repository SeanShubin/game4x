# Simple, finite, and still decidable: where the cliff edges are

2026-09-08. For Sean, who is replacing the recipe structure and wants **every game interaction
expressible in something simple and finite**, with unlimited complexity coming from combination and
from nesting - one recipe calling another. Target expressiveness: Master of Orion (1993) as the gold
standard, plus Distant Worlds 2, Factorio, and Goldberg polyhedra.

**He is prototyping, and this is written for that.** Nothing here proposes specification text. What
it does is put the three cliff edges on the map before he walks toward one, because **all three are
cheap to avoid now and expensive to retreat from later.**

**Every citation was checked against a source in the session that wrote this.** Where a claim is this
lane's inference, it says so.

## The one-paragraph version

What he is describing has two established formalisms, not one. **A recipe network is a Petri net**;
**a recipe that calls another recipe is a Hierarchical Task Network**. Each has a precisely located
point where analysis stops being possible, and his current structure already stands on one of them.
The good news is sharper than the bad: **the thing that costs everything is not `limit 0` as such,
but `limit 0` on an unbounded quantity** - and the distinction is exactly the fact-versus-quantity
split that came out of `C-74`.

## 1. The recipe network is a Petri net, and `limit 0` is the cliff

`require`, `consume` and `produce` over counted things are, formally, a **Petri net**: places hold
tokens, transitions consume from some places and produce into others. This is not an analogy.
Factorio's recipe graph is the same object, which is why throughput and ratio analysis works there.

**What that buys, and it is a lot.** For a plain Petri net, reachability is **decidable**, and the
standard structural analyses answer questions a 4X designer actually asks: **P-invariants** name
quantities that are conserved no matter what is played, and **T-invariants** name cycles of recipes
that return the world to where it started - which is to say, **a self-sustaining loop, or an
unintended infinite resource generator, is something you can compute rather than playtest for.**

**What breaks it.** An **inhibitor arc** - a transition that may fire only when a place is empty - is
a **zero test**, and Petri nets with inhibitor arcs are **Turing-complete**. Two inhibitor arcs
suffice to model a two-counter machine, at which point every question above becomes undecidable.
Reachability survives **one** inhibitor arc and dies at two.

**`limit 0` is an inhibitor arc.** The current garrison rule - `limit 0` and `produce 1` - is a zero
test in exactly the technical sense. So the present structure is already on the far side of the
line, and nothing in it says so.

### The distinction that saves it, and it is not the obvious one

The obvious move is to make the zero test a guard on an effect rather than a precondition, which is
what his `create-if-missing` sketch does. **That is the right move for a different reason and it
does not, by itself, buy decidability** - a zero-guarded effect can record the result of the test in
another place, and the branch is back. This lane's first draft said otherwise and was wrong.

**The real line is boundedness.** A Petri net cannot test an unbounded place for zero; **a bounded
place can be zero-tested safely**, by the standard complementary-place construction - a companion
place holding `capacity - count`, so *is it zero* becomes *is the companion full*, which is an
ordinary arc. Capacity constraints were introduced to make modelling easier precisely because, on
bounded places, they add no power.

So the rule is mechanical and a check could enforce it:

| Zero test on                                   | Verdict                                                                                |
| ---------------------------------------------- | -------------------------------------------------------------------------------------- |
| A **bounded** place - a garrison, capacity one | **Safe.** Expressible without an inhibitor arc; the net stays decidable and analysable |
| An **unbounded** place - food, metal, citizens | **The cliff.** A genuine inhibitor arc, and two of them end all analysis               |

**This is the same split `C-74` produced from the other side.** A garrison is a *fact* - at most one,
present or absent - and citizens are a *quantity*. A fact is a capacity-one place, and a capacity-one
place may be zero-tested for free. **So the fix for `limit 0` is not to remove it but to declare what
kind of thing it tests**, and the structure he is designing is the place to make that declarable
rather than incidental.

## 2. Nesting is HTN, and recursion is the second cliff

**One recipe calling another has a name**: hierarchical task network planning, where a method
decomposes a compound task into subtasks. It is the right shape for what he wants, and the
literature is unambiguous that it buys real power: **HTN planning is strictly more expressive than
STRIPS-style planning.**

**And it is undecidable in general**, precisely because decomposition can recurse - infinite
zero-cost decomposition paths are, in the literature's own word, *rampant* even in severely
restricted subclasses.

**The lever is syntactic, which is the useful part.** Many decidable subclasses of HTN are
**syntactically identifiable** - you can tell by looking at the rules, not by running them. The
cheapest such restriction is **acyclic decomposition**: no recipe may transitively call itself.
That is a graph check over the recipe set, it runs in milliseconds, it can be a gate, and it
preserves nearly all of the combinatorial explosion he actually wants - **the explosion comes from
the branching factor of composition, not from recursion depth.**

**This lane's inference, flagged as one**: unbounded recursion is what he would be giving up, and it
is worth asking whether any 4X interaction needs it. Master of Orion has deep composition and, as far
as this lane can see, no rule that invokes itself without bound.

## 3. The precedent for the whole ambition already exists

**Someone has already tried "one simple finite language for any game", and it worked.** The **Game
Description Language**, Genesereth's General Game Playing project at Stanford from 2005, is a variant
of **Datalog** in which a game is declared with a handful of predicates - roles, the initial state,
`legal`, `next`, `terminal`, `goal`. It has enough power to describe **any turn-based, finite,
deterministic, n-player game with full information**, including simultaneous moves.

That is the existence proof for his instinct, and the fact that GDL is Datalog connects it to
`C-74`'s answer: the same formalism that separates a rule table from a selection table is the one
that turned out to be enough to describe games in general.

**What matters more is where GDL's stated limits fall, because his three references fall outside all
of them.** GDL is limited to perfect information and a discrete, finite state space. Extensions
exist and are named: **GDL-II** for randomness and imperfect information, **GDL-III** for
introspection, **rtGDL** to remove the turn-based restriction.

| What he named         | What it needs beyond base GDL                                                                                       |
| --------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Master of Orion, 1993 | Hidden information - unexplored systems, enemy strength - and randomness in combat and events. **GDL-II territory** |
| Distant Worlds 2      | Continuous rather than turn-based time. **rtGDL territory**                                                         |
| Factorio              | Continuous throughput over a recipe network. **The Petri-net half, above**                                          |

**So the target is not base GDL and never was.** This is worth knowing on day one of a prototype
rather than on day ninety: **imperfect information and randomness are not features to add to a
finite deterministic core, they change what the core is.** A language designed as deterministic and
fully observable does not get them by extension without a redesign - which is precisely why the
GDL people gave the extensions different names rather than a version number.

## What this suggests for the prototype, offered as constraints rather than a design

Three properties, each mechanically checkable, each cheap now:

1. **Declare whether a place is bounded**, and permit a zero test only where it is. That keeps the
   whole Petri-net analysis toolkit and costs one column.
2. **Require the call graph between recipes to be acyclic**, and check it. That keeps HTN's
   expressive gain and steps back from undecidability.
3. **Decide where hidden information and randomness live before the core is fixed**, because the
   three games he named all need them and no extension adds them cleanly afterwards.

**And one thing that is his and not the literature's.** The reason any of this matters practically is
the interface: `X-8` established that a selection-only interface shows the **applicable ground
actions**, so *what can I do now* is a computation the game must perform every frame.
**Undecidability is not an abstract loss here - it is the menu failing to be computable.** That is
the strongest argument for the three constraints above, and it is an argument from what a player
sees rather than from theory.

## What this lens is not saying

It is not saying the recipe structure should be a Petri net or an HTN. Those are the names for what
he has already described; naming them is what makes the cliffs visible. **What is on the far side of
each cliff is a decision about what he is willing to give up**, and this lane has deliberately not
made any of them.

**Goldberg polyhedra are not treated here** - `prototypes/goldberg-view` already recorded an answer
to that question, and it was that appearance was never the constraint and diminishing strategic depth
was. That answer stands and this report does not revisit it.

## Sources

- Inhibitor arcs, zero tests and Turing-completeness; reachability surviving one inhibitor arc -
  [Reachability in Petri Nets with Inhibitor Arcs, Reinhardt](https://www.sciencedirect.com/science/article/pii/S1571066108005057/pdf),
  [Analysis issues in Petri nets with inhibitor arcs](https://www.sciencedirect.com/science/article/pii/S030439750100127X),
  [Small Universal Petri Nets with Inhibitor Arcs](https://arxiv.org/pdf/1312.4414)
- Bounded places, complementary-place construction, and why capacity adds no power -
  [Petri Net Theory and the Modeling of Systems, ch. 7](http://jklp.org/profession/books/pn/7.html),
  [ETH, Discrete Event Systems](https://disco.ethz.ch/courses/hs15/des/lectures/des_chapter9.pdf)
- Reachability decidable but Ackermann-complete, so *decidable* is not *cheap* -
  [Reachability in Vector Addition Systems is Ackermann-complete](https://arxiv.org/pdf/2104.13866),
  [Improved Ackermannian Lower Bound](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.STACS.2022.46)
- HTN: strictly more expressive than STRIPS, undecidable in general, decidable subclasses
  syntactically identifiable - [Erol, Hendler and Nau, *HTN Planning: Complexity and
  Expressivity*, AAAI 1994](https://www.cs.umd.edu/~nau/papers/erol1994htn.pdf),
  [Expressivity of STRIPS-Like and HTN-Like Planning](https://link.springer.com/chapter/10.1007/978-3-540-72830-6_13)
- GDL, its coverage, and the named extensions -
  [Game Description Language](https://en.wikipedia.org/wiki/Game_Description_Language),
  [Thielscher, *The GGP Description Language Is Universal*, IJCAI 2011](https://www.ijcai.org/Proceedings/11/Papers/189.pdf)
