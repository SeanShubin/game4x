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

use std::path::PathBuf;

use thin_engine::script::{Files, Report, run_test};

mod common;
use common::{every_test, mine, rows};
use thin_engine::notation::Row;

/// The directory, as something the engine can ask for a file by name.
///
/// **This is where `std::fs` lives and the only place it may.** `src/` reads nothing - it is
/// handed this and asks it, which is what lets `load` exist without the engine opening anything.
struct Directory(PathBuf);

impl Files for Directory {
    fn read(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(name)).ok()
    }
}

fn data() -> Directory {
    Directory(mine().join("data").join("foundation"))
}

/// One test's rows: what every test loads, then the test itself.
fn script_of(file: &str) -> Vec<Row> {
    let mut all = rows("data/foundation/setup.4x");
    all.extend(rows(file));
    all
}

fn report_of(file: &str) -> Report {
    run_test(&script_of(file), &data()).unwrap_or_else(|why| panic!("{file}: {why}"))
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
    for directory in ["foundation", "friendly"] {
        for file in every_test() {
            let named = file.replace("data/foundation/", &format!("data/{directory}/"));
            let text = std::fs::read_to_string(mine().join(&named)).expect(&named);
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
    // pass every assertion above and be caught here.
    assert_eq!(
        checked,
        every_test().len() * 8,
        "four marks in each test, in each of the two directories"
    );
}

/// **Every test in the directory gets from its `given` to its `then`.**
///
/// **Nothing here names a test**, so adding one is adding a file and this does not change.
#[test]
fn every_test_gets_from_its_given_to_its_then() {
    for file in every_test() {
        let report = report_of(&file);

        // **Printed as well as asserted**, so `cargo test -- --nocapture` shows the report the
        // script composed rather than only the fact that it was right.
        println!("{report}");

        assert!(
            report.same(),
            "{file}: the state after the command is not the `then` state:\n{report}"
        );
    }
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
        vec!["adjacency", "residency", "territory", "thing"],
        "all four of the game's relations are state, and all four are compared"
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
           compared  adjacency, residency, territory, thing\n  \
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
    // poison is a row rather than a file. The last residency is the `then` one.
    let mut script = script_of("data/foundation/tests/the-scout-moves-to-an-adjacent-place.4x");
    let at = script
        .iter()
        .rposition(|row| row.relation == "residency")
        .expect("a `then` residency");
    script[at] = thin_engine::notation::read("{residency what:1 where:1 quantity:1}")
        .expect("the state before, offered as the state after")
        .remove(0);

    let report = run_test(&script, &data()).unwrap_or_else(|why| panic!("{why}"));

    assert!(
        !report.same(),
        "the scout did move, so this is not as expected"
    );
    assert_eq!(
        report.missing,
        vec!["{residency what:1 where:1 quantity:1}"]
    );
    assert_eq!(report.extra, vec!["{residency what:1 where:2 quantity:1}"]);
    assert!(
        format!("{report}").contains("NOT as expected"),
        "and the report says so: {report}"
    );
}
