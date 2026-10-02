//! The friendly source is what a person writes and `data/foundation/` is what it converts to.
//!
//! **Sean, 2026-09-15**: *Lets make friendly the source and not omit anything. This presumes we
//! can reliably convert between friendly and foundation. Also it is ok that sometimes they happen
//! to be the same thing.*
//!
//! So both forms hold the same rows, nothing is left out of either, and **this is what says they
//! say the same thing**: converting the friendly source has to produce the committed foundation
//! byte for byte, and rendering the foundation has to produce the friendly source back row for row.
//!
//! # Where each form lives, and why this file no longer says
//!
//! **`P-576` put `schema.4x` and `rules.4x` in `spec/data/` as the friendly source**, and left
//! `engine.4x`, `script.4x` and `setup.4x` here because they name no game noun. **The program that
//! converts is the one place that knows that** - this borrows `examples/render.rs` the way
//! `tests/scenario.rs` borrows `examples/scenario.rs`, so the thing that converts and the thing
//! that checks the conversion cannot disagree about what converts to what.
//!
//! **`P-563` got the direction backwards and nothing failed**, which is the reason the borrowing
//! matters. It moved the *converted* form into Sean's column, so he owned a rendering and the rules
//! he authors stayed in this one - and these tests stayed green throughout, because holding two
//! things equal row for row is not the same as holding the right one to be the source.

mod common;
use friendly_notation::{self as friendly, Names};

use game_model::notation::{Row, write};

/// **The converter, borrowed rather than copied.** `files`, `friendly_at`, `foundation_at` and the
/// conversion itself all come from the program that writes the generated files.
#[path = "../examples/render.rs"]
#[allow(dead_code)]
mod render;

use render::{files, foundation_at, friendly_at, friendly_rows, mine, rows};

/// Every row in `data/foundation`, so a count below is derived rather than written down.
///
/// **Adding a test must not mean editing a number.** Sean, 2026-09-15: *I intend to have one test
/// per file*, and a total written by hand is a line every new test would have to move. **The floor
/// is what keeps it honest** - a derived total compared against itself passes over an empty
/// directory, which is `CLAUDE.md`'s count over nothing.
/// Whether a file is one this comparison can hold, which a drifted test is not.
///
/// **`P-611`**: *a test whose rows have changed since I read it is in the first state.* So its
/// verdict is cleared and `spec/tests/` holds an edit nobody has read - and comparing the
/// foundation against that edit fails by definition, for a test the build must not fail over.
///
/// **Sean, 2026-10-02**: *I want to make sure a test I have not reviewed does not fail the build.*
///
/// **What runs is still what he approved**, because the foundation comes from the record. This
/// skips the comparison, not the test.
fn is_compared(file: &str) -> bool {
    let shared = [
        "schema.4x",
        "engine.4x",
        "rules.4x",
        "script.4x",
        "setup.4x",
    ];
    if shared.contains(&file) {
        return true;
    }
    render::state_of(
        std::fs::read_to_string(mine().join("../../reviewed/rule").join(file))
            .ok()
            .as_deref(),
        &std::fs::read_to_string(mine().join("../../spec/tests/rule").join(file))
            .unwrap_or_default(),
    )
    .state
        == render::APPROVED
}

fn every_row() -> usize {
    let total: usize = files()
        .iter()
        .filter(|(file, _)| is_compared(file))
        .map(|(file, _)| rows(&foundation_at(file)).len())
        .sum();
    assert!(total > 150, "only {total} rows, so a total proves nothing");
    total
}

/// One file's rows from whichever form, both already folded where folding applies.
fn of(from: &str, file: &str) -> Vec<Row> {
    match from {
        "foundation" => rows(&foundation_at(file)),
        _ => friendly_rows(file),
    }
}

fn store(of_game: bool, from: &str) -> Vec<Row> {
    // **A merged test file spans two stores**, so which one a row belongs to is a fact about where
    // it sits: a section's rows are the game's, the rest of that file is the script's.
    // **Deduplicated**, because `then` repeats `given` and two rows named `scout` would take away
    // every thing's name.
    let mut all: Vec<Row> = Vec::new();
    for (file, game) in files() {
        let these = of(from, &file);
        let mine = friendly::states_a_world(&these);
        for (row, is_game) in these.into_iter().zip(mine) {
            if (game || is_game) == of_game && !all.contains(&row) {
                all.push(row);
            }
        }
    }
    all
}

/// **What is committed under `data/foundation/` is what the friendly source converts to, byte for
/// byte.**
///
/// # It compared rows, and that is one order short
///
/// **This asserted row for row, and a row is a set of named values.** `Schema::write` puts them in
/// the order the relation declares and `notation::write` sorts them, so `{column id:47 relation:17
/// seq:1 name:id}` and `{column id:47 name:id relation:17 seq:1}` are one row and two files - and
/// a row comparison passes over the difference. **18 rows of `script.4x` are how this lane found
/// that**, and `examples/foundation.rs` had already found it once, in all 54 test files.
///
/// **A generated file that is committed and stale reads exactly like one that is current**, which
/// is the argument `crates/game-console/tests/dumps_are_current.rs` makes for the reports. So this
/// compares the text, and fails with the first line that differs.
#[test]
fn the_foundation_is_what_the_friendly_source_converts_to() {
    let produced = render::converted();
    let mut compared = 0;
    for (at, text) in &produced {
        let committed = std::fs::read_to_string(mine().join(at))
            .unwrap_or_else(|why| panic!("{at}: {why} - run `cargo run --example render`"));
        if committed != *text {
            let differs = committed
                .lines()
                .zip(text.lines())
                .enumerate()
                .find(|(_, (was, now))| was != now);
            match differs {
                Some((line, (was, now))) => panic!(
                    "{at} is stale at line {}: it says\n  {was}\nand the friendly source converts \
                     to\n  {now}\nRun `cargo run --example render`.",
                    line + 1
                ),
                None => panic!(
                    "{at} has {} line(s) and the conversion produces {} - run `cargo run --example \
                     render`",
                    committed.lines().count(),
                    text.lines().count()
                ),
            }
        }
        compared += 1;
    }
    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. Converting
    // no files would compare no files and pass.
    assert_eq!(
        compared, 5,
        "five shared files, and this compared {compared}"
    );
}

/// **And rendering the foundation gives the friendly source back**, so neither form can drift from
/// the other without this failing.
///
/// **This is the direction nothing generates**, and that is what it is for: Sean's premise was that
/// the two forms convert *reliably*, which is a claim about both ways round. A conversion that lost
/// something would still round-trip its own output and fail here.
#[test]
fn the_friendly_source_is_what_the_foundation_renders_to() {
    let mut checked = 0;
    let of_game = Names::of(&store(true, "foundation"));
    let of_script = Names::of(&store(false, "foundation"));
    let mut skipped: Vec<String> = Vec::new();
    for (file, game) in files() {
        if !is_compared(&file) {
            skipped.push(file.clone());
            continue;
        }
        let friendly = of("friendly", &file);
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
                .unwrap_or_else(|why| panic!("{}: {why}", foundation_at(&file)));
            assert_eq!(
                rendered,
                friendly[at],
                "{}: `{}` renders to `{}` and `{}` says `{}`",
                foundation_at(&file),
                write(row),
                names.row(row),
                friendly_at(&file),
                write(&friendly[at])
            );
            checked += 1;
        }
    }
    if !skipped.is_empty() {
        println!(
            "{} test(s) were not compared because their verdict is cleared: {skipped:?}",
            skipped.len()
        );
    }
    assert_eq!(
        checked,
        every_row(),
        "every row of the foundation was rendered"
    );
}

/// **Every shared file exists in both forms**, because nothing is omitted from either.
///
/// # It asks about files rather than about directories
///
/// **This compared two directory listings and that stopped being the same question.** The friendly
/// source is spread across two columns - `spec/data/` holds two files and `data/friendly/` holds
/// three - so a listing comparison says they disagree when what it means to ask is whether either
/// form is missing anything.
///
/// **So it asks of each file in turn where that file lives.** The population is `files()`, which
/// reads the tests rather than listing them, and the count is asserted - a check that resolved
/// every name to nothing would otherwise pass over an empty loop.
#[test]
fn neither_form_omits_anything() {
    let shared: Vec<String> = files()
        .iter()
        .map(|(f, _)| f.to_string())
        .filter(|f| !f.starts_with("tests/"))
        .collect();
    let mut looked = 0;
    for file in &shared {
        for at in [foundation_at(file), friendly_at(file)] {
            let path = mine().join(&at);
            assert!(
                path.is_file(),
                "`{at}` is named by `files()` and is not there"
            );
            looked += 1;
        }
    }
    assert_eq!(looked, shared.len() * 2, "both forms, every shared file");
    assert_eq!(looked, 10, "five shared files in two forms");
}

/// **What the shipped binary carries is the generated form, and it is this one.**
///
/// **`game_model::foundation::PATHS` and `render::foundation_at` are two statements of where a
/// foundation file lives**, and nothing made them agree. `P-563` pointed the first at `spec/data/`
/// and `P-576` made that directory the friendly source, so the binary embedded names the engine
/// cannot resolve - and it compiled, because `include_str!` only embeds text.
#[test]
fn the_binary_carries_what_the_conversion_writes() {
    let carried: Vec<&str> = game_model::foundation::PATHS.to_vec();
    let mut matched = 0;
    for at in &carried {
        let name = at.rsplit('/').next().unwrap_or(at);
        assert_eq!(
            *at,
            foundation_at(name),
            "the binary carries `{at}` and the conversion writes `{}`",
            foundation_at(name)
        );
        assert!(
            render::converted().iter().any(|(to, _)| to == at),
            "`{at}` is carried and `examples/render.rs` does not write it"
        );
        matched += 1;
    }
    assert_eq!(matched, carried.len());
    assert_eq!(matched, 3, "three files are the foundation");
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

/// **A test's references resolve from the shared rows and its own, and from nothing else** -
/// `S-247`.
///
/// # The coupling this refuses
///
/// **`render::store` reads every file of the store, which is every test.** So the name table
/// `report.rs` converted with was built from the union of all of them, and a reference resolved only
/// if some test *anywhere* happened to declare a row with that id.
///
/// **Measured when `S-247` was fixed: four tests declare `{territory id:3}` and none declares
/// `{territory id:4}`.** So `of:territory-3` resolved because four *other* tests have a third
/// territory, and `of:territory-4` stayed a name and was refused with *no `territory` has that key* -
/// three lines under the row that declares it.
///
/// **A test was not self-contained, which is the defect rather than the fourth territory.** Adding a
/// test with four territories would have made that one pass; deleting one with three would have
/// broken four others.
///
/// # Why this is built rather than a test with four territories
///
/// **`spec/tests/` is the specification's and a test there is Sean's to read.** This needs no
/// approval and asserts nothing about the game: it is a world built here, converted here, and
/// thrown away - **the question is whether the converter resolves a name the shared rows have never
/// seen.**
#[test]
fn a_reference_resolves_from_the_shared_rows_and_the_test_being_folded() {
    let shared = render::store(true);
    // **The floor: a fourth territory must be absent from the shared rows**, or this passes because
    // the coupling it refuses is satisfied rather than because the converter works.
    let declared = |id: &str| {
        shared
            .iter()
            .any(|it| it.relation == "territory" && it.value("id") == Some(id))
    };
    assert!(
        !declared("4"),
        "the shared rows already declare a fourth territory, so this no longer tests the coupling"
    );
    assert!(
        declared("1"),
        "the shared rows declare no territory at all, so there is nothing to be self-contained \
         against"
    );

    // A world the shared rows have never seen: four territories, each with a terrain.
    let own =
        friendly_rows_of("{territory id:4 name:territory-4}\n{terrain of:territory-4 is:desert}\n");
    assert_eq!(own.len(), 2);

    let mut whole = shared.clone();
    whole.extend(own.clone());
    let names = Names::of(&whole);

    let terrain = own
        .iter()
        .find(|it| it.relation == "terrain")
        .expect("the terrain row");
    let converted = names
        .foundation(terrain)
        .unwrap_or_else(|why| panic!("{terrain:?}: {why}"));
    assert_eq!(
        converted.value("of"),
        Some("4"),
        "`of:territory-4` did not resolve, so the name table does not hold the rows being folded"
    );

    // **And the same row against the shared table alone fails**, which is what the fix changed. A
    // check that only asserted the first half would pass before the fix and after it.
    let without = Names::of(&shared);
    let stale = without
        .foundation(terrain)
        .unwrap_or_else(|why| panic!("{terrain:?}: {why}"));
    assert_eq!(
        stale.value("of"),
        Some("territory-4"),
        "the shared table resolved a name it has never seen, so this comparison says nothing"
    );
}

/// Fold a snippet of the friendly form, for a test that owns the text.
fn friendly_rows_of(said: &str) -> Vec<Row> {
    let (_, schema) = render::table(true);
    friendly::fold(said, &schema).expect("the snippet folds")
}
