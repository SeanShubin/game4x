# thin-engine

**The question**: can a thin engine run the game from data?

That is `C-114`'s open half. `C-114` is Sean's reason, and it is a reason rather than a
preference: *a thin engine running a data driven game forces inadequacies in the engine and data
structure to come to light sooner*. **A thin engine is an instrument, and the three explosions -
of the code, of the data structure, of the data - are its three readings.**

`C-123` is what the instrument was pointed at first, from the other side. The main tree's engine
has a struct per game noun and a method per rule, and `spec/data/` describes to a reader what the
code already knows; nine of twenty-six recipes could be run from rows and seventeen could not.
**This asks the opposite thing: let the engine know nothing, and see whether the data can say
everything.**

Built to `S-136`. **Nothing here is a decision** - it is research, and if its answer implies one,
that reaches Sean as a proposal through the specification lane, not from this directory.

## The answer

**Three concepts cost the engine nothing, and the fourth cost it half as much again.** `src/` was
byte-identical across a second rule, a place that must exist, and a number. Then a turn - rules that
fire with no command - took it from 232 lines to 343. **The engine still names no noun the game
has**, which is the claim that survived; what did not survive is *a mechanic is rows*.

**And the reason is one sentence: the engine was thin because the command was doing the work.** Every
`$name` in a rule was bound by the player typing it. Take the player away and the engine has to find
the bindings itself, which is a query. Measured on 2026-09-14:

|                                                        |                                                                |
| ------------------------------------------------------ | -------------------------------------------------------------- |
| Code that runs, in `src/`                              | **343 lines** - notation 77, store 101, engine 162, `lib.rs` 3 |
| What that was before the turn                          | **232 lines** - so a turn cost **111 lines, 48%**              |
| Mechanics it runs                                      | **3** - `move`, `found` and `grow`, none named in the code     |
| The same engine in `crates/game-model/src/`            | **1662 lines**, of which `rules.rs` is 636                     |
| Rows of data                                           | **35** - fifteen of world, twenty of rule                      |
| Of those, rows standing in for a word the engine lacks | **6 of the world's 15** - three `vacant`, three `less`         |
| Game nouns in code that runs                           | **0**, checked against a list read out of `data/`              |
| Tests                                                  | **27**, all passing                                            |

**The two numbers are not comparable and the table says so by being read carefully.** 343 lines run
three mechanics and 1662 run twenty-six, so the honest reading was never *five times smaller*.

**The claim this file carried for three concepts was that the engine does not grow when a mechanic
is added, because a mechanic is rows.** It was tested three times and held three times, and then a
mechanic was added that is rows and it grew by 111 lines anyway. **So the claim was true of the
mechanics it had been tested on and false as stated**, and what separates the two groups is not how
big the mechanic is:

| The first three concepts                                                    | The fourth                                                        |
| --------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| Every `$name` is bound by the command                                       | No command, so nothing binds anything                             |
| The engine substitutes into a pattern and asks the world *is this row true* | The engine has to ask the world *which rows would make this true* |
| A test against a store                                                      | A join over a store                                               |
| Zero lines                                                                  | 111 lines                                                         |

**Six of the world's fifteen rows are the other half of the same story.** They are not facts about
the game anybody would want to write down; they are there because the engine has no word for *not*
and no word for *minus*. **Where the engine has no word, the data carries a row** - and where the
data cannot carry one, the engine grows. A turn is the second case.

**What is established is the part that was in doubt, and a turn did not take it away**: a rule can
be *stated* as rows - what it needs, what it drops, what it adds, and what fires it - and executed
without the engine knowing what any of it means. **`grow` is run by 343 lines that have never heard
of a settlement.** The engine's vocabulary is four relations, `rule`, `needs`, `drops` and `adds`,
plus `by` and its two values `command` and `turn` - every one of them about how a rule is written
rather than about what the game is.

## The three readings, which is what `C-114` asked for

| Does it explode?       | At four concepts                                                                                                                                                                         |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **The code**           | **Not for three concepts, and then yes.** 232 lines through `found`, *a place must exist* and fuel; 343 once rules fire without a command                                                |
| **The data structure** | **No, and it is now the one that has held.** Still one structure - a row. A rule is rows, and so is what fires it: `{rule name:grow by:turn}`                                            |
| **The data**           | **Yes, and it is the only one that has.** Six of fifteen world rows exist because the engine cannot say *not* or *minus* - `{vacant place:N}` per place, `{less of:N is:N-1}` per amount |

**All three readings have now moved, and they moved in an order worth reading.** The data went
first, at the second concept, and the code held for three - then the code went at the fourth, and
the data structure has not moved at all. **The one that never moved is the answer to `C-114`'s
middle third**: a row was enough to state every rule this prototype has, including the rule about
when rules fire.

**And the code's reading has a shape rather than a size.** It did not creep up a little per
mechanic; it sat at exactly 232 for three and then jumped 48% for one. **That is not a cost of
mechanics, it is the cost of one concept** - and the concept is search.

**And one thing is answered that the question did not ask.** The main tree's `move` is a method
whose failure is a `Rejection` variant; here a refusal is *the world does not have this row*, named
back to the caller as the row itself. `{adjacent from:1 to:3}` **is** the error message. Every
refusal this engine has is one of six, none of them about the game.

## The fourteen tests of the game, which are the whole of it

Three territories, two adjacencies, two vehicles - `data/world.4x`. Three rules - `data/rules.4x`.

`tests/moving.rs`:

- `the_scout_moves_to_a_place_that_is_adjacent` - `{move it:scout from:1 to:2 had:3 left:2}`
  succeeds, and the scout is at 2 and nowhere else. **`had` and `left` arrived with fuel** - see
  *the wall is not arithmetic* below, which is what they are evidence of
- `the_scout_does_not_move_to_a_place_that_is_not_adjacent` - `{move it:scout from:1 to:3}` is
  refused, naming `{adjacent from:1 to:3}`
- `a_place_that_does_not_exist_and_a_place_that_is_not_adjacent_refuse_differently` -
  `{move it:scout from:1 to:9}` is refused naming `{territory id:9}`, and the test asserts the two
  refusals differ rather than only asserting the words of each

`tests/founding.rs`:

- `founding_puts_a_settlement_where_the_founder_is` - `{found it:scout where:1}` succeeds, and the
  world gains `{settlement owner:scout place:1}`
- `nothing_founds_a_settlement_where_it_is_not` - `{found it:scout where:2}` is refused, naming
  `{at place:2 thing:scout}`
- `a_place_takes_one_settlement_and_the_second_founding_is_refused` - the pioneer cannot found
  where the scout already has. **Written as the requirement and not as the encoding**: *after two
  foundings at one place there is one settlement there*. What makes that true is the data's
  business, and this is what says something has.

`tests/counting.rs`:

- `moving_burns_one_fuel` - the scout has three fuel and arrives with two
- `a_vehicle_with_no_fuel_cannot_move` - the pioneer has none, and the refusal names `{less is:0
  of:0}`, the row nobody wrote
- `a_command_that_misstates_the_fuel_is_refused_on_both_halves` - claiming fuel the world does not
  state is refused by the world, and claiming to arrive with what you left with is refused by the
  table that says what paying is

`tests/turning.rs`:

- `a_turn_fires_a_rule_once_for_every_way_the_world_satisfies_it` - two settlements, two foods, and
  nothing told the engine where they were
- `a_settlement_whose_owner_walked_away_does_not_grow` - **the test that earns the join**. The scout
  founds at 2 and walks to 3, so `$who` bound by one clause has to filter the next
- `a_turn_does_not_fire_a_rule_a_command_fires` - nothing moved, because `move` says `by:command`
- `a_turn_over_a_world_with_no_settlement_changes_nothing` - the control, so that the first test
  cannot be passing over an empty search
- `a_command_cannot_fire_a_rule_that_belongs_to_the_turn` - `{grow where:1}` is refused, so `by`
  means something in both directions

`tests/isolation.rs` holds the two checks that are about the engine rather than the game - that it
reads no file and depends on no crate, and that it names no noun the game has.

**No turns, no capacity, no combat, and one resource only because moving spends it.** Every
absence below is a concept not yet added rather than a thing left undone.

## Why the notation is re-implemented, which looks like a defect and is not

`crates/command-language` already parses this notation and **this crate does not use it**.

Sean, in `S-136`: *I don't want any dependencies or assumptions creeping in from existing code,
even the notation.* **A borrowed parser is a borrowed decision about what a line may say** - and
the notation is the thing most likely to have to change here, because what the data must express
is exactly what is under test. `src/notation.rs` is 77 lines and answers to this directory alone.

The isolation is checked rather than promised, by `tests/isolation.rs`:

- the `[dependencies]` table is empty, so there is no path dependency on any crate
- `std::fs`, `include_str!` and `env!` appear in no line of `src/` that runs, so **the engine
  reads no file at all** - the tests read `data/`, and nothing reaches outside this directory
- its own `[workspace]` in `Cargo.toml`, so it is not a member of the root one and a half-built
  intermediate state cannot redden a gate every lane commits against

## The three things that look like defects and are the design

**Which refusal a reader sees is chosen by the order of rows in a data file.** `move` needs five
things and `data/rules.4x` states the destination-is-a-place clause before the adjacency one, so
moving to 9 says *there is no territory 9* rather than *1 is not next to 9*, and both are true.
The fuel clauses come last for the same reason: *you are not next to that* is a better answer than
*you cannot afford it* when both are true.
**The message is the data's to choose and not the engine's**, which is the same property as a
refusal being a row: the engine has nothing to say about `move` and so cannot rank its reasons.

**Adjacency is one-directional.** `{adjacent from:1 to:2}` does not let the scout go back. Making
it symmetric is either a second row per pair, which the data can do today, or a property of a
relation, which is a concept the engine does not have. **Nothing here needs it, so nothing here
decides it.**

**A hole nothing bound is refused rather than matched - when a command fired the rule.** `$name` is
substitution there, and a hole the command did not bind is an error rather than a wildcard.

**A turn is the other case, and it is where the day this predicted arrived.** A rule with no command
has nothing to bind its holes, so a turn matches them against the world and fires once per solution
- *which one, when several match* answered as *all of them*. **The two halves of `$name` now differ
by who fired the rule**, which is a seam rather than a design, and the section on a turn below says
what it cost.

## The concepts added since, and what each cost

**The cost of a concept is lines of `src/`**, measured the same way every time: lines that are
neither blank nor `//`, taken before `#[cfg(test)]`, summed over the four modules. **The baseline is
232**, which is what the engine was when it ran one rule - not the 343 in the table above, which is
what it became.

## 1. A place must exist - `src/` did not change

**Cost: zero lines.** `{territory id:1}` had been stated and read by nothing; one `needs` row
began reading it:

```text
{needs rule:move relation:territory id:$to}
```

**This is weaker evidence than it looks and the difference is worth stating.** A clause is not a
mechanic. What this shows is that the engine could already check a row the command did not hand
it - the machinery for *no such place* was the machinery for *not adjacent*, and the only thing
missing was a row saying so. **The claim that `src/` does not grow when a mechanic is added is
still untested**, because no mechanic has been added yet. That is concept 2.

## 2. A second rule - `src/` did not change, and the data paid instead

**Cost: zero lines of `src/`, and one row of world per territory.** `found` is the second mechanic,
and it was chosen for what the game wants rather than for what the engine can do: **a settlement is
founded where the founder is, on a place that is a place, and where there is not already one.**

**Two of those three clauses cost nothing.** They are `needs` rows like `move`'s, and they worked
first time:

```text
{needs rule:found relation:at thing:$it place:$where}
{needs rule:found relation:territory id:$where}
```

**The third is a negative, and the engine has no word for one.** A `needs` clause can say *this row
is true* and there is no clause that says *this row is not*. So the rule was first stated without
it, and what that run did is the evidence this concept turns on:

> Stated with two clauses, `{found it:pioneer where:1}` at a place the scout had already settled
> **succeeded**, and added a second settlement. Not an error - a success, silently.

**The fix is to write the absence as a fact.** The world states `{vacant place:N}` for each
territory; `found` needs that row and drops it. The second founding is then refused by the ordinary
machinery, naming `{vacant place:1}` - the row the world no longer has.

## Where the cost went, which is the whole of this reading

**There were two ways to take the negative and they put the cost in different columns.**

| Take it in     | What it costs                                                                        | What it is                                                                                 |
| -------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| **The engine** | A fifth word - `forbids`, or a `needs` that may be negated - and the lines to run it | Zero rows. Every place is silently free until something settles it                         |
| **The data**   | One row per place, per negative any rule states                                      | Zero lines. `{vacant place:N}` is an ordinary fact and `needs` was already able to read it |

**This prototype took the data, and the price is written down rather than absorbed.** A world of
ten thousand territories states ten thousand `vacant` rows, and a second rule needing a second
negative states ten thousand more - the cost is places times negatives, which is the shape that
would eventually bite.

**Neither column is free, and that is the finding rather than a complaint.** The instrument exists
to say which column a mechanic's cost lands in, and this is the first time it has had two to
compare. **Which one the real game should take is a decision and is not made here** - it reaches
Sean as a proposal through the specification lane, if it ever does.

**And one test had been asserting more than it meant.** `the_scout_moves_to_a_place_that_is_adjacent`
filtered every `at` row in the world and compared the list, under a message reading *the scout is at
2, and is nowhere else*. A second vehicle standing in territory 1 failed it. The sentence asked
about the scout and the assertion asked about the world - **the instrument answering a narrower
question than the one asked**, which `CLAUDE.md` names, found the ordinary way rather than by a
check.

## 3. A number - `src/` did not change, and the prediction above it was wrong

**Cost: zero lines of `src/`, and one row of world per amount.** Moving burns a fuel. The engine has
no arithmetic - `1` and `scout` are the same sort of thing to `src/notation.rs` - so subtraction is
stated the way the negative was, as rows:

```text
{less of:3 is:2}
{less of:2 is:1}
{less of:1 is:0}
```

**This file predicted that this concept would fail, and it did not.** Written one concept earlier,
at `cbf8f92`, and deleted by the edit that recorded the outcome - so the words are quoted here
rather than pointed at:

> **a sum is not a fact anybody can state in advance**, and there is no row that stands in for
> addition

**It was wrong, and the way it was wrong is the useful part.** Over a bounded range a sum is exactly a fact anybody can state in
advance - `{less of:3 is:2}` is the row that stands in for subtraction - and what the prediction had
confused was *unbounded* with *hard*. **Three amounts is three rows; an amount with no ceiling is
the case that has no table**, and that distinction is the whole of it.

**The floor is a row that is not there.** Nothing states what one less than zero is, so a vehicle at
zero has no move available. **That is the second time an absence has stood in for a word the engine
lacks**, after `vacant`, and the prototype found it both times by trying to state a rule rather than
by looking for it.

**The count is two and not three, and the one left out is worth naming.** Moving to territory 9 is
also refused by a row that is not there - but *no such place* is what the absence of a place row
honestly means, and nothing is standing in for anything. `vacant` and the missing `{less of:0 ...}`
are different: **both are rows written so that an absence can be asked about**, which is the thing
the engine has no word for.

## The wall is not arithmetic, it is reading the world

**`$name` is substitution and not search.** The engine can put a value the command gave it into a
pattern; it cannot go the other way and take a value out of a row. So it cannot read the scout's
fuel, and **the command has to carry what the world already knows**:

```text
{move it:scout from:1 to:2 had:3 left:2}
```

**That is sound and it is not acceptable.** Sound, because neither half can be lied about: an
overstated `had` is not a `fuel` row the world has, and a `left` that is not one less is not a
`less` row anybody wrote - `a_command_that_misstates_the_fuel_is_refused_on_both_halves` is that
test, and it fails in both directions. Not acceptable, because a player should not have to state
their own fuel and do the subtraction, and **the day a rule needs a value nobody can state in
advance, this stops being ugly and starts being impossible.**

**So the concept the engine is actually missing has a name, and it is not arithmetic.** It is a
hole that *matches* rather than substitutes - which this file has called a search since its first
commit, and named as the thing that would have to answer *which one, when several match*. Fuel is
the first mechanic to want one, and it got away without it because a vehicle has exactly one fuel
row.

**And adding a mechanic to a rule changed the form of every command that fires it and succeeds.**
`{move it:scout from:1 to:2}` became `{move it:scout from:1 to:2 had:3 left:2}`. The two refusal
tests did not change, because neither reaches the fuel clauses - **so the coupling is between a
command and the clauses a successful run actually gets to**, which is worse than it sounds: it is
invisible until the command starts working.

## What the specification already says about this, which nobody told the prototype

**The stand-in for a negative is a construction `spec/` has already reasoned through, and arriving
at it from the other end is evidence rather than a coincidence.** This directory was built knowing
nothing of the game, and `vacant` was reached by trying to state `found` and failing.

`spec/invariants.md` states the rule:

> **A rule may ask whether something is absent only where what would hold it declares a limit for
> it.** Where a limit is declared there is free capacity to record, and *none is present* is read
> from it rather than measured.

And [`docs/designing-rules.md`](../../docs/designing-rules.md) names the construction and the
reason:

> **A place bounded by a stated capacity can be zero-tested for free.** The standard
> complementary-place construction turns *is it empty* into *is the room full*, with no inhibitor
> arc and nothing lost.

**`{vacant place:N}` is that complementary place, and `{less of:N is:N-1}` is the same trick applied
to a counter.** Both work for the same reason and **both stop working at the same place**: where the
thing is bounded, the absence is a row, and where it is not, there is no row to write. So the
prototype inherits the cliff the specification already identified rather than escaping it - and
`docs/designing-rules.md` says which kinds fall off it.

**This is corroboration and not a decision.** Nothing here is normative, and that two independent
routes reached one construction is a fact about the construction, not permission to write it
anywhere.

## 4. A turn - the 232 moved, and this file said it would

**Cost: 111 lines of `src/`, which is 48%.** `store.rs` went from 65 to 101 and `engine.rs` from 87
to 162; `notation.rs` did not change at all, and neither did `lib.rs`. **The prediction this file
made one concept ago was right**, and it is quoted here rather than pointed at because the edit that
recorded the outcome deleted it - the words are at `2105590`:

> A rule that fires without a command has no command to carry `had` and `left`, **so a turn cannot
> be built the way fuel was** - the engine will have to read a value out of the world, which is the
> search this concept found the edge of. **That is the first concept with a real chance of moving
> the 232.**

**The rule itself is three rows, exactly like the others**:

```text
{rule name:grow by:turn}
{needs rule:grow relation:settlement place:$where owner:$who}
{needs rule:grow relation:at thing:$who place:$where}
{adds rule:grow relation:food place:$where}
```

**So the data structure did not move and the code did**, which is the reverse of every concept
before it, and it is the whole reason the two are counted separately.

## What the 111 lines are, since a number on its own is not a reading

**Thirty-six of them are a search**, `solutions` in `src/store.rs`: every way one pattern can be
made true by one row, extending what is already bound. **It is `fill` run backwards** - `fill` puts a
bound value into a pattern and refuses a hole nothing bound; this takes a hole nothing bound and
reads a value out of a row.

**A value already bound filters rather than rebinds**, and that one line is what joins a rule's
clauses into a rule rather than two. **It is also the line a test had to be added for.** `grow` was
written with one `needs` clause, and with one clause there is nothing to join - so the property was
unchecked and nothing said so. The poison confirms it: with the filter removed, four of the five
turn tests still pass and only `a_settlement_whose_owner_walked_away_does_not_grow` fails. **The
test that separates them is the one where the answer differs**, and it had to be built on purpose,
because in the obvious world both settlements have somebody standing in them and a cross product
lands on the right answer by accident.

**Seventy-five are the turn itself**, in `src/engine.rs`: `by` and the refusal that enforces it,
`turn`, and the three helpers `run` and `turn` now share rather than each having their own.

**And one pass, not a fixpoint.** Solutions are worked out against the world as the turn found it
and applied to a copy, so a rule cannot see what another firing of it has just done. **That makes a
turn terminate by construction** rather than by a rule about loops, and it is a choice: a fixpoint
would fire until nothing changed, and would not terminate for every rule anybody could write.

**What two contending firings should do is not settled, and is not settled on purpose.** No rule in
`data/` both drops something and fires on a turn, so the case has not come up. When it does it is a
concept with a name.

## What this says about the answer, which is the part worth carrying out of here

**A thin engine is thin in proportion to how much the caller has already decided.** Three concepts
cost nothing because every hole in every rule was filled in by the person typing the command - the
engine never had to find anything, only to check it. **The 232 was not the size of an engine that
runs a game from data; it was the size of an engine that runs rules somebody has already
instantiated.**

**The first rule nobody types cost 48%**, and it will be the last thing to cost that: a second turn
rule is rows again, because the search is now there. **So the shape to expect is a step and not a
slope** - flat until a concept the engine has no machinery for, then a jump, then flat again.

**And `C-114`'s question has a better-formed version now than the one it was asked in.** *Can a thin
engine run the game from data* turns out to depend on what the game asks the engine to find rather
than on how many mechanics it has. **The question worth putting to the next prototype is how much of
the game fires without a command**, because that is the part that pays for a query engine.

## What comes next

**Nothing is scheduled, and the four concepts this file set out are done.** What it would take
next, in the order the findings point:

1. **A second turn rule** - which is what would show the step is a step. If it is rows and `src/`
   does not move, the claim above is tested rather than argued
2. **Two firings that contend** - a turn rule that drops something two solutions both want, which
   is the case named above and deliberately left open
3. **An unbounded amount** - `{less of:N is:N-1}` is a table, and the specification already says
   which kinds have no ceiling to build one against

**Nothing here is a decision and none of it reaches Sean as one.** If an answer implies a decision
it goes through the specification lane as a proposal, which is where this directory's findings stop
and somebody else's work starts.

## Running it

```
cd prototypes/thin-engine
cargo test
```

It is not in the workspace, so the root `cargo test` does not reach it.

**Every check here was poisoned before being trusted**, and each one failed the run it should have:

| The check                            | Poisoned by                                          | What failed                                                        |
| ------------------------------------ | ---------------------------------------------------- | ------------------------------------------------------------------ |
| The engine names no game noun        | a game noun added to `src/`                          | `no_relation_the_data_names_appears_in_code_that_runs`             |
| The engine reads no file             | a file read added to `src/`                          | `nothing_in_src_reads_a_file_or_depends_on_another_crate`          |
| Moving needs an adjacency            | `{adjacent from:1 to:3}` added to the world          | `the_scout_does_not_move_to_a_place_that_is_not_adjacent`          |
| A place must exist                   | the `territory` clause absent                        | the refusal named `{adjacent from:1 to:9}`, not `{territory id:9}` |
| A place takes one settlement         | the `vacant` clause absent                           | the second founding succeeded, silently                            |
| A place takes one settlement         | the `drops vacant` row removed                       | the second founding succeeded, silently                            |
| Founding needs a free place          | `{vacant place:1}` removed from the world            | the *first* founding was refused                                   |
| Moving burns a fuel                  | `{less of:3 is:2}` removed                           | the scout could not move at all                                    |
| Moving burns a fuel                  | the `drops fuel` row removed                         | the scout kept three fuel and gained two                           |
| Zero fuel is a floor                 | `{less of:0 is:0}` added to the world                | a vehicle with no fuel moved                                       |
| A turn fires every solution          | `grow` set to `by:command`                           | three of the five turn tests                                       |
| A turn fires only its own rules      | `move` set to `by:turn`                              | all five - the scout moved on a turn                               |
| A bound hole filters the next clause | the filter in `solutions` removed, so a hole rebinds | **only** `a_settlement_whose_owner_walked_away_does_not_grow`      |

**The pairs are what make these worth having.** A rule that needs a row and does not drop it fires
for ever, and a rule that drops a row nothing states never fires at all - **the same clause missing
from either side, failing in opposite directions.** The `{less of:0 is:0}` row is the other shape: a
row *added* to the world rather than taken from it, because the floor there is an absence and an
absence is poisoned by filling it in.

**And the last row is the one that paid for itself.** Four of the five turn tests survive it, so
without the test that was written specifically to separate a join from a cross product, the filter
could have been deleted and the suite would have stayed green. **That is what a poison is for**: it
does not check the code, it checks whether the tests would notice.
