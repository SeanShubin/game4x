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

/// The printed delta accounts for every row the printed states gained or lost.
///
/// **The check I first declined, and my reason for declining was wrong.** I said the delta
/// and the states share `dump::tables`, so there is no second derivation and comparing them
/// would compare a path to itself. The specification lane pointed out that the lens's
/// caution forbids a second derivation that shares the **computation**, not one that shares
/// the inputs - and reading the printed tables back out of `turns.md` and counting them
/// calls neither `expected::compare` nor `dump::tables`. It runs on the artifact.
///
/// **The real reason to stop short is cost, and it is why this counts rows rather than
/// differencing fields.** The delta prints as items and the state prints as tables, so a
/// full comparison means translating between two representations - and a translator wrong in
/// the same direction as the printer is circular again, only harder to see. Row counts need
/// no translation: a table has so many rows before and so many after, and the delta says how
/// many appeared and vanished. Those must agree.
///
/// So this catches a delta that **omits** a row - `S-39`'s failure - and does not catch one
/// that reports a field wrongly. Field-level differencing is buildable and not built, and
/// the reason is the translator, not circularity.
#[test]
fn the_delta_accounts_for_every_row_the_states_gained_or_lost() {
    let report = std::fs::read_to_string(root().join("turns.md")).expect("turns.md is generated");

    // Rows per table, in each turn's printed state, counted from the text.
    let counts = |state: &str| -> std::collections::BTreeMap<String, usize> {
        let mut out = std::collections::BTreeMap::new();
        let mut table = String::new();
        for line in state.lines() {
            if let Some(name) = line.trim().strip_prefix("### ") {
                table = name.trim().to_string();
                continue;
            }
            let line = line.trim();
            // A body row: starts with `|`, and is not the header or the separator.
            if line.starts_with('|') && !line.contains("---") && !table.is_empty() {
                *out.entry(table.clone()).or_insert(0usize) += 1;
            }
        }
        // The header of each table counted as a row above; take it back off.
        for value in out.values_mut() {
            *value = value.saturating_sub(1);
        }
        out
    };

    let turns: Vec<&str> = report.split("\n# Turn ").skip(1).collect();
    assert!(turns.len() > 1, "only {} turns parsed", turns.len());

    let mut checked = 0usize;
    let mut previous: Option<std::collections::BTreeMap<String, usize>> = None;
    for (at, turn) in turns.iter().enumerate() {
        let state = turn.split("## what is there now").nth(1).unwrap_or("");
        let now = counts(state);
        assert!(!now.is_empty(), "turn {} prints no tables", at + 1);

        if let Some(before) = previous {
            let delta = turn
                .split("## what changed")
                .nth(1)
                .and_then(|rest| rest.split("## what is there now").next())
                .unwrap_or("");

            // How many rows of each table the delta says appeared and vanished. A row reads
            // `- {table field:value ...}` under **new** or **gone**.
            let named = |heading: &str| -> std::collections::BTreeMap<String, usize> {
                let mut out = std::collections::BTreeMap::new();
                let Some(section) = delta.split(heading).nth(1) else {
                    return out;
                };
                for line in section.lines() {
                    let line = line.trim();
                    if line.starts_with("**") {
                        break;
                    }
                    if let Some(row) = line.strip_prefix("- {") {
                        let table = row
                            .split_whitespace()
                            .next()
                            .unwrap_or("")
                            .trim_matches('"')
                            .to_string();
                        *out.entry(table).or_insert(0) += 1;
                    }
                }
                out
            };
            let appeared = named("**new**");
            let vanished = named("**gone**");

            for (table, after) in &now {
                let was = before.get(table).copied().unwrap_or(0);
                let grew = *after as i64 - was as i64;
                let said = appeared.get(table).copied().unwrap_or(0) as i64
                    - vanished.get(table).copied().unwrap_or(0) as i64;
                assert_eq!(
                    grew,
                    said,
                    "turn {}: the `{table}` table went from {was} rows to {after}, and the \
                     delta accounts for {said}. A row changed hands without the delta \
                     saying so, which is what a correct-looking report that omits \
                     something looks like.",
                    at + 1
                );
                checked += 1;
            }
        }
        previous = Some(now);
    }

    // Over every case, and how many there were.
    assert!(
        checked > 20,
        "only {checked} table-turns compared; the report's shape has probably changed"
    );
}
