//! Where something is, and what went wrong there.
//!
//! Failure is **data**, in this layer and in every layer above it. Nothing here panics on
//! bad input and nothing returns an error by unwinding. The predecessor modelled failure
//! as a sealed type in the parser and as a thrown exception in the assembler immediately
//! above it, so the same pipeline reported the same kind of problem two different ways;
//! that is the mistake this file exists to avoid.
//!
//! A failure also carries **where** and **what was expected**. The predecessor's failure
//! was a bare object, and a message was recovered afterwards by diffing cursors - which
//! can say where parsing stopped but never what would have let it continue. For a language
//! a player types, "expected a number, found `orbit`, at line 3 column 12" is the whole
//! difference between usable and not.

use std::fmt;

/// A place in the source, counted the way a person reads it: lines and columns from one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// The position of the first character of the first line.
    pub const START: Self = Self { line: 1, column: 1 };
}

impl fmt::Display for Position {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "line {} column {}", self.line, self.column)
    }
}

/// A stretch of source, from where something starts to just past where it ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub from: Position,
    pub to: Position,
}

impl Span {
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    /// A span covering a single position, for things with no width.
    pub fn at(position: Position) -> Self {
        Self {
            from: position,
            to: position,
        }
    }
}

/// What the parser wanted, in the terms a reader would use.
///
/// Kept as a list because a failure at one position can have several ways forward: at the
/// start of a line every command is a candidate, and saying so is more useful than naming
/// whichever happened to be tried last.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub position: Position,
    /// Everything that would have been accepted here, in the order the grammar offers it.
    pub expected: Vec<String>,
    /// What was there instead, or `None` at the end of the input.
    pub found: Option<String>,
    /// Which source it came from, when there is more than one in play.
    pub source: Option<String>,
}

impl Failure {
    pub fn new(position: Position, expected: impl IntoIterator<Item = String>) -> Self {
        Self {
            position,
            expected: dedup_keeping_order(expected),
            found: None,
            source: None,
        }
    }

    pub fn found(mut self, what: impl Into<String>) -> Self {
        self.found = Some(what.into());
        self
    }

    pub fn in_source(mut self, name: impl Into<String>) -> Self {
        self.source = Some(name.into());
        self
    }

    /// Merges two failures, keeping whichever got further.
    ///
    /// Ordered choice tries alternatives in turn, and the useful report is the one that
    /// read the most before giving up - that is almost always the branch the writer meant.
    /// Where two get equally far, their expectations are both worth saying.
    pub fn or_further(self, other: Self) -> Self {
        match self.position.cmp(&other.position) {
            std::cmp::Ordering::Greater => self,
            std::cmp::Ordering::Less => other,
            std::cmp::Ordering::Equal => {
                let mut expected = self.expected;
                expected.extend(other.expected);
                Self {
                    position: self.position,
                    expected: dedup_keeping_order(expected),
                    found: self.found.or(other.found),
                    source: self.source.or(other.source),
                }
            }
        }
    }
}

fn dedup_keeping_order(items: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut kept: Vec<String> = Vec::new();
    for item in items {
        if !kept.contains(&item) {
            kept.push(item);
        }
    }
    kept
}

impl fmt::Display for Failure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.source {
            write!(out, "{source}, ")?;
        }
        write!(out, "{}: expected ", self.position)?;
        match self.expected.len() {
            0 => write!(out, "nothing")?,
            1 => write!(out, "{}", self.expected[0])?,
            _ => {
                let (last, rest) = self.expected.split_last().expect("checked not empty");
                write!(out, "{} or {last}", rest.join(", "))?;
            }
        }
        match &self.found {
            Some(found) => write!(out, ", found `{found}`"),
            None => write!(out, ", found end of line"),
        }
    }
}

impl std::error::Error for Failure {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failure_reads_as_a_sentence() {
        let failure = Failure::new(Position::new(3, 12), ["a number".to_string()]).found("orbit");
        assert_eq!(
            failure.to_string(),
            "line 3 column 12: expected a number, found `orbit`"
        );
    }

    #[test]
    fn several_expectations_are_listed() {
        let failure = Failure::new(
            Position::new(1, 1),
            ["land".to_string(), "move".to_string(), "build".to_string()],
        );
        assert_eq!(
            failure.to_string(),
            "line 1 column 1: expected land, move or build, found end of line"
        );
    }

    /// The report that helps is the one from the branch that read furthest, because that
    /// is nearly always the one the writer intended.
    #[test]
    fn merging_keeps_whichever_got_further() {
        let early = Failure::new(Position::new(1, 1), ["land".to_string()]);
        let late = Failure::new(Position::new(1, 9), ["a number".to_string()]);
        assert_eq!(early.clone().or_further(late.clone()), late);
        assert_eq!(late.clone().or_further(early), late);
    }

    #[test]
    fn merging_at_the_same_place_keeps_both_expectations() {
        let one = Failure::new(Position::new(1, 5), ["a number".to_string()]);
        let two = Failure::new(Position::new(1, 5), ["a name".to_string()]);
        let merged = one.or_further(two);
        assert_eq!(merged.expected, vec!["a number", "a name"]);
    }

    #[test]
    fn an_expectation_is_never_repeated() {
        let one = Failure::new(Position::new(1, 5), ["a number".to_string()]);
        let two = Failure::new(Position::new(1, 5), ["a number".to_string()]);
        assert_eq!(one.or_further(two).expected, vec!["a number"]);
    }

    #[test]
    fn a_source_is_named_when_there_is_one() {
        let failure = Failure::new(Position::new(2, 1), ["start".to_string()])
            .found("stop")
            .in_source("setup.cmd");
        assert!(
            failure
                .to_string()
                .starts_with("setup.cmd, line 2 column 1")
        );
    }
}
