# One notation, two readers - `ee9e386~1..6a81b38`, 2026-09-06

Sean asked for two things about the new input formats: **that there are no duplicate code
fragments, and that the parsing is isolated from what it does not need to know.** This answers
those first and the code lane's own four questions second.

**Gate confirmed independently.** `cargo test --workspace --no-fail-fast` exit 0 - 54 targets, 544
passed, 0 failed. `cargo fmt` is not this lens's to run.

**A correction to my own scope, from Sean.** I recorded a baseline while the code lane was still
committing, so `6a81b38` was *after* their work rather than before it and `6a81b38..HEAD` was empty.
The range is `ee9e386~1..HEAD`, sixteen commits across two lanes.

---

## 1. The isolation is clean, and I checked it three ways

**`crates/command-language` has no dependencies at all.** Its `[dependencies]` section is empty.

**It names no game type and no game command.** Every occurrence of `deploy-ark`, `end-turn`,
`build-extractor` and the rest inside that crate is in a `#[cfg(test)]` module, building a fixture
grammar. **The real grammar is constructed one layer up**, in `crates/game-console/src/grammar.rs`,
and passed in - `parse_line(grammar, line, line_number)`. So the parser is told what is legal rather
than knowing it.

**And the data reader refuses knowledge it could have had.** `state.rs` does depend on
`game_model::containment::{Description, Entry, tree}`, which is right - it builds them - but it
deliberately does not resolve kinds: *the kind is compared and written as text; it is not looked up,
because a data file may legitimately name a kind this program does not have and the honest failure
is the comparison rather than the read.* That is `docs/architecture.md` rule 10 applied by choice
rather than by accident.

**Nothing to do here.** Recorded because *checked and sound* is the answer to a question that was
asked, and because a later report should not present this as unexamined.

## 2. The duplication is real, and it is the tokenizer rather than the parser

**The two parsers are genuinely different things and should not be merged.**
`command_language::parse_line` is **grammar-directed**: it tokenizes, then tries each declared
`Form` until one matches. `state::read` is shape-only and must stay so, for the reason quoted
above. Folding one into the other would force the data reader to hold a grammar it is written not
to have.

**What is written twice is the lexical layer.** `S-59` made every command
`{name field:value ...}` - the notation data files already used. **Before it there were two
notations and two readers, which was right; after it there is one notation and two readers.** The
shared facts now stated in both places: that `{` and `}` are their own tokens, that a field splits
on the first `:`, that whitespace separates, and that `#` begins a comment.

**`command_language::tokenize` is the piece that already exists**: public, grammar-free, brace-aware,
comment-stripping, and every token carries a line and column. `game-console` already depends on
`command-language`, so using it adds no dependency and no coupling - and the column is what
`state.rs` needs for indentation depth.

## 3. The first divergence has already happened, and it is live

**Where.** `spec/console.md:21` against `crates/game-console/src/state.rs:119`.

**What.** The spec says, unqualified, in the section that covers the language: *A `#` begins a
comment. The rest of the line is ignored.* `command_language::tokenize` honours it anywhere in a
line. `state.rs` skips a line only when it **starts with** `#`, so a trailing comment is a parse
error.

**Demonstrated rather than argued**, both readers over the same text:

    "{game phase:play}"                        data reader: accept
                                               tokenizer  : ["{", "game", "phase:play", "}"]
    "{game phase:play} # a trailing comment"   data reader: reject - "`# a trailing comment`
                                                            follows the description"
                                               tokenizer  : ["{", "game", "phase:play", "}"]

**Why it costs something.** It is a rule in `spec/console.md` that one reader of the notation obeys
and the other refuses, so the same line is legal in a command file and an error in a data file. And
it is the thing the duplication was always going to produce: **nobody wrote a divergence, one reader
simply never learned a rule the other one has.**

**Whether.** Worth fixing, and the fix is the remedy for finding 2 rather than a patch: having
`state.rs` tokenize with `command_language::tokenize` makes the comment rule impossible to diverge
again, because there is then one implementation of it. If instead the rule is meant to be
line-start-only in data files, that is a change to `spec/console.md` and belongs in the queue - but
it cannot stay as it is, because the document says one thing and the two readers do different ones.

## 4. The code lane's own questions

**Their `C-53`, which they most wanted checked: they have read `spec/logistics.md` correctly.**
Lines 18-20, in the specification's own words: *That maximum is its **total capacity** for that
kind, and it is **stored**. **Used capacity** is how many of that kind it holds, and **available
capacity** is the total less the used; **both are derived**.* So total is stored and the derived
pair is used and available. A promotion reasoning that total capacity is computed from what a thing
holds contradicts that, and the finding they were hesitating over is sound.

**Their other three were not examined**, and are theirs as reported: the `move` field's form
(`C-56`), the count that was narrow (`C-54`), and the check whose population went to zero. Sean's
brief was duplication and isolation, and that is where the time went.

## 5. Noted, and deliberately not

`Description::kind` is `&'static str`, which suits a writer using literals and costs the reader a
`Box::leak` per kind read - `state.rs:218`. It is bounded by the number of distinct kinds in a file
and harmless at these sizes. Recorded because it is the same shape as the rest of this report - a
type shaped for one side of the notation, paid for by the other - and so that a later report does
not present it as new.
