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

#[test]
fn every_committed_dump_is_what_the_scenario_produces() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let generated = dump::generated(&Files(root.join("commands")));

    // The count first, because a loop that stopped finding files would pass by checking
    // none - and this is the check whose whole job is noticing absence.
    assert_eq!(
        generated.len(),
        5,
        "five dump files are generated; `dump::generated` returned {}",
        generated.len()
    );

    let mut stale = Vec::new();
    for (name, text) in &generated {
        let at = root.join(name);
        let Ok(committed) = std::fs::read_to_string(&at) else {
            stale.push(format!("{name} is generated and is not committed"));
            continue;
        };
        if &committed == text {
            continue;
        }
        // Name the line, because a whole-file diff of a padded table is unreadable and the
        // answer is nearly always one row.
        let at_line = committed
            .lines()
            .zip(text.lines())
            .position(|(theirs, ours)| theirs != ours)
            .map(|n| n + 1);
        stale.push(match at_line {
            Some(line) => format!(
                "{name} line {line}:\n    committed: {}\n    generates: {}",
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

    assert!(
        stale.is_empty(),
        "a committed dump is not what the scenario produces:\n\n  {}\n\n\
         Run `cargo run -p game-console --bin dump-state` and commit the result in the same \
         commit as the change that caused it.",
        stale.join("\n\n  ")
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
