# thin-engine

**The question**: can a thin engine run the game from data?

That is `C-114`'s open half. `C-114` is Sean's reason, and it is a reason rather than a
preference: *a thin engine running a data driven game forces inadequacies in the engine and data
structure to come to light sooner*. **A thin engine is an instrument, and the three explosions -
of the code, of the data structure, of the data - are its three readings.**

Built to `S-136`. **Nothing here is a decision** - it is research, and if its answer implies one,
that reaches Sean as a proposal through the specification lane, not from this directory.

## The invariants

**Sean, 2026-09-15**, saying what is essential here and what may bend. **They are about this
prototype's friendly notation and not about the game's command language**, which is
`spec/console.md`.

**1 - The friendly notation states the minimum.** *The user should only have to specify the minimal
needed information to execute the command in the friendly notation.*

**2 - Arguments are identified by name.** *The user should identify arguments by name rather than
id, except where the id is part of the minimal needed information to execute the command.*

**3 - Friendly to foundation is the direction that must work.** *While we should retain two-way
translation between friendly and foundation notations if we can, this should not come at the
expense of the other invariants, being able to translate from friendly to foundation is the
direction we must support, the translation from foundation to friendly can be abandoned if we
must.*

| Invariant               | May it bend                                       |
| ----------------------- | ------------------------------------------------- |
| **1**, the minimum      | no                                                |
| **2**, names before ids | no, and **1** is written into it as its exception |
| **3**, the round trip   | **yes**, and it yields to **1** and **2**         |

**2's exception is 1 applied to an identifier**, so the two cannot collide: an id appears only
where the minimum genuinely is one.

**Neither 1 nor 2 has a check yet**, and cannot until a friendly command is a rule-named row at
all. **3 is a precedence rather than a property**, so what checks it is which test is allowed to
be deleted, named below.

## What the invariants settle, which the prototype had left open

**Minting is permitted.** This README has called the minting question *closed rather than solved*,
and `tests/common/friendly.rs` rests on *nothing is minted*. **That was a consequence of the
friendly format carrying every id**, which 1 removes: a minimal command states no `command` id and
no `argument` ids, so the converter must make them. **3 is what says that cost is acceptable.**

**And 3 names the test that may go.** `tests/directories.rs` asserts both directions;
`the_friendly_source_is_what_the_foundation_renders_to` is the expendable one, and
`the_foundation_is_what_the_friendly_source_converts_to` is not.

## What 1 opens, and it is not what this lane first wrote

**This section said the minimum was `{move what:scout to:territory-2}`, and that was wrong.**
Sean, 2026-09-15: *I don't think that is true, or if it is true it is only true in a coincidental
sense and not a general sense. Say I have 3 territories, all adjacent to each other, and two scouts
in each territory. If I want to move a scout to territory-2 I have to specify if I mean a scout
from territory-1 or a scout from territory-3.*

**The error was generalizing from a one-instance world.** `before.4x` holds one scout, so `what`
happened to identify a thing, so `from` happened to be derivable. **Nothing in the model says
either** - the same failure this directory had demonstrated one commit earlier by adding a second
`residency` row and watching the structure accept it.

**`from` is part of the intent whenever `what` is indefinite**, and *a scout* is indefinite.

## Six scouts, and the second invariant is the one that breaks

**Sean's world does not merely make `from` necessary; it cannot be written down.** Rendered with
six things named `scout`, measured rather than reasoned:

```text
{residency id:1 what:1 where:territory-1}
```

**`what:1` is an id.** `tests/common/friendly.rs` makes nameability all-or-nothing per relation, so
six things sharing a name means `thing` is not nameable and every reference to one falls back to
its id. `where:territory-1` survives because territories are distinct. **That is invariant 2
failing, in the world invariant 1 was being argued about.**

**So referring to one of several interchangeable things is the open question**, and both answers
cost an invariant:

| How a scout is referred to       | What it costs                                                              |
| -------------------------------- | -------------------------------------------------------------------------- |
| a unique name per thing          | **1** - naming `scout-4` is more than the minimum when the intent is *any* |
| a shared name, `scout`           | **2** as it stands - `thing` stops being nameable and `what` renders as id |
| a kind, with `from` to narrow it | neither, and it is what Sean wrote                                         |

**The release is already on the third line, which is evidence rather than proof.**
`releases/first-release.md` -> Recipes names an individual nowhere: `consume 2 citizens`, `consume
1 metal`, `require 1 unit | moving at least 1` at `$from`. **Every ingredient is a kind narrowed by
a trait and a place** - which is `{move what:scout from:territory-1 to:territory-2}` exactly. The
prototype's single named scout is the unrepresentative world, not Sean's six.

## Scouts are fungible, so a row carries a quantity

**Sean, 2026-09-15**: *Scouts are fungable, so the id associated with scout refers to the scout
category. We are going to need to represent quantities, so I could in principle have a million
scouts in the same territory without having a million rows. If I had a million scouts across 3
territories, I would need 3 rows to express the only difference between the scouts.*

**This settles what `what:scout` refers to**, which the section above left open. It is not one of
six things sharing a name - it is **the category**, which has one row and one name. **So invariant 2
was never in danger**: `scout` is unique among categories, and the six-scout world measured above is
one the model does not have.

**And `from` stays necessary, for the reason Sean gave rather than the one this lane argued.** A
million scouts across three territories is three rows, and *a scout* names no one of them. **The
place is what picks the row**, so `{move what:scout from:territory-1 to:territory-2}` states the
minimum exactly.

## What quantities need that the engine has not got

**A key over two columns.** Three rows for one category is what is being asked for, and
`(what, where)` is what tells them apart - so that pair has to be the key. It cannot be:

```text
two `residency` rows have `what` of `1`, so it names neither
```

**A key is one column**: `src/schema.rs` returns `columns[0].name`, and the structure check counts
`(relation, that one value)`. **Keeping the surrogate `id` avoids the refusal and buys the wrong
thing** - three rows load, and so does a fourth putting scouts in territory 1 a second time, which
was tried and accepted. **The same fact then has two spellings, which is what a key exists to
stop.**

**And the three roles do not cover arithmetic.** Moving one scout out of a territory holding two
leaves one; it does not remove the row. `require`, `remove` and `add` are set operations over whole
rows, and `remove` removes *every* match. **The release already has the shape this wants**:
`releases/first-release.md` -> Recipes has a **Qty** column and the roles `consume` and `produce`
beside `require` and `put`. **The prototype's three are a subset that predates quantities.**

## The three answers, 2026-09-15

**A row at zero goes.** Sean: *Definitly goes, the model is a minimal expression of intent.*
**That is invariant 1 said of the data rather than of the notation** - a row recording that no
scouts are somewhere expresses no intent, so it is not written. A quantity reaching zero is a
deletion and not a value.

**A quantity of one is still written.** Sean: *my intuition is that it may not [be omitted], but
that is not a hard requirement, it is my estimation of what will be forced upon us to keep the
model and code simple.* **Marked soft by its author**, and it does not collide with invariant 1:
one scout of five is not derivable from anything, so stating the one *is* the minimum.

## Two sorts of input, which is what `thing` becomes

**Sean, on what the split is for:** *Looking at this from the ability to specify inputs in commands.
Territories are not fungable, they are unique by id and always have a count of 1, but scouts are
fungable, don't have an id and may have a count greater than zero. Both kinds of things may need to
be inputs. I may need to specify territory-2, or 3 scouts.*

| Sort         | Identified by | Count     | Written       |
| ------------ | ------------- | --------- | ------------- |
| non-fungible | an id         | always 1  | `territory-2` |
| fungible     | its category  | 0 or more | `3 scouts`    |

**So an input is one of two things and the `input` relation says which today by accident.** `of`
names a relation, which is enough to say *a territory*; it is not enough to say *three of the scout
category*, because the count has nowhere to go. **That is the next thing the model needs and it is
not built.**

**A reading this lane offers rather than measures**: the split may be *places against contents*.
A territory is never a `what` and always a `where`; a scout is never a `where`. If that holds it is
one rule rather than two sorts - **but it is inference, and the release is where it would be
checked** rather than here.

**What the release does say, counted rather than recalled**: it declares **19 kinds**, and
`reports/catalog.md` gives them **19 distinct signatures over 171 pairs**. Both `territory` and
`orbit` are among them, so the release models a place as a kind like any other - which is evidence
against the split being a rule about kinds and for it being a rule about *roles in a relation*.

## Where a composite key is needed, tried three ways

**Sean, 2026-09-15**: *show me an example of where we need composite keys? And does this need
manifest in the friendly notation, foundation notation, or both?*

**One relation - a residency carrying a count - and each single-column key in turn:**

| Key              | Rows that break it                            | Outcome                                 |
| ---------------- | --------------------------------------------- | --------------------------------------- |
| `what`           | scouts in three territories                   | refused: *two rows have `what` of `1`*  |
| `where`          | one territory holding scouts **and** pioneers | refused: *two rows have `where` of `1`* |
| a surrogate `id` | two rows for scouts in territory 1            | **accepted, and that is the defect**    |

**Neither column can be the key alone**, and each fails on a world the game plainly has. The third
is the one to look at:

```text
{residency id:1 what:1 where:1 count:2}
{residency id:4 what:1 where:1 count:3}
```

**The structure accepted both.** Scouts in territory 1: 2, 3, or 5? Nothing can say. **The only
complaint came from the rule, after the check had passed.**

## Why quantities are what force it, and nothing before them did

**The store is a set, and that did most of a key's work for free.** Two identical rows collapse -
`Store::add` will not hold a fact twice. So before a count existed, a duplicate residency was
*redundant* and the state it described was still unambiguous.

**A count is exactly the column that breaks that.** Two rows differing only in their count are not
identical, so the set does not collapse them, and they do not agree. **The ambiguity arrives with
the quantity**, which is why this was not a defect worth fixing until now.

## Foundation, and the friendly notation only inherits it

**A key does two jobs here, and `src/` splits them:** uniqueness, in the structure check; and
resolving a reference, in `has_key` and in the check that a reference points at a row.

**For `residency` the second job is vacant** - nothing in the schema references a residency,
verified rather than assumed. **So its key is purely a constraint and never an identifier.**

**A constraint is a schema fact, and the schema is foundation data**, so that is where the need is:
*first column wins* has to become something declared, which is rows rather than a rule in `src/`.

**The friendly notation does not change where it would be expected to.**

| Where                    | Does it change                                                         |
| ------------------------ | ---------------------------------------------------------------------- |
| a state row, `before.4x` | **no** - `{residency what:scout where:territory-1 count:2}` either way |
| the schema, `schema.4x`  | yes, but only as rows, which it renders like any other                 |
| the translator           | **it gets smaller**                                                    |

**Smaller because the special case was already redundant.** The renderer leads with the key column
and then writes the rest in declared order - and the key *is* the first column, so that is declared
order written twice. **A composite key deletes the special case rather than complicating it.** And
`Names` only generates a name for a relation something references, so a residency never had one to
lose.

**One caveat, and it is the thing to watch.** All of that holds *because nothing points at a
residency*. **If anything ever needs to, the friendly notation needs a way to name a row whose key
is two columns** - `scout@territory-1`, or whatever it would be - and that is real work on the
friendly side. It does not arise today and would arrive the first time a rule needs to refer to a
residency rather than match one.




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

|                                              |                                                                              |
| -------------------------------------------- | ---------------------------------------------------------------------------- |
| Code that runs, in `src/`                    | **1013 lines** - notation 77, store 33, schema 249, engine 333, script 316   |
| Rows of data                                 | **219** across eight files, holding **681** values                           |
| Of those, rows nothing reads                 | **8**, all bindings on a clause nothing in the suite tries to violate        |
| Values nothing reads                         | **17 of 681** - thirteen decorations, and four ids on relations with one row |
| Rows that differ between before and expected | **1** - `{residency what:1 where:1}` becomes `{residency what:1 where:2}`    |
| Relations declared                           | **16** in the game and **9** in the script, every one keyed by an `id`       |
| Game nouns in code that runs                 | **0**, checked against a list read out of `data/`                            |
| Tests                                        | **35**, all passing                                                          |

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

## The user-facing format, which is the same notation

**Sean, 2026-09-15**, giving the target and the constraint: *I need to add the constraint that name
must be unique, and the user friendly format is:*

```text
{territory id:1 name:territory-1}
{thing id:1 name:scout}
{adjacency id:1 from:territory-1 to:territory-2}
{residency id:1 what:scout where:territory-1}
```

**It is the same notation and the same rows**, with two differences and no others: every referenced
row has a `name`, generated as `<relation>-<id>` where it has none; and every reference is written
as that name rather than as the id. `before.4x` renders as those seven lines, asserted rather than
claimed.

**Keeping `id` is what makes the round trip exact.** An earlier rendering here dropped the id
wherever a row had a name, which is what made minting look like a problem. It is not one: **the
friendly format carries every id**, so foundation to friendly to foundation invents nothing.

**A name is given only where something references the relation.** That rule is read off the
example - the territories and the thing are named, the adjacencies and the residency are not, and
those two are exactly the relations nothing points at. **A name on a row nothing references would
be a value nothing reads**, which is the thing this prototype is meant not to have.

## Editing this directory, and the tool that exists for it

**Every scripted edit here goes through `tools/anchor`**, and the reason is recorded in `C-129`:
the carrier for *normalize both sides before comparing them* and *write a script to a file* lost to
the failures it prevents, because a three-part edit needed six files and three invocations while a
throwaway `str.replace` script needed one file and one command.

**`anchor edit <file> <edits-file>` is one file and one command**, however many edits. It is now
the smaller thing as well as the right one.

## Two directories, and friendly is the source

**Sean, 2026-09-15**: *Lets make friendly the source and not omit anything. This presumes we can
reliably convert between friendly and foundation. Also it is ok that sometimes they happen to be
the same thing.*

`data/friendly/` and `data/foundation/` hold **the same eight files and the same 219 rows**.
`tests/directories.rs` is what says they say the same thing, in both directions: converting the
friendly source gives the foundation row for row, and rendering the foundation gives the friendly
source back.

**One file is byte-identical in both**, and that is the *sometimes* Sean allowed for: `engine.4x`
is 35 `primitive` rows with no references and names of their own, so there is nothing to rename
and nothing to generate.

| Gains from the friendly format                                                                                                              | Renders identically                                          |
| ------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `adjacency` `argument` `binding` `clause` `column` `command` `compare` `execute` `input` `load` `reference` `residency` `state` `territory` | `primitive` `relation` `report` `role` `rule` `store` `test` |

**`column` is in the left column, which is why it is not omitted.** Its own rows cannot be named,
so a reference *to* a column is an id - but `column.relation` renders as `residency` rather than
`16`, so the table does support the format and is shown. Sean: *it is ok to omit some machinery
tables if they don't support the friendly format, but if they do support the friendly format I want
to see those too.*

## A name the foundation cannot keep is refused, not dropped

**Making friendly the source turned a harmless asymmetry into a data-loss bug**, and it took
writing the conversion the other way to see it. `territory` declares no `name`, so:

```text
{territory id:1 name:home}   ->   {territory id:1}
```

The name was **silently gone**. In the old direction that never mattered, because a generated name
was all there ever was; with friendly as the source it is an author's work disappearing in the
format they author in. **It is refused now**, and the refusal says what the generated name would
have been.

**Naming a territory therefore needs somewhere in the foundation to keep it** - a `name` column on
every relation, or a table mapping a row to a name. That is a schema decision and is not made here.

## Authoring in the friendly format, and the round trip

**Sean, 2026-09-15**: *I expect to be authoring tests in the friendly format and only
debugging/vetting in the foundation format.* So the translation has to go both ways, and it does:
**foundation to friendly to foundation is the identity for all 219 rows**, asserted per row.

**Nothing is minted.** A friendly row carries its own `id`, so translating back is resolving each
reference from a name to an id and dropping the `name` where the relation does not declare one.
No value is invented anywhere, which is why the minting question closed rather than being solved.

## The one relation that cannot satisfy the constraint

**`column`.** Forty-six rows, seventeen distinct names, **sixteen of them called `id`**. It shows
up where it hurts:

```text
{binding id:1 clause:clause-1 column:id input:it}
```

`column:id` names sixteen rows, so it names none of them.

**And it is not a renaming away.** `column.name` is not a name for the row - it is **the token a
row is keyed by**: `{residency id:1 what:1 where:1}` is written with those three words because
`residency`'s columns are called `id`, `what` and `where`. Change the name and every row of that
relation is written differently.

**So two different things are wearing the same column.** A *local* token, unique within its
relation, and a *global* name, unique everywhere. `column.name` is the first and the constraint
wants the second. **`input.name` is the same shape**: `it`, `what`, `from`, `to` are local to their
rule, and a second rule with its own `to` would collide.

**Sean chose to leave it alone**, over renaming the column or adding a second one: *binding and
column are machinery*, and the friendly format is for authoring tests and reading the game rather
than the self-description. **So a reference to a column is written as an id**:

```text
{binding id:1 clause:clause-1 column:44 input:it}
```

**All or nothing, per relation.** Some column names happen to be unique - `what`, `where` - and
taking those while falling back for the rest rendered one kind of thing two ways, `column:what`
beside `column:44`. A relation whose names collide gets none.

**And a relation that declares `name` gets no generated one either.** Its `name` slot is taken by
the token, so a generated name would appear nowhere a reader could find it, and a reference to it
would be a name that resolves against nothing. That was the first version, and `column-44` named a
row that never said it was called that.

## The user-facing style, which is not part of the engine

**Sean, 2026-09-15**: *I don't consider the translation between user friendly format and
foundational format part of the engine. The engine should only know about the foundational format.
The user friendly format is for the test harness and debugging.*

**So it lives in `tests/common/friendly.rs`, and that is what keeps it free.** `src/` may name no
noun the game has, and every constant in it is a word `data/engine.4x` lists as delegated. Neither
applies in `tests/`. **The translator can be as thick as it likes and the engine does not grow a
line** - it is still 1013.

**All eight files render**, which `every_file_in_data_renders` asserts rather than claims:

```text
data/command.4x
  command-1  rule=move
  argument-1  command=command-1  input=move.it    value=residency-1
  argument-2  command=command-1  input=move.what  value=scout
  argument-3  command=command-1  input=move.from  value=territory-1
  argument-4  command=command-1  input=move.to    value=territory-2
```

## What a row is called, in three rules

**A row with no name gets one made** - Sean: *I was thinking of having a generated name for the
user friendly style, in this case `territory-1`.* The rules are tried in order, and each is there
because the one before it was not enough:

| Rule                                         | Example                                               | Why the earlier rule was not enough                  |
| -------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- |
| The `name`, if it names one row              | `{thing id:1 name:scout}` → `scout`                   | -                                                    |
| The name qualified by what the row points at | `{column id:44 relation:16 name:id}` → `residency.id` | **Twenty-five columns are called `id`**              |
| `<relation>-<id>`                            | `{territory id:1}` → `territory-1`                    | Fifteen of twenty-five relations have no name at all |

**The second rule takes the shortest qualification that names one row.** Joining every reference
gave `move.residency.it` for an input - an input points at its rule *and* at the relation it is
typed as, and only the first of those says which input it is. It is `move.it`.

**And one reference the schema cannot state, the renderer can.** `argument.value` points at
whatever the input's `of` says, which is data rather than schema - so no `{reference ...}` row
describes it, and the translator follows the input itself. **A translator may know that; the engine
may not.**

## What is not done, and the size of it

**This renders and does not parse.** An earlier version of this section said going back could only
reproduce the original *up to renaming*, because ids would have to be minted. **Sean asked why
minting could not simply be deterministic, and it can** - that was never the difficulty.

**The narrower true statement**: the round trip is the identity only if the foundation's ids were
assigned by the same rule the minting uses. That is a constraint on how ids are chosen, not an
obstacle to translating.

**And the exposure is smaller than it reads.** Of 212 rows with an id, **100 already carry it in
the label** - `territory-1`, `binding-6`, `clause-3` - and need no minting at all. The other 112
are almost all `column` (73) and `relation` (25): **the schema and the vocabulary, which are the
stable parts.** The world state carries its own ids.

**The one real cost is what a minting rule does to stability.** Ids derived from content stop being
stable under an edit: insert a thing and, under a sorted rule, another thing's id moves - which
matters as soon as anything outside the data cites one. **Three ways out, and which is right
depends on whether the harness ever authors in the friendly style rather than only reading it**:
mint deterministically and canonicalise the foundation once; have the friendly form carry every id;
or never parse it back at all.

## The foundation style: every relation keyed by one opaque integer

**Sean, 2026-09-15**: *I want the truth of the data model to be fully normalized, which I believe
means always referencing by ids, and a single id at that. The names are decorations, but important
decorations.* And: *there will have to be two styles. The foundational style is equivalent to a
database row. The user friendly style won't be normalised, it will model user intentions.*

**The data is the foundation style now.** Every one of the 25 relations has `id` as its first
column, every reference is to one of those ids, and nothing is readable without the schema:

```text
{clause id:3 rule:1 seq:3 role:2 relation:16}
{binding id:6 clause:3 column:44 input:1}
{residency id:1 what:1 where:1}
```

**Three things fell out of it that were not obvious beforehand.**

**The engine branches on names, and the data references by ids**, so the two have to meet
somewhere. `role:2` means nothing until the row is looked up, so a value that selects a code path
is resolved to its `name` first. **That makes `role.name`, `store.name` and `relation.name`
load-bearing where every other name is decoration** - the opposite of the rule, and a consequence
of it.

**`add` must name every column and `require` must not.** Once a row has an `id`, an `add` clause
has to say what it will be - so `move` gained an input for the residency it moves, and the
residency keeps its identity across the move rather than becoming a different row. **A `require` is
a pattern**, which is what lets the adjacency clause ask whether *any* road runs that way.

**A nonsense mutation asks the wrong question of a key.** Changing an id to `mutated` leaves it
distinct, so every id in the data survived and looked dead. **An id is swapped for another row's id
now**, which collides - and that, with key uniqueness, is what made all but four of them
load-bearing.

## A key names one row, and nothing checked it

**`{thing id:1 name:scout}` and `{thing id:1 name:pioneer}` were both accepted**, and
`{residency what:1}` then pointed at neither. A reference names a row by its key, so **a key that
names two rows is a reference that names nothing** - and the foundation rests on references being
by id.

It is checked now. **Finding it needed looking for it**: no test failed, because no data had ever
had a duplicate.

**It also made two checks honest that had been passing for the wrong reason.** The reference test
added `{residency what:1 where:9}` beside an existing `{residency what:1 where:1}`, so once keys
had to be unique it was refused for the key rather than for the territory - it uses a second thing
now. And the mutation check's generated violations were a row cloned and broken, which kept the
original's key; they replace the original instead, which keeps every reference to it resolving.

## Every relation is keyed by an `id`, and `residency` is the one that should not be

**Sean, 2026-09-15**: *I want the truth of the data model to be fully normalized, which I believe
means always referencing by ids, and a single id at that. The names are decorations, but important
decorations.*

**The data does that now, and this section said otherwise until it was re-derived.** It reported
eight relations keyed by an `id` and thirteen keyed by whatever their first column happened to be,
with `adjacency` as the sharpest case. **All sixteen are keyed by an `id`**, read out of
`data/foundation/schema.4x` rather than recalled. **Nothing edited this section when they changed**,
which is `docs/notes/nothing-removes.md` happening in this lane's own file.

**A surrogate key is not the end of it.** It admits two rows stating the same fact - two residencies
for the same thing in the same place, accepted by the structure when it was tried - and once a
residency carries a quantity that is exactly what must not be admitted. **What `residency` wants is
a key over `(what, where)`**, which the next section measures and the engine has not got.



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
