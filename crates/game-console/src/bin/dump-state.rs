//! Write the generated dump files.
//!
//! `cargo run -p game-console --bin dump-state`, or `scripts/dump-state.sh`.
//!
//! **The scenario comes from a file and so does everything it names.** `spec/invariants.md`:
//! *a scenario is a file too*, so what a run exercises can be changed without changing the
//! program. This runs `commands/setup.4x` and `commands/play.4x` and writes what they left.
//!
//! It knows almost nothing: `dump::generated` runs the scenario and renders it, and this
//! writes the result. The check that the committed files match uses the same function, so
//! there is no second copy of the steps for the two to disagree about.

use std::path::{Path, PathBuf};

use game_console::{Library, dump};

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

    for (name, text) in dump::generated(&files) {
        let at = root.join(name);
        std::fs::write(&at, text)
            .unwrap_or_else(|why| panic!("cannot write {}: {why}", at.display()));
        println!("wrote {name}");
    }
}
