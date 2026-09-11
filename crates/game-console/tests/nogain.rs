//! The no-gain check decides `spec/invariants.md`'s invariant, and is not vacuous - `S-93`.
//!
//! **The danger here is not a wrong answer, it is a right one about the wrong question.** A
//! weighting can satisfy every rule by saying nothing: weigh everything at zero, or let the
//! places be so coarse that the rules which move things between states appear to move nothing.
//! `S-93` names the second directly - at kind granularity `refresh` takes a thing and makes a
//! thing, and *a weighting over kinds would report the readiness economy as doing nothing at
//! all*.
//!
//! So these check the instrument as hard as the answer: what the places are, that the rules
//! which ought to bite do bite, and that a rule which really does gain is caught.

use game_console::nogain::{self, Found, Place};
use std::path::Path;

fn release() -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release")
}

/// A weighting exists, and under it no rule ends with more than it began.
///
/// **Re-checked here rather than taken from the solver.** The solver's own arithmetic is
/// rational and the published weighting is whole, so the two are not the same calculation -
/// this is the second one, and it is what would have caught the rounding that broke
/// `work (metal x8)` before it was replaced by exact scaling.
#[test]
fn a_weighting_exists_and_no_rule_gains_under_it() {
    let document = release();
    let rules = nogain::rules(&document);
    let Found::Weighting(weighting) = nogain::solve(&rules) else {
        panic!("no weighting; `reports/nogain.md` names the rules that force it");
    };

    let mut checked = 0;
    for rule in &rules {
        let worth: i64 = rule
            .delta
            .iter()
            .map(|(place, change)| change * weighting.get(place).copied().unwrap_or(1))
            .sum();
        assert!(
            worth <= 0,
            "`{}` ends with {worth} more than it began, under the weighting this solved for",
            rule.name
        );
        checked += 1;
    }
    assert_eq!(
        checked,
        rules.len(),
        "every rule is checked, and the count is said so a loop over nothing cannot pass"
    );
    assert!(
        rules.len() > 30,
        "only {} rules were ground out of the release, which is too few for this to be about \
         the game rather than about a parse that failed",
        rules.len()
    );

    // **Every weight is at least one.** A weighting of zeroes satisfies every rule and says
    // nothing about the game, which is the vacuous answer this check exists to avoid.
    for (place, weight) in &weighting {
        assert!(
            *weight >= 1,
            "`{}` is weighed at {weight}, and a weight of nothing means the rules were not \
             balanced so much as ignored",
            place.label()
        );
    }
}

/// The readiness economy is visible, and the actions do not pool.
///
/// **`S-93`'s third point, and `P-399` changed what it takes to satisfy it.** While readiness
/// was a trait, the danger was a weighting over kinds reporting the whole economy as doing
/// nothing - `refresh` took a thing and made a thing. It is a kind now, so the danger moved:
/// the places must separate one action's tokens from another's, because `P-399` declares that
/// *two recipes naming the same action draw on the same tokens* and two naming different
/// actions never compete. One `readiness` place would say the opposite.
#[test]
fn readiness_is_spent_by_acting_and_the_actions_do_not_pool() {
    let document = release();
    let rules = nogain::rules(&document);

    let token = |action: &str| Place {
        kind: "readiness".to_string(),
        state: format!("for `{action}`"),
    };

    // Every action declared is spent by something and made by `refresh`.
    let actions = nogain::actions(&document);
    assert_eq!(
        actions,
        ["move", "labor", "work", "bearing"],
        "the `for` trait's declared values are what `refresh` grounds against"
    );

    let mut checked = 0;
    for action in &actions {
        let spent: Vec<&str> = rules
            .iter()
            .filter(|rule| rule.delta.get(&token(action)).copied().unwrap_or(0) < 0)
            .map(|rule| rule.name.as_str())
            .collect();
        assert!(
            !spent.is_empty(),
            "nothing spends a readiness `for {action}`, so the action is declared and never              costs anything"
        );

        let made = rules
            .iter()
            .find(|rule| rule.name == format!("refresh ({action})"))
            .unwrap_or_else(|| panic!("`refresh` does not ground to `{action}`"));
        assert_eq!(made.delta.get(&token(action)), Some(&1));
        assert_eq!(
            made.draws_from(),
            ["time"],
            "`refresh ({action})` makes a readiness out of nothing, where `P-388` says it              draws one out of time"
        );
        checked += 1;
    }
    assert_eq!(
        checked,
        actions.len(),
        "every action, and the count with it"
    );
    assert!(
        !actions.is_empty(),
        "no actions at all, so nothing above ran"
    );

    // **The tokens are separate places, which is the half a single `readiness` place loses.**
    // `work` must not be payable with a readiness for moving.
    let work = rules
        .iter()
        .find(|rule| rule.name.starts_with("work ("))
        .expect("`work` is spelled out per density");
    assert_eq!(work.delta.get(&token("work")), Some(&-1));
    assert!(
        work.delta.get(&token("move")).is_none(),
        "`work` spends a readiness for moving, so the actions have been pooled"
    );

    // **A qualified thing is two places and a token is one**, which is the distinction that
    // took a run to find: `perish` takes a citizen out of the unpaid pool *and* out of the
    // citizens, where counting only the pool had it remove nobody.
    let perish = rules
        .iter()
        .find(|rule| rule.name == "perish")
        .expect("`perish` is a rule");
    assert_eq!(
        perish.delta.get(&Place {
            kind: "citizen".to_string(),
            state: String::new()
        }),
        Some(&-1),
        "`perish` leaves the citizens unchanged, so it removes nobody: {:?}",
        perish.delta
    );
}

/// A rule that really gains is caught, and the check says which.
///
/// **The half that cannot be shown against the release**, because the release does not gain.
/// A check that has only ever passed is a claim rather than evidence, so this hands it a
/// document with one rule added that makes two metal out of one and asserts it is refused.
#[test]
fn a_rule_that_makes_more_than_it_takes_is_refused_by_name() {
    let document = release();
    let honest = nogain::rules(&document);
    assert!(
        matches!(nogain::solve(&honest), Found::Weighting(_)),
        "the release does not gain, which is what makes the doctored version below a test of \
         the check rather than of the release"
    );

    // One row on, one row off: a rule that turns a metal into two.
    let doctored = document.replace(
        "| **discard**         | world  | consume | 1                                    | metal     |                                               |                          |",
        "| **glitch**          | world  | consume | 1                                    | metal     |                                               |                          |\n\
         |                     |        | produce | 2                                    | metal     |                                               |                          |",
    );
    assert_ne!(
        doctored, document,
        "the doctored release is the release, so this tests nothing"
    );

    let rules = nogain::rules(&doctored);
    assert!(
        rules.iter().any(|rule| rule.name == "glitch"),
        "the added rule did not survive the grounding, so the refusal below would be about \
         something else"
    );
    match nogain::solve(&rules) {
        Found::Weighting(weighting) => {
            let glitch = rules
                .iter()
                .find(|rule| rule.name == "glitch")
                .expect("added");
            let worth: i64 = glitch
                .delta
                .iter()
                .map(|(place, change)| change * weighting.get(place).copied().unwrap_or(1))
                .sum();
            panic!("a rule making two metal from one was accepted, at a worth of {worth}");
        }
        Found::NotFound { forcing } => {
            assert!(
                forcing.iter().any(|name| name == "glitch"),
                "refused, and the rule that forces it is not named: {forcing:?}"
            );
        }
    }
}

/// The grounding spells families and densities out, and neither silently drops a rule.
#[test]
fn every_block_becomes_at_least_one_rule() {
    let document = release();
    let rules = nogain::rules(&document);

    // **`refresh` grounds per action now, not per kind that readies.** `P-399` turned the
    // question round: it was *which things can be refreshed*, read from the *Readies* column,
    // and it is *which actions are there*, read from the `for` trait's declared values. One
    // rule per action, because tokens for different actions never compete.
    let refreshed: Vec<&str> = rules
        .iter()
        .filter(|rule| rule.name.starts_with("refresh ("))
        .map(|rule| rule.name.as_str())
        .collect();
    assert_eq!(
        refreshed.len(),
        nogain::actions(&document).len(),
        "`refresh` ground to {refreshed:?}, and the `for` trait declares {:?}",
        nogain::actions(&document)
    );
    assert!(
        !refreshed.is_empty(),
        "no actions at all, so the comparison above is between two empty lists"
    );

    // `age` and `spoil` name `thing` too, and `keeps` is declared of food alone - so grounding
    // them against every kind would give this release twelve kinds that expire.
    assert_eq!(
        nogain::keeps(&document),
        ["food".to_string()],
        "the release declares a `keeps` counter of these kinds"
    );

    // Every block is represented. Counted by name rather than by position, because a block
    // that ground to nothing would leave a gap nothing else reports.
    for name in [
        "deploy ark",
        "move",
        "work",
        "upkeep",
        "bear",
        "breed",
        "refresh",
    ] {
        assert!(
            rules
                .iter()
                .any(|rule| rule.name == name || rule.name.starts_with(&format!("{name} ("))),
            "`{name}` is in the release and ground to no rule at all"
        );
    }
}
