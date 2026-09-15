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

const FILES: [(&str, bool); 8] = [
    ("schema.4x", true),
    ("engine.4x", true),
    ("rules.4x", true),
    ("before.4x", true),
    ("command.4x", true),
    ("expected.4x", true),
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
    // **`expected.4x` is a second world for the same schema**, so it is not part of the store the
    // others make up - including it would put two rows of each relation in it.
    let store = |of_game: bool| -> Vec<Row> {
        let mut all = Vec::new();
        for (file, game) in FILES {
            if game == of_game && file != "expected.4x" {
                all.extend(rows(&format!("data/foundation/{file}")));
            }
        }
        all
    };
    let of_game = Names::of(&store(true));
    let of_script = Names::of(&store(false));

    for (file, game) in FILES {
        let names = if game { &of_game } else { &of_script };
        let from = format!("data/foundation/{file}");
        let text = std::fs::read_to_string(mine().join(&from)).expect(&from);

        let mut out = String::new();
        for line in text.lines() {
            let bare = line.trim();
            if bare.is_empty() || bare.starts_with('#') {
                // **Comments and blank lines are carried across**, so the friendly file keeps the
                // prose that explains it rather than becoming a bare list of rows.
                out.push_str(line);
            } else {
                let row = read(bare).unwrap_or_else(|why| panic!("{from}: {why}"));
                out.push_str(&names.row(&row[0]));
            }
            out.push('\n');
        }
        std::fs::write(mine().join(format!("data/friendly/{file}")), &out).expect(file);
        println!("data/friendly/{file}");
    }
}
