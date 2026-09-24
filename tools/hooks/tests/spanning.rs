//! The race `pre-commit` did not catch, where it actually was, and what now reports it.
//!
//! # What `pre-commit` guarantees, and what it did not
//!
//! **It refuses a commit whose files span two columns**, which `tests/columns.rs` holds it to.
//! What that check reads is the index, at the moment it runs. **The hook then runs for
//! seconds** - `tools/outbox` three times at about 3.3 seconds each, `tools/spec chains` at
//! 2.8, the padder over every staged markdown file, all measured 2026-09-21 - and the index it
//! read is shared by three lanes.
//!
//! **The hook is not wrong; it was early.** `CLAUDE.md` says as much: *staging by name bounds
//! what you add and not what you commit, so no amount of care closes it: the window is between
//! your `git add` and your `git commit`.*
//!
//! # Three times, and the third was not silent
//!
//! Twice before - twenty-six lines, then twenty-one - both recorded in `CLAUDE.md`. The third
//! is `b055da2b`, a promotion of the specification lane's carrying twenty lines of
//! `crates/game-console/tests/first_release.rs`, which the code lane had staged seconds
//! earlier and was about to commit when it hit the index lock.
//!
//! **The first draft of this file said nothing had reported it. That was wrong.** The
//! specification lane checked and found the hook had said so: that commit's output ends
//! *pre-commit: rustfmt on staged Rust files*, which `pre-commit` prints only when Rust is
//! staged, on a promotion that should have carried two markdown files. **The record was silent
//! and the terminal was not**, which are two claims and only the first holds.
//!
//! **That located the window, which was the more useful half of it.** The rustfmt line is near
//! the end of the hook and the column check near the start, so the file arrived *during* the
//! hook's run rather than after it - inside ten seconds of tool runs rather than in an
//! instant.
//!
//! # So the hook checks twice, and `post-commit` reports what is left
//!
//! The second check removes the seconds. Nothing removes the instant between the hook
//! returning and git reading the index, so `post-commit` counts the columns of what landed and
//! says so afterwards - detection, which is all anything can be there.
//!
//! **The evidence is a check that would have failed before it existed**, which is the bar
//! `CLAUDE.md` sets, and it names the commit it would have caught rather than one imagined for
//! it. **A tell somebody has to notice is what a check exists to replace**, which is the
//! specification lane's own account of reading past that line.

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

/// `pre-commit` counts the columns twice, and the second time is the one that would have
/// caught `b055da2b`.
///
/// # Where the window actually was
///
/// **The check at the top reads the index and then the hook runs for seconds.** Measured on
/// 2026-09-21: `tools/outbox` takes about 3.3 seconds and the hook invokes it three times,
/// `tools/spec chains` takes 2.8, and the padder runs over every staged markdown file. So the
/// gap between the column check and the hook returning is the better part of ten seconds.
///
/// **`b055da2b` is inside that gap rather than after it, and the hook's own output says so.**
/// It ends *pre-commit: rustfmt on staged Rust files*, which `pre-commit` prints only when
/// `staged '*.rs'` is non-empty - on a promotion that carried two markdown files and a Rust
/// test. The Rust file was therefore in the index by the time the hook reached that line, and
/// not in it when the hook reached the column check two hundred lines earlier.
///
/// **So the second call removes the seconds, and nothing removes the instant.** A commit can
/// still be raced between the hook returning and git reading the index; `hooks/post-commit`
/// reports that case after the fact, which is all anything can do.
#[test]
fn pre_commit_counts_the_columns_again_before_it_returns() {
    let at = root().join("hooks/pre-commit");
    let text = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));

    let calls = text.matches("\nrefuse_if_two_columns ").count();
    assert_eq!(
        calls, 2,
        "`pre-commit` calls `refuse_if_two_columns` {calls} times; it needs one before its \
         tools run and one after, because the index it reads is shared and the tools are slow"
    );

    // **One implementation, called twice.** Two copies of the count would be the thing being
    // checked written twice, which is the argument `column_of_source` already makes.
    assert_eq!(
        text.matches("refuse_if_two_columns() {").count(),
        1,
        "`refuse_if_two_columns` is defined more than once"
    );

    // The second call has to come after the slow tools, or it is the first check again.
    let last_tool = text
        .rfind("cargo run --quiet --manifest-path tools/")
        .expect("`pre-commit` runs a tool from `tools/`");
    let last_call = text
        .rfind("\nrefuse_if_two_columns ")
        .expect("`pre-commit` calls the check");
    assert!(
        last_call > last_tool,
        "the last column check runs before the last tool, so it cannot see an index that \
         changed while that tool was running"
    );
}

/// And the line that was the tell is still conditional, so it is still a tell.
///
/// **This is the premise the account above rests on**, re-derived rather than taken: if
/// `pre-commit` printed *rustfmt on staged Rust files* unconditionally, that line in
/// `b055da2b`'s output would have said nothing about what was staged, and the window would be
/// unlocated. It is inside an `if`, so it says what it was read to say.
#[test]
fn the_rustfmt_line_is_printed_only_when_rust_is_staged() {
    let at = root().join("hooks/pre-commit");
    let text = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));

    let at_line = text
        .find("echo \"pre-commit: rustfmt on staged Rust files\"")
        .expect("`pre-commit` announces the rustfmt step");
    let before = &text[..at_line];
    let guard = before
        .rfind("rust=$(staged '*.rs')")
        .expect("`pre-commit` reads the staged Rust files");
    assert!(
        before[guard..].contains("if [ -n \"$rust\" ]; then"),
        "the rustfmt announcement is no longer guarded by there being staged Rust, so its \
         presence in a commit's output no longer says a Rust file was staged"
    );
}

/// `post-commit` reports what a pathspec commit left staged, which is `S-166`.
///
/// # The guard was narrower than the hazard it described
///
/// **`pre-commit` stages what it changes** - the regenerated `pending.md`, and every markdown
/// file `tools/pad-tables` repads - and a pathspec commit takes the working tree rather than
/// the index. So whatever the hook staged is still staged afterwards, and the next commit in
/// this tree takes it, whoever runs it.
///
/// **`post-commit`'s own header stated that in general and then guarded one file.** It
/// unstages `pending.md` and said nothing about the rest, which held until
/// `crates/game-inspect/README.md` was committed as it had been padded by hand and the
/// padder's version was left in the index - differing from both `HEAD` and the working tree.
///
/// # Reported rather than unstaged, and that is not a smaller fix
///
/// Unstaging is right for `pending.md`: generated, owned by nobody, and it stays in the
/// working tree. It would have been **wrong** in the instance that prompted this, where the
/// index held the correct file and the working tree held the wrong one - so unstaging would
/// have left a README the padder disagrees with and broken `tools/pad-tables`'s check a lane
/// over.
///
/// # Demonstrated in a scratch repository, both ways
///
/// A pathspec commit with a second file staged reports it and leaves it staged; an ordinary
/// commit of the same index reports nothing and strands nothing. **The second half is what
/// stops this being a hook that fires on every commit**, and it is the specification lane's
/// measurement: staging by name and committing with no pathspec takes the index, so the
/// hook's own changes land.
#[test]
fn post_commit_reports_what_a_pathspec_commit_left_staged() {
    let at = root().join("hooks/post-commit");
    let text = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));

    assert!(
        text.contains("stranded=$(git diff --cached --name-only)"),
        "`hooks/post-commit` no longer reads what is staged after a commit"
    );
    assert!(
        text.contains("these are staged and were not in that commit"),
        "`hooks/post-commit` no longer reports a stranded file"
    );

    // **Reported and not reset**, which is the decision rather than an implementation detail.
    // `git reset` appears once in this file and it is `pending.md`'s.
    assert_eq!(
        text.matches("git reset").count(),
        1,
        "`hooks/post-commit` resets something other than `pending.md`, and unstaging is wrong \
         wherever the index holds the file the working tree should have"
    );
}
