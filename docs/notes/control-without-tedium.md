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

## Can a rule play the whole game?

Sean asked whether enough is settled to write a rule that takes a single Ark in orbit to a fully
exploited planet and a launched Ark. **The actions are complete; three things are missing, and none
of them is a verb.**

### The action side is finished, and by construction rather than luck

`spec/invariants.md` says every change to game state is representable and executable as a console
command, and P-115 says a rule is a source of transitions. **So a rule that emits commands can do
anything a player can do, and the action half of P-111 needs no further work.** Walking the release's
loop against the command set finds no gap:

| Step in the loop                 | Command                                                 |
| -------------------------------- | ------------------------------------------------------- |
| Land the ark, founding           | `land <unit> <territory>`                               |
| Work the food extractor          | `work <count> <structure> <territory> [<resource>]`     |
| Build extractors on other nodes  | `build <structure> <territory> [<resource>]`            |
| Produce pioneers, spread by land | `produce <unit> <territory>`, `move <unit> <territory>` |
| Build a Yard, produce an Ark     | `build`, `produce`                                      |
| Launch                           | `launch <unit>`                                         |

**And the goal is already a checkable predicate.** `spec/control.md` defines *fully exploited* - every
territory that can be taken taken, every structure that can be built built, every storage structure
full - so the rule has something to stop at rather than a vibe.

### What is missing, one: nothing names a condition

`spec/console.md` says *commands to query the game state are available* and `show <subject>` exists,
**but no subject is named anywhere in the specification.** A rule needs to ask whether an extractor is
unworked, how much metal a territory holds, whether a citizen is ready, which neighbours are
uncontrolled. Every one of those is a fact the model must already have, and none of them has a name.

**This is the open question this note has carried from the start, now promoted from tidy-up to
blocker.** It is the only one of the three that cannot be worked around.

### What is missing, two: nothing names a thing

`move <unit> <territory>` takes a particular territory. A rule cannot name one in advance - it has to
say *which*, and *which* is a choice made against the state at the moment it fires.

**Sean's scout example already contains the answer.** *The next unexplored adjacent jungle, then
grassland, then forest, then mountain* is an ordered list of preferences that yields a thing rather
than a truth. So **a priority list is not only how a rule expresses a conditional - it is also how a
rule names a thing**, and the two jobs need one construct rather than two.

`work <count> ...` wants the same treatment in numeric form, since the common case is *as many as
can* rather than a constant.

### What is missing, three: `end turn` undoes the bound

**This is the sharp one, and it falls straight out of P-117.**

The termination argument was that every action exhausts something and nothing becomes ready again
until the turn ends. **`end turn` is precisely the action that makes everything ready again** - so a
rule that can end the turn breaks the very construction that bounded it, and could run for ever.
P-117 forbids that.

**But a rule that goes from one Ark to the next has to span turns.** So P-117 does not merely forbid
something here; it forces an explicit stop, and the design has to supply one.

**A turn budget is the cheapest stop that is guaranteed rather than hoped for.** *Run this for at most
N turns* terminates by construction, is a single number in a user interface, and is exactly the shape
of what Sean wants it for - **simulate two hundred turns on a ninety-two territory planet and tell me
what happened**. Paired with the *fully exploited* predicate it gives the natural form: run until
won, or until the budget runs out, whichever comes first.

**The elegant-looking alternative is unsound and worth naming so nobody reaches for it.** *Stop when a
turn passes in which nothing was done* fails, because `spec/turn.md` grows population at the end of a
turn - so a turn in which the player does nothing still changes the state, and the next turn may well
have something to do.

### So the answer is: not yet, and the gap is narrow

**No new verb is needed and no new engine concept is needed.** What is needed is a vocabulary for
conditions, one construct that both chooses and decides, and a stop. The first is the only one that
is genuinely undecided.

## A text form for rules

Sean wants one, and it earns its place three times over rather than being a convenience.

**It is how P-114 actually works.** *Given to another player* needs a thing to give. A build posted on
a forum, pasted into a message, or committed to a repository is text or it is nothing.

**It is the strongest form of P-113.** *Any rule can be read* is satisfied more completely by text
than by any inspector, because text can be diffed. **Two published builds can be compared line by
line**, which is how a community actually improves one.

**And it stops the two views drifting.** The failure mode is an editor that can express something the
file cannot, or a file that can express something the editor cannot open - and once either appears,
sharing quietly stops being reliable. **The property to hold is the round trip**, not the syntax:
anything the editor builds can be written, and anything written can be opened. Filed as P-119.

**There is an obvious candidate for the format and it should be examined rather than assumed.** A
rule's actions *are* commands, and `spec/console.md` already has a command language with a grammar,
error positions and a file form in `run <file>`. So a rule file could be a command file with
conditions wrapped around it, and the two would share a parser. **The question that decides it is
whether a condition vocabulary belongs in the same language as the commands** - which is the open
question above, arriving from a second direction.

## Why the predecessor's two halting checks were the wrong tool

Sean built both, and diagnosed both correctly: **halt on the same state twice in a row** misses any
cycle longer than one, and **halt on any state seen before** is bounded only by the size of the state
space, which is astronomical. Two further reasons matter more than either, and they are worth
recording because they apply to the whole family of approaches rather than to those two attempts.

**Neither would ever fire in this game.** A 4X accumulates - population grows, resources accrue,
territories are taken and not given back - and `spec/invariants.md` makes a game state exactly the
fold of every transition, so a state that has already occurred is not something the game returns to.
**Exact repetition is not the failure mode here.** The failure mode is a rule that makes small,
meaningless, genuinely-different changes for ever, and cycle detection is blind to that by
construction. It is a correct answer to a question this game does not ask.

**And a detector is not a bound.** This matters more for a game than for an interpreter: a check that
fires when it happens to fire is something the player cannot reason about, plan around, or be taught.
**A number they chose in advance is predictable**, which is the property a player actually needs.
Cycle detection also spends memory proportional to the run, which is the wrong direction for the
thing Sean wants this for - simulating two hundred turns on a large planet.

So the turn budget is not merely the cheapest answer. **It is the only one of the three that is a
bound rather than a diagnosis**, and it is filed as P-120.

## Fragments: the constraint and the mechanic are the same object

Sean's idea is to make the budget a thing the player has more or less of - *AI fragments*, stronger or
weaker along several dimensions - rather than a constant the engine imposes.

**The move is right, and the reason is that it inverts how the limit reads.** A budget imposed by the
engine is a limitation and is felt as one. A budget you acquired is power, and the same number stops
feeling arbitrary the moment it has a source.

**It also fixes a balance problem that automation otherwise creates.** If automation is free and
total from the first turn, the player becomes a spectator to their own game. **Tedium arrives late** -
the twentieth territory is tedious in a way the first is not - so automation should arrive late too,
and a resource that accrues over a game matches those two curves without anyone having to tune them
against each other.

### The dimensions are not invented; they are the measurements the structure already has

Sean's three: how many turns it can run, how many levels deep, how many elements in a single portion.

**Depth and breadth are exactly the two measurements of the acyclic reference graph** that P-117's
third construction already requires - depth is the longest path through it, elements-per-portion is
its branching factor, and the size of what a rule can express is bounded by roughly one raised
against the other. So the strength dimensions are not knobs invented for the mechanic. **They are the
graph's own axes, given a price.**

**The three split into two kinds that behave differently, and the difference shows up in the
interface.**

| Kind        | Dimension       | Checked                | What running out looks like   |
| ----------- | --------------- | ---------------------- | ----------------------------- |
| **Static**  | depth, elements | when the rule is built | the editor will not build it  |
| **Dynamic** | turns           | while the rule runs    | the rule stops and hands back |

**That is P-117 made concrete from both sides at once.** *Nothing that can be built runs forever* is
enforced statically by the editor refusing the shape, and dynamically by the turn budget running out.
The safety constraint and the game mechanic turn out to be one object seen from two directions, which
is the strongest argument for the idea: it is not a mechanic bolted onto a limitation.

### The one decision that determines whether this encourages modular design

Sean's stated purpose is *to encourage modular design*. **Nothing in the scheme does that by itself,
and one choice decides it entirely: whether a rule used by three parents costs one element or three.**

**Charge per use and the system punishes reuse and rewards copy-paste** - which is precisely backwards,
and would be discovered by players within a day. **Charge per distinct rule and factoring out shared
structure is literally how a player affords more**, so the budget teaches the lesson Sean wants
taught, without a tutorial and without a penalty.

It is the same economics that makes a shared subroutine cheaper than a duplicated one, turned into a
price the player can see.

### Fragments are a level requirement, which is the half of the precedent not yet taken

**This closes a gap in P-114 that had no answer before.** A rule is text, is not part of any one game,
and can be given to anyone. A fragment is capacity, and would be earned inside a game. So **a
downloaded build can be one you cannot yet afford to run** - which is exactly a Path of Exile build
with a level requirement, and Sean chose that precedent himself.

**It does not weaken P-114's sharing guarantee.** *A rule does the same thing for whoever holds it*
is about the rule's meaning, and affordability is not meaning. Two players running the same build get
the same behaviour; one of them may only be able to run less of it, for fewer turns.

### What has to be settled before any of this can be filed

**Does the budget belong to the rule or to the player?** If to the player, a build carries a
prerequisite and part of the game is growing into the builds you have collected. If to the rule, a
rule declares its own cost and anyone can run anything. Everything else here - how fragments are
acquired, whether they are spent or held, what happens when a build overruns - follows from that one
answer, and none of it can be guessed at usefully before it.

**Answered provisionally by Sean on 2026-08-29: the budget belongs to the rule**, analogous to
pre-allocated stack space, with an overkill number. What stays deferred is whether it ever *moves* to
the player - and that is the right ordering rather than merely the convenient one, since how large
real builds turn out to be is exactly what decides it, and nobody knows that yet. See
[the backlog entry](spec-backlog.md) for what has to be measured meanwhile, and the section above for
what the stack analogy settles.

## What the stack analogy gives, and the one place it does not transfer

Sean, 2026-08-29: *"lets make the budget belong to the rule for now, analogous to pre-allocated stack
space for a function. We will just set an overkill number that will tell us something has gone
horribly wrong if we ever actually hit it."*

**The analogy carries three things and breaks on the fourth**, and the break is worth naming because
following it too far would produce the wrong design.

**It carries: the bound travels with the thing.** A frame belongs to its function, not to whoever
called it - and that is exactly what makes P-114's *a rule does the same thing for whoever holds it*
true rather than approximately true. See the amended P-120.

**It carries: pre-allocated, not grown on demand.** The number is fixed before the rule runs, which is
what lets it be checked rather than watched. A budget that could be extended mid-run would be a
request, and a request can be refused at the worst moment.

**It carries: exhaustion is a defect report.** Nobody sizes a function's stack to be exactly consumed;
they allocate plenty, and an overflow means a bug rather than a busy day. **That framing changes what
the budget is for** - it is a backstop, not a schedule. The rule is expected to stop because it
reached its goal or ran out of things to do, and the budget only ever fires when neither happened.

**It does not carry: turns are not stack-like.** Stack is *per frame* and nests - a callee's frame
sits inside its caller's. **A turn is global**: it belongs to the game, not to the rule that is
running, so a sub-rule does not consume turns *within* its parent's allowance in the way a callee
consumes stack within its caller's. Turns are a fuel tank, not a stack.

**So the three dimensions are not three of a kind, and the analogy tells us which is which.**

| Dimension | Stack-like?                       | Belongs to       | Checked                |
| --------- | --------------------------------- | ---------------- | ---------------------- |
| Depth     | **exactly** - it *is* stack depth | the rule's shape | when the rule is built |
| Elements  | roughly - a frame's size          | the rule's shape | when the rule is built |
| Turns     | **not at all** - a global clock   | the game         | while the rule runs    |

### What the budget actually catches, which is not what it looks like

**Stack overflow in practice usually means runaway recursion** - and that failure is already
impossible here, because P-117's third construction makes rule references acyclic and the editor
never offers the cycle. **So the turn budget is not the recursion guard**; that job is already done,
statically, by something else.

What it catches is the failure this note identified when Sean's two halting checks were rejected: **a
rule that keeps finding something legitimate to do, forever, without ever reaching its goal.** Small
real progress, indefinitely. That is invisible to cycle detection because no state repeats, and
invisible to a static check because the rule is perfectly well formed.

**Two failures, two mechanisms, neither redundant.** Acyclic references catch the structural loop
before it can be built; the turn budget catches the semantic one while it runs. It is worth knowing
they are not two guards on the same thing, because someone will eventually propose dropping one.

### One consequence that is open and has a cheap answer available

**When the budget runs out, the rule has already acted.** Its commands are in the history by P-115,
and the game is in whatever state the partial run left it - so the fault is a **report**, not an undo,
unless somebody decides otherwise.

**And deciding otherwise is unusually cheap here.** A game state is exactly the fold of its
transitions, so rewinding to before the rule ran is re-folding a prefix rather than reversing
anything. **The machinery for a transactional rule already exists as a side effect of the one-function
invariant**, which is worth knowing before the question is answered on the assumption that it would be
expensive.

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
