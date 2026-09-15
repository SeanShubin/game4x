//! What the engine is, checked rather than promised.
//!
//! **Neither of these is about the game.** They are the two properties the experiment rests on -
//! that the engine reads no file and depends on no crate, and that it names no noun the game has.
//! `tests/first_test.rs` is the game; this is the instrument.

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
        6,
        "six modules, and each is checked: {files:?}"
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
        // total**, so that one module going empty is not hidden by the other four.
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
        read >= 300,
        "the six modules came to {read} lines of code, and the crate is larger than that - so the filter is eating something it should not"
    );

    // **A path dependency would not show up above**, so the manifest is checked too.
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
/// was meant to check.
///
/// ## Where the line falls, now that the structure describes itself
///
/// `data/before.4x` declares `relation`, `column`, `reference`, `role`, `rule`, `input`, `clause`,
/// `binding`, `command` and `argument` as relations, exactly as it declares `territory`. **Those
/// ten are how a thing is written down and the engine may name them**; every other relation, and
/// every rule, is what the game *is* and it may not.
///
/// **Relation and rule names only.** A column like `from` or `to` is also `String::from` and
/// `to_string`, so including column names would fire on Rust rather than on game vocabulary.
///
/// **`move` is a Rust keyword**, so a `move` closure in `src/` would fail this. There is none, and
/// **a false fire is the correct error here** - it costs a rename and catches the real thing. It
/// has already caught one, a local named `found` in a search.
#[test]
fn no_relation_or_rule_the_data_names_appears_in_code_that_runs() {
    let engines = [
        "relation",
        "column",
        "reference",
        "state",
        "role",
        "rule",
        "input",
        "clause",
        "binding",
        "command",
        "argument",
        "primitive",
        "store",
        "test",
        "load",
        "execute",
        "compare",
        "report",
    ];

    let mut nouns: BTreeSet<String> = BTreeSet::new();
    for file in ["data/schema.4x", "data/script.4x", "data/rules.4x"] {
        for row in rows(file) {
            // A relation declares itself by name, and so does a rule. Nothing else is read:
            // a column name is not vocabulary the engine could be accused of knowing.
            if (row.relation == "relation" || row.relation == "rule")
                && let Some(name) = row.value("name")
            {
                nouns.insert(name.to_string());
            }
        }
    }
    for engine in engines {
        assert!(
            nouns.remove(engine),
            "`{engine}` is not a relation the data declares"
        );
    }
    assert_eq!(
        nouns.len(),
        5,
        "five nouns the game has - territory, thing, adjacency, residency, move; this found {nouns:?}"
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
                    "`src/{name}` says `{noun}` in a line that runs, and the engine knows no noun the game has"
                );
            }
            looked += 1;
        }
    }
    assert_eq!(looked, 30, "five nouns over six modules");
}
