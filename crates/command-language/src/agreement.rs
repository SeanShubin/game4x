//! Checking that the grammar and the handlers describe the same language.
//!
//! Two declarative tables sit either side of this crate's seam: a grammar naming forms,
//! and a binding table supplying a handler for each. Nothing makes them agree on its own.
//! In the predecessor a form with no handler was a runtime exception the first time
//! anybody typed that command - so the failure surfaced in front of a player, in a
//! program that had compiled and passed its tests.
//!
//! Checking it costs one test. What that test asserts is here rather than in the caller,
//! so every grammar gets the same check for free.

use crate::grammar::Grammar;

/// Everything wrong with a grammar and a set of handlers, taken together.
///
/// A list rather than the first problem found: if three commands are unhandled, hearing
/// about all three is worth more than hearing about one, three times.
pub fn disagreements(grammar: &Grammar, handled: &[&str]) -> Vec<String> {
    let mut problems = Vec::new();

    for name in grammar.duplicate_names() {
        problems.push(format!(
            "the grammar has more than one form named `{name}`, so a handler can only reach one of them"
        ));
    }
    for name in grammar.forms_with_repeated_holes() {
        problems.push(format!(
            "form `{name}` uses one hole name twice, so one would hide the other"
        ));
    }
    for name in grammar.form_names() {
        if !handled.contains(&name) {
            problems.push(format!(
                "form `{name}` is in the grammar with no handler, so typing it would fail at run time"
            ));
        }
    }
    for name in handled {
        if grammar.form(name).is_none() {
            problems.push(format!(
                "handler `{name}` has no form in the grammar, so it can never run"
            ));
        }
    }

    problems
}

/// Whether every form is handled and every handler is reachable.
pub fn agree(grammar: &Grammar, handled: &[&str]) -> bool {
    disagreements(grammar, handled).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Form, Kind, Term};

    fn grammar() -> Grammar {
        Grammar::new(vec![
            Form::new(
                "land",
                vec![Term::Keyword("land"), Term::required("unit", Kind::Name)],
                "land a unit",
            ),
            Form::new(
                "end-turn",
                vec![Term::Keyword("end"), Term::Keyword("turn")],
                "end the turn",
            ),
        ])
    }

    #[test]
    fn a_matching_pair_of_tables_agrees() {
        assert!(agree(&grammar(), &["land", "end-turn"]));
        assert!(disagreements(&grammar(), &["land", "end-turn"]).is_empty());
    }

    #[test]
    fn a_form_with_no_handler_is_reported() {
        let problems = disagreements(&grammar(), &["land"]);
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("end-turn"), "{problems:?}");
    }

    #[test]
    fn a_handler_with_no_form_is_reported() {
        let problems = disagreements(&grammar(), &["land", "end-turn", "fly"]);
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("fly"), "{problems:?}");
    }

    #[test]
    fn every_problem_is_reported_not_just_the_first() {
        let problems = disagreements(&grammar(), &["swim"]);
        assert_eq!(
            problems.len(),
            3,
            "two unhandled forms and one stray handler: {problems:?}"
        );
    }

    #[test]
    fn a_grammar_that_cannot_be_bound_is_reported_too() {
        let clashing = Grammar::new(vec![
            Form::new("show", vec![Term::Keyword("show")], "one"),
            Form::new("show", vec![Term::Keyword("show")], "another"),
        ]);
        let problems = disagreements(&clashing, &["show"]);
        assert!(
            problems.iter().any(|p| p.contains("more than one form")),
            "{problems:?}"
        );
    }
}
