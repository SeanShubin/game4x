//! Record that Sean has read a test, by copying the version he read.
//!
//! **Sean, 2026-09-16**: *I was thinking of something along the lines of a copy of the tests that I
//! put some sort of marker in, and some automated way of detecting a non-syntax difference between
//! the test I put my marker in, and the test that was run.*
//!
//! **The copy is the marker, and writing it is the review.** There is nothing to maintain beside a
//! test: you read the current one, say it is right, and this puts that version in `reviewed/`. A
//! later edit makes the two differ, and `report.html` says so until you look again.
//!
//! **The copy is of the friendly file**, because that is the one a person reads.
//!
//! **This lane never runs this.** Sean approves a test, this lane changes tests - and a lane that
//! could stamp its own change would be approving its own work, which is the rule `CLAUDE.md`
//! already has for not marking a capability vetted.
//!
//! **There is no `--all`.** Approving everything at once is approving without reading, which is
//! the one thing the copy exists to make impossible to do by accident.
//!
//! `cargo run --example review -- the-scout-moves-to-an-adjacent-place`

use std::path::PathBuf;

fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Every test, by stem, read rather than listed.
fn every_test() -> Vec<String> {
    let mut found: Vec<String> =
        std::fs::read_dir(mine().join("data").join("friendly").join("tests"))
            .expect("data/friendly/tests")
            .filter_map(|it| it.ok())
            .filter_map(|it| it.file_name().to_str().map(str::to_string))
            .filter_map(|name| name.strip_suffix(".4x").map(str::to_string))
            .collect();
    found.sort();
    found
}

fn main() {
    let asked: Vec<String> = std::env::args().skip(1).collect();
    let known = every_test();

    if asked.is_empty() {
        eprintln!("which test? one of:");
        for name in &known {
            eprintln!("  {name}");
        }
        std::process::exit(2);
    }

    let reviewed = mine().join("reviewed");
    std::fs::create_dir_all(&reviewed).expect("reviewed/");

    for name in &asked {
        // **A name that is not a test is refused rather than guessed at.** Approving a file that
        // does not exist would leave a copy nothing is ever compared against.
        if !known.contains(name) {
            eprintln!("`{name}` is not a test - there is no data/friendly/tests/{name}.4x");
            std::process::exit(2);
        }
        let from = mine().join(format!("data/friendly/tests/{name}.4x"));
        let text = std::fs::read_to_string(&from).expect("the test");
        let to = reviewed.join(format!("{name}.4x"));
        let was = std::fs::read_to_string(&to).unwrap_or_default();
        std::fs::write(&to, &text).expect("the copy");
        if was.is_empty() {
            println!("reviewed/{name}.4x  first review");
        } else if was == text {
            println!("reviewed/{name}.4x  unchanged");
        } else {
            println!("reviewed/{name}.4x  updated");
        }
    }
}
