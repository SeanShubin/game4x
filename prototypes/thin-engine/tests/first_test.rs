//! The first test, which is `temporary-notes/first-test.md` - and the test itself is data.
//!
//! **`data/test.4x` is the test.** It sets the schema up, initializes the state from `before.4x`,
//! executes the command in `command.4x`, compares what that produced with `expected.4x`, and
//! composes a report. Nothing in this file says any of that; it reads `test.4x` and runs it.
//!
//! **What is left in Rust is the two things that cannot be data**: handing the engine a way to
//! read a file, and asserting that the report says what it should.

use std::path::PathBuf;

use thin_engine::script::{Files, Report, run_test};

mod common;
use common::{mine, rows};

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
    Directory(mine().join("data"))
}

fn report() -> Report {
    run_test(&rows("data/test.4x"), &data()).unwrap_or_else(|why| panic!("{why}"))
}

/// **The whole of the first test: the engine gets from before to expected.**
#[test]
fn the_engine_gets_from_before_to_expected() {
    let report = report();

    // **Printed as well as asserted**, so `cargo test -- --nocapture` shows the report the script
    // composed rather than only the fact that it was right.
    println!("{report}");

    assert!(
        report.same(),
        "the state after the command is not the expected state:\n{report}"
    );
}

/// **The report says what it compared**, so a green test cannot be one that compared nothing.
///
/// **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. If
/// `{state relation:...}` were dropped from `schema.4x`, the comparison would scope to no
/// relations, find no differences, and report success in exactly the same words.
#[test]
fn the_report_says_which_relations_it_compared() {
    let report = report();

    assert_eq!(
        report.compared,
        vec!["adjacency", "residency", "territory", "thing"],
        "all four of the game's relations are state, and all four are compared"
    );
    assert_eq!(report.test, "the-scout-moves-to-an-adjacent-place");
    assert_eq!(report.title, "the-first-test");
}

/// **The report is composed and readable**, which is the step `test.4x` ends with.
#[test]
fn the_report_reads_as_a_report() {
    assert_eq!(
        format!("{}", report()),
        "the-first-test\n  \
           test      the-scout-moves-to-an-adjacent-place\n  \
           compared  adjacency, residency, territory, thing\n  \
           result    as expected"
    );
}

/// **A wrong expectation is reported rather than passed over**, and the report names both rows.
///
/// This is the poison written down: `expected.4x` is replaced by one that says the scout stayed
/// where it was, and the test that would have to notice is this one.
#[test]
fn a_state_that_is_not_expected_is_reported_as_both_rows() {
    struct Wrong(Directory);
    impl Files for Wrong {
        fn read(&self, name: &str) -> Option<String> {
            if name == "expected.4x" {
                // The state before, offered as the state after.
                return self.0.read("before.4x");
            }
            self.0.read(name)
        }
    }

    let report =
        run_test(&rows("data/test.4x"), &Wrong(data())).unwrap_or_else(|why| panic!("{why}"));

    assert!(
        !report.same(),
        "the scout did move, so this is not as expected"
    );
    assert_eq!(report.missing, vec!["{residency id:1 what:1 where:1}"]);
    assert_eq!(report.extra, vec!["{residency id:1 what:1 where:2}"]);
    assert!(
        format!("{report}").contains("NOT as expected"),
        "and the report says so: {report}"
    );
}
