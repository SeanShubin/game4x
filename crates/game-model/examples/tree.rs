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

use game_model::engine::Game;
use game_model::notation::read;

/// **The foundation form of all three, because that is what `Game::of` reads.**
///
/// **Two of these named `spec/data/` and were right until `P-576`.** That promotion made
/// `spec/data/` the *friendly* source - `{column id:1 relation:relation ...}` where the engine
/// wants `relation:1` - so `Game::of` refused with *`column`.`relation` is `relation`, and no
/// `relation` has that key*. **Nothing ran this between P-576 and the Yard landing**, because
/// `tree.txt` only needs regenerating when a rule moves, and no rule had.
///
/// **The check that names this program is what found it**:
/// `the_tree_is_what_the_file_says_it_is` went red on `build-yard` and said to run this, which
/// is the shape a generated file and its check are supposed to have.
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
