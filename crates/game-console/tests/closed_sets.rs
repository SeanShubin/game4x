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

use game_console::dump;
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
/// `S-22` names both and this answers both. **The sets examined are written out rather than
/// discovered**, because a trait whose values are free text has no table to check against
/// and discovery would silently skip it - so the list is a decision, and the count of it is
/// asserted. **And a table that lists nothing agrees with an empty set**, so each side is
/// asserted non-empty before the comparison that would otherwise be vacuous.
///
/// **One of the two is not a trait, since `P-417`.** The `kind` row is gone from *Traits* -
/// a kind is not a trait, and `spec/console.md` lists the two as different categories - so
/// what is held against *## Kinds* is the model's kinds themselves. The comparison is the
/// one it always was; what moved is what it may be called.
#[test]
fn every_value_a_trait_admits_is_a_row_in_the_table_that_lists_them() {
    let document = release();

    // Written out, not discovered. Two of the model's sets are closed and have a table to
    // be held against; every other trait is free text or a number and has none.
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
    assert_eq!(closed.len(), 2, "two sets are closed and written down");

    let mut compared = 0;
    for (set, heading, admits) in &closed {
        let listed: BTreeSet<String> = named_under(&document, heading).into_iter().collect();
        let admits: BTreeSet<String> = admits.iter().cloned().collect();

        assert!(
            !listed.is_empty(),
            "{heading} lists nothing, so it would agree with any model at all"
        );
        assert!(
            !admits.is_empty(),
            "the model admits no `{set}`, so this would agree with any table at all"
        );

        let unwritten: Vec<&String> = admits.difference(&listed).collect();
        let unbuilt: Vec<&String> = listed.difference(&admits).collect();
        assert!(
            unwritten.is_empty() && unbuilt.is_empty(),
            "the model's `{set}` and {heading} admit different sets:\n  \
             the model has and {heading} does not list: {unwritten:?}\n  \
             {heading} lists and the model does not have: {unbuilt:?}"
        );
        compared += admits.len();
    }

    // Over every case, and how many cases there were. Twelve kinds and six biomes; a run
    // that compared two empty sets twice would satisfy everything above it.
    // Fifteen, and the last three all arrived the same way. `store` in `P-260`, `deposit` in
    // `P-322` where `density` went, and `adjacency` in `P-334` where a pair went - each time
    // because a description is a flat map and the thing being written was not one value.
    // **This count is what catches a kind arriving**: the release declares it, the model does
    // not have it, and the gate is red for every lane until the model follows. It has done
    // that four times, the fourth being `game` in `P-351`.
    // The fifth was `fertility`, in the saturating rewrite. **The sixth went away again**:
    // `P-399` turned `readiness` from a yes-or-no trait into a kind and `P-411` turned it
    // back into a count carried as a trait, so seventeen is where the count returned to.
    assert_eq!(
        compared, 23,
        "seventeen kinds and six biomes were compared when this was written; {compared} were"
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

/// Every trait the release says is *of a territory* is shown in the dump.
///
/// **`S-43` asked whether the dump's fields could be held to the release's traits, and the
/// broad version of that cannot be built.** Measured: of 25 non-key columns in the dump, 8
/// are declared traits and 17 are not - and the 17 are four unrelated kinds. Counts of
/// things (`citizens`, `yards`, `built`, `count`), quantities (`amount`, `made`, `spent`,
/// `left`), names that differ from the trait they show (`capacity` for *total capacity*,
/// `readiness` for *ready*), and fields that are simply not traits (`phase`, `turn`,
/// `in-play`). An exemption list of seventeen against a population of twenty-five is the
/// column list written twice, and the second copy is what goes stale.
///
/// **The narrow version needs no exemption list, because the release supplies the
/// discriminator itself.** The *Traits* table has an **Of** column. Four traits say *a
/// territory*, and the question *is this one shown?* has a definite answer for each. That is
/// the direction the bug was in: `founded` was printed and never declared, `control`
/// declared and never printed, and nothing compared the two lists in either direction.
///
/// # What it finds, both carried as named exceptions rather than asserted away
///
/// **`control` is declared and shown nowhere.** `P-255`: Sean dropped `founded` and chose
/// not to print `control` in its place, on his own test - *if we actually need it I will
/// notice when reviewing*. So this is a decision, not a defect, and it is named here so that
/// the check does not have to be weakened to accommodate it.
///
/// **`total capacity` is shown as `capacity`.** Not a decision - nobody chose it, and it is
/// the same shape as `founded`: the dump naming a thing differently from the release, with
/// nothing comparing them. `C-25`, and this lane does not rename a field Sean is reading
/// without asking.
#[test]
fn every_trait_of_a_territory_is_shown_in_the_dump() {
    /// Traits of a territory the dump does not show under that name, and why.
    // **The pattern, and where it stops working.** A named exception carries a reason and
    // fails when it is repaired, so a gap cannot outlive itself and cannot be closed by
    // quietly weakening the assertion. It holds at one or two. **Past about two it stops
    // being a guard and becomes the list written twice** - the same reason
    // `closed_sets.rs` declines to check every dump column against the release's traits:
    // an exemption list of seventeen against a population of twenty-five is not a check,
    // it is a second copy of the thing being checked, and the second copy is what rots.
    // If a third is wanted here, that is the signal to fix the rule rather than the list.
    // **One since `P-331`, and the one that went is the pattern working.** `total capacity`
    // was excepted because the dump printed it as `capacity` - the dump naming a thing
    // differently from the release, which is `founded`'s shape. The row moved to the deposit
    // and the name is `total-capacity` in one place now, so the exception is gone rather than
    // repaired: `C-25` dissolved with the row it was about.
    const NOT_SHOWN: [(&str, &str); 1] = [(
        "control",
        "`P-255`: Sean dropped `founded` and declined to print `control` in its place - \
         *if we actually need it I will notice when reviewing*. A decision, not a defect",
    )];

    let document = release();
    let mut of_a_territory = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line.trim() == "## Traits";
            continue;
        }
        let line = line.trim();
        if !inside || !line.starts_with("| **") {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        let name = cells.first().unwrap_or(&"").trim_matches('*').trim();
        let of = cells.get(1).unwrap_or(&"");
        // *a territory*, *a territory, per resource*, *a territory, per kind*. The qualifier
        // says how many rows it takes, not what it is a trait of.
        if of.starts_with("a territory") {
            of_a_territory.push(name.to_string());
        }
    }
    // **Three, and both that left went to the deposit.** `density` moved in `P-322` and
    // `total capacity` in `P-331`, because a description is a flat map and a territory had
    // one of each per resource and per kind. So this count going down twice is the rule
    // moving rather than traits being lost, and what is left - `control`, `biome`,
    // `nature` - is one value each.
    assert_eq!(
        of_a_territory.len(),
        3,
        "three traits are of a territory; the release has {} ({of_a_territory:?})",
        of_a_territory.len()
    );

    // Every column of every table the dump produces, so a trait shown anywhere counts.
    let game = game_model::Game::new();
    let columns: Vec<String> = dump::tables(&game)
        .iter()
        .flat_map(|table| table.columns.iter().map(|c| c.to_string()))
        .collect();
    assert!(
        columns.len() > 20,
        "only {} columns in the dump, so this would agree with anything",
        columns.len()
    );

    let mut missing = Vec::new();
    for name in &of_a_territory {
        if columns.iter().any(|column| column == name) {
            continue;
        }
        if NOT_SHOWN.iter().any(|(named, _)| named == name) {
            continue;
        }
        missing.push(name.clone());
    }
    assert!(
        missing.is_empty(),
        "the release declares these traits of a territory and the dump shows none of them: \
         {missing:?}"
    );

    // An exception that has been repaired is a lie in the other direction, and nothing else
    // would notice: this would go on passing while claiming a gap that had closed.
    for (named, why) in NOT_SHOWN {
        assert!(
            !columns.iter().any(|column| column == named),
            "`{named}` is shown now, so delete its exception: {why}"
        );
        assert!(
            of_a_territory.iter().any(|name| name == named),
            "`{named}` is excepted here and the release no longer declares it of a territory"
        );
    }
    assert_eq!(
        NOT_SHOWN.len(),
        1,
        "one is not shown, and it is a decision rather than a finding"
    );
}
