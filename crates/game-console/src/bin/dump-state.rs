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
    for command in ["run setup", "start", "run play"] {
        session
            .run(command, &files)
            .unwrap_or_else(|why| panic!("`{command}` failed: {why}"));
    }

    let at = root.join("state.md");
    let text = dump::markdown(&session.game, "State after `commands/play.4x`");
    std::fs::write(&at, text).unwrap_or_else(|why| panic!("cannot write {}: {why}", at.display()));
    println!("wrote {}", at.display());
}
