//! The boundaries `docs/architecture.md` states, checked rather than promised.
//!
//! **`spec/README.md` rule 9 says where this lives**: *a check on the artifact's shape lives
//! outside the column it constrains.* `tools/spec/` is the specification lane's, so the code
//! lane cannot weaken what judges it, and `hooks/pre-push` and CI both run every
//! `tools/*/Cargo.toml` - so it stops the push of the lane it binds without being theirs.
//!
//! **This is the pattern `crates/game-model/src/lib.rs` already established** and its three
//! habits are copied deliberately: drop `#[cfg(test)]` before scanning, because a test has to
//! name what it forbids in order to look for it; skip comment lines, because *quoting a thing
//! and doing it are the same bytes*; and assert how many files were read, because an empty scan
//! finds nothing and reports success.
//!
//! **Why `docs/architecture.md` is the source rather than a list here.** A list in this file
//! goes stale silently the next time a crate moves layers. The document states the membership,
//! this reads it, and a crate that is in no layer fails - which is the same shape as
//! `tools/outbox/tests/architecture.rs` reading the crate table rather than carrying one.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|why| panic!("{}: {why}", path.display()))
}

/// The layer each crate is in, read out of `docs/architecture.md`.
fn layers() -> BTreeMap<String, String> {
    let text = read(&root().join("docs/architecture.md"));
    let start = text
        .find("**Which crate is in which layer**")
        .expect("docs/architecture.md states the layer membership");
    let end = text[start..]
        .find("**One kind spans two layers")
        .expect("the membership table ends where the sentence after it begins")
        + start;
    let mut out = BTreeMap::new();
    for line in text[start..end].lines() {
        if !line.starts_with("| ") || !line.contains('`') {
            continue;
        }
        let cells: Vec<&str> = line.trim().trim_matches('|').split('|').collect();
        assert_eq!(cells.len(), 2, "a membership row has two cells: {line}");
        let layer = cells[0].trim().to_string();
        for crate_name in cells[1].split(',') {
            let name = crate_name.trim().trim_matches('`').to_string();
            assert!(
                out.insert(name.clone(), layer.clone()).is_none(),
                "{name} is in two layers"
            );
        }
    }
    assert!(out.len() >= 15, "read {} crates from the table", out.len());
    out
}

/// Every crate in `crates/` is placed in exactly one layer, and no layer names a crate that
/// is not there.
///
/// **The set is the fact and the other columns are judgements** - the same division
/// `tools/outbox/tests/architecture.rs` draws. This checks only the set.
#[test]
fn every_crate_is_in_exactly_one_layer() {
    let placed: BTreeSet<String> = layers().keys().cloned().collect();
    let on_disk: BTreeSet<String> = std::fs::read_dir(root().join("crates"))
        .expect("crates/")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    assert!(
        !on_disk.is_empty(),
        "no crates found, so this checked nothing"
    );
    let unplaced: Vec<&String> = on_disk.difference(&placed).collect();
    let ghosts: Vec<&String> = placed.difference(&on_disk).collect();
    assert!(
        unplaced.is_empty() && ghosts.is_empty(),
        "in crates/ and no layer: {unplaced:?}; in a layer and not on disk: {ghosts:?}"
    );
    assert_eq!(placed.len(), on_disk.len(), "over {} crates", on_disk.len());
}

/// **Rule 4**: engine types live only in the adapter layer.
///
/// **The one exception is asserted to be present before it is set aside**, which is the
/// discipline the code lane used for `keeps of:thing`: an exception that can hide the absence
/// of what it excuses is worse than no check. When `S-160` lands, this test fails on the
/// exception being unnecessary, and that failure is how the exception gets removed.
#[test]
fn bevy_appears_only_in_the_adapter_layer() {
    // `crates/game4x/src/inspect.rs` writes a Bevy plugin in the composition root, where rule 4
    // allows assembling one. Filed as `S-160`; the fix is a crate of its own that `game4x` adds.
    const KNOWN: &str = "game4x";

    let layers = layers();
    let mut offences: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut scanned = 0;
    for (name, layer) in &layers {
        if layer == "Engine adapter" {
            continue;
        }
        let src = root().join("crates").join(name).join("src");
        if !src.is_dir() {
            continue;
        }
        for entry in std::fs::read_dir(&src).expect("src") {
            let path = entry.expect("entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            scanned += 1;
            let text = read(&path);
            // The rule binds the code that ships, and a test must name what it forbids.
            let code = match text.find("#[cfg(test)]") {
                Some(at) => &text[..at],
                None => &text[..],
            };
            for (number, line) in code.lines().enumerate() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") {
                    continue;
                }
                if trimmed.contains("bevy::") || trimmed.starts_with("use bevy") {
                    offences.entry(name.clone()).or_default().push(format!(
                        "{}:{}",
                        path.file_name().unwrap().to_string_lossy(),
                        number + 1
                    ));
                }
            }
        }
    }

    // **A floor, because the number is a property of how the crates are laid out.** What it has
    // to catch is the scan collapsing - `read_dir` is not recursive, so a module moved into a
    // subdirectory of `src/` would go unread and this would stay green.
    assert!(
        scanned >= 30,
        "scanned {scanned} files outside the adapter layer, which is too few to have looked"
    );

    assert!(
        offences.contains_key(KNOWN),
        "{KNOWN} no longer names an engine type, so `S-160` is fixed and this exception must go"
    );
    let unexpected: Vec<(&String, &Vec<String>)> = offences
        .iter()
        .filter(|(name, _)| name.as_str() != KNOWN)
        .collect();
    assert!(
        unexpected.is_empty(),
        "engine types outside the adapter layer, over {} crates and {scanned} files: {unexpected:?}",
        layers.len()
    );
}
