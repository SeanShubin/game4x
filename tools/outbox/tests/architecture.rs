//! `docs/architecture.md` names every crate, and the workspace decides which crates exist.
//!
//! `S-2`. That document enumerates the set twice - the table of layers and dependencies,
//! and rule 5's requirement that each crate's `README.md` be linked from it - and both have
//! gone stale twice: once when `planet-terrain` landed, and again when `planet-presentation`,
//! `game-globe`, `planet-raster` and `planet-flat` did.
//!
//! # Why a test rather than a generated table
//!
//! The same hazard in the pre-push gate was fixed the other way, by selecting crates with
//! `--exclude` so that coverage is the default. That was right **because the thing being
//! fixed was the gate**: a detector needs somewhere trustworthy to report to, and there was
//! nothing. A table check has no such problem - the gate is now the trustworthy thing, so a
//! test in it is wired to a failure by construction.
//!
//! Coverage by default is still the instinct, and it is satisfied here by what the test
//! iterates: the workspace, not a list. A crate that lands and is not written down fails
//! this, and nobody has to have remembered anything.
//!
//! The table stays hand-written because its other three columns are judgements - what layer
//! a crate is, and what it holds. Only the *set* is a fact, and only the set is checked.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every crate the workspace builds, as a path from the repository root.
fn members(manifest: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let Some(from) = manifest.find("members = [") else {
        panic!("the workspace manifest has no members list");
    };
    let rest = &manifest[from..];
    let Some(to) = rest.find(']') else {
        panic!("the members list is not closed");
    };
    for piece in rest[..to].split('"').skip(1).step_by(2) {
        found.insert(piece.trim_end_matches('/').to_string());
    }
    found
}

/// Every crate the document has a row for, and the README each row links to.
fn rows(document: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for line in document.lines() {
        let line = line.trim();
        if !line.starts_with("| [`") {
            continue;
        }
        // | [`crates/x`](../crates/x/README.md) | kind | deps | what it holds |
        let Some(name) = line
            .split_once("[`")
            .and_then(|(_, rest)| rest.split_once('`'))
            .map(|(name, _)| name.to_string())
        else {
            continue;
        };
        let Some(link) = line
            .split_once("](")
            .and_then(|(_, rest)| rest.split_once(')'))
            .map(|(link, _)| link.to_string())
        else {
            continue;
        };
        found.push((name, link));
    }
    found
}

#[test]
fn every_crate_has_a_row_and_every_row_has_a_crate() {
    let manifest = std::fs::read_to_string(root().join("Cargo.toml"))
        .expect("the workspace manifest is at the root");
    let document = std::fs::read_to_string(root().join("docs/architecture.md"))
        .expect("the architecture document is where the workflow says it is");

    let built = members(&manifest);
    let written: BTreeSet<String> = rows(&document).into_iter().map(|(name, _)| name).collect();

    // The test has to be able to fail. If the table were reformatted so that no row parsed,
    // this would report every crate as missing rather than passing in silence - but a table
    // that parsed to nothing and a workspace with nothing in it look identical from here.
    assert!(
        built.len() >= 10,
        "only {} workspace members parsed; the manifest's shape has changed",
        built.len()
    );
    assert!(
        !written.is_empty(),
        "no rows parsed out of docs/architecture.md; the table's shape has changed"
    );

    let missing: Vec<&String> = built.difference(&written).collect();
    let extra: Vec<&String> = written.difference(&built).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "docs/architecture.md and the workspace disagree about which crates exist\n\
         \n  in the workspace, with no row: {missing:?}\
         \n  has a row, not in the workspace: {extra:?}\n\
         \nThe table's other columns are judgements and are not checked. Only the set is a \
         fact.\nSee S-2."
    );
}

/// Rule 5: each crate's `README.md` is linked from the document. A row that links to a file
/// that is not there satisfies the rule in form and not in fact.
#[test]
fn every_row_links_to_a_readme_that_exists() {
    let document = std::fs::read_to_string(root().join("docs/architecture.md"))
        .expect("the architecture document is where the workflow says it is");
    let mut broken = Vec::new();
    let mut checked = 0usize;
    for (name, link) in rows(&document) {
        // Links are relative to `docs/`, which is where the document lives.
        let target = root().join("docs").join(&link);
        checked += 1;
        if !Path::new(&target).exists() {
            broken.push(format!("{name} -> {link}"));
        }
    }
    assert!(checked >= 10, "only {checked} rows parsed");
    assert!(
        broken.is_empty(),
        "rows link to a README that is not there:\n  {}",
        broken.join("\n  ")
    );
}
