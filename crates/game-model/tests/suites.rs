//! The three suites that are not about a scenario, each checked on its own.
//!
//! **`docs/process.md`**: *every type of thing that is data has a generated suite of its own, and
//! there are four.* `tests/regression.rs` is the first; these are the other three.
//!
//! # One test per suite, which is what `D-6` is vetted on
//!
//! **`D-6`**: *I delete one of them, run, and the diff holds that suite's cases and no others.*
//! A single test over all three would still satisfy that - the suites write into different
//! directories - but it would report the first failure and hide the rest, which is `Q-92`'s
//! lesson in miniature. One test each means a stale `rules/` does not conceal a stale
//! `primitives/`.
//!
//! # Both ways, and the count, on the two suites that have a closed population
//!
//! **`D-6`**: *every case in `rules/` is one of the fifteen rules and every rule has one, and the
//! same both ways for the words in `primitives/`, so a suite cannot be partly built and look
//! finished.* One direction catches a case nothing produces; the other catches a rule nobody
//! filed. **Neither alone is the property** - a suite holding one case for one rule passes the
//! first, and an empty suite passes neither only because the count is asserted too.
//!
//! **`types/` has no such clause and gets the same treatment anyway**, because the argument does
//! not depend on the clause: a relation with no case is the same hole under another name.

use std::collections::{BTreeMap, BTreeSet};

#[path = "../examples/suites.rs"]
#[allow(dead_code)]
mod suites;

use suites::{Case, check, on_disk};

/// The case names a generator produced, and the ones sitting in that directory.
///
/// **Compared as sets rather than counted**, so the message names what is missing rather than
/// saying how many. A count that disagrees tells you to go and look; a set difference is the
/// looking.
fn both_ways(suite: &str, cases: &[Case], floor: usize) {
    let produced: BTreeSet<String> = cases.iter().map(|it| it.name.clone()).collect();
    let found: BTreeSet<String> = on_disk(suite).into_iter().collect();

    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. Two empty
    // sets are equal, so without this the suite would agree with itself while holding nothing.
    assert!(
        produced.len() >= floor,
        "`{suite}` produced {} case(s) and the floor is {floor}, so comparing the two sets \
         would prove almost nothing",
        produced.len()
    );
    assert_eq!(
        produced.len(),
        cases.len(),
        "`{suite}` produced {} case(s) under {} name(s), so two things share a file",
        cases.len(),
        produced.len()
    );

    let missing: Vec<&String> = produced.difference(&found).collect();
    assert!(
        missing.is_empty(),
        "`{suite}` is missing {} case(s) that the data says it should hold: {missing:?} - \
         a suite that is partly built must not look finished",
        missing.len()
    );
    let extra: Vec<&String> = found.difference(&produced).collect();
    assert!(
        extra.is_empty(),
        "`regression/{suite}/` holds {} case(s) the data no longer produces: {extra:?} - \
         they cover nothing and any lane may remove them",
        extra.len()
    );
}

/// **The transformations over the things**, one case per rule of `spec/data/rules.4x`.
#[test]
fn every_rule_has_a_case_and_every_case_is_a_rule() {
    let cases = suites::rules_cases();
    if let Some(why) = check("rules", &cases) {
        panic!("{why}");
    }
    both_ways("rules", &cases, 10);
}

/// **The definitions of the things**, one case per relation of `spec/data/schema.4x`.
#[test]
fn every_thing_has_a_case_and_every_case_is_a_thing() {
    let cases = suites::types_cases();
    if let Some(why) = check("types", &cases) {
        panic!("{why}");
    }
    both_ways("types", &cases, 30);
}

/// **The words the engine implements**, one case per `{primitive}` of `engine.4x`.
#[test]
fn every_word_has_a_case_and_every_case_is_a_word() {
    let cases = suites::primitives_cases();
    if let Some(why) = check("primitives", &cases) {
        panic!("{why}");
    }
    both_ways("primitives", &cases, 40);
}

/// **A suite writes only into its own directory**, which is the half of `D-6` the three tests
/// above cannot state between them.
///
/// **`D-6`**: *I delete one of them, run, and the diff holds that suite's cases and no others.*
/// Each test above checks its own suite, so nothing in them would notice `rules` writing a file
/// into `types`. **This asks the names rather than the outcome of a run**, because the outcome
/// would need a deletion to observe and a deletion is Sean's gesture.
#[test]
fn no_two_suites_would_write_the_same_file() {
    let suites = [
        ("rules", suites::rules_cases()),
        ("types", suites::types_cases()),
        ("primitives", suites::primitives_cases()),
    ];
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut checked = 0;
    for (suite, cases) in &suites {
        for Case { name, .. } in cases {
            assert!(
                !name.contains('/') && !name.contains('\\'),
                "`{suite}` names a case `{name}`, which is a path - a suite of this shape is one \
                 directory deep and deleting it is one gesture"
            );
            assert!(
                seen.insert(format!("{suite}/{name}")),
                "`{suite}/{name}` is produced twice"
            );
            checked += 1;
        }
    }
    let total: usize = suites.iter().map(|(_, it)| it.len()).sum();
    assert_eq!(
        checked, total,
        "every case of every suite was asked, and the count is what says so"
    );
    assert!(
        total > 100,
        "only {total} case(s) across three suites, so this compared almost nothing"
    );
}

/// **Every documented door regenerates every suite** - `S-209`.
///
/// # The failure this exists for, which a green suite could not see
///
/// **`scripts/regression.ps1` named `--test regression` and nothing else.** Deleting
/// `regression/types/` and running it passed, wrote nothing, and left fifty-one files deleted -
/// and **an empty diff reads as *the cases were already current***, which is the one conclusion
/// that must never be available by accident.
///
/// **The workspace gate could not catch it**, because `cargo test --workspace` runs every binary:
/// the suites were always whole by the time anything looked. Only a person following the path
/// `scripts/README.md` promises saw nothing happen. Found by the specification lane making Sean's
/// gesture rather than reading a report of it.
///
/// # What is asserted, and what it cannot reach
///
/// **That each script names each binary that writes under `regression/`.** The binaries are read
/// from `tests/` by which suite each one writes, rather than listed here, so a fifth suite in a
/// third binary fails this instead of being quietly uncovered.
///
/// **It reads the script rather than running it**, which is the limit: a script that names both
/// binaries and is broken some other way passes. That is why the scripts carry the command on one
/// line - `hooks/pre-push` records a continuation arriving as a literal backslash-n and a run of
/// 0 of 638 tests - and why this asserts the line is whole.
#[test]
fn every_documented_door_runs_every_binary_that_writes_a_suite() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    // **Which binary writes which suite, read rather than listed.** A test that writes a suite
    // says so by naming the directory it writes into.
    let mut writes: BTreeMap<String, String> = BTreeMap::new();
    for entry in std::fs::read_dir(root.join("crates/game-model/tests")).expect("tests/") {
        let path = entry.expect("an entry").path();
        if path.extension().and_then(|it| it.to_str()) != Some("rs") {
            continue;
        }
        let stem = path.file_stem().and_then(|it| it.to_str()).unwrap_or("");
        let text = std::fs::read_to_string(&path).expect("a test file");
        // **Built rather than written, so this line is not its own match.** Spelled out, the
        // probe for `scenario` found itself here and every suite resolved to this file - which
        // is `CLAUDE.md`'s *quoting a thing and doing it are the same bytes*, arriving in the
        // check written to catch a different instance of the same week's lesson. The `check`
        // probe needs no such care: `{suite}` is interpolated, so the string it looks for is
        // never a literal in this file.
        let scenario_probe = format!("regression{}()", "_at");
        for suite in ["scenario", "rules", "types", "primitives"] {
            let writes_it = text.contains(&format!("check(\"{suite}\""))
                || (suite == "scenario" && text.contains(&scenario_probe));
            if writes_it {
                writes.insert(suite.to_string(), stem.to_string());
            }
        }
    }
    assert_eq!(
        writes.len(),
        4,
        "four suites should each have a binary that writes them, and this found {writes:?}"
    );

    let binaries: BTreeSet<&String> = writes.values().collect();
    assert!(
        binaries.len() > 1,
        "every suite is written by one binary, so a script naming it cannot miss one - \
         this check would pass over a population it cannot fail on"
    );

    let doors = ["scripts/regression.sh", "scripts/regression.ps1"];
    let mut asked = 0;
    for door in doors {
        let text =
            std::fs::read_to_string(root.join(door)).unwrap_or_else(|why| panic!("{door}: {why}"));
        let line = text
            .lines()
            .find(|it| it.trim_start().starts_with("cargo test"))
            .unwrap_or_else(|| panic!("{door} runs no cargo test"));
        // **A backslash-n that is two characters rather than a newline**, which is how
        // `hooks/pre-push` ran 0 of 638 tests, and a continuation of either shell's kind.
        let mangled = format!("{}n", char::from(92));
        assert!(
            !line.contains(&mangled) && !line.ends_with(char::from(92)) && !line.ends_with('`'),
            "{door}'s cargo line is continued or carries a literal backslash-n, and a stray \
             word reaching `cargo test` is the test-name filter:\n  {line}"
        );
        for binary in &binaries {
            assert!(
                line.contains(&format!("--test {binary}")),
                "{door} does not run `--test {binary}`, which writes {:?} - so deleting that \
                 suite and running this script would pass and write nothing",
                writes
                    .iter()
                    .filter(|(_, it)| it == binary)
                    .map(|(suite, _)| suite.clone())
                    .collect::<Vec<String>>()
            );
            asked += 1;
        }
    }
    assert_eq!(
        asked,
        doors.len() * binaries.len(),
        "every door was asked about every binary, and the count is what says so"
    );
}
