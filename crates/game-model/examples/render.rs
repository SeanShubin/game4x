//! Write `data/foundation/` from the friendly source, which is what `tests/directories.rs` asserts.
//!
//! **The two forms hold the same rows in two spellings, and one is generated from the other.**
//! Sean, 2026-09-15: *Lets make friendly the source and not omit anything. This presumes we can
//! reliably convert between friendly and foundation.* So the friendly side is authored and this
//! side is produced, and `tests/directories.rs` is what says they agree.
//!
//! # It ran the other way until `P-576`, and that was the wrong way
//!
//! **This wrote `data/friendly/` from `data/foundation/`** - the source from the rendering. It was
//! written when the foundation side was kept by hand and the friendly side was catching up, and it
//! outlived that: nothing failed, because `tests/directories.rs` holds the two equal row for row
//! and a check that they agree is not a check that the right one is the source.
//!
//! **What made it visible is a column boundary.** `P-563` moved `rules.4x` and `schema.4x` into
//! `spec/data/` and named the *converted* form, so Sean's column got a rendering and the rules he
//! authors stayed in this one. `P-576` corrects the direction; inverting this program is what makes
//! the direction something the build does rather than something a document claims.
//!
//! # Where each form of each file lives
//!
//! **One place knows, and it is [`friendly_at`] and [`foundation_at`].** `tests/directories.rs`
//! borrows this module rather than spelling any of it a second time - the way `tests/scenario.rs`
//! borrows `examples/scenario.rs` - so the thing that converts and the thing that checks the
//! conversion cannot disagree about what converts to what.
//!
//! # It writes the five shared files and no test
//!
//! **A test's foundation side is generated from `reviewed/` and not from here.** `spec/README.md`
//! rule 3: *the rendering is generated from `reviewed/` and never from `spec/tests/`, so that what
//! the engine runs is derived from what has been read rather than compared with it.*
//! `examples/foundation.rs` is that program. Generating a test's foundation side here would read
//! what was **typed** where the rule asks for what was **read**, which is the one substitution the
//! whole arrangement exists to forbid.
//!
//! `cargo run --example render`

use std::path::PathBuf;

use friendly_notation as friendly;

use friendly::Names;
use game_model::notation::{Row, read, write};
use game_model::schema::Schema;

/// Every file the two forms share, then one entry per test.
///
/// **Read rather than listed.** Sean, 2026-09-15: *I intend to have one test per file*, so a list
/// here would be a second place to remember - and `data/foundation/tests/` holding only tests is
/// what makes reading it safe. The flag is whether the whole file is the game's store.
pub fn files() -> Vec<(String, bool)> {
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

pub fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Where the friendly source of a file is, relative to this crate.
///
/// **Two of the five are Sean's and three are this lane's, and that is the whole of the split.**
/// `P-576`: the game's rules and kinds live in `spec/data/` and the engine's primitives stay in
/// `crates/` - so `schema.4x` and `rules.4x` are the specification's, and `engine.4x`, `script.4x`
/// and `setup.4x` name no game noun and stay here. A test's friendly side is `spec/tests/`'s, which
/// `P-532` settled.
pub fn friendly_at(file: &str) -> String {
    match file {
        "schema.4x" | "rules.4x" => format!("../../spec/data/{file}"),
        _ => match file.strip_prefix("tests/") {
            Some(name) => format!("../../spec/tests/{name}"),
            None => format!("data/friendly/{file}"),
        },
    }
}

/// Where the generated foundation form of a file is, relative to this crate.
///
/// **All of it is here, because all of it is generated**, and `CLAUDE.md` gives a generated file no
/// owner. `game_model::foundation::PATHS` names the three the shipped binary carries and has to
/// agree with this; `tests/directories.rs` is what asserts that it does.
pub fn foundation_at(file: &str) -> String {
    format!("data/foundation/{file}")
}

pub fn rows(at: &str) -> Vec<Row> {
    let text = std::fs::read_to_string(mine().join(at)).unwrap_or_else(|why| panic!("{at}: {why}"));
    read(&text).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// One friendly file's rows, with any `-> n` folded back into the column that holds it.
///
/// **The schema is read first and from the friendly side**, because the fold has to know which
/// column a quantity belongs in - and a schema file carries no arrow itself, so reading it needs
/// nothing that is not already there.
pub fn friendly_rows(file: &str) -> Vec<Row> {
    let at = friendly_at(file);
    let schema = Schema::of(&rows(&friendly_at("schema.4x"))).expect("a schema");
    let text =
        std::fs::read_to_string(mine().join(&at)).unwrap_or_else(|why| panic!("{at}: {why}"));
    friendly::fold(&text, &schema).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// Every friendly row of one of the two stores, deduplicated.
///
/// **A merged test file spans two stores**, so the store a row belongs to is a fact about where it
/// sits: a section's rows are the game's, everything else in that file is the script's.
/// **Deduplicated**, because a `then` section repeats its `given` one and two rows named `scout`
/// would make `thing` non-nameable and take every thing's name away with it.
pub fn store(of_game: bool) -> Vec<Row> {
    let mut seen: Vec<Row> = Vec::new();
    for (file, game) in files() {
        let these = friendly_rows(&file);
        let section = friendly::states_a_world(&these);
        for (row, is_game) in these.into_iter().zip(section) {
            if (game || is_game) == of_game && !seen.contains(&row) {
                seen.push(row);
            }
        }
    }
    seen
}

/// What one store needs to be converted: the names ids come from, and the schema that orders the
/// columns of what comes out.
///
/// # Two schemas and not one, which `examples/foundation.rs` found the expensive way
///
/// **`Schema::write` puts a row's values in the order its relation declares them, and a relation it
/// cannot find falls back to the notation's alphabetical order without saying so.** So the writing
/// schema is built over every row of the store rather than over `schema.4x` alone - `schema.4x`
/// declares 49 relations and `move` is not among them, and `{move what:28 from:1 to:2}` came out
/// `{move from:1 to:2 what:28}`: the same row, a different file, nothing complaining.
///
/// **And the script's is a second schema rather than more of the first.** `script.4x` declares
/// `store`, `test` and `load` over ids `schema.4x` already uses for other things, which is why the
/// two stores exist at all - and `Schema::of` refuses the merge, which is what says they are two.
pub fn table(of_game: bool) -> (Names, Schema) {
    let rows = store(of_game);
    let schema = Schema::of(&rows).expect("a schema over the store's own rows");
    (Names::of(&rows), schema)
}

/// The foundation form of every shared file, as text, paired with where it belongs.
///
/// **Produced here and used twice**: `main` writes it, and `tests/directories.rs` compares it with
/// what is committed. **A generator and a check that the generated file is current are the same
/// derivation**, so they are one function - which is what stops the check being a second opinion
/// about what conversion means.
pub fn converted() -> Vec<(String, String)> {
    let (of_game, game_schema) = table(true);
    let (of_script, script_schema) = table(false);

    let mut all = Vec::new();
    for (file, game) in files() {
        // **A test's foundation side is not this program's to write** - see the note at the top.
        // Skipped rather than filtered out of `files()`, because the name tables above are built
        // from every file and a test's rows are most of what the names are drawn from.
        if file.starts_with("tests/") {
            continue;
        }
        let from = friendly_at(&file);
        let text = std::fs::read_to_string(mine().join(&from))
            .unwrap_or_else(|why| panic!("{from}: {why}"));
        // **Which name table reads a row is a fact about where the row sits**: a script file's
        // prologue is the script's and its sections are the game's.
        let parsed = friendly_rows(&file);
        let of_the_game = friendly::in_a_section(&parsed);
        let mut at = 0;

        let mut out = String::new();
        for line in text.lines() {
            let bare = line.trim();
            if bare.is_empty() || bare.starts_with('#') {
                // **Comments and blank lines are carried across**, so the generated file keeps the
                // prose that explains it rather than becoming a bare list of rows.
                out.push_str(line);
            } else {
                let (names, writing) = if game || of_the_game[at] {
                    (&of_game, &game_schema)
                } else {
                    (&of_script, &script_schema)
                };
                let row = &parsed[at];
                at += 1;
                let converted = names
                    .foundation(row)
                    .unwrap_or_else(|why| panic!("{from}: `{}`: {why}", write(row)));
                // **`Schema::write` rather than `notation::write`** - see [`table`]. The notation
                // sorts a row's columns and the relation declares an order, and 18 rows of
                // `script.4x` are how this lane found that out a second time.
                out.push_str(&writing.write(&converted));
            }
            out.push('\n');
        }
        assert_eq!(
            at,
            parsed.len(),
            "{from}: {at} of {} rows were converted, so a line was read as prose",
            parsed.len()
        );
        all.push((foundation_at(&file), out));
    }
    assert_eq!(
        all.len(),
        5,
        "five shared files convert, and this converted {}",
        all.len()
    );
    all
}

fn main() {
    let mut changed = 0;
    for (at, text) in converted() {
        // **Only what changed**, so a run that converts nothing new says so rather than touching
        // every file's timestamp.
        let to = mine().join(&at);
        let before = std::fs::read_to_string(&to).unwrap_or_default();
        if before != text {
            std::fs::write(&to, &text).unwrap_or_else(|why| panic!("{}: {why}", to.display()));
            println!("{at}");
            changed += 1;
        }
    }
    println!("{changed} rewritten");
}
