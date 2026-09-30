//! `C-69`: the hook that judges every commit, judged.
//!
//! Twelve cases were run by hand when `P-352` landed and nothing re-ran them. These are those
//! cases, plus the one a hand run cannot do: asking whether every path in this tree resolves
//! to a column at all.

use hooks::{column_of, column_of_act, root, tracked};

/// Each path the hook is meant to place, and where.
///
/// **The table is the check and the count is asserted**, because a table that lost its rows
/// would agree with any mapping at all.
#[test]
fn every_kind_of_path_lands_in_the_column_that_owns_it() {
    let cases: [(&str, Option<&str>); 17] = [
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
        // **A case arriving is nobody's** - `P-582`. It is written by the test, and committing
        // what the test wrote is publishing rather than approving, so it spans nothing and rides
        // with whatever lands it. **Removing one is Sean's**, which is the act this spelling
        // cannot ask about; `a_case_arriving_is_nobodys_and_removing_one_is_seans` does.
        ("regression/rules/move.4x", None),
        ("reports/catalog.md", Some("code")),
        ("hooks/pre-commit", Some("code")),
        ("tools/outbox/src/lib.rs", Some("code")),
        // A lens's own, and its own tools directory with it.
        ("lenses/quality/outbox.md", Some("lens:quality")),
        // Generated, owned by nobody, so a commit carrying it spans nothing.
        ("pending.md", None),
        // **The same, and it has to be asked**, because `decide/*` is the specification lane's
        // and this file sits inside it - `S-225`. A spelling that read the directory alone
        // would give every lane's commit a second column the moment the hook rewrote it.
        ("decide/attention.md", None),
        // And the rest of the directory is still that lane's.
        ("decide/proposals.md", Some("spec")),
    ];
    assert_eq!(cases.len(), 17, "the table lost a row");

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
/// **Two deliberate exceptions, both generated and both belonging to no perspective.**
/// `pending.md` is written from every outbox; `decide/attention.md` from the outboxes,
/// `spec/tests/` and `reviewed/`. They are named here so that their absence from a column is a
/// decision on the record rather than a gap that looks like one.
///
/// **This is the check that caught `decide/attention.md` arriving** - the hook's exemption was
/// written and this list was not, and the file becomes tracked in the commit that adds it, which
/// is after any gate run made before it.
#[test]
fn every_tracked_path_is_owned_by_somebody() {
    const OWNED_BY_NOBODY: [&str; 2] = ["pending.md", "decide/attention.md"];

    /// Directories whose files have no column while they are arriving, with what makes that so.
    ///
    /// **`regression/` is the only one and `P-582` is why**: *adding a case and removing one are
    /// different acts on the same path... the deletion is the approval, and it is his alone.* A
    /// case arriving is written by the test, and committing what the test wrote is publishing
    /// rather than approving.
    ///
    /// **The removal side is asserted below and that is what keeps this honest.** An entry
    /// saying only *no column* would go on passing if the hook's `regression/` arm were deleted
    /// outright - every file would be unowned and the exception would excuse it. Asking the
    /// other direction means the entry can only be satisfied by the rule actually being there.
    const NO_COLUMN_ARRIVING: [(&str, &str); 1] = [(
        "regression/",
        "a generated case: arriving it is nobody's, and removing it is Sean's approval",
    )];

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
        if NO_COLUMN_ARRIVING
            .iter()
            .any(|(prefix, _)| file.starts_with(prefix))
        {
            continue;
        }
        if column_of(file).is_none() {
            unassigned.push(file.clone());
        }
    }

    // **Each of these is checked in both directions**, so the entry is satisfied only by the rule
    // being there rather than by the paths having no rule at all.
    let mut both_ways = 0;
    for (prefix, why) in NO_COLUMN_ARRIVING {
        let under: Vec<&String> = files.iter().filter(|it| it.starts_with(prefix)).collect();
        assert!(
            !under.is_empty(),
            "nothing under `{prefix}` is tracked any more, so delete its entry: {why}"
        );
        for file in under {
            assert!(
                column_of(file).is_none(),
                "`{file}` has a column while arriving, so delete its entry: {why}"
            );
            assert_eq!(
                column_of_act(file, "removed"),
                Some("sean".to_string()),
                "removing `{file}` is not Sean's, so the hook has stopped telling the two acts \
                 apart and this entry is excusing a gap rather than describing a rule"
            );
            both_ways += 1;
        }
    }
    assert!(
        both_ways > 50,
        "only {both_ways} file(s) were asked both ways, so this proved almost nothing"
    );

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

/// **Adding a case and removing one are different acts on the same path** - `P-582`.
///
/// # Why a column cannot answer this on its own
///
/// **`CLAUDE.md` -> Perspectives**: *a commit that removes a file under `regression/` is Sean's; a
/// commit that adds one is any lane's... the deletion is the approval, and it is his alone.*
///
/// **`reviewed/` needs no such split because nothing but the review application writes it.** Here
/// both gestures are ordinary and only one of them is his - so the hook is given the act as well
/// as the path, and this is the only directory that reads it.
///
/// **`C-160` is the item that found the gap**, by the column going missing when `D-6` moved the
/// cases out of `scenario/`. This lane restored the old column rather than choosing; Sean chose.
///
/// # Both directions, and over every suite rather than one
///
/// **One case would pass while the pattern matched only that suite.** Four suites exist and each
/// is asked both ways, with the count asserted, so a `case` arm narrowed to `regression/rules/*`
/// fails here rather than leaving three directories unguarded.
#[test]
fn a_case_arriving_is_nobodys_and_removing_one_is_seans() {
    let suites = [
        "scenario/01/01-move",
        "rules/move",
        "types/place",
        "primitives/add",
    ];
    let mut asked = 0;
    for suite in suites {
        let path = format!("regression/{suite}.4x");
        assert_eq!(
            column_of_act(&path, "removed"),
            Some("sean".to_string()),
            "removing `{path}` is Sean's approval and no lane may ride it into a commit"
        );
        assert_eq!(
            column_of_act(&path, "added"),
            None,
            "`{path}` arriving is written by the test, so it belongs to no column and \
             spans nothing"
        );
        asked += 2;
    }
    assert_eq!(
        asked,
        suites.len() * 2,
        "every suite was asked both ways, and the count is what says so"
    );

    // **The act reaches only this directory**, which is what keeps it a distinction rather than a
    // second ownership system. A path with an owner has the same owner either way.
    for path in [
        "crates/game-model/src/engine.rs",
        "spec/logistics.md",
        "reviewed/a.4x",
    ] {
        assert_eq!(
            column_of_act(path, "removed"),
            column_of_act(path, "added"),
            "`{path}` answers differently depending on the act, and only `regression/` may"
        );
    }
}
