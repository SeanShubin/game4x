//! `data/friendly/` is the source and `data/foundation/` is what it converts to.
//!
//! **Sean, 2026-09-15**: *Lets make friendly the source and not omit anything. This presumes we
//! can reliably convert between friendly and foundation. Also it is ok that sometimes they happen
//! to be the same thing.*
//!
//! So both directories hold the same eight files, nothing is left out of either, and **this is
//! what says they say the same thing**. Converting the friendly directory has to produce the
//! foundation directory row for row, and rendering the foundation directory has to produce the
//! friendly one back.

mod common;
use common::friendly::{self, Names};

use common::{mine, rows};

use thin_engine::notation::{Row, write};
use thin_engine::schema::Schema;

/// The eight files, and which store each belongs to.
const FILES: [(&str, bool); 8] = [
    ("schema.4x", true),
    ("engine.4x", true),
    ("rules.4x", true),
    ("before.4x", true),
    ("command.4x", true),
    ("expected.4x", true),
    ("script.4x", false),
    ("test.4x", false),
];

fn store(of_game: bool, from: &str) -> Vec<Row> {
    let mut all = Vec::new();
    for (file, game) in FILES {
        // `expected.4x` is a second world for the same schema, so it is not part of the store
        // the others make up - including it would put two rows of each relation in it.
        if game == of_game && file != "expected.4x" {
            all.extend(of(from, file, of_game));
        }
    }
    all
}

/// One file's rows, from whichever directory.
///
/// **The friendly side may carry `-> n` and the foundation never does**, so friendly files go
/// through the fold that puts a quantity back into its column. **The schema is read first and
/// from the same directory**, because the fold has to know which column that is - and a schema
/// file carries no arrow itself, so reading it needs nothing that is not already there.
fn of(from: &str, file: &str, of_game: bool) -> Vec<Row> {
    let at = format!("data/{from}/{file}");
    if from == "foundation" {
        return rows(&at);
    }
    let declares = if of_game { "schema.4x" } else { "script.4x" };
    let schema = Schema::of(&rows(&format!("data/friendly/{declares}"))).expect("a schema");
    let text =
        std::fs::read_to_string(mine().join(&at)).unwrap_or_else(|why| panic!("{at}: {why}"));
    friendly::fold(&text, &schema).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// **Converting the friendly directory gives the foundation directory, row for row.**
///
/// This is the direction that matters now that friendly is the source: what is committed under
/// `foundation/` is what `friendly/` converts to, and nothing else.
#[test]
fn the_foundation_is_what_the_friendly_source_converts_to() {
    let mut checked = 0;
    for (file, game) in FILES {
        let names = Names::of(&store(game, "friendly"));
        let friendly = of("friendly", file, game);
        let foundation = rows(&format!("data/foundation/{file}"));
        assert_eq!(
            friendly.len(),
            foundation.len(),
            "{file}: {} friendly rows against {} foundation rows",
            friendly.len(),
            foundation.len()
        );
        for (at, row) in friendly.iter().enumerate() {
            let converted = names
                .foundation(row)
                .unwrap_or_else(|why| panic!("data/friendly/{file}: {why}"));
            assert_eq!(
                converted,
                foundation[at],
                "data/friendly/{file}: `{}` converts to `{}` and `foundation/{file}` says `{}`",
                write(row),
                write(&converted),
                write(&foundation[at])
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked, 215,
        "every row of the friendly source was converted"
    );
}

/// **And rendering the foundation gives the friendly source back**, so neither directory can drift
/// from the other without this failing.
#[test]
fn the_friendly_source_is_what_the_foundation_renders_to() {
    let mut checked = 0;
    for (file, game) in FILES {
        let names = Names::of(&store(game, "foundation"));
        let friendly = of("friendly", file, game);
        let foundation = rows(&format!("data/foundation/{file}"));
        for (at, row) in foundation.iter().enumerate() {
            // **Compared as rows and not as text.** A rendering writes the columns in the order
            // the relation declares; `write` sorts them. Two spellings of the same row.
            let rendered = names
                .parse(&names.row(row))
                .unwrap_or_else(|why| panic!("data/foundation/{file}: {why}"));
            assert_eq!(
                rendered,
                friendly[at],
                "data/foundation/{file}: `{}` renders to `{}` and `friendly/{file}` says `{}`",
                write(row),
                names.row(row),
                write(&friendly[at])
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 215, "every row of the foundation was rendered");
}

/// **Both directories hold the same eight files**, because nothing is omitted from either.
#[test]
fn neither_directory_omits_anything() {
    for which in ["friendly", "foundation"] {
        let mut found: Vec<String> = std::fs::read_dir(mine().join("data").join(which))
            .expect(which)
            .filter_map(|it| it.ok())
            .filter_map(|it| it.file_name().to_str().map(str::to_string))
            .filter(|name| name.ends_with(".4x"))
            .collect();
        found.sort();
        let mut wanted: Vec<String> = FILES.iter().map(|(f, _)| f.to_string()).collect();
        wanted.sort();
        assert_eq!(
            found, wanted,
            "`data/{which}` holds a different set of files"
        );
    }
}

/// **A name the foundation has nowhere to keep is refused, not dropped.**
///
/// `territory` declares no `name`, so `{territory id:1 name:home}` cannot be converted. **Silently
/// dropping it would lose an author's work in the format they author in**, which is the one way a
/// conversion that is supposed to be reliable could fail quietly.
#[test]
fn a_name_the_foundation_cannot_keep_is_refused() {
    let names = Names::of(&store(true, "friendly"));
    let row = thin_engine::notation::read("{territory id:1 name:home}").expect("a row");

    let why = names
        .foundation(&row[0])
        .expect_err("there is nowhere to keep `home`");
    assert!(why.contains("nowhere to keep a name"), "{why}");
    assert!(
        why.contains("territory-1"),
        "and it says what the generated name is: {why}"
    );

    // The control: the generated name converts, so the refusal is about the name and not about
    // territories having a name at all.
    let row = thin_engine::notation::read("{territory id:1 name:territory-1}").expect("a row");
    assert_eq!(
        write(&names.foundation(&row[0]).expect("the generated name")),
        "{territory id:1}"
    );
}
