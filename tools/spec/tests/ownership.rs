//! Every top-level path is written down in `CLAUDE.md`, and no two columns claim one.
//!
//! **`P-548` promised this check and it exists because a list went stale.** The Code row named
//! `commands/` for nineteen days after `ddbaed66` deleted that directory, and nothing noticed
//! because nothing was reading the directory - so this reads the filesystem and asks the
//! document about what it finds, never the other way round.
//!
//! **What it catches is a directory nobody assigned.** Sean, 2026-09-24, on why the answer
//! matters: `CLAUDE.md` is the file a lane reads to find out what it may do, and a path it does
//! not mention is a path every lane may reasonably believe is theirs.
//!
//! **Three paths are covered by a word rather than by a name** - `cargo` for the two manifests,
//! `CI` for the workflow directory. Each alias asserts its own necessity, so adding the literal
//! path to the document makes this fail as unnecessary rather than pass in silence.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A path the document covers by a word instead of by its name, and the word.
const ALIASES: &[(&str, &str)] = &[
    ("Cargo.toml", "cargo"),
    ("Cargo.lock", "cargo"),
    (".github/", "CI"),
];

/// Every directory, and every tracked file in the root.
fn top_level() -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for entry in std::fs::read_dir(root()).expect("the repository root") {
        let entry = entry.expect("entry");
        if entry.path().is_dir() {
            out.insert(format!("{}/", entry.file_name().to_string_lossy()));
        }
    }
    let listed = Command::new("git")
        .arg("ls-files")
        .current_dir(root())
        .output()
        .expect("git ls-files");
    assert!(listed.status.success(), "git ls-files failed");
    let listed = String::from_utf8(listed.stdout).expect("utf-8");
    for line in listed.lines() {
        if !line.is_empty() && !line.contains('/') {
            out.insert(line.to_string());
        }
    }
    out
}

/// **`CLAUDE.md` mentions every top-level path**, or covers it by a declared alias.
#[test]
fn nothing_in_the_root_is_unwritten() {
    let text = std::fs::read_to_string(root().join("CLAUDE.md")).expect("CLAUDE.md");
    let paths = top_level();
    assert!(
        paths.len() >= 20,
        "found {} top-level paths, too few to have looked",
        paths.len()
    );

    // An alias that is no longer needed is a defect, the same way an exception is.
    for (path, word) in ALIASES {
        assert!(
            !text.contains(path),
            "`{path}` is named in CLAUDE.md now, so its `{word}` alias must go"
        );
        assert!(
            text.contains(word),
            "`{path}` is covered by `{word}` and CLAUDE.md does not say `{word}`"
        );
    }

    let aliased: BTreeSet<&str> = ALIASES.iter().map(|(path, _)| *path).collect();
    let unwritten: Vec<&String> = paths
        .iter()
        .filter(|path| !aliased.contains(path.as_str()) && !text.contains(path.as_str()))
        .collect();
    assert!(
        unwritten.is_empty(),
        "in the root and not in CLAUDE.md, over {} paths: {unwritten:?}",
        paths.len()
    );
}

/// **No path is claimed by two columns.**
///
/// The column table is the one place ownership is stated as a list, so it is the one place two
/// claims can collide without a reader noticing.
#[test]
fn no_path_is_in_two_columns() {
    let text = std::fs::read_to_string(root().join("CLAUDE.md")).expect("CLAUDE.md");
    let start = text
        .find("| Perspective ")
        .expect("CLAUDE.md states the columns as a table");
    let end = text[start..].find("\n\n").expect("the table ends") + start;

    let mut claimed: Vec<(String, String)> = Vec::new();
    for line in text[start..end].lines() {
        if !line.starts_with("| **") {
            continue;
        }
        let cells: Vec<&str> = line.trim().trim_matches('|').split('|').collect();
        assert_eq!(cells.len(), 3, "a column row has three cells: {line}");
        let who = cells[0].trim().to_string();
        for path in cells[1].split(',') {
            let path = path.trim().trim_matches('`').to_string();
            if path.is_empty() {
                continue;
            }
            claimed.push((path, who.clone()));
        }
    }
    assert!(
        claimed.len() >= 10,
        "read {} claims from the column table, too few to have looked",
        claimed.len()
    );

    let mut twice = Vec::new();
    for (index, (path, who)) in claimed.iter().enumerate() {
        for (other, other_who) in claimed.iter().skip(index + 1) {
            if path == other && who != other_who {
                twice.push(format!("{path}: {who} and {other_who}"));
            }
        }
    }
    assert!(
        twice.is_empty(),
        "claimed by two columns, over {} claims: {twice:?}",
        claimed.len()
    );
}
