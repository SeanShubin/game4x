//! `temporary-notes/` is Sean's, and the walk does not go in.
//!
//! **`CLAUDE.md`**: *`temporary-notes/` is Sean's and no instance writes there* - and *no
//! instance reads it unless he points at a file*. The directory is not tracked, which is the
//! whole trouble: a file of his was reformatted by a walk with no business in there, and
//! nothing downstream would have said so. `git status` shows an untracked directory the same
//! way whether or not something inside it changed.
//!
//! **Found by the specification lane running the script**, filed as `S-138`. The exposure was
//! never the pre-commit hook, which passes the staged files and can never stage an untracked
//! one; it was a person running `scripts/pad-tables.sh` with no arguments, which is the
//! documented way to run it.
//!
//! # The control is the point of the test
//!
//! **A test that only asserts the note was left alone passes when nothing is walked at all** -
//! a broken path, a typo in the argument, a tool that exits early. So a second file sits
//! beside the skipped directory and *is* padded, and the test asserts both. **One says the
//! walk happened; the other says it stopped at the door.**

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A scratch directory of our own, because this package has no dev-dependencies and taking
/// one for four lines would put a crate in `cargo tree` to save writing them.
fn scratch(named: &str) -> PathBuf {
    let at = std::env::temp_dir().join(format!("pad-tables-{named}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&at);
    fs::create_dir_all(&at).expect("a scratch directory");
    at
}

/// A table nobody has padded, so that padding it is a visible difference.
const NARROW: &str = "| a | bbbb |\n| --- | --- |\n| cccc | d |\n";

fn written(at: &Path, name: &str) -> PathBuf {
    let file = at.join(name);
    fs::create_dir_all(file.parent().expect("a parent")).expect("a directory");
    fs::write(&file, NARROW).expect("a file");
    file
}

#[test]
fn the_walk_pads_what_it_finds_and_does_not_enter_temporary_notes() {
    let at = scratch("notes");
    let kept = written(&at, "kept/note.md");
    let his = written(&at, "temporary-notes/note.md");

    let ran = Command::new(env!("CARGO_BIN_EXE_pad-tables"))
        .arg(&at)
        .output()
        .expect("the tool runs");
    assert!(
        ran.status.success(),
        "the tool failed: {}",
        String::from_utf8_lossy(&ran.stderr)
    );

    // **The control.** If this one is unchanged the walk never happened, and the assertion
    // below would be true for the wrong reason.
    assert_ne!(
        fs::read_to_string(&kept).expect("the padded file"),
        NARROW,
        "nothing was padded at all, so this test says nothing about `temporary-notes`"
    );

    assert_eq!(
        fs::read_to_string(&his).expect("the untouched file"),
        NARROW,
        "`temporary-notes/` is Sean's and the walk went in"
    );

    fs::remove_dir_all(&at).expect("the scratch directory goes");
}
