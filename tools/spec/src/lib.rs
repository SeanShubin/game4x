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

/// Phrases that turn a commit message into an assertion about the tree.
///
/// Three of 2026-09-01's defects were a claim made before the check that would refute it, and one
/// of them - *the word bin now appears nowhere* - was false when written. A message saying one of
/// these has to say what it counted.
const ASSERTING: [&str; 6] = [
    "appears nowhere",
    "appears in none",
    "nowhere in the",
    "does not appear anywhere",
    "no longer appears anywhere",
    "appears exactly",
];

/// Whether a commit message asserts something about the tree that ought to have been counted.
pub fn asserts_about_the_tree(message: &str) -> bool {
    let lower = message.to_lowercase();
    ASSERTING.iter().any(|phrase| lower.contains(phrase))
}

/// One countable claim.
///
/// `out_of` is what a claim of **zero** is zero out of. `within` is the `## ` heading whose
/// section both were counted in, and a claim of zero must give it.
///
/// The quality lens poisoned a version that asked only for a denominator: over a whole
/// `proposals.md` with an empty queue, *`**into**` is zero out of `P-`* passed, because the
/// accepted ledger holds `P-` while the population the claim was about held nothing.
///
/// Naming the region does not make the choice correct - **it makes it written down**. Claiming
/// zero in `## Accepted` for a field that lives in `## Open` is a visible mistake, where picking a
/// convenient denominator over a whole file was an invisible one. Past that the guard would have
/// to know what the sentence means, which is not a thing to attempt.
pub struct Claim<'a> {
    pub needle: &'a str,
    pub expected: usize,
    pub out_of: Option<&'a str>,
    pub within: Option<&'a str>,
}

/// Check what a message says against what the text holds.
pub fn check_claims(text: &str, claims: &[Claim]) -> Result<(), Problem> {
    for claim in claims {
        let region = match claim.within {
            None => text.to_string(),
            Some(heading) => {
                let (start, end) = Doc::new(text).section(heading)?;
                text.lines().collect::<Vec<_>>()[start..end].join("\n")
            }
        };
        let place = claim.within.unwrap_or("the whole text");
        let found = region.matches(claim.needle).count();
        if found != claim.expected {
            return problem(format!(
                "the message says {:?} appears {} times in {place}; it appears {found}",
                claim.needle, claim.expected
            ));
        }
        if claim.expected == 0 {
            let Some(out_of) = claim.out_of else {
                return problem(format!(
                    "a claim that {:?} appears nowhere must say what it is nowhere among",
                    claim.needle
                ));
            };
            if claim.within.is_none() {
                return problem(format!(
                    "a claim that {:?} is zero must say which section it counted",
                    claim.needle
                ));
            }
            if region.matches(out_of).count() == 0 {
                return problem(format!(
                    "{:?} is zero out of {out_of:?} in {place}, which is also zero",
                    claim.needle
                ));
            }
        }
    }
    Ok(())
}

/// Open proposals addressed to Sean that carry no text he could approve.
///
/// P-166, P-168 and P-181 were filed as findings with options and no verbatim block. He said
/// *promote* and there was nothing to copy, three times.
pub fn proposals_without_text(proposals: &str) -> Vec<String> {
    let Some(open) = proposals.find("## Open") else {
        return Vec::new();
    };
    let end = proposals[open..]
        .find("## Accepted")
        .map(|at| open + at)
        .unwrap_or(proposals.len());
    let mut without = Vec::new();
    let mut id: Option<String> = None;
    let mut to_sean = false;
    let mut quoted = false;
    for line in proposals[open..end].lines().chain(["### P-0 "]) {
        if line.starts_with("### P-") {
            if let Some(previous) = id.take() {
                if to_sean && !quoted {
                    without.push(previous);
                }
            }
            id = line.split_whitespace().nth(1).map(str::to_string);
            to_sean = false;
            quoted = false;
        } else if line.contains("**to** sean") && line.contains("**status** open") {
            to_sean = true;
        } else if line.starts_with("> ") {
            quoted = true;
        }
    }
    without
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

    /// Computed rather than asserted: the quality lens's point that these ten tests each encode a
    /// hand-written expectation, which is the component this lane has just called least reliable.
    /// This one states no expected text - it says only that one cell moved and no other did.
    #[test]
    fn setting_a_cell_changes_that_cell_and_no_other() {
        let before = Doc::new(TRAITS);
        let mut after = Doc::new(TRAITS);
        after
            .set_cell("## Traits", "| **unpaid**", "Of", "anything at all", None)
            .unwrap();
        let cells_of = |doc: &Doc| -> Vec<String> {
            doc.text()
                .lines()
                .filter(|l| l.starts_with('|'))
                .flat_map(|l| {
                    cells(l)
                        .iter()
                        .map(|c| c.trim().to_string())
                        .collect::<Vec<_>>()
                })
                .collect()
        };
        let (a, b) = (cells_of(&before), cells_of(&after));
        assert_eq!(a.len(), b.len(), "the shape of the tables changed");
        let differing: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
        assert_eq!(differing.len(), 1, "cells that changed: {differing:?}");
    }

    /// The queue as it stood on the day the quality lens got it wrong: `## Open` empty, and
    /// `## Accepted` holding ledger rows that a careless denominator can find.
    const EMPTY_QUEUE: &str = "## Open\n\n## Accepted\n\n| P-182, a lane owns its tools |\n";

    #[test]
    fn a_claim_of_zero_must_say_what_it_is_zero_among() {
        let e = check_claims(
            "some text",
            &[Claim {
                needle: "zzz",
                expected: 0,
                out_of: None,
                within: None,
            }],
        )
        .unwrap_err();
        assert!(e.0.contains("must say what it is nowhere among"), "{e}");
    }

    /// Their first poison, and their original error: the pattern right, the population empty.
    #[test]
    fn a_claim_of_zero_over_an_empty_population_is_refused() {
        let e = check_claims(
            EMPTY_QUEUE,
            &[Claim {
                needle: "**into**",
                expected: 0,
                out_of: Some("### P-"),
                within: Some("## Open"),
            }],
        )
        .unwrap_err();
        assert!(e.0.contains("which is also zero"), "{e}");
    }

    /// Their second poison, which passed before the region existed: a denominator one character
    /// shorter, found in the ledger rather than in the queue the claim was about.
    #[test]
    fn a_denominator_found_outside_the_region_no_longer_passes() {
        let e = check_claims(
            EMPTY_QUEUE,
            &[Claim {
                needle: "**into**",
                expected: 0,
                out_of: Some("P-"),
                within: Some("## Open"),
            }],
        )
        .unwrap_err();
        assert!(e.0.contains("which is also zero"), "{e}");
    }

    /// And with no region named, which is how that poison got in at all.
    #[test]
    fn a_claim_of_zero_must_say_where_it_looked() {
        let e = check_claims(
            EMPTY_QUEUE,
            &[Claim {
                needle: "**into**",
                expected: 0,
                out_of: Some("P-"),
                within: None,
            }],
        )
        .unwrap_err();
        assert!(e.0.contains("which section it counted"), "{e}");
    }

    #[test]
    fn a_claim_of_zero_over_a_real_population_passes() {
        check_claims(
            "## Open\n\n### P-1 a proposal\n### P-2 another\n",
            &[Claim {
                needle: "**into**",
                expected: 0,
                out_of: Some("### P-"),
                within: Some("## Open"),
            }],
        )
        .unwrap();
    }

    #[test]
    fn a_count_that_is_wrong_is_refused() {
        let e = check_claims(
            "a a a",
            &[Claim {
                needle: "a",
                expected: 2,
                out_of: None,
                within: None,
            }],
        )
        .unwrap_err();
        assert!(e.0.contains("it appears 3"), "{e}");
    }

    #[test]
    fn a_message_asserting_about_the_tree_is_recognised() {
        assert!(asserts_about_the_tree(
            "The word bin now appears nowhere in spec/"
        ));
        assert!(!asserts_about_the_tree(
            "Rewrite P-165 with the conservation arithmetic"
        ));
    }

    #[test]
    fn an_open_proposal_with_no_text_to_approve_is_named() {
        let queue = "## Open\n\n\
            ### P-1 - a finding\n\n**to** sean - **status** open - **kind** gap\n\nTwo ways.\n\n\
            ### P-2 - a proposal\n\n**to** sean - **status** open - **kind** gap\n\n> the text\n\n\
            ## Accepted\n";
        assert_eq!(proposals_without_text(queue), vec!["P-1".to_string()]);
    }
}
