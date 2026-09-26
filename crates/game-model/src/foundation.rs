//! The foundation the engine runs on, carried in the binary.
//!
//! **`releases/rules-become-data.md`, `D-3`**: *the game reads its data from the data files at
//! run time, and deleting a row changes the game.* This is the half of that which ships. Until
//! now the only thing that read `data/` was `tests/common/mod.rs`, so the data was a fixture -
//! read by the suite and by nothing a player could run.
//!
//! # Why `include_str!` and not a path
//!
//! **A browser has no filesystem**, and the game ships to one. `crates/game-front/src/library.rs`
//! already settled this for the scenario's command files and the argument is the same: both
//! builds get the same bytes, so what the acceptance test reads off disk is what a player runs.
//!
//! **It is not a transcription, which is what `D-3` forbids.** `include_str!` carries the file;
//! it does not restate it. Delete a row from `spec/data/rules.4x`, build, and the game fires a
//! different rule - which is the capability's own sentence, and the tests below are where it is
//! asserted rather than claimed.
//!
//! # Why it is here and not in the engine
//!
//! **The engine reads no file** - `tests/isolation.rs` walks the engine's modules and fails on
//! `std::fs`, `include_str!`, `include_bytes!` or `env!`. That rule is what makes
//! `Game::of(rows)` the only way in, so this module is **beside** the engine rather than part of
//! it, and `common::BESIDE` names it so that the walk skips it on purpose rather than by not
//! noticing.
//!
//! `layers.md` already draws this line for the data: *the harness is beside the others rather
//! than under them* - `store`, `test` and `load` are how a scenario is loaded and are not part of
//! the game at any level. **This is that layer written in Rust**, and it holds the same position.

use crate::engine::Game;
use crate::notation::{Row, read};
use crate::schema::Malformed;

/// The foundation's files: a name, where it lives relative to this crate, and its bytes.
///
/// **One list, and it is the only place any of the three paths is written.** `library.rs` writes
/// its names three times - once to embed and once in each of two tests - which is the shape
/// `Q-46` is about: a list that has to be edited in more than one place is a list that will be
/// edited in one. **The tests and `tests/common`'s `LOADED` read this rather than repeating it**,
/// which is what made `P-563` a three-line change here instead of a sweep.
///
/// **Two of the three are not in this crate, and `P-563` is why.** Sean answered `H3` on
/// 2026-09-26: the game's rules and kinds live in `spec/data/` and the engine's primitives stay
/// in `crates/`. So `schema.4x` and `rules.4x` are the specification's and `engine.4x` is this
/// lane's - **which is the column boundary drawn through the foundation**, and the reason this
/// list has a path column at all.
///
/// **The order does not matter to the engine** and is kept anyway. Everything goes into one store
/// and is validated together, so this is the order a reader should meet them in rather than a
/// constraint: the structure, then the words the engine implements, then this game's rules.
pub const FOUNDATION: [(&str, &str, &str); 3] = [
    (
        "schema",
        "../../spec/data/schema.4x",
        include_str!("../../../spec/data/schema.4x"),
    ),
    (
        "engine",
        "data/foundation/engine.4x",
        include_str!("../data/foundation/engine.4x"),
    ),
    (
        "rules",
        "../../spec/data/rules.4x",
        include_str!("../../../spec/data/rules.4x"),
    ),
];

/// Where each foundation file lives, relative to this crate - the paths of [`FOUNDATION`].
///
/// **`tests/common`'s `LOADED` is this**, so a file moving between columns is one edit.
pub const PATHS: [&str; 3] = [FOUNDATION[0].1, FOUNDATION[1].1, FOUNDATION[2].1];

/// Every row of the foundation, read out of the bytes carried above.
///
/// **A malformed file is a failure of this build and not of this call**, since the bytes are
/// fixed at compile time - so it panics naming the file rather than handing back an error to a
/// caller who could do nothing with it.
pub fn rows() -> Vec<Row> {
    let mut all = Vec::new();
    for (_, at, text) in FOUNDATION {
        let rows = read(text).unwrap_or_else(|why| panic!("{at}: {why}"));
        all.extend(rows);
    }
    all
}

/// The game the foundation states, before anything has been run on it.
///
/// **This can still fail, and the failure is the structure refusing the rows** rather than a file
/// being missing. `Game::of` checks every row against the structure the rows themselves declare,
/// and this returns what that check said.
pub fn game() -> Result<Game, Malformed> {
    Game::of(rows())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **What is carried is what is on disk, byte for byte.**
    ///
    /// `include_str!` guarantees it at build time and this says so at test time, which is what
    /// makes every other test in this crate - all of which read `data/` off disk - a test of what
    /// ships. **Borrowed from `game-front/src/library.rs`**, which needs it for the same reason
    /// and stated it first.
    #[test]
    fn what_is_carried_is_what_is_on_disk() {
        let mut compared = 0;
        for (name, at, carried) in FOUNDATION {
            let path = format!("{}/{at}", env!("CARGO_MANIFEST_DIR").replace('\\', "/"));
            let disk = std::fs::read_to_string(&path)
                .unwrap_or_else(|why| panic!("cannot read {path}: {why}"));
            assert_eq!(carried, disk, "`{name}` has drifted from {at}");
            compared += 1;
        }
        // **Both columns are covered, asserted rather than assumed.** `P-563` split the
        // foundation across two lanes' directories, and a list that had quietly lost one of them
        // would still compare three files and still pass.
        assert!(
            FOUNDATION.iter().any(|(_, at, _)| at.contains("spec/data")),
            "no foundation file is the specification's, so `H3` is not what this is reading"
        );
        assert!(
            FOUNDATION
                .iter()
                .any(|(_, at, _)| at.starts_with("data/foundation")),
            "no foundation file is this crate's, so the engine's primitives have gone missing"
        );
        // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. An
        // empty `FOUNDATION` would compare nothing and pass in exactly these words.
        assert_eq!(compared, FOUNDATION.len());
        assert_eq!(compared, 3, "three files are the foundation");
    }

    /// **The foundation the binary carries builds a game**, which is the whole of what shipping it
    /// is for.
    #[test]
    fn the_carried_foundation_is_a_game() {
        let game = game().expect("the foundation states a game");
        // **A floor rather than a figure**, because the number is the ruleset's and moves
        // whenever Sean changes a rule. What this has to catch is the load collapsing to
        // nothing, which is the state every assertion below would pass over in silence.
        assert!(
            game.rows().rows().len() > 500,
            "the foundation came to {} rows, which is not this ruleset",
            game.rows().rows().len()
        );
    }

    /// **Deleting a row changes the game** - `D-3`'s own sentence, asserted rather than argued.
    ///
    /// # It is checked over every row rather than on one
    ///
    /// **`CLAUDE.md`: check the rule over every case, not on one case, and assert how many cases
    /// there were.** A test that drops one chosen row shows the mechanism on that row and stops
    /// showing anything the day that row is edited away - and goes on passing. This drops each
    /// row of the foundation in turn.
    ///
    /// # Two outcomes count as changed, and the counts are asserted separately
    ///
    /// A row whose absence leaves the rows still valid gives a **different** game; a row whose
    /// absence breaks the structure gives **no** game at all. **Both are the capability
    /// holding.** What would refute it is a row that can be deleted with the game coming out
    /// identical, which would mean the row was carried and never read.
    #[test]
    fn deleting_any_row_of_the_foundation_changes_the_game() {
        let whole = game().expect("the foundation states a game");
        let all = rows();
        assert!(
            all.len() > 500,
            "only {} rows, so this checked almost nothing",
            all.len()
        );

        let (mut differed, mut refused) = (0, 0);
        let mut inert = Vec::new();
        for at in 0..all.len() {
            let mut fewer = all.clone();
            let dropped = fewer.remove(at);
            match Game::of(fewer) {
                Ok(lesser) if lesser == whole => inert.push(dropped),
                Ok(_) => differed += 1,
                Err(_) => refused += 1,
            }
        }

        assert!(
            inert.is_empty(),
            "{} rows can be deleted with the game coming out identical, so they are carried and \
             never read: {:?}",
            inert.len(),
            inert.iter().take(5).collect::<Vec<&Row>>()
        );
        // **Both populations, because either being zero would make this a different check.**
        // With nothing refused it would say only that the rows are distinct from each other;
        // with nothing differing it would say only that the structure is strict.
        assert!(
            differed > 0 && refused > 0,
            "{differed} rows left a different game and {refused} left none - a zero either side \
             means this is not the check it says it is"
        );
        assert_eq!(
            differed + refused,
            all.len(),
            "every row was dropped in turn and every drop was accounted for"
        );
        // **Printed rather than written down**, because the three numbers are the ruleset's and
        // move whenever Sean changes a rule. A figure in this comment would be right today and
        // stale without anyone editing it; `--nocapture` is always current.
        println!(
            "{} rows dropped in turn: {differed} left a different game, {refused} left none",
            all.len()
        );
    }
}
