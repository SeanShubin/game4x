//! The committed dump files are what the scenario produces now.
//!
//! **`S-29`'s free one, and it is free because it depends on nothing.** A generated file has
//! to be current whatever generates it, and five of the seven generated files in this
//! repository were held to nothing at all.
//!
//! **A generated file that nobody regenerates is worse than no generated file.**
//! `prototypes/kinds/tests/catalog_is_current.rs` says it first and it generalises: a
//! derived file reads as more authoritative than prose *because* it is derived, so it is
//! trusted harder while it goes stale. And it goes stale without changing, which is the same
//! shape as a check that stops running.
//!
//! It matters more than usual this week. Sean is deriving `commands/play.4x` by hand against
//! `state.md` and `turns.md`; a stale file would send him looking for an error in his
//! arithmetic that is really an error in the file.
//!
//! **Two of the seven are not checked here and both omissions are deliberate.**
//! `catalog.md` is `prototypes/kinds`', which already holds it. `pending.md` is written by
//! `hooks/pre-commit`, which **refuses** while any outbox has unstaged changes - so that it
//! never renders somebody's half-written finding. It can therefore be legitimately stale,
//! and requiring it to be current would fail on a correct refusal. Making the hook
//! unconditional to satisfy a test would be trading a real safeguard for a green light.

use std::path::{Path, PathBuf};

use game_console::{Library, dump};

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Every file carrying the generated marker, found on disk rather than listed here.
///
/// **This is the half a list cannot do.** A test that regenerates each *known* file and
/// compares it will pass while the program grows output nobody asked for, or while a file it
/// stopped producing sits on disk being read - because the list of known files is the thing
/// that went stale. So the set is discovered, and the marker is what makes discovery
/// possible: every generated file says `Generated. Do not edit.` in its own first lines.
///
/// `catalog.md` carries the marker and is excluded by name: `prototypes/kinds` produces it
/// and already holds it to being current. `pending.md` carries no marker, which is just as
/// well - `hooks/pre-commit` refuses to rewrite it while an outbox has unstaged changes, so
/// it can be correctly stale and does not belong in a currency comparison at all.
///
/// # Why only this directory
///
/// **`S-33`: the predicate is unbounded content matching, and the scope is what saves it.**
/// Any file containing that sentence matches - and `docs/notes/proposals.md` contains it
/// three times, because reporting a defect in a generated file's header means writing that
/// header down. The scan does not see it only because generated files all happen to live in
/// the repository root today, which is a fact nothing states.
///
/// So it is stated here, and the count below is what keeps it honest. **If a generated file
/// ever lands outside the root** - `P-224` says only *a file of its own* - widening this
/// scan picks up prose immediately, and the count fails rather than the extra file passing
/// as a dump. **If a hand-written root file ever quotes the marker**, the same. `README.md`
/// and `CLAUDE.md` are both in the root and neither carries it, but nothing stops one.
///
/// A marker that cannot be quoted in prose would be stronger and costs every generated file
/// a change. Not worth it while the count holds.
fn marked_on_disk(root: &Path) -> Vec<String> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if name == "catalog.md" {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        // Only the head, so a file *discussing* the marker is not mistaken for one carrying
        // it. Twenty-four lines because the HTML puts it below its stylesheet, at line 20 -
        // twelve found the three markdown dumps and silently missed both pages, which is the
        // failure this test is about, arriving inside the test itself.
        let head: String = text.lines().take(24).collect::<Vec<_>>().join(
            "
",
        );
        if head.contains("Generated. Do not edit.") {
            found.push(name);
        }
    }
    found.sort();
    found
}

/// The committed dumps are what the scenario produces, in all three directions.
///
/// **`missing`, `extra` and `different`, summed and asserted at zero.** Two of those a list
/// can find and one it cannot: `extra` is a file on disk that nothing produces any more, and
/// no amount of regenerating what is listed will notice it, because the list is what went
/// stale. That file goes on being read after it has stopped being true - and it reads as
/// authoritative, because it says it was generated.
#[test]
fn every_committed_dump_is_what_the_scenario_produces() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let generated = dump::generated(&Files(root.join("commands")));

    assert_eq!(
        generated.len(),
        5,
        "five dump files are generated; `dump::generated` returned {}",
        generated.len()
    );

    let produced: std::collections::BTreeSet<String> =
        generated.iter().map(|(name, _)| name.to_string()).collect();
    let on_disk: std::collections::BTreeSet<String> = marked_on_disk(&root).into_iter().collect();

    let missing: Vec<&String> = produced.difference(&on_disk).collect();
    let extra: Vec<&String> = on_disk.difference(&produced).collect();

    let mut different = Vec::new();
    for (name, text) in &generated {
        let Ok(committed) = std::fs::read_to_string(root.join(name)) else {
            continue; // counted as missing above
        };
        if &committed == text {
            continue;
        }
        let at_line = committed
            .lines()
            .zip(text.lines())
            .position(|(theirs, ours)| theirs != ours)
            .map(|n| n + 1);
        different.push(match at_line {
            Some(line) => format!(
                "{name} line {line}:\n      committed: {}\n      generates: {}",
                committed.lines().nth(line - 1).unwrap_or("").trim_end(),
                text.lines().nth(line - 1).unwrap_or("").trim_end()
            ),
            None => format!(
                "{name} is {} lines committed and {} generated",
                committed.lines().count(),
                text.lines().count()
            ),
        });
    }

    let wrong = missing.len() + extra.len() + different.len();
    assert_eq!(
        wrong,
        0,
        "the committed dumps and the scenario disagree:\n\n  \
         missing ({}): {missing:?}\n  \
         extra ({}): {extra:?} - on disk, marked generated, and produced by nothing\n  \
         different ({}):\n    {}\n\n\
         Run `cargo run -p game-console --bin dump-state` and commit the result in the same \
         commit as the change that caused it.",
        missing.len(),
        extra.len(),
        different.len(),
        different.join("\n    ")
    );

    // The set was discovered, so it can be empty for the wrong reason. This says it was not.
    assert_eq!(
        on_disk.len(),
        5,
        "five files carry the generated marker; found {} ({on_disk:?})",
        on_disk.len()
    );
}

/// The scenario produces something, so the comparison above is not two empty strings.
///
/// **Every file being identical is the same green as every file being blank.** This is the
/// half that says the generator did any work, and it reads the content rather than the
/// count, because five empty files would satisfy a count of five.
#[test]
fn the_scenario_produces_tables_rather_than_empty_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let generated = dump::generated(&Files(root.join("commands")));

    for (name, text) in &generated {
        assert!(
            text.lines().count() > 20,
            "{name} is {} lines, which is not a dump of anything",
            text.lines().count()
        );
        let marker = if name.ends_with(".html") {
            "<td>"
        } else {
            "| "
        };
        assert!(
            text.contains(marker),
            "{name} contains no {marker:?}, so it has no rows in it"
        );
    }
}
