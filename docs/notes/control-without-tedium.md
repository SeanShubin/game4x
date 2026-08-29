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

## What this opens that is not settled

**Is a policy editor a fourth surface?** `spec/interface.md` names three - the game, the console, the
data browser - and says all three are reachable in every build. A list of rules the player edits is
none of them. It may be part of the game surface, or a fourth, and the answer decides whether a
terminal build must offer it.

**HUD against world.** Sean noted that a heads-up layer and the 3D scene are different things with
different engine support. That is implementation, and `spec/interface.md` deliberately says nothing
about it - but a policy editor is the first thing in this project that clearly wants to be a HUD
rather than a thing drawn on a planet.

**Where the vocabulary comes from.** A rule names conditions - *unworked*, *unspent*, *adjacent to a
territory I control*. Those are facts about game state, and the console can already ask for state
with `show`. **Whether the rule vocabulary and the query vocabulary are the same vocabulary** is
worth deciding early, because they will diverge quietly if nobody does.
