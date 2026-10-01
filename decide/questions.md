# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

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
