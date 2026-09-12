//! `C-70` and `C-71`: a column is found by its name, and a family reaches its members.
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
    assert_eq!(kinds.len(), 18, "eighteen kinds is the population here");

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
    assert_eq!(compared, 18, "a kind was skipped");
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

/// A trait declared of a family is carried by that family's members.
///
/// **`C-71`, and the two halves of a signature agreeing at last.** `recipe_rows` expanded
/// families and `trait_rows` matched the kind's own name alone, so `fuel` - declared *of a
/// unit* - reached neither ark nor pioneer, and `keeps` - declared *of thing* - reached none of
/// the sixteen. `Signature`'s own doc said reaching through a family counts as naming, which
/// was true of one half and false of the other in the same struct's documentation.
///
/// **Read from the release rather than listed here.** The families and their members are the
/// document's, so this cannot drift from it by someone editing a list in a test.
#[test]
fn a_trait_of_a_family_reaches_its_members() {
    let document = release();
    let families = body_under(&document, "## Families");
    assert_eq!(families.len(), 4, "four families is the population here");

    // `fuel` is *of a unit*, and the unit family is ark and pioneer.
    for kind in ["ark", "pioneer"] {
        let carried = signature(&document, kind).traits;
        assert!(
            carried.contains(&"fuel".to_string()),
            "`{kind}` is a unit and `fuel` is declared of a unit: {carried:?}"
        );
    }
    // And nothing outside the family gains it.
    for kind in ["citizen", "food", "territory"] {
        let carried = signature(&document, kind).traits;
        assert!(
            !carried.contains(&"fuel".to_string()),
            "`{kind}` is not a unit and should not carry `fuel`: {carried:?}"
        );
    }

    // `id` is *of a place* since `P-462`, and the place family is territory and orbit. It was
    // in the describing-cell test below until that landed: *a thing that must be named
    // individually* is a predicate, and *a place* is a family.
    for kind in ["territory", "orbit"] {
        let carried = signature(&document, kind).traits;
        assert!(
            carried.contains(&"id".to_string()),
            "`{kind}` is a place and `id` is declared of a place: {carried:?}"
        );
    }
    for kind in ["citizen", "food", "ark"] {
        let carried = signature(&document, kind).traits;
        assert!(
            !carried.contains(&"id".to_string()),
            "`{kind}` is not a place and should not carry `id`: {carried:?}"
        );
    }

    // `keeps` is *of thing*, and the thing family is written `every kind above` - a membership
    // rather than a list, which is the form both joins used to split on commas and miss.
    let kinds: Vec<String> = body_under(&document, "## Kinds")
        .iter()
        .map(|row| plain(&row[0]))
        .collect();
    assert_eq!(kinds.len(), 18, "eighteen kinds is the population here");
    for kind in &kinds {
        assert!(
            signature(&document, kind)
                .traits
                .contains(&"keeps".to_string()),
            "`{kind}` is a thing and `keeps` is declared of thing"
        );
    }
}

/// A cell that describes rather than names is attributed to nothing.
///
/// **`S-78`'s third case, deliberately left out and pinned here so it stays out.** `upkeep` is
/// declared *of a thing with upkeep*, which contains the word `thing` and is a predicate rather
/// than the *thing* family. Matching a family by word attributed `upkeep`, `unpaid` and `id` to
/// all sixteen kinds - which is the first attempt at `C-71`, caught by regenerating the report
/// and reading it.
///
/// **`id` has left this list, and it left by the release changing rather than by this test
/// weakening.** `P-462` made it *of a place*, which is a family the Families table lists, so it
/// is now checked by `a_trait_of_a_family_reaches_its_members` above - a stronger assertion than
/// the one it was under here, not a dropped one.
///
/// Whether these should be resolved from *Units and structures*, which has a column for
/// `Readies` and one for `Movable`, is a design decision and not this repair.
#[test]
fn a_cell_that_describes_rather_than_names_reaches_nothing() {
    let document = release();
    let described = ["upkeep", "unpaid", "ready", "movable"];
    let kinds: Vec<String> = body_under(&document, "## Kinds")
        .iter()
        .map(|row| plain(&row[0]))
        .collect();

    let mut checked = 0;
    for name in described {
        for kind in &kinds {
            assert!(
                !signature(&document, kind)
                    .traits
                    .contains(&name.to_string()),
                "`{name}` describes which things carry it and names no family, so no kind \
                 should be given it - `{kind}` was"
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked,
        described.len() * kinds.len(),
        "a case was skipped, so this checked less than it says"
    );
}
