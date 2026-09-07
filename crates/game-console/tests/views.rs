//! A value compared across rows is a column; one that is not may be a node in a cell.
//!
//! **`P-216`, and `S-26` lists it as buildable now.** `spec/console.md` -> *Commands*
//! distinguishes the two views the release ships, and the distinction is about what a reader
//! can do with a column rather than about how much fits in one:
//!
//! - **The normalized view has no nested cells.** `state.md` is one table per relation, and
//!   the whole reason for a relation is that a value can be compared down a column. A cell
//!   holding `force 0 manned 0` cannot be compared with anything; it is two facts wearing one
//!   heading.
//! - **The entity view may.** `entities.md` is one row per thing and one column per
//!   component, so a component that is itself several facts has nowhere else to go. Its
//!   garrison cell reads `force 0 manned 0` today.
//!
//! # Why this needed a check rather than a look
//!
//! **Both views were already right and nothing said so.** That is the state a rule is in just
//! before it stops being true: the next person to add a column to `state.md` has no reason to
//! know which of the two files they are in, and a nested cell there would read as a tidy way
//! to fit two numbers rather than as a rule broken.
//!
//! **It reads the generated files rather than the code that writes them** - `C-28`. Counting
//! what `dump.rs` says would tell you what was written and nothing about what a reader opens.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn report(name: &str) -> String {
    std::fs::read_to_string(root().join("reports").join(name))
        .unwrap_or_else(|why| panic!("{name} is generated: {why}"))
}

/// Every body cell of every table, with the table it is in and its column.
///
/// **Parsed to cells rather than matched as text**, for the reason `CLAUDE.md` gives: the
/// padder rewrites column widths, so anything matching on the shape of a row stops matching
/// the next time a value gets longer.
fn cells(document: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    let mut table = String::new();
    let mut columns: Vec<String> = Vec::new();
    let mut after_separator = false;

    for line in document.lines() {
        let trimmed = line.trim();
        if let Some(name) = trimmed
            .strip_prefix("### ")
            .or_else(|| trimmed.strip_prefix("## "))
        {
            table = name.trim().to_string();
            columns.clear();
            after_separator = false;
            continue;
        }
        if !trimmed.starts_with('|') {
            continue;
        }
        let row: Vec<String> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect();
        if row.iter().all(|cell| cell.chars().all(|c| c == '-')) {
            after_separator = true;
            continue;
        }
        if !after_separator {
            columns = row;
            continue;
        }
        for (at, cell) in row.into_iter().enumerate() {
            let column = columns.get(at).cloned().unwrap_or_default();
            out.push((table.clone(), column, cell));
        }
    }
    out
}

/// A cell holding more than one fact, which is what a node in a cell is.
///
/// **Two or more words**, because a name is one word - `spec/console.md`: *where it needs
/// more than one, the words are joined with dashes*. So a cell of two words is two things
/// beside each other rather than one thing with a long name, and there is no third case in
/// these files.
fn is_a_node(cell: &str) -> bool {
    cell.split_whitespace().count() > 1
}

/// The normalized view has no nested cells, over every cell of it.
///
/// **Over every cell rather than on a sample, and the count with it.** A check that looked at
/// one table would go on passing after somebody nested a value in another - which is the shape
/// `docs/notes/checks-outlive-examples.md` records three times in one day.
#[test]
fn the_normalized_view_has_no_nested_cells() {
    let cells = cells(&report("state.md"));
    assert!(
        cells.len() > 300,
        "only {} cells in state.md, so finding none nested would mean nothing",
        cells.len()
    );

    let nested: Vec<String> = cells
        .iter()
        .filter(|(_, _, cell)| is_a_node(cell))
        .map(|(table, column, cell)| format!("{table}.{column}: `{cell}`"))
        .collect();
    assert!(
        nested.is_empty(),
        "the normalized view has nested cells, and a value that cannot be compared down its \
         own column is not a column:\n  {}",
        nested.join("\n  ")
    );

    // And the tables are really there, so this did not walk an empty document.
    let tables: std::collections::BTreeSet<&String> =
        cells.iter().map(|(table, _, _)| table).collect();
    assert_eq!(
        tables.len(),
        11,
        "eleven tables in state.md; {} were read ({tables:?})",
        tables.len()
    );
}

/// The entity view may have one, and does - so the two views differ rather than agreeing.
///
/// **This is the half that makes the check above mean something.** If neither view ever
/// nested a cell, the rule would be satisfied by a program that could not nest one at all,
/// and the distinction `P-216` draws would be untested in the direction it permits.
#[test]
fn the_entity_view_may_have_a_node_in_a_cell_and_has_one() {
    let cells = cells(&report("entities.md"));
    assert!(
        cells.len() > 100,
        "only {} cells in entities.md, so this would agree with anything",
        cells.len()
    );

    let nested: Vec<&(String, String, String)> = cells
        .iter()
        .filter(|(_, _, cell)| is_a_node(cell))
        .collect();
    assert!(
        !nested.is_empty(),
        "no cell of the entity view holds a node, so nothing here distinguishes it from the \
         normalized view"
    );
    // Named, because *which* cell it is matters: the garrison is the one `P-216` cites.
    assert!(
        nested
            .iter()
            .any(|(_, column, cell)| column == "garrison" && cell.starts_with("force ")),
        "the garrison cell is the node `P-216` names, and it is not one: {nested:?}"
    );
}

/// The check can tell a node from a name, which is what the whole rule turns on.
///
/// **Poisoning it, because a predicate that has only ever answered *no* is a claim.** Both
/// tests above rest entirely on `is_a_node`, and a version of it that always returned false
/// would make one of them green and the other red - so only the pair is evidence, and only
/// once the predicate itself has been shown to answer both ways.
#[test]
fn a_node_is_told_from_a_name_and_from_a_number() {
    for one in [
        "citizen",
        "labor-spent",
        "12",
        "food",
        "yes",
        "territory-resource",
    ] {
        assert!(!is_a_node(one), "`{one}` is one word and is not a node");
    }
    for several in ["force 0 manned 0", "the game", "1 food per turn"] {
        assert!(is_a_node(several), "`{several}` is more than one fact");
    }
    // And the reader finds cells at all in a document that has them, which is the failure
    // that would make both tests above pass over nothing.
    let two_tables =
        "## one\n\n| a | b |\n| - | - |\n| 1 | x y |\n\n## two\n\n| c |\n| - |\n| z |\n";
    let read = cells(two_tables);
    assert_eq!(
        read,
        vec![
            ("one".to_string(), "a".to_string(), "1".to_string()),
            ("one".to_string(), "b".to_string(), "x y".to_string()),
            ("two".to_string(), "c".to_string(), "z".to_string()),
        ],
        "the reader pairs each cell with its table and its column"
    );
    assert!(cells("## empty\n\nnothing here.\n").is_empty());
}
