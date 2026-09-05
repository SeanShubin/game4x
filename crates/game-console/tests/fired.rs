//! The commands artifact says what actually fired, and is held to the release for it.
//!
//! **`S-24`.** The artifact is only worth anything if the recipe beside each command is the
//! one that ran. Everything here reads `releases/first-release.md` at test time rather than
//! a copy of it, so a recipe added, renamed or removed fails here instead of quietly
//! dropping out of the file a person is deriving from.

use game_console::{Library, fired};
use std::path::{Path, PathBuf};

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn ran() -> Vec<fired::Fired> {
    fired::ran(&Files(root().join("scenario/commands")))
}

/// Every recipe the release declares, with who runs it.
fn declared() -> Vec<(String, String)> {
    let release = std::fs::read_to_string(root().join("releases/first-release.md"))
        .expect("releases/first-release.md");
    let mut out = Vec::new();
    let mut inside = false;
    for line in release.lines() {
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line.trim() == "## Recipes";
            continue;
        }
        if !inside || !line.trim().starts_with("| **") {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').collect();
        let name = cells.first().unwrap_or(&"").trim().trim_matches('*').trim();
        let owner = cells.get(1).unwrap_or(&"").trim();
        if !name.is_empty() {
            out.push((name.to_string(), owner.to_string()));
        }
    }
    out
}

/// The six recipes an `end turn` runs are the six the release calls the world's.
///
/// **The set is checked and the order is not**, and the difference is worth stating rather
/// than leaving for a reader to assume. `spec/turn.md` names four moments - upkeep, then
/// growing or starving, then what expires, then everything becoming ready - and there are
/// six recipes, so two of them are placed by reading rather than by being told. A world
/// recipe added or renamed fails here; one put in the wrong place does not.
#[test]
fn ending_a_turn_runs_exactly_the_recipes_the_release_calls_the_worlds() {
    let mut worlds: Vec<String> = declared()
        .into_iter()
        .filter(|(_, owner)| owner == "world")
        .map(|(name, _)| name)
        .collect();
    worlds.sort();

    let mut ours: Vec<String> = fired::ENDING_A_TURN.iter().map(|s| s.to_string()).collect();
    ours.sort();

    assert_eq!(
        ours, worlds,
        "`ENDING_A_TURN` and the release's world recipes have parted"
    );
    assert_eq!(
        worlds.len(),
        6,
        "six world recipes; the release has {}",
        worlds.len()
    );
}

/// Every player recipe the release declares is fired by something the scenario ran.
///
/// **Asked of the outcome, not of the command word, and that is the whole point of it.**
/// `crates/game-console/tests/dump.rs` asks whether a line of `play.4x` *begins with* the
/// command that can fire each recipe. That check is correct and it is not about what its
/// name says: `move` and `found by land` are two recipes fired by one command word, so one
/// `move pioneer 2` satisfies both - and in fact it founds, so **the recipe `move` has never
/// once been fired by this scenario** while a green check said all nine were covered. This
/// reads `fired::ran`, which asks the model what happened.
///
/// # The exception, named rather than skipped
///
/// `move` is the gap and it is left open deliberately. Fixing it means adding a command to
/// `scenario/commands/play.4x`, and `S-26` says in bold not to change the scenario's
/// commands while Sean is deriving them by hand. So it is one named exception with a reason
/// and a way out, rather than a weaker assertion that hides it. `C-21` is the item; when the
/// scenario is unfrozen the exception goes and this becomes nine of nine.
#[test]
fn every_player_recipe_the_release_declares_is_actually_fired() {
    /// The recipes no command in the scenario has ever fired, and why each is allowed.
    ///
    /// **Empty, and it was not.** `move` sat here because the scenario's one `move` line
    /// founded, so it fired `found by land` and the recipe `move` had never once run.
    /// `P-214` split the command in two and the scenario now says which it means, so the
    /// exception expired on schedule rather than being deleted to make a test pass - the
    /// assertion below fails when an excepted recipe starts firing, and that is what it did.
    const NOT_FIRED: [(&str, &str); 0] = [];

    let players: Vec<String> = declared()
        .into_iter()
        .filter(|(_, owner)| owner == "player")
        .map(|(name, _)| name)
        .collect();
    assert_eq!(
        players.len(),
        9,
        "nine player recipes when this was written; the release has {} ({players:?})",
        players.len()
    );

    let ran = ran();
    let mut missing = Vec::new();
    for recipe in &players {
        if ran
            .iter()
            .any(|one| one.recipes.iter().any(|fired| fired == recipe))
        {
            continue;
        }
        match NOT_FIRED.iter().find(|(name, _)| name == recipe) {
            Some(_) => {}
            None => missing.push(recipe.clone()),
        }
    }
    assert!(
        missing.is_empty(),
        "the scenario declares these player recipes and fires none of them: {missing:?}"
    );

    // An exception that has been repaired is a lie in the other direction, and nothing else
    // would notice: the test would go on passing while claiming a gap that had closed.
    for (name, why) in NOT_FIRED {
        assert!(
            ran.iter()
                .all(|one| one.recipes.iter().all(|fired| *fired != name)),
            "`{name}` is fired now, so delete its exception: {why}"
        );
        assert!(
            players.contains(&name.to_string()),
            "`{name}` is excepted here and the release no longer declares it"
        );
    }
    assert_eq!(
        NOT_FIRED.len(),
        0,
        "no exceptions: all nine player recipes fire"
    );
}

/// Every command in the artifact says something, and none of them says nothing.
///
/// A blank cell in the *fires* column is the one outcome a person deriving by hand cannot
/// act on - it reads the same whether the command fires no recipe or whether this crate
/// failed to work out which. So a command with no recipe carries a reason instead, and this
/// is what says every one of them does.
#[test]
fn a_command_that_fires_no_recipe_says_why_rather_than_leaving_a_gap() {
    let ran = ran();
    let mut silent = Vec::new();
    for one in &ran {
        if one.recipes.is_empty() && one.instead.trim().is_empty() {
            silent.push(one.command.clone());
        }
    }
    assert!(silent.is_empty(), "these say nothing at all: {silent:?}");

    let firing = ran.iter().filter(|one| !one.recipes.is_empty()).count();
    let explained = ran.iter().filter(|one| one.recipes.is_empty()).count();
    assert_eq!(
        firing + explained,
        ran.len(),
        "every command is one or the other"
    );
    assert!(
        firing > 0 && explained > 0,
        "{firing} fire and {explained} explain, and a run with none of either is not this \
         scenario"
    );
}

/// The flattening does something: the artifact is longer than the file it starts from.
///
/// **`run setup` is followed rather than recorded.** `play.4x` is 79 commands and the run is
/// 136, because the design that `setup.4x` reaches through two more files is where half the
/// numbers in `state.md` come from. An artifact that merely copied `play.4x` would look
/// right, be shorter than the truth, and leave a person deriving by hand without the
/// densities.
#[test]
fn the_artifact_is_the_flattening_and_not_a_copy_of_the_scenario_file() {
    let ran = ran();
    let play = std::fs::read_to_string(root().join("scenario/commands/play.4x")).expect("play.4x");
    let lines = play
        .lines()
        .filter(|line| {
            let line = line.trim();
            !line.is_empty() && !line.starts_with('#')
        })
        .count();

    assert_eq!(lines, 79, "play.4x is 79 commands; it is now {lines}");
    assert!(
        ran.len() > lines,
        "the run is {} commands and play.4x is {lines}, so nothing was flattened in",
        ran.len()
    );

    let design = ran.iter().filter(|one| one.turn == 0).count();
    assert!(
        design > 50,
        "only {design} design commands, and the twelve territories alone need more"
    );
    // No `run` line survives into the artifact - it is where commands are kept, not a move.
    assert!(
        ran.iter().all(|one| !one.command.starts_with("run ")),
        "a `run` line reached the artifact"
    );
}

/// Every `# Turn n.` in the scenario is the turn the game is actually on beneath it.
///
/// **This was wrong, and it was wrong in the file Sean derives from by hand.** `play.4x`
/// labelled its blocks 1 to 7 and then jumped to 9, so every comment after the gap read one
/// turn high - a block headed *Turn 9* over commands the dump calls turn 8. A person
/// checking the dump against the commands finds two documents disagreeing and no way to tell
/// which is lying, which is exactly the hunt these artifacts exist to prevent.
///
/// It is a comment, so no test could ever have failed on it and none did. This one can,
/// because `fired::ran` knows which turn each command ran in and the label is right above
/// it.
#[test]
fn every_turn_the_scenario_labels_is_the_turn_it_is_on() {
    let play = std::fs::read_to_string(root().join("scenario/commands/play.4x")).expect("play.4x");
    let ran = ran();

    // Walk the file and the run together. A label applies to the next command after it, and
    // that command's turn is what the model says it is.
    let mut turn_of: Vec<u32> = Vec::new();
    for one in &ran {
        if one.turn > 0 {
            turn_of.push(one.turn);
        }
    }

    let mut labels: Vec<u32> = Vec::new();
    let mut at = 0usize; // how many play commands have been passed
    let mut pending: Option<u32> = None;
    for line in play.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("# Turn ") {
            if let Some(number) = rest.split('.').next().and_then(|n| n.parse::<u32>().ok()) {
                pending = Some(number);
            }
            continue;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(said) = pending.take() {
            let actually = turn_of
                .get(at)
                .copied()
                .unwrap_or_else(|| panic!("the run is shorter than play.4x"));
            assert_eq!(
                said, actually,
                "`# Turn {said}.` sits above `{line}`, which runs in turn {actually}"
            );
            labels.push(said);
        }
        at += 1;
    }

    // Consecutive from one, so a label cannot be skipped the way `Turn 8` was. Each one
    // agreeing with the run individually would not catch a gap: the labels after it were
    // each wrong by one, and each would have been checked against the wrong command.
    assert_eq!(
        labels,
        (1..=labels.len() as u32).collect::<Vec<_>>(),
        "the turn labels are not 1, 2, 3 ... with nothing missed"
    );
    assert_eq!(
        labels.len(),
        9,
        "nine labelled turns; found {}",
        labels.len()
    );
    assert_eq!(
        turn_of.iter().max().copied().unwrap_or(0),
        9,
        "and nine is the last turn a command runs in"
    );
}
