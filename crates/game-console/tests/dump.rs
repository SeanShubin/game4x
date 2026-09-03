//! The dump names every table and every column, including the empty ones.
//!
//! **That is the requirement, not a nicety.** Sean is reading this to find bad names:
//! *something like that is the only way I am going to be able to actually identify the
//! problems with names.* A table that disappears when it holds nothing takes its column
//! names with it, and those are exactly the ones nobody has reviewed - because a thing that
//! is never present is a thing nobody has looked at.

use std::path::PathBuf;

use game_console::{Library, Session, dump};

/// Command files, read off disk, the way `first_release.rs` reads them.
struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

/// A game played through `commands/play.4x`, which is the state the dump is written for.
fn played() -> Session {
    let files = Files(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../commands"));
    let mut session = Session::new();
    for command in ["run setup", "start", "run play"] {
        session
            .run(command, &files)
            .unwrap_or_else(|why| panic!("`{command}` failed: {why}"));
    }
    session
}

/// Every table has its columns, and every row has one cell per column.
///
/// The second half is what stops a table drifting into meaning something else: a row longer
/// than its header renders as a table with a hidden column, and markdown will not complain.
#[test]
fn every_table_names_its_columns_and_every_row_matches_them() {
    let session = played();
    let tables = dump::tables(&session.game);

    assert!(tables.len() >= 8, "only {} tables", tables.len());
    for table in &tables {
        assert!(
            !table.columns.is_empty(),
            "{} names no columns, so its rows say nothing",
            table.name
        );
        for (at, row) in table.rows.iter().enumerate() {
            assert_eq!(
                row.len(),
                table.columns.len(),
                "{} row {at} has {} cells and {} columns",
                table.name,
                row.len(),
                table.columns.len()
            );
        }
    }
}

/// A table with nothing in it still appears, with its columns and an explicit count.
///
/// **Poison-tested by construction**: this asserts against a game in the *design* phase,
/// where almost everything is empty, so it fails if emptiness ever means omission. Asserting
/// it only against a played game would prove nothing - a played game fills most tables, and
/// the ones it leaves empty are the ones this is about.
#[test]
fn an_empty_table_is_named_rather_than_omitted() {
    let fresh = Session::new();
    let tables = dump::tables(&fresh.game);
    let empty: Vec<&str> = tables
        .iter()
        .filter(|t| t.rows.is_empty())
        .map(|t| t.name)
        .collect();
    assert!(
        !empty.is_empty(),
        "a new game should leave tables empty; if it does not, this test has stopped \
         checking what it was written for"
    );

    let text = dump::markdown(&fresh.game, "a new game");
    for table in &tables {
        assert!(
            text.contains(&format!("## {}\n", table.name)),
            "{} is missing from the document",
            table.name
        );
        for column in table.columns {
            assert!(
                text.contains(column),
                "{} does not name its column {column:?}",
                table.name
            );
        }
    }
    assert!(
        text.contains("*(empty) 0 rows*"),
        "an empty table has to say so, the way sql.html does"
    );
}

/// The dump describes the state it is given, not the latest one.
///
/// **This is what makes browsing intermediate states a loop rather than a rewrite.** Sean
/// said he *may just have to add more reporting that allows me to browse intermediate
/// states*; a dump that reached for the final state would have to be rebuilt to do it.
#[test]
fn the_dump_describes_the_state_it_is_handed() {
    let before = Session::new();
    let after = played();
    assert_ne!(
        dump::markdown(&before.game, "before"),
        dump::markdown(&after.game, "after"),
        "two moments must not render the same"
    );

    // Asserted on the cells rather than on the rendered line. This test used to look for
    // `| play |` and broke the moment the columns were padded to a fixed width - matching
    // rendered bytes made it a test of the layout, which is not what it is about.
    let phase = |session: &Session| {
        dump::tables(&session.game)
            .into_iter()
            .find(|t| t.name == "game")
            .expect("every dump has a game table")
            .rows[0][0]
            .clone()
    };
    assert_eq!(phase(&before), "design", "a new game is in design");
    assert_eq!(phase(&after), "play", "a played game is in play");
}

/// The entity view names every kind, and says plainly when it can name no columns.
///
/// **The empty case is different here and the difference is worth stating.** In the
/// normalized view the columns are written down in code, so an empty table still names them.
/// In this view a column name *is* a component name and components live on entities - so a
/// kind with no instances has no columns to show. It says that, rather than printing an
/// empty header row and implying it knew what would have been there.
#[test]
fn the_entity_view_names_every_kind_and_admits_what_it_cannot_name() {
    let session = played();
    let tables = dump::entity_tables(&session.game);
    let kinds: Vec<&str> = tables.iter().map(|t| t.kind.as_str()).collect();
    assert_eq!(kinds, ["game", "territory", "unit"]);

    let unit = tables.iter().find(|t| t.kind == "unit").expect("listed");
    assert!(unit.rows.is_empty(), "play.4x leaves no unit alive");
    let text = dump::entities_markdown(&session.game, "after");
    assert!(text.contains("## unit"), "an empty kind still gets a table");
    assert!(
        text.contains("no columns either"),
        "and says why it names none"
    );

    // Every row is as wide as the columns, including entities that carry only some of them.
    for table in &tables {
        for row in &table.rows {
            assert_eq!(row.len(), table.columns.len(), "{}", table.kind);
        }
    }
}

/// The HTML is the same rows as the markdown, and carries no game data of its own.
///
/// `spec/invariants.md`: *what the game is made of lives in a data file, not in code and not
/// in markup.* So the assertion is not that the markup looks right - it is that every name
/// in it came from the state, and that the two renderings do not disagree.
#[test]
fn the_html_carries_the_same_rows_and_names_nothing_itself() {
    let session = played();
    let sections = dump::normalized_sections(&session.game);
    let page = dump::html(&sections, "after");

    for section in &sections {
        let (name, columns, rows) = (&section.name, &section.columns, &section.rows);
        assert!(page.contains(name), "the page is missing the {name} table");
        for column in columns {
            assert!(page.contains(column), "{name} is missing column {column:?}");
        }
        assert!(
            page.contains(&format!("{} row(s)", rows.len())),
            "{name} does not say how many rows it has"
        );
    }

    // **The content follows the state, which is the claim worth testing.** A page whose
    // vocabulary came from the markup would be the same page for any game. A fresh game has
    // no territories, so nothing can have told it about grassland.
    //
    // The first version of this asserted that `ark` was absent, and `ark` is legitimately
    // there - the `unit kind` table names every kind from the model's enumeration whether
    // any exist or not, which is the whole point of that table. Asserting a name is absent
    // tests the scenario; asserting the page changes with the state tests the renderer.
    let empty = dump::html(&dump::normalized_sections(&Session::new().game), "before");
    assert_ne!(empty, page, "two states must not render the same page");
    assert!(
        page.contains("grassland"),
        "the played game has grassland territories"
    );
    assert!(
        !empty.contains("grassland"),
        "and a game with no territories cannot have learned that word from the markup"
    );
    assert!(page.contains("<!doctype html>"), "and it is a page");

    // Nothing in the frame around the tables names anything in the game.
    let head = page.split("<body>").next().expect("a page has a head");
    for word in ["grassland", "citizen", "extractor", "pioneer", "territory"] {
        assert!(
            !head.contains(word),
            "the head names {word:?}, so the markup carries game data"
        );
    }
}
