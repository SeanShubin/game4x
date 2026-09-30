//! A world for the console's own tests, stated here rather than run out of `scenario/commands/`.
//!
//! # Why these tests stopped reading the scenario
//!
//! **Sean, 2026-09-29, on the five test files that read it**: *delete 11, rebuild 6.* `D-4`
//! deletes `scenario/commands/` and the expectation beside it, and eleven of the seventeen tests
//! there were about the old ruleset's numbers or about the scenario playing through - the first
//! have nothing left to assert, and the second are `D-5`'s
//! `the_arc_d5_describes_is_the_arc_that_runs` said twice.
//!
//! **The six that remain are about the console and not about the ruleset**: the grammar, the
//! history, the state round trip, the diff and the seeding protocol. They used the scenario as a
//! convenient world - `four_kinds.rs` said so itself, *a designed planet with play begun, so every
//! form has something real to act on*. **They need a world, not that world.**
//!
//! # This is a waypoint and says so
//!
//! **It is still typed commands against the old model**, because the console has not been ported
//! yet - `C-175`. What it buys today is that `scenario/commands/` can go without taking these six
//! with it. **When the console plays the engine, this becomes rows through `Game::of`** and the
//! six are rebuilt against it, which is what `D-1` to `D-3` ask for.
//!
//! **`add-ark-orbit` below is in `spec/console.md` and `set-resource` is too.** The first was
//! taken out by `P-591` and put back by `P-592` - nothing else puts a unit anywhere, and a rule
//! that removes a command *when the notation can say what it said* protects it until something
//! can. `C-180`.

#![allow(dead_code)]
// **Each test binary compiles this whole module**, so a helper only one file uses is dead code in
// the others. That is how `mod common` works in Rust and not a sign of an unused helper.

use game_console::{Embedded, Session};

/// A planet with one developed territory and an ark above it.
///
/// **One territory rather than twelve.** What these tests need is somewhere real to act, and a
/// world that says more than that is a world whose details they would start depending on.
pub const SETUP: &str = "\
{create-planet size:tiny-12}
{set-resource territory:1 resource:food extractors:1 density:4}
{set-resource territory:1 resource:food extractors:2 density:4}
{set-resource territory:1 resource:metal extractors:2 density:4}
{set-resource territory:1 resource:energy extractors:2 density:4}
{add-ark-orbit territory:1}
";

/// Enough play to leave a state worth writing down and a history worth reading.
///
/// **The questions in it are the point of the questions being in it.** `spec/console.md` says a
/// history holds what changed the game and not what was asked of it, and the check for that ran
/// for months over `scenario/commands/play.4x` - **366 lines that asked nothing at all.** A true
/// statement about an empty population, which is `CLAUDE.md`'s *a count over nothing is the same
/// failure with the sign flipped*. These three make it bite.
pub const PLAY: &str = "\
{start}
{deploy where:2 what:ark}
{show-planet}
{toil where:1}
{work where:1 what:metal}
{help}
{toil where:1}
{work where:1 what:food}
{history}
{end-turn}
";

/// The lines above that ask rather than do, and none of them may reach the history.
pub const ASKED: [&str; 3] = ["{show-planet}", "{help}", "{history}"];

/// How many lines the fixture issues that should be recorded.
///
/// **Counted from the fixture rather than written down.** What reads it is a check about every
/// command reaching the history, so a number typed beside it would be the thing being checked
/// stated twice.
pub fn commands_issued() -> usize {
    let of = |text: &str| {
        text.lines()
            .filter(|line| !line.is_empty() && !ASKED.contains(line))
            .count()
    };
    of(SETUP) + of(PLAY) // `{start}` is a line of PLAY, so it is already counted
}

/// The two above, as a library the console can `{run file:...}`.
pub fn library() -> Embedded {
    Embedded::of(&[("setup", SETUP), ("play", PLAY)])
}

/// A designed planet with play begun, so every form has something real to act on.
pub fn playing() -> Session {
    let library = library();
    let mut session = Session::new();
    // **`{start}` is a line of `PLAY`**, so the design phase ends there rather than here - which
    // is what lets a test of the design phase run this and stop.
    session
        .run("{run file:setup}", &library)
        .unwrap_or_else(|why| panic!("the fixture does not set up: {why}"));
    session
}

/// A planet played a little way, which is what a state and a history are wanted from.
pub fn played() -> Session {
    let library = library();
    let mut session = playing();
    session
        .run("{run file:play}", &library)
        .unwrap_or_else(|why| panic!("the fixture does not play: {why}"));
    session
}
