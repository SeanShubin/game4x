//! Print the `constraints` relation as the release implies it.
fn main() {
    let at =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md");
    let document = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    print!("{}", game_console::declare::constraints(&document));
}
