//! Operating the application by remote control: drive it, photograph it, dump it.
//!
//! # Why this is a crate rather than a module of the root
//!
//! **`docs/architecture.md` rule 4**: the composition root *may assemble plugins, but it may
//! not compute with engine types*, and rule 6 says it twice - an algorithm *never names
//! `Entity`, `Query`, `Commands` or `Res`*. This file takes `Res<Errand>` and writes through
//! `ResMut<Orbit>`, which is computing with them.
//!
//! **It was `crates/game4x/src/inspect.rs` until `S-160`**, where nothing was red and the rule
//! had been written down and unheld for as long as it had existed. Moving it makes it an
//! engine adapter, which is the layer allowed to name those types, and leaves the root
//! assembling.
//!
//! **Nothing about what it does changed**, which is the property its own header rests on:
//! *nothing here is compiled differently from what ships - the same binary plays and poses*.
//! A crate boundary keeps that true where a rewrite would not.
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

use std::sync::Arc;

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use game_front::game_state::Drives;

pub mod options;

pub use options::{Misuse, Options, USAGE, read};

/// Drives the application from the command line and writes what it finds.
pub struct InspectPlugin {
    pub options: Options,
    /// The game, as the one surface this crate needs - `S-227`.
    ///
    /// **Handed down by the composition root rather than reached for.** This is the wider of
    /// the two surfaces `C-188` measured, and it is the harness - so the crate that reads the
    /// game rather than watching it is the one nothing a player runs depends on.
    pub game: Arc<dyn Drives>,
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
    game: Arc<dyn Drives>,
    frames: u32,
    asked: bool,
    waited: u32,
}

impl Plugin for InspectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Errand {
            options: self.options.clone(),
            game: Arc::clone(&self.game),
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
            let said = errand.game.submit(&line);
            let last = said.lines().last().unwrap_or_default().to_string();
            info!("ran `{line}`: {last}");
        }
        if errand.options.realistic {
            errand.game.change_drawing();
        }
    }

    if !errand.options.is_errand() || errand.frames <= errand.options.settle {
        return;
    }

    if !errand.asked {
        errand.asked = true;
        if let Some(dump) = errand.options.dump.clone() {
            let text = describe(*drawn, errand.game.as_ref());
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
fn describe(drawn: planet_bevy::globe::Drawn, game: &dyn Drives) -> String {
    let mut lines = vec![
        format!("drawing: {}", drawn.drawing.name()),
        String::new(),
        "-- the game, as the console reports it --".to_string(),
        // **A non-answer is written as one** - `Q-111`. `says` hands back the reason rather
        // than a variant name, so this line cannot read as the planet when there was no planet
        // to report. The marker is what a reader of a dump scans for.
        match game.says("{show-planet}") {
            Ok(said) => said,
            Err(why) => format!("!! the console did not answer {{show-planet}}: {why}"),
        },
        String::new(),
        "-- every entity --".to_string(),
        game.browser(),
        String::new(),
        "-- what the engine was given --".to_string(),
    ];

    lines.push(format!(
        "regions {}  vertices {}  triangles {}  labels on the sphere {}",
        drawn.regions, drawn.vertices, drawn.triangles, drawn.labels
    ));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A game that answers nothing, which is the case the dump has to survive.
    struct AnswersNothing;

    impl Drives for AnswersNothing {
        fn submit(&self, _: &str) -> String {
            String::new()
        }
        fn change_drawing(&self) {}
        fn browser(&self) -> String {
            "-- no entities --".to_string()
        }
        fn says(&self, _: &str) -> Result<String, String> {
            Err("the command changed the game and answered nothing".to_string())
        }
    }

    /// **A non-answer does not read as content in the dump a person vets.**
    ///
    /// `Q-111` is about this line and nothing else: the dump is evidence Sean reads for `D-5`, and
    /// a variant name under *the game, as the console reports it* is plausible English in the
    /// place a planet belongs.
    ///
    /// # Why this is here and not beside `says`
    ///
    /// **The quality lens placed it after closing `Q-111`**: `says_names_every_outcome_rather_than
    /// _falling_back` lives in `game-front`, where the method is, and is sound about that file -
    /// but **the property was never *`says` is honest*, it was *a non-answer does not read as
    /// content in the dump***. That rested on one unchecked `format!` here, and changing it to
    /// `Err(why) => why` would restore the original defect with the other guard still green.
    ///
    /// **A check placed where the code is rather than where the property is.** The lens found the
    /// same shape twice in one day across two lanes - `Q-110` puts a fake in the crate defining a
    /// trait rather than the crate whose systems were the reason for it - and in both cases the
    /// check is correct about its own file, which is why neither looks wrong when read.
    #[test]
    fn a_dump_says_so_when_the_console_answered_nothing() {
        let drawn = planet_bevy::globe::Drawn {
            drawing: planet_bevy::globe::Drawing::Practical,
            regions: 12,
            vertices: 0,
            triangles: 0,
            labels: 12,
        };
        let said = describe(drawn, &AnswersNothing);

        // **The reason is there and so is a marker that is not English.** Either alone is not
        // enough: the reason alone reads as a sentence the console might have said, and a marker
        // with no reason sends a reader back to the code.
        assert!(
            said.contains("!!"),
            "nothing in the dump marks the non-answer:\n{said}"
        );
        assert!(
            said.contains("answered nothing"),
            "the dump does not say why:\n{said}"
        );
        // **And the line is where a reader looks for the planet**, which is the whole hazard.
        let at = said
            .find("-- the game, as the console reports it --")
            .expect("the heading is what makes the next line read as a reading");
        assert!(
            said[at..].contains("!!"),
            "the marker is not under the heading it has to be under:\n{said}"
        );
    }
}
