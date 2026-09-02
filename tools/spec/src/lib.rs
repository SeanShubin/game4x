//! Editing the specification, as named operations rather than as arbitrary text substitution.
//!
//! Every operation here exists because a hand-written edit did the wrong thing on 2026-09-01 and
//! the check that was supposed to catch it asked the wrong question. The design is in
//! `docs/notes/tools-spec-design.md`; the failures it answers are in
//! `docs/notes/how-this-lane-fails.md`.
//!
//! The point is not that these operations are safer to run. It is that **the unsafe edit is not
//! expressible**. A cell cannot be addressed by its position, so it cannot be written to the
//! neighbouring column; a replacement that matches nothing is an error rather than a no-op.

use std::fmt;

/// What went wrong, in words a person can act on rather than a code.
#[derive(Debug, PartialEq, Eq)]
pub struct Problem(pub String);

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn problem<T>(message: impl Into<String>) -> Result<T, Problem> {
    Err(Problem(message.into()))
}

/// A markdown document being edited. Holds its text and nothing else.
pub struct Doc {
    text: String,
}

/// One table row, split into cells, without the empty strings either side of the outer bars.
fn cells(line: &str) -> Vec<&str> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return Vec::new();
    }
    parts[1..parts.len() - 1].to_vec()
}

fn is_separator(line: &str) -> bool {
    let inner = cells(line);
    !inner.is_empty()
        && inner
            .iter()
            .all(|c| !c.trim().is_empty() && c.trim().chars().all(|ch| ch == '-'))
}

impl Doc {
    pub fn new(text: impl Into<String>) -> Doc {
        Doc { text: text.into() }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// The half-open line range of one `## ` section, ending at the next one.
    ///
    /// Two tables under two headings is the shape that made a slice span both on 2026-09-01, so
    /// this ends at the next heading rather than at a later anchor somebody chose.
    pub fn section(&self, heading: &str) -> Result<(usize, usize), Problem> {
        let lines: Vec<&str> = self.text.lines().collect();
        let starts: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.trim_end() == heading)
            .map(|(i, _)| i)
            .collect();
        match starts.len() {
            0 => problem(format!("no section {heading:?}")),
            1 => {
                let start = starts[0];
                let end = lines[start + 1..]
                    .iter()
                    .position(|l| l.starts_with("## "))
                    .map(|offset| start + 1 + offset)
                    .unwrap_or(lines.len());
                Ok((start, end))
            }
            n => problem(format!(
                "{n} sections named {heading:?}; the name does not identify one"
            )),
        }
    }

    /// Replace text that appears exactly once. Zero matches is an error, not a silent no-op -
    /// which is how a paragraph came to be inserted twice.
    pub fn replace_once(&mut self, old: &str, new: &str) -> Result<(), Problem> {
        if old.lines().any(|l| l.trim_start().starts_with('|')) {
            return problem("the text to replace holds a table row; address the row instead");
        }
        match self.text.matches(old).count() {
            1 => {
                self.text = self.text.replacen(old, new, 1);
                Ok(())
            }
            n => problem(format!("{n} matches for {:?}, expected one", brief(old))),
        }
    }

    /// Rebuild one row, found by a prefix that stays inside the first column.
    ///
    /// A prefix reaching past a bar breaks the moment `pad-tables` moves the column, and the
    /// failure is a silent no-match rather than an error - so the prefix is refused instead.
    pub fn set_row(&mut self, prefix: &str, row: &str) -> Result<(), Problem> {
        if prefix.matches('|').count() > 1 {
            return problem("the prefix crosses a column boundary; padding will move it");
        }
        let mut lines: Vec<String> = self.text.lines().map(str::to_string).collect();
        let hits: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.starts_with(prefix))
            .map(|(i, _)| i)
            .collect();
        match hits.len() {
            1 => {
                lines[hits[0]] = row.to_string();
                self.text = rejoin(&lines, &self.text);
                Ok(())
            }
            n => problem(format!("{n} rows start {:?}, expected one", brief(prefix))),
        }
    }

    /// Set one cell, addressing the column **by its heading**.
    ///
    /// The column cannot be given as a number, because on 2026-09-01 a number was one out and the
    /// value landed in the column beside the one meant. `expect` is what the cell must already
    /// say: an edit that finds something else has misunderstood the row and stops.
    pub fn set_cell(
        &mut self,
        heading: &str,
        row_prefix: &str,
        column: &str,
        value: &str,
        expect: Option<&str>,
    ) -> Result<(), Problem> {
        let (start, end) = self.section(heading)?;
        let mut lines: Vec<String> = self.text.lines().map(str::to_string).collect();
        let mut header: Option<Vec<String>> = None;
        for index in start..end {
            let line = lines[index].clone();
            if !line.starts_with('|') {
                continue;
            }
            let names: Vec<String> = cells(&line).iter().map(|c| c.trim().to_string()).collect();
            if header.is_none() {
                if !names.iter().any(|n| n == column) {
                    return problem(format!(
                        "no column {column:?} in {heading}; it has {names:?}"
                    ));
                }
                header = Some(names);
                continue;
            }
            if !line.starts_with(row_prefix) || is_separator(&line) {
                continue;
            }
            let at = header
                .as_ref()
                .unwrap()
                .iter()
                .position(|n| n == column)
                .unwrap();
            let mut row = cells(&line)
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>();
            if let Some(want) = expect {
                let found = row[at].trim();
                if found != want {
                    return problem(format!("{column} says {found:?}, expected {want:?}"));
                }
            }
            row[at] = format!(" {value} ");
            lines[index] = format!("|{}|", row.join("|"));
            self.text = rejoin(&lines, &self.text);
            return Ok(());
        }
        problem(format!(
            "no row starting {:?} in {heading}",
            brief(row_prefix)
        ))
    }

    /// Everything that must hold of the document however it was edited.
    pub fn check(&self) -> Result<(), Problem> {
        self.tables_are_square()?;
        self.no_paragraph_twice()
    }

    fn tables_are_square(&self) -> Result<(), Problem> {
        let mut width: Option<usize> = None;
        for line in self.text.lines() {
            if !line.starts_with('|') {
                width = None;
                continue;
            }
            let n = cells(line).len();
            match width {
                None => width = Some(n),
                Some(w) if w != n => {
                    return problem(format!(
                        "a row has {n} cells where its table has {w}: {}",
                        brief(line)
                    ));
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// A paragraph appearing twice is how an insert that ran a second time went unnoticed.
    fn no_paragraph_twice(&self) -> Result<(), Problem> {
        let mut seen: Vec<&str> = Vec::new();
        for para in self.text.split("\n\n") {
            let p = para.trim();
            if p.is_empty() || p.starts_with('|') || p.starts_with('#') || p.len() < 40 {
                continue;
            }
            if seen.contains(&p) {
                return problem(format!("this paragraph appears twice: {}", brief(p)));
            }
            seen.push(p);
        }
        Ok(())
    }
}

fn rejoin(lines: &[String], original: &str) -> String {
    let mut out = lines.join("\n");
    if original.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn brief(s: &str) -> String {
    let one_line: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if one_line.chars().count() <= 60 {
        one_line
    } else {
        format!("{}...", one_line.chars().take(57).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every test here is a defect that happened, named by what it did.
    const TRAITS: &str = "\
## Traits

| Trait | Of | Values |
| --- | --- | --- |
| **unpaid** | a unit | yes or no |

## What a territory has room for

| Kind | Room |
| --- | --- |
| **citizen** | 8 |
";

    #[test]
    fn a_column_cannot_be_addressed_by_position_so_the_wrong_one_cannot_be_written() {
        let mut doc = Doc::new(TRAITS);
        doc.set_cell(
            "## Traits",
            "| **unpaid**",
            "Of",
            "a thing with upkeep",
            Some("a unit"),
        )
        .unwrap();
        assert!(
            doc.text()
                .contains("| **unpaid** | a thing with upkeep | yes or no |")
        );
        assert!(!doc.text().contains("| a unit | a thing with upkeep |"));
    }

    #[test]
    fn a_cell_that_does_not_say_what_was_expected_stops_the_edit() {
        let mut doc = Doc::new(TRAITS);
        let e = doc
            .set_cell("## Traits", "| **unpaid**", "Of", "x", Some("a thing"))
            .unwrap_err();
        assert!(e.0.contains("expected"), "{e}");
    }

    #[test]
    fn a_section_ends_at_the_next_heading_rather_than_spanning_two_tables() {
        let doc = Doc::new(TRAITS);
        let (start, end) = doc.section("## Traits").unwrap();
        let all: Vec<&str> = doc.text().lines().collect();
        let inside = &all[start..end];
        assert!(inside.iter().any(|l| l.starts_with("| **unpaid**")));
        assert!(!inside.iter().any(|l| l.starts_with("| **citizen**")));
    }

    #[test]
    fn a_replacement_that_matches_nothing_is_an_error_and_not_a_no_op() {
        let mut doc = Doc::new("a sentence.\n");
        let e = doc.replace_once("not here", "x").unwrap_err();
        assert!(e.0.starts_with("0 matches"), "{e}");
    }

    #[test]
    fn a_replacement_that_matches_twice_refuses_to_guess() {
        let mut doc = Doc::new("same.\n\nsame.\n");
        assert!(doc.replace_once("same.", "x").is_err());
    }

    #[test]
    fn text_to_replace_may_not_hold_a_table_row() {
        let mut doc = Doc::new(TRAITS);
        let e = doc
            .replace_once("| **unpaid** | a unit | yes or no |", "x")
            .unwrap_err();
        assert!(e.0.contains("table row"), "{e}");
    }

    #[test]
    fn a_row_prefix_may_not_cross_a_column_boundary() {
        let mut doc = Doc::new(TRAITS);
        let e = doc.set_row("| **unpaid** | a unit", "x").unwrap_err();
        assert!(e.0.contains("column boundary"), "{e}");
    }

    #[test]
    fn a_ragged_row_is_caught_however_it_got_there() {
        let doc = Doc::new("| a | b |\n| --- | --- |\n| 1 | 2 | 3 |\n");
        let e = doc.check().unwrap_err();
        assert!(e.0.contains("cells where its table has"), "{e}");
    }

    #[test]
    fn a_paragraph_inserted_twice_is_caught() {
        let twice = "An ingredient may be given a name, and another may refer to it.\n\n\
                     An ingredient may be given a name, and another may refer to it.\n";
        let e = Doc::new(twice).check().unwrap_err();
        assert!(e.0.contains("appears twice"), "{e}");
    }

    #[test]
    fn a_document_nobody_has_broken_passes() {
        Doc::new(TRAITS).check().unwrap();
    }
}
