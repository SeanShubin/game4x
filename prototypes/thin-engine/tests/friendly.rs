//! The user-facing format, rendered from the foundation format.
//!
//! **The translator is not part of the engine** - Sean, 2026-09-15. It lives in `tests/`, which is
//! why it may name anything it likes and why `data/engine.4x` does not grow a word for it.

mod common;
use common::friendly::Names;
use common::{game_rows, rows};

/// **`given.4x` renders as Sean wrote it**, line for line.
#[test]
fn the_world_renders_in_the_user_facing_format() {
    let game = game_rows();
    let names = Names::of(&game);

    let rendered = names.all(&common::section("given"));
    println!("{rendered}");

    // **A kind is a relation, so the scout is its own row rather than a residency of a
    // category.** Sean, 2026-09-17, looking at three co-located things written three ways: *the
    // way we specify this is different.* It is not now.
    assert_eq!(
        rendered,
        [
            "{territory id:1 name:territory-1}",
            "{place id:1 of:territory-1 layer:surface name:place-1}",
            "{territory id:2 name:territory-2}",
            "{place id:2 of:territory-2 layer:surface name:place-2}",
            "{territory id:3 name:territory-3}",
            "{place id:3 of:territory-3 layer:surface name:place-3}",
            "{adjacency id:1 from:territory-1 to:territory-2}",
            "{adjacency id:2 from:territory-2 to:territory-3}",
            "{scout where:place-1 moving:1} -> 1",
        ]
        .join(
            "
"
        )
    );
}

/// **Every id survives the rendering**, which is what makes translating back exact rather than a
/// minting problem.
#[test]
fn every_row_keeps_its_id() {
    let game = game_rows();
    let names = Names::of(&game);

    let mut checked = 0;
    for file in ["data/foundation/rules.4x", "data/foundation/schema.4x"] {
        for row in rows(file).into_iter().chain(common::section("given")) {
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
/// for the row - it is the token a row is keyed by, so fourteen columns are called `id`. Sean chose
/// to leave it that way rather than give `column` a second column: *binding and column are
/// machinery.* **So a reference to a column is written as an id**, and this is what says so.
#[test]
fn a_column_is_referenced_by_id_because_its_name_is_a_token() {
    let game = game_rows();
    let names = Names::of(&game);

    // A column's `name` is the token, and it is not unique: twenty columns are called `id`.
    let called_id = game
        .iter()
        .filter(|row| row.relation == "column" && row.value("name") == Some("id"))
        .count();
    assert_eq!(called_id, 20, "twenty columns are called `id`");

    // So no column has a name, and a reference to one is its id.
    assert_eq!(names.name("column", "44"), "44");
    let binding = game
        .iter()
        .find(|row| row.relation == "binding" && row.value("id") == Some("42"))
        .expect("a binding of the move rule");
    assert_eq!(
        names.row(binding),
        "{binding id:42 clause:clause-29 column:134 input:from}"
    );

    // The control: a relation whose names are its own is referenced by name - and a family's
    // values are relations, so `of:unit` is looked up among them.
    assert_eq!(names.name("unit", "28"), "scout");
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
    // **Only the prologue of the test.** Its sections are game rows, and mixing the two stores
    // gives one set of ids two meanings.
    script.extend(
        rows("data/foundation/tests/the-scout-moves-to-an-adjacent-place.4x")
            .into_iter()
            .zip(common::friendly::in_a_section(&rows(
                "data/foundation/tests/the-scout-moves-to-an-adjacent-place.4x",
            )))
            .filter(|(_, section)| !section)
            .map(|(row, _)| row),
    );
    let of_script = Names::of(&script);

    let mut checked = 0;
    for file in [
        "data/foundation/schema.4x",
        "data/foundation/engine.4x",
        "data/foundation/rules.4x",
        "data/foundation/script.4x",
        "data/foundation/setup.4x",
    ]
    .iter()
    .map(|it| it.to_string())
    .chain(common::every_test())
    {
        let file = file.as_str();
        let these = rows(file);
        let mine = common::friendly::in_a_section(&these);
        for (at, row) in these.iter().enumerate() {
            let row = row.clone();
            let names = if file.ends_with("script.4x")
                || file.ends_with("setup.4x")
                || (file.contains("/tests/") && !mine[at])
            {
                &of_script
            } else {
                &of_game
            };
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
    assert!(
        checked > 150,
        "only {checked} rows went round, so a count proves nothing"
    );
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

    let scout = game
        .iter()
        .find(|row| row.relation == "scout")
        .expect("a scout");
    assert_eq!(names.row(scout), "{scout where:place-1 moving:1} -> 1");

    // **Friendly to foundation is the direction that must work** - Sean, 2026-09-15 - so the
    // arrow is read back as well as written.
    let parsed = names.parse(&names.row(scout)).expect("read back");
    assert_eq!(&names.foundation(&parsed).expect("converted"), scout);

    // The control: `territory` is identified rather than counted, so nothing is appended to it.
    let territory = game
        .iter()
        .find(|row| row.relation == "territory")
        .expect("a territory");
    assert_eq!(names.row(territory), "{territory id:1 name:territory-1}");
}
