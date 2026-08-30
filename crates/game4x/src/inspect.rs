//! Operating the application by remote control: drive it, photograph it, dump it.
//!
//! Everything below the engine can be tested with no window open, which is what the
//! layering is for. The picture cannot, and half of `spec/planet.md` describes it: *the
//! terrain of the realistic drawing is continuous*, *nothing in the terrain reveals how the
//! sphere was divided*, *the two drawings share the camera and nothing else*. Those are
//! claims about pixels.
//!
//! So this plugin turns the application into something that can be asked a question and
//! made to answer with a file. Put the camera at a known place, choose a drawing, run some
//! commands, wait for the world to settle, then write a PNG and a text dump and quit.
//!
//! # It changes nothing about how the game works
//!
//! Every line from `--run` goes through the one console, exactly as typing it would.
//! Nothing here reaches into the model, and nothing here is compiled differently from what
//! ships - the same binary plays and poses. A harness that ran a special path would be
//! evidence about the harness.

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::options::Options;

/// Drives the application from the command line and writes what it finds.
pub struct InspectPlugin {
    pub options: Options,
}

/// How many frames to wait after asking for the screenshot before giving up on it.
///
/// The capture crosses to the render world and back, so it cannot be observed on the frame
/// it was asked for. This is a backstop so an errand always terminates: a run that never
/// captured should end and say so, not hang a build.
const PATIENCE: u32 = 240;

#[derive(Resource)]
struct Errand {
    options: Options,
    frames: u32,
    asked: bool,
    waited: u32,
}

impl Plugin for InspectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Errand {
            options: self.options.clone(),
            frames: 0,
            asked: false,
            waited: 0,
        })
        .add_systems(Startup, place_the_camera)
        .add_systems(Update, run_the_errand);
    }
}

/// Puts the camera where it was asked for, before the first frame is drawn.
///
/// `spec/planet.md` says the two drawings share the camera, so a screenshot of each from
/// the same numbers is the evidence for that - which only works if the numbers can be
/// stated rather than dragged to.
fn place_the_camera(errand: Res<Errand>, mut orbit: ResMut<planet_bevy::globe::Orbit>) {
    let options = &errand.options;
    if let Some(yaw) = options.yaw {
        orbit.yaw = yaw;
    }
    if let Some(pitch) = options.pitch {
        orbit.pitch = pitch;
    }
    if let Some(distance) = options.distance {
        orbit.distance = distance;
    }
}

fn run_the_errand(
    mut errand: ResMut<Errand>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    drawn: Res<planet_bevy::globe::Drawn>,
) {
    errand.frames += 1;

    // The console lines run on the first frame rather than before the app starts, so that
    // a rebuild of the globe sees them - the globe follows the game by watching a counter,
    // and nothing is watching before there is a frame.
    if errand.frames == 1 {
        for line in errand.options.run.clone() {
            let said = game_front::shell::submit(&line);
            let last = said.lines().last().unwrap_or_default().to_string();
            info!("ran `{line}`: {last}");
        }
        if errand.options.realistic {
            game_front::shell::change_drawing();
        }
    }

    if !errand.options.is_errand() || errand.frames <= errand.options.settle {
        return;
    }

    if !errand.asked {
        errand.asked = true;
        if let Some(dump) = errand.options.dump.clone() {
            let text = describe(*drawn);
            match std::fs::write(&dump, &text) {
                Ok(()) => info!("dumped to {dump}"),
                Err(why) => error!("cannot write {dump}: {why}"),
            }
        }
        match errand.options.shot.clone() {
            Some(path) => {
                commands
                    .spawn(Screenshot::primary_window())
                    .observe(save_to_disk(path));
            }
            // Nothing to photograph, so the errand is already done.
            None => {
                exit.write(AppExit::Success);
            }
        }
        return;
    }

    // `save_to_disk` writes on an observer, so the file appears a frame or two after the
    // request. Waiting a fixed few frames is enough and cannot deadlock.
    errand.waited += 1;
    if errand.waited > 8 || errand.waited > PATIENCE {
        exit.write(AppExit::Success);
    }
}

/// What is on screen, as text.
///
/// Two kinds of fact, and the split matters. The game is asked through
/// [`game_front`], so what comes back is what a player would be told by `show` - no
/// second opinion, no privileged access. The picture is measured from the mesh the engine
/// was actually given, because that is the only place those facts exist.
fn describe(drawn: planet_bevy::globe::Drawn) -> String {
    let mut lines = vec![
        format!("drawing: {}", drawn.drawing.name()),
        String::new(),
        "-- the game, as the console reports it --".to_string(),
        game_front::shell::with(|console| {
            console
                .session
                .run("show planet", &game_front::library())
                .map(|outcome| match outcome {
                    game_console::Outcome::Said(said) => said,
                    other => format!("{other:?}"),
                })
                .unwrap_or_else(|problem| problem.to_string())
        }),
        String::new(),
        "-- every entity --".to_string(),
        game_front::shell::browser(),
        String::new(),
        "-- what the engine was given --".to_string(),
    ];

    lines.push(format!(
        "regions {}  vertices {}  triangles {}  labels on the sphere {}",
        drawn.regions, drawn.vertices, drawn.triangles, drawn.labels
    ));
    lines.join("\n")
}
