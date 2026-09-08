//! `Q-73`: every text file is LF in the working tree, so a fresh clone is this tree.
//!
//! **The defect this pins down was invisible from inside any existing working tree.** Every
//! generated report here was written by its generator, which writes LF; a fresh clone on
//! Windows had them rewritten to CRLF by `core.autocrlf`, and `dump.rs` asking whether
//! `turns.md` contains `"\n# Turn 1\n"` got no. The blob is the same either way and both trees
//! are status-clean, so nothing in a diff, a status or a test run here could show it.
//!
//! **CI could not show it either.** The gate runs on Linux, where `autocrlf` is off, and the
//! only Windows job builds without testing - so the first place it appeared was `pre-push` on
//! a fresh Windows clone, failing for a reason unrelated to whatever was being pushed.
//!
//! **So the check is on the attribute rather than on the bytes.** Asserting that files in this
//! tree contain no carriage return is what an existing tree always satisfies; asking git what
//! it will *write at checkout* is the question that was being got wrong.
//!
//! It lives beside the column check because both are facts about this repository rather than
//! about the game, and both need the same two helpers.

use hooks::{root, tracked};
use std::process::Command;

/// What git will write for these paths at checkout, one line per path.
fn eol_attributes(paths: &[String]) -> Vec<(String, String)> {
    let mut command = Command::new("git");
    command
        .args(["check-attr", "eol", "--"])
        .current_dir(root());
    for path in paths {
        command.arg(path);
    }
    let out = command.output().expect("git check-attr");
    assert!(
        out.status.success(),
        "git check-attr failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|line| {
            // `<path>: eol: <value>`, and a path may contain `: ` so this splits from the end.
            let (path, value) = line.rsplit_once(": ")?;
            let path = path.strip_suffix(": eol")?;
            Some((path.to_string(), value.to_string()))
        })
        .collect()
}

/// Every text file git tracks is checked out with LF.
///
/// **Over every tracked file rather than the ones that broke**, because the next parser to be
/// written will read a file nobody has thought about yet. The population is asserted, and so
/// is the presence of the files that actually failed - a `.gitattributes` narrowed to cover
/// nothing would otherwise pass this.
#[test]
fn every_tracked_text_file_is_checked_out_with_lf() {
    let files = tracked();
    assert!(
        files.len() > 100,
        "only {} tracked files, so this would agree with anything",
        files.len()
    );
    for named in ["reports/turns.md", "crates/game-console/tests/dump.rs"] {
        assert!(
            files.iter().any(|file| file == named),
            "`{named}` is not tracked any more, and it is one of the files this is about"
        );
    }

    let attributes = eol_attributes(&files);
    assert_eq!(
        attributes.len(),
        files.len(),
        "git answered for {} of {} files, so some were not asked about",
        attributes.len(),
        files.len()
    );

    let wrong: Vec<&(String, String)> = attributes
        .iter()
        .filter(|(_, value)| value != "lf")
        .collect();
    assert!(
        wrong.is_empty(),
        "these files are not settled to LF at checkout, so a fresh clone is not this tree:\n  \
         {}\n`.gitattributes` is what settles them - `Q-73`.",
        wrong
            .iter()
            .map(|(path, value)| format!("{path}: {value}"))
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

/// The rule is one line and covers everything, rather than a list that goes stale.
///
/// **A narrowed `.gitattributes` would pass the test above on the day it was narrowed** - it
/// asks about the files that exist now. This asks whether a file nobody has written yet is
/// covered, which is the direction the other one cannot see.
#[test]
fn a_path_nobody_has_written_yet_is_covered() {
    let invented = [
        "some/new/directory/report.md",
        "a.txt",
        "crates/whatever/x.rs",
    ]
    .map(str::to_string);
    let answered = eol_attributes(&invented);
    // **The denominator, said out loud - `Q-74`.** Every assertion here was inside the loop,
    // so an `eol_attributes` that returned nothing passed it having checked nothing. That is
    // the same shape as `Q-72` one file over, in the test written to close `Q-73`: a count
    // over an empty population agrees with anything. Breaking only the parse - not the
    // command - is what exposes it, and this is the line that catches that.
    assert_eq!(
        answered.len(),
        invented.len(),
        "git answered for {} of {} invented paths, so this checked less than it says",
        answered.len(),
        invented.len()
    );
    for (path, value) in answered {
        assert_eq!(
            value, "lf",
            "`{path}` does not exist and is not covered, so the rule is a list rather than a rule"
        );
    }
}
