//! Reading `data/` - the only thing the tests share, and the only place a file is read from.
//!
//! **`src/` reads no file at all**, which is what `S-136` means by isolated and what
//! `tests/isolation.rs` checks. So the loaders live here, in `tests/`, where the path is this
//! directory's own.

#![allow(dead_code)]
// **Each test binary compiles this whole module**, so a helper only `founding.rs` uses is dead
// code in `moving.rs` and the other way round. That is how `mod common` works in Rust and not a
// sign of an unused helper; the alternative is a helper per file, which is the duplication this
// module exists to remove.

use std::path::PathBuf;

use thin_engine::notation::{Row, read};
use thin_engine::store::Store;

/// **The only directory anything here reads**, which is what `S-136` means by isolated.
pub fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn rows(at: &str) -> Vec<Row> {
    let path = mine().join(at);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|why| panic!("{at}: {why}"));
    read(&text).unwrap_or_else(|why| panic!("{at}: {why}"))
}

pub fn world() -> Store {
    Store::of(rows("data/world.4x"))
}

pub fn rules() -> Vec<Row> {
    rows("data/rules.4x")
}

pub fn command(text: &str) -> Row {
    read(text).expect(text).pop().expect(text)
}
