//! A turn: rules that fire without a command, which is the fourth concept.
//!
//! **This is the one `README.md` predicted would move the 232**, and the prediction is the point
//! of the file. A rule fired by a command gets its `$name` holes from the command. A rule fired by
//! a turn has no command, so every hole has to be bound from the world - which is the *search*
//! that `tests/counting.rs` found the edge of and got away without.
//!
//! **Two settlements and not one.** A search that enumerates is a search that can be got wrong in
//! a way one solution never shows, so the world under test has two and the count is asserted.

use thin_engine::engine::{Refused, run, turn};
use thin_engine::notation::write;
use thin_engine::store::Store;

mod common;
use common::{command, rules, world};

fn rows_of(store: &Store, relation: &str) -> Vec<String> {
    let mut found: Vec<String> = store
        .rows()
        .iter()
        .filter(|row| row.relation == relation)
        .map(write)
        .collect();
    found.sort();
    found
}

/// Two settlements, reached the way a player would reach them.
fn two_settlements() -> Store {
    let after = run(
        &world(),
        &rules(),
        &command("{move it:scout from:1 to:2 had:3 left:2}"),
    )
    .expect("the scout moves to 2");
    let after =
        run(&after, &rules(), &command("{found it:scout where:2}")).expect("the scout founds at 2");
    run(&after, &rules(), &command("{found it:pioneer where:1}")).expect("the pioneer founds at 1")
}

/// The same world, with the scout walked off the settlement it founded.
fn one_owner_walked_away() -> Store {
    run(
        &two_settlements(),
        &rules(),
        &command("{move it:scout from:2 to:3 had:2 left:1}"),
    )
    .expect("the scout leaves the settlement it founded")
}

/// **Every settlement grows, and the engine was told which ones by nobody.**
#[test]
fn a_turn_fires_a_rule_once_for_every_way_the_world_satisfies_it() {
    let before = two_settlements();
    assert_eq!(
        rows_of(&before, "settlement").len(),
        2,
        "two settlements, so the search below has two solutions and not one"
    );

    let after = turn(&before, &rules()).expect("a turn");

    assert_eq!(
        rows_of(&after, "food"),
        vec!["{food place:1}".to_string(), "{food place:2}".to_string()],
        "one food per settlement, and the engine bound `$where` from the world both times"
    );
}

/// **A turn fires no rule that a command fires**, which is what `by` is for.
#[test]
fn a_turn_does_not_fire_a_rule_a_command_fires() {
    let before = two_settlements();
    let after = turn(&before, &rules()).expect("a turn");

    assert_eq!(
        rows_of(&after, "at"),
        rows_of(&before, "at"),
        "nothing moved, because `move` is fired by a command and not by a turn"
    );
    assert_eq!(
        rows_of(&after, "settlement"),
        rows_of(&before, "settlement"),
        "nothing was founded either"
    );
}

/// **A turn over a world that satisfies nothing changes nothing**, and the assertion is that it
/// changed nothing rather than that it did not fail.
///
/// **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. If the search
/// returned no solutions for every rule, the test above would go green on an empty world too;
/// this is the control that says the world here really is the empty case.
#[test]
fn a_turn_over_a_world_with_no_settlement_changes_nothing() {
    let before = world();
    assert_eq!(
        rows_of(&before, "settlement").len(),
        0,
        "the starting world has no settlement, which is what makes this the empty case"
    );

    let after = turn(&before, &rules()).expect("a turn over a world that satisfies nothing");

    assert_eq!(
        rows_of(&after, "food").len(),
        0,
        "nothing grew, because nothing was there to grow"
    );
    assert_eq!(after, before, "and the world is the one it started as");
}

/// **A rule fired by a turn cannot be fired by a command**, and the refusal says so by name.
///
/// Without this `by` would mean something in one direction only: a turn would skip `move`, and a
/// player could still type `{grow where:1}` and make food out of a command.
#[test]
fn a_command_cannot_fire_a_rule_that_belongs_to_the_turn() {
    let why = run(&two_settlements(), &rules(), &command("{grow where:1}"))
        .expect_err("`grow` is the turn's rule");

    assert_eq!(
        why,
        Refused::NotByCommand {
            name: "grow".to_string(),
            by: "the turn".to_string()
        },
        "the refusal names the rule and what it is fired by"
    );
}

/// **A hole bound by one clause filters the next**, which is what joins a rule's clauses into one
/// rule rather than two.
///
/// `grow` needs a settlement at `$where` owned by `$who`, and `$who` to be standing at `$where`.
/// The scout founded at 2 and then walked to 3, so one of the two settlements has an absent owner
/// and does not grow. **With `$where` and `$who` bound independently per clause both would**, and
/// the test above could not tell: both places have somebody in them, so a cross product lands on
/// the same two answers. This is the case that separates them.
#[test]
fn a_settlement_whose_owner_walked_away_does_not_grow() {
    let before = one_owner_walked_away();
    assert_eq!(
        rows_of(&before, "settlement").len(),
        2,
        "still two settlements - what changed is where their owners are"
    );

    let after = turn(&before, &rules()).expect("a turn");

    assert_eq!(
        rows_of(&after, "food"),
        vec!["{food place:1}".to_string()],
        "the pioneer is still at 1 and the scout has left 2, so only 1 grows"
    );
}
