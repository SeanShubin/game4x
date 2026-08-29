# Control Without Tedium

**Derived.** Written by Claude from conversation, 2026-08-28. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Intermediate steps](intermediate-steps.md) · [Documentation map](../README.md)

Sean stated a principle he wants the specification never to work against: **complete control over
every little detail, and zero tedium.** He named the resolution - specify the rules for what you
would otherwise do by hand - and the thing that usually goes missing when people try it.

## Why the two goals only look contradictory

**Control and tedium are the same axis only if control means acting.** Every detail you control by
hand is a detail you must attend to every turn, so more control is more work, and the usual escape
is to take control away: an advisor, an auto-manage toggle, a governor that does what it likes.

**They come apart the moment control means *specifying* rather than *acting*.** A rule expressed once
covers every turn it applies to. The player keeps the decision and loses the repetition, which is the
whole of what tedium is.

Factorio and Distant Worlds get closest, and it is worth being precise about how. Factorio's
answer is that **the factory is the program** - a splitter is a conditional and a belt is a data
path, so specifying and building are the same act. Distant Worlds' is a set of **automation
switches**, each of which hands a whole domain to the computer. **Factorio has the middle layer and
Distant Worlds does not**, which is why one is famous for control and the other for its advisors
being overridden.

## The missing middle, which is Sean's real point

> Frameworks take away control by specifying too much, and some libraries do this by giving you an
> advanced function without the intermediate functions to compose the advanced function. It is the
> middle layer everyone misses, in games and programming.

**Low-level tools are easy to provide and high-level tools for specific problems are easy to
provide.** What is rare is the layer where the high-level thing is **visibly made of** the low-level
things and can be taken apart.

A high-level action nobody can open is a framework: it does what it does, and a player who wants
something slightly different has to abandon it and rebuild from commands. **The rebuild is the
tedium, and it arrives precisely when the player has an opinion** - which is the worst possible
moment for a game about control.

### The test this gives, which is sharp enough to check against

> The size of the change in how I specify my desire is proportional to the size of the change of the
> effect desired.

**That is a design criterion, not an aspiration.** *Develop this territory, but never build energy
extractors* is a small change in effect. If it costs one deleted line, the middle layer is present.
If it costs rewriting the whole behaviour out of primitives, it is not.

Worth stating what it forbids: **any high-level action that is not literally composed of smaller
ones.** If *develop this territory* is a function in the engine rather than a list a player can read,
the proportionality fails by construction, however well the function is written.

## What the middle layer is here

The pieces already exist, and one of them is doing more work than it looks.

**A rule is a scope, a condition and an action.** *For each unworked food extractor, if a citizen is
unspent, work it.* The action is a command the player could have typed; the condition and scope are
what save them typing it forty times.

**A policy is an ordered list of rules.** That is the composable object: readable top to bottom,
reorderable, and each line removable on its own. *Develop this territory* is not a verb - it is a
named list, and opening it shows the rules it contains.

**Running a policy is the loop the turn already has.** `spec/turn.md` says producing happens in any
order and **when everything is exhausted there is nothing left to do**. So a policy runs by trying
each rule in order, repeatedly, until none applies - which is exactly the predecessor's
`ZeroOrMoreCommand`, repeating an action until it fails, with failure as the loop's terminator rather
than an error. See [the predecessor note](game-4x-predecessor.md).

### It terminates by construction, and that is why it need not be a language

Sean: *it does not need to be Turing complete (it can be by accident, but I find that unlikely).*
**It cannot be, and the reason is already in the spec.**

Every action exhausts something, and `spec/turn.md` says **nothing becomes ready again until the turn
ends**. So each firing strictly reduces the set of things that can still act, and a policy run over a
turn cannot loop. **Termination is not a property of the rule language - it is a property of the
game**, which means the language can be as expressive as the UI can express without ever risking a
hang.

That is the strongest argument for keeping rules as condition-and-action over a scope rather than
reaching for loops or recursion: **the loop is already there and it is already bounded.**

### And it needs no new engine concept

`spec/console.md` says the design phase is commands, and
[generating versus designing](generating-versus-designing.md) notes that a planet generator is
**something that emits those commands**. A policy is the same shape one phase later: **something that
emits play commands.**

So automation adds no path into the model. `spec/invariants.md` says a game state and a transition
yield a new game state and there is no other way for state to change - and a policy is a source of
transitions, not a second kind of them. **The history still records what happened, and a game
automated end to end still rebuilds from its own history.**

## Why doing it now is the right call

Sean's reason: **to test large planets without playing through the turns.** A tiny planet is seven
turns to fill one territory; a 92-territory planet is not something to play by hand to find out
whether the numbers hold.

**It is also the cheapest moment.** Automation that is designed after the fact has to be retrofitted
onto whatever the interface grew, and the usual result is a second way to change state - the thing
`spec/invariants.md` forbids. Designed now, it is a command emitter, which is a shape the engine
already has two of.

## The complaint about automatic scouting

Sean: *"I never was made to specify precisely how the scouting works. The game always did that for
me... I was denied the ability to ever understand exactly how scouting worked."*

**The harm is not that it was automatic. It is that it was unreadable**, and the difference decides
what the fix is. If the objection were to automation, the fix would be a toggle - and Sean is
explicit that he does not want to scout by hand. **The fix is that the behaviour has to be a rule,
because rules can be read.**

**It is the framework complaint from the previous section with a different victim.** Auto-scouting is
a high-level behaviour supplied only as a whole: it cannot be opened, so it cannot be understood, and
because it cannot be understood it cannot be adjusted. The player who wants scouts to avoid a
neighbour's border has no move except to stop using it.

**The sharp edge is what this says about defaults.** A game that ships with a scouting behaviour is
not thereby in breach - it is in breach if that behaviour is *engine code* rather than a rule the
player can open. **A shipped default is a rule like any other, written by a person, readable and
editable.** That single line converts the whole class of auto-manage toggles from opaque to
inspectable without removing one of them.

## Respecifying every game is its own tedium

Sean grants the reason games do it this way: *"it would be a pain in the ass to respecify scouting
every game, which is why there is going to need to be a meta-layer of the game where players can
store."*

**That is a second kind of tedium and the first two proposals do not address it.** A rule that dies
with its game means paying the specification cost every time, which is worse than the automation it
replaced. So rules have to be objects with a life outside any one game: named, kept, reused - and,
since they are objects, given away.

## What a published build actually is here

Sean's precedents are Path of Exile passive trees and StarCraft build orders: the community finds
something strong, publishes it, and anyone can adopt it. He wants that **baked in** rather than
happening on a wiki.

**His are a different and better thing than either precedent, and the reason is that his world
varies.** A build order is a *sequence* and a passive-tree build is a *fixed allocation*; both work
because the game is the same every time. **A planet is generated**, so a food-generation build cannot
be a recording of what someone did. It has to be a policy that responds to what the planet gives it -
conditions over territories, not a list of moves.

Two things follow, and the second is the useful one:

- **It degrades rather than breaks.** A build order fails when the map is not the map it assumed. A
  policy applied to a planet with no jungle simply never fires its jungle rules.
- **A published build is also a benchmark.** Two builds run on the same seed is a controlled
  experiment, and the difference is attributable to the builds because everything else was identical.
  That closes the loop with Sean's reason for doing this now: **testing large planets without playing
  through the turns** is the same machinery as comparing two published builds.

## The question this forces: what does the history record?

**This is where the meta-layer meets `spec/invariants.md`, and it has to be settled before rules
land.** A rule is stored outside the game and can arrive from another player. A game state is
**exactly** the result of applying every transition in order. So when a build spends a player's
output for them, what goes into the history - the commands, or the fact that a build did it?

**The invariant already forces the answer and leaves no freedom.** There is no other way for state to
change, so what a rule produces must be transitions, and the history records **what was done**, not
that a rule did it. Filed as P-115.

**What that buys is worth naming, because it is more than tidiness.** A saved game replays without
the build that produced it - so a game can be shared, or re-examined a year later, with no dependency
on a file that may have changed or vanished. **The rule is readable and so is everything it did**,
which is the transparency Sean asked for in both directions at once.

## What it costs: the vocabulary becomes a public interface

**Once builds are shared, the names a rule uses are a compatibility surface.** Path of Exile builds
break every league, and the reason is not that the game got worse - it is that a published artifact
referenced a vocabulary that moved.

This is the same worry as *where the rule vocabulary comes from*, one consequence further along. If
rules name conditions the way `show` names state, then **changing a query changes every published
build that used it**. That is not an argument against sharing; it is an argument for choosing the
vocabulary deliberately rather than letting it accrete, and for deciding early what happens to a
build that names something the game no longer has.

## Conditional logic that does not look like an if

Sean: *"A scout can prioritize exploring the next unexplored adjacent jungle, then grasslands, then
forest, then mountain... It is conditional logic dressed as a priority list."*

**It is not merely dressed as one.** An ordered list where the first applicable rule fires **is** an
if/else chain, with the else supplied by the ordering rather than by a keyword. There is no syntax
for *otherwise*, because the otherwise is the next line.

**And it is the presentation the proportionality test demands, not just the friendlier one.**
Reordering two lines changes which biome a scout prefers; deleting one drops a preference; inserting
one adds it. Each is a one-gesture edit for a one-item change in behaviour, which is P-112 passing.
**A textual if/else has identical semantics and fails that test** - swapping two branches means moving
braces, and adding a case means finding where the chain ends.

So the answer to *can we have something close to an if/else* is that **the priority list already is
one**, and the question worth asking instead is what it cannot say. One thing: nesting. *If adjacent
to a rival, prefer defensible ground; otherwise prefer food* is a list inside a list - which costs
nothing beyond letting a list entry be a list.

## And, or, and three-of-five are one construct

**A threshold over a group subsumes all three.** *At least N of the following*: N equal to the group
size is **and**, N equal to one is **or**, and everything between is what boolean operators express
badly. Three of five written in and/or is a ten-term disjunction, which is unreadable and unmaintainable
- so the sophisticated case Sean asked about is the one that most needs its own construct rather than
the one that least does.

**Nesting thresholds gives every boolean function.** *A and (B or C)* is *all of [A, any of [B, C]]*.
So groups that can contain groups need no additional operators at all, and the resulting shape is a
tree - which draws as nested boxes and needs no parentheses, since the nesting is visible.

## What actually threatens termination

**Sean's caution about assignment and looping is warranted, and an earlier claim in this note was too
strong.** The exhaustion argument shows that the **outer loop** terminates - it says nothing about a
condition that evaluates forever, and it holds only while **every firing takes a game action**. A
rule that writes a value and does nothing else exhausts nothing, so the loop stops shrinking and can
spin. Assignment is precisely the construct that breaks the guarantee.

Three constructions restore it, and together they are what Sean suspected was available:

1. **Every firing takes a game action.** Each action exhausts something and `spec/turn.md` says
   nothing becomes ready again until the turn ends, so the set of things that can still act strictly
   shrinks. **This gives a bound rather than mere termination:** at most one firing per thing that
   can act, which is a number the planet fixes in advance.
2. **Conditions are finite queries over game state.** The state is finite - so many territories, so
   many units - so any quantifier over it finishes. *Finite is not the same as fast*, and a condition
   that searches over move sequences could be slow while remaining perfectly terminating.
3. **Rule references form a directed acyclic graph.** This is the one that matters most, and it is
   the answer to Sean's worry about goto: a jump whose targets can only be rules defined earlier
   cannot come back.

**None of the three is a runtime check**, which is what *by construction* has to mean here. The
editor does not offer the move that would break them - a rule cannot be dragged above one it uses,
and there is no widget that writes a value.

## The tension Sean named, and where it actually bites

Sean: *"Things we do to make it easy to avoid duplication also make us more expressive which also
allows us to express things we shouldn't."*

**He is right, and subroutines are the exact case.** He gets there himself - *I don't think we can get
around the need for something similar to a subroutine, as that is essentially what composition is* -
and it is sharper than that, because **P-112 does not merely permit composition, it requires it.** So
the one construct that cannot be declined is also the one that reintroduces recursion.

**Here the tension resolves rather than trading off**, and the resolution is construction 3: a named
rule is a subroutine, and an acyclic reference graph is a subroutine that cannot recurse. **Reuse
kept, unboundedness dropped**, and the player never sees the constraint because the editor never
offers the cycle.

**The same move handles the case that would otherwise want assignment.** What a variable is usually
for here is naming an intermediate - *the territory with the most unworked food nodes* - and that is a
query, not a stored value. **Name the condition, not the value.** A named query is a rule with no
action, it sits in the same acyclic graph, and it gives reuse without mutation - which is the
duplication problem solved without the construct that breaks termination.

**Where Sean's tension does bite is elsewhere, and tuning is the right expectation.** Nothing above
prevents a rule that is legal, terminating and *bad* - a policy that starves its own population, or
one so intricate nobody can read it. That is not a safety property and no construction gives it; it
is balance, and it is discovered by playing.

## What this opens that is not settled

**Settled while this note was being written.** The rule editor is **its own screen**, and its
interface is **two-dimensional**; three-dimensional elements in it are decoration. That answers both
questions this section originally opened - it is a fourth surface rather than part of the game
surface, and the heads-up-against-world tension does not arise, because the editor is not drawn over
a planet at all. Filed as P-116.

**Where the vocabulary comes from.** A rule names conditions - *unworked*, *unspent*, *adjacent to a
territory I control*. Those are facts about game state, and the console can already ask for state
with `show`. **Whether the rule vocabulary and the query vocabulary are the same vocabulary** is
worth deciding early, because they will diverge quietly if nobody does.

**What happens to a build that names something the game no longer has.** The previous section argues
the rule vocabulary becomes a compatibility surface the moment builds are shared. **Whether a stale
build fails, warns, or silently skips the rule it cannot resolve** is a decision, and the wrong answer
is the silent one - a build that quietly does nine tenths of what it says is worse than one that
refuses.

**How a build gets from one player to another.** Sean wants sharing *baked in* rather than left to a
wiki. That could be a file, a paste, or something the game fetches. It is a product decision rather
than an entailment, and nothing else waits on it.
