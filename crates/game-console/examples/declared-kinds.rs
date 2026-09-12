//! Print the *Kinds* data file rule 7 asks for, so the specification lane can promote it.
//!
//! ```text
//! cargo run -q -p game-console --example declared-kinds
//! ```
//!
//! **It writes nothing.** The file belongs in the specification directory, which this lane may
//! not write, so the bytes go to standard output and reach `spec/` by promotion like any other
//! content - which is the whole point of `C-49`'s answer: the data is the release's own, moved
//! rather than authored, so nothing here is an idea needing a decision.
//!
//! **Generated rather than transcribed by hand**, which is what makes *go with the data we have
//! been using* checkable. `tests/declare.rs` compares what this prints against the release's
//! table in both directions, so a word invented here or dropped here fails the gate.
fn main() {
    let at =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md");
    let document = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    print!("{}", game_console::declare::kinds(&document));
}
