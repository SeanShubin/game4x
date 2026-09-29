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
//! it does not restate it. Delete a row from `data/foundation/rules.4x`, build, and the game fires
//! a different rule - which is the capability's own sentence, and the tests below are where it is
//! asserted rather than claimed.
//!
//! **The file it carries is generated from `spec/data/`, so a row deleted there takes one more
//! step.** `cargo run --example render` converts the friendly source Sean authors into the form the
//! engine reads, and `tests/directories.rs` fails whenever the two have come apart - so the extra
//! step cannot be forgotten silently. **Why there is a step at all is under [`FOUNDATION`].**
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
/// **All three are in this crate, and `P-576` is why they came back.** `P-563` moved `schema.4x`
/// and `rules.4x` to `spec/data/` and named the **converted** form, which is the wrong half:
/// `tests/directories.rs` line 1 says *`data/friendly/` is the source and `data/foundation/` is
/// what it converts to*, so Sean's column got a rendering and the rules he authors stayed here.
/// `P-576` corrects it, and `spec/data/` now holds the **friendly source**.
///
/// **So what this carries is generated**, and `CLAUDE.md` gives a generated file no owner. The
/// engine cannot convert: `Names` lives in `friendly-notation`, which depends on this crate and is
/// a dev-dependency here precisely so `tests/isolation.rs` can go on reading an engine that
/// depends on nothing. **Converting at run time would make that cycle real**, so the conversion
/// happens once, in `examples/render.rs`, and its output is what ships.
///
/// **The order does not matter to the engine** and is kept anyway. Everything goes into one store
/// and is validated together, so this is the order a reader should meet them in rather than a
/// constraint: the structure, then the words the engine implements, then this game's rules.
pub const FOUNDATION: [(&str, &str, &str); 3] = [
    (
        "schema",
        "data/foundation/schema.4x",
        include_str!("../data/foundation/schema.4x"),
    ),
    (
        "engine",
        "data/foundation/engine.4x",
        include_str!("../data/foundation/engine.4x"),
    ),
    (
        "rules",
        "data/foundation/rules.4x",
        include_str!("../data/foundation/rules.4x"),
    ),
];

/// Where each foundation file lives, relative to this crate - the paths of [`FOUNDATION`].
///
/// **`tests/common`'s `LOADED` is this**, so a file moving between columns is one edit.
pub const PATHS: [&str; 3] = [FOUNDATION[0].1, FOUNDATION[1].1, FOUNDATION[2].1];

/// Every foundation file a run reads - the three above, and the two a script fetches.
///
/// # `PATHS` answers a narrower question than *what does a run read*
///
/// **`S-220`, and it is this week's shape for the fifth time.** `R-11` asks that every file the
/// engine reads as input be reachable from the index, the index listed `PATHS`, and `PATHS` is
/// what `include_str!` compiles in. **`setup.4x` is read every time a test runs** - it is what
/// every test loads before it runs - and it reaches `script.4x` by `{load file:script.4x
/// into:1}`. So the engine read five and the list named three, and a check over the three
/// returned a clean answer about the wrong population.
///
/// **This is not a second list that agrees with the first today**, which `S-220` says is the
/// arrangement that produced the defect. `the_foundation_is_every_file_in_its_directory` compares
/// it against the directory itself, so a sixth file makes the check red on the day it lands
/// rather than on the day somebody notices the page is short.
///
/// **Why the two are not `include_str!`-ed beside the three.** What carries a file into the
/// binary is what the engine loads without being asked; these are fetched by a script, and
/// `script.rs` resolves them through the harness. **Listing them is about a reader finding them**,
/// which is what `R-11` asks for, and says nothing about how they are carried.
pub const READ_BY_A_RUN: [&str; 5] = [
    FOUNDATION[0].1,
    FOUNDATION[1].1,
    FOUNDATION[2].1,
    "data/foundation/script.4x",
    "data/foundation/setup.4x",
];

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

    /// **`READ_BY_A_RUN` is the directory, not a list beside it.**
    ///
    /// **This is what stops `S-220` happening again**, and the item says why a list would not:
    /// *two lists that agree today is the arrangement that produced this.* So the population is
    /// asked of the disk and the constant is required to be exactly it - a sixth file makes this
    /// red on the day it lands.
    ///
    /// **`tests/` is a directory and is excluded by being one.** What is counted is the files a
    /// run loads, and the tests under it are loaded one at a time by name.
    #[test]
    fn the_foundation_is_every_file_in_its_directory() {
        let at = format!(
            "{}/data/foundation",
            env!("CARGO_MANIFEST_DIR").replace('\\', "/")
        );
        let mut on_disk: Vec<String> = std::fs::read_dir(&at)
            .unwrap_or_else(|why| panic!("cannot read {at}: {why}"))
            .filter_map(|it| it.ok())
            .map(|it| it.path())
            .filter(|it| it.is_file())
            .filter_map(|it| {
                it.file_name()
                    .and_then(|it| it.to_str())
                    .map(str::to_string)
            })
            .filter(|name| name.ends_with(".4x"))
            .map(|name| format!("data/foundation/{name}"))
            .collect();
        on_disk.sort();

        // **A count over nothing is the same failure with the sign flipped** - an unreadable
        // directory would make the two lists agree at zero.
        assert!(
            on_disk.len() >= 5,
            "only {} foundation file(s) on disk, so this compared almost nothing",
            on_disk.len()
        );

        let mut listed: Vec<String> = READ_BY_A_RUN.iter().map(|it| it.to_string()).collect();
        listed.sort();
        assert_eq!(
            listed, on_disk,
            "`READ_BY_A_RUN` and `data/foundation/` disagree - a file a run reads that nothing              lists is one no reader can reach from the index, which is `R-11`"
        );
    }

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
        // **Nothing carried is the friendly source, and that is the assertion that was missing.**
        //
        // `P-563` pointed two of these three at `spec/data/`, and `P-576` made that directory the
        // **friendly** form - so the binary embedded names where the engine resolves ids, and the
        // whole suite went red at once. Compilation was unaffected, because `include_str!` only
        // embeds text.
        //
        // **This fails on that state rather than describing it**: an `include_str!` reaching
        // outside `data/foundation/` is reaching for something nobody generated.
        let source: Vec<&str> = FOUNDATION
            .iter()
            .map(|(_, at, _)| *at)
            .filter(|at| !at.starts_with("data/foundation/"))
            .collect();
        assert!(
            source.is_empty(),
            "{source:?} is carried and is not generated - the engine reads the foundation form, \
             and `spec/data/` is the friendly source it is converted from"
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
