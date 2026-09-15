//! Reading `data/` - the only thing the tests share, and the only place a file is read from.
//!
//! **`src/` reads no file at all**, which is what `S-136` means by isolated and what
//! `tests/isolation.rs` checks. So the loaders live here, in `tests/`, where the path is this
//! directory's own.

#![allow(dead_code)]
// **Each test binary compiles this whole module**, so a helper only one file uses is dead code in
// the others. That is how `mod common` works in Rust and not a sign of an unused helper.

use std::path::PathBuf;

use thin_engine::engine::Game;
use thin_engine::notation::{Row, read};

/// **The only directory anything here reads**, which is what `S-136` means by isolated.
pub fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn rows(at: &str) -> Vec<Row> {
    let path = mine().join(at);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|why| panic!("{at}: {why}"));
    read(&text).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// The entirety, before: structure, rule, command and world in one list.
pub fn before() -> Game {
    game("data/before.4x")
}

/// The entirety, after - what the first test expects to end at.
pub fn after() -> Game {
    game("data/after.4x")
}

pub fn game(at: &str) -> Game {
    Game::of(rows(at)).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// The entirety with some rows added, for checking what the structure refuses.
pub fn with(extra: &str) -> Result<Game, thin_engine::schema::Malformed> {
    let mut all = rows("data/before.4x");
    all.extend(read(extra).expect(extra));
    Game::of(all)
}
