//! Every recipe the release declares has a worked example, and every example runs.
//!
//! **`R-7`, from `P-330`.** Sean vets it by reading `reports/recipes.md` and deriving the
//! after from the rule and the before by hand. What he cannot check by reading is whether one
//! is *missing* - a recipe with no example looks exactly like a recipe he has not reached yet.
//!
//! So the population is the release's own Recipes table, read at test time: **a recipe the
//! release declares and `worked::examples` does not cover fails here.** That is the only
//! version of this that stays true when a recipe is added.

use std::collections::BTreeSet;
use std::path::PathBuf;

use game_console::worked;

fn release() -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release document")
}

/// Every recipe name, in the order the release's table gives them.
fn declared(document: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line.trim() == "## Recipes";
            continue;
        }
        let line = line.trim();
        if !inside || !line.starts_with("| **") {
            continue;
        }
        let name = line
            .trim_matches('|')
            .split('|')
            .next()
            .unwrap_or_default()
            .trim()
            .trim_matches('*')
            .trim()
            .to_string();
        if !name.is_empty() {
            names.push(name);
        }
    }
    names
}

/// Which recipes have no example, and why each is a decision rather than an omission.
///
/// **The world's six fire on one command and four cannot be shown alone.** `{end-turn}` runs
/// `upkeep`, `grow`, `perish`, `spoil`, `age` and `refresh` in that order, so an example of
/// `grow` is an example of `upkeep` first - a citizen has upkeep, and there is no state with
/// citizens and surplus food in which upkeep does nothing. `R-7` asks for *the command that
/// fires it*, and the world's recipes do not have one each.
///
/// **Named rather than skipped**, so the gap is counted in the file Sean reads rather than
/// looking like a recipe nobody reached. `C-59` carries the question.
const NO_COMMAND: [(&str, &str); 6] = [
    ("upkeep", "fires on `{end-turn}` with five others"),
    (
        "grow",
        "fires on `{end-turn}`, and never without `upkeep` having run first",
    ),
    (
        "perish",
        "fires on `{end-turn}`, and only after `upkeep` has left something unpaid",
    ),
    ("spoil", "fires on `{end-turn}` with five others"),
    ("age", "fires on `{end-turn}` with five others"),
    ("refresh", "fires on `{end-turn}` with five others"),
];

/// Every recipe has an example, or is one of the six the world fires.
#[test]
fn every_recipe_the_release_declares_has_a_worked_example() {
    let declared = declared(&release());
    assert_eq!(
        declared.len(),
        16,
        "sixteen recipes were declared when this was written; the release has {} ({declared:?})",
        declared.len()
    );

    let covered: BTreeSet<&str> = worked::examples()
        .iter()
        .map(|example| example.recipe)
        .collect();
    let excused: BTreeSet<&str> = NO_COMMAND.iter().map(|(name, _)| *name).collect();

    let missing: Vec<&String> = declared
        .iter()
        .filter(|name| !covered.contains(name.as_str()) && !excused.contains(name.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "these recipes are declared and have no worked example: {missing:?}"
    );

    // **An excuse that has been repaired is a lie in the other direction.** This would go on
    // passing while claiming a gap that had closed.
    for (name, why) in NO_COMMAND {
        assert!(
            !covered.contains(name),
            "`{name}` has an example now, so delete its excuse: {why}"
        );
        assert!(
            declared.iter().any(|declared| declared == name),
            "`{name}` is excused here and the release no longer declares it"
        );
    }
    assert_eq!(
        excused.len(),
        6,
        "six recipes are the world's and fire on one command"
    );
    assert_eq!(
        covered.len() + excused.len(),
        declared.len(),
        "every recipe is either worked or excused; {} are worked and {} excused",
        covered.len(),
        excused.len()
    );
}

/// Every example runs, and every one changes something.
///
/// **An example that changed nothing would render two identical states**, which reads as a
/// recipe that does nothing rather than as a fixture that was wrong. `worked::run` panics on
/// a refusal; this is the other half.
#[test]
fn every_example_fires_and_moves_the_state() {
    let examples = worked::examples();
    assert!(
        examples.len() >= 8,
        "only {} examples, so this checks almost nothing",
        examples.len()
    );
    for example in &examples {
        let run = worked::run(example);
        assert!(
            !run.before.is_empty() && !run.after.is_empty(),
            "`{}` shows an empty state",
            run.recipe
        );
        assert_ne!(
            run.before, run.after,
            "`{}` fired and nothing changed, so the example shows nothing",
            run.recipe
        );
        // **Both states name the same places**, which is what *only what it touches* means:
        // the places shown are the ones that differ, in both directions.
        assert_eq!(
            run.before.lines().next(),
            run.after.lines().next(),
            "`{}` shows a different game in each state",
            run.recipe
        );
    }
}
