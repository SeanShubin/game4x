//! Every hash an outbox cites is a commit in this repository.
//!
//! `CLAUDE.md` has a producer cite the id of the item it acted on, so the path back to Sean
//! is checkable rather than assumed, and `Q-38`'s reconciliation reads those citations to
//! decide whether an open item has already been settled.
//!
//! **A citation that points at nothing is worse than no citation.** It reads as evidence and
//! answers nothing, which is the same shape as a check nobody runs: absence looks exactly
//! like correctness.
//!
//! This exists because it happened. `C-12`'s own entry cited `4dbd3ac`, a hash written
//! before the commit it was meant to name existed - in the item recording that a check had
//! finally been wired to the gate.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every backticked hex run of seven or more, which is how a hash is written here.
fn cited(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for piece in text.split('`').skip(1).step_by(2) {
        let word = piece.trim();
        if word.len() >= 7 && word.chars().all(|c| c.is_ascii_hexdigit()) {
            found.push(word.to_string());
        }
    }
    found
}

fn is_a_commit(root: &Path, hash: &str) -> bool {
    Command::new("git")
        .current_dir(root)
        .args(["cat-file", "-e", hash])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[test]
fn every_hash_an_outbox_cites_is_a_commit() {
    let root = root();
    // A shallow clone has the files and not the history, and reporting every citation as
    // missing there would be noise rather than a finding.
    if !is_a_commit(&root, "HEAD") {
        return;
    }

    let mut checked = 0usize;
    let mut missing = Vec::new();
    for at in outbox::places(&root) {
        let Ok(text) = std::fs::read_to_string(&at) else {
            continue;
        };
        for hash in cited(&text) {
            checked += 1;
            if !is_a_commit(&root, &hash) {
                missing.push(format!("{}: {hash}", at.display()));
            }
        }
    }

    // Over every case, and how many cases there were: a run that found no citations would
    // pass while checking nothing, which is the failure this file is about.
    assert!(
        checked >= 20,
        "only {checked} citations found across the outboxes; the way one is written has \
         probably changed and this has stopped watching anything"
    );
    assert!(
        missing.is_empty(),
        "an outbox cites a commit that does not exist:\n  {}",
        missing.join("\n  ")
    );
}
