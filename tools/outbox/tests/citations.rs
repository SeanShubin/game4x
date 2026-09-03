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
///
/// **Except inside a double-backtick span, which is how this repository shows markup
/// literally.** `docs/notes/proposals.md` explains the field-parsing bug by displaying
/// `` **cited** `abc1234` - **source** `1234567abc` `` - two hashes that were never meant to
/// resolve, in a line whose whole purpose is to be an example. Treating them as citations
/// reported a defect in a document that was describing one.
///
/// A displayed hash is not a claim about a commit, which is the thing this checks.
fn cited(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for line in text.lines() {
        // Drop what is being shown rather than said, then read the rest.
        let mut said = String::new();
        let mut rest = line;
        while let Some((before, after)) = rest.split_once("``") {
            said.push_str(before);
            match after.split_once("``") {
                Some((_shown, tail)) => rest = tail,
                None => {
                    rest = "";
                    break;
                }
            }
        }
        said.push_str(rest);

        for piece in said.split('`').skip(1).step_by(2) {
            let word = piece.trim();
            if word.len() >= 7 && word.chars().all(|c| c.is_ascii_hexdigit()) {
                found.push(word.to_string());
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::cited;

    /// A hash being shown is not a hash being cited.
    #[test]
    fn a_displayed_hash_is_not_a_citation() {
        let shown = "into the next field, so `` **cited** `abc1234` - **source** `1234567abc` `` returned both.";
        assert!(cited(shown).is_empty(), "{:?}", cited(shown));

        // And an ordinary citation on the same shape of line still counts.
        let said = "**to** spec · **status** **answered** 2026-09-02 · `a6b67a7`";
        assert_eq!(cited(said), ["a6b67a7"]);

        // A line with both: the shown one is dropped and the said one is kept.
        let both = "`a6b67a7` did it, unlike `` `abc1234` ``";
        assert_eq!(cited(both), ["a6b67a7"]);
    }
}

fn is_a_commit(root: &Path, hash: &str) -> bool {
    Command::new("git")
        .current_dir(root)
        .args(["cat-file", "-e", hash])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Whether this clone has the files without the history.
///
/// **`HEAD` resolving does not answer this**, which is what the guard below used to ask.
/// A shallow clone has exactly one commit and `git cat-file -e HEAD` succeeds in it, so the
/// guard passed and then every historical citation was reported missing. `actions/checkout`
/// is shallow by default, so that is every CI run: this failed the gate on 2026-09-02 with
/// forty-odd citations listed as though the outboxes were wrong.
fn is_shallow(root: &Path) -> bool {
    Command::new("git")
        .current_dir(root)
        .args(["rev-parse", "--is-shallow-repository"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim() == "true")
        .unwrap_or(false)
}

#[test]
fn every_hash_an_outbox_cites_is_a_commit() {
    let root = root();
    // A shallow clone has the files and not the history, and reporting every citation as
    // missing there would be noise rather than a finding.
    if !is_a_commit(&root, "HEAD") || is_shallow(&root) {
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
