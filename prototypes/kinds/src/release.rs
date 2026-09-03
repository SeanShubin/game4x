//! Reading the release's tables, which `spec/invariants.md` calls the data.
//!
//! > The tables that define kinds, families, traits and recipes are the data the game loads.
//! > Nothing restates them; every other form of them is derived, and a derived form is
//! > generated rather than written.
//!
//! **This parser existed and threw its answer away.** It lived in
//! `tests/against_the_release.rs`, where it read the tables only to compare them with data
//! written by hand in this crate - two copies kept in step by a check. It is here now so
//! that something can be derived from the tables rather than checked against them.
//!
//! It reads cells rather than bytes, because `tools/pad-tables` owns the column widths and
//! rewrites them whenever anything else in the file changes.

use std::path::{Path, PathBuf};

/// Where the release lives, relative to this crate.
pub fn release_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md")
}

pub fn release() -> String {
    let at = release_path();
    std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()))
}

/// The rows of the first table under this heading, as trimmed cells.
///
/// The separator row is dropped: it is punctuation, and the padder rewrites it. The header
/// row is kept, because a caller that wants to know what a column is called needs it.
pub fn table_under(document: &str, heading: &str) -> Vec<Vec<String>> {
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
                // a different document than this was written for.
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

/// The rows under a heading with the header row dropped, which is what most callers want.
pub fn body_under(document: &str, heading: &str) -> Vec<Vec<String>> {
    table_under(document, heading).into_iter().skip(1).collect()
}

/// A name as the tables write it, with the emphasis markers taken off.
pub fn plain(cell: &str) -> String {
    cell.trim().trim_matches('*').trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOCUMENT: &str = "\
## Kinds

| Kind        | What it is |
| ----------- | ---------- |
| **citizen** | a person   |
| **ark**     | it carries |

Prose after the table ends it.

## Families

| Family    | Members          |
| --------- | ---------------- |
| **thing** | every kind above |
";

    #[test]
    fn a_table_is_read_as_cells_and_the_separator_is_not_one() {
        let kinds = table_under(DOCUMENT, "## Kinds");
        assert_eq!(
            kinds,
            vec![
                vec!["Kind".to_string(), "What it is".to_string()],
                vec!["**citizen**".to_string(), "a person".to_string()],
                vec!["**ark**".to_string(), "it carries".to_string()],
            ]
        );
        assert_eq!(body_under(DOCUMENT, "## Kinds").len(), 2, "the header goes");
    }

    /// The next heading ends a table, and so does prose.
    ///
    /// Both matter: without the first, every later table would be read as part of the
    /// earlier one; without the second, a paragraph between two tables under one heading
    /// would join them.
    #[test]
    fn a_table_ends_at_prose_and_at_the_next_heading() {
        assert_eq!(body_under(DOCUMENT, "## Families").len(), 1);
        assert!(
            !table_under(DOCUMENT, "## Kinds")
                .iter()
                .any(|row| row[0].contains("thing")),
            "the Families table is not part of the Kinds table"
        );
    }

    /// A heading that is not there reads as no rows, rather than as the whole file.
    #[test]
    fn an_absent_heading_is_empty_rather_than_everything() {
        assert!(table_under(DOCUMENT, "## Recipes").is_empty());
    }

    #[test]
    fn emphasis_is_not_part_of_a_name() {
        assert_eq!(plain("**citizen**"), "citizen");
        assert_eq!(plain(" *unit* "), "unit");
        assert_eq!(plain("territory"), "territory");
    }
}
