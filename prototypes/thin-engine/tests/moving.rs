//! Moving: the first rule, stated entirely in `data/rules.4x`.
//!
//! **Three territories, two adjacencies, one vehicle.** Moving `scout` from 1 to 2 succeeds;
//! moving it to 3 fails because 3 is not adjacent, and moving it to 9 fails because 9 is not a
//! place. Every mechanic the main tree has - resources, turns, capacity, combat - is absent
//! because none of these needs one.

use thin_engine::engine::{Refused, run};
use thin_engine::notation::write;

mod common;
use common::{command, rules, world};

/// The scout is at 1, and 1 is adjacent to 2.
#[test]
fn the_scout_moves_to_a_place_that_is_adjacent() {
    let after = run(&world(), &rules(), &command("{move it:scout from:1 to:2}"))
        .expect("1 is adjacent to 2 and the scout is at 1");

    // **Where the scout is, and not where everything is.** Written the second way first, and a
    // second vehicle in `data/world.4x` failed it - the assertion said *the scout is at 2 and
    // nowhere else* and checked *the world contains exactly this one `at` row*, which is a
    // narrower question than the one the sentence asks. `CLAUDE.md` names the class.
    let at: Vec<String> = after
        .rows()
        .iter()
        .filter(|row| row.relation == "at" && row.value("thing") == Some("scout"))
        .map(write)
        .collect();
    assert_eq!(
        at,
        vec!["{at place:2 thing:scout}".to_string()],
        "the scout is at 2, and is nowhere else"
    );
}

/// The scout is at 1, and 1 is adjacent to 2 and to nothing else.
#[test]
fn the_scout_does_not_move_to_a_place_that_is_not_adjacent() {
    let why = run(&world(), &rules(), &command("{move it:scout from:1 to:3}"))
        .expect_err("1 is not adjacent to 3");

    assert_eq!(
        why,
        Refused::NotSo {
            rule: "move".to_string(),
            wanted: "{adjacent from:1 to:3}".to_string()
        },
        "the refusal names the row the world does not have"
    );
}

/// **A place that is not a place, and a place that is not next to you, are different refusals.**
/// That is the whole of this concept, so the test asserts the difference and not only the words:
/// before `{needs rule:move relation:territory id:$to}` was a row, both of these named
/// `{adjacent from:1 to:...}` and nothing could tell them apart.
///
/// **The engine did not change to take this.** `territory` was already stated in `data/world.4x`
/// and read by nothing; one `needs` row is what began reading it.
#[test]
fn a_place_that_does_not_exist_and_a_place_that_is_not_adjacent_refuse_differently() {
    let no_such_place = run(&world(), &rules(), &command("{move it:scout from:1 to:9}"))
        .expect_err("there is no territory 9");
    assert_eq!(
        no_such_place,
        Refused::NotSo {
            rule: "move".to_string(),
            wanted: "{territory id:9}".to_string()
        },
        "the refusal names the place that is not one"
    );

    let not_adjacent = run(&world(), &rules(), &command("{move it:scout from:1 to:3}"))
        .expect_err("1 is not adjacent to 3");
    assert_eq!(
        not_adjacent,
        Refused::NotSo {
            rule: "move".to_string(),
            wanted: "{adjacent from:1 to:3}".to_string()
        },
        "territory 3 exists, so the refusal is about the adjacency and not about the place"
    );

    assert_ne!(
        no_such_place, not_adjacent,
        "the two refusals are the concept; if they are equal the engine cannot tell them apart"
    );
}
