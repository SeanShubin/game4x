//! Prints the release's six tables back out, rendered from the data that holds them.
//!
//! `cargo run -p kinds`, or `scripts/kinds.sh`. The rows are the ones
//! `releases/first-release.md` carries, and `tests/against_the_release.rs` compares them.

fn main() {
    for (heading, rows) in [
        ("## Kinds", kinds::kinds_table()),
        ("## Families", kinds::families_table()),
        ("## Where things are", kinds::bins_table()),
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
