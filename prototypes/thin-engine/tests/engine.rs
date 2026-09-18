//! Where the data stops describing and starts delegating.
//!
//! **This is the boundary the prototype is for.** A `{clause ...}` row is data the engine moves
//! around without knowing what it means - a second rule is rows and no code. A word in
//! `data/engine.4x` is the other thing: the engine holds the string and branches on it, so a word
//! that is not there cannot be added without writing Rust.
//!
//! **The list is checked both ways.** A constant in `src/` with no row fails; a row with no
//! constant fails. So the boundary cannot drift, which is the only reason a list like this is
//! worth having rather than a paragraph saying roughly the same thing.

use std::collections::BTreeSet;

mod common;
use common::{mine, rows};

/// Every string constant the engine compares a data value against.
///
/// **Read out of `src/` rather than written here.** A hand list checks what somebody remembered,
/// and `CLAUDE.md` has the case of a count summed over the hand list it was meant to check.
///
/// **Only the code that runs**, so a constant declared inside `#[cfg(test)]` is not vocabulary -
/// `schema.rs` has one, and it is a fixture rather than a word the engine knows.
fn constants() -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut files = 0;
    for file in std::fs::read_dir(mine().join("src")).expect("src") {
        let file = file.expect("a module").path();
        if file.extension().map(|it| it != "rs").unwrap_or(true) {
            continue;
        }
        files += 1;
        let text = std::fs::read_to_string(&file).expect("a module");
        for line in text.split("#[cfg(test)]").next().unwrap_or("").lines() {
            let line = line.trim();
            let Some(rest) = line.strip_prefix("const ") else {
                continue;
            };
            let Some((_, value)) = rest.split_once(": &str = \"") else {
                continue;
            };
            let Some(value) = value.strip_suffix("\";") else {
                continue;
            };
            if !value.is_empty() {
                found.insert(value.to_string());
            }
        }
    }
    assert_eq!(files, 8, "eight modules were read");
    found
}

/// Every word `data/engine.4x` says the engine implements.
fn declared() -> BTreeSet<String> {
    rows("data/foundation/engine.4x")
        .iter()
        .filter(|row| row.relation == "primitive")
        .filter_map(|row| row.value("word").map(str::to_string))
        .collect()
}

/// **The data's list of what the engine implements is the engine's list, both ways.**
///
/// This is the check that makes the boundary readable off a file instead of inferred from the
/// code. **Both directions matter and for different reasons**: a constant with no row means a
/// branch was added and the data does not admit it; a row with no constant means the data claims
/// the engine implements something it does not.
#[test]
fn the_words_the_data_delegates_are_the_words_the_engine_implements() {
    let implemented = constants();
    let claimed = declared();

    assert!(
        implemented.len() >= 25,
        "only {} constants were found in `src/`, which is too few for this to be a check: {implemented:?}",
        implemented.len()
    );

    let undeclared: Vec<&String> = implemented.difference(&claimed).collect();
    assert!(
        undeclared.is_empty(),
        "`src/` branches on these and `data/engine.4x` does not list them: {undeclared:?}"
    );

    let unimplemented: Vec<&String> = claimed.difference(&implemented).collect();
    assert!(
        unimplemented.is_empty(),
        "`data/engine.4x` lists these and `src/` does not branch on them: {unimplemented:?}"
    );

    assert_eq!(
        implemented.len(),
        46,
        "46 words, and the count is written down so that adding one is a decision somebody makes rather than a line somebody adds"
    );
}

/// **A relation the game declares is not automatically a word the engine implements**, and the
/// difference between the two lists is the thing worth looking at.
///
/// `territory`, `thing`, `adjacency`, `residency` and `move` are declared in `data/` and the
/// engine has never heard of them - that is `tests/isolation.rs`. `clause`, `binding` and `input`
/// are declared *and* implemented, because the engine walks them. **The game's own nouns being
/// absent from `engine.4x` is what says the engine is thin.**
#[test]
fn no_noun_the_game_has_is_a_word_the_engine_implements() {
    let claimed = declared();
    let mut checked = 0;
    for noun in ["territory", "thing", "adjacency", "residency", "move"] {
        assert!(
            !claimed.contains(noun),
            "`{noun}` is a noun the game has, and `data/engine.4x` says the engine implements it"
        );
        checked += 1;
    }
    assert_eq!(checked, 5, "five nouns the game has");

    // The control: the words that *are* implemented really are in the list, so the five above
    // are absent for their own reason rather than because the list is empty.
    for word in ["clause", "binding", "require", "load"] {
        assert!(claimed.contains(word), "`{word}` should be implemented");
    }
}
