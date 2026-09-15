//! Moving, and the two checks that say the engine is still what it claims to be.
//!
//! **Three territories, two adjacencies, one vehicle.** Moving `scout` from 1 to 2 succeeds;
//! moving it to 3 fails because 3 is not adjacent, and moving it to 9 fails because 9 is not a
//! place. Every mechanic the main tree has - resources, turns, capacity, combat - is absent
//! because none of these needs one.

use std::collections::BTreeSet;
use std::path::PathBuf;

use thin_engine::engine::{Refused, run};
use thin_engine::notation::{Row, read, write};
use thin_engine::store::Store;

/// **The only directory anything here reads**, which is what `S-136` means by isolated.
fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rows(at: &str) -> Vec<Row> {
    let path = mine().join(at);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|why| panic!("{at}: {why}"));
    read(&text).unwrap_or_else(|why| panic!("{at}: {why}"))
}

fn world() -> Store {
    Store::of(rows("data/world.4x"))
}

fn rules() -> Vec<Row> {
    rows("data/rules.4x")
}

fn command(text: &str) -> Row {
    read(text).expect(text).pop().expect(text)
}

/// The scout is at 1, and 1 is adjacent to 2.
#[test]
fn the_scout_moves_to_a_place_that_is_adjacent() {
    let after = run(&world(), &rules(), &command("{move it:scout from:1 to:2}"))
        .expect("1 is adjacent to 2 and the scout is at 1");

    let at: Vec<String> = after
        .rows()
        .iter()
        .filter(|row| row.relation == "at")
        .map(write)
        .collect();
    assert_eq!(
        at,
        vec!["{at place:2 thing:scout}".to_string()],
        "the scout is at 2, and is nowhere else"
    );
}

/// The scout is at 1, and 1 is adjacent to 2 and to nothing else.
#[test]
fn the_scout_does_not_move_to_a_place_that_is_not_adjacent() {
    let why = run(&world(), &rules(), &command("{move it:scout from:1 to:3}"))
        .expect_err("1 is not adjacent to 3");

    assert_eq!(
        why,
        Refused::NotSo {
            rule: "move".to_string(),
            wanted: "{adjacent from:1 to:3}".to_string()
        },
        "the refusal names the row the world does not have"
    );
}

/// **The isolation is a property of the code and is checked rather than promised** - `S-136`.
///
/// The crate reads no file at all: `std::fs` appears in `tests/`, where the path is this
/// directory's, and nowhere in `src/`. So *no file read outside its own directory* holds by
/// construction, and this is what says the construction is still that way.
///
/// **It reads code and not comments, and it was written the other way first.** Naming `std::fs`
/// in a doc comment to say the engine does not use it failed this check - *quoting a thing and
/// doing it are the same bytes*, which `CLAUDE.md` names after four instances in one evening.
/// The carrier is the same one `cited()` in `tools/outbox` uses: drop what is being shown, then
/// read the rest.
#[test]
fn nothing_in_src_reads_a_file_or_depends_on_another_crate() {
    let src = mine().join("src");
    let files: Vec<PathBuf> = std::fs::read_dir(&src)
        .expect("src")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|it| it == "rs").unwrap_or(false))
        .collect();
    assert_eq!(
        files.len(),
        4,
        "four modules, and each is checked: {files:?}"
    );
    let mut read = 0;

    for file in &files {
        let text = std::fs::read_to_string(file).expect("a module");
        let name = file.file_name().and_then(|it| it.to_str()).unwrap_or("?");
        let code: Vec<&str> = text
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect();
        // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. If
        // the filter above ever dropped everything, every assertion below would pass over an
        // empty population and say so in exactly the same words. **Per file rather than in
        // total**, so that one module going empty is not hidden by the other three; the floor is
        // low because `lib.rs` is four lines of `pub mod` and nothing else.
        assert!(
            code.len() >= 3,
            "`src/{name}` has {} lines of code, which is too few for this to be a check",
            code.len()
        );
        read += code.len();
        let code = code.join("\n");
        for forbidden in ["std::fs", "include_str!", "include_bytes!", "env!"] {
            assert!(
                !code.contains(forbidden),
                "`src/{name}` says `{forbidden}` in code, and the engine reads no file"
            );
        }
    }

    assert!(
        read >= 200,
        "the four modules came to {read} lines of code, and the crate is larger than that - so          the filter is eating something it should not"
    );

    // **A path dependency would not show up above**, so the manifest is checked too. The
    // `[dependencies]` table is empty and stays empty.
    let manifest = std::fs::read_to_string(mine().join("Cargo.toml")).expect("the manifest");
    let after = manifest
        .split_once("[dependencies]")
        .expect("a dependencies table, even an empty one")
        .1;
    assert_eq!(
        after.trim(),
        "",
        "the prototype depends on nothing, and this says `{after}`"
    );
}

/// **The engine names no noun the game has**, which is the claim the experiment rests on.
///
/// **The word list is read out of `data/` rather than written here**, because a hand list checks
/// what somebody remembered - and `CLAUDE.md` has the case: a count summed over the hand list it
/// was meant to check. So this takes every relation the data names, sets aside the four the
/// engine owns, and asserts the rest appear in no line of `src/` that runs.
///
/// ## What it cannot see, said rather than left to be discovered
///
/// **Relation names only.** A key like `from` or `to` is also `String::from` and `to_string`, so
/// including keys would fire on Rust rather than on game vocabulary. The nouns are the claim;
/// the keys are not.
///
/// **`move` is a Rust keyword**, so a `move` closure in `src/` would fail this. There is none.
/// **A false fire is the correct error here** - it costs a rename and catches the real thing.
#[test]
fn no_relation_the_data_names_appears_in_code_that_runs() {
    // `rule`, `needs`, `drops` and `adds` are how a rule is *stated*; every other relation is
    // what the game *is*. That is the whole of the line this test draws.
    let engines = ["rule", "needs", "drops", "adds"];

    let mut nouns: BTreeSet<String> = BTreeSet::new();
    for file in ["data/world.4x", "data/rules.4x"] {
        for row in rows(file) {
            nouns.insert(row.relation.clone());
            // A clause names the relation it is about, and `{rule name:move}` names the rule.
            for key in ["relation", "name"] {
                if let Some(value) = row.value(key) {
                    nouns.insert(value.to_string());
                }
            }
        }
    }
    for engine in engines {
        assert!(
            nouns.remove(engine),
            "`{engine}` is not a relation the data names"
        );
    }
    assert_eq!(
        nouns.len(),
        4,
        "four nouns the game has - at, adjacent, territory, move; this found {nouns:?}"
    );

    let mut looked = 0;
    for file in std::fs::read_dir(mine().join("src")).expect("src") {
        let file = file.expect("a module").path();
        let name = file
            .file_name()
            .and_then(|it| it.to_str())
            .unwrap_or("?")
            .to_string();
        let text = std::fs::read_to_string(&file).expect("a module");
        // **Comments and tests are where the game is allowed to be named**, so both are dropped
        // before looking - the same carrier as the check above, for the same reason.
        let runs = text.split("#[cfg(test)]").next().unwrap_or("").to_string();
        let runs: String = runs
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<&str>>()
            .join(" ");
        for noun in &nouns {
            for word in runs.split(|it: char| !it.is_alphanumeric() && it != '_') {
                assert!(
                    word != noun,
                    "`src/{name}` says `{noun}` in a line that runs, and the engine knows no                      noun the game has"
                );
            }
            looked += 1;
        }
    }
    assert_eq!(looked, 16, "four nouns over four modules");
}

/// **A place that is not a place, and a place that is not next to you, are different refusals.**
/// That is the whole of this concept, so the test asserts the difference and not only the words:
/// before `{needs rule:move relation:territory id:$to}` was a row, both of these named
/// `{adjacent from:1 to:...}` and nothing could tell them apart.
///
/// **The engine did not change to take this.** `territory` was already stated in `data/world.4x`
/// and read by nothing; one `needs` row is what began reading it.
#[test]
fn a_place_that_does_not_exist_and_a_place_that_is_not_adjacent_refuse_differently() {
    let no_such_place = run(&world(), &rules(), &command("{move it:scout from:1 to:9}"))
        .expect_err("there is no territory 9");
    assert_eq!(
        no_such_place,
        Refused::NotSo {
            rule: "move".to_string(),
            wanted: "{territory id:9}".to_string()
        },
        "the refusal names the place that is not one"
    );

    let not_adjacent = run(&world(), &rules(), &command("{move it:scout from:1 to:3}"))
        .expect_err("1 is not adjacent to 3");
    assert_eq!(
        not_adjacent,
        Refused::NotSo {
            rule: "move".to_string(),
            wanted: "{adjacent from:1 to:3}".to_string()
        },
        "territory 3 exists, so the refusal is about the adjacency and not about the place"
    );

    assert_ne!(
        no_such_place, not_adjacent,
        "the two refusals are the concept; if they are equal the engine cannot tell them apart"
    );
}
