//! The shapes a command may take.
//!
//! **No game nouns live here.** `land`, `territory` and `metal` are strings this file
//! happens to carry; nothing in this crate knows what any of them mean. That seam is the
//! one part of the predecessor's design worth keeping intact, and keeping it means the
//! grammar can be handed in from outside as data.
//!
//! A form is flat: keywords and holes, in order. That is enough for a language of one
//! command to a line, and choice between commands is choice between forms rather than a
//! construct inside one. If the language ever grows nesting or arithmetic, this is the
//! file that has to grow a real expression type, and the absence of left recursion will
//! have to be faced deliberately rather than inherited by accident.

use std::fmt;

/// What sort of word a hole accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A bare word: a unit's name, a resource, a subject.
    Name,
    /// A whole number: a count, a density, a territory's id.
    Number,
}

impl Kind {
    /// How to describe this in a failure, in the reader's terms rather than the parser's.
    pub fn describe(self) -> &'static str {
        match self {
            Kind::Name => "a name",
            Kind::Number => "a number",
        }
    }
}

/// One position in a form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    /// A fixed word that must appear exactly.
    Keyword(&'static str),
    /// A value the caller supplies, reached afterwards by this name and never by position.
    Hole {
        name: &'static str,
        kind: Kind,
        required: bool,
    },
}

impl Term {
    pub fn required(name: &'static str, kind: Kind) -> Self {
        Term::Hole {
            name,
            kind,
            required: true,
        }
    }

    pub fn optional(name: &'static str, kind: Kind) -> Self {
        Term::Hole {
            name,
            kind,
            required: false,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Keyword(word) => write!(out, "{word}"),
            Term::Hole {
                name,
                required: true,
                ..
            } => write!(out, "<{name}>"),
            Term::Hole {
                name,
                required: false,
                ..
            } => write!(out, "[<{name}>]"),
        }
    }
}

/// One command's shape.
#[derive(Clone, Debug)]
pub struct Form {
    /// How handlers refer to this form. Unique within a grammar.
    pub name: &'static str,
    pub terms: Vec<Term>,
    /// One line, for `help`.
    pub summary: &'static str,
}

impl Form {
    pub fn new(name: &'static str, terms: Vec<Term>, summary: &'static str) -> Self {
        Self {
            name,
            terms,
            summary,
        }
    }

    /// The form written out the way a player would type it.
    pub fn syntax(&self) -> String {
        self.terms
            .iter()
            .map(|term| term.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The words that must open this form, for reporting what was expected at the start
    /// of a line.
    pub fn opening(&self) -> String {
        match self.terms.first() {
            Some(Term::Keyword(word)) => (*word).to_string(),
            Some(Term::Hole { kind, .. }) => kind.describe().to_string(),
            None => "an empty command".to_string(),
        }
    }

    /// Every hole in this form, in order.
    pub fn holes(&self) -> impl Iterator<Item = (&'static str, Kind, bool)> + '_ {
        self.terms.iter().filter_map(|term| match term {
            Term::Keyword(_) => None,
            Term::Hole {
                name,
                kind,
                required,
            } => Some((*name, *kind, *required)),
        })
    }
}

/// Every command shape the language accepts, in the order they are tried.
///
/// The order matters and is not an accident of listing. Matching is first-wins, so a form
/// whose opening words are a prefix of another's must come after it - `end turn` before a
/// hypothetical bare `end`. That is ordered-choice behaviour, and it is written down here
/// because the predecessor relied on exactly this and never said so.
#[derive(Clone, Debug, Default)]
pub struct Grammar {
    forms: Vec<Form>,
}

impl Grammar {
    pub fn new(forms: Vec<Form>) -> Self {
        Self { forms }
    }

    pub fn forms(&self) -> &[Form] {
        &self.forms
    }

    pub fn form(&self, name: &str) -> Option<&Form> {
        self.forms.iter().find(|form| form.name == name)
    }

    /// The names of every form, which is what a binding table has to cover exactly.
    pub fn form_names(&self) -> Vec<&'static str> {
        self.forms.iter().map(|form| form.name).collect()
    }

    /// Every form whose first keyword is this word - what `help <command>` reports.
    pub fn forms_beginning(&self, word: &str) -> Vec<&Form> {
        self.forms
            .iter()
            .filter(
                |form| matches!(form.terms.first(), Some(Term::Keyword(first)) if *first == word),
            )
            .collect()
    }

    /// Fails if two forms share a name, since a binding table is keyed by name and could
    /// then only reach one of them.
    pub fn duplicate_names(&self) -> Vec<&'static str> {
        let mut seen: Vec<&'static str> = Vec::new();
        let mut duplicated: Vec<&'static str> = Vec::new();
        for form in &self.forms {
            if seen.contains(&form.name) {
                if !duplicated.contains(&form.name) {
                    duplicated.push(form.name);
                }
            } else {
                seen.push(form.name);
            }
        }
        duplicated
    }

    /// Fails if a form reuses a hole name, since arguments are reached by name and one
    /// would shadow the other.
    pub fn forms_with_repeated_holes(&self) -> Vec<&'static str> {
        self.forms
            .iter()
            .filter(|form| {
                let mut names: Vec<&str> = form.holes().map(|(name, _, _)| name).collect();
                let before = names.len();
                names.sort_unstable();
                names.dedup();
                names.len() != before
            })
            .map(|form| form.name)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn a_form_writes_itself_the_way_it_is_typed() {
        assert_eq!(
            grammar().form("land").unwrap().syntax(),
            "land <unit> <territory>"
        );
        assert_eq!(
            grammar().form("build").unwrap().syntax(),
            "build <structure> <territory> [<resource>]"
        );
        assert_eq!(grammar().form("end-turn").unwrap().syntax(), "end turn");
    }

    #[test]
    fn holes_are_listed_in_order_with_their_kinds() {
        let form = grammar();
        let form = form.form("build").unwrap();
        let holes: Vec<_> = form.holes().collect();
        assert_eq!(
            holes,
            [
                ("structure", Kind::Name, true),
                ("territory", Kind::Number, true),
                ("resource", Kind::Name, false),
            ]
        );
    }

    #[test]
    fn forms_can_be_found_by_the_word_that_opens_them() {
        assert_eq!(grammar().forms_beginning("land").len(), 1);
        assert_eq!(grammar().forms_beginning("end").len(), 1);
        assert!(grammar().forms_beginning("fly").is_empty());
    }

    /// A binding table is keyed by form name, so two forms sharing one would leave a
    /// command unreachable rather than merely ambiguous.
    #[test]
    fn duplicate_form_names_are_reported() {
        let clashing = Grammar::new(vec![
            Form::new("show", vec![Term::Keyword("show")], "one"),
            Form::new("show", vec![Term::Keyword("show")], "another"),
        ]);
        assert_eq!(clashing.duplicate_names(), ["show"]);
        assert!(grammar().duplicate_names().is_empty());
    }

    /// Arguments are reached by name, so a form using one twice would hide the first.
    #[test]
    fn a_form_may_not_use_one_hole_name_twice() {
        let clashing = Grammar::new(vec![Form::new(
            "move",
            vec![
                Term::required("territory", Kind::Number),
                Term::required("territory", Kind::Number),
            ],
            "two of the same",
        )]);
        assert_eq!(clashing.forms_with_repeated_holes(), ["move"]);
        assert!(grammar().forms_with_repeated_holes().is_empty());
    }
}
