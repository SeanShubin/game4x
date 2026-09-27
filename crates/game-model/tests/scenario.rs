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

    // **The places are derived and never named**, because the foundation form has no names -
    // `place-1` is the friendly rendering of `{place id:1 ...}`. **Naming them would be hard-coding
    // ids**, and a scenario that renumbered its world would pass this while showing something else.
    //
    // # Derived from the settlements and not from the move, which is a correction
    //
    // **This read the first move's `from` as the landing and its `to` as the ground taken.** That
    // held while the scenario moved exactly one thing. Sean then asked for the ark to move once
    // before the rest, and the first move became the ark's between two orbits - so the test took an
    // orbit for the landing and reported *the landing has 0 working extractors*, which was true of
    // an orbit and nothing to do with the arc.
    //
    // **A `deploy` is what makes a settlement**, and `D-5` names two of them: the ark's and the
    // pioneer's, in that order. **Where each settled is the `where` of what it made** - the
    // extractors and citizens it put on the ground - which is the place itself rather than the
    // `where` the command names, because an ark deploys from the orbit above and a pioneer from the
    // ground it stands on.
    let settled: Vec<String> = history
        .iter()
        .filter(|effect| effect.fired.iter().any(|it| it == "deploy"))
        .map(|effect| {
            effect
                .made
                .iter()
                .find_map(|row| row.value("where"))
                .unwrap_or_else(|| panic!("a deploy made nothing anywhere"))
                .to_string()
        })
        .collect();
    assert_eq!(
        settled.len(),
        2,
        "two settlements: an ark deploys and a pioneer deploys - found {settled:?}"
    );
    let (landed, taken) = (settled[0].clone(), settled[1].clone());
    assert_ne!(landed, taken, "both settlements are in one place");

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
        "both settlements are in one territory, so nothing was taken by land"
    );

    // **And some move joins those two territories**, which is what *by land* means. The scenario may
    // move other things - an ark between orbits does not take ground - so this asks whether any move
    // crossed between the settled territories rather than assuming which move did.
    let by_land = history.iter().any(|effect| {
        effect.fired.iter().any(|it| it == "move")
            && [effect.command.value("from"), effect.command.value("to")]
                .iter()
                .filter_map(|it| it.map(territory_of))
                .collect::<BTreeSet<String>>()
                == BTreeSet::from([territory_of(&landed), territory_of(&taken)])
    });
    assert!(
        by_land,
        "no move joins the two settled territories, so the second was not taken by land"
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

    // **Nothing here counts moves.** It asserted exactly one, which was true while the pioneer was
    // the only thing that moved and stopped being true the moment Sean asked for the ark to move
    // too. **What `D-5` asks is that the second territory be reached by land**, and the `by_land`
    // check above asks that directly - whether some move crossed between the two settled
    // territories - so a scenario may move as many things as it likes.

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

/// **The loop sustains its population, and it needs nothing stored to do it.**
///
/// **Sean, 2026-09-26**: *every territory should have a food deposit with a density, so we should be
/// able to sustain a population without food stores.*
///
/// **An earlier version of this scenario starved both settlements** and this lane reported that as a
/// fact about the ruleset. It was not: turn four spent every labour on the launch and worked no
/// food, so `upkeep` had nothing to feed anybody with. **Measured separately before this was
/// changed**: two citizens over a food deposit of density six become six in four turns with nothing
/// stored - `upkeep` sixteen times, `breed` four, `perish` never.
///
/// **So this asserts the thing that was wrong**: every settled place ends with citizens on it, and
/// `perish` never fires.
#[test]
fn nobody_starves_and_nothing_had_to_be_stored() {
    let (after, history, refused) = scenario::played();
    assert_eq!(refused, None, "the scenario was refused partway");

    let perished = history
        .iter()
        .flat_map(|effect| effect.fired.iter())
        .filter(|it| *it == "perish")
        .count();
    assert_eq!(perished, 0, "a sustained loop starves nobody");

    // **Per place rather than in total**, because one settlement thriving while another dies is
    // exactly the failure a total would hide.
    let mut peopled = 0;
    for place in ["1", "3"] {
        let there: usize = after
            .rows()
            .rows()
            .iter()
            .filter(|row| row.relation == "citizen" && row.value("where") == Some(place))
            .filter_map(|row| row.value("quantity"))
            .filter_map(|it| it.parse::<usize>().ok())
            .sum();
        assert!(there > 0, "place {place} ends with nobody on it");
        peopled += 1;
    }
    assert_eq!(peopled, 2, "both settlements were asked about");

    // **Nothing was stored that they needed.** The one bin holds metal, so the food that fed them
    // came out of the ground on the turn it was eaten.
    let food = after
        .rows()
        .rows()
        .iter()
        .filter(|row| row.relation == "food")
        .count();
    assert_eq!(
        food, 0,
        "food survived the turn, so this proves nothing about deposits"
    );
}

/// **The committed playthrough is what the scenario produces now.**
///
/// **`scenario/played.md` is the thing Sean reads**, so a rule change that moves it has to move the
/// file too. `crates/game-console/tests/dumps_are_current.rs` holds the reports this way and the
/// argument is the same: a generated file that is committed and stale is worse than one that is
/// absent, because it reads exactly like one that is current.
///
/// **It fails with the first line that differs** rather than with a byte count, because the point of
/// a committed playthrough is that its diff says what changed.
#[test]
fn the_committed_playthrough_is_current() {
    let at = scenario::played_at();
    let committed = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("{}: {why} - run `scripts/scenario.sh`", at.display()));
    let produced = scenario::played_as_markdown();

    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. Two empty
    // strings are equal.
    assert!(
        produced.lines().count() > 100,
        "the runner produced {} line(s), which is not a playthrough",
        produced.lines().count()
    );

    if committed != produced {
        let differs = committed
            .lines()
            .zip(produced.lines())
            .enumerate()
            .find(|(_, (was, now))| was != now);
        match differs {
            Some((at_line, (was, now))) => panic!(
                "scenario/played.md is stale at line {}: it says\n  {was}\nand the scenario now \
                 produces\n  {now}\nRun `scripts/scenario.sh`.",
                at_line + 1
            ),
            None => panic!(
                "scenario/played.md has {} line(s) and the scenario produces {} - run \
                 `scripts/scenario.sh`",
                committed.lines().count(),
                produced.lines().count()
            ),
        }
    }
}

/// **`D-5`'s two halves: every rule a typical game uses fires, and one that does not is named.**
///
/// **`D-5`, promoted 2026-09-26**: *every rule a typical game uses fires at least once while it
/// runs, measured by what fired rather than by what the file says, and a rule that does not fire is
/// named with the unusual situation it needs - so an omission is something I can read rather than
/// something I have to notice.*
///
/// # The second half is the one that carries the signal
///
/// **The clause this replaces asked only that every rule fire.** Under it, a rule that silently
/// stopped firing read as fourteen-of-fifteen and nothing said which or why. **Under this one it is
/// a failure unless somebody wrote down the unusual situation it needs**, which is what
/// `scenario::UNUSUAL` is - and `scenario/played.md` renders it, so the omission reaches the file
/// Sean reads.
///
/// # Both directions, and both counts
///
/// **A named list that silently empties is the same hole one level up**, which the specification
/// lane asked to be closed here. So: every rule either fired or is named; nothing is named that
/// fired; and the two counts are asserted against today's figures rather than only against each
/// other.
///
/// **The figures move when a starvation scenario is built**, which is the point - `perish` will fire
/// then, `UNUSUAL` empties, and this fails until somebody says so.
///
/// **Measured by what fired.** `Effect::fired` is what the engine applied; counting
/// `command.relation` instead reported six of fifteen, because `end-turn` runs five rules through
/// `{part ...}` and the count saw one of them.
#[test]
fn every_rule_fires_or_is_named_with_what_it_needs() {
    let (_, history, refused) = scenario::played();
    assert_eq!(refused, None, "the scenario was refused partway");

    let rules = scenario::every_rule();
    let fired: BTreeSet<String> = history
        .iter()
        .flat_map(|effect| effect.fired.iter().cloned())
        .collect();
    let named: BTreeSet<String> = scenario::UNUSUAL
        .iter()
        .map(|(rule, _)| rule.to_string())
        .collect();

    // **Every rule is accounted for one way or the other**, which is the whole clause in one line.
    let unaccounted: Vec<&String> = rules
        .difference(&fired)
        .filter(|it| !named.contains(*it))
        .collect();
    assert!(
        unaccounted.is_empty(),
        "{} rule(s) fired nowhere and are named nowhere: {unaccounted:?} - `D-5` asks that an \
         omission be readable rather than noticed",
        unaccounted.len()
    );

    // **Nothing is named that fired**, so an entry cannot outlive the situation it describes. A
    // starvation scenario makes `perish` fire, and this is what says to delete its entry.
    let stale: Vec<&String> = named.intersection(&fired).collect();
    assert!(
        stale.is_empty(),
        "these are named as unusual and fired anyway: {stale:?} - delete their entries in \
         `scenario::UNUSUAL`"
    );

    // **Nothing is named that is not a rule**, which catches a typo in an entry rather than letting
    // it excuse nothing.
    let strangers: Vec<&String> = named.difference(&rules).collect();
    assert!(
        strangers.is_empty(),
        "these are named in `UNUSUAL` and are not rules: {strangers:?}"
    );

    // **And nothing fired that is not a rule**, which catches the day `Effect::fired` records
    // something else.
    let odd: Vec<&String> = fired.difference(&rules).collect();
    assert!(odd.is_empty(), "these fired and are not rules: {odd:?}");

    // **Both counts, because either set being empty would make this a different check.** With
    // nothing fired every rule would be unaccounted and the first assertion would catch it; with
    // nothing named it would catch that too - but a run where `rules` itself was empty passes every
    // assertion above in silence.
    assert_eq!(rules.len(), 15, "the ruleset is fifteen rules: {rules:?}");
    assert_eq!(fired.len(), 14, "fourteen of them fire: {fired:?}");
    assert_eq!(named.len(), 1, "one is named as unusual: {named:?}");
    assert_eq!(
        fired.len() + named.len(),
        rules.len(),
        "the two halves partition the ruleset, with nothing counted twice"
    );
}
