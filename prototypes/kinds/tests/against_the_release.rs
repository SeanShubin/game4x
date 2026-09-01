//! The compilable specification and the written one are the same specification.
//!
//! `S-4`. Two copies of fifteen recipes would be one copy and one guess: the data
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
    let written = table_under(&document, "## Recipes");
    assert!(
        written.len() > 40,
        "only {} rows parsed out of the recipes table; its shape has changed",
        written.len()
    );
    compare("The recipes table", &written, &kinds::recipes_table());
}

/// Eighteen: the fifteen `S-4` asked for, and `upkeep`, `perish` and `revert` from `P-140`.
#[test]
fn there_are_eighteen_recipes() {
    assert_eq!(kinds::RECIPES.len(), 18);
}

/// The measurements the four questions were answered from, kept as assertions so that a
/// later change to the release makes the answer stale loudly rather than quietly.
#[test]
fn the_figures_the_answers_rest_on() {
    use kinds::{Port, Quantity, Scope};

    let quantities: Vec<Quantity> = kinds::RECIPES
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

    let here = kinds::RECIPES
        .iter()
        .filter(|t| t.scope == Scope::Here)
        .count();
    // The three P-140 added are all `every`, which is the shape of what it named: upkeep,
    // perishing and reverting happen to a planet rather than being asked for.
    assert_eq!((here, kinds::RECIPES.len() - here), (10, 8));

    // `node, unworked` and `food, surplus` are the derived ones, and the cost of choosing
    // between derived kinds and comparisons is measured by how often they appear.
    let derived = kinds::RECIPES
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
    // Two when the table had fifteen recipes, four now: `whose upkeep is unpaid` and
    // `force below its force of nature` are comparisons too, and both arrived with P-140.
    assert_eq!(derived, 4, "four rows out of sixty-odd is what it costs");
}

/// A family matches what carries it, with no hierarchy anywhere.
///
/// The two rows that settle the fifth finding: one recipe naming `unit` matches an
/// Ark and matches a Pioneer, and membership is a list rather than a parent class.
#[test]
fn a_recipe_naming_a_family_matches_every_kind_in_it() {
    use kinds::Kind;

    assert!(Kind::Unit.covers(Kind::Ark));
    assert!(Kind::Unit.covers(Kind::Pioneer));
    assert!(!Kind::Unit.covers(Kind::Yard), "a yard does not move");
    assert!(!Kind::Unit.covers(Kind::Citizen), "nor does a citizen");

    for resource in [Kind::Food, Kind::Metal, Kind::Energy] {
        assert!(Kind::Resource.covers(resource));
    }
    assert!(!Kind::Resource.covers(Kind::Labor), "labor is not stored");

    // A leaf covers itself and nothing else, so `covers` is safe to ask of any kind.
    assert!(Kind::Ark.covers(Kind::Ark));
    assert!(!Kind::Ark.covers(Kind::Pioneer));
    assert!(Kind::Ark.members().is_empty());
}

/// What a `thing` is, now that the release says.
///
/// This crate reported that `ready` named a family the release never defined, and the test
/// that asserted the gap said to delete it the day the release filled it. `P-140` gave the
/// units table a **Readies** column and the prose beneath it says *nothing outside this
/// table readies* - so the gap is closed, and that test is gone rather than left passing
/// vacuously. A test that knows it is temporary is worth more than one that does not.
#[test]
fn a_thing_is_whatever_the_units_table_readies() {
    use kinds::Kind;

    assert!(!Kind::Thing.members().is_empty(), "the release says now");
    for kind in [Kind::Citizen, Kind::Extractor, Kind::Ark, Kind::Pioneer] {
        assert!(Kind::Thing.covers(kind), "{} readies", kind.name());
    }
    // A garrison and a yard have no Readies cell, and nothing outside the table readies.
    assert!(!Kind::Thing.covers(Kind::Garrison));
    assert!(!Kind::Thing.covers(Kind::Yard));
    assert!(!Kind::Thing.covers(Kind::Labor));

    // And no recipe names a family the release leaves undefined, which the gap was.
    let unstated = kinds::RECIPES
        .iter()
        .flat_map(|recipe| recipe.ports.iter())
        .filter(|port| {
            let subject = match port {
                kinds::Port::In { subject, .. } | kinds::Port::Out { subject, .. } => subject,
            };
            subject.kind.is_family() && subject.kind.members().is_empty()
        })
        .count();
    assert_eq!(
        unstated, 0,
        "a recipe names a family the release does not define"
    );
}
