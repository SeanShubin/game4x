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

/// The readiness economy is visible, which is the whole reason the places are not kinds.
///
/// **`S-93`'s third point, checked rather than trusted.** At kind granularity `refresh` takes
/// a thing and makes a thing and nets to zero, and so does `work` - the code lane found the
/// second of those in the drawing, which is what corrected the item. If the places here
/// collapsed to kinds, both rules would net nothing and the check would pass while saying
/// nothing at all.
#[test]
fn readiness_is_a_place_and_the_rules_that_move_it_are_not_silent() {
    let document = release();
    let rules = nogain::rules(&document);

    let ready = |kind: &str| Place {
        kind: kind.to_string(),
        state: "ready".to_string(),
    };

    // `work` spends an extractor's readiness and makes material. At kind granularity it takes
    // an extractor and makes an extractor: nothing.
    let work = rules
        .iter()
        .find(|rule| rule.name.starts_with("work ("))
        .expect("`work` is spelled out per density");
    assert_eq!(
        work.delta.get(&ready("extractor")),
        Some(&-1),
        "`work` does not spend an extractor's readiness, so the drawing's blindness has been \
         inherited: {:?}",
        work.delta
    );
    assert!(
        !work.delta.contains_key(&Place {
            kind: "extractor".to_string(),
            state: String::new()
        }),
        "`work` changes how many extractors there are, and it should change only what one of \
         them has left to give"
    );

    // `refresh` puts readiness back, and draws it out of time to do so.
    let refresh = rules
        .iter()
        .find(|rule| rule.name == "refresh (extractor)")
        .expect("`refresh` is ground to the kinds that ready");
    assert_eq!(refresh.delta.get(&ready("extractor")), Some(&1));
    assert_eq!(
        refresh.draws_from(),
        ["time"],
        "`refresh` makes readiness out of nothing, where `P-388` says it draws one out of time"
    );

    // **`renew` is the same shape and nothing said so until the rule was applied.** The
    // release calls fertility *renewed each turn*; that makes it a readiness extractor too,
    // which falls out of `P-388` rather than being decided here.
    let renew = rules
        .iter()
        .find(|rule| rule.name == "renew")
        .expect("`renew` is a rule");
    assert_eq!(
        renew.draws_from(),
        ["time"],
        "`renew` refills a capacity and draws on nothing to do it"
    );

    // **And a rule that makes a new thing does not draw on time for the capacity it comes
    // with.** Without that distinction every rule that makes anything would read as a draw,
    // because a thing arrives ready.
    let breed = rules
        .iter()
        .find(|rule| rule.name == "breed")
        .expect("`breed` is a rule");
    assert!(
        breed.draws_from().is_empty(),
        "`breed` draws on {:?}; a new citizen's readiness came with the citizen, which was \
         paid for in fertility and food",
        breed.draws_from()
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

    // `refresh` names the family `thing`; the release closes readiness to four kinds.
    let refreshed: Vec<&str> = rules
        .iter()
        .filter(|rule| rule.name.starts_with("refresh ("))
        .map(|rule| rule.name.as_str())
        .collect();
    assert_eq!(
        refreshed.len(),
        nogain::readies(&document).len(),
        "`refresh` ground to {refreshed:?}, and the release's *Readies* column names {:?}",
        nogain::readies(&document)
    );
    assert!(
        !refreshed.is_empty(),
        "nothing readies, so the comparison above is between two empty lists"
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
        "renew",
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
