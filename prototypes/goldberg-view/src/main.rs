//! Prototype: the ten smallest Goldberg solids, one at a time, in the abstract drawing.
//!
//! **The question.** `spec/planet.md` allows only certain territory counts - `10T + 2` for
//! `T = m^2 + mn + n^2` - and the game has picked five of them. Which ones actually read as
//! a planet you could play on? Twelve territories is five neighbours everywhere and looks
//! like a die; two hundred and twelve is a lot of small cells. The answer is a judgement
//! about a picture, so the way to get it is to look at all ten in turn without rebuilding
//! anything in between.
//!
//! **Abstract, deliberately.** This shows the practical drawing only: flat colours, a
//! groove at every boundary, an id on every face. `spec/planet.md` says that drawing exists
//! to *make adjacency legible*, and adjacency is the whole subject here. The realistic
//! drawing would hide the very thing being compared under terrain.
//!
//! # What it borrows and what it does not
//!
//! It is a composition root, like the application, and it borrows the parts that draw:
//! `planet-render` builds the mesh, and `planet-bevy` gives the sphere, the camera and the
//! turning. It borrows nothing that plays: there is no game here, no console, no biome and
//! no terrain, which is why it asks for [`GlobePlugin::detached`].
//!
//! That is the whole test of whether the boundary is real. If a prototype about polyhedra
//! had to link the command language to draw a sphere, the layering would be a diagram
//! rather than a fact.

use bevy::prelude::*;
use bevy::window::PresentMode;
use planet_bevy::globe::{GlobePlugin, Planet};
use planet_render::{Params, WorldSpec};

/// How many Goldberg counts to offer.
const HOW_MANY: usize = 10;

/// Which one it opens on. `GP(1,0)` is the dodecahedron - the smallest planet the game
/// allows, and the one whose faces are large enough to see what a territory *is*.
const OPENS_ON: usize = 0;

fn main() {
    let counts = smallest_goldberg_counts(HOW_MANY);
    let spec = WorldSpec {
        params: Params {
            region_count: counts[OPENS_ON],
            ..Params::default()
        },
        soccer: false,
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window()))
        // Detached: the globe draws what this program tells it to, and never asks a game.
        .add_plugins(GlobePlugin::detached(spec))
        .insert_resource(Showing {
            counts,
            at: OPENS_ON,
        })
        .add_systems(Update, step_through_the_solids)
        .run();
}

/// Which of the ten is on screen.
#[derive(Resource)]
struct Showing {
    counts: Vec<usize>,
    at: usize,
}

/// `[` and `]` walk the list; the digits are free here because there is no game to start.
///
/// Wrapping rather than stopping at the ends, because the question is a comparison and the
/// comparison people actually make is between the two extremes.
fn step_through_the_solids(
    keys: Res<ButtonInput<KeyCode>>,
    mut showing: ResMut<Showing>,
    mut planet: ResMut<Planet>,
) {
    let last = showing.counts.len() - 1;
    let step = if keys.just_pressed(KeyCode::BracketRight) {
        1
    } else if keys.just_pressed(KeyCode::BracketLeft) {
        last
    } else {
        return;
    };
    showing.at = (showing.at + step) % showing.counts.len();
    let count = showing.counts[showing.at];
    // Only write when it would change something: touching the resource rebuilds the world.
    if planet.regions() != count {
        planet.show(count);
    }
}

/// The smallest region counts a Goldberg polyhedron can have, in order.
///
/// Not a table. `sphere-tessellation` already knows which `(m, n)` arrangements exist and
/// how many regions each gives, so the list is derived from the same place the solids are
/// built from - a written-down table could disagree with the geometry, and this cannot.
///
/// Several arrangements can give the same count once the numbers get larger, so the counts
/// are deduplicated: the question is about the shape of a planet, and two ways of reaching
/// the same face count are one planet size to look at.
fn smallest_goldberg_counts(how_many: usize) -> Vec<usize> {
    // Far enough up the list to be sure of finding that many distinct counts.
    let mut counts: Vec<usize> = sphere_tessellation::goldberg::arrangements_up_to(2000)
        .into_iter()
        .map(|(m, n)| sphere_tessellation::goldberg::region_count(m, n))
        .collect();
    counts.sort_unstable();
    counts.dedup();
    counts.truncate(how_many);
    counts
}

fn window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "goldberg view - [ and ] to change solid".to_string(),
            resolution: (1280, 800).into(),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ten smallest are the ones `spec/planet.md` describes: `10T + 2`, giving 12, 32,
    /// 42, 72, 92 and upward with nothing in between.
    #[test]
    fn the_ten_smallest_are_the_counts_the_specification_names() {
        let counts = smallest_goldberg_counts(HOW_MANY);
        assert_eq!(counts.len(), HOW_MANY);
        assert_eq!(&counts[..5], &[12, 32, 42, 72, 92]);
        // Every one is `10T + 2` for a whole `T`, and they are strictly increasing.
        for count in &counts {
            assert_eq!((count - 2) % 10, 0, "{count} is not 10T + 2");
        }
        assert!(
            counts.windows(2).all(|pair| pair[0] < pair[1]),
            "{counts:?}"
        );
    }

    /// The five the game has are all in the list, so the prototype covers what the game
    /// does as well as what it does not.
    #[test]
    fn every_planet_size_the_game_has_is_one_of_them() {
        let counts = smallest_goldberg_counts(HOW_MANY);
        for size in [12, 32, 42, 72, 92] {
            assert!(counts.contains(&size), "{size} is missing from {counts:?}");
        }
    }
}
