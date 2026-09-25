//! What the engine runs is what `reviewed/` generates, row for row.
//!
//! **`spec/README.md`, rule 3**: *a test is stated in the friendly form, and the foundation form
//! is a rendering of it. The rendering is generated from `reviewed/` and never from `spec/tests/`,
//! so that what the engine runs is derived from what has been read rather than compared with it.*
//!
//! # Which side is the expectation, which is the whole of the reversal
//!
//! **`tests/directories.rs` converts `spec/tests/` and asserts `data/foundation/` equals it.** The
//! working copy is the expectation there, and a test Sean has not read is in it.
//!
//! **This asserts the other way**: the record generates, and what the engine runs has to be that.
//! A test with no record is **not** in the population - `every_read_test`'s contract - so a draft
//! constrains nothing here, and is named and counted rather than refused.
//!
//! # The rows, not the bytes, and fourteen relations are why
//!
//! **Fourteen of the relations a test names are rule names** - `move`, `deploy`, `work` and eleven
//! more - and **no `{relation}` row declares any of them.** So `Schema::write` cannot find a
//! column order for them and falls back to the notation's, which is alphabetical, while the
//! committed files carry an order somebody chose by hand.
//!
//! **Measured before this was written**: all 54 files are identical as rows, and **six** of the
//! fourteen are where the written order actually differs today - the other eight happen to be
//! alphabetical already, or to have one column. **So the bytes are not reproducible and the rows
//! are**, and asserting the bytes would assert a hand's habit rather than the rule.
//!
//! **The first version of the check below asserted the six**, because six is what a diff of the
//! two directories reports. **That is the population that differs, not the population at risk** -
//! the other eight are undeclared too and would differ the moment anyone reordered one by hand.
//!
//! **That is a fact about the notation rather than about this check**, and it is recorded here
//! because the obvious stronger assertion - byte equality - looks correct and would be wrong.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use friendly_notation::{Names, fold, in_a_section, states_a_world};
use thin_engine::notation::{Row, write};
use thin_engine::schema::Schema;

use common::{mine, rows};

/// The fewest records that can be there before a run proves nothing.
///
/// **The same forty `first_test.rs` and `examples/foundation.rs` use.** Three places with three
/// opinions about when a run is vacuous would be three chances to pick the wrong one.
const FLOOR: usize = 40;

fn records_at() -> PathBuf {
    mine().join("..").join("..").join("reviewed")
}

fn tests_at() -> PathBuf {
    mine().join("..").join("..").join("spec").join("tests")
}

fn names_in(at: &PathBuf) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(at) else {
        return Vec::new();
    };
    let mut found: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|it| it == "4x").unwrap_or(false))
        .filter_map(|path| {
            path.file_name()
                .and_then(|it| it.to_str())
                .map(str::to_string)
        })
        .collect();
    found.sort();
    found
}

/// A row with its values in a fixed order, so two spellings of one row compare equal.
fn shape(row: &Row) -> (String, Vec<(String, String)>) {
    let mut values: Vec<(String, String)> = row
        .values
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    values.sort();
    (row.relation.clone(), values)
}

/// Every test the engine runs is the one `reviewed/` generates.
#[test]
fn what_the_engine_runs_is_what_the_record_generates() {
    let schema = Schema::of(&rows("data/friendly/schema.4x")).expect("a schema");

    let records: BTreeSet<String> = names_in(&records_at()).into_iter().collect();
    let drafts = names_in(&tests_at());
    let unread: Vec<String> = drafts
        .iter()
        .filter(|name| !records.contains(*name))
        .cloned()
        .collect();

    // **The floor, first.** With `reviewed/` missing, everything below would range over nothing
    // and report that what the engine runs agrees with it perfectly.
    assert!(
        records.len() >= FLOOR,
        "only {} records in reviewed/, and the floor is {FLOOR} - a comparison over that many \
         proves almost nothing. Is reviewed/ there?",
        records.len()
    );

    // **Named and counted rather than refused** - `every_read_test`'s contract. Drafting a test
    // is not an error, and this printing is how a test waiting on Sean is reported rather than
    // merely absent.
    if unread.is_empty() {
        println!("every test in spec/tests has a record");
    } else {
        println!(
            "{} of {} tests have not been read and are not compared here: {}",
            unread.len(),
            drafts.len(),
            unread.join(", ")
        );
    }

    // The two name stores, built the way `examples/foundation.rs` builds them.
    let mut of_game: Vec<Row> = Vec::new();
    let mut of_script: Vec<Row> = Vec::new();
    let mut add = |these: Vec<Row>, whole_file_is_the_game: bool| {
        let mine = states_a_world(&these);
        for (row, is_game) in these.into_iter().zip(mine) {
            let into = if whole_file_is_the_game || is_game {
                &mut of_game
            } else {
                &mut of_script
            };
            if !into.contains(&row) {
                into.push(row);
            }
        }
    };
    for (file, game) in [
        ("schema.4x", true),
        ("engine.4x", true),
        ("rules.4x", true),
        ("script.4x", false),
        ("setup.4x", false),
    ] {
        let at = format!("data/friendly/{file}");
        let text =
            std::fs::read_to_string(mine().join(&at)).unwrap_or_else(|why| panic!("{at}: {why}"));
        add(
            fold(&text, &schema).unwrap_or_else(|why| panic!("{at}: {why}")),
            game,
        );
    }
    for name in &records {
        let at = records_at().join(name);
        let text =
            std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        add(
            fold(&text, &schema).unwrap_or_else(|why| panic!("reviewed/{name}: {why}")),
            false,
        );
    }
    let of_game = Names::of(&of_game);
    let of_script = Names::of(&of_script);

    let mut compared = 0;
    let mut wrong: Vec<String> = Vec::new();
    for name in &records {
        let at = records_at().join(name);
        let text =
            std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        let friendly = fold(&text, &schema).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        let sections = in_a_section(&friendly);

        let generated: Vec<Row> = friendly
            .iter()
            .enumerate()
            .map(|(at, row)| {
                let names = if sections[at] { &of_game } else { &of_script };
                names
                    .foundation(row)
                    .unwrap_or_else(|why| panic!("reviewed/{name}: `{}`: {why}", write(row)))
            })
            .collect();

        let running = rows(&format!("data/foundation/tests/{name}"));
        if generated.len() != running.len() {
            wrong.push(format!(
                "{name}: the record generates {} rows and the engine runs {}",
                generated.len(),
                running.len()
            ));
            continue;
        }
        for (mine, theirs) in generated.iter().zip(running.iter()) {
            if shape(mine) != shape(theirs) {
                wrong.push(format!(
                    "{name}: the record generates `{}` and the engine runs `{}`",
                    write(mine),
                    write(theirs)
                ));
            }
            compared += 1;
        }
    }

    // **Both populations**, so a record set that generated nothing could not pass by having
    // nothing to disagree with.
    assert!(
        compared > 900,
        "only {compared} rows were compared across {} records, which is too few for any of them \
         to be a test",
        records.len()
    );
    assert!(
        wrong.is_empty(),
        "{} row(s) the engine runs are not what the record generates:\n  {}",
        wrong.len(),
        wrong.join("\n  ")
    );
}

/// The six relations no schema declares, named so the row-shaped comparison above is honest.
///
/// **This exists because the obvious stronger check would be wrong.** Asserting byte equality
/// between the generated foundation and the committed one fails on six relations, and the reason
/// is not drift: **they are rule names, nothing declares them as relations, and `Schema::write`
/// has no column order to use.** The notation's own order is alphabetical and the committed files
/// carry a hand's.
///
/// **So this pins the reason rather than the workaround.** If one of these ever gains a
/// `{relation}` declaration, its order becomes reproducible and this fails - which is the signal
/// that the comparison above could be strengthened.
#[test]
fn the_relations_written_in_no_declared_order_are_the_rule_names() {
    let schema = Schema::of(&rows("data/friendly/schema.4x")).expect("a schema");
    let rules = rows("data/friendly/rules.4x");

    let named: BTreeSet<String> = rules
        .iter()
        .filter(|row| row.relation == "rule")
        .filter_map(|row| row.value("name").map(str::to_string))
        .collect();
    assert!(
        named.len() > 5,
        "only {} rules are named, so the check below is about almost nothing: {named:?}",
        named.len()
    );

    // Which of them any test actually writes a row of, and whether the schema declares it.
    let mut undeclared: BTreeMap<String, usize> = BTreeMap::new();
    let mut seen = 0;
    for name in names_in(&records_at()) {
        for row in rows(&format!("data/foundation/tests/{name}")) {
            seen += 1;
            if named.contains(&row.relation) && schema.relation(&row.relation).is_none() {
                *undeclared.entry(row.relation.clone()).or_default() += 1;
            }
        }
    }
    assert!(
        seen > 900,
        "only {seen} rows were looked at, so a relation could be missed by there being none"
    );

    let which: Vec<&String> = undeclared.keys().collect();
    assert_eq!(
        which,
        [
            "breed",
            "build-bin",
            "build-extractor",
            "build-pioneer",
            "deploy",
            "discard-disorder",
            "end-turn",
            "gather",
            "launch",
            "move",
            "refresh",
            "toil",
            "upkeep",
            "work"
        ],
        "these are the relations a test names that no `{{relation}}` row declares, so their \
         column order is the notation's rather than the schema's"
    );
}
