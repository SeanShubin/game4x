//! Binds the globe to the one game: the counter it watches, and the keys that type a line.
//!
//! `planet-bevy` draws a planet. This makes that planet be *the planet being played* — and
//! it is a separate crate for one reason, which is worth stating because the reason is the
//! whole of it:
//!
//! **A prototype about polyhedra was linking the command language in order to draw a
//! sphere.** `planet-bevy` named `game-front`, so everything that drew a globe got the
//! console, the grammar and the game with it. `GlobePlugin::detached` removed the systems
//! and could not remove the dependency, because the code was still in the crate.
//!
//! Now the direction is the other way round. `planet-bevy` knows nothing about a game;
//! this crate knows about both and joins them, which is what a binding is. A prototype
//! does not have to touch the game code — but the fact that it *could* is what keeps the
//! boundaries honest, and a dependency edge is the only part of that a compiler can check.
//!
//! # Why a counter, four times over
//!
//! The one `Session` lives outside the engine, and on the web it is not even on the same
//! call stack, because the page calls into it. So it cannot hand anything over when
//! something changes. Every system here watches a number that only goes up and acts when
//! the number it last saw is not the number it sees now.
//!
//! Four of them: the game changed, the view was reset, the drawing was switched, and a
//! size was chosen. The first three are asked for from the page; the fourth types a line.

use bevy::prelude::*;
use planet_bevy::globe::{DecidesWhatToDraw, Drawing, FollowsTheGame, Orbit, Planet};
use planet_model::PlanetSize;

/// Makes a globe follow the one game.
///
/// Added beside [`planet_bevy::globe::GlobePlugin`], never instead of it. It sets
/// [`FollowsTheGame`], which is what puts the game's own bindings in the readout — so the
/// systems and the advertisement of them arrive together and cannot disagree. A globe used
/// to advertise five keys that started no game, on the first screen of a prototype,
/// because those were two separate things to remember.
pub struct FollowsTheGamePlugin;

impl Plugin for FollowsTheGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FollowsTheGame(true))
            .insert_resource(Followed::default())
            .insert_resource(ResetsSeen::default())
            .insert_resource(DrawingAsksSeen::default())
            .add_systems(
                Update,
                (
                    follow_the_game,
                    keys_to_choose_size,
                    a_control_asks_for_a_reset,
                    a_control_asks_to_change_the_drawing,
                )
                    .chain()
                    .in_set(DecidesWhatToDraw),
            );
    }
}

/// The generation the globe last redrew for.
#[derive(Resource, Default)]
struct Followed(u64);

/// The globe follows the game.
///
/// What the globe draws is whichever game the front end is holding, and that changes in two
/// ways: a transition, and `/new <size>` putting a different game there entirely. The
/// counter moves for both, because both leave the same amount to redraw. Number keys used
/// to set the size directly, which let the view hold a world the model did not have; the
/// view is a projection of the model, so that had to go rather than be kept as a
/// convenience.
fn follow_the_game(mut followed: ResMut<Followed>, mut planet: ResMut<Planet>) {
    let generation = game_front::shell::generation();
    if generation == followed.0 {
        return;
    }
    followed.0 = generation;
    // No planet yet is not an error. A game begins with nothing in it and is designed into
    // existence, so this is what the first few commands of any game look like.
    let Some(count) = game_front::shell::territory_count() else {
        return;
    };
    // Only write when it would change something. Touching a `ResMut` marks it changed, and
    // the globe rebuilds the whole world when it sees that.
    if planet.regions() != count {
        planet.show(count);
    }
}

/// The digits that choose a size, smallest to largest.
const SIZE_KEYS: [KeyCode; 5] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
];

/// Number keys choose a planet size, smallest to largest.
///
/// **By typing the line, not by writing the size.** `releases/first-release.md` says
/// *choosing a planet size abandons the current game and starts one on a planet of that
/// size*, and the way to say that is `/new <size>`. A key that set the planet directly
/// would let the view hold a planet the model does not have, which is what these keys used
/// to do before the globe followed the game.
///
/// So a key and a typed line take the same path, and the globe learns about the result the
/// same way either way: through [`follow_the_game`], watching the counter.
fn keys_to_choose_size(keys: Res<ButtonInput<KeyCode>>) {
    for (digit, size) in SIZE_KEYS.into_iter().zip(PlanetSize::ALL) {
        if keys.just_pressed(digit) {
            game_front::shell::with(|console| console.submit(&chooses(size)));
        }
    }
}

/// The line a size key types. Exactly what a person would type, because it is the same
/// thing arriving by a different route.
///
/// It is `/new <size>` rather than `create planet <size>` because the second is available
/// only before `start`, and the shipped build opens on a game already under way - so every
/// size key would have been refused, correctly and uselessly.
fn chooses(size: PlanetSize) -> String {
    format!("/new {}", size.name())
}

/// How many resets the view had been asked for when it last obeyed.
#[derive(Resource, Default)]
struct ResetsSeen(u64);

/// A control on the page asks for the view to be reset.
///
/// The `R` key is `planet-bevy`'s, and always was - a globe with no game behind it still
/// needs a way back, because a sphere has no edge to bump into. This is the other half:
/// `spec/interface.md` says actions like this *never require a gesture or a key the
/// platform may lack*, and a tablet lacks every key, so the control has to reach the same
/// place. It does it through a counter, because a button on a page is not on the engine's
/// call stack.
fn a_control_asks_for_a_reset(mut asked: ResMut<ResetsSeen>, mut orbit: ResMut<Orbit>) {
    let requested = game_front::shell::resets();
    if requested != asked.0 {
        asked.0 = requested;
        *orbit = Orbit::default();
    }
}

/// How many drawing changes had been asked for when the drawing last changed.
#[derive(Resource, Default)]
struct DrawingAsksSeen(u64);

/// A control on the page asks for the other drawing. The `T` key is the other half, and is
/// `planet-bevy`'s, for the same reason the `R` key is.
fn a_control_asks_to_change_the_drawing(
    mut asked: ResMut<DrawingAsksSeen>,
    mut drawing: ResMut<Drawing>,
) {
    let requested = game_front::shell::drawing_changes();
    if requested != asked.0 {
        asked.0 = requested;
        *drawing = drawing.other();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One key per size, in the order the sizes are listed, and each types the line a person
    /// would type. `game-front`'s `starting_over_gives_a_planet_of_the_size_asked_for` is the
    /// other half: that each of those lines does what it says.
    #[test]
    fn the_size_keys_are_one_per_size_smallest_to_largest() {
        assert_eq!(SIZE_KEYS.len(), PlanetSize::ALL.len());
        let typed: Vec<String> = PlanetSize::ALL.into_iter().map(chooses).collect();
        assert_eq!(
            typed,
            [
                "/new tiny",
                "/new small",
                "/new medium",
                "/new large",
                "/new huge",
            ]
        );
        // Ascending, so the digits read the way they look on the keyboard.
        let counts: Vec<usize> = PlanetSize::ALL
            .into_iter()
            .map(PlanetSize::territory_count)
            .collect();
        assert!(
            counts.windows(2).all(|pair| pair[0] < pair[1]),
            "{counts:?}"
        );
    }
}
