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

/// The all-zero object id, which names nothing by convention rather than by accident.
///
/// **git's own null id**, written when a ref points at no object - the old value in a
/// `pre-receive` line for a branch being created, and what a reader means by *a hash that
/// exists nowhere*. Nobody cites it; an outbox that contains one is describing the absence.
///
/// **Exempted because it is a convention and not because it is inconvenient.** Every other
/// unreachable hash stays a finding, including one an author is only showing - see
/// [`cited`], which drops what is inside double backticks and reads everything else.
fn names_nothing(hash: &str) -> bool {
    hash.chars().all(|c| c == '0')
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
    use super::{cited, names_nothing};

    /// The all-zero id names nothing by convention, and every other absent hash still counts.
    ///
    /// **Written after an outbox described one.** The quality lens explained why a poison could
    /// not have exercised the reachability arm - *repointing a citation at `0000000` is a hash
    /// that exists nowhere* - and this reported it as an outbox citing a missing commit, which
    /// is a defect found in a sentence about that defect.
    ///
    /// **The exemption is the convention and not the inconvenience.** `0000000` is git's own
    /// null object id; a hash that merely happens not to resolve is still a finding, because
    /// the reader of an outbox cannot tell a hash that was wrong from one that has stopped
    /// existing - which is the whole of why this check exists.
    #[test]
    fn the_null_object_id_is_not_a_citation_and_nothing_else_is_excused() {
        assert!(names_nothing("0000000"));
        assert!(names_nothing("0000000000000000000000000000000000000000"));

        let mut refused = 0;
        for hash in ["abc1234", "69ae559", "0000001", "1000000", "deadbee"] {
            assert!(
                !names_nothing(hash),
                "`{hash}` is not the null id and must be checked like any other"
            );
            refused += 1;
        }
        assert_eq!(refused, 5, "five hashes that are not the null id");

        // And it is still read out of the text, so the exemption is at the question rather
        // than at the reader - a citation of it is found and then excused, which is what lets
        // the doc comment above be about a convention instead of about a parser.
        assert_eq!(cited("filed at `0000000` for now"), ["0000000"]);
    }

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

/// What a clone of this repository would make of one cited hash.
///
/// **Five answers rather than two.** `is_a_commit` and `is_reachable` together could say *absent*
/// or *present*, and a reader of a failure had to guess which of several causes it was. **An
/// ambiguous short hash and a hash naming a tree used to read as the same thing** as a hash that
/// was never here.
#[derive(Debug, PartialEq, Eq)]
enum Verdict {
    /// Resolves to a commit a fresh clone would have.
    Good,
    /// No object here by that name.
    NotAnObject,
    /// Several objects share the prefix, so it names no one commit.
    Ambiguous,
    /// An object, and not a commit - a blob or a tree. `cat-file -e` passed these.
    NotACommit(String),
    /// A commit this clone holds and nothing reaches: what an amend or a reset leaves behind.
    Unreachable,
}

/// What this clone can say about a batch of cited hashes, in two `git` processes.
///
/// # Why a batch at all
///
/// **One `git` spawn per hash, twice, over 1,020 citations was fifty seconds** - and fifty seconds
/// is why this ran in the gate and in `hooks/pre-push` and **not in `hooks/pre-commit`**, which is
/// the only moment that would have caught `S-262`: a run id written in single backticks, committed
/// because the gate that would have refused it had run before the item was written.
///
/// **Two processes regardless of how many citations there are.** `cat-file --batch-check` resolves
/// every hash at once and says what each object *is*; `rev-list HEAD` is the set a fresh clone
/// would get. A citation is good when it resolves to a `commit` that is in that set.
///
/// # It is a stronger question than the one it replaces
///
/// `cat-file -e` succeeded for **any** object - a blob or a tree would have passed as a commit, and
/// the comment above `is_reachable` already said *existing here is not the question*. This reads the
/// type, so a hash naming a tree now fails and says which it was.
fn verdicts(root: &Path, hashes: &[String]) -> std::collections::HashMap<String, Verdict> {
    let mut out = std::collections::HashMap::new();
    if hashes.is_empty() {
        return out;
    }

    // **Every hash resolved in one call.** The input line is echoed back, so a reply is matched to
    // its request by that rather than by position - `cat-file` reorders nothing, and relying on
    // that would be a premise nothing checks.
    // **Each hash is sent twice and comes back once**, because `--batch-check` does not echo
    // the input it found - it prints `<oid> <type> <size>`, and **the first version of this
    // read the size as the type**: a committed commit came back `NotACommit("160")` and every
    // short hash in the tree came back *not a commit here*, because the map was keyed by full
    // oids that nothing looked up. **`%(rest)` is everything after the first space of the
    // input**, so the second copy travels through and the reply names what was asked.
    let asked = hashes
        .iter()
        .map(|it| format!("{it} {it}"))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n";
    let resolved = {
        let mut child = Command::new("git")
            .current_dir(root)
            .args([
                "cat-file",
                "--batch-check=%(objectname) %(objecttype) %(rest)",
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap_or_else(|why| panic!("git cat-file --batch-check: {why}"));
        use std::io::Write;
        child
            .stdin
            .take()
            .expect("a pipe to cat-file")
            .write_all(asked.as_bytes())
            .expect("to send the hashes");
        let out = child.wait_with_output().expect("cat-file to finish");
        String::from_utf8_lossy(&out.stdout).to_string()
    };

    // **The reachable set, which is what a clone would have.** An orphan - what an amend or a reset
    // leaves behind - is an object this clone holds and no clone would fetch.
    let reachable: std::collections::HashSet<String> = {
        let out = Command::new("git")
            .current_dir(root)
            .args(["rev-list", "HEAD"])
            .output()
            .unwrap_or_else(|why| panic!("git rev-list HEAD: {why}"));
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|it| it.trim().to_string())
            .filter(|it| !it.is_empty())
            .collect()
    };

    for line in resolved.lines() {
        // **A refusal names the whole input line**, so `<hash> <hash> missing` has the
        // asked-for hash first; a hit is `<oid> <type> <hash>` with the echo last. **Read by
        // shape rather than by position alone**, which is what the size being mistaken for a
        // type cost.
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (asked_for, verdict) = match parts.as_slice() {
            [asked, .., "missing"] => (*asked, Verdict::NotAnObject),
            [asked, .., "ambiguous"] => (*asked, Verdict::Ambiguous),
            [oid, "commit", asked] => (
                *asked,
                if reachable.contains(*oid) {
                    Verdict::Good
                } else {
                    Verdict::Unreachable
                },
            ),
            [_, kind, asked] => (*asked, Verdict::NotACommit((*kind).to_string())),
            _ => continue,
        };
        out.insert(asked_for.to_string(), verdict);
    }
    out
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

    // **The batch path, which is what the sweep runs** - a writer and a reader each checked alone
    // leaves the composition unchecked, and this is the only test that has an unreachable commit
    // to check it against.
    let said = verdicts(&at, &[reachable.clone(), orphan.clone()]);
    assert_eq!(
        said.get(&reachable),
        Some(&Verdict::Good),
        "a committed commit is good"
    );
    assert_eq!(
        said.get(&orphan),
        Some(&Verdict::Unreachable),
        "the question that matters: the object is here and nothing reaches it, so a clone gets          nothing"
    );
    // **Both answers came from one pair of processes**, which is the property that let this move
    // into `hooks/pre-commit`.
    assert_eq!(said.len(), 2, "one call answered both");

    // **The other verdicts, so none of them is a branch nothing runs.** `NotACommit` is the one
    // `cat-file -e` could not see at all - it succeeds for any object, so a hash naming a tree
    // passed as a commit for as long as this check has existed.
    let tree_said = verdicts(&at, &[tree.clone(), "0123456789abcdef".to_string()]);
    assert_eq!(
        tree_said.get(&tree),
        Some(&Verdict::NotACommit("tree".to_string())),
        "a tree is an object and not a commit, which the old question could not ask"
    );
    assert_eq!(
        tree_said.get("0123456789abcdef"),
        Some(&Verdict::NotAnObject),
        "nothing here by that name"
    );

    // **`Ambiguous` is not constructed here and that is deliberate.** Forcing two objects to share
    // a seven-character prefix means mining for a collision, which would make this test slow and
    // non-deterministic for one message. **It is reachable rather than exercised**, and says so.

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
    let mut asked: Vec<(PathBuf, String)> = Vec::new();
    for at in outbox::places(&root) {
        let Ok(text) = std::fs::read_to_string(&at) else {
            continue;
        };
        for hash in cited(&text) {
            if names_nothing(&hash) {
                continue;
            }
            asked.push((at.clone(), hash));
        }
    }

    // **One pair of `git` processes for every citation in the tree**, which is what makes this
    // cheap enough to run before a commit rather than after one.
    let wanted: Vec<String> = asked.iter().map(|(_, hash)| hash.clone()).collect();
    let said = verdicts(&root, &wanted);
    for (at, hash) in &asked {
        checked += 1;
        let why = match said.get(hash) {
            Some(Verdict::Good) => continue,
            Some(Verdict::NotAnObject) | None => "is not a commit here".to_string(),
            Some(Verdict::Ambiguous) => {
                "names more than one object here, so it names no one commit".to_string()
            }
            Some(Verdict::NotACommit(kind)) => {
                format!("is a {kind} rather than a commit")
            }
            Some(Verdict::Unreachable) => {
                "exists here but nothing reaches it, so a clone will not have it".to_string()
            }
        };
        missing.push(format!("{}: {hash} {why}", at.display()));
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
