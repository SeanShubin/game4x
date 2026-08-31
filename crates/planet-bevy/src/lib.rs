//! The Bevy adapter: a window, input, and vsync presentation.
//!
//! It owns no logic of its own — it maps Bevy's events onto plain methods and hands over
//! what comes back. Everything interesting happens on the other side of that boundary, in
//! crates that can be driven and tested with no engine at all.
//!
//! The reason for the split is not tidiness. It is that the engine is the part most
//! likely to be replaced: this project already replaced minifb with Bevy once, and the
//! model, camera and rasterizer did not change a line.
//!
//! What is here is [`globe`], the sphere you turn in your hands, and the window every
//! composition root asks for. The flat projection used to be here too and is
//! `planet-flat` now: it was a second, unrelated adapter that the game never used, and
//! every binary was paying for both.
//!
//! # Why Bevy rather than a framebuffer
//!
//! minifb has no vsync — none, it is not a setting — so it blits whenever asked and
//! the blit lands mid-scanout, which tears. Bevy presents through wgpu with
//! [`PresentMode::AutoVsync`], so frames are swapped during the vertical blank and
//! there is no seam.

pub mod globe;

use bevy::prelude::*;
use bevy::window::PresentMode;

/// A window configured the way this project wants it, vsync included.
///
/// Handed to the composition root rather than applied here, so that assembling the app
/// stays visible in one place.
pub fn window_plugin(width: u32, height: u32) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "planet view".to_string(),
            resolution: (width, height).into(),
            // The whole reason for moving off minifb.
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }
}
