# thin-engine

**The question**: can a thin engine run the game from data?

That is `C-114`'s open half. `C-114` is Sean's reason, and it is a reason rather than a
preference: *a thin engine running a data driven game forces inadequacies in the engine and data
structure to come to light sooner*. **A thin engine is an instrument, and the three explosions -
of the code, of the data structure, of the data - are its three readings.**

Built to `S-136`. **Nothing here is a decision** - it is research, and if its answer implies one,
that reaches Sean as a proposal through the specification lane, not from this directory.

## The answer

**Yes, and the whole of it is in two files you can read.** `data/before.4x` states everything -
the structure, the structure's own structure, the roles, the rule, the command and the world.
`data/after.4x` is that file with **one row different**. The engine is what gets from one to the
other, and `tests/first_test.rs` is what proves it. Measured on 2026-09-14:

|                                           |                                                                           |
| ----------------------------------------- | ------------------------------------------------------------------------- |
| Code that runs, in `src/`                 | **638 lines** - notation 77, store 33, schema 230, engine 294             |
| Rows of data                              | **98**, of which 7 are the world and 91 are structure, rule and command   |
| Rows that differ between before and after | **1** - `{residency what:1 where:1}` becomes `{residency what:1 where:2}` |
| Relations declared                        | **14** - ten that describe the structure, four that are the game          |
| Game nouns in code that runs              | **0**, checked against a list read out of `data/`                         |
| Tests                                     | **22**, all passing                                                       |

**The engine names nothing the game has.** `territory`, `thing`, `adjacency`, `residency` and
`move` appear in `data/`, in the tests and in comments, and in no line of `src/` that runs.

## What making the structure explicit cost, which is the reading

**638 lines against 232.** An earlier version of this prototype ran the same first test in 232
lines, with rows that had no declared columns and a `$name` substitution in place of a binding.
**Stating the structure the way `temporary-notes/first-test.md` states it cost 2.75 times the
engine**, and where it went is worth reading off:

| Module        | Was | Is      | Why                                                                     |
| ------------- | --- | ------- | ----------------------------------------------------------------------- |
| `notation.rs` | 77  | **77**  | Unchanged. A line is still `{name key:value}`                           |
| `store.rs`    | 65  | **33**  | **Shrank.** `$name` substitution went away entirely                     |
| `schema.rs`   | 0   | **230** | New. Relations, ordered columns, references, and checking               |
| `engine.rs`   | 87  | **294** | Reads the rule, its clauses, their bindings and the command out of rows |

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

## The three places most likely to be unnecessary

**Sean's job is removing what is not needed and mine is filling the structure out**, so these are
marked `WHY` in `data/before.4x` rather than decided here.

**The self-description, which is 53 of the 98 rows.** Nothing needs it to run the first test - the
engine could simply know its ten relations. It is there so that *everything is data* is checkable.
**Cut it and 45 rows are left**, of which 7 are the world.

The 98 break down as: 53 the structure's own structure, 15 the game's four tables, 3 roles, 16 the
rule, 4 the command, 7 the world.

**`residency`'s key being `what`, so a thing is in one place and cannot be in two.** That is an
inference from the note rather than something it says - the note gives `residency` no key at all.
It is what makes `remove` then `add` an update rather than two unrelated edits.

**`argument.value` has no reference and cannot have one.** What it points at is whatever
`input.of` says, which is data - so its type is checked when the command runs and not when the
data is read. **It is the one place the structure cannot state a constraint that the engine still
enforces**, and if that asymmetry is unacceptable the fix is a decision rather than a patch.

## The three readings, which is what `C-114` asked for

| Does it explode?       | Against the first test, fully normalized                                                                                                     |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| **The code**           | **Yes, 2.75 times** - and all of it in two modules, for two things: knowing the structure, and reading a rule that no longer has holes in it |
| **The data structure** | **No.** One structure - a row - and it describes itself. Fourteen relations, no nesting, no second kind of thing                             |
| **The data**           | **Yes, in ceremony rather than in size.** Seven rows of world need 91 rows of structure, rule and command                                    |

**The readings point in opposite directions and that is the finding.** Everything the code gained,
the data paid for in explicitness - and the thing that did not move in either direction is the
notation. **A thin engine and an explicit structure are not the same goal**, and this is the first
measurement of the gap between them.

## The first test

`temporary-notes/first-test.md`, which `S-136` states in Sean's words: three territories `1, 2,
3`; two adjacencies `1-2` and `2-3`; one vehicle named `scout`. Moving the scout from 1 to 2
succeeds; from 1 to 3 fails. **No mechanic that is not needed to pass it.**

`tests/first_test.rs`:

- `the_engine_gets_from_before_to_after` - running command 1 turns the entirety before into the
  entirety after, compared whole
- `before_and_after_differ_by_one_row_and_nothing_else` - **the control**, without which the test
  above could pass by the two files having drifted into agreement
- `the_scout_does_not_move_to_a_place_that_is_not_adjacent` - refused, naming
  `{adjacency from:1 to:3}`
- `a_refused_command_changes_nothing`

`tests/structure.rs` checks what declaring the structure buys; `tests/isolation.rs` checks that
the engine reads no file, depends on no crate, and names no noun the game has.

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

| The check                            | Poisoned by                              | What failed                                       |
| ------------------------------------ | ---------------------------------------- | ------------------------------------------------- |
| The engine gets from before to after | the `add` effect made a no-op            | `the_engine_gets_from_before_to_after` alone      |
| Before and after really differ       | `after.4x` made identical to `before.4x` | that test **and** its control, which is the point |
| The engine names no game noun        | a game noun added to `src/`              | `no_relation_or_rule_the_data_names...`           |
| The engine reads no file             | a file read added to `src/`              | `nothing_in_src_reads_a_file_...`                 |

**The second row is why the control exists.** A test that the engine turns one file into another
passes trivially if the two files are the same, and nothing about it would look wrong.
