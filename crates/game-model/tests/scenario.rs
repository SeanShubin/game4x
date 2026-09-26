//! `D-5`'s clauses, asked of the main scenario as it actually runs.
//!
//! **`D-5`**: *a main scenario exists over the reviewed ruleset and I have watched it run: an Ark
//! deploys, that first territory is developed, a second is taken by land and developed too, and an
//! Ark launches from the second. Every rule the reviewed tests describe fires at least once while
//! it runs, measured by what fired rather than by what the file says.*
//!
//! **The watching is `examples/scenario.rs` and this is the part a gate can hold.** A person still
//! has to watch it; what this stops is the scenario quietly ceasing to show what it is for.
//!
//! **Every clause is read off the run**, never off `scenario/main.4x`. The file says what to play;
//! these say what happened.

use std::collections::BTreeSet;

/// **The runner, borrowed rather than copied**, the way `tests/reviewed.rs` borrows
/// `examples/report.rs` - so the thing Sean watches and the thing the gate holds cannot disagree
/// about what the scenario did.
#[path = "../examples/scenario.rs"]
#[allow(dead_code)]
mod scenario;

/// **Every rule fires, measured by what fired.**
///
/// # Why this cannot be read off the command list
///
/// **A command names one rule and may run many.** `end-turn` is `{part of:5 ...}` eight times over,
/// calling `upkeep`, `perish`, `breed`, `discard-disorder` and `refresh` six times - so counting the
/// relation each command opens with counts the player's half and calls it the whole.
///
/// **This lane wrote that version first and it reported six of fifteen**, which is a plausible
/// number over the wrong population. `Effect::fired` is what the engine actually applied, and the
/// same scenario reported nine. The other six came from the scenario growing, not from the
/// instrument.
///
/// **A refused firing is not in it**, which is what separates this from the transitive closure of
/// `{part ...}`: `repeatedly` builds each attempt aside and merges it only when the attempt
/// stands, so `upkeep` feeding nobody records nothing.
#[test]
fn every_rule_the_ruleset_declares_fires_at_least_once() {
    let (_, history, refused) = scenario::played();
    assert_eq!(refused, None, "the scenario was refused partway");

    let rules = scenario::every_rule();
    assert!(
        rules.len() > 10,
        "only {} rules, so finding them all fired means little",
        rules.len()
    );

    let fired: BTreeSet<String> = history
        .iter()
        .flat_map(|effect| effect.fired.iter().cloned())
        .collect();
    let missing: Vec<&String> = rules.difference(&fired).collect();
    assert!(
        missing.is_empty(),
        "{} of {} rules never fired: {missing:?}",
        missing.len(),
        rules.len()
    );

    // **Nothing fired that is not a rule**, which is the other direction and catches the day
    // `Effect::fired` starts recording something else.
    let strangers: Vec<&String> = fired.difference(&rules).collect();
    assert!(
        strangers.is_empty(),
        "these fired and are not rules: {strangers:?}"
    );
}

/// **The four things `D-5` says it has to show**, each read off the state the run left.
///
/// **A clause per assertion and the population asserted**, because *an ark is somewhere* and *an
/// ark is in the orbit above the territory that was taken* are different claims and only one of
/// them is `D-5`'s.
#[test]
fn the_arc_d5_describes_is_the_arc_that_runs() {
    let (after, history, refused) = scenario::played();
    assert_eq!(refused, None, "the scenario was refused partway");

    let rows = after.rows().rows();
    let holds = |relation: &str, wants: &[(&str, &str)]| -> usize {
        rows.iter()
            .filter(|row| row.relation == relation)
            .filter(|row| {
                wants
                    .iter()
                    .all(|(key, value)| row.value(key) == Some(*value))
            })
            .count()
    };

    // **The places are derived from the move and not named**, because the foundation form has no
    // names - `place-1` is the friendly rendering of `{place id:1 ...}`. **Naming them would be
    // hard-coding ids**, and a scenario that renumbered its world would pass this while showing
    // something else.
    //
    // **The move is what says which territory was taken**: its `from` is the surface landed on and
    // its `to` is the surface reached by land. Everything else follows from those two.
    let moved = history
        .iter()
        .find(|effect| effect.fired.iter().any(|it| it == "move"))
        .expect("the scenario moves something");
    let landed = moved
        .command
        .value("from")
        .expect("a move says where from")
        .to_string();
    let taken = moved
        .command
        .value("to")
        .expect("a move says where to")
        .to_string();
    assert_ne!(
        landed, taken,
        "a move that goes nowhere is not taking ground"
    );

    let territory_of = |place: &str| -> String {
        rows.iter()
            .find(|row| row.relation == "place" && row.value("id") == Some(place))
            .and_then(|row| row.value("of"))
            .unwrap_or_else(|| panic!("no place `{place}`"))
            .to_string()
    };
    assert_ne!(
        territory_of(&landed),
        territory_of(&taken),
        "both places are in one territory, so nothing was taken by land"
    );

    // **An Ark deploys**, and it is the first thing that happens.
    let deployed: Vec<&String> = history
        .iter()
        .flat_map(|effect| effect.fired.iter())
        .collect();
    assert_eq!(
        deployed.iter().filter(|it| **it == "deploy").count(),
        2,
        "an ark deploys and a pioneer deploys, which is the two settlements"
    );

    // **Both territories are developed**, which is extractors standing and working on each.
    for (what, place) in [("the landing", &landed), ("the one taken", &taken)] {
        let working = holds("extractor", &[("where", place), ("working", "1")]);
        assert!(
            working >= 2,
            "{what} has {working} working extractor(s), which is not a developed territory"
        );
    }

    // **A second is taken by land**, which is `move` and not a second landing.
    assert_eq!(
        deployed.iter().filter(|it| **it == "move").count(),
        1,
        "the second territory is reached by land exactly once"
    );

    // **An Ark launches from the second**, so the one ark left is in the territory that was taken
    // and is not on its surface - which is what *launched* means without naming a layer.
    let arks: Vec<&game_model::notation::Row> =
        rows.iter().filter(|row| row.relation == "ark").collect();
    assert_eq!(arks.len(), 1, "exactly one ark is left: {arks:?}");
    let at = arks[0].value("where").expect("an ark is somewhere");
    assert_eq!(
        territory_of(at),
        territory_of(&taken),
        "the ark is not in the territory that was taken by land"
    );
    assert_ne!(
        at, taken,
        "the ark is on the surface it was built on, so nothing launched"
    );
}

/// **The scenario ends with nobody alive, and that is recorded rather than asserted away.**
///
/// **`perish` fires twice on the closing turn** and each firing takes a whole group, so both
/// settlements starve. `scenario/main.4x` says why at length: nothing was stored, and
/// `reviewed/an-ark-lands-a-planet-is-developed-and-an-ark-leaves.4x` already says *what happens
/// to them next is the next turn's problem*. This is the next turn.
///
/// **It is a test so that it changes loudly.** If Sean decides the scenario should keep its
/// people - `C-150` - this fails, and whoever changes it has to say so rather than discovering
/// later that the ending moved.
#[test]
fn the_closing_turn_starves_both_settlements() {
    let (after, history, _) = scenario::played();
    let perished = history
        .iter()
        .flat_map(|effect| effect.fired.iter())
        .filter(|it| *it == "perish")
        .count();
    assert_eq!(perished, 2, "one firing per settlement");
    assert_eq!(
        after
            .rows()
            .rows()
            .iter()
            .filter(|row| row.relation == "citizen")
            .count(),
        0,
        "the scenario is written to end with nobody, and it did not"
    );
}
