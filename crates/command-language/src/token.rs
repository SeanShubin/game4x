//! Splitting a line into words, each knowing where it came from.
//!
//! Positions are carried from here all the way to a failure message, which is the point:
//! the predecessor's cursor knew its index and threw it away, so no error could ever say
//! where it was.

use crate::failure::{Position, Span};

/// A word, and where it sat.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub text: String,
    pub span: Span,
}

impl Token {
    /// The whole number this word denotes, if it is one.
    ///
    /// Only digits count. A leading minus is not accepted: every quantity in this language
    /// is a count, a density or an identifier, and none of those is ever negative. A
    /// negative number is therefore a mistake worth reporting rather than a value to
    /// carry forward.
    pub fn as_number(&self) -> Option<i64> {
        if self.text.is_empty() || !self.text.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        self.text.parse().ok()
    }
}

/// The two characters that open and close a command.
///
/// **`P-321`: a command is written `{name field:value ...}`.** They are their own tokens
/// rather than characters stuck to the word beside them, so `{work` is never one word and a
/// failure can point at the brace itself.
pub const BRACES: [char; 2] = ['{', '}'];

/// The character that begins a comment, to the end of the line.
///
/// **Not in `spec/console.md`.** A command file that sets up twelve territories is
/// unreadable without a way to say what each one is for, so this is here to make the
/// setup file legible - but it is an addition to the language and wants a decision.
pub const COMMENT: char = '#';

/// Splits one line into words, discarding whitespace and any trailing comment.
///
/// Whitespace never reaches the grammar. Punctuation is the same kind of thing - it is
/// dropped by whatever binds syntax to meaning, not marked "insignificant" here - which is
/// the one structural idea worth keeping from the predecessor.
pub fn tokenize(line: &str, line_number: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut word = String::new();
    let mut began_at = 1usize;

    // Columns count characters rather than bytes, because that is what a person editing
    // the file is looking at.
    for (offset, character) in line.chars().chain(std::iter::once('\n')).enumerate() {
        let column = offset + 1;
        if character == COMMENT {
            break;
        }
        if character.is_whitespace() || BRACES.contains(&character) {
            if !word.is_empty() {
                tokens.push(Token {
                    span: Span::new(
                        Position::new(line_number, began_at),
                        Position::new(line_number, column),
                    ),
                    text: std::mem::take(&mut word),
                });
            }
            // A brace is a token of its own, so that a missing one is reported where it
            // should have been rather than as part of the word beside it.
            if BRACES.contains(&character) {
                tokens.push(Token {
                    span: Span::new(
                        Position::new(line_number, column),
                        Position::new(line_number, column + 1),
                    ),
                    text: character.to_string(),
                });
            }
        } else {
            if word.is_empty() {
                began_at = column;
            }
            word.push(character);
        }
    }

    if !word.is_empty() {
        let column = began_at + word.chars().count();
        tokens.push(Token {
            span: Span::new(
                Position::new(line_number, began_at),
                Position::new(line_number, column),
            ),
            text: word,
        });
    }
    tokens
}

/// Where the end of a line is, for reporting something missing from it.
pub fn end_of(tokens: &[Token], line_number: usize) -> Position {
    match tokens.last() {
        Some(last) => last.span.to,
        None => Position::new(line_number, 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(line: &str) -> Vec<String> {
        tokenize(line, 1).into_iter().map(|t| t.text).collect()
    }

    #[test]
    fn a_line_splits_into_words() {
        assert_eq!(
            words("{deploy-ark territory:1}"),
            ["{", "deploy-ark", "territory:1", "}"]
        );
    }

    /// A brace is a word of its own even with no space around it.
    ///
    /// **That is why it is a token rather than a character stuck to its neighbour.** Without
    /// it, `{deploy` is one word and no form's name ever matches, and the failure reported
    /// would be about a word nobody typed.
    #[test]
    fn a_brace_is_its_own_word_however_it_is_written() {
        assert_eq!(words("{start}"), ["{", "start", "}"]);
        assert_eq!(words("{ start }"), ["{", "start", "}"]);
        assert_eq!(words("{end-turn}"), ["{", "end-turn", "}"]);
    }

    #[test]
    fn runs_of_whitespace_are_one_separator() {
        assert_eq!(
            words("  build   extractor\t3  metal "),
            ["build", "extractor", "3", "metal"]
        );
    }

    #[test]
    fn an_empty_line_has_no_words() {
        assert!(words("").is_empty());
        assert!(words("   \t ").is_empty());
    }

    #[test]
    fn a_comment_runs_to_the_end_of_the_line() {
        assert_eq!(words("land ark 1 # the landing site"), ["land", "ark", "1"]);
        assert!(words("# nothing but a comment").is_empty());
    }

    /// Columns are what a failure message quotes, so they have to be right.
    #[test]
    fn every_word_knows_where_it_started() {
        let tokens = tokenize("{deploy-ark territory:12}", 4);
        let starts: Vec<(usize, usize)> = tokens
            .iter()
            .map(|t| (t.span.from.line, t.span.from.column))
            .collect();
        // `{` `deploy-ark` `territory:12` `}` - four words, and the brace is one of them.
        assert_eq!(starts, [(4, 1), (4, 2), (4, 13), (4, 25)]);
        assert_eq!(tokens[2].span.to.column, 25, "just past the last character");
    }

    #[test]
    fn a_word_of_digits_is_a_number() {
        let tokens = tokenize("7 07 x7 7x -3", 1);
        assert_eq!(tokens[0].as_number(), Some(7));
        assert_eq!(
            tokens[1].as_number(),
            Some(7),
            "leading zeroes are harmless"
        );
        assert_eq!(tokens[2].as_number(), None);
        assert_eq!(tokens[3].as_number(), None);
        assert_eq!(tokens[4].as_number(), None, "counts are never negative");
    }

    #[test]
    fn a_number_too_large_to_hold_is_not_a_number() {
        let tokens = tokenize("99999999999999999999999999", 1);
        assert_eq!(tokens[0].as_number(), None);
    }
}
