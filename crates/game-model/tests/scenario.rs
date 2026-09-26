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

/// **Fourteen of the fifteen rules fire, and the fifteenth is `perish`.**
///
/// # `D-5` and `spec/scenarios.md` cannot both hold, which is `C-150`
///
/// **`D-5`**: *every rule the reviewed tests describe fires at least once while it runs.*
/// **`spec/scenarios.md`**: *a mechanic that only appears in an unusual situation belongs in a
/// scenario of its own. Those are not built until the main scenario satisfies its reader.*
///
/// **Starvation is that mechanic.** A loop that sustains its people never fires `perish`, so the
/// main scenario can satisfy one sentence or the other. **This asserts what is true today and names
/// the one exception**, rather than asserting fifteen and getting there by playing badly - which is
/// what the first version of this scenario did.
///
/// **Measured by what fired.** `Effect::fired` is what the engine applied; counting
/// `command.relation` instead reported six of fifteen, because `end-turn` runs five rules through
/// `{part ...}` and the count saw one of them.
#[test]
fn every_rule_but_perish_fires_and_perish_is_named() {
    let (_, history, refused) = scenario::played();
    assert_eq!(refused, None, "the scenario was refused partway");

    let rules = scenario::every_rule();
    assert!(
        rules.len() > 10,
        "only {} rules, so this would say little",
        rules.len()
    );

    let fired: BTreeSet<String> = history
        .iter()
        .flat_map(|effect| effect.fired.iter().cloned())
        .collect();
    let missing: Vec<&str> = rules.difference(&fired).map(String::as_str).collect();
    assert_eq!(
        missing,
        ["perish"],
        "the only rule this scenario does not fire is `perish`"
    );

    // **Nothing fired that is not a rule**, which catches the day `Effect::fired` records something
    // else.
    let strangers: Vec<&String> = fired.difference(&rules).collect();
    assert!(
        strangers.is_empty(),
        "these fired and are not rules: {strangers:?}"
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
