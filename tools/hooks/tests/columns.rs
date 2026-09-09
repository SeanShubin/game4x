//! `C-69`: the hook that judges every commit, judged.
//!
//! Twelve cases were run by hand when `P-352` landed and nothing re-ran them. These are those
//! cases, plus the one a hand run cannot do: asking whether every path in this tree resolves
//! to a column at all.

use hooks::{column_of, root, tracked};

/// Each path the hook is meant to place, and where.
///
/// **The table is the check and the count is asserted**, because a table that lost its rows
/// would agree with any mapping at all.
#[test]
fn every_kind_of_path_lands_in_the_column_that_owns_it() {
    let cases: [(&str, Option<&str>); 14] = [
        // The specification lane's column.
        ("spec/console.md", Some("spec")),
        ("releases/first-release.md", Some("spec")),
        ("docs/notes/proposals.md", Some("spec")),
        ("README.md", Some("spec")),
        ("CLAUDE.md", Some("spec")),
        ("tools/spec/src/main.rs", Some("spec")),
        // The code lane's, including production support.
        ("crates/game-model/src/game.rs", Some("code")),
        ("prototypes/kinds/src/lib.rs", Some("code")),
        ("scenario/commands/play.4x", Some("code")),
        ("reports/catalog.md", Some("code")),
        ("hooks/pre-commit", Some("code")),
        ("tools/outbox/src/lib.rs", Some("code")),
        // A lens's own, and its own tools directory with it.
        ("lenses/quality/outbox.md", Some("lens:quality")),
        // Generated, owned by nobody, so a commit carrying it spans nothing.
        ("pending.md", None),
    ];
    assert_eq!(cases.len(), 14, "the table lost a row");

    for (path, want) in cases {
        assert_eq!(
            column_of(path).as_deref(),
            want,
            "`{path}` is in the wrong column"
        );
    }
}

/// A lens's `tools/` directory belongs to that lens, and is asked of the tree.
///
/// **Not a list.** `hooks/pre-commit` asks whether `lenses/<name>` exists rather than naming
/// the lenses, so a fourth lens is covered the day it starts. This checks that mechanism on
/// the lenses that are actually there.
#[test]
fn a_lens_owns_its_own_tools_directory() {
    let lenses: Vec<String> = std::fs::read_dir(root().join("lenses"))
        .expect("lenses/")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    assert!(
        lenses.len() >= 2,
        "only {} lens directories, which is too few to be this tree",
        lenses.len()
    );
    for lens in &lenses {
        assert_eq!(
            column_of(&format!("lenses/{lens}/outbox.md")).as_deref(),
            Some(format!("lens:{lens}").as_str()),
            "a lens does not own its own directory"
        );
        assert_eq!(
            column_of(&format!("tools/{lens}/src/lib.rs")).as_deref(),
            Some(format!("lens:{lens}").as_str()),
            "a lens does not own `tools/{lens}`"
        );
    }
    // A tools directory that is not a lens's is production support, and the code lane's.
    assert_eq!(
        column_of("tools/nobody/src/lib.rs").as_deref(),
        Some("code")
    );
}

/// Every path in this tree resolves to a column.
///
/// **This is the direction the hand run could not cover, and the way the hook actually rots.**
/// `column_of` is a `case` over prefixes with an unassigned arm, so a new top-level directory
/// falls through it silently - and a tree where nothing is assigned looks exactly like a tree
/// where nothing spans two columns. **A file nobody has placed cannot make a commit refuse**,
/// so the guard quietly stops covering whatever arrives next.
///
/// `pending.md` is the one deliberate exception: it is generated from every outbox and belongs
/// to no perspective. It is named here so that its absence from a column is a decision on the
/// record rather than a gap that looks like one.
#[test]
fn every_tracked_path_is_owned_by_somebody() {
    const OWNED_BY_NOBODY: [&str; 1] = ["pending.md"];

    /// Placed by nothing, with the reason each is allowed to be here.
    ///
    /// **A named exception fails when it is repaired**, so a gap cannot outlive itself - and
    /// this list is empty because that is exactly what happened. It held
    /// `notes-to-incorporate-then-remove/`, tracked since the first specification commit and
    /// named by `CLAUDE.md` in no column at all, reported as `C-72` and asked of the
    /// specification lane as `P-357`. Sean moved its one file to `temporary-notes/` in
    /// `e6cb9e8`, the directory stopped being tracked, and the check went red asking for its
    /// own exception back - which is the exception working rather than failing.
    ///
    /// **Empty is not a weaker check here.** Nothing rests on this list having entries: the
    /// claim is that every tracked path has a column, and it is asserted below over more than
    /// a hundred files. An entry only ever excused one of them.
    const NOT_PLACED: [(&str, &str); 0] = [];

    let files = tracked();
    assert!(
        files.len() > 100,
        "only {} tracked files, so this would agree with anything",
        files.len()
    );

    let mut unassigned: Vec<String> = Vec::new();
    for file in &files {
        if OWNED_BY_NOBODY.contains(&file.as_str()) {
            assert!(
                column_of(file).is_none(),
                "`{file}` is named as owned by nobody and the hook gives it a column"
            );
            continue;
        }
        if NOT_PLACED
            .iter()
            .any(|(prefix, _)| file.starts_with(prefix))
        {
            continue;
        }
        if column_of(file).is_none() {
            unassigned.push(file.clone());
        }
    }

    // **An exception that has been repaired is a lie in the other direction.**
    for (prefix, why) in NOT_PLACED {
        let still = files.iter().any(|file| file.starts_with(prefix));
        assert!(
            still,
            "nothing under `{prefix}` is tracked any more, so delete its exception: {why}"
        );
        assert!(
            files
                .iter()
                .filter(|file| file.starts_with(prefix))
                .all(|file| column_of(file).is_none()),
            "`{prefix}` has a column now, so delete its exception: {why}"
        );
    }

    assert!(
        unassigned.is_empty(),
        "these tracked files are in no perspective's column, so the hook cannot refuse a \
         commit that mixes them with anything:\n  {}\n\
         Either give them a pattern in `hooks/pre-commit`, or name them above and say why.",
        unassigned.join("\n  ")
    );
}
