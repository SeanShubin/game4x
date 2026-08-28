//! The composition root for game4x.
//!
//! This module contains no logic. Its whole job is to decide what to build and to wire
//! the pieces together:
//!
//! ```text
//!   command-language  +  game-model                the game, pure
//!            \              /
//!             game-console                         the only door into it
//!                   |
//!              game-front                          the console and browser, no engine
//!                   |
//!   sphere-tessellation  +  graph-coloring         the geometry, pure
//!            \                  /
//!             planet-render                        a view model, no engine
//!                   |
//!              planet-bevy                         window, input, presentation
//!                   |
//!              this module                         wiring, and nothing else
//! ```
//!
//! It is the only place that knows both that Bevy exists and that the game exists at the
//! same time. See `docs/architecture.md`.
//!
//! The same binary runs natively and in a browser, and the *game* is identical in both
//! because nothing below the engine layer knows there is an engine. What differs is how
//! the console and the data browser are reached, which `spec/interface.md` allows: how a
//! thing is presented, and how the user acts on it, may follow the platform, while what
//! the user can do stays the same.
//!
//! | Surface      | Desktop            | Web                                     |
//! | ------------ | ------------------ | --------------------------------------- |
//! | The game     | a window           | a canvas                                |
//! | The console  | stdin and stdout   | a text field and a transcript, on the page |
//! | The browser  | `/browser`         | a panel, reached by its own button      |

use bevy::prelude::*;
use bevy::window::PresentMode;
use planet_render::{Params, WorldSpec};

/// The planet to fall back on if the game has not made one yet.
///
/// It normally has: the console builds the release's world as it opens, so by the time
/// there is a frame to draw there are twelve territories to draw. This is what the view
/// would show for a game designed no further than `create planet`, and 92 rather than 12
/// so that a blank one is obviously a blank one.
const UNDESIGNED: usize = 92;

fn main() {
    // What world to draw is a question for the game, not a constant here. The console is
    // already open on the release's planet, having built it out of `commands/setup.4x`
    // the only way a world can be built - by running commands.
    let territories = game_front::shell::territory_count().unwrap_or(UNDESIGNED);
    let spec = WorldSpec {
        params: Params {
            region_count: territories,
            ..Params::default()
        },
        // Ask for the world to be built from the region count. At a Goldberg count this
        // constructs the exact solid and relaxes nothing.
        soccer: false,
    };
    let topology = planet_render::topology_of(spec);

    // The console, on whatever this platform offers. On the desktop it reads stdin, which
    // needs a thread of its own because the engine's event loop never returns. On the web
    // the page calls in through `#[wasm_bindgen]` and there is nothing to start.
    #[cfg(not(target_arch = "wasm32"))]
    std::thread::spawn(game_front::shell::terminal::serve);

    App::new()
        .add_plugins(DefaultPlugins.set(window()))
        // Game entities and the turn: ECS, no rendering, no rules.
        .add_plugins(planet_ecs::PlanetEcsPlugin::new(topology))
        // The solid, a camera, and the pointer: the only place an engine's opinions land.
        // It follows the one Session by watching a counter; it never reaches into it.
        .add_plugins(planet_bevy::globe::GlobePlugin::new(spec))
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
