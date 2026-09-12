//! Print the *Families* data file rule 7 asks for, so the specification lane can promote it.
//!
//! ```text
//! cargo run -q -p game-console --example declared-families
//! ```
//!
//! **It writes nothing**, for the reason its sibling gives: the file belongs in a directory
//! this lane may not write, so the bytes reach the specification by promotion.
fn main() {
    let at =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md");
    let document = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    print!("{}", game_console::declare::families(&document));
}
