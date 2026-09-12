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

/// The counts are spent by acting and put back by `refresh`, and they do not pool.
///
/// **`S-93`'s third point, and `P-411` changed what it takes to satisfy it for the second
/// time.** While readiness was a trait the danger was a weighting over kinds reporting the
/// whole economy as doing nothing - `refresh` took a thing and made a thing. `P-399` made it
/// a kind and the danger moved to pooling one action's tokens with another's. It is a count
/// carried as a trait again, so the place is the kind **and** the count - `citizen, laboring`
/// beside `citizen, bearing` - and a weighting blind to the difference would let a citizen
/// pay for laboring with the capacity it has to bear.
#[test]
fn a_count_is_spent_by_acting_and_the_counts_do_not_pool() {
    let document = release();
    let rules = nogain::rules(&document);

    // The counts are read from the Traits table, where `P-411` declares them.
    let counts = nogain::counts(&document);
    assert_eq!(
        counts,
        ["moving", "laboring", "working", "bearing", "defending"],
        "the traits whose values are `0 or 1` are the counts a thing carries"
    );

    // Every count is spent by something and put back by `refresh`, on some kind that has it.
    let mut checked = 0;
    for count in &counts {
        let of = |rule: &nogain::Rule, sign: i64| {
            rule.delta
                .iter()
                .any(|(place, change)| place.state == *count && change.signum() == sign)
        };
        let spent: Vec<&str> = rules
            .iter()
            .filter(|rule| of(rule, -1))
            .map(|rule| rule.name.as_str())
            .collect();
        assert!(
            !spent.is_empty(),
            "nothing spends a `{count}`, so the count is declared and never costs anything"
        );

        let put_back: Vec<&nogain::Rule> = rules
            .iter()
            .filter(|rule| rule.name.starts_with("refresh (") && of(rule, 1))
            .filter(|rule| rule.name.ends_with(&format!(" {count})")))
            .collect();
        assert!(
            !put_back.is_empty(),
            "`refresh` puts no `{count}` back, so acting would cost one thing for ever"
        );
        for rule in &put_back {
            assert_eq!(
                rule.draws_from(),
                ["time"],
                "`{}` puts a count back out of nothing, where `P-388` says it draws one out of time",
                rule.name
            );
        }
        checked += 1;
    }
    assert_eq!(checked, counts.len(), "every count, and the count with it");
    assert!(!counts.is_empty(), "no counts at all, so nothing above ran");

    // **The counts are separate places, which is the half a single place per kind loses.**
    // `work` must not be payable with the extractor's capacity to do something else.
    let work = rules
        .iter()
        .find(|rule| rule.name.starts_with("work ("))
        .expect("`work` is spelled out per density");
    let working = Place {
        kind: "extractor".to_string(),
        state: "working".to_string(),
    };
    assert_eq!(work.delta.get(&working), Some(&-1));
    assert!(
        work.delta.keys().all(|place| place.kind != "citizen"),
        "`work` spends something of a citizen's, so the counts have been pooled: {:?}",
        work.delta
    );

    // **A citizen's three counts are three places**, which is `C-90`'s whole point arriving
    // in the arithmetic: two citizens differing only in what they have left to do are two
    // descriptions, and two places.
    let citizen_counts: std::collections::BTreeSet<&str> = rules
        .iter()
        .flat_map(|rule| rule.delta.keys())
        .filter(|place| place.kind == "citizen" && !place.state.is_empty())
        .map(|place| place.state.as_str())
        .filter(|state| counts.iter().any(|count| count == state))
        .collect();
    assert_eq!(
        citizen_counts,
        ["bearing", "defending", "laboring"].into_iter().collect(),
        "a citizen's counts are three separate places"
    );

    // **A qualified thing is two places and a count is one**, which is the distinction that
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

/// Force is mustered from a count and swept in the same ending, and the arithmetic sees it.
///
/// **`P-414`, and it is the one thing in the release whose quantity is not a number.**
/// `muster` produces *that citizen's force* and `stand` *that unit's force*, so a check that
/// could only read a number would have dropped both rows and reported a game where force is
/// made from nothing - the exact shape `spec/invariants.md` exists to refuse.
#[test]
fn force_is_mustered_from_a_count_and_costs_what_it_takes() {
    let document = release();
    let rules = nogain::rules(&document);
    let force = Place {
        kind: "force".to_string(),
        state: String::new(),
    };

    // The release's own numbers: a citizen is force 1, an ark and a pioneer 2.
    let forces = nogain::forces(&document);
    assert_eq!(forces.get("citizen"), Some(&1));
    assert_eq!(forces.get("ark"), Some(&2));
    assert_eq!(forces.get("pioneer"), Some(&2));

    let muster = rules
        .iter()
        .find(|rule| rule.name.starts_with("muster"))
        .expect("`muster` is a rule");
    assert_eq!(
        muster.delta.get(&force),
        Some(&1),
        "`muster` makes one citizen's force: {:?}",
        muster.delta
    );
    assert_eq!(
        muster.delta.get(&Place {
            kind: "citizen".to_string(),
            state: "defending".to_string()
        }),
        Some(&-1),
        "and it costs the citizen's capacity to defend: {:?}",
        muster.delta
    );

    let stood: Vec<&nogain::Rule> = rules
        .iter()
        .filter(|rule| rule.name.starts_with("stand ("))
        .collect();
    assert_eq!(
        stood.len(),
        2,
        "`stand` names the family `unit`, which is an ark and a pioneer: {:?}",
        stood.iter().map(|rule| &rule.name).collect::<Vec<_>>()
    );
    for rule in &stood {
        assert_eq!(
            rule.delta.get(&force),
            Some(&2),
            "`{}` makes that unit's force, which the release says is two",
            rule.name
        );
    }

    // And it is swept in the same ending, so nothing carries force from one turn to the next.
    assert!(
        rules
            .iter()
            .any(|rule| rule.name.starts_with("discard") && rule.delta.get(&force) == Some(&-1)),
        "nothing discards force, so it would accumulate across turns"
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

    // **`refresh` grounds per block again, and the blocks are what `P-414` left.** Six of
    // them: four counts put back on a citizen or an extractor, and two on a unit - which is
    // a family, so each of those two becomes an ark and a pioneer. Eight rules from six
    // blocks, and every one of them names the count it puts back.
    let refreshed: Vec<&str> = rules
        .iter()
        .filter(|rule| rule.name.starts_with("refresh ("))
        .map(|rule| rule.name.as_str())
        .collect();
    assert_eq!(
        refreshed,
        [
            "refresh (ark moving)",
            "refresh (pioneer moving)",
            "refresh (citizen laboring)",
            "refresh (citizen bearing)",
            "refresh (extractor working)",
            "refresh (citizen defending)",
            "refresh (ark defending)",
            "refresh (pioneer defending)",
        ],
        "`refresh` ground to {refreshed:?}"
    );
    assert!(
        !refreshed.is_empty(),
        "no refresh at all, so the comparison above is between two empty lists"
    );

    // `age` and `spoil` name `thing` too, and `keeps` is declared of food alone - so grounding
    // them against every kind would give this release twelve kinds that expire.
    assert_eq!(
        nogain::keeps(&document),
        ["food".to_string()],
        "the release declares a `keeps` counter of these kinds - and `X-30` is why a second          one has to stop the gate rather than widen the grounding: `age` fires to exhaustion,          so a kind with `keeps` 2 or more ages to nothing in one turn"
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
        "muster",
        "stand",
    ] {
        assert!(
            rules
                .iter()
                .any(|rule| rule.name == name || rule.name.starts_with(&format!("{name} ("))),
            "`{name}` is in the release and ground to no rule at all"
        );
    }
}

/// Every place is a kind or a count a thing carries, and never a derived trait.
///
/// **The page says this and a sentence on a generated page is not a check.** `markdown` tells
/// a reader that the invariant is over the kinds, that `metal in it` is derived and invisible
/// here, and that a green run is therefore not evidence about the release's *conserved*. That
/// claim is only worth making if something holds the vocabulary to it.
///
/// **The risk it guards is the one this repository keeps writing down.** A page titled
/// *nothing comes back round with more* reads as deciding more than it decides, and the
/// cheapest way for it to start lying is for a place to appear whose name is a derived trait -
/// at which point the page would look like it had begun covering conservation without anyone
/// deciding it should.
///
/// **Over every place, with the count**, because a run over no places would satisfy *none is
/// derived* for the wrong reason.
#[test]
fn every_place_is_a_kind_or_a_count_and_never_a_derived_trait() {
    let document = release();
    let rules = nogain::rules(&document);
    // **Every trait the release names, not only the counts.** `age` puts `keeps one less`
    // since `P-431`, so `keeps` is a place - and it is a trait the release declares, which
    // is the category this check is really about. Reading only the `0 or 1` counts made the
    // allowlist narrower than the claim it guards, and `keeps` arriving is what said so.
    let declared: Vec<String> = traits_marked(&document, "");

    let mut places: std::collections::BTreeSet<&Place> = std::collections::BTreeSet::new();
    for rule in &rules {
        places.extend(rule.delta.keys());
    }
    assert!(
        places.len() > 20,
        "only {} places, which is too few for this to be about the release",
        places.len()
    );

    // The three sources are places too, and they are not kinds - `spec/invariants.md` names
    // them, and they are the one thing here that is neither a kind nor a state of one.
    let sources = ["the planet", "the star", "time"];
    let mut checked = 0;
    for place in &places {
        let state = place.state.as_str();
        let known = state.is_empty()
            || declared.iter().any(|name| name == state)
            || nogain::COUNTERS.contains(&state);
        assert!(
            known || sources.contains(&place.kind.as_str()),
            "`{}` is a place and `{state}` is neither a trait the Traits table declares nor \
             one of the counters this check keeps - a place whose state names nothing the \
             release declares is a vocabulary this check invented",
            place.label()
        );
        checked += 1;
    }
    assert_eq!(checked, places.len(), "every place, and the count with it");

    // **The half above consults `COUNTERS`, which is this file's own list, so on its own it
    // would pass for a state added to that list.** What follows does not: the derived traits
    // are read from the release's *Stored or derived* column, and no place may use one.
    // **That is the page's claim stated against the release rather than against a constant
    // here**, and it is what catches the way this check would actually start covering
    // conservation - a derived trait becoming a place.
    let derived_traits: Vec<String> = traits_marked(&document, "derived");
    assert!(
        derived_traits.len() >= 3,
        "only {} derived traits read from the release, which is too few for the sweep below          to mean anything: {derived_traits:?}",
        derived_traits.len()
    );
    for place in &places {
        for name in &derived_traits {
            assert!(
                place.state != *name && place.kind != *name,
                "`{}` is a place and `{name}` is declared derived, so this check has begun                  reading a derived trait and the page's account of what it does not decide                  is false",
                place.label()
            );
        }
    }

    // **And the derived trait the page names by hand is really derived and really absent.**
    // Naming it keeps the page's example honest rather than only its general claim.
    let derived = document
        .lines()
        .find(|line| line.contains("**metal in it**"))
        .expect("the Traits table declares `metal in it`");
    assert!(
        derived.contains("derived"),
        "`metal in it` is not declared derived any more, so the page's example is wrong: \
         {derived}"
    );
    assert!(
        places
            .iter()
            .all(|place| !place.label().contains("metal in it")),
        "`metal in it` is a place, so this check now sees binding metal and the page says it \
         does not"
    );
}

/// Every trait the release's *Traits* table marks with this word, by name.
///
/// **Read from the table rather than listed**, so that a trait becoming derived tomorrow is
/// covered without anyone editing this file - which is the whole point of the sweep that uses
/// it.
fn traits_marked(document: &str, word: &str) -> Vec<String> {
    // An empty `word` matches every row, which is how the whole Traits table is read.
    let mut out = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line.trim() == "## Traits";
            continue;
        }
        let line = line.trim();
        if !inside || !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        let (Some(name), Some(kept)) = (cells.first(), cells.get(3)) else {
            continue;
        };
        if kept.contains(word) {
            out.push(name.trim_matches('*').trim().to_string());
        }
    }
    out
}
