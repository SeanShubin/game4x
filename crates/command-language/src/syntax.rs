//! What a parsed command is, and how a handler reads it.
//!
//! An utterance is **typed** and its parts are reached **by name**. Both are deliberate.
//! The predecessor handed handlers a `List<Any>` and had them index it - `parts[0] as
//! String` - so the real contract was the arity and order of a grammar rule, written down
//! nowhere, and inserting a term silently shifted every index after it. Here a handler
//! asks for `territory`, and if the grammar stopped providing one it says so instead of
//! quietly reading the wrong thing.

use std::collections::BTreeMap;

use crate::failure::{Failure, Span};
use crate::grammar::Kind;

/// A value supplied at one hole.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Argument {
    Name(String, Span),
    Number(i64, Span),
}

impl Argument {
    pub fn span(&self) -> Span {
        match self {
            Argument::Name(_, span) | Argument::Number(_, span) => *span,
        }
    }

    pub fn kind(&self) -> Kind {
        match self {
            Argument::Name(..) => Kind::Name,
            Argument::Number(..) => Kind::Number,
        }
    }

    /// How this looks written down, for reporting it back.
    pub fn text(&self) -> String {
        match self {
            Argument::Name(name, _) => name.clone(),
            Argument::Number(value, _) => value.to_string(),
        }
    }
}

/// One command, parsed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Utterance {
    /// Which form matched. A binding table is keyed by this.
    pub form: &'static str,
    /// Where the whole command sat, for reporting a rule failure against it.
    pub span: Span,
    /// The words as typed, so `history` can repeat a command exactly.
    pub source: String,
    arguments: BTreeMap<&'static str, Argument>,
}

impl Utterance {
    pub fn new(
        form: &'static str,
        span: Span,
        source: String,
        arguments: BTreeMap<&'static str, Argument>,
    ) -> Self {
        Self {
            form,
            span,
            source,
            arguments,
        }
    }

    pub fn argument(&self, name: &str) -> Option<&Argument> {
        self.arguments.get(name)
    }

    pub fn arguments(&self) -> impl Iterator<Item = (&&'static str, &Argument)> {
        self.arguments.iter()
    }

    /// The name at this hole.
    ///
    /// Returning a failure rather than panicking keeps the rule that failure is data at
    /// every layer. Asking for a hole the form does not have is a mistake in the binding
    /// table rather than in what the player typed, and
    /// [`crate::agreement`] is what stops it reaching a player at all.
    pub fn name(&self, hole: &str) -> Result<&str, Failure> {
        match self.arguments.get(hole) {
            Some(Argument::Name(name, _)) => Ok(name),
            Some(other) => Err(self.wrong_kind(hole, Kind::Name, other)),
            None => Err(self.missing(hole)),
        }
    }

    /// The number at this hole.
    pub fn number(&self, hole: &str) -> Result<i64, Failure> {
        match self.arguments.get(hole) {
            Some(Argument::Number(value, _)) => Ok(*value),
            Some(other) => Err(self.wrong_kind(hole, Kind::Number, other)),
            None => Err(self.missing(hole)),
        }
    }

    /// The name at an optional hole, if it was supplied.
    pub fn optional_name(&self, hole: &str) -> Option<&str> {
        match self.arguments.get(hole) {
            Some(Argument::Name(name, _)) => Some(name),
            _ => None,
        }
    }

    /// The number at an optional hole, if it was supplied.
    pub fn optional_number(&self, hole: &str) -> Option<i64> {
        match self.arguments.get(hole) {
            Some(Argument::Number(value, _)) => Some(*value),
            _ => None,
        }
    }

    fn missing(&self, hole: &str) -> Failure {
        Failure::new(
            self.span.from,
            [format!("`{}` to supply `{hole}`", self.form)],
        )
        .found(self.source.clone())
    }

    fn wrong_kind(&self, hole: &str, wanted: Kind, found: &Argument) -> Failure {
        Failure::new(
            found.span().from,
            [format!("{} for `{hole}`", wanted.describe())],
        )
        .found(found.text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::failure::{Position, Span};

    fn span() -> Span {
        Span::at(Position::START)
    }

    fn utterance() -> Utterance {
        let mut arguments = BTreeMap::new();
        arguments.insert("unit", Argument::Name("ark".to_string(), span()));
        arguments.insert("territory", Argument::Number(1, span()));
        Utterance::new("land", span(), "land ark 1".to_string(), arguments)
    }

    #[test]
    fn arguments_are_reached_by_name() {
        assert_eq!(utterance().name("unit").unwrap(), "ark");
        assert_eq!(utterance().number("territory").unwrap(), 1);
    }

    /// The predecessor's failure mode: asking for something the form does not carry gave
    /// a cast exception naming no rule. Here it is a failure that names the hole.
    #[test]
    fn asking_for_a_hole_that_is_not_there_is_a_failure_not_a_panic() {
        let failure = utterance().name("resource").unwrap_err();
        assert!(failure.to_string().contains("resource"), "{failure}");
    }

    #[test]
    fn asking_for_the_wrong_kind_is_a_failure_that_says_which() {
        let failure = utterance().number("unit").unwrap_err();
        assert!(failure.to_string().contains("a number"), "{failure}");
        assert!(failure.to_string().contains("ark"), "{failure}");
    }

    #[test]
    fn an_absent_optional_hole_is_simply_absent() {
        assert_eq!(utterance().optional_name("resource"), None);
        assert_eq!(utterance().optional_name("unit"), Some("ark"));
        assert_eq!(utterance().optional_number("territory"), Some(1));
    }
}
