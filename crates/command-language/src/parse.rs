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

/// A command is `{name field:value ...}`, so a form is its name words and its named fields.
///
/// **`P-321`, `P-323` and `P-328`.** `spec/console.md`:
///
/// > A command is written `{name field:value ...}`. **Its name is one word**, dashed where
/// > it needs more, and its arguments are named.
///
/// The predecessor read a form as a flat sequence - keywords and holes, in order - so
/// `land ark 1` bound `1` to `territory` by counting, and the same three words in another
/// order meant nothing.
///
/// **The name is one token and the fields carry their own names**, so
/// `{build-extractor territory:1 resource:metal}` and the same two fields swapped are one
/// command. A form is still a list of keywords followed by holes, and every form in the
/// console's grammar now has exactly one keyword.
///
/// **A `Term::Keyword` is a word of the name and a `Term::Hole` is a field.** That is a
/// reinterpretation of the existing grammar rather than a new one, which keeps every failure
/// this crate already reports - the position, the expectation, and the command a rejection was
/// found inside.
fn match_form(
    form: &Form,
    tokens: &[Token],
    line_number: usize,
    line: &str,
) -> Result<Utterance, Failure> {
    let mut at = 0usize;

    let open = tokens
        .get(at)
        .ok_or_else(|| Failure::new(end_of(tokens, line_number), ["{".to_string()]))?;
    if open.text != "{" {
        return Err(Failure::new(open.span.from, ["{".to_string()]).found(open.text.clone()));
    }
    at += 1;

    // The name: every keyword of the form, in order, before any field.
    for term in &form.terms {
        let Term::Keyword(word) = term else { break };
        let token = tokens
            .get(at)
            .ok_or_else(|| Failure::new(end_of(tokens, line_number), [(*word).to_string()]))?;
        if token.text != *word {
            return Err(
                Failure::new(token.span.from, [(*word).to_string()]).found(token.text.clone())
            );
        }
        at += 1;
    }

    // The fields, in whatever order they were written.
    let mut arguments: BTreeMap<&'static str, Argument> = BTreeMap::new();
    while let Some(token) = tokens.get(at) {
        if token.text == "}" {
            break;
        }
        let Some((name, value)) = token.text.split_once(':') else {
            return Err(
                Failure::new(token.span.from, ["a field, written name:value".to_string()])
                    .found(token.text.clone()),
            );
        };
        let Some(term) = form.terms.iter().find_map(|term| match term {
            Term::Hole {
                name: held, kind, ..
            } if *held == name => Some((*held, *kind)),
            _ => None,
        }) else {
            // **Named rather than counted**, which is the whole gain of the form: a field the
            // command does not take says so, where a surplus word could only say *end of
            // line*.
            let taken: Vec<String> = form
                .terms
                .iter()
                .filter_map(|term| match term {
                    Term::Hole { name, .. } => Some(format!("{name}:")),
                    Term::Keyword(_) => None,
                })
                .collect();
            let expected = if taken.is_empty() {
                vec!["no fields at all".to_string()]
            } else {
                taken
            };
            return Err(Failure::new(token.span.from, expected).found(format!("{name}:")));
        };
        let (held, kind) = term;
        // The value is read where it sits, so a failure points at the value rather than at
        // the field that carried it.
        let value_at = Span::new(
            Position::new(
                token.span.from.line,
                token.span.from.column + name.chars().count() + 1,
            ),
            token.span.to,
        );
        let read_as = Token {
            text: value.to_string(),
            span: value_at,
        };
        let Some(argument) = read(&read_as, kind) else {
            return Err(
                Failure::new(value_at.from, [kind.describe().to_string()]).found(value.to_string())
            );
        };
        if arguments.insert(held, argument).is_some() {
            return Err(
                Failure::new(token.span.from, ["a field not already given".to_string()])
                    .found(format!("{name}: twice")),
            );
        }
        at += 1;
    }

    for term in &form.terms {
        if let Term::Hole {
            name,
            kind,
            required: true,
        } = term
            && !arguments.contains_key(name)
        {
            return Err(Failure::new(
                end_of(tokens, line_number),
                [format!("{name}:<{}>", kind.describe())],
            ));
        }
    }

    let close = tokens
        .get(at)
        .ok_or_else(|| Failure::new(end_of(tokens, line_number), ["}".to_string()]))?;
    if close.text != "}" {
        return Err(Failure::new(close.span.from, ["}".to_string()]).found(close.text.clone()));
    }
    at += 1;

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
                    Term::Keyword("deploy-ark"),
                    Term::required("territory", Kind::Number),
                ],
                "bring an ark down from orbit",
            ),
            Form::new(
                "build",
                vec![
                    Term::Keyword("build-extractor"),
                    Term::required("territory", Kind::Number),
                    Term::optional("resource", Kind::Name),
                ],
                "build an extractor",
            ),
            Form::new("end-turn", vec![Term::Keyword("end-turn")], "end the turn"),
        ])
    }

    fn parse(line: &str) -> Result<Option<Utterance>, Failure> {
        parse_line(&grammar(), line, 1)
    }

    #[test]
    fn a_command_parses_into_named_arguments() {
        let utterance = parse("{deploy-ark territory:1}").unwrap().unwrap();
        assert_eq!(utterance.form, "land");
        assert_eq!(utterance.number("territory").unwrap(), 1);
        // **`ark` is a word of the name and not an argument**, so asking for it as one says
        // so rather than handing back the word that happened to sit there - `P-323`.
        assert!(utterance.optional_name("unit").is_none());
    }

    #[test]
    fn an_optional_argument_may_be_left_out_or_supplied() {
        let without = parse("{build-extractor territory:3}").unwrap().unwrap();
        assert_eq!(without.optional_name("resource"), None);
        let with = parse("{build-extractor territory:3 resource:metal}")
            .unwrap()
            .unwrap();
        assert_eq!(with.optional_name("resource"), Some("metal"));
    }

    #[test]
    fn a_form_may_be_all_keywords() {
        assert_eq!(parse("{end-turn}").unwrap().unwrap().form, "end-turn");
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
        let failure = parse("{deploy-ark territory:orbit}").unwrap_err();
        // **At the value and not at the field that carried it.** `territory:` opens at
        // column 13 and `orbit` at column 23, and the one a reader has to change is the
        // value - so the position points past the name rather than at the start of the pair.
        assert_eq!(failure.position, Position::new(1, 23));
        assert!(
            failure.expected.contains(&"a number".to_string()),
            "{failure}"
        );
        assert_eq!(failure.found.as_deref(), Some("orbit"));
    }

    #[test]
    fn a_missing_argument_is_reported_at_the_end_of_the_line() {
        let failure = parse("{deploy-ark}").unwrap_err();
        // **The field is named, which a positional grammar could not do.** It could say a
        // number was wanted and never which of the numbers, because the thing missing had no
        // name until `P-321` gave every argument one.
        assert!(
            failure
                .expected
                .contains(&"territory:<a number>".to_string()),
            "{failure}"
        );
        assert_eq!(failure.found, None, "there is nothing there to quote");
    }

    #[test]
    fn a_surplus_word_is_reported_rather_than_ignored() {
        let failure = parse("{end-turn} now").unwrap_err();
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
        let failure = parse("{fly ark territory:1}").unwrap_err();
        // Column 2, which is the first word of the name: column 1 is the brace, and every
        // form got that far.
        assert_eq!(failure.position, Position::new(1, 2));
        for expected in ["deploy-ark", "build-extractor", "end-turn"] {
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
        let failure = parse("{build-extractor territory:three}").unwrap_err();
        assert_eq!(failure.position, Position::new(1, 28));
        assert!(
            failure.expected.contains(&"a number".to_string()),
            "{failure}"
        );
    }

    #[test]
    fn a_script_parses_every_line_in_order() {
        let script = "{deploy-ark territory:1}\n\n# a note\n{end-turn}\n";
        let commands = parse_script(&grammar(), script).unwrap();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].form, "land");
        assert_eq!(commands[1].form, "end-turn");
    }

    #[test]
    fn a_script_stops_at_the_first_line_that_fails_and_says_which() {
        let script = "{deploy-ark territory:1}\n{deploy-ark territory:orbit}\n{end-turn}\n";
        let failure = parse_script(&grammar(), script).unwrap_err();
        assert_eq!(failure.position.line, 2);
    }

    #[test]
    fn a_command_remembers_how_it_was_written() {
        let utterance = parse("  {deploy-ark territory:1}  ").unwrap().unwrap();
        assert_eq!(utterance.source, "{deploy-ark territory:1}");
    }
}
