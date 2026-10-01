//! Write `reports/foundation/` from `reviewed/`, which is `R-12` and rule 3.
//!
//! **`spec/README.md`, rule 3**: *a test is stated in the friendly form, and the foundation form
//! is a rendering of it. The rendering is generated from `reviewed/` and never from `spec/tests/`,
//! so that what the engine runs is derived from what has been read rather than compared with it.*
//!
//! Sean, 2026-09-25: *foundation lives in reports*. So this writes a generated file, nobody edits
//! it, and padding it changes nothing.
//!
//! `cargo run --example foundation`
//!
//! # It reads `reviewed/` and never `spec/tests/`, which is the whole of the rule
//!
//! **`render.rs` has carried the plan for this program in its header since 2026-09-21** and named
//! the wrong source: *`spec/tests/*.4x` read, and `data/foundation/tests/*.4x` written from it.*
//! Reading `spec/tests/` generates from **what was typed**; reading `reviewed/` generates from
//! **what was read**, and the difference is the whole reason rule 3 exists. The header is
//! corrected rather than left as a second plan somebody might follow.
//!
//! # An unread test is named and counted, and is never an error
//!
//! **`tests/common/mod.rs`, `every_read_test`**: *a test with no record is left out rather than
//! failed, which is the half of the rule that is easy to get backwards. Drafting a test is not an
//! error; it is a thing that constrains nothing until he has read it - so this returns fewer files
//! and the runner says how many and which.*
//!
//! **So this mirrors that contract rather than inventing one.** `C-141` proposed refusing, which
//! would make drafting a test an error - and drafting is what an assistant does on Sean's
//! direction, so the gate would redden every time a test was written and before he had any chance
//! to read it. That cost lands on him rather than on a lane.
//!
//! **The floor is what stops the quiet version being vacuous.** With `reviewed/` missing or empty,
//! generating nothing and reporting nothing would look exactly like a directory with nothing left
//! to do - *a count over nothing is the same failure with the sign flipped*.

use std::collections::BTreeSet;
use std::path::PathBuf;

use friendly_notation::{Names, fold, in_a_section, states_a_world};
use game_model::engine::Game;
use game_model::notation::{Row, read, write};
use game_model::schema::Schema;

/// **The converter, borrowed rather than copied** - it is the one place that knows where the
/// friendly source of each file lives.
#[path = "render.rs"]
#[allow(dead_code)]
mod render;

/// The fewest records that can be there before a run proves nothing.
///
/// **The same forty `first_test.rs` uses**, and for the same reason: the two would otherwise be
/// two opinions about when a run is vacuous, and the one that is lower is the one that decides.
const FLOOR: usize = 40;

/// Which suite of `spec/tests/` this reads - `spec/tests/` split into `rule/` and `interface/` on
/// 2026-10-01.
///
/// **Six files spell these two paths twenty times between them**, and `report.rs`'s own comment
/// says they are *named once rather than spelled out ten times*. **That was a claim rather than a
/// fact** and the split hit all six - which is why this constant is a stopgap and `C-203` asks for
/// one source.
const SUITE: &str = "rule";

fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The record of what Sean has read, which is what rule 3 makes the source.
fn records_at() -> PathBuf {
    mine().join("..").join("..").join("reviewed").join(SUITE)
}

/// The working copies, read **only** to say which of them are waiting on him.
///
/// **Nothing is generated from here.** This directory is what a test says; `reviewed/` is what he
/// has read, and rule 3 says the rendering follows the second.
fn tests_at() -> PathBuf {
    mine()
        .join("..")
        .join("..")
        .join("spec")
        .join("tests")
        .join(SUITE)
}

/// Where the generated foundation form goes.
/// Where the form the suite runs lives, which this program writes and nothing else does.
fn suite_at() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("foundation")
        .join("tests")
}

fn reports_at() -> PathBuf {
    mine()
        .join("..")
        .join("..")
        .join("reports")
        .join("foundation")
}

/// Every `.4x` file in a directory, by name, sorted.
fn names_in(at: &PathBuf) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(at) else {
        return Vec::new();
    };
    let mut found: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|it| it == "4x").unwrap_or(false))
        .filter_map(|path| {
            path.file_name()
                .and_then(|it| it.to_str())
                .map(str::to_string)
        })
        .collect();
    found.sort();
    found
}

fn rows_of(text: &str, at: &str) -> Vec<Row> {
    read(text).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// The five shared files, which are the world every test is read against.
///
/// **Borrowed from the converter rather than listed here.** A test's rows mean nothing without the
/// schema and the rules they name, and where each of those lives is `render::friendly_at`'s answer -
/// `P-576` put two of the five in `spec/data/`.
///
/// **The flag is which store a whole file's rows belong to**: `schema`, `engine` and `rules`
/// describe the game the engine runs, and `script` and `setup` are the script's. A test file is
/// neither, so its rows are routed one at a time by whether they state a world.
///
/// # It filtered by whether the file was there, and that was a silent narrowing
///
/// **This read `.filter(|(file, _)| mine().join("data/friendly").join(file).exists())`**, which was
/// written while the files were moving. `P-576` moved `schema.4x` and `rules.4x` out of that
/// directory, and the filter would have dropped both **without a word** - generating every test
/// against a world with no schema and no rules. A missing file is loud now.
fn shared() -> Vec<(String, bool)> {
    let all: Vec<(String, bool)> = render::files()
        .into_iter()
        .filter(|(file, _)| !file.starts_with("tests/"))
        .collect();
    assert_eq!(
        all.len(),
        5,
        "five shared files, and this found {}: {all:?}",
        all.len()
    );
    all
}

fn main() {
    let at = render::friendly_at("schema.4x");
    let schema_text =
        std::fs::read_to_string(mine().join(&at)).unwrap_or_else(|why| panic!("{at}: {why}"));
    let schema = Schema::of(&rows_of(&schema_text, &at)).expect("a schema");

    // **Which tests have a record, and which are waiting on him.**
    let records: BTreeSet<String> = names_in(&records_at()).into_iter().collect();
    let drafts = names_in(&tests_at());
    let unread: Vec<String> = drafts
        .iter()
        .filter(|name| !records.contains(*name))
        .cloned()
        .collect();

    // **The floor, before anything is written.** A missing `reviewed/` would otherwise generate
    // nothing, report nothing, and read exactly like a directory with nothing left to do.
    assert!(
        records.len() >= FLOOR,
        "only {} records in reviewed/, and the floor is {FLOOR} - a run over almost nothing \
         would write almost nothing and look finished. Is reviewed/ there?",
        records.len()
    );

    // **Every friendly row there is, so a name can be resolved.** A test names the things the
    // shared files declare, so the two stores are built over both.
    let mut shared_rows: Vec<Row> = Vec::new();
    let mut script_rows: Vec<Row> = Vec::new();
    let mut of_game: Vec<Row> = Vec::new();
    let mut of_script: Vec<Row> = Vec::new();
    let mut add = |these: Vec<Row>, whole_file_is_the_game: bool| {
        let mine = states_a_world(&these);
        for (row, is_game) in these.into_iter().zip(mine) {
            let into = if whole_file_is_the_game || is_game {
                &mut of_game
            } else {
                &mut of_script
            };
            if !into.contains(&row) {
                into.push(row);
            }
        }
    };
    for (file, game) in shared() {
        let at = render::friendly_at(&file);
        let text =
            std::fs::read_to_string(mine().join(&at)).unwrap_or_else(|why| panic!("{at}: {why}"));
        let these = fold(&text, &schema).unwrap_or_else(|why| panic!("{at}: {why}"));
        // **The schema is built from the three files that describe the game**, which is the
        // same flag that says which store their rows go to. `examples/report.rs` uses exactly
        // `schema.4x`, `engine.4x` and `rules.4x` for this and nothing else.
        //
        // **`script.4x` and `setup.4x` are not among them and adding them fails loudly.**
        // `Schema::of` refuses a relation whose columns arrive twice with the same sequence -
        // `BadOrder { relation: "column", seq: ["1", "1", "2", "2", ...] }` - which is right,
        // because a schema holding a column twice does not say what order it wants.
        if game {
            shared_rows.extend(these.iter().cloned());
        } else if file == "script.4x" {
            script_rows.extend(these.iter().cloned());
        }
        add(these, game);
    }
    for name in &records {
        let at = records_at().join(name);
        let text =
            std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        // **A record Sean has written and the data cannot yet read.** `spec/README.md` asks him
        // to read a test before the rule it is about exists, so this is an ordinary state and not
        // a corruption - and the message says which of the two files has to move, because *line
        // 35: `yard` has no quantity* on its own sends a reader to the test he just approved.
        let these = fold(&text, &schema).unwrap_or_else(|why| {
            panic!(
                "reviewed/{name}: {why}\n\
                 This test has been read and `spec/data/` cannot express it yet, so its \
                 foundation form cannot be generated and the suite cannot run it. The test is \
                 not what is wrong - the data has not caught up with it."
            )
        });
        add(these, false);
    }
    let of_game = Names::of(&of_game);
    let of_script = Names::of(&of_script);

    // **A second schema, built over every shared row rather than over `schema.4x` alone.**
    //
    // **Folding uses the first and writing uses this one**, and they are two jobs rather than
    // one. `tests/directories.rs` says *always the game's schema* for the fold, because only a
    // game row carries `-> n`. Writing needs something else: `Schema::write` puts a row's values
    // in the order its relation declares them, and **a relation it cannot find falls back to the
    // notation's alphabetical order** without saying so.
    //
    // **That fallback is silent, which is how it was found.** `schema.4x` declares 49 relations
    // and `move` is not among them, so `{move what:28 from:1 to:2}` came out
    // `{move from:1 to:2 what:28}` - the same row, a different file, and nothing complaining.
    // `examples/report.rs` already builds its schema over `shared` plus the test's own rows;
    // this is that, and it was found by comparing against the committed foundation rather than
    // by reading either.
    // **The engine's own schema and not `Schema::of`'s**, because a command's relation is
    // declared by its rule rather than by `{column}` rows.
    //
    // **`Schema::write` falls back to alphabetical order for a relation it cannot find, and says
    // nothing.** `Schema::of(&shared_rows)` does not know `move`, so `{move what:28 from:1 to:2}`
    // came out `{move from:1 to:2 what:28}` - the same row, a different file. **Measured against
    // the committed foundation**: 38 of the 54 differed, every one of them on a command line, and
    // the rows were right in all 38.
    //
    // `Game::of` derives a command's columns from the rule's `{input}` rows, which is where the
    // declared order actually lives - so this asks the thing that knows.
    //
    // **Converted here rather than read from `data/foundation/`**, so this program does not
    // depend on `examples/render` having run first. A stale generated file would otherwise give
    // a stale writing schema, silently.
    let shared_foundation: Vec<Row> = shared_rows
        .iter()
        .map(|row| {
            of_game
                .foundation(row)
                .unwrap_or_else(|why| panic!("`{}`: {why}", write(row)))
        })
        .collect();
    let whole = Game::of(shared_foundation)
        .expect("a game over every shared row")
        .schema()
        .clone();

    // **And the script's, which is a second schema rather than more of the first.** `script.4x`
    // declares its own relations from id 17 - `store`, `test`, `load` - over ids `schema.4x`
    // already uses for other things. **That is why the two stores exist**: a game row and a
    // script row are read against different declarations, and writing them is the same split.
    //
    // **Merging the two is what `Schema::of` refused**, and the refusal is what said they were
    // two: `BadOrder { relation: "column", seq: ["1", "1", "2", "2", ...] }` is one relation's
    // columns arriving twice, which is exactly what two schemas in one bag look like.
    let of_script_schema = Schema::of(&script_rows).expect("a schema over the script's rows");

    // **Written fresh, so a record deleted here stops having a rendering there.** A generated
    // directory that only ever grows is one that publishes what was withdrawn.
    let out = reports_at();
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).expect("reports/foundation");

    // **Written in place rather than wiped, and a form whose record is gone is removed.**
    //
    // **`reports/` is wiped because it is a rendering; this is not, because writing every file
    // every run would touch 55 timestamps to change two.** But *in place* was read as *only ever
    // added to*, and the directory reached 57 against 55 records: Sean unreviewed the two arcs
    // `51a13065` deleted, and their generated forms stayed.
    //
    // **Removing one is not removing a record.** The record is already gone - by his hand, in
    // the review application - and this directory is generated in full from what is there, which
    // `CLAUDE.md` says is the whole of what a generated file is. **A test the suite runs that
    // nobody has read is exactly what should not survive a run of the generator.**
    //
    // **`every_reading_reaches_the_suite_...` keeps its teeth**, because the case it guards is a
    // file arriving here by some other hand than this one.
    std::fs::create_dir_all(suite_at()).expect("data/foundation/tests");
    let mut dropped: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(suite_at()).expect("data/foundation/tests") {
        let path = entry.expect("an entry").path();
        let Some(name) = path.file_name().and_then(|it| it.to_str()) else {
            continue;
        };
        if name.ends_with(".4x") && !records.iter().any(|it| it == name) {
            let name = name.to_string();
            std::fs::remove_file(&path).unwrap_or_else(|why| panic!("{name}: {why}"));
            dropped.push(name);
        }
    }
    let mut moved: Vec<String> = Vec::new();
    let mut written = 0;
    let mut converted = 0;
    for name in &records {
        let at = records_at().join(name);
        let text =
            std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        let friendly = fold(&text, &schema).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        let sections = in_a_section(&friendly);

        let mut lines = Vec::new();
        let mut to_write: Vec<Row> = Vec::new();
        for (at, row) in friendly.iter().enumerate() {
            let (names, writing) = if sections[at] {
                (&of_game, &whole)
            } else {
                (&of_script, &of_script_schema)
            };
            let foundation = names
                .foundation(row)
                .unwrap_or_else(|why| panic!("reviewed/{name}: `{}`: {why}", write(row)));
            to_write.push(foundation.clone());
            // **`Schema::write` rather than `notation::write`, and the difference is column
            // order.** A `Row` holds its values in a `BTreeMap`, so the notation writes them
            // alphabetically and the schema writes them the way the relation declares them.
            //
            // **The rows were right and every one of the 54 files differed**, which is what
            // comparing against the committed foundation said before anything was believed
            // about this generator. `{place id:1 layer:surface of:1}` against
            // `{place id:1 of:1 layer:surface}` - the same row, and not the same file.
            lines.push(writing.write(&foundation));
            converted += 1;
        }

        let page = format!(
            "# Generated from `reviewed/{name}` by `cargo run --example foundation`. Do not edit.\n\
             #\n\
             # `spec/README.md` rule 3: a test is stated in the friendly form, and the foundation\n\
             # form is a rendering of it - generated from `reviewed/` and never from `spec/tests/`.\n\
             \n{}\n",
            lines.join("\n")
        );
        std::fs::write(out.join(name), page).unwrap_or_else(|why| panic!("writing {name}: {why}"));
        written += 1;

        // **And the copy the suite runs, which nothing wrote until now.**
        //
        // # A check with no generator behind it
        //
        // **`data/foundation/tests/` was hand-carried.** `render.rs` skips tests by name and this
        // program wrote only the published rendering, so the directory had last been touched by
        // `f633864a` - the crate move - and every reference to it in the tree is a *read*.
        // `what_the_engine_runs_is_what_the_record_generates` compares the records against it and
        // **there was nothing to run when it disagreed**, which is why `S-215` could name the gap
        // and not close it.
        //
        // **The comments are carried across**, which is the whole difference from the page above:
        // a test explains itself in its own words and the form the engine reads keeps them, the
        // same way `render::converted` keeps them for the five shared files.
        let mut carried = String::new();
        let mut row_at = 0;
        for line in text.lines() {
            let bare = line.trim();
            if bare.is_empty() || bare.starts_with('#') {
                carried.push_str(line);
            } else {
                carried.push_str(&lines[row_at]);
                row_at += 1;
            }
            carried.push('\n');
        }
        assert_eq!(
            row_at,
            lines.len(),
            "reviewed/{name}: {row_at} of {} rows were placed, so a line was read as prose",
            lines.len()
        );
        // **Compared as rows and not as bytes, which is what the file is for.**
        //
        // **A command's column order is its rule's `{input seq:}`** - `move` is `what, from, to`
        // - and no `Schema` holds that, so `Schema::write` falls back to alphabetical for every
        // command line. **Writing bytes would have reordered 60 lines across 38 approved
        // renderings and changed not one row.** What this directory owes the records is that it
        // *says the same thing*, which is precisely what
        // `what_the_engine_runs_is_what_the_record_generates` asserts about it.
        //
        // **So a file whose rows already match is left alone**, and the cost is that a stale
        // comment in one is never refreshed. That is the right way round: the rows are the
        // specification and the prose is a reader's.
        let to = suite_at().join(name);
        let before = std::fs::read_to_string(&to).unwrap_or_default();
        let same_rows = read(&before).map(|rows| rows == to_write).unwrap_or(false);
        if !same_rows {
            std::fs::write(&to, &carried).unwrap_or_else(|why| panic!("writing {name}: {why}"));
            moved.push(name.clone());
        }
    }

    // **A count over nothing, guarded on the other side too.** Records with no rows between them
    // would write a directory of empty files and report a healthy number of them.
    assert!(
        converted > written,
        "{written} files came to {converted} rows between them, which is too few for any of them \
         to hold a test"
    );

    println!("wrote {written} foundation files to reports/foundation, {converted} rows");
    match (moved.len(), dropped.len()) {
        (0, 0) => println!("data/foundation/tests: all {written} current"),
        (n, 0) => println!("data/foundation/tests: {n} rewritten - read the diff: {moved:?}"),
        (0, g) => {
            println!("data/foundation/tests: {g} removed, their records are gone: {dropped:?}")
        }
        (n, g) => {
            println!("data/foundation/tests: {n} rewritten {moved:?}, {g} removed {dropped:?}")
        }
    }

    // **Named and counted, and not an error** - `every_read_test`'s contract, mirrored.
    if unread.is_empty() {
        println!("every test in spec/tests has a record");
    } else {
        println!(
            "{} of {} tests have not been read and were not generated: {}",
            unread.len(),
            drafts.len(),
            unread.join(", ")
        );
    }
}
