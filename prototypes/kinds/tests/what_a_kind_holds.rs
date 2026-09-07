//! Every row of *Where things are* reaches a section, and one that reaches none is loud.
//!
//! **`S-58`, and the defect it fixes was found by a person rather than by a check.** Sean read
//! `scenario/expected/play.4x`, asked whether territory 1's twelve energy was disorder about
//! to be wiped, and could not answer it from the four artifacts: the capacity is the stores
//! times what a store holds, and **ten appeared in none of them.** In his words - *it isn't
//! necessarily 10, it just happened to be 10, which means I needed that information
//! explicitly.*
//!
//! The release states it. The catalog read two headings and not that one, so the fact was
//! written down and not gathered.
//!
//! # Why the loud failure is the part worth testing
//!
//! `S-58` asked for it by name: **make it fail loudly when a row matches nothing** rather than
//! quietly contributing no line. That is not a general preference, it is this defect's own
//! shape - a matcher that finds nothing produces a section with one fewer line, and a section
//! with one fewer line reads exactly like a kind that holds nothing.
//!
//! So the guard is exercised here rather than asserted: a document with a container naming no
//! kind must panic, and one naming a kind must not.

use kinds::catalog::catalog;
use kinds::release::{body_under, plain};

fn release() -> String {
    std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../releases/first-release.md"),
    )
    .expect("the release document")
}

/// Every row of *Where things are* appears under at least one kind.
///
/// **Over every row, and the count with it**, because a run over an empty table would satisfy
/// *none was dropped* for the wrong reason.
#[test]
fn every_container_the_release_declares_is_gathered_under_a_kind() {
    let document = release();
    let rows = body_under(&document, "## Where things are");
    assert_eq!(
        rows.len(),
        3,
        "three sorts of capacity in this release; *Where things are* has {} ({rows:?})",
        rows.len()
    );

    let text = catalog(&document);
    let held: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with("**Holds**"))
        .collect();
    // Four, because *a unit's tank* is carried to both members of the `unit` family.
    assert_eq!(
        held.len(),
        4,
        "three rows reach four sections - a store, a territory, and both units; {} lines were \
         written ({held:?})",
        held.len()
    );

    // Every row's own words reach the page, so this is not agreeing with a line it invented.
    for row in &rows {
        let holds = row.get(1).cloned().unwrap_or_default();
        assert!(
            held.iter().any(|line| line.contains(&holds)),
            "*{}* is in *Where things are* and in no section",
            plain(&row[0])
        );
    }
}

/// The number Sean needed is in the catalog, and it says it is a fact about the kind.
///
/// **Named rather than left to the count above.** The count would pass if all four lines were
/// about units; what the defect cost was one specific number, and this is that number.
#[test]
fn a_store_holds_ten_and_the_catalog_says_so_is_a_fact_about_the_kind() {
    let text = catalog(&release());
    let store = text
        .split("\n## ")
        .find(|section| section.starts_with("store\n"))
        .expect("the catalog has a store section");
    let line = store
        .lines()
        .find(|line| line.starts_with("**Holds**"))
        .expect("the store section says what a store holds");
    assert!(
        line.contains("up to 10"),
        "the store's capacity is the number that was missing: {line}"
    );
    assert!(
        line.contains("a fact about the kind"),
        "and ten being a constant rather than this store's own number is the half that \
         answers *is this about to be wiped*: {line}"
    );
}

/// Two of the three are not constants, and the catalog says which.
///
/// `S-58`: *a unit's tank is the unit's fuel, a per-instance trait, and a territory's is
/// derived from what it holds. Only the store's is a fact about the kind* - so one label for
/// all three would be honest about one row and misleading about two.
#[test]
fn a_bound_that_is_not_a_number_is_not_called_a_fact_about_the_kind() {
    let text = catalog(&release());
    let mut constant = 0;
    let mut per_thing = 0;
    for line in text.lines().filter(|line| line.starts_with("**Holds**")) {
        if line.contains("a fact about the kind") {
            constant += 1;
        } else if line.contains("a fact about each one") {
            per_thing += 1;
        } else {
            panic!("a Holds line says neither which it is: {line}");
        }
    }
    assert_eq!(
        (constant, per_thing),
        (1, 3),
        "one constant - the store's ten - and three that depend on the thing"
    );
}

/// A container naming no kind stops the generator instead of contributing nothing.
///
/// **The guard exercised rather than claimed.** Without this, `containers`' assertion is a
/// line that has never run: the real release satisfies it, and a version of the matcher that
/// accepted anything would look identical.
#[test]
#[should_panic(expected = "names no kind and no family")]
fn a_container_that_names_no_kind_is_refused() {
    catalog(
        "## Kinds\n\n\
         | Kind      | What it is |\n\
         | --------- | ---------- |\n\
         | **store** | it holds   |\n\
         \n## Families\n\n\
         | Family    | Members |\n\
         | --------- | ------- |\n\
         | **thing** | every kind above |\n\
         \n## Where things are\n\n\
         | Container   | Holds | Up to |\n\
         | ----------- | ----- | ----- |\n\
         | a warehouse | metal | 40    |\n",
    );
}

/// And one that does name a kind is gathered, so the test above fails for its own reason.
///
/// **The control.** A `should_panic` on its own cannot tell a guard that fires correctly from
/// one that fires at everything, and a matcher that rejected every phrase would pass it.
#[test]
fn a_container_that_names_a_kind_is_gathered() {
    let text = catalog(
        "## Kinds\n\n\
         | Kind      | What it is |\n\
         | --------- | ---------- |\n\
         | **store** | it holds   |\n\
         \n## Families\n\n\
         | Family    | Members |\n\
         | --------- | ------- |\n\
         | **thing** | every kind above |\n\
         \n## Where things are\n\n\
         | Container | Holds | Up to |\n\
         | --------- | ----- | ----- |\n\
         | a store   | metal | 40    |\n",
    );
    assert!(
        text.contains("**Holds** metal, up to 40"),
        "a row naming a kind reaches that kind's section:\n{text}"
    );
}
