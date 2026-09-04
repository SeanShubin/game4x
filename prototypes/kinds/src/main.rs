//! Prints the release's seven tables back out, rendered from the data that holds them.
//!
//! `cargo run -p kinds`, or `scripts/kinds.sh`. The rows are the ones
//! `releases/first-release.md` carries, and `tests/against_the_release.rs` compares them.

fn main() {
    // `catalog` writes the generated view; with no argument it prints the tables back out,
    // which is what this binary has always done.
    if std::env::args().nth(1).as_deref() == Some("recipes") {
        let at = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../recipes.md");
        let text = kinds::recipes::recipes(&kinds::release::release());
        std::fs::write(&at, text)
            .unwrap_or_else(|why| panic!("cannot write {}: {why}", at.display()));
        println!("wrote {}", at.display());
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("catalog") {
        let at = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../catalog.md");
        let text = kinds::catalog::catalog(&kinds::release::release());
        std::fs::write(&at, text)
            .unwrap_or_else(|why| panic!("cannot write {}: {why}", at.display()));
        println!("wrote {}", at.display());
        return;
    }

    for (heading, rows) in [
        ("## Kinds", kinds::kinds_table()),
        ("## Families", kinds::families_table()),
        ("## Where things are", kinds::capacities_table()),
        (
            "## What bounds a kind in a territory",
            kinds::bounds_table(),
        ),
        ("## Traits", kinds::traits_table()),
        ("## Units and structures", kinds::units_table()),
        ("## Recipes", kinds::recipes_table()),
    ] {
        println!("{heading}\n");
        for row in rows {
            println!("| {} |", row.join(" | "));
        }
        println!();
    }
}
