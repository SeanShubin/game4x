//! `C-70`: a column is found by its name, so moving one changes nothing.
//!
//! **`P-346` deleted a column and a reader that counted them started answering a different
//! question.** `first_release.rs` had `cells.get(5)` for *Costs to produce*, the column moved
//! to 4, and the test reported that a pioneer has no metal cost - true about column 5 and
//! nothing at all about the release.
//!
//! **A test that counts columns fails loudly; a generator that counts them does not.**
//! `catalog.rs` read the Recipes table's *Kind*, *Role* and *Where* by position, and a column
//! moving there would have attributed rows to the wrong kind and written a catalog that was
//! wrong and current at the same time - `the_committed_catalog_is_what_the_release_generates`
//! compares the file against the same computation, so it would have stayed green.
//!
//! **The check is a permutation rather than an example.** Reordering the columns of a table
//! is the operation that broke it, so this does exactly that and asserts nothing moved.

use kinds::catalog::{signature, signatures};
use kinds::release::{body_under, column_of, plain, release};

/// The document with the columns of one table reordered.
///
/// **Header and body together, so the table stays true.** A permutation that moved the header
/// and not the rows would be a corrupt document, and a test against one proves nothing.
fn reordered(document: &str, heading: &str, order: &[usize]) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut inside = false;
    let mut touched = 0;
    for line in document.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            inside = trimmed == heading;
            out.push(line.to_string());
            continue;
        }
        if !inside || !trimmed.starts_with('|') {
            out.push(line.to_string());
            continue;
        }
        let cells: Vec<String> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect();
        if cells.len() != order.len() {
            out.push(line.to_string());
            continue;
        }
        let moved: Vec<String> = order.iter().map(|at| cells[*at].clone()).collect();
        out.push(format!("| {} |", moved.join(" | ")));
        touched += 1;
    }
    assert!(
        touched > 3,
        "only {touched} rows of `{heading}` were reordered, so this proves nothing"
    );
    out.join("\n")
}

/// Every kind's signature survives the Recipes table's columns being reordered.
///
/// **Over every kind, and the population is asserted**, because an equality between two empty
/// sets of signatures would hold for the wrong reason.
#[test]
fn a_signature_does_not_depend_on_which_column_a_value_sits_in() {
    let document = release();
    let kinds: Vec<String> = body_under(&document, "## Kinds")
        .iter()
        .map(|row| plain(&row[0]))
        .collect();
    assert_eq!(kinds.len(), 16, "sixteen kinds is the population here");

    // Recipe, Owner, Role, Qty, Kind, Traits, Where -> reversed, which moves all seven.
    let shuffled = reordered(&document, "## Recipes", &[6, 5, 4, 3, 2, 1, 0]);
    assert_ne!(shuffled, document, "the reordering did nothing");
    assert_eq!(
        column_of(&shuffled, "## Recipes", "Kind"),
        2,
        "the Kind column should have moved from 4 to 2"
    );

    let mut compared = 0;
    let mut pairs_seen = 0;
    for kind in &kinds {
        let before = signature(&document, kind);
        let after = signature(&shuffled, kind);
        assert_eq!(
            before.key(),
            after.key(),
            "`{kind}` reads differently when the columns move"
        );
        pairs_seen += before.pairs.len();
        compared += 1;
    }
    assert_eq!(compared, 16, "a kind was skipped");
    assert!(
        pairs_seen > 0,
        "no kind had a single (recipe, role) pair, so the columns under test were never read"
    );

    // And the grouping, which is what `R-8` reports.
    assert_eq!(
        signatures(&document).len(),
        signatures(&shuffled).len(),
        "the grouping changed when only the column order did"
    );
}

/// A column the release does not have is refused rather than guessed at.
#[test]
#[should_panic(expected = "has no `Nonesuch` column")]
fn a_column_that_is_not_there_is_refused() {
    column_of(&release(), "## Recipes", "Nonesuch");
}
