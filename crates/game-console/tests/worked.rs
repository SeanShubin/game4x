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

/// Every recipe the release declares has a worked example. No exceptions.
///
/// **There have been two sets and both expired on schedule rather than being deleted.** The
/// world's six were excused as a group because no command fires one alone, until `P-332` chose
/// one example shown under all of them. Then `age` alone, because the model discarded all food
/// at every ending and nothing could age - until `P-340` put `age` before `spoil` and made the
/// release's order the model's. **The exception failed the moment `age` gained an example**,
/// which is what a named exception is for: `C-61`.
#[test]
fn every_recipe_the_release_declares_has_a_worked_example() {
    let blocks = declared(&release());
    // **An example belongs to a recipe, and a recipe is a name.** The release states `stow`
    // twice and `discard` four times, once per kind, because `P-373` makes a rule whose
    // subject is a family a rule for each of them - twenty-four blocks under twenty names.
    // Asking for an example per *block* would be asking `discard` to be shown four times to
    // say one thing.
    //
    // **The neighbour assumption broke and failed loudly, which is what it was written to
    // do.** This used `dedup`, which removes only adjacent duplicates, on the ground that a
    // repeated name's blocks sit together - and the note said a version that separated them
    // would fail the count rather than pass quietly. `P-414` separated them: `refresh` is four
    // blocks, then `muster` and `stand`, then two more `refresh` and a `discard`. So the
    // distinct names are taken as a set, which needs no assumption about order at all.
    let mut declared = blocks.clone();
    declared.sort();
    declared.dedup();
    assert_eq!(
        blocks.len(),
        31,
        "the release states thirty-one blocks of recipe rows; it has {} ({blocks:?})",
        blocks.len()
    );
    assert_eq!(
        declared.len(),
        21,
        "those blocks are stated under twenty-one names; there are {} ({declared:?})",
        declared.len()
    );
    assert!(
        declared.len() < blocks.len(),
        "no name is stated twice, so the deduplication above is doing nothing and the two \
         counts are one check"
    );

    let covered: BTreeSet<&str> = worked::examples()
        .iter()
        .flat_map(|example| std::iter::once(example.recipe).chain(example.also.iter().copied()))
        .collect();

    let missing: Vec<&String> = declared
        .iter()
        .filter(|name| !covered.contains(name.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "these recipes are declared and have no worked example: {missing:?}"
    );

    // **Every recipe, and one example carries ten of them.** `P-332`: the world's are shown
    // once, together, on `{end-turn}`. So the two counts differ and the difference is the
    // point - twelve examples for twenty recipes.
    assert_eq!(
        covered.len(),
        declared.len(),
        "every declared recipe has an example; {} covered and {} declared",
        covered.len(),
        declared.len()
    );
    let shared: usize = worked::examples()
        .iter()
        .map(|example| example.also.len())
        .sum();
    // **Nine, and it was five.** The world's six became ten in the saturating rewrite, and
    // one `{end-turn}` is still the only command that fires any of them - so the one example
    // carries nine recipes besides its own.
    // **Ten since `P-414` added `muster` and `stand`**, so one `{end-turn}` carries the
    // world's eleven.
    assert_eq!(
        shared, 10,
        "one example carries ten recipes besides its own, which is the world's eleven"
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
