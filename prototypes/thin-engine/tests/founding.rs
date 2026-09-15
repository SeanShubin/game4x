//! Founding: the second rule, and the first test of whether the engine grows when the game does.
//!
//! **The claim under test is the one `README.md` says is unproven**: that 232 lines of `src/` do
//! not grow when a mechanic is added, because a mechanic is rows. `move` established that a rule
//! can be stated as rows; it could not establish this, because there was only one of them.
//!
//! **The mechanic is chosen for what the game wants and not for what the engine can do.** Founding
//! a settlement needs three things - the founder is there, the place is a place, and there is not
//! already a settlement there - and the third of those is a negative, which the engine has no word
//! for. That is the point of picking it.

use thin_engine::engine::{Refused, run};
use thin_engine::notation::write;

mod common;
use common::{command, rules, world};

/// Every settlement in a world, written back in the notation and sorted so the order is nobody's.
fn settlements(store: &thin_engine::store::Store) -> Vec<String> {
    let mut found: Vec<String> = store
        .rows()
        .iter()
        .filter(|row| row.relation == "settlement")
        .map(write)
        .collect();
    found.sort();
    found
}

/// The scout is at 1, and 1 is a place.
#[test]
fn founding_puts_a_settlement_where_the_founder_is() {
    let after = run(&world(), &rules(), &command("{found it:scout where:1}"))
        .expect("the scout is at 1 and 1 is a place");

    assert_eq!(
        settlements(&after),
        vec!["{settlement owner:scout place:1}".to_string()],
        "one settlement, at 1, owned by the founder"
    );
}

/// **A founder founds where it is**, and the refusal names the row that is not true.
#[test]
fn nothing_founds_a_settlement_where_it_is_not() {
    let why = run(&world(), &rules(), &command("{found it:scout where:2}"))
        .expect_err("the scout is at 1, not at 2");

    assert_eq!(
        why,
        Refused::NotSo {
            rule: "found".to_string(),
            wanted: "{at place:2 thing:scout}".to_string()
        },
        "the refusal names the row the world does not have"
    );
}

/// **A place takes one settlement**, which is the half of this rule the engine has no word for.
///
/// Stated as a requirement rather than as an encoding: *after two foundings at the same place,
/// there is one settlement there*. Whatever makes that true is the data's business, and this test
/// is what says something has.
#[test]
fn a_place_takes_one_settlement_and_the_second_founding_is_refused() {
    let first = run(&world(), &rules(), &command("{found it:scout where:1}"))
        .expect("the scout founds at 1");

    let why = run(&first, &rules(), &command("{found it:pioneer where:1}"))
        .expect_err("1 already has a settlement");

    assert!(
        matches!(why, Refused::NotSo { .. }),
        "the second founding is refused by a row the world does not have, and said: {why}"
    );
    assert_eq!(
        settlements(&first),
        vec!["{settlement owner:scout place:1}".to_string()],
        "the one settlement at 1 is the scout's, and the pioneer's founding added nothing"
    );
}
