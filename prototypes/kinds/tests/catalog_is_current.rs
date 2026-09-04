//! The committed catalog is the one the release generates today.
//!
//! **A generated file that nobody regenerates is worse than no generated file.** It reads as
//! derived, so a reader trusts it more than prose, and it goes stale without changing - the
//! same shape as a check that stops running. `pending.md` has this problem solved by habit;
//! this solves it by failing.
//!
//! It also makes the rule enforceable. `spec/invariants.md` says a derived form is generated
//! rather than written, and the way that rule breaks is not somebody hand-editing the
//! catalog - it is somebody changing the release and not regenerating, so the catalog
//! quietly describes a game that is no longer specified.

use std::path::Path;

#[test]
fn the_committed_catalog_is_what_the_release_generates() {
    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../catalog.md");
    let committed = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    let generated = kinds::catalog::catalog(&kinds::release::release());

    // Compared by line so the failure names the line rather than the file.
    let mismatch = committed
        .lines()
        .zip(generated.lines())
        .enumerate()
        .find(|(_, (theirs, ours))| theirs != ours);
    if let Some((at, (theirs, ours))) = mismatch {
        panic!(
            "catalog.md line {} is stale:\n  committed: {theirs}\n  generates: {ours}\n\n\
             The release moved and the catalog did not. Run `cargo run -p kinds -- catalog` \
             and commit the result in the same commit as the change that caused it.",
            at + 1
        );
    }
    assert_eq!(
        committed.lines().count(),
        generated.lines().count(),
        "catalog.md has a different number of lines than the release generates; \
         run `cargo run -p kinds -- catalog`"
    );

    // A comparison of two empty strings passes. This says there was something to compare.
    assert!(
        generated.lines().count() > 50,
        "only {} lines generated; the parser has probably stopped finding the tables",
        generated.lines().count()
    );
}

/// The catalog answers something no single table in the release answers.
///
/// **This is the test of whether a derived form was worth generating.** A copy of the tables
/// under a new filename would satisfy *generated rather than written* and be worth nothing.
/// A pioneer's section joins the Kinds, Families, Traits, capacity, Units and Recipes
/// tables, and the release performs that join nowhere.
#[test]
fn a_section_gathers_what_six_tables_say_separately() {
    let generated = kinds::catalog::catalog(&kinds::release::release());
    let section = generated
        .split("\n## ")
        .find(|part| part.starts_with("pioneer"))
        .expect("the release declares a pioneer");

    for said in [
        "founds a territory",          // Kinds
        "**In families** thing, unit", // Families
        "`force`",                     // Traits
        "a capacity of 2",             // What bounds a kind in a territory
        "Costs to produce: 3 metal",   // Units and structures
        "`found by land` consumes 1",  // Recipes
    ] {
        assert!(
            section.contains(said),
            "a pioneer's section does not carry {said:?}, so the join is incomplete:\n{section}"
        );
    }
}

/// The committed `recipes.md` is what the release generates today.
///
/// **The same rule as the catalog, and it earns its place the same way.** `P-228`: a check
/// guards the repetition rather than the one-off. Regenerating after every promotion is
/// exactly the mundane thing nobody remembers - and a stale recipe view is worse than none,
/// because it reads as derived and is therefore trusted harder than prose while being wrong.
#[test]
fn the_committed_recipes_are_what_the_release_generates() {
    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../recipes.md");
    let committed = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    let generated = kinds::recipes::recipes(&kinds::release::release());

    let mismatch = committed
        .lines()
        .zip(generated.lines())
        .enumerate()
        .find(|(_, (theirs, ours))| theirs != ours);
    if let Some((at, (theirs, ours))) = mismatch {
        panic!(
            "recipes.md line {} is stale:\n  committed: {theirs}\n  generates: {ours}\n\n\
             Run `cargo run -p kinds -- recipes` and commit the result.",
            at + 1
        );
    }
    assert_eq!(
        committed.lines().count(),
        generated.lines().count(),
        "recipes.md has a different number of lines than the release generates"
    );

    // Over every recipe, and how many there were: a view of nothing would compare clean.
    let sections = generated.lines().filter(|l| l.starts_with("## ")).count();
    assert_eq!(
        sections,
        kinds::RECIPES.len(),
        "{sections} recipes rendered and {} declared",
        kinds::RECIPES.len()
    );
}
