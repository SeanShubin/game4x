# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-603 - Rule 3 now says an interface test is in the friendly form, and nobody has decided that

**to** sean · **status** open · **raised** 2026-10-01 · **asks** a decision · **kind** recovered · **into** `spec/README.md` -> rule 3 · **source** the re-read trigger firing after `P-601` landed in a section `P-600` had landed in

**`P-601` declared a second set of tests and rule 3 already had a sentence about every test's
form.** Neither proposal was wrong and together they claim something neither said.

```
already there    **A test is stated in the friendly form, and the foundation form is a
                 rendering of it.** The rendering is generated from `reviewed/` ...
P-601 landed     **There are two sets of unit tests** ... spec/tests/interface/ states what
                 the interface shows: which items are displayed, whether each is active, and
                 which one has my attention.
```

**So rule 3 now says an interface test is stated in the friendly form**, and its foundation form is
generated and run by the engine. **The friendly form states game rows**, and *displayed*, *active* and
*has attention* are not game rows - which is the open question `docs/notes/spec-backlog.md` records
and this sentence has quietly answered.

## The choice, and it is not about wording

**Either the friendly form stretches**, and an interface test is rows in the same notation - which
means relations for menu items and attention, and the engine running them. **That is the cheaper
answer and it is a real claim about the game's data model**, not a formatting decision.

**Or it does not**, and the sentence about the friendly form has to say which set it is about - at
which point the two sets differ in more than subject, and `reviewed/`, the foundation rendering and
the suite all have one shape for one set and another for the other.

**Nothing is offered until you choose**, because the wording follows the answer and not the other way
round.

## Why this is a proposal rather than a note

**`CLAUDE.md` asks for it.** A promotion that makes something else stale is either refused or followed
immediately by a cleanup proposal, and the trigger that found it is the one about a section taking a
second proposal: *re-read that section whole and ask whether all of it can hold at once.* **It fired
on the first section it has ever been given two proposals in on one day**, and it caught a
contradiction neither proposal's own staleness check could see, because the contradiction is between
them rather than directional.
### P-599 - Three words decide where a file lands and they are held in Rust, not in data

**to** sean · **status** open · **raised** 2026-09-30 · **asks** a decision · **kind** recovered · **shape** text · **into** `releases/rules-become-data.md` -> `D-1` · **source** the code lane stopping on `P-598` rather than guessing a row

**`D-1` is built on the measure that `crates/game-model` holds no rule.** It does not. **But the test
script's own vocabulary is held in Rust**, and `crates/game-model/src/script.rs` says so about itself:
the words it knows *are not declared in `data/`*, and this is *the one part of this prototype that is
not yet self-describing.*

```
script   where a test script is loaded
game     where the schema, the engine and the rules are loaded
expected where a `{then}` goes, for `{compare}` to read
```

**Three words, and they decide where a file lands.** Adding a fourth store means editing Rust, which
is the thing `D-1` says is over.

## Why it surfaced now, and why it is not urgent

**It stopped a generator rather than a game.** `P-598` needs a row naming the store a scenario's
invariant rows load into, the code lane declined to guess one, and the answer turned out derivable -
the set is closed at three and two are excluded by what they are for, so **`into:game` is the only
reading left** and `S-234` says so. **Nothing is blocked.**

**What is open is only whether this counts.** `D-1`'s clause is about rules; these are not rules, they
are the vocabulary of the harness that runs the tests. **A reasonable person could say `D-1` is
satisfied and this is a separate want**, or that *the game's rules are data* was never meant to stop
at the game.

## The choice

**Say `D-1` does not reach it**, and the three words stay in Rust until something else wants them in
data. `script.rs`'s own sentence should then stop calling itself *not yet* self-describing, because
nothing is coming.

**Or say it does**, and a fourth file joins `schema.4x`, `engine.4x` and `rules.4x` - the stores
declared as rows, the way the primitives already are at `{primitive id:12 word:load}`. **That is the
shape that already exists for the neighbouring vocabulary**, which is the strongest argument for it
and is not an argument this lane should make for you.

**No wording is offered** because neither answer has any until the choice is made, and `D-1` is a
capability you have not yet vetted - so this may be a sentence in its clause rather than a new
capability at all.
