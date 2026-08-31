//! Prints the two tables back out, so the data can be read as the release writes it.
//!
//! `cargo run -p kinds` — the same rows `releases/first-release.md` carries, rendered from
//! the Rust data rather than copied. `tests/against_the_release.rs` compares them.

fn main() {
    println!("## Units and structures\n");
    for row in kinds::units_table() {
        println!("| {} |", row.join(" | "));
    }
    println!("\n## Transformations\n");
    for row in kinds::transformations_table() {
        println!("| {} |", row.join(" | "));
    }
}
