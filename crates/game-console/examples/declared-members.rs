//! Print the *member* data file `P-497` asks for, so the specification lane can promote it.
//!
//! ```text
//! cargo run -q -p game-console --example declared-members
//! ```
//!
//! **It writes nothing**, for the reason every example beside it gives: the file belongs in the
//! specification directory, which this lane may not write, so the bytes go to standard output
//! and reach `spec/` by promotion.
//!
//! **This one exists because `P-497` took the family off the kind's line.** `kinds.4x` was
//! `{kind name:pioneer family:unit binding defending fuel ...}` and is now `{kind
//! name:pioneer}`; the family it carried is a row here, and the traits it carried are rows in
//! `carries.4x`. A relation may not hold a repeating group, and a kind's line held two.
fn main() {
    let at =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md");
    let document = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    print!("{}", game_console::declare::members(&document));
}
