//! The composition root for game4x.
//!
//! This module contains no logic. Its whole job is to decide what to build and to wire
//! the pieces together:
//!
//! ```text
//!   sphere-tessellation  +  graph-coloring     the model, pure
//!            \                  /
//!             planet-render                    a view model, no engine
//!                   |
//!              planet-bevy                     window, input, presentation
//!                   |
//!              this module                     wiring, and nothing else
//! ```
//!
//! It is the only place that knows both that Bevy exists and that the planet exists at
//! the same time. See `docs/architecture.md`.
//!
//! The same binary runs natively and in a browser. Nothing below the engine layer knows
//! which, because nothing below the engine layer knows there is an engine.

use bevy::prelude::*;
use bevy::window::PresentMode;
use planet_render::{Params, WorldSpec};

/// The world the application opens on.
///
/// Ninety-two regions is `GP(3,0)` - twelve pentagons at the icosahedron's vertices and
/// eighty hexagons between them, constructed rather than searched for. Region counts
/// that are not Goldberg numbers still work; they just have no perfect answer to be
/// built, so they fall back to relaxation. See `docs/theory/region-splitting.md`.
const REGIONS: usize = 92;

fn main() {
    let spec = WorldSpec {
        params: Params {
            region_count: REGIONS,
            ..Params::default()
        },
        // Ask for the world to be built from the region count, so that changing the
        // count above is all it takes. At a Goldberg count this constructs the exact
        // solid and relaxes nothing.
        soccer: false,
    };
    let topology = planet_render::topology_of(spec);

    App::new()
        .add_plugins(DefaultPlugins.set(window()))
        // Game entities and the turn: ECS, no rendering, no rules.
        .add_plugins(planet_ecs::PlanetEcsPlugin::new(topology))
        // The solid, a camera, and the pointer: the only place an engine's opinions land.
        .add_plugins(planet_bevy::globe::GlobePlugin::new(spec))
        // The three surfaces `spec/interface.md` asks for: the game, the console, and the
        // data browser. Added unconditionally, because nothing may be available in one
        // build and not another.
        .add_plugins(planet_bevy::surfaces::SurfacesPlugin)
        .run();
}

/// The window, described here rather than in the engine adapter so that assembling the
/// application stays visible in one place.
fn window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "game4x".to_string(),
            resolution: (1280, 800).into(),
            // Vsync. Tearing while turning the world was the reason for moving off a
            // hand-rolled framebuffer in the first place.
            present_mode: PresentMode::AutoVsync,
            // On the web the canvas is the whole page, so let it follow the element it
            // is placed in rather than keeping the fixed resolution above.
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }
}
