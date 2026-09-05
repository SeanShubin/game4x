//! The turn report is sufficient to reconstruct the next state.
//!
//! **`S-39`, and the property is the quality lens's.** *A delta that is correct and omits
//! something, and a command list that is correct and incomplete, look exactly like a good
//! report.* Nothing in the report says otherwise - and it is the artifact Sean uses to check
//! the state function, so a gap in it reads as a gap in the game.
//!
//! # Only one of the two checks it suggested is buildable, and the other must not be built
//!
//! The lens named the trap with the check: **if the generator produces the states and the
//! delta by the same path, comparing them proves only that the path is consistent with
//! itself.**
//!
//! It does. `expected::rows` renders a state and `expected::compare` differences two of
//! them, and both the printed delta *and* the printed state come from `dump::tables`. There
//! is no second derivation available - the model's own state comparison **is** `compare`.
//! So *the delta equals the difference of the two states* would compare a path to itself and
//! report green whatever either said. **It is not built, and that is the finding rather than
//! a gap.**
//!
//! The other one is real, because the second derivation is the model. **Replaying the
//! commands the report prints must reach the state the report prints.** A command the report
//! failed to list did work that shows up in the state, so the replay diverges and says
//! where.
//!
//! **And it reads the commands out of `turns.md` rather than out of memory**, which is what
//! makes it about the artifact. Replaying the in-memory list would test that a `Vec` equals
//! itself; replaying what was *written* tests the thing somebody reads.

use std::path::PathBuf;

use game_console::{Library, Session, expected};

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The commands each turn of the report lists, in order.
fn commands_per_turn(report: &str) -> Vec<Vec<String>> {
    let mut turns = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut inside = false;
    for line in report.lines() {
        if line.trim() == "```" {
            if inside {
                turns.push(std::mem::take(&mut current));
            }
            inside = !inside;
            continue;
        }
        if inside {
            current.push(line.trim().to_string());
        }
    }
    turns
}

#[test]
fn replaying_what_the_report_lists_reaches_what_the_report_shows() {
    let root = root();
    let files = Files(root.join("commands"));
    let report = std::fs::read_to_string(root.join("turns.md")).expect("turns.md is generated");
    let listed = commands_per_turn(&report);

    assert!(
        listed.len() > 1,
        "only {} turns found in turns.md; its shape has changed",
        listed.len()
    );

    // The scenario as the report tells it, and the scenario as the file runs it, must be
    // the same game at every turn boundary.
    let mut replayed = Session::new();
    for line in ["run setup", "start"] {
        replayed.run(line, &files).expect("the setup runs");
    }
    let mut played = Session::new();
    for line in ["run setup", "start"] {
        played.run(line, &files).expect("the setup runs");
    }

    let scenario = files.fetch("play").expect("commands/play.4x");
    let mut whole: Vec<String> = scenario
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_string)
        .collect();
    whole.reverse();

    let mut ran = 0usize;
    for (at, turn) in listed.iter().enumerate() {
        assert!(!turn.is_empty(), "turn {} lists no commands at all", at + 1);
        for line in turn {
            replayed
                .run(line, &files)
                .unwrap_or_else(|why| panic!("turn {}: `{line}` failed: {why}", at + 1));
            ran += 1;
        }
        // The same number of commands from the file, so a listed turn that dropped one
        // shows as the two games parting rather than as a shorter list.
        for _ in 0..turn.len() {
            let line = whole.pop().unwrap_or_default();
            if !line.is_empty() {
                played.run(&line, &files).ok();
            }
        }

        let wrong = expected::compare(
            &expected::rows(&played.game),
            &expected::rows(&replayed.game),
        );
        assert_eq!(
            wrong.total(),
            0,
            "turn {} does not reconstruct: replaying what the report lists reaches a \
             different state from running the scenario.\n{}\n\
             A command the report did not list did work that shows here.",
            at + 1,
            wrong.report()
        );
    }

    assert_eq!(
        ran,
        scenario
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .count(),
        "the report lists {ran} commands and the scenario has more; some ran without being \
         written down, which is exactly what this exists to catch"
    );
}
