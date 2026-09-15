//! The first test, which is `temporary-notes/first-test.md`.
//!
//! **Sean's words, in `S-136`**: three territories `1, 2, 3`; two adjacencies `1-2` and `2-3`; one
//! vehicle named `scout`. Moving the scout from 1 to 2 succeeds; from 1 to 3 fails. **No mechanic
//! that is not needed to pass it.**
//!
//! # Entirety before, entirety after
//!
//! `data/before.4x` is everything - the structure, the structure's own structure, the rule, the
//! command and the world. `data/after.4x` is that file with one row different. **The engine is
//! what gets from one to the other, and this is what proves it does.**
//!
//! The two files are compared whole. That is stronger than comparing the rows anybody expected to
//! change, and stronger in the direction that has already bitten here: an earlier version compared
//! only the scout's rows, under a message claiming the scout was *nowhere else*, and could not
//! have noticed anything else moving.

use thin_engine::engine::run;

mod common;
use common::{after, before};

/// **Running command 1 turns the entirety before into the entirety after.**
#[test]
fn the_engine_gets_from_before_to_after() {
    let start = before();
    let end = run(&start, "1").expect("the scout is at 1 and 1 is adjacent to 2");

    assert_eq!(
        end.shown(),
        after().shown(),
        "the whole of the data afterwards is `data/after.4x`, row for row"
    );
}

/// **The two files differ by exactly one row**, which is what makes the test above say what it
/// looks like it says.
///
/// Without this, `before.4x` and `after.4x` could have drifted into agreeing - and a test that the
/// engine turns one into the other would pass by the two being the same file. **The count is the
/// control**: 98 rows, 97 of them shared.
#[test]
fn before_and_after_differ_by_one_row_and_nothing_else() {
    let start = before().shown();
    let end = after().shown();

    assert_eq!(start.len(), 98, "the entirety is 98 rows");
    assert_eq!(end.len(), 98, "and so is the entirety afterwards");

    let moved: Vec<&String> = start.iter().filter(|row| !end.contains(row)).collect();
    let arrived: Vec<&String> = end.iter().filter(|row| !start.contains(row)).collect();
    assert_eq!(moved, vec!["{residency what:1 where:1}"], "one row leaves");
    assert_eq!(
        arrived,
        vec!["{residency what:1 where:2}"],
        "one row arrives"
    );
}

/// The scout is at 1, and 1 is adjacent to 2 and to nothing else.
///
/// **The refusal names the row the world does not have.** There is no error about movement in the
/// engine - `{adjacency from:1 to:3}` is the whole of the message.
#[test]
fn the_scout_does_not_move_to_a_place_that_is_not_adjacent() {
    // The same command with one argument changed, stated the way every other command is.
    let start = common::with(
        "{command id:2 rule:move}\n\
                              {argument id:2.what command:2 input:move.what value:1}\n\
                              {argument id:2.from command:2 input:move.from value:1}\n\
                              {argument id:2.to command:2 input:move.to value:3}",
    )
    .expect("a second command is well formed");

    let why = run(&start, "2").expect_err("1 is not adjacent to 3");

    assert_eq!(
        format!("{why}"),
        "`move` needs {adjacency from:1 to:3} and it is not",
        "the refusal names the row the world does not have"
    );
}

/// **A refused command leaves everything exactly as it was.**
///
/// **Not a property this test establishes**: `run` takes `&Game` and returns a new one, so a
/// half-applied world is not a thing that can be built, and this would pass without the property.
/// It asserts the observable half - that the caller still holds what it started with.
#[test]
fn a_refused_command_changes_nothing() {
    let start = common::with(
        "{command id:2 rule:move}\n\
                              {argument id:2.what command:2 input:move.what value:1}\n\
                              {argument id:2.from command:2 input:move.from value:1}\n\
                              {argument id:2.to command:2 input:move.to value:3}",
    )
    .expect("a second command is well formed");
    let held = start.shown();

    run(&start, "2").expect_err("1 is not adjacent to 3");

    assert_eq!(start.shown(), held, "the data is what it started as");
    assert!(
        held.contains(&"{residency what:1 where:1}".to_string()),
        "and the scout is still in territory 1, which is the row that would have moved"
    );
}
