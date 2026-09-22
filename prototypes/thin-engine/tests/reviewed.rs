//! Every record of a reading is a record of reading something that is there.
//!
//! **A file in `reviewed/` says Sean has read the test of that name.** `examples/report.rs` looks
//! one up per test, so a record naming no test is walked by nobody and nothing notices it.
//!
//! # Untidy today, and a false approval tomorrow
//!
//! **Two records outlived their tests** - `a-transport-takes-two-berths-where-a-scout-takes-one`,
//! deleted in `4f89b719`, and `an-ark-gathers-an-energy-from-the-sun`. Neither cost an approval:
//! both successors carry records of their own, so what he read he read. **What it costs is ahead
//! of us.** A test given a name a stale record already holds opens as *reviewed* without anyone
//! having read a line of it - and the page would say so in the same words it uses when he has.
//!
//! **Nothing else in the suite can see it.** `review_of` is asked about each test in turn, so it
//! answers for the fifty-three that exist and is never asked about the records that answer to
//! nothing. **A check has to walk the other way**, which is the whole of this file.
//!
//! Found by the specification lane checking an argument of this lane's rather than accepting it -
//! `S-145`, and the argument was that `reviewed/` is the approval record an executable
//! specification would rest on. **A record that can drift is a poor thing to rest on**, which is
//! why this landed the day the argument was made rather than the day it mattered.

use std::collections::BTreeSet;

mod common;
use common::mine;

/// The stem of every `.4x` in a directory, named from this prototype.
///
/// **Both of these left the prototype on 2026-09-21** - `P-532` put the tests in `spec/tests/`
/// and the records in `reviewed/`, at the repository root, where they are the specification and
/// the record of Sean having read it. **What this checks did not change**; only where it looks.
fn stems(under: &str) -> BTreeSet<String> {
    let at = mine().join(under);
    let Ok(entries) = std::fs::read_dir(&at) else {
        return BTreeSet::new();
    };
    entries
        .filter_map(|it| it.ok())
        .map(|it| it.path())
        .filter(|path| path.extension().and_then(|it| it.to_str()) == Some("4x"))
        .filter_map(|path| {
            path.file_stem()
                .and_then(|it| it.to_str())
                .map(str::to_string)
        })
        .collect()
}

#[test]
fn every_record_of_a_reading_names_a_test_that_is_there() {
    let tests = stems("../../spec/tests");
    let records = stems("../../reviewed");

    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. With no
    // records read, every record would name a test vacuously, and this would pass in the same
    // words it passes in now.
    assert!(
        tests.len() > 40,
        "only {} tests were found, so this is not about the suite",
        tests.len()
    );
    assert!(
        records.len() > 40,
        "only {} records were found, so this is not about the reading",
        records.len()
    );

    let orphaned: Vec<&String> = records.difference(&tests).collect();
    assert!(
        orphaned.is_empty(),
        "these say a test was read and there is no such test: {orphaned:?} - a test later given \
         one of those names would open as reviewed without anyone having read it"
    );
}

/// **A note asks for a change to a test, so it is addressed to one that exists.**
///
/// **The same failure with a quieter end.** An orphaned record claims a reading that did not
/// happen; an orphaned note asks for a change to nothing, and shows on no page - so it is a thing
/// Sean asked for that nobody will ever read.
#[test]
fn every_note_is_about_a_test_that_is_there() {
    let tests = stems("../../spec/tests");
    let Ok(text) = std::fs::read_to_string(mine().join("../../reviewed/asked.md")) else {
        return;
    };
    let named: Vec<String> = text
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .map(|name| name.trim().to_string())
        .collect();
    let orphaned: Vec<&String> = named.iter().filter(|it| !tests.contains(*it)).collect();
    assert!(
        orphaned.is_empty(),
        "these ask for a change to a test that is not there: {orphaned:?}"
    );
}
