//! Print the *Traits* data file, so the specification lane can check its own against it.
//!
//! ```text
//! cargo run -q -p game-console --example declared-traits
//! ```
//!
//! **The keys are `P-457`'s proposal and not a promoted rule**, so these bytes change if Sean
//! changes either. The content does not: which traits are declared, what each admits and how
//! each is kept are read from the release.
fn main() {
    let at =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md");
    let document = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    print!("{}", game_console::declare::traits(&document));
}
