//! The compilable specification and the written one are the same specification.
//!
//! `S-4`. Two copies of fifteen transformations would be one copy and one guess: the data
//! in `src/lib.rs` is rendered back into the release's two tables and compared with
//! `releases/first-release.md` on disk, so neither can move without the other. That is the
//! habit `crates/game-console/tests/quotations.rs` already has - read the document at test
//! time rather than trusting a copy of it.
//!
//! **Cells are compared, not bytes.** `tools/pad-tables` owns the column widths and rewrites
//! them whenever anything else in the file changes, so a byte comparison would fail on
//! whitespace nobody wrote. Trimming each cell compares exactly what the table says and
//! nothing about how it is laid out.

use std::path::PathBuf;

fn release() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", path.display()))
}

/// The rows of the first table under this heading, as trimmed cells.
///
/// The separator row is dropped: it is punctuation, and the padder rewrites it.
fn table_under(document: &str, heading: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        let line = line.trim();
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line == heading;
            continue;
        }
        if !inside || !line.starts_with('|') {
            if inside && !rows.is_empty() && !line.starts_with('|') && !line.is_empty() {
                // Prose after the table ends it. A second table under one heading would be
                // a different document than this test was written for.
                break;
            }
            continue;
        }
        let cells: Vec<String> = line
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect();
        if cells.iter().all(|cell| cell.chars().all(|c| c == '-')) {
            continue;
        }
        rows.push(cells);
    }
    rows
}

fn compare(what: &str, written: &[Vec<String>], compiled: &[Vec<String>]) {
    let mut wrong = Vec::new();
    for (at, row) in compiled.iter().enumerate() {
        match written.get(at) {
            Some(theirs) if theirs == row => {}
            Some(theirs) => wrong.push(format!(
                "  row {at}\n    release: {}\n    crate:   {}",
                theirs.join(" | "),
                row.join(" | ")
            )),
            None => wrong.push(format!(
                "  row {at} is missing from the release: {}",
                row.join(" | ")
            )),
        }
    }
    for (at, theirs) in written.iter().enumerate().skip(compiled.len()) {
        wrong.push(format!(
            "  row {at} is in the release and not in the crate: {}",
            theirs.join(" | ")
        ));
    }
    assert!(
        wrong.is_empty(),
        "{what} in `releases/first-release.md` and in this crate are different:\n\n{}\n\n\
         The release is the specification. If it moved, move this crate to match; if this \n\
         crate is right, that is a proposal rather than an edit.",
        wrong.join("\n")
    );
}

#[test]
fn the_units_table_is_the_one_in_the_release() {
    let document = release();
    let written = table_under(&document, "## Units and structures");
    // A parser that found nothing would agree with anything.
    assert!(
        written.len() > 5,
        "only {} rows parsed out of the units table; its shape has changed",
        written.len()
    );
    compare("The units table", &written, &kinds::units_table());
}

#[test]
fn the_transformations_table_is_the_one_in_the_release() {
    let document = release();
    let written = table_under(&document, "## Transformations");
    assert!(
        written.len() > 40,
        "only {} rows parsed out of the transformations table; its shape has changed",
        written.len()
    );
    compare(
        "The transformations table",
        &written,
        &kinds::transformations_table(),
    );
}

/// Fifteen, because the proposal that asked for this said fifteen.
#[test]
fn there_are_fifteen_transformations() {
    assert_eq!(kinds::TRANSFORMATIONS.len(), 15);
}

/// The measurements the four questions were answered from, kept as assertions so that a
/// later change to the release makes the answer stale loudly rather than quietly.
#[test]
fn the_figures_the_answers_rest_on() {
    use kinds::{Port, Quantity, Scope};

    let quantities: Vec<Quantity> = kinds::TRANSFORMATIONS
        .iter()
        .flat_map(|t| t.ports.iter())
        .map(|port| match port {
            Port::In { quantity, .. } | Port::Out { quantity, .. } => *quantity,
        })
        .collect();
    let unusual = quantities
        .iter()
        .filter(|quantity| !matches!(quantity, Quantity::Exactly(_)))
        .count();
    assert_eq!(
        unusual, 4,
        "a quantity that is not a number is why Quantity is not a u32"
    );

    let here = kinds::TRANSFORMATIONS
        .iter()
        .filter(|t| t.scope == Scope::Here)
        .count();
    assert_eq!((here, kinds::TRANSFORMATIONS.len() - here), (10, 5));

    // `node, unworked` and `food, surplus` are the derived ones, and the cost of choosing
    // between derived kinds and comparisons is measured by how often they appear.
    let derived = kinds::TRANSFORMATIONS
        .iter()
        .flat_map(|t| t.ports.iter())
        .filter(|port| {
            let subject = match port {
                Port::In { subject, .. } | Port::Out { subject, .. } => subject,
            };
            subject
                .distinguished_by
                .is_some_and(|distinction| distinction.is_derived())
        })
        .count();
    assert_eq!(derived, 2, "two rows out of fifty-odd is what it costs");
}
