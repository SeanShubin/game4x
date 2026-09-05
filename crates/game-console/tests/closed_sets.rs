//! Every value a trait admits is a row in the table that lists them.
//!
//! **`S-22`.** `P-209` and `P-210` deleted the counts a check was built to compare: the
//! `kind` trait said *one of the twelve* and now says *one of the kinds*, and `biome` the
//! same. `prototypes/kinds` handles the half that is about the document agreeing with itself
//! - a stated count matching a row count, and a named set naming a table that exists.
//!
//! **This is the half that is strictly stronger, and it is stronger for a reason worth
//! stating.** A count can agree while the membership is wrong, and it did: for two days the
//! `kind` trait's count was right and `territory` was in neither the Kinds table nor the
//! Families table. Two numbers agreed and the sets did not.
//!
//! # Why it cannot live in `prototypes/kinds`
//!
//! That crate *copies* the release, so a check there compares the release with a
//! transcription of itself. **Two things that agree cannot notice they are both wrong.** The
//! model is the independent witness: `Kind::ALL` and `Biome::ALL` are what the game actually
//! admits, arrived at by being implemented rather than by being read off the document. So the
//! comparison here is between a document and a program, which is the only pairing that can
//! fail for the right reason.

use std::collections::BTreeSet;
use std::path::PathBuf;

use game_model::Biome;
use game_model::thing::Kind;

fn release() -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release document")
}

/// The first cell of every row under a heading, unbolded and lowercased.
///
/// Stops at the next heading, so a table further down the file is never gathered into the
/// one being read - which is how a set can appear complete while being two tables joined.
fn named_under(document: &str, heading: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line.trim() == heading;
            continue;
        }
        let line = line.trim();
        if !inside || !line.starts_with('|') || line.contains("---") {
            continue;
        }
        let first = line
            .trim_matches('|')
            .split('|')
            .next()
            .unwrap_or_default()
            .trim()
            .trim_matches('*')
            .trim()
            .to_lowercase();
        if first.is_empty() {
            continue;
        }
        out.push(first);
    }
    // The header row, which is a column name rather than a value.
    if !out.is_empty() {
        out.remove(0);
    }
    out
}

/// Every value the model admits is listed, and everything listed is a value it admits.
///
/// **Both directions, because they fail differently.** A value the model has and the table
/// lacks is a rule nobody wrote down - `territory` was one for two days. A row the table has
/// and the model lacks is a rule nobody built, and it reads as delivered because it is
/// written in the release.
///
/// # The two ways this could pass over nothing
///
/// `S-22` names both and this answers both. **The traits examined are written out rather
/// than discovered**, because a trait whose values are free text has no table to check
/// against and discovery would silently skip it - so the list is a decision, and the count
/// of it is asserted. **And a table that lists nothing agrees with an empty set**, so each
/// side is asserted non-empty before the comparison that would otherwise be vacuous.
#[test]
fn every_value_a_trait_admits_is_a_row_in_the_table_that_lists_them() {
    let document = release();

    // Written out, not discovered. Two traits name a closed set; the rest are free text or
    // numbers and have no table to be held against.
    let closed: [(&str, &str, Vec<String>); 2] = [
        (
            "kind",
            "## Kinds",
            Kind::ALL.iter().map(|k| k.name().to_lowercase()).collect(),
        ),
        (
            "biome",
            "## Biomes",
            Biome::ALL.iter().map(|b| b.name().to_lowercase()).collect(),
        ),
    ];
    assert_eq!(closed.len(), 2, "two traits name a closed set");

    let mut compared = 0;
    for (trait_name, heading, admits) in &closed {
        let listed: BTreeSet<String> = named_under(&document, heading).into_iter().collect();
        let admits: BTreeSet<String> = admits.iter().cloned().collect();

        assert!(
            !listed.is_empty(),
            "{heading} lists nothing, so it would agree with any model at all"
        );
        assert!(
            !admits.is_empty(),
            "the model admits no `{trait_name}`, so this would agree with any table at all"
        );

        let unwritten: Vec<&String> = admits.difference(&listed).collect();
        let unbuilt: Vec<&String> = listed.difference(&admits).collect();
        assert!(
            unwritten.is_empty() && unbuilt.is_empty(),
            "the `{trait_name}` trait and {heading} admit different sets:\n  \
             the model has and {heading} does not list: {unwritten:?}\n  \
             {heading} lists and the model does not have: {unbuilt:?}"
        );
        compared += admits.len();
    }

    // Over every case, and how many cases there were. Twelve kinds and six biomes; a run
    // that compared two empty sets twice would satisfy everything above it.
    assert_eq!(
        compared, 18,
        "twelve kinds and six biomes were compared when this was written; {compared} were"
    );
}

/// The check finds a value missing from the table, and one the table has alone.
///
/// **Poisoning the real release is not an option here** - it is Sean's file and this lane
/// does not edit it - so the two failures are demonstrated against a document written to
/// carry each. Without this, both arms are claims: the test passes on today's release and
/// would pass just as green if `named_under` returned nothing at all.
#[test]
fn the_check_catches_a_value_missing_from_the_table_and_one_missing_from_the_model() {
    let two_rows = "## Kinds\n\n\
         | Kind        | What it is |\n\
         | ----------- | ---------- |\n\
         | **citizen** | a person   |\n\
         | **ark**     | it carries |\n";

    let listed = named_under(two_rows, "## Kinds");
    assert_eq!(
        listed,
        vec!["citizen", "ark"],
        "the header is dropped and the bold markers with it"
    );

    // A value the model has and the table lacks.
    let model: BTreeSet<String> = ["citizen", "ark", "pioneer"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let table: BTreeSet<String> = listed.iter().cloned().collect();
    assert_eq!(
        model.difference(&table).collect::<Vec<_>>(),
        vec!["pioneer"],
        "a kind nobody wrote down"
    );

    // A row the table has and the model lacks - `territory`'s shape, which is the one that
    // survived a matching count for two days.
    let three_rows = format!("{two_rows}| **territory** | ground |\n");
    let table: BTreeSet<String> = named_under(&three_rows, "## Kinds").into_iter().collect();
    let model: BTreeSet<String> = ["citizen", "ark"].iter().map(|s| s.to_string()).collect();
    assert_eq!(
        table.difference(&model).collect::<Vec<_>>(),
        vec!["territory"],
        "a kind nobody built, which reads as delivered because it is in the release"
    );

    // And the guard against the vacuous pass: a heading with no table under it.
    assert!(
        named_under("## Kinds\n\nnothing here yet.\n", "## Kinds").is_empty(),
        "an empty table has to read as empty, or the assertion above it never fires"
    );
}
