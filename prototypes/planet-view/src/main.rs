//! The composition root for the planet view prototype.
//!
//! This module contains no logic. Its whole job is to decide what to build and to wire
//! the pieces together:
//!
//! ```text
//!   sphere-tessellation  +  graph-coloring     the model, pure
//!            \                  /
//!             planet-render                    pixels, no engine
//!                   |
//!              planet-bevy                     window, input, vsync
//!                   |
//!              this module                     wiring, and nothing else
//! ```
//!
//! It is also the only place that knows both that Bevy exists and that the planet
//! exists at the same time. See `docs/architecture.md`.

mod capture;
mod options;
mod photograph;

use bevy::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let options = options::parse()?;
    match options.capture.clone() {
        // Headless: no window, no engine, straight to a PNG.
        Some(path) => capture::write_frame(&options, &path),
        None => {
            run(&options);
            Ok(())
        }
    }
}

fn run(options: &options::Options) {
    // The one place that knows every layer exists at once. It builds the world, hands
    // the integer graph to the ECS layer and the whole thing to the view layer, and
    // holds no logic of its own.
    let spec = options.spec();
    let topology = planet_render::topology_of(spec);

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(planet_bevy::window_plugin(
        options.width as u32,
        options.height as u32,
    )))
    // Game entities and the turn: ECS, no rendering, no rules.
    .add_plugins(planet_ecs::PlanetEcsPlugin::new(topology))
    // Window, input, and presentation: the only place an engine's opinions land.
    .add_plugins(planet_flat::PlanetViewPlugin::new(spec))
    .insert_resource(options.renderer);
    if let Some(errand) = shutter(options) {
        app.add_plugins(errand);
    }
    app.run();
}

/// The photograph errand, or nothing at all when none was asked for.
///
/// The shader draws to a frame and not to an array, so photographing it means opening a
/// window and screenshotting - which is why this is a plugin here rather than a headless
/// path beside `--capture`. See [`photograph`].
fn shutter(options: &options::Options) -> Option<photograph::PhotographPlugin> {
    options
        .shot
        .clone()
        .map(|path| photograph::PhotographPlugin {
            path,
            settle: options.settle,
        })
}
