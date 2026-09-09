# One fact, a console that types and an interface that selects

2026-09-08. Answers [`C-74`](../../crates/outbox.md), which Sean asked to reach this lens while he
prototypes. **He does not want the present structure presumed, so nothing here proposes any.** What
follows is what the literature already settles, what it warns about, and the names - so that the
choice is made with them rather than rediscovered.

**Every citation was checked against a source in the session that wrote this**, not recalled. Where a
claim is this lane's own inference from the sources, it says so.

## The short answer

**All three of `C-74`'s puzzles are one question, and it has a name: grounding.** A recipe is an
operator with parameters; the rows a person selects from are that operator *instantiated against the
current state*, one row per binding that satisfies the preconditions. Once that is the relationship,
each puzzle resolves in the same move:

- **Whether a condition is a precondition or a guard on an effect** decides whether a row **appears
  at all**. That is question one.
- **Which parameters exist** decides **what the person picks**, because every parameter is a choice
  and a derived term is not one. That is question two.
- **The rows mentioning a selected thing** are the verbs available on it. That is question three.

The console and the interface are then **two renderings of one operator**, not two designs to keep in
agreement.

## 1. `create-if-missing` is a conditional effect, and the distinction is visible to a player

**It has a name and a standard treatment.** STRIPS operators cannot express it; **ADL (Pednault,
1989) added conditional effects** - effects that occur only when their effect condition holds - and
PDDL carries them as the `(when ...)` construct behind the `:conditional-effects` requirement flag.
So `create-if-missing` is not a fifth role beside `require`, `limit`, `consume` and `produce`. **It is
an effect with a guard**, and the guard sits in a different place from a precondition.

**Why that placement is not a modelling nicety here.** A precondition decides whether the action
**applies**. A guard decides whether one **effect fires**. Today's garrison is `limit 0` **and**
`produce 1`: the condition is a precondition, so a garrison already present makes `deploy ark`
**inapplicable**. Sean's version keeps the action applicable and skips one effect.

**In a selection-only interface, that is the difference between the player seeing the option and not
seeing it** - because the menu is the set of applicable actions. This lane's own inference, and it is
the reason the question is worth his attention rather than a footnote: the two forms are equivalent in
a console, where you type what you like and get told no, and **they are not equivalent in an
interface**, where inapplicable means invisible. That difference is observable by a person, which is
this repository's own test of a real thing.

### And there is a second half, which is why no role says it

**In STRIPS an effect is a fact, and adding a fact that already holds is a no-op.** Create-if-missing
is *free* in that world - it is what an add effect already means. **The four roles are quantities**,
which is the numeric-fluent world introduced by **PDDL 2.1 (Fox and Long, 2003)**, where effects
`increase` and `decrease` and **no increase is idempotent**.

So the reason none of the four roles can say *create if missing* is not an oversight in the four. **It
is that a garrison and a citizen are different kinds of thing sharing one Qty column**: a garrison is
a fact - present or absent, at most one - and citizens are a quantity. `produce 1` means *one more*
for a citizen and *bring into being* for a garrison, and only the second is idempotent.

**Where the analogy stops, and it is worth knowing before leaning on it.** Classical planning assumes
a **fixed universe of objects**; `territory.create citizen 2` brings objects into existence. Object
creation sits outside STRIPS and ADL, and is a known hard edge rather than a solved one. The
precondition-and-effect vocabulary is sound for everything else in the sketch and this is the part it
does not cover.

## 2. The subject moves: a relation names two things, a function names one

`{deploy-ark territory:1}` is **relational** - the operator takes the territory as a parameter, and
something must constrain the ark to be above it. `ark.location.below` is **functional** - a term that
denotes the territory, so there is nothing to bind and nothing to constrain.

**The trade is documented and it runs both ways.** A function is total and single-valued: it cannot be
given the wrong territory, so a whole class of error stops being rejectable and starts being
unrepresentable. A relation allows what a function cannot express - partial information, or two
candidates - at the cost of a parameter and a precondition.

**For an interface the consequence is direct: every parameter is something a person must pick.** So
choosing the representation *is* choosing what the player selects. Derive the territory and the ark is
the whole selection; name the territory and it is a second one. **`C-74` says the blank `Where`
decides which thing a person selects, and that is exactly right** - it is the same choice, currently
made in a column whose name does not suggest it is being made.

**One consequence worth flagging.** `crates/game-model/src/rejection.rs` carries
`NotAboveThatTerritory`. Under a functional subject that rejection has nothing to reject - the
territory comes from the ark and cannot disagree with it. **A rejection that becomes unreachable is
the same event as an error becoming unrepresentable**, and this repository already treats that as a
gain worth measuring.

## 3. The verb attaching to the thing is the oldest settled thing here

**`C-74` guessed there was a name and a history, and there is.** It is the **noun-verb** or
**select-then-operate** paradigm, and its canonical statement is the **Xerox Star**, described in
Smith et al., *Designing the Star User Interface*, Byte, 1982.

**The property that makes it the right choice for a selection-only interface is not preference, it is
modelessness.** In a noun-verb system, selecting an object before choosing a command **does not put
the system in a mode**: the person can decide not to act, or select something else, without escaping
out of anything. Verb-first is modal - the system is waiting for operands.

**Sean's sentence is the paradigm, stated exactly.** *I am going to need to be able to select things,
and those things I select will have things I can do with them* is object-action: **the available verbs
are a function of the selection.** That is a forty-five-year-old design principle arrived at
independently, and this lane's only contribution is the name and the fact that it is not a matter of
taste.

**And verb-noun is right for the console for a reason that is not a concession.** Typing names an
action before its operands exist, which is what makes a command writable in a file, composable and
scriptable. **The console wants verb-noun and the interface wants noun-verb, and one operator can
present as both** - the rendering differs, the fact does not.

## 4. The table question, which `C-74` guessed would be the most useful

It was right, and the distinction has a precise name.

**A table that stores a rule is *intensional*; a table a person selects rows from is *extensional*.**
The terms are from deductive databases and are standard in Datalog as **IDB** and **EDB**: the
extensional database is a set of ground facts describing a state of the world; the intensional
database is a set of rules from which further facts are derived.

**The rule that matters is a structural one: a predicate must be IDB or EDB, and not both.** So the
identical cells are neither coincidence nor sameness. The recipe table is a **schema of operators**.
The interface's table is **its extension in one state** - the applicable ground actions - and it is
*produced from* the recipe table, never maintained beside it.

**This repository already draws this line once and not twice.** `CLAUDE.md` says specification and
presentation do not share a format, that a rendering is generated and never canonical, and that a
markdown table of game data is a rendering of a data file rather than a source. **That covers rule
against rendering-of-a-rule. It does not cover rule against world** - and a selection table is the
second, not the first. It is not a prettier recipe table; it is a different predicate.

**One warning that comes with grounding, and it bears on question two.** The number of ground actions
is the product of the parameter domains. For a planner that is a cost; **for an interface it is the
menu length**, which is a fact the person sees. That is an argument for deriving subjects rather than
parameterising them, arrived at from the interface side and agreeing with what modelessness already
suggested.

## What this lens is not saying

**None of this says what the rule should be**, and none of it belongs in `spec/` because it helped.
The vocabulary is the route, not the destination - `CLAUDE.md` names that trap directly, and this
report is deliberately a place to leave the vocabulary rather than a proposal to carry it forward.

Three things it did not settle, all of them decisions rather than facts:

- **Whether the garrison should be a fact or a quantity.** The literature says the two behave
  differently and cannot share one column's meaning. Which one a garrison is, is Sean's.
- **Whether an action's subject is derived or selected**, which is the same decision as what a player
  clicks first.
- **Whether object creation stays inside this vocabulary at all.** It is the one part of the sketch
  the planning literature does not cover, and pretending otherwise would be the failure this lens
  exists to avoid.

## Sources

- Smith, Irby, Kimball, Verplank, Harslem, *Designing the Star User Interface*, Byte, 1982 -
  [ACM](https://dl.acm.org/doi/10.1145/1500774.1500840),
  [overview](https://link.springer.com/chapter/10.1007/978-1-4757-3510-9_21)
- Noun-verb / select-then-operate, and its modelessness -
  [Usability First glossary](https://www.usabilityfirst.com/glossary/noun-verb-paradigm/)
- ADL (Pednault, 1989) and conditional effects; PDDL `(when ...)` and `:conditional-effects` -
  [ADL notes](https://tridu33.github.io/ZERO-Starting-AI-Planning-in-PDDL-Descripted-World/PlanLanguages/ADL/),
  [PDDL requirements](https://planning.wiki/ref/pddl/requirements)
- Numeric fluents, PDDL 2.1 (Fox and Long, JAIR 20:61-124, 2003) -
  [PDDL 2.1 reference](https://planning.wiki/ref/pddl21/domain),
  [commentary](https://arxiv.org/pdf/1109.5665)
- EDB and IDB, and that a predicate is one or the other -
  [Ullman, Datalog notes](http://infolab.stanford.edu/~ullman/fcdb/aut07/slides/dlog.pdf),
  [Bozen-Bolzano, Foundations of Databases](https://www.inf.unibz.it/~nutt/FDBs0809/FDBsSlides/4-datalog-2.pdf)
