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

    let rendered = names.all(&rows("data/foundation/before.4x"));
    println!("{rendered}");

    assert_eq!(
        rendered,
        "{territory id:1 name:territory-1}\n\
         {territory id:2 name:territory-2}\n\
         {territory id:3 name:territory-3}\n\
         {thing id:1 name:scout}\n\
         {adjacency id:1 from:territory-1 to:territory-2}\n\
         {adjacency id:2 from:territory-2 to:territory-3}\n\
                  {residency what:scout where:territory-1} -> 1"
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
        "data/foundation/before.4x",
        "data/foundation/rules.4x",
        "data/foundation/command.4x",
        "data/foundation/schema.4x",
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

/// **Which relations get names, and which are referenced by id instead.**
///
/// Sean's constraint is that a name is unique - per relation, not globally: *I generally do not
/// want anything globally unique.* A reference knows its target relation from the schema, so
/// `what:scout` is looked up among things and cannot be confused with a role of the same name.
///
/// **`column` is the one relation that cannot satisfy it**, because `column.name` is not a name
/// for the row - it is the token a row is keyed by, so sixteen columns are called `id`. Sean chose
/// to leave it that way rather than give `column` a second column: *binding and column are
/// machinery.* **So a reference to a column is written as an id**, and this is what says so.
#[test]
fn a_column_is_referenced_by_id_because_its_name_is_a_token() {
    let game = game_rows();
    let names = Names::of(&game);

    // A column's `name` is the token, and it is not unique: sixteen columns are called `id`.
    let called_id = game
        .iter()
        .filter(|row| row.relation == "column" && row.value("name") == Some("id"))
        .count();
    assert_eq!(called_id, 16, "sixteen columns are called `id`");

    // So no column has a name, and a reference to one is its id.
    assert_eq!(names.name("column", "44"), "44");
    let binding = game
        .iter()
        .find(|row| row.relation == "binding" && row.value("id") == Some("1"))
        .expect("the first binding");
    assert_eq!(
        names.row(binding),
        "{binding id:1 clause:clause-1 column:44 input:what}"
    );

    // The control: a relation whose names are its own is referenced by name.
    assert_eq!(names.name("thing", "1"), "scout");
    assert_eq!(names.name("territory", "1"), "territory-1");
}

/// **Foundation to friendly to foundation is the identity, for every file.**
///
/// This is what authoring in the friendly format rests on. **Nothing is minted**: a friendly row
/// carries its own `id`, so translating back is resolving names to ids and dropping the generated
/// `name`, with no value invented anywhere.
#[test]
fn every_file_survives_the_round_trip() {
    let game = game_rows();
    let of_game = Names::of(&game);
    let mut script = rows("data/foundation/script.4x");
    script.extend(rows("data/foundation/test.4x"));
    let of_script = Names::of(&script);

    let mut checked = 0;
    for (file, names) in [
        ("data/foundation/schema.4x", &of_game),
        ("data/foundation/engine.4x", &of_game),
        ("data/foundation/rules.4x", &of_game),
        ("data/foundation/before.4x", &of_game),
        ("data/foundation/command.4x", &of_game),
        ("data/foundation/expected.4x", &of_game),
        ("data/foundation/script.4x", &of_script),
        ("data/foundation/test.4x", &of_script),
    ] {
        for row in rows(file) {
            let friendly = names.row(&row);
            let parsed = names
                .parse(&friendly)
                .unwrap_or_else(|why| panic!("{file}: `{friendly}`: {why}"));
            let back = names
                .foundation(&parsed)
                .unwrap_or_else(|why| panic!("{file}: {why}"));
            assert_eq!(
                back,
                row,
                "{file}: `{}` rendered as `{friendly}` and came back as `{}`",
                thin_engine::notation::write(&row),
                thin_engine::notation::write(&back)
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 224, "every row in `data/` went round");
}

/// **A counted relation writes its quantity after the brace, and an identified one has no arrow.**
///
/// **The arrow is the game's own notation rather than one invented here.**
/// `scenario/expected/play.4x` writes `{description} -> quantity` 114 times, under a header saying
/// so. **It is the friendly format's and not the notation's**: `src/notation.rs` reads one `{…}`
/// per line and never learns one, which is what keeps the bridge out of the engine.
#[test]
fn a_counted_relation_renders_with_an_arrow() {
    let game = game_rows();
    let names = Names::of(&game);

    let residency = game
        .iter()
        .find(|row| row.relation == "residency")
        .expect("a residency");
    assert_eq!(
        names.row(residency),
        "{residency what:scout where:territory-1} -> 1"
    );

    // **Friendly to foundation is the direction that must work** - Sean, 2026-09-15 - so the
    // arrow is read back as well as written.
    let parsed = names.parse(&names.row(residency)).expect("read back");
    assert_eq!(&names.foundation(&parsed).expect("converted"), residency);

    // The control: `thing` is identified rather than counted, so nothing is appended to it.
    let thing = game
        .iter()
        .find(|row| row.relation == "thing")
        .expect("a thing");
    assert_eq!(names.row(thing), "{thing id:1 name:scout}");
}
