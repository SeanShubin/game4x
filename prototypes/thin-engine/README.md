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

**Yes so far, and the engine has not grown once.** Two mechanics and three concepts have gone into
`data/`, and `src/` is byte-identical across all three - the same 232 lines it was when it ran one
rule. **The cost did not vanish; it went into the data every time**, and where it goes is the
reading `C-114` asked for. Measured on 2026-09-14:

|                                                        |                                                              |
| ------------------------------------------------------ | ------------------------------------------------------------ |
| Code that runs, in `src/`                              | **232 lines** - notation 77, store 65, engine 87, `lib.rs` 3 |
| Mechanics it runs                                      | **2** - `move` and `found`, neither named in the code        |
| The same engine in `crates/game-model/src/`            | **1662 lines**, of which `rules.rs` is 636                   |
| Rows of data                                           | **31** - fifteen of world, sixteen of rule                   |
| Of those, rows standing in for a word the engine lacks | **6 of the world's 15** - three `vacant`, three `less`       |
| Game nouns in code that runs                           | **0**, checked against a list read out of `data/`            |
| Tests                                                  | **22**, all passing                                          |

**The two numbers are not comparable and the table says so by being read carefully.** 232 lines run
two mechanics and 1662 run twenty-six, so the honest reading is not *seven times smaller*. It is
that **the 232 does not grow when a mechanic is added** - a mechanic is rows - and that claim is
now tested rather than asserted: `found` was added and `git diff` over `src/` is empty.

**One mechanic is not a trend either, and neither is two.** What two establishes is that the first
one was not a coincidence of the engine having been written around it - which is the specific thing
one mechanic could not rule out, because `move` and the engine were written in the same hour.

**And the row under it is the one to read beside it.** Six of the world's fifteen rows are not facts
about the game anybody would want to write down. They are there because the engine has no word for
*not* and no word for *minus*. **The engine stayed at 232 lines, and those six rows are where the
lines it did not grow went.**

**What is established is narrower and is the part that was in doubt**: a rule can be *stated* as
rows - what it needs, what it drops, what it adds - and executed without the engine knowing what
any of it means. The engine's whole vocabulary is four words, `rule`, `needs`, `drops` and `adds`,
and they are about how a rule is written rather than about what the game is.

## The three readings, which is what `C-114` asked for

| Does it explode?       | At three concepts                                                                                                                                                                                                   |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **The code**           | **No, three times.** 232 lines, unchanged by `found`, by *a place must exist*, and by fuel                                                                                                                          |
| **The data structure** | **No, and this is the one that surprised.** There is one structure - a row - and rules are written in it too. `{needs rule:move relation:at thing:$it place:$from}` is the same shape as `{at thing:scout place:1}` |
| **The data**           | **Yes, and it is the only one that has.** Six of fifteen world rows exist because the engine cannot say *not* or *minus* - `{vacant place:N}` per place, `{less of:N is:N-1}` per amount                            |

**The instrument has separated its three readings, which is what it was built to do.** Two have
stayed flat across three concepts and the third has not - and the third is the one nobody was
watching, because `C-114` named all three and the code was the one in doubt. **A thin engine puts
its cost in the data, and the mechanism is that it has no word for things**, so the data carries a
row where a word would have been.

**And one thing is answered that the question did not ask.** The main tree's `move` is a method
whose failure is a `Rejection` variant; here a refusal is *the world does not have this row*, named
back to the caller as the row itself. `{adjacent from:1 to:3}` **is** the error message. Every
refusal this engine has is one of five, none of them about the game.

## The nine tests of the game, which are the whole of it

Three territories, two adjacencies, two vehicles - `data/world.4x`. Two rules - `data/rules.4x`.

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

**A hole nothing bound is refused rather than matched.** `$name` is substitution and not search: a
hole that matched anything would have to answer *which one, when several match*, and that is the
question a query language exists to answer. The day a rule needs one it arrives as a concept with
a name.

## The concepts added since, and what each cost

**The cost of a concept is lines of `src/`**, measured the same way every time: lines that are
neither blank nor `//`, taken before `#[cfg(test)]`, summed over the four modules. The baseline is
the 232 in the table above.

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

## What comes next

**A turn** - rules that fire without a command, which is where `block.4x`'s firing order lives in
the main tree and lives nowhere here. One concept, with a test that fails before it.

**And the one after it is now named rather than guessed at.** A rule that fires without a command
has no command to carry `had` and `left`, **so a turn cannot be built the way fuel was** - the
engine will have to read a value out of the world, which is the search this concept found the edge
of. **That is the first concept with a real chance of moving the 232.**

## Running it

```
cd prototypes/thin-engine
cargo test
```

It is not in the workspace, so the root `cargo test` does not reach it.

**Every check here was poisoned before being trusted**, and each one failed the run it should have:

| The check                     | Poisoned by                                 | What failed                                                        |
| ----------------------------- | ------------------------------------------- | ------------------------------------------------------------------ |
| The engine names no game noun | a game noun added to `src/`                 | `no_relation_the_data_names_appears_in_code_that_runs`             |
| The engine reads no file      | a file read added to `src/`                 | `nothing_in_src_reads_a_file_or_depends_on_another_crate`          |
| Moving needs an adjacency     | `{adjacent from:1 to:3}` added to the world | `the_scout_does_not_move_to_a_place_that_is_not_adjacent`          |
| A place must exist            | the `territory` clause absent               | the refusal named `{adjacent from:1 to:9}`, not `{territory id:9}` |
| A place takes one settlement  | the `vacant` clause absent                  | the second founding succeeded, silently                            |
| A place takes one settlement  | the `drops vacant` row removed              | the second founding succeeded, silently                            |
| Founding needs a free place   | `{vacant place:1}` removed from the world   | the *first* founding was refused                                   |
| Moving burns a fuel           | `{less of:3 is:2}` removed                  | the scout could not move at all                                    |
| Moving burns a fuel           | the `drops fuel` row removed                | the scout kept three fuel and gained two                           |
| Zero fuel is a floor          | `{less of:0 is:0}` added to the world       | a vehicle with no fuel moved                                       |

**The pairs are what make these worth having.** A rule that needs a row and does not drop it fires
for ever, and a rule that drops a row nothing states never fires at all - **the same clause missing
from either side, failing in opposite directions.** The last row is the other shape: a row *added*
to the world rather than taken from it, because the floor here is an absence and an absence is
poisoned by filling it in.
