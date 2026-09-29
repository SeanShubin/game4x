//! Nothing comes back round with more, decided rather than believed.
//!
//! **`spec/invariants.md` -> Nothing comes back round with more**: *whether this holds is decided
//! mechanically, from the rules alone, and stays so however many rules there are.* `S-219` is
//! that sentence having had nothing behind it since `a8386450`.
//!
//! **The reader and the solver are in `examples/nogain.rs`**, which also writes the page. What is
//! here is what fails the gate.
//!
//! # Why a green run here is worth something
//!
//! **Passing tests prove nothing on their own** - `CLAUDE.md`. So this does not only ask whether
//! the game is sound today; it asks whether the instrument could say otherwise, by handing it two
//! games that gain and requiring it to refuse both. One of the two gains only around a cycle, and
//! no single rule of it makes anything from nothing - which is the case a simpler check would
//! wave through.

use std::collections::BTreeMap;

/// The reader, the solver and the page, borrowed rather than run as a subprocess.
///
/// **A subprocess would contend on the target lock**, which is what made
/// `every_committed_report_is_what_the_generator_writes` flaky until it borrowed `write_all`.
#[path = "../examples/nogain.rs"]
#[allow(dead_code)]
mod nogain;

use nogain::{Decision, Found, Place, Rule, SOURCES};

/// **A weighting exists, so no sequence of rules ends holding more than it began with.**
#[test]
fn a_weighting_exists_over_every_rule() {
    let decision = nogain::decided();

    // **Assert both populations** - a weighting over no rules exists trivially, and would be a
    // green run about nothing. `CLAUDE.md`: *a count over nothing is the same failure with the
    // sign flipped.*
    assert_eq!(
        decision.read.rules_read, 16,
        "`spec/data/rules.4x` states a different number of rules than this was written against"
    );
    assert!(
        decision.read.ground.len() >= 30,
        "only {} ground rule(s), so the weighting was solved over almost nothing",
        decision.read.ground.len()
    );

    let Found::Weighting(weights) = &decision.found else {
        let Found::NotFound { forcing } = &decision.found else {
            unreachable!()
        };
        panic!(
            "no weighting exists over the rules in `spec/data/rules.4x`, so some sequence of \
             them comes back round with more - which `spec/invariants.md` says may never \
             happen. The rules that make without taking are where to look: {}",
            forcing.join(", ")
        );
    };
    assert!(
        weights.len() >= 25,
        "only {} place(s) were weighed, so the rules were read too coarsely to say anything",
        weights.len()
    );
}

/// **The published weighting is checked by substitution, not by trusting the tableau.**
///
/// **This is the answer produced a second way.** The simplex says feasible; this adds each rule
/// up under the numbers it returned and requires every one to be at or below zero. A pivoting
/// bug that returned a plausible weighting would pass the test above and fail this one - and
/// `CLAUDE.md` records that class as the one no check catches, because *a check is the thing
/// that has the predicate*. Here there are two predicates and they are not the same one.
#[test]
fn every_rule_is_non_increasing_under_the_weighting_that_was_found() {
    let decision = nogain::decided();
    let Found::Weighting(weights) = &decision.found else {
        panic!("no weighting, which the test above reports properly");
    };

    // **Every weight is at least one**, which is what stops `w = 0` satisfying everything and
    // saying nothing about the game.
    for (place, weight) in weights {
        assert!(
            *weight >= 1,
            "`{}` is weighed at {weight}, and a weighting of nothing satisfies every rule",
            place.label()
        );
    }

    let mut checked = 0;
    for one in &decision.read.ground {
        let mut margin = 0;
        for (place, by) in &one.rule.delta {
            let weight = weights.get(place).unwrap_or_else(|| {
                panic!(
                    "`{}` moves `{}`, which the weighting does not weigh at all",
                    one.rule.name,
                    place.label()
                )
            });
            margin += by * weight;
        }
        assert!(
            margin <= 0,
            "`{}` comes out at {margin} under the published weighting, so it ends holding more \
             than it began with",
            one.rule.name
        );
        checked += 1;
    }
    assert_eq!(
        checked,
        decision.read.ground.len(),
        "not every rule was weighed"
    );
    assert!(checked >= 30, "only {checked} rule(s) were weighed");
}

/// **A rule that makes something out of nothing is refused**, which is the easy half.
#[test]
fn a_rule_that_makes_something_from_nothing_is_refused() {
    let decision = nogain::decided();
    let mut rules = decision.rules();
    let before = rules.len();

    rules.push(Rule {
        name: "a glitch that mints a metal".to_string(),
        delta: BTreeMap::from([(
            Place {
                kind: "metal".to_string(),
                state: String::new(),
            },
            1,
        )]),
    });
    assert_eq!(rules.len(), before + 1);

    assert!(
        matches!(nogain::solve(&rules), Found::NotFound { .. }),
        "the solver accepted a rule that makes a metal and takes nothing, so it would accept an \
         infinite resource glitch and a green run from it would mean nothing"
    );
}

/// **A game that gains only around a cycle is refused too**, which is the half that matters.
///
/// **No single rule here makes anything from nothing.** Take the time away from `refresh` and
/// every rule still takes something for what it makes - but `toil` turns a citizen from
/// `laboring 1` to `laboring 0` and hands back a labor, and `refresh` turns it back for free, so
/// the two together mint labor forever. **A check that only looked at one rule at a time would
/// pass this**, and that is exactly the glitch Sean asked to be sure of catching.
#[test]
fn a_game_that_gains_only_around_a_cycle_is_refused() {
    let decision = nogain::decided();
    let free: Vec<Rule> = decision
        .rules()
        .into_iter()
        .map(|mut rule| {
            rule.delta.retain(|place, _| place.kind != "time");
            rule
        })
        .collect();

    // **The change has to have bitten**, or this would assert about the game as it is. Without
    // it, a rename of the source would leave the test passing for the wrong reason.
    let touched = decision
        .rules()
        .iter()
        .filter(|rule| rule.delta.keys().any(|place| place.kind == "time"))
        .count();
    assert!(
        touched >= 6,
        "only {touched} rule(s) draw on time, so removing that draw changed almost nothing and \
         this test is not about the cycle it names"
    );
    assert!(SOURCES.contains(&"time"), "time is no longer a source");

    assert!(
        matches!(nogain::solve(&free), Found::NotFound { .. }),
        "with nothing paid for a turn's refresh, `toil` and `refresh` mint labor around a cycle \
         and the solver still found a weighting - so it does not see a gain that no single rule \
         commits"
    );
}

/// **Padding the generated page changes nothing**, which `CLAUDE.md` requires of a generated file.
///
/// **The property rather than the tool.** `tools/pad-tables` keeps its own workspace *deliberately
/// so it never appears in `cargo tree`*, so this cannot call it; what it asserts instead is the
/// invariant that tool's own test asserts - every line of a table is the same width. A generator
/// that wrote ragged tables would be padded by `hooks/pre-commit` into something it does not
/// write, and the check above would then fail on every commit.
#[test]
fn the_generated_tables_are_already_padded() {
    let at = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../reports/nogain.md");
    let text = std::fs::read_to_string(&at).expect("reports/nogain.md");

    let mut tables = 0;
    let mut rows = 0;
    let mut widths: Vec<usize> = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with('|') {
            widths.push(line.chars().count());
            rows += 1;
            if lines.peek().is_none_or(|next| !next.starts_with('|')) {
                assert!(
                    widths.windows(2).all(|two| two[0] == two[1]),
                    "a table in `reports/nogain.md` has rows of {widths:?} characters, so \
                     `hooks/pre-commit` would pad it into something the generator does not write"
                );
                tables += 1;
                widths.clear();
            }
        }
    }

    // **Assert both populations.** With no tables found, every table is padded vacuously.
    assert!(tables >= 2, "only {tables} table(s) were checked");
    assert!(rows >= 40, "only {rows} table row(s) were checked");
}

/// **The committed page is what the generator writes**, so a stale verdict fails the gate.
///
/// **This is `dumps_are_current`'s shape**, and the reason `R-9` asks for it: a generated page
/// nobody compares describes whatever the rules looked like when somebody last remembered to run
/// something.
#[test]
fn the_committed_report_is_what_the_generator_writes() {
    let reports = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../reports");
    let before: Vec<(std::path::PathBuf, String)> = ["nogain.html", "nogain.md"]
        .iter()
        .map(|name| {
            let at = reports.join(name);
            let text = std::fs::read_to_string(&at).unwrap_or_default();
            (at, text)
        })
        .collect();
    for (at, text) in &before {
        assert!(
            !text.is_empty(),
            "`{}` is missing or empty - run `scripts/reports.sh`",
            at.display()
        );
    }

    let decision: Decision = nogain::decided();
    nogain::write_report(&decision);

    let moved: Vec<String> = before
        .iter()
        .filter(|(at, was)| std::fs::read_to_string(at).unwrap_or_default() != *was)
        .map(|(at, _)| at.display().to_string())
        .collect();
    assert!(
        moved.is_empty(),
        "the committed no-gain report is not what the rules now say - run `scripts/reports.sh` \
         and commit the result:\n  {}",
        moved.join("\n  ")
    );
}
