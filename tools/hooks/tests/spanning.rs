//! The race `pre-commit` cannot catch, and the thing that says it happened.
//!
//! # What `pre-commit` guarantees, and what it does not
//!
//! **It refuses a commit whose files span two columns**, which `tests/columns.rs` holds it to.
//! What that check reads is the index, at the moment the hook runs. Then the hook returns and
//! git reads the index to build the tree, and **nothing runs in between** - so a `git add` from
//! another lane landing in that gap puts a file into a commit the hook already approved.
//!
//! **The hook is not wrong; it is early.** `CLAUDE.md` says as much: *staging by name bounds
//! what you add and not what you commit, so no amount of care closes it: the window is between
//! your `git add` and your `git commit`.*
//!
//! # It has happened three times and was silent every time
//!
//! Twice before - twenty-six lines, then twenty-one - both recorded in `CLAUDE.md` and both
//! found afterwards. The third is `b055da2b` on 2026-09-21, a promotion of the specification
//! lane's carrying `crates/game-console/tests/first_release.rs`, which the code lane had staged
//! seconds earlier and was about to commit when it hit the index lock.
//!
//! **Nothing reported any of the three.** The third was noticed only because the lane whose
//! file it was read `git log` for an unrelated reason a minute later.
//!
//! # So `post-commit` says so, and this is what says it would have
//!
//! Detection rather than prevention, because nothing can run between the hook and the write.
//! **The evidence is a check that would have failed before it existed**, which is the bar
//! `CLAUDE.md` sets - and this one names the commit it would have caught, so it rests on a
//! failure that actually occurred rather than on one imagined for it.

use std::process::Command;

use hooks::{column_of, root};

/// Which columns a commit's files fall into, the way `post-commit` counts them.
fn columns_of(commit: &str) -> Vec<String> {
    let out = Command::new("git")
        .args(["show", "--name-only", "--format=", commit])
        .current_dir(root())
        .output()
        .expect("git show");
    assert!(out.status.success(), "git show {commit} failed");

    let mut found: Vec<String> = Vec::new();
    for file in String::from_utf8_lossy(&out.stdout).lines() {
        if file.trim().is_empty() {
            continue;
        }
        if let Some(column) = column_of(file.trim())
            && !found.contains(&column)
        {
            found.push(column);
        }
    }
    found.sort();
    found
}

/// **The commit that got through, and it still spans two columns.**
///
/// `b055da2b` is not going to change - a commit is immutable and `CLAUDE.md` forbids amending
/// one here - so this is a fixed case rather than a moving target, and it fails only if the
/// column mapping stops putting these two files in different columns. **That would itself be
/// worth knowing**, because the whole guard rests on that mapping.
#[test]
fn the_commit_the_hook_approved_and_should_not_have_spans_two_columns() {
    let columns = columns_of("b055da2b");
    assert_eq!(
        columns,
        vec!["code".to_string(), "spec".to_string()],
        "`b055da2b` carried the specification lane's promotion and the code lane's test, and \
         the mapping no longer says so"
    );
}

/// And an ordinary commit of one lane's work does not, so the check is not simply always true.
///
/// **Two cases and not one.** A `columns_of` that returned two columns for everything would
/// pass the test above, which is the reason this one is here rather than being obvious.
#[test]
fn a_commit_of_one_lanes_work_spans_one_column() {
    // The code lane's, and every file in it is `crates/` or `prototypes/` or `reports/`.
    let columns = columns_of("fec232bd");
    assert_eq!(
        columns,
        vec!["code".to_string()],
        "`fec232bd` is this lane's prototype work and nothing else"
    );
}

/// `post-commit` reads the mapping out of `pre-commit` rather than carrying its own copy.
///
/// **The copy is what rots** - the argument `tools/hooks/src/lib.rs` already makes for
/// extracting the same function by text. This says the extraction is still the one that file
/// performs, so a rename of `column_of` breaks both together rather than leaving the
/// post-commit half quietly testing nothing.
#[test]
fn post_commit_extracts_the_mapping_from_pre_commit() {
    let at = root().join("hooks/post-commit");
    let text = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));

    assert!(
        text.contains("/^column_of() {/,/^}$/p"),
        "`hooks/post-commit` no longer lifts `column_of` out of `hooks/pre-commit`, so the \
         two can now disagree about what a column is"
    );
    assert!(
        !text.contains("crates/*|web/*"),
        "`hooks/post-commit` carries its own copy of the column mapping, which is the thing \
         being checked written twice"
    );
    assert!(
        text.contains("spans $# perspectives' columns"),
        "`hooks/post-commit` no longer reports a commit that spanned two columns"
    );
}
