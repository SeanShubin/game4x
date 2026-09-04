//! A generated markdown file is written already padded.
//!
//! **Sean, 2026-09-03:** *lets make sure that regarding markdown table padding, that we
//! always generate markdown tables the same way they would end up being padded.*
//!
//! Until this, generating twice from the same data gave two different files, and only a
//! commit made them agree - `hooks/pre-commit` pads on the way in, so a file was narrow when
//! written and wide once committed. That gap is what made `against_the_release.rs` and
//! `tools/outbox` both compare **cells** rather than bytes: a cost paid twice, in two
//! crates, to work around it.
//!
//! **This checks rather than calls, and that is not a compromise.** The obvious fix is for
//! each generator to call `pad_tables`, and it cannot: this package is deliberately outside
//! the workspace - *not part of the game, so it stays out of `cargo build --workspace` and
//! out of `cargo tree`* - and `game-console` compiles into the shipped WASM binary. A
//! dependency would put a documentation tool inside the game to save duplicating one rule.
//!
//! So the rule may be written twice and may not disagree twice. `crates/game-console`
//! computes its own widths; this is what stops that copy drifting, and it fails the moment
//! it does.

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every markdown file in this repository that something writes rather than someone.
///
/// **Listed rather than discovered.** A check that swept the tree for generated files would
/// find them by a marker, and a file that lost its marker would leave the list silently -
/// which is the failure this repository keeps producing. A name added here is a decision;
/// a name missing from here fails the count below.
const GENERATED: [&str; 5] = [
    "catalog.md",
    "pending.md",
    "state.md",
    "entities.md",
    "turns.md",
];

#[test]
fn every_generated_file_is_already_padded() {
    let root = root();
    let mut wrong = Vec::new();
    let mut checked = 0usize;

    for name in GENERATED {
        let at: &Path = &root.join(name);
        let Ok(content) = std::fs::read_to_string(at) else {
            wrong.push(format!("{name} is not there, and it is on the generated list"));
            continue;
        };
        checked += 1;
        let padded = pad_tables::pad_tables(&content);
        if padded == content {
            continue;
        }
        // Name the first line that differs, because a whole-file diff of a padded table is
        // unreadable and the answer is always one column.
        let at_line = content
            .lines()
            .zip(padded.lines())
            .position(|(theirs, ours)| theirs != ours)
            .map(|n| n + 1)
            .unwrap_or(0);
        wrong.push(format!(
            "{name} line {at_line} is not padded as `pad-tables` would pad it:\n    \
             on disk:  {}\n    padded:   {}",
            content.lines().nth(at_line - 1).unwrap_or("").trim_end(),
            padded.lines().nth(at_line - 1).unwrap_or("").trim_end(),
        ));
    }

    assert!(
        wrong.is_empty(),
        "a generated file is not written the way it would be padded:\n\n  {}\n\n\
         Whatever writes it should produce padded tables, or this check will only be true \
         between a commit and the next generation.",
        wrong.join("\n\n  ")
    );

    // **The count is the check on the check.** Padding a file that has no tables in it
    // changes nothing, so a run that found no files would pass for the wrong reason - and so
    // would a run whose list had quietly emptied.
    assert_eq!(
        checked,
        GENERATED.len(),
        "{checked} generated file(s) read of {}; one on the list is missing",
        GENERATED.len()
    );
}

/// The padder has something to do, so the check above is not vacuous.
///
/// Padding is a no-op on a file with no tables, so *every generated file is already padded*
/// would pass over three files that never had a table between them. This shows the rule
/// biting on a table written narrow.
#[test]
fn the_padder_changes_a_narrow_table() {
    let narrow = "| a | bbbb |\n| --- | --- |\n| cccc | d |\n";
    let padded = pad_tables::pad_tables(narrow);
    assert_ne!(padded, narrow, "a narrow table has to be widened");
    assert!(
        padded.contains("| a    | bbbb |"),
        "columns widen to their widest cell, and this is what the generators must match:\n{padded}"
    );
}
