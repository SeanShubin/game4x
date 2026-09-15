//! The user-facing style, rendered from the foundation style.
//!
//! **The translator is not part of the engine** - Sean, 2026-09-15. It lives in `tests/`, which is
//! why it may name anything it likes and why `data/engine.4x` does not grow by a word for it.

mod common;
use common::friendly::Names;
use common::{game_rows, rows};

/// Everything the game store holds, as a person would read it.
#[test]
fn the_game_renders() {
    let all = game_rows();
    let names = Names::of(&all);
    println!("{}", names.all(&all));
}

/// **The first test's world, which is where Sean's note started.**
#[test]
fn the_world_renders_as_the_note_states_it() {
    let all = game_rows();
    let names = Names::of(&all);

    let before: Vec<String> = rows("data/before.4x")
        .iter()
        .map(|r| names.row(r))
        .collect();
    println!("before:\n  {}", before.join("\n  "));

    assert!(
        before.contains(&"residency-1  what=scout  where=territory-1".to_string()),
        "the scout is in territory 1: {before:?}"
    );
    assert!(
        before.contains(&"adjacency-1  from=territory-1  to=territory-2".to_string()),
        "1 is next to 2: {before:?}"
    );
    assert!(
        before.contains(&"scout".to_string()),
        "the thing is the scout"
    );
    assert!(
        before.contains(&"territory-1".to_string()),
        "a territory has no name, so it gets one"
    );
}

/// **A rule reads as a rule**, which is the file the foundation style makes least readable.
#[test]
fn the_rule_renders() {
    let all = game_rows();
    let names = Names::of(&all);

    let rule: Vec<String> = rows("data/rules.4x").iter().map(|r| names.row(r)).collect();
    println!("move:\n  {}", rule.join("\n  "));

    assert!(
        rule.contains(&"clause-3  rule=move  seq=3  role=remove  relation=residency".to_string()),
        "the third clause removes a residency: {rule:?}"
    );
    assert!(
        rule.contains(&"move.it  rule=move  seq=1  of=residency".to_string()),
        "the first input is a residency: {rule:?}"
    );
}

/// **A label names one row, or it is not a label.**
///
/// This is what the uniqueness half of the rule buys: `name` is used only where it picks out one
/// row, and `<relation>-<id>` otherwise. **Without it `column` would render twenty-five rows as
/// `id`**, and a reference to any of them would be unreadable rather than merely ugly.
#[test]
fn every_label_names_exactly_one_row() {
    let all = game_rows();
    let names = Names::of(&all);

    let mut seen: std::collections::BTreeMap<(String, String), usize> = Default::default();
    let mut counted = 0;
    for row in &all {
        // The key is the first column, which is `id` for every relation the data has.
        let Some(id) = row.value("id") else { continue };
        let label = names.label(&row.relation, id);
        *seen.entry((row.relation.clone(), label)).or_default() += 1;
        counted += 1;
    }
    assert!(counted > 150, "only {counted} rows were labelled");

    let shared: Vec<&(String, String)> = seen
        .iter()
        .filter(|(_, how_many)| **how_many > 1)
        .map(|(what, _)| what)
        .collect();
    assert!(
        shared.is_empty(),
        "these labels name more than one row: {shared:?}"
    );
}

/// **The rule is doing something, and this is the case that shows it.**
///
/// `thing` is labelled by its name and `column` is qualified, because twenty-five columns are
/// called `id`. **A rule of *use the name if there is one* would render all of them as `id`.**
#[test]
fn a_name_that_names_many_rows_is_not_used_as_a_label() {
    let all = game_rows();
    let names = Names::of(&all);

    assert_eq!(
        names.label("thing", "1"),
        "scout",
        "one thing is called scout"
    );
    assert_eq!(
        names.label("column", "44"),
        "residency.id",
        "many columns are called `id`, so each is qualified by the relation it belongs to"
    );

    let called_id = all
        .iter()
        .filter(|row| row.relation == "column" && row.value("name") == Some("id"))
        .count();
    assert!(
        called_id > 10,
        "only {called_id} columns are called `id`, which is too few for this to be the case it is about"
    );
}

/// **Every file renders, and this is the half the game store does not cover.**
///
/// `script.4x` and `test.4x` are their own store with their own schema, so rendering them needs
/// the two read together - the same way the engine reads them. `expected.4x` goes with the game's
/// schema, which is what makes it comparable to what a command leaves.
#[test]
fn every_file_in_data_renders() {
    let game = game_rows();
    let of_game = Names::of(&game);

    let mut script = rows("data/script.4x");
    script.extend(rows("data/test.4x"));
    let of_script = Names::of(&script);

    let mut rendered = 0;
    for (file, names) in [
        ("data/schema.4x", &of_game),
        ("data/engine.4x", &of_game),
        ("data/rules.4x", &of_game),
        ("data/before.4x", &of_game),
        ("data/command.4x", &of_game),
        ("data/expected.4x", &of_game),
        ("data/script.4x", &of_script),
        ("data/test.4x", &of_script),
    ] {
        let lines: Vec<String> = rows(file).iter().map(|row| names.row(row)).collect();
        assert!(!lines.is_empty(), "{file} rendered nothing");
        // **A rendering that still shows a bare id has not rendered.** `label` marks a value it
        // could not place with a trailing `?`, so this is what says every reference was resolved.
        for line in &lines {
            assert!(
                !line.contains('?'),
                "{file}: `{line}` points at something that is not there"
            );
        }
        println!("{file}\n  {}\n", lines.join("\n  "));
        rendered += 1;
    }
    assert_eq!(rendered, 8, "all eight files in `data/`");
}
