//! What this release says about biomes, which since `P-522` is nothing.
//!
//! # What this file asked, and what it asks now
//!
//! **`S-42`.** `spec/control.md`: *taking a territory takes force greater than the existing
//! force*, and *holding a territory takes force equal to its force of nature*. Nature's force
//! was per biome and the release's *Biomes* table set it; the force a player can bring is per
//! unit and *Units and structures* sets it. **Nothing had ever asked whether the second
//! reaches the first**, and it did not matter while every biome's nature was one and every
//! unit's force was two - `P-253` gave jungle a nature of two, and *greater than* is not *at
//! least*.
//!
//! **`P-522` cut the Biomes section and the whole of force from this release.** So there is no
//! table to read, no nature to hold ground with, and no claim left for those three tests to
//! make. `spec/planet.md` keeps every biome and `spec/control.md` keeps force, so the question
//! is deferred rather than answered, and the tests that asked it are one commit back.
//!
//! # Why the cut is asserted rather than the file deleted
//!
//! **An empty hand list is what hides half a cut.** A release that deleted the table while
//! leaving a `biome` column in *Kinds*, or a Strength row that something still musters, would
//! satisfy a deletion and fail nothing. So what is left in place of the three is one test that
//! reads the release and says: no Biomes table, no table row mentioning a biome, and no recipe
//! row producing or consuming a force.
//!
//! **It fails the day the table comes back**, which is the day the three tests are wanted
//! again and the day `S-42`'s question needs an answer.
//!
//! # What did not depend on biomes and stayed
//!
//! `the_scenario_gives_each_territory_the_numbers_the_release_gives_it` reads *Territory
//! resources*, which `P-522` did not touch: twelve territories, three resources each, capacity
//! and density compared against the planet the scenario builds. **It is asked of the model
//! rather than of arithmetic about the model** - a test that multiplied the numbers itself
//! would be a second implementation agreeing with the first exactly when both were wrong,
//! which is how `S-22`'s count agreed for two days while the sets did not.

use game_console::{Library, Session};
use game_model::{Resource, TerritoryId};
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

/// The release states no biome and no force, and states it everywhere rather than in patches.
///
/// **This is what is left of three tests whose subject `P-522` cut**, and it is written to
/// fail in both directions: a half-cut release fails now, and a release that brings biomes
/// back fails then. See the note at the top of this file for which three and where they went.
#[test]
fn the_release_declares_no_biome_and_no_force_of_nature() {
    let document =
        std::fs::read_to_string(root().join("releases/first-release.md")).expect("the release");

    // A count over nothing is the same failure with the sign flipped - `CLAUDE.md`. A release
    // that failed to load would mention nothing and pass every assertion below.
    let rows: Vec<&str> = document
        .lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with('|'))
        .collect();
    assert!(
        rows.len() > 80,
        "only {} table rows parsed out of the release, so this said almost nothing",
        rows.len()
    );

    assert!(
        !document.contains("## Biomes"),
        "the release has a Biomes section again, so the three tests this replaced are wanted"
    );

    // **Table rows and not prose.** The release's own record of the cut says the word - `R-4`
    // is a capability delivered and then put out of scope - and a release explaining why
    // something went is not the release stating it.
    let saying: Vec<&&str> = rows
        .iter()
        .filter(|row| row.to_lowercase().contains("biome"))
        .collect();
    assert!(
        saying.is_empty(),
        "the release has no Biomes table and {} of its rows still say `biome`: {saying:?}",
        saying.len()
    );

    // **Force is the other half and it went with the same promotion.** Nothing produces or
    // consumes one, which is what made *holding a territory* unaskable in this release.
    let forceful: Vec<&&str> = rows
        .iter()
        .filter(|row| {
            let cells: Vec<&str> = row.trim_matches('|').split('|').map(str::trim).collect();
            cells.get(4).is_some_and(|kind| *kind == "force")
        })
        .collect();
    assert!(
        forceful.is_empty(),
        "{} recipe rows name `force` as a kind and `P-522` cut it: {forceful:?}",
        forceful.len()
    );
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
