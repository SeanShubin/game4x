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
use game_model::notation::{Row, write};
use game_model::schema::Schema;

use common::{mine, rows};

/// **The converter, borrowed rather than copied** - it is the one place that knows where the
/// friendly source of each file lives, and `P-576` moved two of the five into `spec/data/`.
#[path = "../examples/render.rs"]
#[allow(dead_code)]
mod render;

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

// **`render::under` lists a directory of suites** - `S-256`. This was `names_in`, one of five copies of *list the `.4x` files here*.
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
    let schema = Schema::of(&rows(&render::friendly_at("schema.4x"))).expect("a schema");

    // **Only an approved record generates** - `spec/README.md` rule 3 since `P-605`. This ran
    // every file in the directory, which was right while presence meant both *I read this* and
    // *this binds*; **a denied test is now a test the code is not bound by**, and running it is
    // the failure that looks exactly like nothing being wrong.
    let approved: BTreeSet<String> = render::under(&records_at())
        .into_iter()
        .filter(|name| {
            let said = std::fs::read_to_string(records_at().join(name))
                .unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
            match render::verdict_of(&said).unwrap_or_else(|why| panic!("reviewed/{name}: {why}")) {
                render::Verdict::Approved => true,
                render::Verdict::Denied => false,
            }
        })
        .collect();
    let records = approved;
    let drafts = render::under(&tests_at());
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
    // **The five shared files come from `render::files()`**, which is what the conversion walks,
    // so a sixth cannot exist here and not there.
    for (file, game) in render::files()
        .into_iter()
        .filter(|(f, _)| !f.starts_with("tests/"))
    {
        let at = render::friendly_at(&file);
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
            render::folded_record(&text, &schema)
                .unwrap_or_else(|why| panic!("reviewed/{name}: {why}")),
            false,
        );
    }
    let of_game = Names::of(&of_game);
    let of_script = Names::of(&of_script);

    // **At least one record is in the format the readers are written for**, or every reader that
    // drops the verdict line is exercised over nothing and passes for the wrong reason.
    //
    // **This is the floor the bug walked under.** Fifty-six records predate `P-605` and carry no
    // verdict, so `fold` over a record never met one - and the day the first did, the suite said
    // *the record generates 13 rows and the engine runs 12*, the thirteenth being the verdict.
    // **Nothing was wrong with the reader until there was something for it to read.**
    let carrying = records
        .iter()
        .filter(|name| {
            std::fs::read_to_string(records_at().join(name))
                .is_ok_and(|it| it.starts_with("{verdict"))
        })
        .count();
    assert!(
        carrying > 0,
        "no record carries a verdict, so dropping one is checked over nothing"
    );

    let mut compared = 0;
    let mut wrong: Vec<String> = Vec::new();
    for name in &records {
        let at = records_at().join(name);
        let text =
            std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        let friendly = render::folded_record(&text, &schema)
            .unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
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

    // **The findings before the floor**, because the floor masked the thing it was added to
    // protect. On 2026-10-02 every one of fifty-nine records disagreed with its generated
    // foundation, every comparison took the `continue` above, and the test said **only 0 rows
    // were compared** - a sentence about itself - while the fifty-nine messages saying *what*
    // differed sat in the assertion underneath and never printed.
    //
    // **A floor that fires first turns a diagnosis into a silence.** Both are still asserted
    // and only the order moved: a reader learns what disagrees, then whether there was enough
    // to disagree about.
    assert!(
        wrong.is_empty(),
        "{} row(s) the engine runs are not what the record generates:\n  {}",
        wrong.len(),
        wrong.join("\n  ")
    );

    // **Both populations**, so a record set that generated nothing could not pass by having
    // nothing to disagree with.
    assert!(
        compared > 900,
        "only {compared} rows were compared across {} records, which is too few for any of them \
         to be a test",
        records.len()
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
    let schema = Schema::of(&rows(&render::friendly_at("schema.4x"))).expect("a schema");
    let rules = rows(&render::friendly_at("rules.4x"));

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
    for name in render::under(&records_at()) {
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
            "build-yard",
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

/// **A verdict is read, and `denied` keeps a test out of the suite.**
///
/// `P-605`, `spec/README.md` rule 3: *no record means I have not looked; a record saying
/// `approved` means the code is bound by it; a record saying `denied` means it is not.*
///
/// # Why this is driven rather than observed
///
/// **No record in the tree is denied today**, so every reader that now consults a verdict is
/// taking the same branch it took before and nothing would notice if the other branch were
/// wrong. **The one to fear is the suite running a denied test** - a failure that looks exactly
/// like nothing being wrong, because a denied test either goes red where the code is correct or
/// goes green and records agreement with something he rejected.
///
/// **So the three states are driven over text rather than over the directory.** The directory is
/// what cannot be arranged: `reviewed/` is written by the review application acting as Sean, and
/// a test that wrote a denial there to watch it being skipped would be a lane writing a verdict.
#[test]
fn a_denied_record_is_read_as_denied_and_an_old_one_as_approved() {
    let rows = "{test name:x}\n\n{given}\n{scout where:place-1} -> 1\n";

    // **A record from before `P-605` has no verdict row and is approved**, because presence
    // meant both halves of the weld under the rule it was written under. All 57 in the tree are
    // this shape today.
    assert_eq!(
        render::verdict_of(rows),
        Ok(render::Verdict::Approved),
        "a record with no verdict is what every record was until P-605"
    );
    assert_eq!(
        render::verdict_of(&format!("{{verdict state:approved}}\n{rows}")),
        Ok(render::Verdict::Approved)
    );
    assert_eq!(
        render::verdict_of(&format!("{{verdict state:denied}}\n{rows}")),
        Ok(render::Verdict::Denied)
    );

    // **A state nobody defined is refused rather than guessed.** Treating it as approved would
    // bind the code on a word with no meaning; treating it as denied would silently drop a test
    // he approved. **Neither is available, so it fails.**
    assert!(render::verdict_of(&format!("{{verdict state:pending}}\n{rows}")).is_err());
    assert!(render::verdict_of(&format!("{{verdict}}\n{rows}")).is_err());

    // **Two verdicts in one record say more than one thing about one test.**
    assert!(
        render::verdict_of(&format!(
            "{{verdict state:approved}}\n{{verdict state:denied}}\n{rows}"
        ))
        .is_err()
    );

    // **And every record in the tree reads**, which is the population this ranges over for real:
    // a reader that errored on the records that exist would have failed the suite above, and this
    // says so rather than leaving it to that.
    let mut read = 0;
    for name in render::under(&records_at()) {
        let said = std::fs::read_to_string(records_at().join(&name)).expect("a record");
        render::verdict_of(&said).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        read += 1;
    }
    assert!(read >= FLOOR, "only {read} record(s) were read");
}
