//! A generated given/when/then for every command of the main scenario.
//!
//! **Sean, 2026-09-27**: *I want to generate a detailed regression test out of `scenario/main.4x`.
//! For every single command, I want a generated given/when/then test that focuses in that single
//! command... This is an excessive amount of detail, but I won't be looking at all of them, when
//! something changes I will know exactly what changed.*
//!
//! # The pattern is his and it is written down
//!
//! **`docs/process.md`**: *When I change my mind, I delete the expected data and run the scenario
//! again. **Absent expected data means I accept what it does now**, so the test writes it, and what
//! I review is the diff in version control. **Nothing else may write it**: an expectation that can
//! be edited in place is one that can be edited by accident, which is the thing it exists to
//! prevent.*
//!
//! So this does exactly three things and no fourth:
//!
//! ```text
//! absent      write it, and say so. Deleting a file is how he says "this is right now"
//! present     compare, and fail with the first line that differs
//! ever        never overwrite one that is there
//! ```
//!
//! **That last line is the whole of the guarantee.** `scenario/played.md` is a report and is
//! rewritten every run; these are expectations and are not. The two look alike and are opposites.
//!
//! # What is in each case, and the shape that was tried first
//!
//! **`{given}` is what the command took and `{then}` is what it made** - the engine's own account
//! of that one command, and nothing else.
//!
//! **The whole world was the first shape and it was wrong**, measured rather than argued: one
//! density changed from six to seven and **all thirty-four files moved**, because every case
//! carried a `{given}` it had merely inherited from the commands before it. Sean asked for this so
//! that *when something changes I will know exactly what changed*, and thirty-four files is not
//! knowing.
//!
//! **What is lost is that a case is no longer a world that could be run on its own.** That is the
//! trade and it is worth saying: `scenario/played.md` is where the whole world at each turn's end
//! lives, so nothing is unavailable - it is somewhere else.

use std::collections::BTreeSet;

#[path = "../examples/scenario.rs"]
#[allow(dead_code)]
mod scenario;

/// **Every command's case is on disk and says what the scenario does now.**
///
/// **It writes what is missing and compares what is there**, which is `docs/process.md`'s pattern
/// and is why this is one test rather than a generator plus a check: the writing only ever happens
/// where Sean has deleted something, so the two halves cannot get out of step.
#[test]
fn every_command_has_an_expectation_and_it_is_current() {
    let cases = scenario::regression_cases();
    let at = scenario::regression_at();
    std::fs::create_dir_all(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));

    let (mut written, mut compared) = (Vec::new(), 0);
    let mut stale: Vec<String> = Vec::new();
    for (name, produced) in &cases {
        let path = at.join(name);
        match std::fs::read_to_string(&path) {
            // **Present: compared and never rewritten.** A difference is reported with the line, so
            // the message says what moved rather than that something did.
            Ok(committed) => {
                compared += 1;
                if committed != *produced {
                    let differs = committed
                        .lines()
                        .zip(produced.lines())
                        .enumerate()
                        .find(|(_, (was, now))| was != now);
                    stale.push(match differs {
                        Some((line, (was, now))) => format!(
                            "{name} line {}:\n      was  {was}\n      now  {now}",
                            line + 1
                        ),
                        None => format!(
                            "{name}: {} line(s) committed, {} produced",
                            committed.lines().count(),
                            produced.lines().count()
                        ),
                    });
                }
            }
            // **Absent: accepted.** His words - *absent expected data means I accept what it does
            // now, so the test writes it, and what I review is the diff in version control.*
            Err(_) => {
                std::fs::write(&path, produced)
                    .unwrap_or_else(|why| panic!("{}: {why}", path.display()));
                written.push(name.clone());
            }
        }
    }

    if !written.is_empty() {
        println!(
            "wrote {} expectation(s) that were absent; read the diff: {written:?}",
            written.len()
        );
    }

    assert!(
        stale.is_empty(),
        "{} of {} expectation(s) no longer say what the scenario does. **Delete the ones you are \
         happy with and run again**, and read the diff:\n    {}",
        stale.len(),
        cases.len(),
        stale.join("\n    ")
    );

    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. On a fresh
    // checkout everything is written and nothing compared, which is correct and is not evidence.
    assert!(
        compared + written.len() == cases.len(),
        "every case was either compared or written"
    );

    // **And nothing is left behind that the scenario no longer plays**, which is the direction that
    // rots: a command deleted from `scenario/main.4x` leaves its file, where it reads exactly like
    // a case still played - and everything above walks what the scenario produces, so it would
    // never be looked at. **A renumbering does this too**: the cases are named by position, so
    // inserting a command at turn one renames every file after it.
    //
    // **In this test rather than beside it.** It was its own test for an hour and raced with this
    // one over the same directory - two tests, one of which writes, and `cargo test` runs them at
    // once. **The writing and the sweep are one operation and had to be one test.**
    // **One directory and no subdirectories, which is Sean's own constraint.** Sean, 2026-09-27:
    // *if I am deleting multiple files rather than one, they need to be in a single directory.*
    // **Accepting a turn's worth of behaviour is several deletions**, and a file two levels down is
    // one he would have to go and find.
    //
    // **Checked both ways**, because a name and a directory entry are different populations: no
    // case is named with a path in it, and nothing in the directory is a directory.
    for (name, _) in &cases {
        assert!(
            !name.contains('/') && !name.contains('\\'),
            "`{name}` is a path rather than a name, so the cases would not be in one directory"
        );
    }
    let nested: Vec<String> = std::fs::read_dir(&at)
        .unwrap_or_else(|why| panic!("{}: {why}", at.display()))
        .filter_map(|it| it.ok())
        .filter(|it| it.path().is_dir())
        .filter_map(|it| it.file_name().to_str().map(str::to_string))
        .collect();
    assert!(
        nested.is_empty(),
        "`scenario/regression/` holds {nested:?}, and deleting several expectations has to be          several deletions in one directory"
    );

    let wanted: BTreeSet<String> = cases.iter().map(|(name, _)| name.clone()).collect();
    let found: BTreeSet<String> = std::fs::read_dir(&at)
        .unwrap_or_else(|why| panic!("{}: {why}", at.display()))
        .filter_map(|it| it.ok())
        .filter_map(|it| it.file_name().to_str().map(str::to_string))
        .filter(|name| name.ends_with(".4x"))
        .collect();
    let orphaned: Vec<&String> = found.difference(&wanted).collect();
    assert!(
        orphaned.is_empty(),
        "{} file(s) in `scenario/regression/` are of commands the scenario no longer plays:          {orphaned:?} - delete them",
        orphaned.len()
    );
    assert_eq!(
        found.len(),
        wanted.len(),
        "every command the scenario plays has a file, and nothing else does"
    );
}
