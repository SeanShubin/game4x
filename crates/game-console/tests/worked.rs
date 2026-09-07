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

/// The one recipe that can have no example, and why it is not a gap in the report.
///
/// **`age` is declared and the model does not implement it** - `C-61`. The release gives food
/// a `keeps` and has `age` turn one into a food that keeps one less; `Trait::Keeps` does not
/// exist, and `Territory::end_of_turn_losses` discards **all** food at every ending. So no
/// state makes `age` do anything, and an example of it would have to be drawn - which is
/// exactly what `P-330` says a worked example must never be.
///
/// **Found by `R-7` rather than by reading the model.** Building the world's example is what
/// asked *what does this recipe do here*, and the answer was nothing.
const NOT_IMPLEMENTED: [(&str, &str); 1] = [(
    "age",
    "the model has no `keeps` and discards all food at every ending, so nothing ages - `C-61`",
)];

/// Every recipe the release declares has a worked example, or is the one that cannot.
///
/// **Five of the six excuses went with `P-332`.** The world's used to be excused as a group
/// because no command fires one alone; Sean chose one example shown under all of them over a
/// command each, so the excuses went and the notices in the report went with them.
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
        .flat_map(|example| std::iter::once(example.recipe).chain(example.also.iter().copied()))
        .collect();

    let unbuilt: BTreeSet<&str> = NOT_IMPLEMENTED.iter().map(|(name, _)| *name).collect();
    let missing: Vec<&String> = declared
        .iter()
        .filter(|name| !covered.contains(name.as_str()) && !unbuilt.contains(name.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "these recipes are declared and have no worked example: {missing:?}"
    );

    // **An exception that has been repaired is a lie in the other direction**, and nothing
    // else would notice: this would go on passing while claiming a gap that had closed.
    for (name, why) in NOT_IMPLEMENTED {
        assert!(
            !covered.contains(name),
            "`{name}` has an example now, so it is implemented and this goes: {why}"
        );
        assert!(
            declared.iter().any(|declared| declared == name),
            "`{name}` is excepted here and the release no longer declares it"
        );
    }

    // **Every recipe, and one example carries six of them.** `P-332`: the world's are shown
    // once, together, on `{end-turn}`. So the two counts differ and the difference is the
    // point - eleven examples for sixteen recipes.
    assert_eq!(
        covered.len() + unbuilt.len(),
        declared.len(),
        "every declared recipe is covered or named unbuilt; {} covered, {} unbuilt, {} declared",
        covered.len(),
        unbuilt.len(),
        declared.len()
    );
    let shared: usize = worked::examples()
        .iter()
        .map(|example| example.also.len())
        .sum();
    assert_eq!(
        shared, 4,
        "one example carries four recipes besides its own - the world's six less `age`, \
         which the model does not implement"
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
