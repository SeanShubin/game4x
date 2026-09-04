//! Write the state after a scenario, as markdown.
//!
//! `cargo run -p game-console --bin dump-state`, or `scripts/dump-state.sh`.
//!
//! **The scenario comes from a file and so does everything it names.** `spec/invariants.md`:
//! *a scenario is a file too*, so what a run exercises can be changed without changing the
//! program. This runs `commands/setup.4x` and `commands/play.4x` and dumps what they left.
//!
//! It takes the state as an argument rather than reaching for the final one, so writing a
//! dump per turn is a loop around this call rather than a second implementation.

use std::path::{Path, PathBuf};

use game_console::{Library, Session, dump};

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let files = Files(root.join("commands"));
    let mut session = Session::new();
    for command in ["run setup", "start"] {
        session
            .run(command, &files)
            .unwrap_or_else(|why| panic!("`{command}` failed: {why}"));
    }

    // **The scenario is run line by line rather than with `run play`**, so that a state can
    // be taken at each turn boundary. `S-27`: Sean is deriving these eight turns by hand,
    // and a mismatch at turn 3 reaches him as a wrong number at turn 8 with eight turns of
    // arithmetic to search. Eight dumps turn that from a mystery into a finding.
    //
    // **The boundaries are the scenario's own `end turn` lines and nothing else.** His
    // checkpoints are `play.4x`'s own comments - *Turn 2. Four citizens* - so a dump that
    // counted turns its own way would be worse than no dump at all.
    let scenario = files
        .fetch("play")
        .unwrap_or_else(|| panic!("commands/play.4x is not there"));
    let boundaries = scenario
        .lines()
        .filter(|line| line.trim() == "end turn")
        .count();

    let mut turns: Vec<(usize, String)> = Vec::new();
    for line in scenario.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        session
            .run(line, &files)
            .unwrap_or_else(|why| panic!("`{line}` failed: {why}"));
        if line == "end turn" {
            turns.push((turns.len() + 1, dump::markdown(&session.game, "")));
        }
    }

    // **The count, because a loop that stopped early produces a file that looks finished.**
    // He would derive against a truncated run and find nothing wrong with it.
    assert_eq!(
        turns.len(),
        boundaries,
        "commands/play.4x has {boundaries} `end turn` lines and {} states were taken",
        turns.len()
    );

    // Four files, two views. The markdown is the record because it diffs; the HTML is the
    // same rows in the form that is comfortable to read. One producer feeds both.
    let state = "State after `commands/play.4x`";
    let things = "Entities after `commands/play.4x`";
    // One file with a section per turn rather than eight files: a single diff shows which
    // turn changed and by how much, which is the question a rule change raises.
    let mut per_turn = String::from(
        "# Every turn of `commands/play.4x`

",
    );
    per_turn.push_str(&format!(
        "**Generated. Do not edit.** One section per `end turn` in the scenario - {} of          them.
The turn numbers are the scenario's own boundaries, so they line up with          its comments.

",
        turns.len()
    ));
    for (at, text) in &turns {
        per_turn.push_str(&format!(
            "# Turn {at}

"
        ));
        // The per-state dump opens with its own title and preamble; here the heading above
        // is the title, so both are dropped and the tables kept.
        let tables = text
            .split_once(
                "

## ",
            )
            .map(|(_, rest)| rest);
        match tables {
            Some(rest) => per_turn.push_str(&format!("## {rest}")),
            None => per_turn.push_str(text),
        }
    }

    let written: [(&str, String); 5] = [
        ("state.md", dump::markdown(&session.game, state)),
        (
            "state.html",
            dump::html(&dump::normalized_sections(&session.game), state),
        ),
        (
            "entities.md",
            dump::entities_markdown(&session.game, things),
        ),
        (
            "entities.html",
            dump::html(&dump::entity_sections(&session.game), things),
        ),
        ("turns.md", per_turn),
    ];

    for (name, text) in written {
        let at = root.join(name);
        std::fs::write(&at, text)
            .unwrap_or_else(|why| panic!("cannot write {}: {why}", at.display()));
        println!("wrote {name}");
    }
}
