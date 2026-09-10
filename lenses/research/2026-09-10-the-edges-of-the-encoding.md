# What the encoding never reached, and what the notation cannot say

**2026-09-10. Research lens.** Addressed to the specification lane, answering `S-81`'s questions 3
and 6. Sean asked the two lanes to work overnight so that the specification lane has what it needs
by morning; this is the half of that owed by this lane.

[Research](README.md) · [Outbox](outbox.md) · [The formulas report](formulas.html)

## How this was measured, so it can be re-run rather than trusted

**Every normative line in `spec/` was read** - all eighteen files, 813 lines. **190 of those are
rules** rather than headings, authorship notices or navigation, over the seventeen files that carry
one; `README.md` is the directory's own guide. They were extracted by a script and
then placed by hand, because no check can ask whether a rule is encoded; what the script gives is
the denominator and the file:line, so a claim here can be disagreed with by opening the file.

**The population is `spec/` and not the release.** The release is one delivery and this lane's
prototype was copied from it, so measuring the encoding against the release measures a copy against
its original. Question 3 asks what the encoding never *reached*, and only the destination document
says what there was to reach.

**And `crates/` was opened for every claim of the form *nothing does this*.** That is the correction
this lane owes: two findings were refuted on the day this was written - `X-21` and `X-22`, both
built and neither a recipe - because neither had looked there. Where a claim below says the encoding
does not reach something, the code was checked for it too, and where the code has it the row says
so.

## Question 3: what the encoding never reached

**Named as regions, with what is in each, because a percentage would hide which parts are missing.**

### The region that is not there to reach

**`spec/combat.md` has no rules at all.** Four sections - Scales, Range, Weapons, Resolution - and
every one of them is still the scaffolding prompt that says to delete the line when the section is
written. **Zero normative lines in twenty-seven.** `spec/control.md` says at the top that weapons
and their ranges are in combat, and combat says nothing.

So the encoding did not fail to reach combat; **there is no combat**. What exists instead is the
force contest, which is in `control.md` and is a different thing: a scalar comparison at a boundary,
with no exchange, no range and no weapon. Anything the notation would need for combat cannot be
known until the file has content, and **this lane recommends nothing be inferred about it**, because
the shape of a resolution rule is exactly what decides whether the notation needs a new construct.

Two smaller cases of the same thing:

- **`spec/orbit.md` -> *What sits in orbit*** is scaffolding. The layer exists, one thing is in it -
  an Ark - and what else may be there is unwritten
- **`spec/logistics.md` -> *Moving materials*** is a heading with nothing under it. Costs must be
  paid where the thing is built, and every resource exists in a particular place, so **how a
  resource gets from the place that has it to the place that needs it is unstated.** This lane's
  `load` was invented into that hole, and question 6 has more to say about it

### The region the encoding reached the vocabulary of and none of the rules

**Force.** This is the sharpest answer to the question, and it took refuting `X-22` to see it
properly.

The encoding has **every word**: `force` is a trait of citizen, garrison, ark and pioneer with the
release's own numbers; `nature` is a trait of a territory; `max force of {...}` and
`sum force of {...}` are two of the eight expression forms, and the report calls the second one
Sean's rule written in the language.

**And not one recipe reads either word.** Over 23 recipes, `force` and `nature` appear in exactly
**one** line - `make-territory`, which *writes* nature at design time. Check 11 computes the jungle
capture sequence correctly, in Python, outside the notation - which is the same shape as
`is_fully_exploited` being in Rust.

**Corrected the same night, before this was sent.** *Nothing guards on force* would have been too
strong, and it is the mistake this report exists to stop making. The guard is there, one level down:
`unsustained` is a **derived trait**, `perish` fires on `{thing unsustained:yes}`, and the causes
table declares three of them - upkeep unmet, **breaching without greater force**, and **staying
without equal force**. So the rules are in the model as the definition of a derived trait rather
than as a recipe line. **The encoding reached the nouns, and the verbs are in a table rather than in
a recipe.**

**What was actually wrong was worse than what this lane first claimed.** Both force causes were
written `sum force of {unit in t}` - which leaves citizens out of the breach test entirely and sums
whatever is present - while the note in the very next column said *two citizens organized by a
garrison hold at 2; without one they are `max(1, 1)`.* **Prose beside a formula that disagrees with
it**, which is exactly how the primitive count went wrong on 2026-09-08.

**Fixed tonight, and the fix is the one this lane keeps having to make: state it once.** The
aggregator is now `force_rule` in the data - the organizers are the garrison and every unit, from
`spec/control.md` -> *Coordination* - a territory has a derived `force` trait, both causes read
`t.force`, and check 11 asks one function five times instead of computing the rule five times. **A
poison that empties the organizers now turns the check red**, replacing one that discarded its own
result and could never have failed.

### The regions never touched, and two of them are correctly untouched

| Region                                      | Rules | Reached     | Note                                                                                                                                                                                                                                                                             |
| ------------------------------------------- | ----- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `control.md` -> *Winning*                   | 3     | **none**    | *every territory that can be taken has been taken, every structure built everywhere it can be built, every storage structure full.* Question 6 says why the notation cannot state it                                                                                             |
| `control.md` -> *Losing*                    | 1     | **none**    | *no citizens and nothing that converts into a citizen* - a predicate over the whole game, the same shape                                                                                                                                                                         |
| `planet.md` -> *Presentation*               | 16    | **none**    | **correctly.** Drawing a sphere is not a transition and never was                                                                                                                                                                                                                |
| `planet.md` -> *Shape*, *Distance*          | 13    | 1           | `make-adjacency` is the whole of it. The Goldberg sizes and *oceans never isolate land from land* are world generation, and question 6 says the second must stay outside                                                                                                         |
| `invariants.md` -> *Control without tedium* | 12    | **none**    | The largest unreached region in `spec/`. It is about the **player's** rules - that they finish, carry a turn budget, compose from rules the player also has, and can be named, kept and given away. The notation has recipes; it has nothing that says anything *about* a recipe |
| `interface.md`                              | 6     | 1, sideways | Check 4 is the computation one of its rules needs. See below                                                                                                                                                                                                                     |
| `narrative.md`                              | 2     | n/a         | Framing, not mechanics                                                                                                                                                                                                                                                           |

### One region the encoding reached without knowing the specification had got there first

**`spec/console.md` -> *The language* already specifies most of this lane's notation**, and this
lane designed it again over two days without opening the file. The overlap is not approximate:

- `{extractor territory:1}` and `{territory id:1}` - a description is a kind plus its stored
  traits, and a field naming a kind is a reference to one. **That is the selection syntax exactly**
- **A command is named for the recipe it fires**, and there is one per player recipe
- **A command may carry a `repeat`, which is how many times it fires** - *it is not an argument of
  the recipe; it is a count of firings*. That is Sean's own answer about moving pioneers one at a
  time, already written, and it sits **outside** the recipe where this lane's `each` and `some` sit
  inside. **The two are not the same construct and both are needed**: `repeat 2` fires `move` twice
  in sequence, and `some 2 {pioneer ...}` is one firing that takes two
- **Every word in a data file is a kind, a trait, or one of a trait's values.** A file that uses any
  other word is *wrong about the game rather than describing it*

**That last line is check 10.** This lane's *could an editor build these by choosing?* is not a
proposal and never was - it is **the first measurement of a rule the specification already makes**,
and the answer today is **498 of 498 tokens and 23 of 23 recipes**. It is worth reporting that way:
not *here is a property worth having*, but *`spec/console.md` requires this, and it now holds.*

**And `show <subject>` is the selection model.** *For each action the rules permit on that subject,
it says whether it is possible now, and when it is not, what is missing.* Sean asked this lane about
selection as though it were an open design question; `X-8`'s grounding work and check 5's per-line
*blocked on* are an implementation of a command that is already specified. **The interface region is
reached by exactly one thing, and that thing is check 4**: *an action that would waste part of what
it costs says so before it is taken, and says how much* is `interface.md` -> *What an action shows*,
and hard-versus-soft attachment is the computation that obligation needs.

### The world-builder is missing the command that starts the game

`spec/console.md` lists **five** design commands. The encoding has four of them, decomposed:

| Design command                                               | In the encoding                                                      |
| ------------------------------------------------------------ | -------------------------------------------------------------------- |
| `create planet <size>`                                       | `make-territory`, `make-orbit`, `make-adjacency`                     |
| `set resource <territory> <resource> <extractors> <density>` | `make-deposit`                                                       |
| `set force <territory> <force>`                              | an argument of `make-territory`                                      |
| `set biome <territory> <biome>`                              | an argument of `make-territory`                                      |
| **`add <unit> orbit`**                                       | **nothing**                                                          |
| **`start`**                                                  | **nothing** - `phase` is a trait of the game and no recipe writes it |

**So the encoding cannot place the ark, and cannot end the design phase.** `play.py:72` puts the
starting ark in orbit with a line of Python and prints *start: one ark in orbit above territory 1*.
**The loop this lane reports as playable begins from a state no recipe produces** - which is the
same failure as computing force outside the notation, in the one place least likely to be noticed:
before the first move.

## Question 6: where the notation could not say something the game needs

**Seven, each with the line that needs it. Two are limits this lane recommends keeping.**

### 1. Competition, and the rule that settles it

> `spec/turn.md` -> *Order of operations*: Where two effects cannot both happen, they compete.
> Competing effects are gathered and resolved together, so nothing gains an advantage by being
> considered first. What settles them is a deterministic mechanic of the game, and therefore
> something a person wrote and a player can change.

**Nothing in the notation can express a contest between effects.** Sean's decision that lines are
not sequential removes order *inside* one recipe; this rule is about several recipes arriving at the
same quantity at once, and the notation has no way to name the set of contenders, no way to state
what settles them, and - the hardest part - **no way to make the settling rule a thing the player
can read and change**, which `invariants.md` requires of everything that acts on a player's behalf.

**This is the largest gap and it is not a small construct.** A resolver is a rule over rules, and
the notation has no term that denotes a recipe at all. It is also the one place where the first
release can proceed without it: `end_turn` in the code settles every territory independently and its
doc comment says why - *nothing crosses a boundary, so no territory can affect another's outcome and
the order they are taken in cannot change the result.* **The release is free of competition by
construction, and the specification is not.**

### 2. Reachability, and why it should stay outside

> `spec/planet.md`: Oceans never isolate land from land. Every territory that is not ocean can be
> reached from every other without crossing one.

**A transitive closure, and the notation has no recursion.** `each` and `some` quantify over a
description, `count {...}` counts one, and nothing iterates to a fixed point.

**This lane recommends it stay that way.** `X-9` recorded what recursion costs: with it, boundedness
stops being decidable, and boundedness is what makes a player-authored rule safe to run - which
`invariants.md` states as *a player's rules always finish*. Reachability is a **constraint on a
generated world**, checked once by the generator, not a rule the game runs. It belongs to the same
category as the Goldberg sizes, and **neither should be sayable in the notation.**

### 3. A quantity that is shared rather than moved

> `spec/units.md`: Fuel moves freely between a controlled territory that has it and anything there
> that can hold it.

**Every construct in the notation is a transition that something fires**, so the only way to write
this was `load`, a player recipe that spends an act moving energy into a tank. That is not what the
sentence says, and it collides with an invariant: `spec/invariants.md` -> *No step that is always
taken* - *no action has an intermediate step that is always taken; where one would, the action is
defined to reach the outcome directly.* **A player who must always `load` before `move` is that step
exactly.**

**What is missing is a way to say a bound reads through to what is nearby**, rather than a way to
move a quantity across a boundary. That is the same shape as what the turn's end turned out to be: a
store is a **bound**, not a container, and this lane spent a day believing otherwise. **The notation
can say *how much fits here* and cannot say *what counts as here*.**

### 4. A rule that is itself a thing

> `spec/invariants.md` -> *Control without tedium*: A rule carries the number of turns it may run,
> and stops when they are spent. · Any rule can be read and changed by the player using it. · A rule
> is not part of any one game; it can be named, kept, used in a later game, and given to another
> player.
>
> And -> *The game is data*: The definitions are part of the game state. Defining one is a
> transition like any other, so a game's history is a complete account of it, including what its
> rules were.

**A recipe in this notation is not a thing.** It has no `id`, no traits, and nothing can select one -
so a turn budget has nowhere to live, a rule cannot be given away, and **no recipe can create a
recipe**. This lane's four `creation` recipes make territories, deposits, orbits and adjacencies;
they cannot make a kind and they cannot make a recipe, which is why the design phase is a separate
list in the data rather than a guard.

**Sean has asked for this twice** - *can we define what an ark is first, then define recipes that
define what an ark does?* and *once I have a well defined, working game, I am going to want to
bootstrap the whole thing from a very small set of rules.* It is the subject of
[the kernel report](2026-09-09-bootstrapping-from-a-kernel.md), and **the answer there is two kinds
and two declarations rather than a new construct** - a `recipe` kind and a `kind` kind, which the
notation's own rule that every word is a kind, a trait or a trait's value then covers for free.

### 5. A derived relation

> `spec/logistics.md`: A thing says which of the things in it are next to which. That is a fact
> about the container rather than about its contents.
>
> `spec/planet.md`: A territory is adjacent to the space above it, and two spaces are adjacent when
> the territories below them are. **Neither is a further rule**; both are what sharing a boundary
> comes to when one place is above another.

**The notation has derived traits and no derived relations.** `metal-in-it`, `surplus`, `control`
and `unsustained` are computed and nothing writes them - and an adjacency is a **stored thing** with
`from` and `to`, made one at a time by `make-adjacency`.

So the specification says orbital adjacency **follows** from ground adjacency and is not a further
rule, and the encoding can only reproduce it by materialising every orbit-to-orbit pair as another
stored thing. **A rule the specification states as a consequence becomes data that can disagree with
its premise** - which is exactly what a derived trait exists to prevent: *nothing can leave a derived
trait wrong, because nothing writes one.*

It also puts adjacency in the wrong place. `logistics.md` makes it a fact **about the container**;
the encoding makes it a top-level thing in the game.

### 6. A predicate over the whole game

> `spec/control.md` -> *Winning*: A planet is fully exploited when every territory that can be taken
> has been taken, every structure has been built everywhere it can be built, and every storage
> structure on it is full. A player wins by launching an Ark from a fully exploited planet.

**`each` is a generator for effects and `require` guards one firing; neither is a predicate that
returns a truth value about the game.** So *the planet is fully exploited* cannot be written, which
is why `is_fully_exploited` is a Rust function - and why it went stale in the way the code lane's own
comment predicted: it dropped the storage clause as vacuous, one was added to `releases/`, and **it
did not notice.** `X-26` carries that.

The same for *Losing*: *no citizens and nothing that converts into a citizen* is a count over a
family reaching zero, and there is nowhere to put it.

**Whether this is a gap depends on a decision nobody has made:** is winning a rule of the game, or a
question asked about the game? The notation is for transitions. **A win condition changes nothing**,
so it may belong with `show` and the queries rather than with the recipes - and if it does, the
answer to question 6 here is *the notation is right and the win condition is in the wrong document*.
**This lane takes neither side**, and notes only that leaving it in Rust is the option that has
already failed once.

### 7. Taking a conserved thing apart

> `spec/resources.md`: A **conserved** resource is not destroyed by being used. It changes form, and
> what it was made into can be taken apart to get it back.

**There is no recipe that takes anything apart, and no construct that would write one generically.**
`metal-in-it` is derived - *its binding plus the metal in each of its parts* - so the number to give
back is computable, and nothing reads it. A per-kind demolition recipe would work and would be
sixteen recipes all saying the same thing, which is `X-12`'s argument for `call` arriving in a second
place.

**`X-14` carries the related half** - that the release's conservation declarations sit in the *Kinds*
table and nothing checks them, which check 1 now does. **The gap named here is different**: not that
conservation is unchecked, but that **the sentence promises an action the game does not have.**

## What this lane recommends doing with it

**Nothing here is a proposal, and none of it is filed to `spec` as a new item** - the budget is eight
and it is full. It is a reading list, in the order this lane would take it.

1. **`X-28`, filed tonight**, is the one thing here that is a defect rather than a question: two
   `spec/` files give a founding garrison different force, `P-276` made `P-48` stale on 2026-09-05
   and nothing said so, and `crates/game-model/src/territory.rs:82` settles it with a constructor
   that discards its only argument
2. **Report check 10 as `spec/console.md`'s own rule holding**, rather than as a property this lane
   invented
3. **Combat is not a gap in the encoding**, and saying so saves a reader from looking for one
4. **Force was a gap in the prototype and not in the notation**, and it is closed: the aggregator
   is data, both causes read a derived `force` trait, and a poison moves the check
5. **`X-21`'s refutation changed a number Sean will care about.** This page said a yard's 15 metal
   had to be mined in one turn, so **six of twelve** territories could ever build one. Metal
   carries, so the question is what a territory can be *standing on*: **nine of twelve can build a
   yard, six of them in a single turn.** The old six was not overturned - it was one column of a
   table, and three more territories reach it in two turns. Check 14, poisoned by taking the stores
   away, which drops it back to six
6. **Competition (6.1) and the rule-as-a-thing (6.4) are the two that need Sean**, and he has said
   something bearing on each already

**And two of the seven should be left unsayable on purpose** - reachability, because recursion costs
decidability, and possibly the win condition, because a question asked about the game is not a
transition. **A notation is judged by what it refuses as much as by what it carries**, and these are
the first two refusals this lane would defend.
