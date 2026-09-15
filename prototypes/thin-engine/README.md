# thin-engine

**The question**: can a thin engine run the game from data?

That is `C-114`'s open half. `C-114` is Sean's reason, and it is a reason rather than a
preference: *a thin engine running a data driven game forces inadequacies in the engine and data
structure to come to light sooner*. **A thin engine is an instrument, and the three explosions -
of the code, of the data structure, of the data - are its three readings.**

Built to `S-136`. **Nothing here is a decision** - it is research, and if its answer implies one,
that reaches Sean as a proposal through the specification lane, not from this directory.

## The answer

**Yes, and the test that proves it is data too.** `data/test.4x` is the orchestrator: it sets the
schema up, initializes the state, executes the command, compares what came out with what was
expected, and composes a report. **Nothing in `tests/` says what the test does** - it reads
`test.4x` and runs it.

```text
{test name:the-scout-moves-to-an-adjacent-place}

{load seq:1 file:schema.4x into:game}
{load seq:2 file:rules.4x into:game}
{load seq:3 file:before.4x into:game}
{load seq:4 file:command.4x into:game}
{load seq:5 file:expected.4x into:expected}

{execute seq:6 command:1}
{compare seq:7 this:actual with:expected}
{report seq:8 title:the-first-test}
```

And the report it composes:

```text
the-first-test
  test      the-scout-moves-to-an-adjacent-place
  compared  adjacency, residency, territory, thing
  result    as expected
```

Measured on 2026-09-14:

|                                              |                                                                           |
| -------------------------------------------- | ------------------------------------------------------------------------- |
| Code that runs, in `src/`                    | **927 lines** - notation 77, store 33, schema 230, engine 294, script 288 |
| Rows of data                                 | **175** across eight files, holding **429** values                        |
| Of those, rows nothing reads                 | **0** - every row can be deleted and something fails                      |
| Values nothing reads                         | **51 of 429**, all of them surrogate keys or orderings that do not order  |
| Rows that differ between before and expected | **1** - `{residency what:1 where:1}` becomes `{residency what:1 where:2}` |
| Relations declared                           | **16** - twelve that describe the structure, four that are the game       |
| Game nouns in code that runs                 | **0**, checked against a list read out of `data/`                         |
| Tests                                        | **28**, all passing                                                       |

**The engine names nothing the game has.** `territory`, `thing`, `adjacency`, `residency` and
`move` appear in `data/`, in the tests and in comments, and in no line of `src/` that runs.

## What making the structure explicit cost, which is the reading

**873 lines against 232.** An earlier version of this prototype ran the same first test in 232
lines, with rows that had no declared columns and a `$name` substitution in place of a binding.
**Stating the structure explicitly cost 406 lines and making the test data cost another 235**, and
where it went is worth reading off:

| Module        | Was | Is      | Why                                                                     |
| ------------- | --- | ------- | ----------------------------------------------------------------------- |
| `notation.rs` | 77  | **77**  | Unchanged. A line is still `{name key:value}`                           |
| `store.rs`    | 65  | **33**  | **Shrank.** `$name` substitution went away entirely                     |
| `schema.rs`   | 0   | **230** | New. Relations, ordered columns, references, and checking               |
| `engine.rs`   | 87  | **294** | Reads the rule, its clauses, their bindings and the command out of rows |
| `script.rs`   | 0   | **234** | New. Running a test that is written down rather than compiled in        |

**The notation never moved.** Four concepts were built on it and then thrown away, a whole
relational structure was put through it, and it is the same 77 lines it was at the first commit.
**That is the strongest thing this directory has established**, and nothing set out to test it.

**`store.rs` shrinking is the surprise.** Substituting `$what` into a pattern was 47 lines; a
`{binding ...}` row saying *this column takes that input* needs none of it, because the binding is
data rather than a hole in a string. **The structure did not add a mechanism, it moved one out of
the code.**

## What the structure buys, each checked rather than claimed

**A command naming something that is not there is refused by its type.** `{input id:move.to
rule:move seq:3 name:to of:territory}` says the value is a `territory`'s key, so moving to
territory 9 is refused before `move` is looked at. **No rule says the destination must exist** -
an earlier version needed a row per rule for exactly that, and a declared structure does it once
for all of them.

**A reference is checked against the relation it names.** `{residency what:1 where:1}` reads `1`
twice and they mean different things. `tests/structure.rs` checks the two apart with values that
exist in one relation and not the other, because both being `1` is precisely the case where a
crossed reference would go unnoticed.

**A row is exactly its relation's columns** - not at least them. There is no open row, so a typo
in a column name is refused rather than quietly ignored.

**The structure describes itself.** `relation`, `column` and `reference` are declared by
`{relation ...}` rows like everything else, and checked against their own description. **That is
what makes *everything is data* a thing a test can fail.**

## Fully normalized, and what that turned out to mean

**No relation may have a column that depends on which row it is.** The version before this one
wrote an effect as one row:

```text
{effect rule:move does:remove relation:residency what:$what where:$from}
```

**`what` and `where` are `residency`'s columns, not `effect`'s.** Point that row at a different
relation and its columns change - so `effect` had no fixed columns and was not a relation at all.
It splits into two that do:

```text
{clause  id:move.3 rule:move seq:3 role:remove relation:residency}
{binding id:move.3.what  clause:move.3 column:residency.what  input:move.what}
{binding id:move.3.where clause:move.3 column:residency.where input:move.from}
```

**The rule went from 8 rows to 16, and the `$` disappeared from the data entirely.** Both are the
same change seen from two sides: a hole in a string became a row that names a column and an input.

**Every relation's key is its first column**, which is a convention rather than a dependency - so
there is no `key` relation, and where no natural key existed a surrogate was made. The dotted ones
are readable on purpose: `residency.what`, `move.from`, `move.3.where`.

## The eight files

| File          | Rows   | What it is                                                          |
| ------------- | ------ | ------------------------------------------------------------------- |
| `schema.4x`   | **78** | The shared schema: every relation, its columns in order, references |
| `script.4x`   | **16** | The orchestrator's own vocabulary, declared the same way            |
| `engine.4x`   | **36** | Every word the engine implements - where the data delegates to code |
| `rules.4x`    | **16** | `move`, as inputs, clauses and bindings                             |
| `before.4x`   | **7**  | The state before                                                    |
| `command.4x`  | **4**  | The move command                                                    |
| `expected.4x` | **7**  | The state after, written by hand from the note                      |
| `test.4x`     | **11** | The orchestrator                                                    |

## The three places most likely to be unnecessary

**Sean's job is removing what is not needed and mine is filling the structure out**, so these are
marked `WHY` in the data rather than decided here.

**The self-description, which is 56 of `schema.4x`'s 78 rows.** Nothing needs it to run the first
test - the engine could simply know its eleven relations. It is there so that *everything is data*
is checkable. **Cut it and the schema is 22 rows.**

**`{state relation:...}`, the four rows saying which relations are the game's state.** They are
what bounds the comparison: without them a diff would compare the rule and the command too, which
are in the actual because they were loaded rather than because anything happened. The alternative
is scoping to whatever relations `expected.4x` happens to mention - **the same thing decided by
omission**, where a relation left out of `expected.4x` would go unchecked silently.

**`{test name:...}`, which nothing needs while there is one test.**

**`residency`'s key being `what`, so a thing is in one place and cannot be in two.** That is an
inference from the note rather than something it says - the note gives `residency` no key at all.
It is what makes `remove` then `add` an update rather than two unrelated edits.

**`argument.value` has no reference and cannot have one.** What it points at is whatever
`input.of` says, which is data - so its type is checked when the command runs and not when the
data is read. **It is the one place the structure cannot state a constraint that the engine still
enforces**, and if that asymmetry is unacceptable the fix is a decision rather than a patch.

## The three readings, which is what `C-114` asked for

| Does it explode?       | Against the first test, fully normalized, with the test itself as data                                                                                                             |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **The code**           | **Yes, 3.8 times** - 232 to 873, in three modules, for three things: knowing the structure, reading a rule that no longer has holes in it, and running a test that is written down |
| **The data structure** | **No, and it has now taken a third kind of thing.** One structure - a row. The game is rows, the rule is rows, the command is rows, and so is the test that runs them              |
| **The data**           | **Yes, in ceremony rather than in size.** Seven rows of world need 78 of schema, 16 of rule, 4 of command and 9 of test                                                            |

**The readings point in opposite directions and that is the finding.** Everything the code gained,
the data paid for in explicitness - and the thing that did not move in either direction is the
notation. **A thin engine and an explicit structure are not the same goal**, and this is the first
measurement of the gap between them.

## The first test

`temporary-notes/first-test.md`, which `S-136` states in Sean's words: three territories `1, 2,
3`; two adjacencies `1-2` and `2-3`; one vehicle named `scout`. Moving the scout from 1 to 2
succeeds; from 1 to 3 fails. **No mechanic that is not needed to pass it.**

**The test is `data/test.4x` and `tests/first_test.rs` only runs it.** What is left in Rust is the
two things that cannot be data: handing the engine a way to read a file, and asserting the report
says what it should.

- `the_engine_gets_from_before_to_expected` - the report says *as expected*
- `the_report_says_which_relations_it_compared` - **the control**. If `{state relation:...}` were
  dropped, the comparison would scope to no relations, find no differences and report success in
  the same words; this asserts all four are named
- `the_report_reads_as_a_report` - the composed text, exactly
- `a_state_that_is_not_expected_is_reported_as_both_rows` - `expected.4x` served from `before.4x`,
  so the report has to say `NOT as expected` and name the row that left and the row that arrived

`tests/structure.rs` checks what declaring the structure buys - and one of its checks is that the
four files `tests/` assembles are the four `test.4x` loads, **which caught the two lists disagreeing
about their order the first time it ran**. `tests/isolation.rs` checks that the engine reads no
file, depends on no crate, and names no noun the game has.

## Where the data stops describing and starts delegating

**This is the boundary the prototype is for**, and `data/engine.4x` is it written down: every
string the engine holds and compares a data value against. **34 words.**

**The difference between the two kinds of row is the thing to look at.** A `{clause ...}` or
`{binding ...}` row is data the engine walks without knowing what it means, so a second rule is
rows and no code. A word in `engine.4x` is the other thing - the engine branches on it, so adding
one means writing Rust. Three kinds are mixed in that list:

| Kind                                   | Examples                    | Adding one costs |
| -------------------------------------- | --------------------------- | ---------------- |
| Column names the engine reads by name  | `name`, `seq`, `id`, `of`   | code             |
| Relation names it dispatches on        | `clause`, `binding`, `load` | code             |
| **Values in a row that select a path** | `require`, `remove`, `add`  | code             |

**The third is the purest case, and it is already in the game's data.** `{role name:require}` is a
row whose entire meaning is a match arm in `engine.rs`. That is the shape to recognise: a relation
whose rows are a list of things the engine implements, as against one whose rows the engine only
moves around.

**The list is checked against `src/` both ways** by `tests/engine.rs` - a constant with no row
fails, and a row with no constant fails - so **the boundary cannot drift**. That is the only reason
it is worth a file rather than a paragraph. **It caught something on its first run**: four
constants added an hour earlier, when `compare` was made to read the columns it had been ignoring.

**And the game's own nouns are absent from it**, which is what says the engine is thin.
`territory`, `thing`, `adjacency`, `residency` and `move` are declared in `data/` and the engine
has never heard of any of them.

## Declaring the script's vocabulary, and what it did not buy

`data/script.4x` declares `test`, `load`, `execute`, `compare` and `report` the way the game's
relations are declared, so a step is checked like any other row: a misspelt column is refused by
the structure rather than by a special case in Rust.

```text
{report seq:10 titel:the-first-test}: `report` is (seq title) and this row gives (seq titel)
```

**What it did not buy is any less delegation.** `{relation name:load}` says a `load` row has a
`file` and an `into`; it does not say what loading *is*. **Declaring it makes the delegation
nameable rather than removing it** - which is why `engine.4x` exists beside it.

**It also found a lie in the data.** `{compare seq:9 this:actual with:expected}` had been written
with two columns nothing read - data that looks meaningful and is not. They are read now, and a
`compare` naming a store that is not there is refused.

**The bootstrap, said rather than hidden.** `test.4x` loads `script.4x`, so the `load` step that
fetches the declarations runs before its own description exists. **That is the one place the data
cannot describe itself**: the step that fetches the description. Every other step is validated
before it runs.

## And `seq` sorted as text, which a tenth step found

**Every value in this notation is a string**, and ordering steps by `seq` means deciding what a
`seq` is. Sorted as text, `10` comes before `2` - so the first time the script had ten steps, every
step after the first ran in the wrong order and `report` was reached before `compare` had run.

**It failed loudly rather than quietly**, because `report` refuses when there is nothing to report
on. **That was luck rather than design**: nothing about sorting strings would have complained, and
a script whose steps are order-independent would have been silently wrong. `seq` is parsed as a
number now, and a `seq` that is not one is refused.

## Every row is used, and 51 values are not

**Sean's requirement, in his words**: *there should not be a single value I can change or delete
that doesn't end up breaking something.* `tests/mutation.rs` is that, run over the data: every row
deleted in turn, every value replaced in turn, and something has to notice.

**Every one of the 175 rows is load-bearing.** Delete any of them and the suite fails.

**51 of the 429 values are not**, and they are named and counted so the list cannot grow quietly:

| Not read                                | Count | Why                                                        |
| --------------------------------------- | ----- | ---------------------------------------------------------- |
| `column.id` in `schema.4x`              | 18    | A surrogate key nothing points at                          |
| `column.id` in `script.4x`, `engine.4x` | 12    | The same                                                   |
| `binding.id`                            | 8     | Nothing references a binding                               |
| `clause.seq`                            | 4     | Clauses apply in role passes, so same-role order is free   |
| `input.name`                            | 3     | A binding names its input by id; the name reaches an error |
| `input.seq`                             | 3     | Inputs are sorted and the order changes no outcome         |
| `argument.id`                           | 3     | Nothing references an argument                             |

**41 of the 51 are surrogate keys nothing points at.** They exist because the engine takes a
relation's key to be its first column, so **removing them means composite keys** - `binding` would
be keyed by `(clause, column)` and `argument` by `(command, input)`. That is a decision about the
structure rather than a tidy-up.

**`input.name` is redundant with `input.id`**, which is worth seeing next to the note this came
from: `input what thing` made the name the whole identity, and introducing ids made it decoration.

**`clause.seq` and `input.seq` order things whose order does not matter** - yet. A second rule
where two `remove` clauses contend would make `clause.seq` load-bearing, and that line in
`tests/mutation.rs` is where to look when it does.

## What the mutation check taught about checking

**Nineteen `{reference ...}` rows looked dead and were not.** A reference is a constraint, and a
constraint is worth nothing in a run where nothing violates it - so deleting one changed nothing
anybody looked at. **The suite had no case that needed them.** The fix was to generate a violation
for every reference the data declares, which is now part of what the check means by *working*.

**And the count of them had to be exact.** Written as *at least fifteen references*, deleting one
simply meant one fewer was tested - the loop only ever checks the references that are there. **A
floor asks whether there are enough; the question was whether they are all still there.**

**Three real gaps came out of it**, none of which any other test would have found: loads were never
validated against their own declarations, the `{test ...}` row was skipped entirely, and
`engine.4x` was read from the file rather than from what the script had loaded - so the step
loading it was dead.

## `load` exists and the engine still reads no file

**`{load seq:1 file:schema.4x into:game}` is a row, and `src/script.rs` never opens anything.** It
is handed something that can turn a name into text and asks it; the harness in `tests/` is what
reads the directory, which is where `std::fs` is allowed to be.

```rust
pub trait Files {
    fn read(&self, name: &str) -> Option<String>;
}
```

**That is not a dodge, it is the finding.** A command that loads a file cannot be pure data in an
engine that reads nothing, so **the file system is the first thing this game has needed from
outside itself** - and it arrives as one method rather than as a dependency. Everything else the
engine does is a function of rows it was handed.

**It also made the poison into a test.** Serving `expected.4x` from `before.4x` is four lines of a
different `Files`, so the case where the expected state is wrong is in the suite rather than
something run by hand once.

## What was here before, and what it established

**Four concepts were built on the earlier, schemaless structure and then removed** when the first
test was restated in relational form. The code is at `2610ae0` and the findings are worth keeping:

| Concept                 | Cost to `src/` | What it showed                                                                        |
| ----------------------- | -------------- | ------------------------------------------------------------------------------------- |
| A place must exist      | 0 lines        | The machinery for *not adjacent* was already the machinery for *no such place*        |
| A second rule (`found`) | 0 lines        | The engine has no word for *not*; an absence written as a fact stands in for one      |
| A number (fuel)         | 0 lines        | Over a bounded range a sum **is** statable as rows - a prediction here that was wrong |
| A turn (`grow`)         | **+111, 48%**  | A rule with no command has nothing to bind its holes, so the engine needed a search   |

**The engine was thin because the command was doing the work.** Every hole was bound by the player
typing it, so the engine only ever had to check a row, never find one. That is why the fourth
concept cost 48% and the first three cost nothing.

**And the negative rediscovered something `spec/` already says.** `{vacant place:N}` was the
complementary-place construction, reached by trying to state *found* and failing.
`spec/invariants.md` states the rule it obeys and `docs/designing-rules.md` names the construction
and the cliff it falls off. Filed as `C-128`, read and closed - it implied no proposal.

## Why the notation is re-implemented, which looks like a defect and is not

`crates/command-language` already parses this notation and **this crate does not use it**.

Sean, in `S-136`: *I don't want any dependencies or assumptions creeping in from existing code,
even the notation.* **A borrowed parser is a borrowed decision about what a line may say.**
`src/notation.rs` is 77 lines and answers to this directory alone.

The isolation is checked rather than promised, by `tests/isolation.rs`:

- the `[dependencies]` table is empty, so there is no path dependency on any crate
- `std::fs`, `include_str!` and `env!` appear in no line of `src/` that runs, so **the engine
  reads no file at all** - the tests read `data/`, and nothing reaches outside this directory
- its own `[workspace]` in `Cargo.toml`, so a half-built intermediate state cannot redden a gate
  every lane commits against

## Running it

```
cd prototypes/thin-engine
cargo test
```

It is not in the workspace, so the root `cargo test` does not reach it.

**Every check here was poisoned before being trusted**, and each failed the run it should have:

| The check                            | Poisoned by                           | What failed                                                                                               |
| ------------------------------------ | ------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| The engine gets from before to after | the `add` effect made a no-op         | `the_engine_gets_from_before_to_after` alone                                                              |
| The comparison really compares       | `expected.4x` served from `before.4x` | `a_state_that_is_not_expected_is_reported_as_both_rows`, which is a test rather than a poison run by hand |
| The engine names no game noun        | a game noun added to `src/`           | `no_relation_or_rule_the_data_names...`                                                                   |
| The engine reads no file             | a file read added to `src/`           | `nothing_in_src_reads_a_file_...`                                                                         |

**The second row became a test rather than staying a poison.** A test that the engine turns one
state into another passes trivially if the expected state is the state before, so the suite now
contains that case on purpose: `expected.4x` is served from `before.4x` and the report has to say
`NOT as expected` and name both rows.

**And the report says what it compared**, which is the other half of the same worry. If the
`{state relation:...}` rows were dropped, the comparison would scope to no relations, find no
differences, and report success in exactly the same words - so
`the_report_says_which_relations_it_compared` asserts all four are named.
