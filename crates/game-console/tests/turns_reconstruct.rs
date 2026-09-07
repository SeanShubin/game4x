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
//! It does. `state::entries` renders a state and `state::compare` differences two of them,
//! and both the printed delta *and* the printed state come from the same containment tree.
//! There is no second derivation available - the model's own state comparison **is**
//! `compare`.
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

use game_console::{Library, Session, state};

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
    let files = Files(root.join("scenario/commands"));
    let report =
        std::fs::read_to_string(root.join("reports/turns.md")).expect("turns.md is generated");
    let listed = commands_per_turn(&report);

    assert!(
        listed.len() > 1,
        "only {} turns found in turns.md; its shape has changed",
        listed.len()
    );

    // The scenario as the report tells it, and the scenario as the file runs it, must be
    // the same game at every turn boundary.
    let mut replayed = Session::new();
    for line in ["{run file:setup}", "{start}"] {
        replayed.run(line, &files).expect("the setup runs");
    }
    let mut played = Session::new();
    for line in ["{run file:setup}", "{start}"] {
        played.run(line, &files).expect("the setup runs");
    }

    let scenario = files.fetch("play").expect("scenario/commands/play.4x");
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

        let wrong = state::compare(
            &state::entries(&played.game),
            &state::entries(&replayed.game),
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

/// The printed delta accounts for every thing the printed states gained or lost.
///
/// **The check I first declined, and my reason for declining was wrong.** I said the delta
/// and the states share `dump::tables`, so there is no second derivation and comparing them
/// would compare a path to itself. The specification lane pointed out that the lens's
/// caution forbids a second derivation that shares the **computation**, not one that shares
/// the inputs - and reading the printed tables back out of `turns.md` and counting them
/// calls neither `state::compare` nor `dump::tables`. It runs on the artifact.
///
/// **`S-47` removed the circularity that argument was working around.** The delta comes from
/// the containment tree now and the printed state still comes from `dump::tables`, so the
/// two really are separate projections of the model. What was a reconciliation of an artifact
/// against itself is a reconciliation of two derivations.
///
/// **It reconciles the `kind` table rather than every table**, and that is the translation
/// this can afford. A row of `kind` is *how many of this kind are in play anywhere*, which is
/// exactly what an entry's quantity says - so the two sides need no translator, and a
/// translator wrong in the same direction as the printer would be the circularity again,
/// only harder to see. The other tables print a row per territory and resource whether or not
/// anything is there, so their row counts are not counts of things at all.
///
/// So this catches a delta that **omits** a thing - `S-39`'s failure - and does not catch one
/// that reports the wrong container. Placement-level reconciliation is buildable and not
/// built, and the reason is the translator, not circularity.
///
/// # Turn 1's delta is reconciled by nothing, and that was found by poisoning
///
/// **A comparison needs a state on both sides and turn 1 has one.** The report prints the
/// state after each turn, so the first turn's delta is against a state that was never
/// printed - and the loop below has nothing to be its `previous`. Deleting a line from turn
/// 1's delta leaves this green.
///
/// It was found the way `docs/process.md` says to find this: **the check was poisoned before
/// it was believed**, one delta line removed at a time, and the first attempt landed in the
/// blind region and passed. A check that has only ever passed is a claim - `C-33`.
///
/// **Not repaired here, because repairing it means printing the state before play begins**,
/// which changes the artifact Sean reads rather than the check that reads it. Recorded so
/// that the coverage is nine turns out of ten and says so, rather than reading as ten.
#[test]
fn the_delta_accounts_for_every_thing_the_states_gained_or_lost() {
    let report =
        std::fs::read_to_string(root().join("reports/turns.md")).expect("turns.md is generated");

    // How many of each kind the printed `kind` table says are in play.
    fn in_play(state: &str) -> std::collections::BTreeMap<String, i64> {
        let mut out = std::collections::BTreeMap::new();
        let mut inside = false;
        for line in state.lines() {
            if let Some(name) = line.trim().strip_prefix("### ") {
                inside = name.trim() == "kind";
                continue;
            }
            let line = line.trim();
            if !inside || !line.starts_with('|') || line.contains("---") {
                continue;
            }
            let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
            let (Some(kind), Some(count)) = (cells.first(), cells.get(1)) else {
                continue;
            };
            if let Ok(count) = count.parse::<i64>() {
                out.insert(kind.to_string(), count);
            }
        }
        out
    }

    // The kind an entry is of, which is the innermost description of its path.
    //
    // **The innermost and not the first.** A line reads `{territory id:1} {citizen} -> 8`,
    // and the thing that appeared is the citizen; taking the first would credit every change
    // to the territory that contains it.
    fn kind_of(path: &str) -> Option<String> {
        let last = path.rfind('{')?;
        let inner = &path[last + 1..];
        let end = inner.find(['}', ' '])?;
        Some(inner[..end].to_string())
    }

    let turns: Vec<&str> = report.split("\n# Turn ").skip(1).collect();
    assert!(turns.len() > 1, "only {} turns parsed", turns.len());

    // Nine of the ten, because turn 1 has no printed state before it - see above.
    let reconciled = turns.len() - 1;
    let mut checked = 0usize;
    let mut moved = 0usize;
    let mut previous: Option<std::collections::BTreeMap<String, i64>> = None;
    for (at, turn) in turns.iter().enumerate() {
        let state = turn.split("## what is there now").nth(1).unwrap_or("");
        let now = in_play(state);
        assert!(
            !now.is_empty(),
            "turn {} prints no `kind` table, so this compares nothing",
            at + 1
        );

        if let Some(before) = previous {
            let delta = turn
                .split("## what changed")
                .nth(1)
                .and_then(|rest| rest.split("## what is there now").next())
                .unwrap_or("");

            // What the delta says each kind gained, net. `new` and `gone` carry a quantity
            // after `->`; `changed` carries the two quantities either side of an arrow.
            let mut said: std::collections::BTreeMap<String, i64> =
                std::collections::BTreeMap::new();
            let mut heading = "";
            for line in delta.lines() {
                let line = line.trim();
                if line.starts_with("**") {
                    heading = if line.starts_with("**new**") {
                        "new"
                    } else if line.starts_with("**gone**") {
                        "gone"
                    } else {
                        "changed"
                    };
                    continue;
                }
                let Some(item) = line.strip_prefix("- ") else {
                    continue;
                };
                let Some(kind) = kind_of(item) else { continue };
                let change = match heading {
                    "new" | "gone" => {
                        let Some((_, quantity)) = item.rsplit_once("-> ") else {
                            continue;
                        };
                        let Ok(quantity) = quantity.trim().parse::<i64>() else {
                            continue;
                        };
                        if heading == "new" {
                            quantity
                        } else {
                            -quantity
                        }
                    }
                    _ => {
                        let Some((was, is)) = item.rsplit_once(" \u{2192} ") else {
                            continue;
                        };
                        let Some((_, was)) = was.rsplit_once("\u{b7} ") else {
                            continue;
                        };
                        let (Ok(was), Ok(is)) =
                            (was.trim().parse::<i64>(), is.trim().parse::<i64>())
                        else {
                            continue;
                        };
                        is - was
                    }
                };
                *said.entry(kind).or_insert(0) += change;
            }

            for (kind, after) in &now {
                let was = before.get(kind).copied().unwrap_or(0);
                let grew = after - was;
                let accounted = said.get(kind).copied().unwrap_or(0);
                assert_eq!(
                    grew,
                    accounted,
                    "turn {}: the printed state says `{kind}` went from {was} to {after}, and \
                     the delta accounts for {accounted}. A thing changed hands without the \
                     delta saying so, which is what a correct-looking report that omits \
                     something looks like.",
                    at + 1
                );
                checked += 1;
                if grew != 0 {
                    moved += 1;
                }
            }
        }
        previous = Some(now);
    }

    // Over every case, and how many there were.
    assert!(
        checked > 20,
        "only {checked} kind-turns compared; the report's shape has probably changed"
    );
    assert_eq!(
        checked % reconciled,
        0,
        "{checked} comparisons over {reconciled} turns is not a whole number of kinds each, \
         so some turn printed a different `kind` table from the rest"
    );
    // **And how many of them actually moved.** Every kind agreeing at zero on both sides is a
    // reconciliation of nothing against nothing, which is the count-over-nothing failure with
    // the sign flipped: it would pass on a delta that printed no lines at all.
    assert!(
        moved > 5,
        "only {moved} of {checked} comparisons had anything change, so this would pass on an \
         empty delta"
    );
}
