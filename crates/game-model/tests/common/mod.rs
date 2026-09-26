//! Reading `data/` - the only thing the tests share, and the only place a file is read from.
//!
//! **`src/` reads no file at all**, which is what `S-136` means by isolated and what
//! `tests/isolation.rs` checks. So the loaders live here, in `tests/`, where the path is this
//! directory's own.

#![allow(dead_code)]
// **Each test binary compiles this whole module**, so a helper only one file uses is dead code in
// the others. That is how `mod common` works in Rust and not a sign of an unused helper.

use std::path::PathBuf;

use game_model::engine::Game;
use game_model::notation::{Row, read};
use game_model::schema::Malformed;

/// The modules of the model the engine replaces, named so the engine's own checks can skip them.
///
/// **This is a countdown and not a catalogue.** `releases/rules-become-data.md` measures `D-1` by
/// this crate **stopping** holding rules rather than holding fewer, and every check that excepts
/// these asserts the length - so a module deleted from `src/` is deleted here, the number falls,
/// and when it reaches zero the exceptions have nothing left to except and go with it.
///
/// **Excepting them by name is what keeps the engine's properties true rather than weakened.**
/// *The engine reads no file and names no noun the game has* was a statement about a whole crate
/// while the crate was only the engine. It is the same statement about the same code now; what
/// changed is that the crate holds something else too, and naming that something else is more
/// precise than widening the rule to admit it.
pub const BEING_REPLACED: [&str; 8] = [
    "game",
    "identity",
    "rejection",
    "rules",
    "territory",
    "thing",
    "transition",
    "unit",
];

/// **`lib.rs` is excepted for a different reason and it is temporary too.** It declares both
/// module sets, so it says `pub mod territory;` and `pub mod unit;` - two words that are
/// relations the data names. **It is a declaration of the modules above rather than the engine
/// naming a noun**, and it comes back into scope the moment there is nothing left to declare.
pub const SHARED: [&str; 1] = ["lib"];

/// Modules that are **beside** the engine rather than part of it, and are not going anywhere.
///
/// **This is the one exception list that is permanent, and it says so.** `BEING_REPLACED` and
/// `SHARED` both empty out when the migration finishes; this one does not, because what it names
/// is a layer rather than a leftover.
///
/// **`foundation` carries `data/foundation/` in the binary**, which means it says `include_str!`,
/// one of the four words the engine may not say. That is not a weakening of the engine's rule:
/// the rule is what makes `Game::of(rows)` the only way in, and a module that hands the engine
/// rows is the rule being obeyed rather than bent. `layers.md` already places it: *the harness is
/// beside the others rather than under them.*
///
/// **What keeps it honest is that it is short and each entry carries its reason.** An exception
/// list that grows without one is how a rule stops binding the code it names.
pub const BESIDE: [&str; 1] = ["foundation"];

/// Every module of the engine, read from `src/` rather than listed.
///
/// **Read and then excepted, so a module appearing in `src/` that is in neither list is a
/// failure rather than a silent omission.** A hand list of what to check answers *what did
/// somebody remember*; this answers *what is there*, which is the question the checks are asked.
pub fn engine_modules() -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut skipped = 0;
    for entry in std::fs::read_dir(mine().join("src")).expect("src") {
        let path = entry.expect("a module").path();
        if path.extension().map(|it| it != "rs").unwrap_or(true) {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|it| it.to_str())
            .expect("a module name")
            .to_string();
        if BEING_REPLACED.contains(&stem.as_str())
            || SHARED.contains(&stem.as_str())
            || BESIDE.contains(&stem.as_str())
        {
            skipped += 1;
            continue;
        }
        found.push(path);
    }
    assert_eq!(
        skipped,
        BEING_REPLACED.len() + SHARED.len() + BESIDE.len(),
        "every module excepted by name is a module that is there - one that is not means the \
         migration moved and a list did not"
    );
    assert_eq!(
        found.len(),
        ENGINE_MODULES,
        "the engine is {ENGINE_MODULES} modules, and this found {}: {found:?}",
        found.len()
    );
    found
}

/// **Asserted rather than derived**, so that a module vanishing is a failure and not a smaller
/// population every check downstream then passes over.
pub const ENGINE_MODULES: usize = 7;

/// **The only directory anything here reads**, which is what `S-136` means by isolated.
pub fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn rows(at: &str) -> Vec<Row> {
    let path = mine().join(at);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|why| panic!("{at}: {why}"));
    read(&text).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// The files loaded into the game, in the order they are loaded.
///
/// **Read from `foundation::PATHS` rather than written here**, which is `P-563`: two of the three
/// are `spec/data/`'s now and one is this crate's, and a path written in seven places is a path
/// that will be edited in six. **What that list is checked against is still `script.4x`**, by
/// `the_helper_loads_what_the_script_loads` - which caught the two disagreeing about the order the
/// first time it ran. **The order does not matter to the engine**, since everything goes into one
/// store and is validated together; the lists agreeing is what matters, and asserting the order is
/// the cheapest way to notice that they do not.
pub const LOADED: [&str; 3] = game_model::foundation::PATHS;

/// Where a foundation file lives, given its bare name.
///
/// **`P-563` split `data/foundation/` across two columns and this is the only place that knows
/// it.** `schema.4x` and `rules.4x` are `spec/data/`'s now; `engine.4x`, `script.4x`, `setup.4x`
/// and every test are still this crate's. A reader that spelled `data/foundation/{file}` was
/// right for as long as they were all in one directory, and there were seven of them.
///
/// **Resolved from `foundation::PATHS` rather than from a second list**, so the answer here and
/// the answer the shipped binary uses cannot differ. A name that is not one of the three keeps
/// the old spelling, which is what makes this a lookup rather than a table.
pub fn foundation_at(file: &str) -> String {
    for at in LOADED {
        let named = at.rsplit('/').next().unwrap_or(at);
        if named == file {
            return at.to_string();
        }
    }
    format!("data/foundation/{file}")
}

/// Every test file, read rather than listed - one test per file, and nothing else in `tests/`.
pub fn every_test() -> Vec<String> {
    let mut found: Vec<String> =
        std::fs::read_dir(mine().join("data").join("foundation").join("tests"))
            .expect("data/foundation/tests")
            .filter_map(|it| it.ok())
            .filter_map(|it| it.file_name().to_str().map(str::to_string))
            .filter(|name| name.ends_with(".4x"))
            .map(|name| format!("data/foundation/tests/{name}"))
            .collect();
    found.sort();
    assert!(!found.is_empty(), "no tests, so passing means nothing");
    found
}

/// Every test Sean has read, which is the set the suite runs.
///
/// **`CLAUDE.md`**: *the suite runs the copies in `reviewed/`, so a test nobody has read
/// constrains nothing and a test he has read is red until the code obeys it.* `S-149`, 2026-09-21:
/// it ran every file in the directory instead, so his reading changed what the gate did by
/// nothing at all.
///
/// **What executes is still the foundation notation**, because that is what the engine reads.
/// **What makes those the read bytes is `tests/reviewed.rs`**, which fails when a test and its
/// record differ, on top of `tests/directories.rs` holding the two notations together. So the
/// chain is: the engine runs the foundation, the foundation is the friendly source, and the
/// friendly source is what he approved.
///
/// **A test with no record is left out rather than failed**, which is the half of the rule that is
/// easy to get backwards. Drafting a test is not an error; it is a thing that constrains nothing
/// until he has read it - so this returns fewer files and the runner says how many and which.
///
/// **Every other check still walks every file.** Whether a test is spaced the way Sean spaces
/// them, and whether its two notations agree, are true of a draft as much as of an approved test -
/// only *does the engine have to satisfy it* waits on a reading.
pub fn every_read_test() -> (Vec<String>, Vec<String>) {
    let record = mine().join("..").join("..").join("reviewed");
    let (mut read, mut unread) = (Vec::new(), Vec::new());
    for file in every_test() {
        let name = file.rsplit('/').next().unwrap_or(&file).to_string();
        match record.join(&name).is_file() {
            true => read.push(file),
            false => unread.push(name),
        }
    }
    (read, unread)
}

pub fn game_rows() -> Vec<Row> {
    let mut all = Vec::new();
    for file in LOADED {
        all.extend(rows(file));
    }
    all.extend(section("given"));
    all
}

/// The rows of one section of `data/foundation/test.4x`.
///
/// **The state lives in the test now.** `given` and `then` are two worlds for one schema, so a
/// caller asks for the one it means - putting both in a store would give two `{territory id:1}`
/// rows and a key that names neither.
pub fn section(want: &str) -> Vec<Row> {
    let mut inside = false;
    let mut out = Vec::new();
    for row in rows("data/foundation/tests/the-scout-moves-to-an-adjacent-place.4x") {
        if matches!(row.relation.as_str(), "given" | "when" | "then") {
            inside = row.relation == want;
            continue;
        }
        if inside {
            out.push(row);
        }
    }
    out
}

/// The game as `data/` states it, before anything has run.
pub fn before() -> Game {
    Game::of(game_rows()).unwrap_or_else(|why| panic!("{why}"))
}

/// The same, with some rows added, for checking what the structure refuses.
pub fn with(extra: &str) -> Result<Game, Malformed> {
    let mut all = game_rows();
    all.extend(read(extra).expect(extra));
    Game::of(all)
}
