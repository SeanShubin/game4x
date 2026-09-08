//! List the loops that assert over a population nothing has counted.
//!
//! `cargo run -p quality --bin denominators` from `tools/quality`, or with a path to scan.
//! **The output is a list to read, not a verdict** - see the crate doc for why the deciding
//! instrument is the one `C-28` says cannot exist.

use std::path::{Path, PathBuf};

use quality::{Candidate, Missing, rust_files, vacuous_tests};

fn main() {
    let mut args = std::env::args().skip(1);
    let root = args.next().map_or_else(
        || PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        PathBuf::from,
    );
    let root = root.canonicalize().unwrap_or(root);

    let files = rust_files(&root);
    if files.is_empty() {
        eprintln!(
            "no .rs files under {}, so this found nothing about nothing",
            root.display()
        );
        std::process::exit(2);
    }

    let mut found: Vec<Candidate> = Vec::new();
    let mut looked_at = 0usize;
    for file in &files {
        let Ok(source) = std::fs::read_to_string(file) else {
            continue;
        };
        looked_at += 1;
        let shown = shorten(file, &root);
        found.extend(vacuous_tests(&shown, &source));
    }

    println!("# Tests that can pass having checked nothing");
    println!();
    println!(
        "Read {looked_at} of {} `.rs` files under `{}`.",
        files.len(),
        root.display()
    );
    println!(
        "**{} candidates**, {} never named and {} named but not asserted.",
        found.len(),
        found
            .iter()
            .filter(|c| c.why == Missing::NeverNamed)
            .count(),
        found
            .iter()
            .filter(|c| c.why == Missing::NotAsserted)
            .count()
    );
    println!();
    println!("A candidate is a test worth looking at, not a defect.");
    println!();

    let mut last = String::new();
    for candidate in &found {
        if candidate.file != last {
            println!("\n## {}", candidate.file);
            last.clone_from(&candidate.file);
        }
        println!(
            "- `{}:{}` in `{}` - over `{}` ({})",
            candidate.file, candidate.line, candidate.in_fn, candidate.over, candidate.why
        );
    }
}

fn shorten(file: &str, root: &Path) -> String {
    let root = root.to_string_lossy().replace('\\', "/");
    file.strip_prefix(&root)
        .unwrap_or(file)
        .trim_start_matches('/')
        .to_string()
}
