//! The user-facing format, rendered from the foundation format.
//!
//! **The translator is not part of the engine** - Sean, 2026-09-15. It lives in `tests/`, which is
//! why it may name anything it likes and why `data/engine.4x` does not grow a word for it.

mod common;
use common::friendly::Names;
use common::{game_rows, rows};

/// **`before.4x` renders as Sean wrote it**, line for line.
#[test]
fn the_world_renders_in_the_user_facing_format() {
    let game = game_rows();
    let names = Names::of(&game);

    let rendered = names.all(&rows("data/before.4x"));
    println!("{rendered}");

    assert_eq!(
        rendered,
        "{territory id:1 name:territory-1}\n\
         {territory id:2 name:territory-2}\n\
         {territory id:3 name:territory-3}\n\
         {thing id:1 name:scout}\n\
         {adjacency id:1 from:territory-1 to:territory-2}\n\
         {adjacency id:2 from:territory-2 to:territory-3}\n\
         {residency id:1 what:scout where:territory-1}"
    );
}

/// **Every id survives the rendering**, which is what makes translating back exact rather than a
/// minting problem.
#[test]
fn every_row_keeps_its_id() {
    let game = game_rows();
    let names = Names::of(&game);

    let mut checked = 0;
    for file in [
        "data/before.4x",
        "data/rules.4x",
        "data/command.4x",
        "data/schema.4x",
    ] {
        for row in rows(file) {
            let Some(id) = row.value("id") else { continue };
            let rendered = names.row(&row);
            assert!(
                rendered.contains(&format!("id:{id} ")) || rendered.contains(&format!("id:{id}}}")),
                "{file}: `{rendered}` lost its id"
            );
            checked += 1;
        }
    }
    assert!(checked > 100, "only {checked} rows were checked");
}

/// **Every name is unique, which is what makes a reference by name mean one row.**
///
/// **It does not hold today**, and `column` is where. This states which relations satisfy Sean's
/// constraint and which do not, rather than asserting a thing that is false.
#[test]
fn which_relations_can_satisfy_the_uniqueness_constraint() {
    let game = game_rows();
    let names = Names::of(&game);

    let mut per: std::collections::BTreeMap<String, Vec<String>> = Default::default();
    for row in &game {
        let Some(id) = row.value("id") else { continue };
        per.entry(row.relation.clone())
            .or_default()
            .push(names.name(&row.relation, id));
    }

    let mut collide = Vec::new();
    for (relation, mut all) in per {
        let how_many = all.len();
        all.sort();
        all.dedup();
        if all.len() != how_many {
            collide.push(format!("{relation} ({how_many} rows, {} names)", all.len()));
        }
    }

    assert_eq!(
        collide,
        vec!["column (46 rows, 17 names)"],
        "`column` is the one relation whose names are not unique, because `column.name` is the \
         token a row is keyed by - local to its relation - rather than a name for the row"
    );
}
