//! What the engine is, checked rather than promised.
//!
//! **Neither of these is about the game.** They are the two properties the experiment rests on -
//! that the engine reads no file and depends on no crate, and that it names no noun the game has.
//! The other files in `tests/` are the game; this is the instrument.

use std::collections::BTreeSet;
use std::path::PathBuf;

mod common;
use common::{mine, rows};

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
    // **How a rule is stated, as against what the game is** - that is the whole of the line this
    // test draws. `rule`, `needs`, `drops` and `adds` are the four the engine started with;
    // `command` and `turn` are what a rule may be fired `by`, which is also about the statement
    // and not about the world. Every other word the data uses is the game's, and must appear in
    // no line of `src/` that runs.
    let engines = ["rule", "needs", "drops", "adds", "command", "turn"];

    let mut nouns: BTreeSet<String> = BTreeSet::new();
    for file in ["data/world.4x", "data/rules.4x"] {
        for row in rows(file) {
            nouns.insert(row.relation.clone());
            // A clause names the relation it is about, `{rule name:move by:command}` names the
            // rule, and `by` names what fires it. **`by` is read for the same reason the other
            // two are**: a value the engine compares against is vocabulary, and one this test
            // cannot see is one `src/` could name freely.
            for key in ["relation", "name", "by"] {
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
    // **The count is written down and changes when the game does**, which is the point rather
    // than a maintenance cost: a concept that adds vocabulary fails this line and somebody has
    // to look at what it added. It fired on `found`, `settlement` and `vacant` together.
    assert_eq!(
        nouns.len(),
        11,
        "eleven nouns the game has - at, adjacent, territory, move, found, settlement, vacant, fuel, less, grow, food; this found {nouns:?}"
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
    assert_eq!(looked, 44, "eleven nouns over four modules");
}
