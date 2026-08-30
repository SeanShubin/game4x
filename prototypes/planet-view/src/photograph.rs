//! Photographing what the *engine* drew, rather than what the rasterizer computed.
//!
//! [`crate::capture`] writes a PNG with no window and no engine, which is the right tool
//! for the CPU rasterizer: it draws into an array of pixels, so the array is the picture.
//!
//! **The shader has no such array.** `planet.wgsl` runs on the GPU and its output exists
//! only as a rendered frame, so the only way to see it is to open a window, let it draw,
//! and screenshot the result. Without this the GPU path could not be checked at all -
//! `--capture` photographs `PlanetView::draw`, which is the path that does *not* have a
//! hand-transcribed copy of the palette in it.
//!
//! That is what this exists for. The palette lived twice, once in `planet-render` and once
//! as decimals in the shader, and deleting the copy is a change nobody could verify:
//! nothing could photograph the thing being changed. A renderer that quietly stops matching
//! itself is exactly the failure that duplication invites.
//!
//! # A prototype may take this shortcut
//!
//! `docs/prototypes/README.md` allows a prototype to do something the game may not,
//! provided the document says which and why. This one holds a harness in its composition
//! root: a plugin that counts frames, screenshots and quits. The game's root holds the same
//! shortcut for the same reason, and the reason is that a picture is the only evidence
//! about a picture.

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

/// How many frames to wait for the file to appear before quitting anyway.
const PATIENCE: u32 = 8;

/// Photograph the window after it has settled, then quit.
pub struct PhotographPlugin {
    pub path: String,
    /// Frames to let pass before the shutter. The first frame has no world in it yet.
    pub settle: u32,
}

impl Plugin for PhotographPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Errand {
            path: self.path.clone(),
            settle: self.settle,
            frames: 0,
            asked: false,
            waited: 0,
        })
        .add_systems(Update, take_the_photograph);
    }
}

#[derive(Resource)]
struct Errand {
    path: String,
    settle: u32,
    frames: u32,
    asked: bool,
    waited: u32,
}

fn take_the_photograph(
    mut errand: ResMut<Errand>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
) {
    errand.frames += 1;
    if errand.frames <= errand.settle {
        return;
    }
    if !errand.asked {
        errand.asked = true;
        let path = errand.path.clone();
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
        return;
    }
    // `save_to_disk` writes on an observer, so the file appears a frame or two after the
    // request. Waiting a fixed few frames is enough and cannot deadlock.
    errand.waited += 1;
    if errand.waited > PATIENCE {
        exit.write(AppExit::Success);
    }
}
