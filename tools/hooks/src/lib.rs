//! What `hooks/pre-commit` decides, made askable so it can be checked.
//!
//! **`C-69`.** The hook refuses a commit whose files span two perspectives' columns, which is
//! `CLAUDE.md` -> Perspectives enforced rather than described. It was verified by running
//! twelve cases by hand and nothing re-ran them - and a hook is a check, so *it stayed green*
//! stops being information the moment nobody re-poisons it.
//!
//! **The specific way it rots is a new top-level directory.** `column_of` is a `case` over
//! path prefixes with an unassigned arm, so a directory nobody added a pattern for falls
//! through it silently - and a tree where nothing is assigned looks exactly like a tree where
//! nothing spans two columns. That is the direction a hand run cannot cover and the reason
//! this exists.
//!
//! **The hook's own code is what runs**, extracted from the file rather than copied into
//! Rust. A second implementation here would be the thing being checked written twice, and the
//! copy is what rots.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The repository root, from this tool's manifest.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// `column_of` as `hooks/pre-commit` defines it, lifted out so it can be called.
///
/// **Located by its own text and asserted, not copied.** If the function is renamed or moved
/// this fails loudly rather than testing a stale duplicate.
pub fn column_of_source() -> String {
    let at = root().join("hooks/pre-commit");
    let text = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    let from = text
        .find("column_of() {")
        .unwrap_or_else(|| panic!("{} defines no `column_of`", at.display()));
    let rest = &text[from..];
    let to = rest
        .find("\n}\n")
        .unwrap_or_else(|| panic!("`column_of` in {} does not close", at.display()));
    let body = &rest[..to + 3];
    assert!(
        body.contains("lenses/") && body.contains("pending.md"),
        "the extracted function is not the column mapping:\n{body}"
    );
    body.to_string()
}

/// Which column the hook puts a path in, or `None` where it assigns none.
///
/// Runs the hook's own `sh` function, from the repository root, because it asks the tree
/// which `tools/` directories belong to a lens.
pub fn column_of(path: &str) -> Option<String> {
    let script = format!("{}\ncolumn_of \"$1\"\n", column_of_source());
    let out = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .arg("sh")
        .arg(path)
        .current_dir(root())
        .output()
        .expect("sh should run the hook's own function");
    assert!(
        out.status.success(),
        "`column_of {path}` failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let said = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if said.is_empty() { None } else { Some(said) }
}

/// Every path git tracks, as the hook would see them.
pub fn tracked() -> Vec<String> {
    let out = Command::new("git")
        .args(["ls-files"])
        .current_dir(root())
        .output()
        .expect("git ls-files");
    assert!(out.status.success(), "git ls-files failed");
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}
