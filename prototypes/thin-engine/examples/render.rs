//! Write `data/friendly/` from `data/foundation/`, which is what `tests/directories.rs` asserts.
//!
//! **The two directories hold the same rows in two spellings, and something has to produce one
//! from the other.** Until now nothing did, and the friendly files were kept by hand - which is
//! the arrangement that makes a directory drift while a test that compares them stays green
//! only because somebody remembered.
//!
//! **This does not decide which is the source.** Sean, 2026-09-15: *Lets make friendly the source
//! and not omit anything.* It is the source for authoring; this is how the other side is caught
//! up after a change lands in the foundation, and `tests/directories.rs` is what says they agree
//! however each was produced.
//!
//! `cargo run --example render`

use std::path::PathBuf;

// **Only two of the translator's entry points are used here**, and the rest are the test
// suite's - so this compilation unit sees them as dead where the tests do not.
#[path = "../tests/common/friendly.rs"]
#[allow(dead_code)]
mod friendly;

use friendly::Names;
use thin_engine::notation::{Row, read};

const FILES: [(&str, bool); 5] = [
    ("schema.4x", true),
    ("engine.4x", true),
    ("rules.4x", true),
    ("script.4x", false),
    ("test.4x", false),
];

fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rows(at: &str) -> Vec<Row> {
    let text = std::fs::read_to_string(mine().join(at)).unwrap_or_else(|why| panic!("{at}: {why}"));
    read(&text).unwrap_or_else(|why| panic!("{at}: {why}"))
}

fn main() {
    // **A merged test file spans two stores**, so the store a row belongs to is a fact about
    // where it sits: a section's rows are the game's, everything else in that file is the
    // script's. **The `then` section repeats the `given` one's rows**, and they are deduplicated
    // rather than both kept - a store is a set, and two rows named `scout` would make `thing`
    // non-nameable and take every thing's name away with it. Found by the names vanishing.
    let store = |of_game: bool| -> Vec<Row> {
        let mut all = Vec::new();
        for (file, game) in FILES {
            let these = rows(&format!("data/foundation/{file}"));
            if game {
                if of_game {
                    all.extend(these);
                }
            } else {
                let section = friendly::in_a_section(&these);
                for (row, mine) in these.into_iter().zip(section) {
                    if mine == of_game {
                        all.push(row);
                    }
                }
            }
        }
        all.dedup_by(|left, right| left == right);
        let mut seen = Vec::new();
        for row in all {
            if !seen.contains(&row) {
                seen.push(row);
            }
        }
        seen
    };
    let of_game = Names::of(&store(true));
    let of_script = Names::of(&store(false));

    for (file, game) in FILES {
        let from = format!("data/foundation/{file}");
        let text = std::fs::read_to_string(mine().join(&from)).expect(&from);
        // **A merged test file spans two stores**, so which `Names` reads a row is a fact about
        // where the row sits: the prologue is the script's and the sections are the game's.
        let parsed = rows(&from);
        let of_the_game = friendly::in_a_section(&parsed);
        let mut at = 0;

        let mut out = String::new();
        for line in text.lines() {
            let bare = line.trim();
            if bare.is_empty() || bare.starts_with('#') {
                // **Comments and blank lines are carried across**, so the friendly file keeps the
                // prose that explains it rather than becoming a bare list of rows.
                out.push_str(line);
            } else {
                let names = if game || of_the_game[at] {
                    &of_game
                } else {
                    &of_script
                };
                at += 1;
                let row = read(bare).unwrap_or_else(|why| panic!("{from}: {why}"));
                out.push_str(&names.row(&row[0]));
            }
            out.push('\n');
        }
        std::fs::write(mine().join(format!("data/friendly/{file}")), &out).expect(file);
        println!("data/friendly/{file}");
    }
}
