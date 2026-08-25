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

    App::new()
        .add_plugins(DefaultPlugins.set(planet_bevy::window_plugin(
            options.width as u32,
            options.height as u32,
        )))
        // Game entities and the turn: ECS, no rendering, no rules.
        .add_plugins(planet_ecs::PlanetEcsPlugin::new(topology))
        // Window, input, and presentation: the only place an engine's opinions land.
        .add_plugins(planet_bevy::PlanetViewPlugin::new(spec))
        .run();
}
