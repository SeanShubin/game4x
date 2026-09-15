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

**Yes for two mechanics, and the engine did not grow to take the second.** `move` and `found` both
run from data, with neither word anywhere in the code, and `src/` is byte-identical across the
second one. **The cost did not vanish; it moved into the data**, and where it went is the reading
`C-114` asked for. Measured on 2026-09-14:

|                                             |                                                              |
| ------------------------------------------- | ------------------------------------------------------------ |
| Code that runs, in `src/`                   | **232 lines** - notation 77, store 65, engine 87, `lib.rs` 3 |
| Mechanics it runs                           | **2** - `move` and `found`, neither named in the code        |
| The same engine in `crates/game-model/src/` | **1662 lines**, of which `rules.rs` is 636                   |
| Rows of data                                | **22** - ten of world, twelve of rule                        |
| Game nouns in code that runs                | **0**, checked against a list read out of `data/`            |
| Tests                                       | **19**, all passing                                          |

**The two numbers are not comparable and the table says so by being read carefully.** 232 lines run
two mechanics and 1662 run twenty-six, so the honest reading is not *seven times smaller*. It is
that **the 232 does not grow when a mechanic is added** - a mechanic is rows - and that claim is
now tested rather than asserted: `found` was added and `git diff` over `src/` is empty.

**One mechanic is not a trend either, and neither is two.** What two establishes is that the first
one was not a coincidence of the engine having been written around it - which is the specific thing
one mechanic could not rule out, because `move` and the engine were written in the same hour.

**What is established is narrower and is the part that was in doubt**: a rule can be *stated* as
rows - what it needs, what it drops, what it adds - and executed without the engine knowing what
any of it means. The engine's whole vocabulary is four words, `rule`, `needs`, `drops` and `adds`,
and they are about how a rule is written rather than about what the game is.

## The three readings, which is what `C-114` asked for

| Does it explode?       | At two mechanics                                                                                                                                                                                                           |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **The code**           | **No, and the second mechanic is what says so.** 232 lines, unchanged by `found`                                                                                                                                           |
| **The data structure** | **No, and this is the one that surprised.** There is one structure - a row - and rules are written in it too. `{needs rule:move relation:at thing:$it place:$from}` is the same shape as `{at thing:scout place:1}`        |
| **The data**           | **Yes, a little, and this is its first real reading.** The engine has no word for *not*, so an absence is written as a fact - `{vacant place:N}`, one row per place. Three of the world's ten rows say what is *not* there |

**The second mechanic is where the instrument started reading.** The code stayed at 232 lines,
which is what the first row above could not say on its own - and the third row moved off *unknown*
for the first time, in the direction the first two did not. **Two readings are still not a trend**,
and the third and fourth concepts are what would make them one.

**And one thing is answered that the question did not ask.** The main tree's `move` is a method
whose failure is a `Rejection` variant; here a refusal is *the world does not have this row*, named
back to the caller as the row itself. `{adjacent from:1 to:3}` **is** the error message. Every
refusal this engine has is one of five, none of them about the game.

## The six tests of the game, which are the whole of it

Three territories, two adjacencies, two vehicles - `data/world.4x`. Two rules - `data/rules.4x`.

`tests/moving.rs`:

- `the_scout_moves_to_a_place_that_is_adjacent` - `{move it:scout from:1 to:2}` succeeds, and the
  scout is at 2 and nowhere else
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

`tests/isolation.rs` holds the two checks that are about the engine rather than the game - that it
reads no file and depends on no crate, and that it names no noun the game has.

**No resources, no turns, no capacity, no combat**, because none of the six needs one. Every
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

**Which refusal a reader sees is chosen by the order of rows in a data file.** `move` needs three
things and `data/rules.4x` states the destination-is-a-place clause before the adjacency one, so
moving to 9 says *there is no territory 9* rather than *1 is not next to 9*, and both are true.
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

**Cost: zero lines of `src/`, and one row of world per territory.** `found` is the second
mechanic, and it was
chosen for what the game wants rather than for what the engine can do: **a settlement is founded
where the founder is, on a place that is a place, and where there is not already one.**

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

## What comes next, in order

Each is one concept, and each has a test that fails before it:

1. **A number** - everything is a string today, and the first rule that counts anything forces
   the question `src/notation.rs` records as deliberately open
2. **A turn** - rules that fire without a command, which is where `block.4x`'s firing order lives
   in the main tree and lives nowhere here

**A number is where this is most likely to fail**, and saying so now is the point of writing the
order down. A negative could be pushed into the data because an absence is a fact somebody can
state; **a sum is not a fact anybody can state in advance**, and there is no row that stands in for
addition.

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

**The last two are the pair worth having.** A rule that needs a row and does not drop it fires for
ever, and a rule that drops a row nothing states never fires at all - **the same clause missing from
either side, failing in opposite directions.**
