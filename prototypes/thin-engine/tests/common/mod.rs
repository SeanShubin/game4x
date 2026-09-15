//! Reading `data/` - the only thing the tests share, and the only place a file is read from.
//!
//! **`src/` reads no file at all**, which is what `S-136` means by isolated and what
//! `tests/isolation.rs` checks. So the loaders live here, in `tests/`, where the path is this
//! directory's own.

#![allow(dead_code)]
// **Each test binary compiles this whole module**, so a helper only one file uses is dead code in
// the others. That is how `mod common` works in Rust and not a sign of an unused helper.

pub mod friendly;

use std::path::PathBuf;

use thin_engine::engine::Game;
use thin_engine::notation::{Row, read};
use thin_engine::schema::Malformed;

/// **The only directory anything here reads**, which is what `S-136` means by isolated.
pub fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn rows(at: &str) -> Vec<Row> {
    let path = mine().join(at);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|why| panic!("{at}: {why}"));
    read(&text).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// The four files `data/test.4x` loads into the game, in the order it loads them.
///
/// **Named here as well as in `test.4x`, and the two are checked against each other** by
/// `the_helper_loads_what_the_script_loads` - which caught them disagreeing about the order the
/// first time it ran. **The order does not matter to the engine**, since everything goes into one
/// store and is validated together; the lists agreeing is what matters, and asserting the order is
/// the cheapest way to notice that they do not.
pub const LOADED: [&str; 5] = [
    "data/foundation/schema.4x",
    "data/foundation/engine.4x",
    "data/foundation/rules.4x",
    "data/foundation/before.4x",
    "data/foundation/command.4x",
];

pub fn game_rows() -> Vec<Row> {
    let mut all = Vec::new();
    for file in LOADED {
        all.extend(rows(file));
    }
    all
}

/// The game as `data/` states it, before anything has run.
pub fn before() -> Game {
    Game::of(game_rows()).unwrap_or_else(|why| panic!("{why}"))
}

/// The same, with some rows added, for checking what the structure refuses.
pub fn with(extra: &str) -> Result<Game, Malformed> {
    let mut all = game_rows();
    all.extend(read(extra).expect(extra));
    Game::of(all)
}
