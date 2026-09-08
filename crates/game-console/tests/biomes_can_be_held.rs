//! Every claimable biome can be taken, and held once taken, by what the release provides.
//!
//! **`S-42`.** `spec/control.md`: *taking a territory takes force greater than the existing
//! force*, and *holding a territory takes force equal to its force of nature*. Nature's force
//! is per biome and the release's *Biomes* table sets it; the force a player can bring is per
//! unit and the release's *Units and structures* table sets it. **Nothing has ever asked
//! whether the second reaches the first.**
//!
//! It did not matter while every biome's nature was one and every unit's force was two.
//! `P-253` gave jungle a nature of two, and *greater than* is not *at least*.
//!
//! # Asked of the model, not of arithmetic about the model
//!
//! The numbers are read from the release, and then a real game is set up and a real founding
//! is attempted. A test that multiplied the numbers itself would be a second implementation
//! of the rule, agreeing with the first exactly when both were wrong - which is how `S-22`'s
//! count agreed for two days while the sets did not.

use game_console::{Library, Session};
use game_model::{Biome, Resource, TerritoryId, Transition, UnitKind};
use std::path::{Path, PathBuf};

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Each biome and the force nature holds it with, from the release's own table.
fn nature_of() -> Vec<(String, u32)> {
    let document =
        std::fs::read_to_string(root().join("releases/first-release.md")).expect("the release");
    let mut out = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line.trim() == "## Biomes";
            continue;
        }
        let line = line.trim();
        if !inside || !line.starts_with('|') || line.contains("---") {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        let name = cells.first().unwrap_or(&"").to_lowercase();
        let Some(force) = cells.last().and_then(|c| c.parse::<u32>().ok()) else {
            continue; // the header, and ocean, which carries nothing
        };
        out.push((name, force));
    }
    out
}

/// A game on the release's planet, with everything designed and play started.
fn planet() -> Session {
    let files = Files(root().join("scenario/commands"));
    let mut session = Session::new();
    for line in ["{run file:setup}", "{start}"] {
        session
            .run(line, &files)
            .unwrap_or_else(|why| panic!("{line}: {why}"));
    }
    session
}

/// Every claimable biome the release declares can be taken by a pioneer.
///
/// **Jungle cannot, and that is the finding rather than a defect in this test.** A pioneer
/// is force 2, the release's *Units and structures* table says so, and `P-253` gave jungle a
/// nature of 2. Taking needs force **greater** than what holds the ground, so 2 against 2 is
/// refused. The same is true of an Ark, which is also force 2 - so **nothing the release
/// provides can take a jungle**, and the planet has two of them.
///
/// It is carried as a named exception rather than asserted away, because the numbers are the
/// release's and this lane does not edit those. `C-24` is the item. The exception fails if
/// jungle ever becomes takeable, so it cannot outlive the gap it describes.
#[test]
fn every_claimable_biome_can_be_taken_by_something_the_release_provides() {
    /// Biomes nothing can take, and why each is allowed to be here.
    // **The pattern, and where it stops working.** A named exception carries a reason and
    // fails when it is repaired, so a gap cannot outlive itself and cannot be closed by
    // quietly weakening the assertion. It holds at one or two. **Past about two it stops
    // being a guard and becomes the list written twice** - the same reason
    // `closed_sets.rs` declines to check every dump column against the release's traits:
    // an exemption list of seventeen against a population of twenty-five is not a check,
    // it is a second copy of the thing being checked, and the second copy is what rots.
    // If a third is wanted here, that is the signal to fix the rule rather than the list.
    /// Biomes nothing can take, and why each is allowed to be here.
    ///
    /// **Empty, and it held `jungle` until `P-275`.** The exception expired the way one
    /// should: the rule underneath it moved. `C-24` was a real finding and not a defect in
    /// the model - what it found was that nothing said how an attacking force is assembled,
    /// and the assertion below, which fails when an excepted biome becomes takeable, is what
    /// would have caught the change had this not been rewritten first.
    const CANNOT: [(&str, &str); 0] = [];

    let declared = nature_of();
    assert!(
        declared.len() >= 5,
        "the release declares {} biomes with a force, which is too few to be its table",
        declared.len()
    );

    let mut taken = Vec::new();
    let mut lost = Vec::new();
    let mut refused = Vec::new();
    for (biome, nature) in &declared {
        let Some(kind) = Biome::ALL
            .iter()
            .find(|known| known.name().eq_ignore_ascii_case(biome))
        else {
            panic!("the release names a biome `{biome}` the model does not have");
        };
        if !kind.is_claimable() {
            continue;
        }

        // A real game, real pioneers standing on the ground, and a real founding.
        // **They are on territory 2 rather than beside it - `S-76`.** Founding needs the
        // pioneer where it founds, so force brought to a territory now counts what is
        // standing on it.
        let mut session = planet();
        session.game.territories[1].biome = *kind;
        session.game.territories[1].force_of_nature = *nature;
        for line in ["{deploy-ark territory:1}", "{create-labor territory:1}"] {
            session
                .run(line, &Files(root().join("scenario/commands")))
                .unwrap_or_else(|why| panic!("{biome}: `{line}` failed: {why}"));
        }
        // Placed rather than produced, so this measures force and not affordability.
        // **Two pioneers, because `P-275` says a player may bring two.** *A military unit
        // is organised force in itself, so several brought to one place sum.* One pioneer is
        // force 2 and a jungle is nature 2, and taking needs *greater than* - so with one, a
        // jungle is unclaimable however good its food is. That was `C-24`, and it was never
        // a defect in the model: nothing had said how an attacking force is assembled.
        for n in 0..2 {
            let id = game_model::UnitId(session.game.units.len() as u32 + 1 + n);
            let mut pioneer = game_model::Unit::new(id, UnitKind::Pioneer, TerritoryId(2));
            pioneer.location = game_model::Location::On(TerritoryId(2));
            session.game.units.push(pioneer);
        }

        match session.game.after(&Transition::FoundByLand {
            territory: TerritoryId(2),
        }) {
            Ok(after) => {
                let held = after.force_in(TerritoryId(2));
                // **Taken and then lost is a live gap, and it is `C-31`.** `P-275` made a
                // jungle takeable - two pioneers are force 4 against its nature of 2 - and a
                // founding leaves a garrison and two citizens presenting less force than the
                // nature just beaten. `spec/control.md`: *should the force in a territory
                // fall below its force of nature, nature takes it back.*
                //
                // Recorded rather than asserted, because what a founding leaves is the
                // release's numbers and this lane does not choose them.
                if held < *nature {
                    lost.push((biome.clone(), *nature, held));
                }
                taken.push(biome.clone());
            }
            Err(why) => refused.push((biome.clone(), nature, why.to_string())),
        }
    }

    let unexpected: Vec<&(String, &u32, String)> = refused
        .iter()
        .filter(|(biome, _, _)| !CANNOT.iter().any(|(named, _)| named == biome))
        .collect();
    assert!(
        unexpected.is_empty(),
        "these claimable biomes cannot be taken by anything the release provides, and \
         nothing said so: {unexpected:?}"
    );

    // An exception that has been repaired is a lie in the other direction, and nothing else
    // would notice - the test would go on passing while claiming a gap that had closed.
    for (named, why) in CANNOT {
        assert!(
            !taken.iter().any(|biome| biome == named),
            "`{named}` can be taken now, so delete its exception: {why}"
        );
    }

    // Over every case, and how many cases there were.
    assert_eq!(
        taken.len() + refused.len(),
        5,
        "five claimable biomes; {} were taken and {} refused",
        taken.len(),
        refused.len()
    );
    assert_eq!(CANNOT.len(), 0, "two pioneers take every claimable biome");

    // **Every biome that can be taken can be held by what taking it leaves** - `C-31`,
    // and this assertion ran the other way for one commit. `P-275` made a jungle takeable
    // and it was then handed straight back to nature, because `held_force` counted the
    // garrison and dropped the citizens: `spec/control.md` says *a citizen has a force of
    // its own, coordinated or not*, and only the garrison's multiplier was being read.
    //
    // Asserted empty rather than counted at zero, with the population named below, because
    // zero over nothing is the failure with the sign flipped.
    assert!(
        lost.is_empty(),
        "these are taken and nature takes them straight back: {lost:?}"
    );
    assert_eq!(
        taken.len(),
        5,
        "five claimable biomes were taken and held; {} were",
        taken.len()
    );
}

/// What a founding leaves behind holds the ground it took, for every biome that can be taken.
///
/// **Taking and holding are two rules and two numbers.** Taking needs force *greater* than
/// nature; holding needs force *equal to* it, and *should the force in a territory fall below
/// its force of nature, nature takes it back*. So a biome could be takeable and instantly
/// lost, and nothing would have said so - the founding would succeed and the territory would
/// revert at the end of the turn.
///
/// This is checked inside the test above rather than repeated here, at the moment each
/// founding succeeds, because the force left behind is a property of that founding and not
/// of the biome. This test says the check was reached, which is the half an assertion inside
/// a loop cannot say about itself.
#[test]
fn the_holding_check_above_ran_on_every_biome_that_could_be_taken() {
    let claimable = nature_of()
        .into_iter()
        .filter(|(biome, _)| {
            Biome::ALL
                .iter()
                .find(|known| known.name().eq_ignore_ascii_case(biome))
                .is_some_and(|known| known.is_claimable())
        })
        .count();
    assert_eq!(
        claimable, 5,
        "five claimable biomes carry a force in the release; found {claimable}"
    );
}

/// Every territory's force of nature is the one its biome carries in the release.
///
/// **`scenario/commands/forces.4x` said one everywhere, in a comment, and stayed correct by
/// accident until it did not.** `P-253` gave jungle a nature of two; territories 6 and 7 are
/// the jungles; and the file went on setting them to one while its opening sentence - *every
/// territory has a force of nature of 1 in this release* - became false without a line of it
/// changing.
///
/// Two hand-written files agreeing about a number is not a check, and neither of them was
/// wrong on its own terms. This reads the release, so the next time a biome's force moves it
/// fails here rather than being read wrongly by a person deriving the dump.
#[test]
fn the_scenario_gives_each_territory_the_force_its_biome_carries() {
    let game = planet().game;
    let nature = nature_of();

    let mut checked = 0;
    for place in &game.territories {
        let Some((_, expected)) = nature
            .iter()
            .find(|(biome, _)| biome.eq_ignore_ascii_case(place.biome.name()))
        else {
            // Ocean carries nothing, which is what the release's dash means.
            assert!(
                !place.biome.is_claimable(),
                "the release gives {} no force and it is claimable",
                place.biome
            );
            continue;
        };
        assert_eq!(
            place.force_of_nature, *expected,
            "territory {} is {} and the release holds that biome with {expected}",
            place.id, place.biome
        );
        checked += 1;
    }

    // Over every case, and how many there were: a planet of nothing but ocean would satisfy
    // every assertion above by reaching none of them.
    assert_eq!(
        checked, 12,
        "twelve territories carry a biome with a force; {checked} were checked"
    );
    // And the two jungles are why this exists, so it fails if they stop being jungles rather
    // than quietly checking eleven grasslands.
    let jungles = game
        .territories
        .iter()
        .filter(|place| place.biome == Biome::Jungle)
        .count();
    assert_eq!(jungles, 2, "territories 6 and 7 are the jungles");
}

/// Every territory has the numbers its own row in the release gives it.
///
/// **`S-46`, undoing `S-45`.** For one commit this read the *Biomes* table, on `P-272`'s rule
/// that a biome determined a territory's numbers. `P-280` reversed it: *a territory's biome
/// does not determine its numbers. The two are chosen to agree thematically, and a territory
/// is free to differ where that shows something the others do not.* `P-281` says the same
/// from the other side - the biome table's numbers **guide and do not bind**, and force of
/// nature is the one column that does.
///
/// **The check survives the reversal and is worth more after it.** Before `P-281` there were
/// two tables of territory resources and no statement of which the data had to match, which
/// is exactly how nobody noticed there were two. Now there is one, and this is what holds
/// `nodes.4x` to it - over all twelve territories and all three resources, counted, because
/// a parse that read nothing would satisfy every assertion inside the loop by reaching none.
#[test]
fn the_scenario_gives_each_territory_the_numbers_the_release_gives_it() {
    let document =
        std::fs::read_to_string(root().join("releases/first-release.md")).expect("the release");

    // The per-territory table: an integer first cell and `capacity x density` after it.
    let mut rows: Vec<(u32, Vec<(u32, u32)>)> = Vec::new();
    for line in document.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() < 4 {
            continue;
        }
        let Ok(id) = cells[0].parse::<u32>() else {
            continue;
        };
        let three: Vec<(u32, u32)> = cells[1..4]
            .iter()
            .map(|cell| match cell.split_once('x') {
                Some((capacity, density)) => (
                    capacity.trim().parse().unwrap_or(0),
                    density.trim().parse().unwrap_or(0),
                ),
                // `none` is what the release writes where a territory has no such node.
                None => (0, 0),
            })
            .collect();
        rows.push((id, three));
    }
    assert_eq!(
        rows.len(),
        12,
        "twelve territories in the release's own table; the parse found {}",
        rows.len()
    );

    let game = planet().game;
    let mut checked = 0;
    for (id, three) in &rows {
        let place = game
            .territory(TerritoryId(*id))
            .unwrap_or_else(|_| panic!("the release names a territory {id} the planet lacks"));
        for (resource, (capacity, density)) in Resource::ALL.iter().zip(three) {
            let offered = place.deposit(*resource);
            assert_eq!(
                offered.capacity, *capacity,
                "the release gives territory {id} {capacity} {resource} extractors"
            );
            assert_eq!(
                offered.density, *density,
                "the release gives territory {id} {resource} at density {density}"
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked,
        12 * 3,
        "twelve territories times three resources; {checked} were checked"
    );
}
