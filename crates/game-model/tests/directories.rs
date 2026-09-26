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
use friendly_notation::{self as friendly, Names};

use common::{foundation_at, mine, rows};

use game_model::notation::{Row, write};
use game_model::schema::Schema;

/// Every file in a directory: the shared ones, then one per test.
///
/// **Read rather than listed.** Sean, 2026-09-15: *I intend to have one test per file*, so a list
/// here would be a second place to remember - and `data/{d}/tests/` holding only tests is what
/// makes reading it safe.
/// Every row in `data/foundation`, so a count below is derived rather than written down.
///
/// **Adding a test must not mean editing a number.** Sean, 2026-09-15: *I intend to have one test
/// per file*, and a total written by hand is a line every new test would have to move. **The floor
/// is what keeps it honest** - a derived total compared against itself passes over an empty
/// directory, which is `CLAUDE.md`'s count over nothing.
fn every_row() -> usize {
    let total: usize = files()
        .iter()
        .map(|(file, _)| rows(&foundation_at(file)).len())
        .sum();
    assert!(total > 150, "only {total} rows, so a total proves nothing");
    total
}

fn files() -> Vec<(String, bool)> {
    let mut all: Vec<(String, bool)> = vec![
        ("schema.4x".to_string(), true),
        ("engine.4x".to_string(), true),
        ("rules.4x".to_string(), true),
        ("script.4x".to_string(), false),
        ("setup.4x".to_string(), false),
    ];
    let mut tests: Vec<String> =
        std::fs::read_dir(mine().join("data").join("foundation").join("tests"))
            .expect("data/foundation/tests")
            .filter_map(|it| it.ok())
            .filter_map(|it| it.file_name().to_str().map(str::to_string))
            .filter(|name| name.ends_with(".4x"))
            .collect();
    tests.sort();
    all.extend(
        tests
            .into_iter()
            .map(|name| (format!("tests/{name}"), false)),
    );
    all
}

fn store(of_game: bool, from: &str) -> Vec<Row> {
    // **A merged test file spans two stores**, so which one a row belongs to is a fact about where
    // it sits: a section's rows are the game's, the rest of that file is the script's.
    // **Deduplicated**, because `then` repeats `given` and two rows named `scout` would take away
    // every thing's name.
    let mut all: Vec<Row> = Vec::new();
    for (file, game) in files() {
        let these = of(from, &file, game);
        let mine = friendly::states_a_world(&these);
        for (row, is_game) in these.into_iter().zip(mine) {
            if (game || is_game) == of_game && !all.contains(&row) {
                all.push(row);
            }
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
fn of(from: &str, file: &str, _of_game: bool) -> Vec<Row> {
    if from == "foundation" {
        // **`P-563` split this directory across two columns**, so where a foundation file lives is
        // `common::foundation_at`'s answer rather than a path spelled here. `schema.4x` and
        // `rules.4x` are `spec/data/`'s; the other three and every test are still this crate's.
        return rows(&foundation_at(file));
    }
    let at = format!("data/{from}/{file}");
    // **A test's friendly side left this prototype on 2026-09-21** - `P-532` put it in
    // `spec/tests/`, where it is the specification rather than a rendering of one. **The shared
    // files did not**, so `data/friendly/` still holds five of them and the path a test is found
    // at is now the one thing this function has to know about the split.
    let at = match at.strip_prefix("data/friendly/tests/") {
        Some(name) => format!("../../spec/tests/{name}"),
        None => at,
    };
    // **Always the game's schema.** Only a game row carries `-> n`, so a script row passes through
    // untouched and a section row inside a script file is still folded correctly.
    let schema = Schema::of(&rows("data/friendly/schema.4x")).expect("a schema");
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
    let of_game = Names::of(&store(true, "friendly"));
    let of_script = Names::of(&store(false, "friendly"));
    for (file, game) in files() {
        let friendly = of("friendly", &file, game);
        let mine = friendly::in_a_section(&friendly);
        let foundation = rows(&foundation_at(&file));
        assert_eq!(
            friendly.len(),
            foundation.len(),
            "{file}: {} friendly rows against {} foundation rows",
            friendly.len(),
            foundation.len()
        );
        for (at, row) in friendly.iter().enumerate() {
            let names = if game || mine[at] {
                &of_game
            } else {
                &of_script
            };
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
        checked,
        every_row(),
        "every row of the friendly source was converted"
    );
}

/// **And rendering the foundation gives the friendly source back**, so neither directory can drift
/// from the other without this failing.
#[test]
fn the_friendly_source_is_what_the_foundation_renders_to() {
    let mut checked = 0;
    let of_game = Names::of(&store(true, "foundation"));
    let of_script = Names::of(&store(false, "foundation"));
    for (file, game) in files() {
        let friendly = of("friendly", &file, game);
        let foundation = rows(&foundation_at(&file));
        let mine = friendly::in_a_section(&foundation);
        for (at, row) in foundation.iter().enumerate() {
            let names = if game || mine[at] {
                &of_game
            } else {
                &of_script
            };
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
    assert_eq!(
        checked,
        every_row(),
        "every row of the foundation was rendered"
    );
}

/// **Every shared file exists in both notations**, because nothing is omitted from either.
///
/// # It asks about files rather than about directories, and `P-563` is why
///
/// **This compared two directory listings and that stopped being the same question.** The
/// foundation's `schema.4x` and `rules.4x` are `spec/data/`'s now, so `data/foundation/` holds
/// three files and `data/friendly/` holds five, and a listing comparison says they disagree when
/// what it means to ask is whether either notation is missing anything.
///
/// **So it asks of each file in turn where that file lives.** The population is `files()`, which
/// reads the tests rather than listing them, and the count is asserted - a check that resolved
/// every name to nothing would otherwise pass over an empty loop.
#[test]
fn neither_notation_omits_anything() {
    let shared: Vec<String> = files()
        .iter()
        .map(|(f, _)| f.to_string())
        .filter(|f| !f.starts_with("tests/"))
        .collect();
    let mut looked = 0;
    for file in &shared {
        for at in [foundation_at(file), format!("data/friendly/{file}")] {
            let path = mine().join(&at);
            assert!(
                path.is_file(),
                "`{at}` is named by `files()` and is not there"
            );
            looked += 1;
        }
    }
    assert_eq!(
        looked,
        shared.len() * 2,
        "both notations, every shared file"
    );
    assert_eq!(looked, 10, "five shared files in two notations");
}

/// **A name the foundation has nowhere to keep is refused, not dropped.**
///
/// `territory` declares no `name`, so `{territory id:1 name:home}` cannot be converted. **Silently
/// dropping it would lose an author's work in the format they author in**, which is the one way a
/// conversion that is supposed to be reliable could fail quietly.
#[test]
fn a_name_the_foundation_cannot_keep_is_refused() {
    let names = Names::of(&store(true, "friendly"));
    let row = game_model::notation::read("{territory id:1 name:home}").expect("a row");

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
    let row = game_model::notation::read("{territory id:1 name:territory-1}").expect("a row");
    assert_eq!(
        write(&names.foundation(&row[0]).expect("the generated name")),
        "{territory id:1}"
    );
}
