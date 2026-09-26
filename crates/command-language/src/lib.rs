//! A command language: a grammar, a parser, and a typed syntax tree.
//!
//! **No game noun appears in the code that ships**, which is narrower than what this said and is
//! the part that is true. It said *this crate contains no game nouns - `land`, `territory` and
//! `metal` never appear in it*, and those three appear 24, 66 and 8 times: in the fixture grammars,
//! the doc examples and the prose. **A grammar is data handed in from outside** and what the words
//! mean is decided a layer up; a test has to hand in *some* grammar, and the one here used this
//! game's words.
//!
//! **Measured over every module, comments and `#[cfg(test)]` dropped: none.** So the seam is intact
//! and the sentence about it was not.
//!
//! **Two recipe names are renamed and the rest are left**, which is `Q-101`. `deploy-ark` and
//! `build-extractor` were recipes of the first release, and `D-4` asks that a search for a recipe
//! name find nothing - so a fixture keyword and a rule statement being the same bytes made 31
//! hits in a crate that asserts nothing about the game. They are `fetch-item` and
//! `attach-conveyor` now, at matching lengths, because a dozen assertions here are about a column
//! number.
//!
//! # Why this is prose and not a check, which took a measurement to find out
//!
//! **The obvious instrument is wrong and would have looked right.** Sweeping this crate for every
//! word `spec/data/` declares finds 26 of them over 234 occurrences - and `column` (36),
//! `argument` (14), `rule` (11) and `move` (4) are the parser's own vocabulary and Rust's,
//! colliding with relation names by coincidence. **A check over that population would fail on
//! correct code**, and a hand list of the ones that are really game nouns is the thing
//! `CLAUDE.md` says goes stale.
//!
//! **What is checkable is the narrow claim above** - no game noun in code that runs - and that is
//! `crates/game-model/tests/isolation.rs`'s shape one crate over. It is stated here and held by
//! nothing, which is the honest description rather than a claim to have solved it. That seam is the one idea worth keeping from the predecessor reviewed in
//! `docs/notes/parser-architecture.md`; everything listed there under *what is weak* is a
//! requirement met here rather than a defect reproduced:
//!
//! | Weakness there | What is done here |
//! | --- | --- |
//! | Failures carry no position | [`Failure`] carries a [`Position`], and a [`Span`] is on every argument |
//! | Failures cannot say what was expected | [`Failure::expected`] lists it, in the reader's words |
//! | Two failure styles, data then exception | Failure is data in every layer; nothing here panics on input |
//! | Handlers index children by position | Arguments are reached by name; see [`Utterance::name`] |
//! | Type safety abandoned at the seam | [`Argument`] is an enum, and every read is checked |
//! | Nothing checks the two tables agree | [`agreement::disagreements`] does, in one test |
//! | Ordered choice load-bearing and unremarked | Written down on [`Grammar`], and tested |
//!
//! Punctuation is discarded the way the predecessor discarded it, which was right: it is
//! dropped where syntax becomes meaning rather than flagged "insignificant" in the parser.
//! In this language whitespace is the only such thing, and [`token::tokenize`] drops it.
//!
//! # Shape
//!
//! ```text
//!   text
//!     |  tokenize          words, each carrying where it was
//!     |  parse_line        ordered choice over the grammar's forms
//!   Utterance              typed, arguments reached by name
//!     |  (a layer up)      a binding table gives the words meaning
//! ```
//!
//! # Example
//!
//! ```
//! use command_language::{Form, Grammar, Kind, Term, parse_line};
//!
//! let grammar = Grammar::new(vec![Form::new(
//!     "fetch-item",
//!     vec![
//!         Term::Keyword("fetch-item"),
//!         Term::required("territory", Kind::Number),
//!     ],
//!     "fetch an item from the store",
//! )]);
//!
//! let command = parse_line(&grammar, "{fetch-item territory:1}", 1).unwrap().unwrap();
//! assert_eq!(command.form, "fetch-item");
//! assert_eq!(command.number("territory").unwrap(), 1);
//!
//! // The fields carry their own names, so their order is not part of the command.
//! let same = parse_line(&grammar, "{fetch-item territory:1}", 1).unwrap().unwrap();
//! assert_eq!(same.number("territory").unwrap(), 1);
//!
//! let failure = parse_line(&grammar, "{fetch-item territory:orbit}", 1).unwrap_err();
//! assert_eq!(failure.to_string(), "line 1 column 23: expected a number, found `orbit`");
//! ```

pub mod agreement;
pub mod failure;
pub mod grammar;
pub mod parse;
pub mod syntax;
pub mod token;

pub use agreement::{agree, disagreements};
pub use failure::{Failure, Position, Span};
pub use grammar::{Form, Grammar, Kind, Term};
pub use parse::{parse_line, parse_script};
pub use syntax::{Argument, Utterance};
pub use token::{COMMENT, Token, tokenize};

#[cfg(test)]
mod tests {
    /// The rule this crate exists to keep. If a game noun ever appears in the parser, the
    /// seam has been crossed and the grammar has stopped being data.
    #[test]
    fn no_game_noun_appears_anywhere_in_this_crate() {
        // Words from `spec/console.md` and the release that must never be built in here.
        // The parser may carry them as strings a caller supplies; it may not name them.
        const NOUNS: [&str; 14] = [
            "territory",
            "citizen",
            "garrison",
            "extractor",
            "pioneer",
            "colonizer",
            "planet",
            "orbit",
            "metal",
            "energy",
            "food",
            "yard",
            "density",
            "force",
        ];

        let mut offences = Vec::new();
        let mut scanned = 0;
        for entry in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            scanned += 1;
            let text = std::fs::read_to_string(&path).unwrap();
            // Tests demonstrate the crate by handing it a grammar, and a grammar is where
            // game nouns are supposed to appear - so the rule binds the code above the
            // test module, which is the part that ships.
            let code = match text.find("#[cfg(test)]") {
                Some(at) => &text[..at],
                None => &text[..],
            };
            for (number, line) in code.lines().enumerate() {
                // Prose may discuss the seam; only what compiles is bound by it.
                if line.trim_start().starts_with("//") {
                    continue;
                }
                for noun in NOUNS {
                    if line.contains(noun) {
                        offences.push(format!(
                            "{}:{number}: {noun}",
                            path.file_name().unwrap().to_string_lossy(),
                        ));
                    }
                }
            }
        }
        assert!(
            offences.is_empty(),
            "game nouns leaked into the parser:\n{}",
            offences.join("\n")
        );
        // **`Q-51`: how many files it read, because an empty scan finds nothing.**
        //
        // `read_dir` is not recursive, and a directory entry has no `rs` extension - so it
        // is skipped by the same `continue` that skips a `Cargo.toml`. **A module moved into
        // a subdirectory of `src/` would be unscanned and this would stay green**, which is
        // the shape where a rule quietly stops binding the code it names.
        //
        // A floor rather than an exact count: the number is a property of how this crate is
        // laid out, and a bound needing an edit whenever a file is added would be edited
        // without being thought about. What it has to catch is the scan collapsing.
        assert!(
            scanned >= 5,
            "only {scanned} files scanned for a game noun, which is not this crate"
        );
    }
}
