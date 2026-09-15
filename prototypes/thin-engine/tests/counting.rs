//! Counting: the third concept, and the one `README.md` said was most likely to fail.
//!
//! **Every value is a string, and the engine has no arithmetic.** `1` and `scout` are the same
//! sort of thing to `src/notation.rs`, which records that as deliberately open. Moving burns one
//! fuel, and burning one is a subtraction.
//!
//! **What is under test is where the subtraction lives**, the same question the negative raised
//! in `tests/founding.rs` and with a different answer available: a negative could be written as a
//! fact because an absence is a fact. A sum is not obviously one.

use thin_engine::engine::{Refused, run};
use thin_engine::notation::write;

mod common;
use common::{command, rules, world};

fn fuel(store: &thin_engine::store::Store, thing: &str) -> Vec<String> {
    store
        .rows()
        .iter()
        .filter(|row| row.relation == "fuel" && row.value("thing") == Some(thing))
        .map(write)
        .collect()
}

/// The scout has 3 fuel, and arrives at 2 with one less.
#[test]
fn moving_burns_one_fuel() {
    let after = run(
        &world(),
        &rules(),
        &command("{move it:scout from:1 to:2 had:3 left:2}"),
    )
    .expect("the scout is at 1, 1 is adjacent to 2, and the scout has fuel");

    assert_eq!(
        fuel(&after, "scout"),
        vec!["{fuel amount:2 thing:scout}".to_string()],
        "three fuel became two, and the scout has one fuel row and not two"
    );
}

/// **The floor is a row that is not there.** The world states what one less than each amount is,
/// and states nothing for zero - so a vehicle at zero has no move available and the refusal says
/// which row it wanted.
#[test]
fn a_vehicle_with_no_fuel_cannot_move() {
    let why = run(
        &world(),
        &rules(),
        &command("{move it:pioneer from:1 to:2 had:0 left:0}"),
    )
    .expect_err("the pioneer has no fuel");

    assert_eq!(
        why,
        Refused::NotSo {
            rule: "move".to_string(),
            wanted: "{less is:0 of:0}".to_string()
        },
        "nothing is one less than zero, and the missing row is the refusal"
    );
}

/// **The command has to carry what the world already knows, and cannot lie about it.**
///
/// This is the cost of the encoding rather than a hole in it. `$name` is substitution and not
/// search, so the engine cannot read the scout's fuel out of the world - the command states it.
/// **What keeps that sound is that both halves are checked against rows**: an overstated `had`
/// is not a fuel row the world has, and an understated `left` is not a `less` row anybody wrote.
#[test]
fn a_command_that_misstates_the_fuel_is_refused_on_both_halves() {
    let overstated = run(
        &world(),
        &rules(),
        &command("{move it:scout from:1 to:2 had:9 left:8}"),
    )
    .expect_err("the scout does not have 9 fuel");
    assert_eq!(
        overstated,
        Refused::NotSo {
            rule: "move".to_string(),
            wanted: "{fuel amount:9 thing:scout}".to_string()
        },
        "claiming fuel the world does not state is refused by the world"
    );

    let kept = run(
        &world(),
        &rules(),
        &command("{move it:scout from:1 to:2 had:3 left:3}"),
    )
    .expect_err("3 is not one less than 3");
    assert_eq!(
        kept,
        Refused::NotSo {
            rule: "move".to_string(),
            wanted: "{less is:3 of:3}".to_string()
        },
        "moving without paying is refused by the table that says what paying is"
    );
}
