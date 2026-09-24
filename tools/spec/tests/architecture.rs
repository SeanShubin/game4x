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
/// **The dependency graph is the audit, and rule 7 says so.** *Algorithm crates do not depend on
/// Bevy. That is what makes rule 6 enforced by the compiler rather than by discipline, and
/// `cargo tree` is the audit.* A crate whose manifest does not name `bevy` cannot compute with an
/// engine type, whatever its text says - so the manifest is the assertion and no scan is needed.
///
/// # This predicate was wrong twice before it was right, and both ways are worth recording
///
/// **First it looked for the text `bevy::`.** Rule 4 permits the root to *assemble* plugins,
/// which cannot be done without naming `bevy`, so the check flagged what the rule allows. It
/// carried `game4x` as a named exception for `inspect.rs` writing a plugin in the root, and when
/// `8628c437` moved that plugin into `crates/game-inspect` **the exception's self-deleting
/// assertion went on passing** - the predicate it fired on is one the root satisfies whenever it
/// assembles anything. Found by the code lane, who could not fix it: `tools/spec` is this lane's.
///
/// **Then it looked for rule 6's names** - `Res<`, `ResMut<`, `Query<`, `Commands`, `Entity` -
/// and matched `EntityTable` in `game-console/src/dump.rs` and a markdown heading reading
/// `# Commands`, in a crate whose manifest names no engine at all. **Two of rule 6's five names
/// are ordinary words**, and a text scan cannot tell whose they are.
///
/// **Both failures are one failure**: the instrument asked a question about text where the rule
/// is about dependency. `docs/architecture.md` had already said which instrument to use, in the
/// rule immediately after the one being checked.
///
/// # And the demonstrations failed twice before they worked, which is the same shape again
///
/// Showing that this bites means perturbing the document, and two perturbations silently did
/// nothing: one took the first row starting `| Engine adapter`, which is the layer-knowledge
/// table rather than the membership one, and the next matched `` `planet-flat`, `` where that
/// name is last in its cell and carries no trailing comma. **A perturbation that does nothing
/// looks exactly like a check that does not bite**, and both times the check was fine.
///
/// The working version parses the row to cells and rebuilds it - the discipline this file
/// already uses to read them, applied to writing them too.
///
/// # What is still a scan, and why it is sound there
///
/// The composition root must name `bevy` to assemble, so the manifest cannot judge it. There -
/// and only there - rule 6's names are Bevy's, because nothing else in that crate defines them.
#[test]
fn bevy_appears_only_in_the_adapter_layer() {
    /// The one crate rule 4 lets name an engine while not being an adapter.
    const ROOT: &str = "game4x";
    /// What computing with the engine looks like in a signature - rule 6's own list.
    const ACCESSORS: &[&str] = &["Res<", "ResMut<", "Query<", "Commands", "Entity"];

    let layers = layers();
    let mut naming = Vec::new();
    let mut manifests = 0;
    for name in layers.keys() {
        let manifest = root().join("crates").join(name).join("Cargo.toml");
        if !manifest.is_file() {
            continue;
        }
        manifests += 1;
        let text = read(&manifest);
        let deps = match text.find("[dependencies]") {
            Some(at) => &text[at..],
            None => continue,
        };
        if deps.lines().any(|line| {
            let line = line.trim_start();
            line.starts_with("bevy ") || line.starts_with("bevy.") || line.starts_with("bevy=")
        }) {
            naming.push(name.clone());
        }
    }

    // **The reader is proved to work before its silence is trusted.** A manifest format this
    // does not understand yields an empty list and reports a clean tree in the same words.
    assert!(
        !naming.is_empty(),
        "no manifest under crates/ names bevy, so the reader is broken rather than the tree clean"
    );
    assert!(
        manifests >= 15,
        "read {manifests} manifests, which is too few to have looked"
    );

    let wrong: Vec<&String> = naming
        .iter()
        .filter(|name| {
            name.as_str() != ROOT
                && layers.get(name.as_str()).map(String::as_str) != Some("Engine adapter")
        })
        .collect();
    assert!(
        wrong.is_empty(),
        "naming bevy outside the adapter layer, over {manifests} manifests and {} that name it: {wrong:?}",
        naming.len()
    );

    // The root may assemble and may not compute, and there rule 6's names are the engine's.
    let src = root().join("crates").join(ROOT).join("src");
    let mut computing = Vec::new();
    let mut scanned = 0;
    for entry in std::fs::read_dir(&src).expect("the root's src") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        scanned += 1;
        let text = read(&path);
        let code = match text.find("#[cfg(test)]") {
            Some(at) => &text[..at],
            None => &text[..],
        };
        for (number, line) in code.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            if ACCESSORS.iter().any(|name| trimmed.contains(name)) {
                computing.push(format!(
                    "{}:{}",
                    path.file_name().unwrap().to_string_lossy(),
                    number + 1
                ));
            }
        }
    }
    assert!(
        scanned > 0,
        "the root has no source files, so this checked nothing"
    );
    assert!(
        computing.is_empty(),
        "the root computes with engine types, over {scanned} of its files: {computing:?}"
    );
}
