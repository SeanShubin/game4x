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

/// Whether a fresh clone of this repository would have the commit.
///
/// **Existing here is not the question.** `git cat-file -e` succeeds for any object in the
/// local database, including one no ref points at - an amended commit, a reset branch, a
/// `commit-tree` - and a clone only ever receives what is reachable. So a citation to an
/// orphan passes on the machine that wrote it and fails in CI, which is the most confusing
/// direction for a check to fail in.
///
/// Latent rather than theoretical: all 61 citations were reachable when this was written,
/// and `6650161` had just made the check run in CI for the first time. The gap was found
/// looking for what the first live run could hit.
///
/// **Known and deliberately not handled: this asks *ancestor of `HEAD`*.** Every lane
/// commits to `master` and there are no branches, so the two questions coincide. If this
/// repository ever grows one, a citation to a commit on a pushed side branch is reachable
/// in a clone and would be reported here as unreachable. That is where the false positive
/// will come from, and it is written down so it costs nobody any attention until it does.
///
/// The fix then is a containment test over **all** refs - `git branch --all --contains` -
/// and not over remote ones alone. At pre-push time the commit being pushed is on no remote
/// branch yet: `git branch -r --contains HEAD` is empty right now, so remote-only
/// containment would reject exactly the citations this repository routinely writes, which
/// is how `C-13` cited `ae14f4b` before it was pushed.
fn is_reachable(root: &Path, hash: &str) -> bool {
    Command::new("git")
        .current_dir(root)
        .args(["merge-base", "--is-ancestor", hash, "HEAD"])
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

/// The two questions differ, in a repository built to make them differ.
///
/// **A check that has never been seen to go red is a claim.** This builds a throwaway
/// repository, makes a commit no ref points at, and requires `is_a_commit` to say yes while
/// `is_reachable` says no - which is exactly the state that would pass on a developer's
/// machine and fail in CI, where the clone only has what is reachable.
///
/// It touches nothing outside its own temporary directory, and removes it afterwards.
#[test]
fn an_unreachable_commit_exists_here_and_would_not_survive_a_clone() {
    let at = std::env::temp_dir().join("outbox-reachability-check");
    let _ = std::fs::remove_dir_all(&at);
    std::fs::create_dir_all(&at).expect("a directory to build a repository in");

    let git = |args: &[&str]| -> String {
        let out = Command::new("git")
            .current_dir(&at)
            .args(args)
            .output()
            .unwrap_or_else(|why| panic!("git {args:?}: {why}"));
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };

    git(&["init", "--quiet"]);
    git(&["config", "user.email", "check@example.com"]);
    git(&["config", "user.name", "check"]);
    std::fs::write(at.join("a.txt"), "a").expect("a file to commit");
    git(&["add", "a.txt"]);
    git(&["commit", "--quiet", "-m", "reachable"]);

    let reachable = git(&["rev-parse", "HEAD"]);
    // A commit object with no ref pointing at it, which is what an amend or a reset leaves
    // behind. `commit-tree` writes the object and updates nothing.
    let tree = git(&["rev-parse", "HEAD^{tree}"]);
    let orphan = git(&["commit-tree", &tree, "-p", &reachable, "-m", "unreachable"]);

    assert!(is_a_commit(&at, &reachable) && is_reachable(&at, &reachable));
    assert!(
        is_a_commit(&at, &orphan),
        "the old question: the object is here"
    );
    assert!(
        !is_reachable(&at, &orphan),
        "the question that matters: nothing reaches it, so a clone gets nothing"
    );

    std::fs::remove_dir_all(&at).ok();
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
                missing.push(format!("{}: {hash} is not a commit here", at.display()));
            } else if !is_reachable(&root, &hash) {
                missing.push(format!(
                    "{}: {hash} exists here but nothing reaches it, so a clone will not have it",
                    at.display()
                ));
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
