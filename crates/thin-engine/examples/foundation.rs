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
use thin_engine::notation::{Row, read, write};
use thin_engine::schema::Schema;

/// The fewest records that can be there before a run proves nothing.
///
/// **The same forty `first_test.rs` uses**, and for the same reason: the two would otherwise be
/// two opinions about when a run is vacuous, and the one that is lower is the one that decides.
const FLOOR: usize = 40;

fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The record of what Sean has read, which is what rule 3 makes the source.
fn records_at() -> PathBuf {
    mine().join("..").join("..").join("reviewed")
}

/// The working copies, read **only** to say which of them are waiting on him.
///
/// **Nothing is generated from here.** This directory is what a test says; `reviewed/` is what he
/// has read, and rule 3 says the rendering follows the second.
fn tests_at() -> PathBuf {
    mine().join("..").join("..").join("spec").join("tests")
}

/// Where the generated foundation form goes.
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
/// **They are still `data/friendly/`'s**, because `P-532` moved only the tests to `spec/tests/`.
/// A test's rows mean nothing without the schema and the rules they name.
/// **The flag is which store a whole file's rows belong to**, and it is `tests/directories.rs`'s
/// list rather than a second opinion: `schema`, `engine` and `rules` describe the game the engine
/// runs, and `script` and `setup` are the script's. A test file is neither, so its rows are routed
/// one at a time by whether they state a world.
fn shared() -> Vec<(String, bool)> {
    [
        ("schema.4x", true),
        ("engine.4x", true),
        ("rules.4x", true),
        ("script.4x", false),
        ("setup.4x", false),
    ]
    .iter()
    .filter(|(file, _)| mine().join("data/friendly").join(file).exists())
    .map(|(file, game)| ((*file).to_string(), *game))
    .collect()
}

fn main() {
    let schema_text = std::fs::read_to_string(mine().join("data/friendly/schema.4x"))
        .expect("data/friendly/schema.4x");
    let schema = Schema::of(&rows_of(&schema_text, "data/friendly/schema.4x")).expect("a schema");

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
        let at = format!("data/friendly/{file}");
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
        let these = fold(&text, &schema).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
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
    let whole = Schema::of(&shared_rows).expect("a schema over every shared row");

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

    let mut written = 0;
    let mut converted = 0;
    for name in &records {
        let at = records_at().join(name);
        let text =
            std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        let friendly = fold(&text, &schema).unwrap_or_else(|why| panic!("reviewed/{name}: {why}"));
        let sections = in_a_section(&friendly);

        let mut lines = Vec::new();
        for (at, row) in friendly.iter().enumerate() {
            let (names, writing) = if sections[at] {
                (&of_game, &whole)
            } else {
                (&of_script, &of_script_schema)
            };
            let foundation = names
                .foundation(row)
                .unwrap_or_else(|why| panic!("reviewed/{name}: `{}`: {why}", write(row)));
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
    }

    // **A count over nothing, guarded on the other side too.** Records with no rows between them
    // would write a directory of empty files and report a healthy number of them.
    assert!(
        converted > written,
        "{written} files came to {converted} rows between them, which is too few for any of them \
         to hold a test"
    );

    println!("wrote {written} foundation files to reports/foundation, {converted} rows");

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
