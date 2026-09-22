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
//! # It writes the five shared files and no test, since 2026-09-21
//!
//! **`P-532` moved every test's friendly side to `spec/tests/`**, where `CLAUDE.md` makes it the
//! specification: a test arrives there the way everything in `spec/` arrives, which is that Sean
//! has read it. **So this may not write one.** It is another lane's column, and worse than that
//! it is an approved artifact - regenerating a test from the foundation would let an edit to the
//! generated side silently rewrite the thing that was approved, which is the direction the whole
//! arrangement exists to forbid.
//!
//! **Which leaves the tests with no generator at all, and that is the gap rather than this
//! line.** The foundation side is converted from the friendly side - Sean, 2026-09-15, quoted in
//! `tests/directories.rs` - so what is needed now is the inverse of this program for tests only:
//! `spec/tests/*.4x` read, and `data/foundation/tests/*.4x` written from it. Until that exists, a
//! test he approves does not reach the engine, and `tests/directories.rs` is what would say so.
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

/// Every file in a directory: the shared ones, then one per test.
///
/// **Read rather than listed.** Sean, 2026-09-15: *I intend to have one test per file*, so a list
/// here would be a second place to remember - and `data/{d}/tests/` holding only tests is what
/// makes reading it safe.
fn files() -> Vec<(String, bool)> {
    let mut all: Vec<(String, bool)> = vec![
        ("schema.4x".to_string(), true),
        ("engine.4x".to_string(), true),
        ("rules.4x".to_string(), true),
        ("script.4x".to_string(), false),
        ("setup.4x".to_string(), false),
    ];
    let mut tests: Vec<String> =
        std::fs::read_dir(mine().join("data").join("foundation").join("tests"))
            .expect("data/foundation/tests")
            .filter_map(|it| it.ok())
            .filter_map(|it| it.file_name().to_str().map(str::to_string))
            .filter(|name| name.ends_with(".4x"))
            .collect();
    tests.sort();
    all.extend(
        tests
            .into_iter()
            .map(|name| (format!("tests/{name}"), false)),
    );
    all
}

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
        for (file, game) in files() {
            let these = rows(&format!("data/foundation/{file}"));
            if game {
                if of_game {
                    all.extend(these);
                }
            } else {
                let section = friendly::states_a_world(&these);
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

    let mut changed = 0;
    for (file, game) in files() {
        // **A test's friendly side is not this program's to write** - see the note at the top.
        // Skipped rather than filtered out of `files()`, because the store above is built from
        // every file and a test's rows are most of what the names are drawn from.
        if file.starts_with("tests/") {
            continue;
        }
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
        // **Only what changed, because a silent overwrite is how a hand edit is lost.** Sean's
        // spacing was written into `data/friendly/` and this would have taken it straight back
        // out at the next run, one line among six that all looked the same.
        let at = mine().join(format!("data/friendly/{file}"));
        let before = std::fs::read_to_string(&at).unwrap_or_default();
        if before != out {
            std::fs::write(&at, &out).expect(&file);
            println!("data/friendly/{file}");
            changed += 1;
        }
    }
    println!("{changed} rewritten");
}
