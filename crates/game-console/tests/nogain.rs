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
    //
    // **Read by name since `P-473`.** This took `cells.get(3)`, and deleting the *Of* column
    // moved *Stored or derived* from 3 to 2 - so it read the *Values* cell, found no row
    // marked `derived`, and reported a place whose state was a trait the release declares.
    // **Fourth time in one evening**, and the guard below now covers this table too.
    let at = game_console::recipes::column_of(document, "## Traits", "Stored or derived");
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
        let (Some(name), Some(kept)) = (cells.first(), cells.get(at)) else {
            continue;
        };
        if kept.contains(word) {
            out.push(name.trim_matches('*').trim().to_string());
        }
    }
    out
}

/// No recipe row says `ready`, which is what let a dead branch and a rotted reader hide each
/// other.
///
/// **Two rots, stacked.** `nogain::groundings` had a branch narrowing `thing` to the kinds that
/// ready, taken when a recipe's Traits cell said *ready*. `P-411` made readiness a count and no
/// row has said it since, so the branch was unreachable. The function it called read cell 8 of
/// *Units and structures* and compared it with `yes` - and `P-459` replaced `yes` with counts,
/// then `P-466` cut the table to seven columns. **It returned an empty list unconditionally**,
/// and the assertion that catches an empty grounding sits below the branch that never ran.
///
/// **The gate was green through all of it**, which is the point. Neither rot could be seen from
/// the other: a dead branch hides a broken reader, and a reader nobody calls cannot fail.
///
/// **A claim of zero names what it counted against** - `CLAUDE.md`. Seventy-seven rows, so this
/// cannot pass by the table having parsed to nothing.
#[test]
fn every_recipe_row_names_a_count_rather_than_readiness() {
    let document = release();
    let rows = game_console::recipes::body_under(&document, "## Recipes");
    let at = game_console::recipes::column_of(&document, "## Recipes", "Traits");
    let saying: Vec<String> = rows
        .iter()
        .filter(|row| {
            row.get(at)
                .map(|traits| traits.contains("ready"))
                .unwrap_or(false)
        })
        .map(|row| row.get(at).cloned().unwrap_or_default())
        .collect();
    assert_eq!(
        rows.len(),
        77,
        "seventy-seven recipe rows is the population this counted against"
    );
    assert!(
        saying.is_empty(),
        "a recipe row says `ready`, and the grounding for it was deleted: {saying:?}"
    );

    // **And the counts are what replaced it**, asserted here so that *no row says ready* cannot
    // be satisfied by a table that says nothing about readiness at all.
    let counts = game_console::nogain::counts(&document);
    assert_eq!(
        counts,
        ["moving", "laboring", "working", "bearing", "defending"],
        "five counts replaced the one flag, and they are what a recipe row names instead"
    );
    let naming = rows
        .iter()
        .filter(|row| {
            row.get(at)
                .map(|traits| counts.iter().any(|count| traits.contains(count.as_str())))
                .unwrap_or(false)
        })
        .count();
    assert!(
        naming >= 8,
        "only {naming} recipe rows name a count, which is too few for this table to be the one \
         `P-411` wrote"
    );
}

/// The two tables this crate reads by position, and the order it assumes.
///
/// **This documents an assumption. It does not search for code that violates one.** That
/// distinction cost four readers on the evening it was written: `P-473` deleted the release's
/// *Of* column, this test went on passing - correctly, the release matched the list below -
/// and four readers elsewhere held a different list and reported plausible nonsense. A trait
/// table with no derived trait in it, three closed sets that became zero, a catalog printing
/// `(stored)` beside every trait, and a prototype saying `biome` names no set of values.
///
/// **A guard that asserts an assumption cannot find the code that disagrees with it.** It
/// fires when the release moves away from what is written here; it is blind to a reader that
/// was never reading what is written here. **Only taking the index out finds those** -
/// `recipes::column_of`, and the readers that call it.
///
/// **So read this as a record of what the positional readers that remain assume**, and not as
/// coverage of them. The next person to reach for it will take it for coverage; the lane that
/// wrote it did.
///
/// # What it is still for
///
/// **The release moving is the half it does catch**, and it catches it in the crate that does
/// the indexing. `prototypes/kinds` compares both headers with the release already, so a moved
/// column fails there too - but it fails in a crate that renders rather than one that reads,
/// and whoever repairs it there has no reason to look at a `row.get(n)` three files away.
///
/// **A header, not a count.** Asserting seven columns would pass a rename and a reorder, which
/// are the two changes that break a positional read.
///
/// Three readers went wrong this way before the four above, and each was green: `P-346` moved
/// *Costs to produce* and a reader of cell 5 said a pioneer has no metal cost; `P-466` moved
/// *Readies* to cell 5 and a reader of cell 8 said four things ready nothing; the same removal
/// left `nogain::readies` reading cell 8 of a seven-column table, returning an empty list, with
/// its only caller a branch dead since `P-411`.
#[test]
fn the_tables_this_crate_reads_by_position_are_in_the_order_it_assumes() {
    let document = release();
    let mut checked = 0;
    for (heading, columns) in [
        (
            "## Recipes",
            &["Recipe", "Owner", "Role", "Qty", "Kind", "Traits", "Where"][..],
        ),
        (
            "## Units and structures",
            &[
                "Thing", "Strength", "Fuel", "Upkeep", "Crosses", "Readies", "Movable",
            ][..],
        ),
        // **Added after `P-473` moved *Stored or derived* from 3 to 2** by deleting *Of*,
        // and `traits_marked` read cell 3 - the fourth positional read this evening to
        // survive a column going and report something plausible.
        ("## Traits", &["Trait", "Values", "Stored or derived"][..]),
    ] {
        for (at, column) in columns.iter().enumerate() {
            assert_eq!(
                game_console::recipes::column_of(&document, heading, column),
                at,
                "`{heading}` puts `{column}` somewhere other than {at}, and this crate reads \
                 that table by position. Every `row.get(n)` against it is now reading a \
                 different column and saying something true about the wrong cell - see \
                 `recipes::column_of`"
            );
            checked += 1;
        }
        // And no eighth cell in a row, because a reader that indexes past the end gets
        // `None` and treats it as an empty cell rather than as a question - which is what
        // `nogain::readies` did with cell 8 of this very table, silently, for a day.
        //
        // **Measured on a body row rather than on the header**, because a body row is what a
        // `row.get(n)` actually indexes into, and `body_under` drops the header.
        let width = game_console::recipes::body_under(&document, heading)
            .first()
            .map(Vec::len)
            .unwrap_or_default();
        assert_eq!(
            width,
            columns.len(),
            "a row of `{heading}` has {width} cells and this crate reads {}",
            columns.len()
        );
    }
    assert_eq!(
        checked, 17,
        "seven columns of each of the two wide tables, and three of the Traits table"
    );
}
