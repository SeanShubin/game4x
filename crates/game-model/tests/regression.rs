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

use std::collections::{BTreeMap, BTreeSet};

#[path = "../examples/scenario.rs"]
#[allow(dead_code)]
mod scenario;

use scenario::Case;

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
    // **The name travels beside the diff**, because the message below prints the deletion for
    // each grain and a deletion is composed from the path. Formatting it into one string first
    // and parsing it back out is the shape `S-195` is about.
    let mut stale: Vec<(String, String)> = Vec::new();
    for Case {
        name,
        text: produced,
        ..
    } in &cases
    {
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
                    let said = match differs {
                        Some((line, (was, now))) => format!(
                            "{name} line {}:\n      was  {was}\n      now  {now}",
                            line + 1
                        ),
                        None => format!(
                            "{name}: {} line(s) committed, {} produced",
                            committed.lines().count(),
                            produced.lines().count()
                        ),
                    };
                    stale.push((name.clone(), said));
                }
            }
            // **Absent: accepted.** His words - *absent expected data means I accept what it does
            // now, so the test writes it, and what I review is the diff in version control.*
            Err(_) => {
                // **The turn's directory is made if it is not there**, because a case's name is
                // now `NN/MM-rule.4x` and deleting a whole turn takes the directory with it.
                if let Some(turn) = path.parent() {
                    std::fs::create_dir_all(turn)
                        .unwrap_or_else(|why| panic!("{}: {why}", turn.display()));
                }
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

    // **The gesture the message is asking for is printed rather than described** - `S-206`, and
    // `D-6`: *when a failure names stale cases it prints the deletion for each grain, so I paste
    // it rather than compose it.* Sean, 2026-09-27: *I need it to be very clear to distinguish
    // between them so that I delete the correct directory.*
    //
    // **Three grains, because the suite has three.** One case, one turn, the whole suite. The
    // turn grain is listed only for turns that actually hold a stale case, so pasting the whole
    // block never accepts a turn nothing moved in.
    //
    // **Grouped by turn as well**, which `S-206` offered and nobody asked for: a rule changing
    // moves several turns at once, and a flat list of twelve is how somebody deletes more than
    // they meant.
    if !stale.is_empty() {
        let mut by_turn: BTreeMap<String, Vec<&str>> = BTreeMap::new();
        for (name, said) in &stale {
            let turn = name.split('/').next().unwrap_or_default().to_string();
            by_turn.entry(turn).or_default().push(said);
        }
        let mut out = String::new();
        for (turn, said) in &by_turn {
            out.push_str(&format!("\n  turn {turn}\n"));
            for one in said {
                out.push_str(&format!("    {one}\n"));
            }
        }
        out.push_str("\n  accept one case:\n");
        for (name, _) in &stale {
            out.push_str(&format!("    Remove-Item regression/scenario/{name}\n"));
        }
        out.push_str("\n  accept a whole turn:\n");
        for turn in by_turn.keys() {
            out.push_str(&format!(
                "    Remove-Item -Recurse regression/scenario/{turn}\n"
            ));
        }
        out.push_str("\n  accept the whole suite:\n    Remove-Item -Recurse regression/scenario\n");
        panic!(
            "{} of {} expectation(s) no longer say what the scenario does.\n\
             **Delete what you are happy with, run again, and read the diff in version \
             control.**\n{out}",
            stale.len(),
            cases.len()
        );
    }

    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. On a fresh
    // checkout everything is written and nothing compared, which is correct and is not evidence.
    assert!(
        compared + written.len() == cases.len(),
        "every case was either compared or written"
    );

    // **A directory per turn, one level deep, and nothing else.** Sean, 2026-09-27: *what about
    // each turn being in a separate directory with a numeric prefix, and each file within a turns
    // directory having a numeric prefix.*
    //
    // # This asserted the opposite until today, and the constraint it held is his too
    //
    // **Sean, 2026-09-27, earlier the same day**: *if I am deleting multiple files rather than
    // one, they need to be in a single directory.* **That reading has not gone away** - it has
    // been traded. Accepting one turn's behaviour is now the contents of one directory, which is
    // better for it; accepting one case in each of five turns is five directories, which is
    // worse. **The gesture that made it worth trading is the one he made**: deleting the whole
    // suite, which under this is five directories rather than thirty-five files.
    //
    // **Checked both ways, because a name and a directory entry are different populations.** Every
    // case is named `NN/...` exactly one level down, and every entry at the top is a directory.
    let mut depths = 0;
    for Case { name, .. } in &cases {
        assert!(
            !name.contains('\\'),
            "`{name}` is written with a backslash, and a case name is a path with `/`"
        );
        assert_eq!(
            name.matches('/').count(),
            1,
            "`{name}` is not exactly one level down, so it is not a turn's directory"
        );
        depths += 1;
    }
    assert_eq!(
        depths,
        cases.len(),
        "every case was asked how deep it sits, and the count is what says so"
    );
    let loose: Vec<String> = std::fs::read_dir(&at)
        .unwrap_or_else(|why| panic!("{}: {why}", at.display()))
        .filter_map(|it| it.ok())
        .filter(|it| !it.path().is_dir())
        .filter_map(|it| it.file_name().to_str().map(str::to_string))
        .collect();
    assert!(
        loose.is_empty(),
        "`regression/scenario/` holds {loose:?} outside any turn, and every case belongs to a turn"
    );

    // **A file is left behind when its command is gone, and that is asked of the command rather
    // than of the name.**
    //
    // **`P-572`**: *the two are told apart by `scenario/main.4x` - a case whose command is still
    // there is waiting on Sean, and a case whose command is gone is waiting on nobody.*
    //
    // # This compared filenames and said something false
    //
    // **It was `found.difference(&wanted)` over names.** `S-195` simulated Sean's next edit against
    // it: inserting one `{move}` renamed 33 of 34 files, so 33 names were on disk and not in the
    // produced set, and it failed with *33 file(s) are of commands the scenario no longer plays -
    // delete them.*
    //
    // **Every one of those commands was still played.** And under `P-572` deleting a case whose
    // command is still played is Sean's approval gesture - so **the message instructed the reader to
    // approve thirty-three behaviours it had not shown them.** That is worse than noise.
    //
    // **A scheme that renames less is not a scheme that renames never**, and the check has to be
    // right when one does - which is what keeps it asking the command rather than the name now
    // that a case's position in its turn is back in the filename and does renumber.
    //
    // **It walks the turn directories**, because the cases are one level down. A turn with no
    // directory contributes nothing, and a directory with no cases is reported by the count below
    // rather than passing quietly.
    let played: BTreeSet<String> = cases.iter().map(|it| it.command.clone()).collect();
    let mut gone: Vec<String> = Vec::new();
    let mut looked = 0;
    let mut found: Vec<std::path::PathBuf> = Vec::new();
    for turn in std::fs::read_dir(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display())) {
        let turn = turn.expect("a readable entry").path();
        if !turn.is_dir() {
            continue;
        }
        for entry in
            std::fs::read_dir(&turn).unwrap_or_else(|why| panic!("{}: {why}", turn.display()))
        {
            found.push(entry.expect("a readable entry").path());
        }
    }
    for path in found {
        if path.extension().and_then(|it| it.to_str()) != Some("4x") {
            continue;
        }
        // **Named with its turn**, so a message about a case says where to find it.
        let name = path
            .parent()
            .and_then(|it| it.file_name())
            .and_then(|it| it.to_str())
            .map(|turn| {
                format!(
                    "{turn}/{}",
                    path.file_name().and_then(|it| it.to_str()).unwrap_or("?")
                )
            })
            .unwrap_or_default();
        let text = std::fs::read_to_string(&path).unwrap_or_else(|why| panic!("{name}: {why}"));

        // **The command a case covers is the line under its `{when}`**, which is where every case
        // states it and is the only thing that identifies what it is about.
        let covers = text
            .lines()
            .skip_while(|line| line.trim() != "{when}")
            .nth(1)
            .map(str::trim)
            .unwrap_or_default()
            .to_string();
        assert!(
            !covers.is_empty(),
            "`{name}` has no command under its `{{when}}`, so nothing says what it is about"
        );
        looked += 1;
        if !played.contains(&covers) {
            gone.push(format!("{name} covers {covers}"));
        }
    }

    assert!(
        gone.is_empty(),
        "{} case(s) cover a command `scenario/main.4x` no longer plays, so they are waiting on \
         nobody and any lane may remove them - `P-572`:\n    {}",
        gone.len(),
        gone.join("\n    ")
    );

    // **Both populations, because either being empty would make this vacuous.** No files on disk and
    // every command would be unplayed and none reported; no commands and every file would be.
    assert_eq!(
        looked,
        cases.len(),
        "every file on disk was asked about and every case has one"
    );
    assert!(!played.is_empty(), "the scenario plays no commands");
}
