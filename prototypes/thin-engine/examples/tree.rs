//! Print the rules as a tree - `cargo run --example tree`.
//!
//! **Sean, 2026-09-18**, on why this is worth a file of its own: the specification *had no artifact
//! whose whole structure you could read at once*, and that is how it stopped being something he
//! could hold. This is that artifact for the rules.
//!
//! **It writes `tree.txt` as well as printing**, so the committed file is the one a test compares
//! against - a rule added to the tree makes that test red, and the only way to green it is to
//! regenerate and read what changed.

use std::path::PathBuf;

use thin_engine::engine::Game;
use thin_engine::notation::read;

const LOADED: [&str; 3] = [
    "data/foundation/schema.4x",
    "data/foundation/engine.4x",
    "data/foundation/rules.4x",
];

fn main() {
    let mine = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut rows = Vec::new();
    for file in LOADED {
        let text = std::fs::read_to_string(mine.join(file)).expect(file);
        rows.extend(read(&text).expect(file));
    }
    let game = Game::of(rows).unwrap_or_else(|why| panic!("{why}"));
    let tree = game.tree();
    print!("{tree}");
    let at = mine.join("tree.txt");
    let before = std::fs::read_to_string(&at).unwrap_or_default();
    if before != tree {
        std::fs::write(&at, &tree).expect("tree.txt");
        println!("\ntree.txt rewritten - read it before committing");
    }
}
