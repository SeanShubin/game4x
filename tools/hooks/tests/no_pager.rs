//! Nothing in `scripts/` or `hooks/` can stop and wait for somebody to press a key.
//!
//! **Sean hit this on 2026-09-14.** `scripts/push.sh` printed what was about to go with a
//! bare `git log`, which pages when its output is a terminal - `core.pager` is unset in this
//! repository, so the default is `less`. **The push stopped in the middle and waited**, space
//! by space to the end of the list and then q, in a script whose entire point is that it runs
//! unattended to a deployment.
//!
//! # Why the file rather than the call
//!
//! **A `git log` whose output is captured does not page**, because stdout is a pipe rather
//! than a terminal - which is why every `git diff` in the two hooks was safe. But it was safe
//! by how each call happened to be written, and nothing said so: an uncaptured one added
//! later pages, and a hook that pages hangs the commit.
//!
//! So the rule is about the file: **a script that runs a git command which can page sets
//! `GIT_PAGER`.** That is a carrier rather than a reminder, in this repository's own terms -
//! the line is invisible, covers every call in the file including ones added tomorrow, and
//! cannot be forgotten at a moment of confidence. `--no-pager` on one call is the reminder,
//! and it has to be remembered again at the next call.
//!
//! **It is deliberately over-strict**, and that is the correct error here: a file where every
//! call is captured still has to carry the line, which costs one line, where the miss cost a
//! manual step in the middle of a deployment.
//!
//! It lives beside the column and line-ending checks for the reason that one gives: all three
//! are facts about this repository rather than about the game.

use hooks::root;
use std::collections::BTreeSet;

/// The git subcommands that send their output through a pager.
///
/// **Read from git's own behaviour rather than guessed**: these are the ones `git help config`
/// lists under `pager.<cmd>` as paging by default. `status` and `rev-list` are not here
/// because they do not page, and adding them would make this refuse lines that are fine.
const PAGES: [&str; 10] = [
    "log",
    "diff",
    "show",
    "shortlog",
    "branch",
    "tag",
    "blame",
    "grep",
    "reflog",
    "whatchanged",
];

/// Every file in `scripts/` and `hooks/`, which is what this is a claim about.
fn scripts() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for directory in ["scripts", "hooks"] {
        let at = root().join(directory);
        let mut entries: Vec<std::path::PathBuf> = std::fs::read_dir(&at)
            .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()))
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .collect();
        entries.sort();
        for path in entries {
            let name = format!(
                "{directory}/{}",
                path.file_name().unwrap_or_default().to_string_lossy()
            );
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            out.push((name, text));
        }
    }
    out
}

/// Which pager-capable git subcommands a script runs, if any.
///
/// **Matched on `git <word>` with any `-c key=value` between them**, because `git -c
/// core.pager=less log` is still a `log`. A subcommand named inside a comment counts, which
/// is the over-strictness this file argues for rather than a defect: the cost is one line.
fn pages_in(text: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for line in text.lines() {
        let mut rest = line;
        while let Some(at) = rest.find("git ") {
            let after = &rest[at + 4..];
            let mut words = after.split_whitespace();
            let mut word = words.next().unwrap_or_default();
            // `git -c key=value log` - step over the configuration pairs.
            while word == "-c" {
                let _ = words.next();
                word = words.next().unwrap_or_default();
            }
            if PAGES.contains(&word) {
                found.insert(word.to_string());
            }
            rest = after;
        }
    }
    found
}

/// Every script that can page turns the pager off.
#[test]
fn no_script_leaves_the_pager_on() {
    let scripts = scripts();
    // **The population is asserted**, because a check over an empty directory passes and says
    // nothing - and this reads two directories by name, either of which could be renamed.
    assert!(
        scripts.len() >= 20,
        "only {} files in `scripts/` and `hooks/`, so this would agree with anything",
        scripts.len()
    );

    let mut can_page = 0;
    let mut wrong: Vec<String> = Vec::new();
    for (name, text) in &scripts {
        let pages = pages_in(text);
        if pages.is_empty() {
            continue;
        }
        can_page += 1;
        let disabled = text.contains("GIT_PAGER");
        if !disabled {
            wrong.push(format!(
                "{name} runs `git {}` and does not set GIT_PAGER",
                pages.into_iter().collect::<Vec<_>>().join("`, `git ")
            ));
        }
    }

    // **Four, and naming the number is what tells a real pass from a matcher that broke.**
    // `scripts/push.sh` and `scripts/push.ps1` print what is about to go with `git log`;
    // `hooks/pre-commit` and `hooks/post-commit` ask `git diff` what is staged. A fifth is
    // fine and fails here, which is the prompt to read it rather than an error.
    assert_eq!(
        can_page, 4,
        "four files in `scripts/` and `hooks/` run a git command that can page; this found \
         {can_page}, so either one was added or the matcher stopped reading them"
    );
    assert!(
        wrong.is_empty(),
        "a script can stop and wait for a keypress:\n  {}",
        wrong.join("\n  ")
    );
}

/// And the two that drive `gh` turn its pager off too.
///
/// **A separate assertion because it is a separate program with a separate variable.** `gh run
/// view --json jobs` prints the job list straight to the terminal in `push.sh` rather than
/// into a variable, which is exactly the shape that pages.
#[test]
fn no_script_leaves_the_gh_pager_on() {
    let mut driving = 0;
    let mut wrong: Vec<String> = Vec::new();
    for (name, text) in scripts() {
        // `gh` followed by a word, so the letters appearing inside another word do not count.
        let runs = text.lines().any(|line| {
            line.split_whitespace()
                .zip(line.split_whitespace().skip(1))
                .any(|(word, next)| {
                    word.trim_start_matches(['(', '"', '$']) == "gh"
                        && next.chars().next().is_some_and(char::is_alphabetic)
                })
        });
        if !runs {
            continue;
        }
        driving += 1;
        if !text.contains("GH_PAGER") {
            wrong.push(format!("{name} drives `gh` and does not set GH_PAGER"));
        }
    }
    assert_eq!(
        driving, 2,
        "the two push scripts drive `gh`; this found {driving}"
    );
    assert!(
        wrong.is_empty(),
        "a script can stop and wait for a keypress:\n  {}",
        wrong.join("\n  ")
    );
}
