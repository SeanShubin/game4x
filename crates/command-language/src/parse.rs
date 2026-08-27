//! Matching words against forms.
//!
//! Forms are tried in the order the grammar lists them and the first that matches wins,
//! which is ordered choice. When none matches, the failure reported is the one from
//! whichever form read furthest before giving up - almost always the one the writer meant.

use std::collections::BTreeMap;

use crate::failure::{Failure, Position, Span};
use crate::grammar::{Form, Grammar, Kind, Term};
use crate::syntax::{Argument, Utterance};
use crate::token::{Token, end_of, tokenize};

/// Parses one line.
///
/// A line with nothing on it but whitespace or a comment is not a command and not an
/// error, so it yields `None` rather than either.
pub fn parse_line(
    grammar: &Grammar,
    line: &str,
    line_number: usize,
) -> Result<Option<Utterance>, Failure> {
    let tokens = tokenize(line, line_number);
    if tokens.is_empty() {
        return Ok(None);
    }

    let mut worst: Option<Failure> = None;
    for form in grammar.forms() {
        match match_form(form, &tokens, line_number, line) {
            Ok(utterance) => return Ok(Some(utterance)),
            Err(failure) => {
                worst = Some(match worst {
                    Some(previous) => previous.or_further(failure),
                    None => failure,
                });
            }
        }
    }

    Err(worst.unwrap_or_else(|| {
        // A grammar with no forms at all accepts nothing, and should say so rather than
        // reporting an empty expectation.
        Failure::new(tokens[0].span.from, ["a command".to_string()]).found(tokens[0].text.clone())
    }))
}

/// Parses every line of a script, stopping at the first that fails.
///
/// Stopping is deliberate. Commands are applied in order and each one's meaning depends
/// on the state the ones before it produced, so continuing past a failure would report
/// problems that only exist because of the first one.
pub fn parse_script(grammar: &Grammar, text: &str) -> Result<Vec<Utterance>, Failure> {
    let mut utterances = Vec::new();
    for (offset, line) in text.lines().enumerate() {
        if let Some(utterance) = parse_line(grammar, line, offset + 1)? {
            utterances.push(utterance);
        }
    }
    Ok(utterances)
}

fn match_form(
    form: &Form,
    tokens: &[Token],
    line_number: usize,
    line: &str,
) -> Result<Utterance, Failure> {
    let mut arguments: BTreeMap<&'static str, Argument> = BTreeMap::new();
    let mut at = 0usize;

    for term in &form.terms {
        match term {
            Term::Keyword(word) => {
                let token = tokens.get(at).ok_or_else(|| {
                    Failure::new(end_of(tokens, line_number), [(*word).to_string()])
                })?;
                if token.text != *word {
                    return Err(Failure::new(token.span.from, [(*word).to_string()])
                        .found(token.text.clone()));
                }
                at += 1;
            }
            Term::Hole {
                name,
                kind,
                required,
            } => match tokens.get(at) {
                Some(token) => match read(token, *kind) {
                    Some(argument) => {
                        arguments.insert(name, argument);
                        at += 1;
                    }
                    None if *required => {
                        return Err(Failure::new(token.span.from, [kind.describe().to_string()])
                            .found(token.text.clone()));
                    }
                    // An optional hole that does not fit simply is not there; whatever is
                    // here belongs to a later term, or is surplus and reported below.
                    None => {}
                },
                None if *required => {
                    return Err(Failure::new(
                        end_of(tokens, line_number),
                        [kind.describe().to_string()],
                    ));
                }
                None => {}
            },
        }
    }

    if at < tokens.len() {
        let surplus = &tokens[at];
        return Err(Failure::new(surplus.span.from, ["end of line".to_string()])
            .found(surplus.text.clone()));
    }

    let span = Span::new(
        tokens
            .first()
            .map(|first| first.span.from)
            .unwrap_or(Position::new(line_number, 1)),
        end_of(tokens, line_number),
    );
    Ok(Utterance::new(
        form.name,
        span,
        line.trim().to_string(),
        arguments,
    ))
}

fn read(token: &Token, kind: Kind) -> Option<Argument> {
    match kind {
        Kind::Number => token
            .as_number()
            .map(|value| Argument::Number(value, token.span)),
        // A number is a perfectly good name where a name is wanted - a territory can be
        // called `5`. Rejecting it here would make `show 5` unparseable for no gain.
        Kind::Name => Some(Argument::Name(token.text.clone(), token.span)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::Form;

    fn grammar() -> Grammar {
        Grammar::new(vec![
            Form::new(
                "land",
                vec![
                    Term::Keyword("land"),
                    Term::required("unit", Kind::Name),
                    Term::required("territory", Kind::Number),
                ],
                "bring a unit down from orbit",
            ),
            Form::new(
                "build",
                vec![
                    Term::Keyword("build"),
                    Term::required("structure", Kind::Name),
                    Term::required("territory", Kind::Number),
                    Term::optional("resource", Kind::Name),
                ],
                "build a structure",
            ),
            Form::new(
                "end-turn",
                vec![Term::Keyword("end"), Term::Keyword("turn")],
                "end the turn",
            ),
        ])
    }

    fn parse(line: &str) -> Result<Option<Utterance>, Failure> {
        parse_line(&grammar(), line, 1)
    }

    #[test]
    fn a_command_parses_into_named_arguments() {
        let utterance = parse("land ark 1").unwrap().unwrap();
        assert_eq!(utterance.form, "land");
        assert_eq!(utterance.name("unit").unwrap(), "ark");
        assert_eq!(utterance.number("territory").unwrap(), 1);
    }

    #[test]
    fn an_optional_argument_may_be_left_out_or_supplied() {
        let without = parse("build garrison 3").unwrap().unwrap();
        assert_eq!(without.optional_name("resource"), None);
        let with = parse("build extractor 3 metal").unwrap().unwrap();
        assert_eq!(with.optional_name("resource"), Some("metal"));
    }

    #[test]
    fn a_form_may_be_all_keywords() {
        assert_eq!(parse("end turn").unwrap().unwrap().form, "end-turn");
    }

    #[test]
    fn blank_and_comment_lines_are_neither_commands_nor_errors() {
        assert_eq!(parse("").unwrap(), None);
        assert_eq!(parse("    ").unwrap(), None);
        assert_eq!(parse("# just a note").unwrap(), None);
    }

    /// The whole point of carrying positions: a failure says where and what was wanted.
    #[test]
    fn a_wrong_argument_says_where_it_is_and_what_was_expected() {
        let failure = parse("land ark orbit").unwrap_err();
        assert_eq!(failure.position, Position::new(1, 10));
        assert!(
            failure.expected.contains(&"a number".to_string()),
            "{failure}"
        );
        assert_eq!(failure.found.as_deref(), Some("orbit"));
    }

    #[test]
    fn a_missing_argument_is_reported_at_the_end_of_the_line() {
        let failure = parse("land ark").unwrap_err();
        assert!(
            failure.expected.contains(&"a number".to_string()),
            "{failure}"
        );
        assert_eq!(failure.found, None, "there is nothing there to quote");
    }

    #[test]
    fn a_surplus_word_is_reported_rather_than_ignored() {
        let failure = parse("end turn now").unwrap_err();
        assert!(
            failure.expected.contains(&"end of line".to_string()),
            "{failure}"
        );
        assert_eq!(failure.found.as_deref(), Some("now"));
    }

    /// An unknown verb cannot match any form, so the report is what could have opened a
    /// line - which is the useful thing to say at column one.
    #[test]
    fn an_unknown_command_lists_what_could_have_been_written() {
        let failure = parse("fly ark 1").unwrap_err();
        assert_eq!(failure.position, Position::new(1, 1));
        for expected in ["land", "build", "end"] {
            assert!(
                failure.expected.contains(&expected.to_string()),
                "{failure} should offer {expected}"
            );
        }
    }

    /// Ordered choice: the report comes from the branch that read furthest, so a typo in
    /// an argument is reported there rather than at the start of the line.
    #[test]
    fn the_report_comes_from_whichever_form_read_furthest() {
        let failure = parse("build extractor three").unwrap_err();
        assert_eq!(failure.position, Position::new(1, 17));
        assert!(
            failure.expected.contains(&"a number".to_string()),
            "{failure}"
        );
    }

    #[test]
    fn a_script_parses_every_line_in_order() {
        let script = "land ark 1\n\n# a note\nend turn\n";
        let commands = parse_script(&grammar(), script).unwrap();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].form, "land");
        assert_eq!(commands[1].form, "end-turn");
    }

    #[test]
    fn a_script_stops_at_the_first_line_that_fails_and_says_which() {
        let script = "land ark 1\nland ark orbit\nend turn\n";
        let failure = parse_script(&grammar(), script).unwrap_err();
        assert_eq!(failure.position.line, 2);
    }

    #[test]
    fn a_command_remembers_how_it_was_written() {
        let utterance = parse("  land   ark 1  ").unwrap().unwrap();
        assert_eq!(utterance.source, "land   ark 1");
    }
}
