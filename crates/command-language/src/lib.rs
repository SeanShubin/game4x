//! A command language: a grammar, a parser, and a typed syntax tree.
//!
//! **This crate contains no game nouns.** `land`, `territory` and `metal` never appear in
//! it. A grammar is data handed in from outside, and what the words mean is decided a
//! layer up. That seam is the one idea worth keeping from the predecessor reviewed in
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
//!     "land",
//!     vec![
//!         Term::Keyword("land"),
//!         Term::required("unit", Kind::Name),
//!         Term::required("territory", Kind::Number),
//!     ],
//!     "bring a unit down from orbit",
//! )]);
//!
//! let command = parse_line(&grammar, "land ark 1", 1).unwrap().unwrap();
//! assert_eq!(command.form, "land");
//! assert_eq!(command.name("unit").unwrap(), "ark");
//! assert_eq!(command.number("territory").unwrap(), 1);
//!
//! let failure = parse_line(&grammar, "land ark orbit", 1).unwrap_err();
//! assert_eq!(failure.to_string(), "line 1 column 10: expected a number, found `orbit`");
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
        for entry in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
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
    }
}
