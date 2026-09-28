//! The tests are data, and this runs every one of them.
//!
//! **`data/foundation/tests/` holds one test per file and nothing else.** Each states its name and
//! its three sections; `setup.4x` says what every test reads before it runs. Sean, 2026-09-15:
//! *the supporting infrastructure should eat up no more than one line per test file*, and that
//! line is `{test name:...}`.
//!
//! **Adding a test is adding a file.** Nothing here names one, so nothing here changes.
//!
//! **What is left in Rust is the two things that cannot be data**: handing the engine a way to
//! read a file, and asserting that the report says what it should.

use std::collections::{BTreeMap, BTreeSet};

use game_model::script::{Files, Report, run_test};

mod common;
use common::{every_read_test, every_test, mine, rows};
use game_model::notation::Row;

/// The foundation, as something the engine can ask for a file by name.
///
/// **This is where `std::fs` lives and the only place it may.** `src/` reads nothing - it is
/// handed this and asks it, which is what lets `load` exist without the engine opening anything.
///
/// **`setup.4x` asks for `schema.4x`, `engine.4x` and `rules.4x` by bare name**, and says nothing
/// about where any of them is. So the answer to *where is this file* lives in
/// `common::foundation_at` rather than in a `join` here.
///
/// **That is what let `P-563` move two of them into another column and `P-576` move them back**,
/// each without touching the data - and the engine was untouched by either. `Files::read` is the
/// only thing it needs from outside itself, and it still asks by name and gets text.
struct Foundation;

impl Files for Foundation {
    fn read(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(mine().join(common::foundation_at(name))).ok()
    }
}

fn data() -> Foundation {
    Foundation
}

/// One test's rows: what every test loads, then the test itself.
fn script_of(file: &str) -> Vec<Row> {
    let mut all = rows(&common::foundation_at("setup.4x"));
    all.extend(rows(file));
    all
}

fn report_of(file: &str) -> Report {
    run_test(&script_of(file), &data()).unwrap_or_else(|why| panic!("{file}: {why}"))
}

/// Every relation the schema marks as state, by name.
fn state_relations() -> BTreeSet<String> {
    let schema = rows(&common::foundation_at("schema.4x"));
    let named: BTreeMap<String, String> = schema
        .iter()
        .filter(|row| row.relation == "relation")
        .filter_map(|row| Some((row.value("id")?.to_string(), row.value("name")?.to_string())))
        .collect();
    schema
        .iter()
        .filter(|row| row.relation == "state")
        .filter_map(|row| row.value("relation"))
        .filter_map(|it| named.get(it).cloned())
        .collect()
}

/// Every relation the structure declares, by name.
fn declared_relations() -> BTreeSet<String> {
    rows(&common::foundation_at("schema.4x"))
        .iter()
        .filter(|row| row.relation == "relation")
        .filter_map(|row| row.value("name").map(str::to_string))
        .collect()
}

/// Every rule the ruleset declares, by name.
fn rule_names() -> BTreeSet<String> {
    rows(&common::foundation_at("rules.4x"))
        .iter()
        .filter(|row| row.relation == "rule")
        .filter_map(|row| row.value("name").map(str::to_string))
        .collect()
}

/// **A scenario states a world and an act, and nothing else.**
///
/// **This is the boundary between the scenario layer and the ruleset**, and it had nothing on it.
/// A test's `given` could declare a category, a rule and a role and every test still passed - which
/// is how two tests came to use `thing id:1` for `scout` and for `labor`. **That was fixed as an
/// instance and not as a class**, by moving the categories out; this is the class.
///
/// **It needs no new data.** `{state relation:...}` already says which relations a world is made
/// of, and `{rule name:...}` says what may be commanded. A `given` or a `then` holds the first; a
/// `when` holds the second.
#[test]
fn a_scenario_states_a_world_and_an_act_and_nothing_else() {
    let state = state_relations();
    let rules = rule_names();
    let declared = declared_relations();
    assert!(
        !state.is_empty() && !rules.is_empty(),
        "nothing is declared, so nothing would be checked"
    );

    let mut checked = 0;
    for file in every_test() {
        let mut section = String::new();
        for row in rows(&file) {
            if matches!(row.relation.as_str(), "given" | "when" | "then" | "refused") {
                section = row.relation.clone();
                continue;
            }
            if row.relation == "test" {
                continue;
            }
            if section == "when" {
                assert!(
                    rules.contains(&row.relation),
                    "{file}: `{}` is in a `when` and names no rule - a command is a rule fired",
                    row.relation
                );
            } else if section == "refused" {
                // **A `refused` row is an assertion and not a world.** What a rule needed may be
                // a rule's own declaration rather than a fact of the place - a crowded territory
                // is refused for want of a bigger `{pool ...}`, which is the ruleset's. **Nothing
                // here is loaded into a store**: these rows are written out and compared as text,
                // so a scenario naming one declares nothing and the layer still holds.
                assert!(
                    declared.contains(&row.relation),
                    "{file}: `{}` is in a `refused` and is not a relation at all",
                    row.relation
                );
            } else {
                assert!(
                    state.contains(&row.relation),
                    "{file}: `{}` is in a `{section}` and is not state - a scenario states a world, \
                     and what a category or a rule is belongs to the ruleset",
                    row.relation
                );
            }
            checked += 1;
        }
    }
    assert!(
        checked > 20,
        "only {checked} rows were placed, so a pass proves little"
    );
}

/// **The ruleset states no world.**
///
/// The mirror of the check above: a `{residency ...}` in `rules.4x` is a world smuggled into the
/// rules, and it would quietly become part of every test's starting state.
#[test]
fn the_ruleset_states_no_world() {
    let state = state_relations();
    let mut checked = 0;
    for file in ["schema.4x", "engine.4x", "rules.4x"] {
        for row in rows(&common::foundation_at(file)) {
            assert!(
                !state.contains(&row.relation),
                "{file}: `{}` is state, and a world belongs to the scenario that states it",
                row.relation
            );
            checked += 1;
        }
    }
    assert!(
        checked > 100,
        "only {checked} rows were read, so a pass proves little"
    );
}

/// **A test's sections are set apart by a blank line**, in both directories.
///
/// **Sean's spacing, 2026-09-15**, and a check rather than a habit: he wrote it into the friendly
/// files and the generator would have taken it straight back out, because `render` copies the
/// foundation's blank lines and the foundation had none. **It is in both now**, and this is what
/// says so when the next test is written.
///
/// **Whitespace carries nothing to the engine** - `{given}` is a marker and the rows after it are
/// its section however they are spaced. This is about the one reader who is not the engine.
#[test]
fn a_test_sets_its_sections_apart() {
    let mut checked = 0;
    // **Both spellings of a test, and one of them left the prototype on 2026-09-21.** `P-532`
    // put the friendly side in `spec/tests/`, where it is the specification; the foundation side
    // stayed, being converted from it. **The sections have to be apart in both**, because a
    // reader reads one and the engine runs the other.
    // **A test may be in one spelling and not the other, and that is reported elsewhere.**
    // `reviewed/` can hold a record whose source Sean has deleted - `51a13065` - and
    // `spec/tests/` can hold a source he has not read yet, which has no generated form until he
    // does. **Both are states the process allows**, and both have a check of their own:
    // `every_record_of_a_reading_names_a_test_that_is_there` and
    // `every_reading_reaches_the_suite_and_everything_the_suite_runs_was_read`.
    //
    // **So a missing counterpart is skipped here rather than panicked on.** This asserted the
    // three directories were one population and took the suite down when they stopped being -
    // which said nothing this file is about, and hid what it is about behind a file-not-found.
    let mut skipped = 0;
    for directory in ["data/foundation/tests", "../../spec/tests"] {
        for file in every_test() {
            let named = file.replace("data/foundation/tests", directory);
            let Ok(text) = std::fs::read_to_string(mine().join(&named)) else {
                skipped += 1;
                continue;
            };
            let lines: Vec<&str> = text.lines().collect();
            for (at, line) in lines.iter().enumerate() {
                let opens = matches!(line.trim(), "{given}" | "{when}" | "{then}" | "{refused}");
                let names = line.starts_with("{test ");
                if opens {
                    assert!(
                        at > 0 && lines[at - 1].trim().is_empty(),
                        "{named}: `{line}` wants a blank line before it"
                    );
                    checked += 1;
                }
                if names {
                    assert!(
                        lines
                            .get(at + 1)
                            .map(|next| next.trim().is_empty())
                            .unwrap_or(false),
                        "{named}: `{line}` wants a blank line after it"
                    );
                    checked += 1;
                }
            }
        }
    }
    // **Four a file and two directories**, so a test file that stated no sections at all would
    // pass every assertion above and be caught here. **Less what was skipped**, so the count
    // still says every file present was read and a skip cannot hide an unchecked one.
    assert_eq!(
        checked + skipped * 4,
        every_test().len() * 8,
        "four marks in each test, in each of the two directories; {skipped} file(s) were in one \
         spelling and not the other"
    );
    // **And a skip is a state, not a condition to live in.** Every one is an orphaned record or
    // an unread test, and both have a check that names them - so this only refuses the case where
    // the two directories have stopped overlapping at all.
    assert!(
        skipped * 4 < every_test().len(),
        "{skipped} of {} file(s) were in one spelling only, which is too many for the two \
         directories to be about the same tests",
        every_test().len()
    );
}

/// **Every test in the directory gets from its `given` to its `then`.**
///
/// **Nothing here names a test**, so adding one is adding a file and this does not change.
#[test]
fn every_test_gets_from_its_given_to_its_then() {
    // **Every red, not the first one.** Panicking inside the loop hid the rest, which is the wrong
    // shape for red/green/refactor: a run that is meant to be red is a run where you want to see
    // every test that is.
    let mut red = Vec::new();
    // **The read ones, and the unread ones are named rather than skipped in silence.** `S-149`:
    // `CLAUDE.md` says a test nobody has read constrains nothing, and a test that quietly does not
    // run looks exactly like one that passed.
    let (reading, unread) = every_read_test();
    if !unread.is_empty() {
        println!(
            "{} of {} tests have not been read and did not run: {}",
            unread.len(),
            every_test().len(),
            unread.join(", ")
        );
    }
    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. With every
    // record missing this would run no test and say every one of them reached its `then`.
    assert!(
        reading.len() > 40,
        "only {} of {} tests have been read, so a pass proves almost nothing - is reviewed/ there?",
        reading.len(),
        every_test().len()
    );
    for file in reading.clone() {
        // **A test that will not run is red too**, and unwrapping here hid every test after the
        // first one that could not.
        match run_test(&script_of(&file), &data()) {
            Err(why) => red.push(format!("{file}\n  refused   {why}")),
            Ok(report) => {
                // **Printed as well as asserted**, so `cargo test -- --nocapture` shows the report
                // the script composed rather than only the fact that it was right.
                println!("{report}");
                if !report.same() {
                    red.push(format!("{file}\n{report}"));
                }
            }
        }
    }

    assert!(
        red.is_empty(),
        "{} of {} read tests did not reach their `then`:\n\n{}",
        red.len(),
        reading.len(),
        red.join("\n\n")
    );
}

/// **The report says what it compared**, so a green test cannot be one that compared nothing.
///
/// **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. If
/// `{state relation:...}` were dropped from `schema.4x`, the comparison would scope to no
/// relations, find no differences, and report success in exactly the same words.
#[test]
fn the_report_says_which_relations_it_compared() {
    let report = report_of("data/foundation/tests/the-scout-moves-to-an-adjacent-place.4x");

    assert_eq!(
        report.compared,
        vec![
            "adjacency",
            "ark",
            "bin",
            "capacity",
            "citizen",
            "consumes",
            "deposit",
            "energy",
            "extractor",
            "food",
            "labor",
            "metal",
            "pioneer",
            "place",
            "provides",
            "scout",
            "terrain",
            "territory",
            "transport",
            "yard"
        ],
        "twenty of the game's relations are state - an allowance is a column of the thing now, so `working` is not one of them; `terrain` is one since `P-578` and `yard` since `P-583`"
    );
    assert_eq!(report.test, "the-scout-moves-to-an-adjacent-place");
}

/// **The report is composed and readable**, and a test's name is its title.
///
/// **There were two names for one test** - `{test name:...}` and `{report title:...}` - and the
/// second went with the `report` relation. A test that is one file needs one name.
#[test]
fn the_report_reads_as_a_report() {
    assert_eq!(
        format!(
            "{}",
            report_of("data/foundation/tests/the-scout-moves-to-an-adjacent-place.4x")
        ),
        "the-scout-moves-to-an-adjacent-place\n  \
           compared  adjacency, ark, bin, capacity, citizen, consumes, deposit, energy, extractor, food, labor, metal, pioneer, place, provides, scout, terrain, territory, transport, yard\n  \
           result    as expected"
    );
}

/// **A wrong expectation is reported rather than passed over**, and the report names both rows.
///
/// This is the poison written down: the `then` section is rewritten to say the scout stayed where
/// it was, and the test that would have to notice is this one.
///
/// **The poison moved with the file.** It used to swap `expected.4x` for `given.4x`; there are no
/// such files now, so it edits the one file's `then` section instead - which is a truer poison,
/// because a wrong expectation is exactly a wrong `then`.
#[test]
fn a_state_that_is_not_expected_is_reported_as_both_rows() {
    // **The script is handed in, not read through `Files`** - the sections are in it, so the
    // poison is a row rather than a file. The last scout is the `then` one.
    let mut script = script_of("data/foundation/tests/the-scout-moves-to-an-adjacent-place.4x");
    let at = script
        .iter()
        .rposition(|row| row.relation == "scout")
        .expect("a `then` scout");
    script[at] = game_model::notation::read("{scout where:1 moving:1 quantity:1}")
        .expect("the state before, offered as the state after")
        .remove(0);

    let report = run_test(&script, &data()).unwrap_or_else(|why| panic!("{why}"));

    assert!(
        !report.same(),
        "the scout did move, so this is not as expected"
    );
    assert_eq!(report.missing, vec!["{scout where:1 moving:1 quantity:1}"]);
    assert_eq!(report.extra, vec!["{scout where:2 moving:0 quantity:1}"]);
    assert!(
        format!("{report}").contains("NOT as expected"),
        "and the report says so: {report}"
    );
}
