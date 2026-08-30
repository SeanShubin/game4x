//! How the planet is shown and steered, with no engine underneath it.
//!
//! This is the half of the presentation that is *policy*: where the camera is, what a
//! hand on the glass is asking for, which ink an id needs to stay legible, what the
//! heads-up line says. None of it needs a window, a device or a frame.
//!
//! # Why it is a crate rather than a module
//!
//! It used to live in `planet-bevy`, and every test in it was pure and none of them ran
//! before deploy. The gate tests the engine-free crates and leaves the engine-facing ones
//! to a job that runs *after* the page is published - so a regression in the pinch floor,
//! or in the ink, would have been announced by a green deployment and a red notification
//! arriving in that order. One of those tests guards a bug that had already shipped once.
//!
//! Moving the policy is what fixes that, rather than widening the gate. A rule that can be
//! checked without an engine should not sit where checking it needs one.
//!
//! # The line between here and the engine
//!
//! Here: what a step of drag means, what two fingers mean, where the limits are, what the
//! text says. There: what a `Vec2` is, what a `Color` is, which system runs when, and how
//! a resource is stored. `planet-bevy` wraps each of these in a newtype so Bevy can hold
//! it, and converts at that seam and nowhere else.

use planet_render::{World, mesh};

/// A step across the glass, in pixels.
///
/// Its own type rather than the engine's vector, because taking the engine's would put the
/// engine back underneath everything here. Two operations are wanted and both are here.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Step {
    pub x: f32,
    pub y: f32,
}

impl Step {
    pub const ZERO: Self = Self::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// How far apart two points are. Used for one thing: the separation of a pair of
    /// fingers, which is what a pinch is measured from.
    pub fn distance(self, other: Self) -> f32 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

impl std::ops::Add for Step {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Step {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

/// How far back the camera sits at rest, in sphere radii.
///
/// Far enough that there is sky above the north pole. At 3.1 the planet very nearly filled
/// the window, and the pole's letter - which floats above the spike, further out than any
/// part of the planet - projected past the top edge and was simply not on the screen.
/// Anything drawn beyond the surface needs room beyond the surface to be drawn in.
pub const RESTING_DISTANCE: f32 = 3.45;
pub const CLOSEST: f32 = 1.35;
pub const FURTHEST: f32 = 9.0;

/// The tilt the view opens on, and returns to when reset. Enough to show that the world is
/// a ball rather than a disc, and to bring the north pole into view without hiding it.
pub const RESTING_PITCH: f32 = 0.35;

/// Radians of turn per pixel of drag. Slow enough to place a region deliberately.
pub const DRAG_SENSITIVITY: f32 = 0.006;

/// How far the pitch may travel before it would tip past the pole and invert the world.
///
/// The margin is a hundredth of a radian, a little over half a degree, and it exists
/// because the axis degenerates exactly at the pole: yaw and roll stop being separable
/// there. You can still get near enough to be looking straight down at a pole.
pub const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;

/// Where the viewer is: two angles and a distance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Orbit {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: RESTING_PITCH,
            distance: RESTING_DISTANCE,
        }
    }
}

impl Orbit {
    /// Turn by an angle. The pitch is clamped short of the pole, where the axis
    /// degenerates and yaw and roll stop being separable.
    ///
    /// There is no argument for roll, and that is the point: `spec/planet.md` says the
    /// roll for any point on the planet is fixed and nothing the user does changes it, so
    /// there is nothing for a gesture to reach even if one offered it.
    pub fn turn(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch = (self.pitch + pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    /// Turn by a step across the glass, in pixels.
    ///
    /// A mouse drag, a held arrow key and a finger are three ways of producing this step
    /// and are not three features. `spec/interface.md` says *how a thing is presented, and
    /// how the user acts on it, may follow the platform it runs on* while what the user can
    /// do stays the same - so the device decides only how the step is produced.
    pub fn drag(&mut self, step: Step) {
        self.turn(step.x * DRAG_SENSITIVITY, step.y * DRAG_SENSITIVITY);
    }

    /// Move the viewer nearer or further by a factor of the distance remaining, so the same
    /// gesture covers the same proportion whether you are close in or far out.
    pub fn scale_distance(&mut self, factor: f32) {
        self.distance = (self.distance * factor).clamp(CLOSEST, FURTHEST);
    }
}

/// The smallest separation, in pixels, that a pinch is measured against.
///
/// Two fingers that land almost on top of each other give a ratio with a very small number
/// underneath it, and one further pixel of separation would then read as an enormous zoom.
/// Below this the pair is held but not acted on.
pub const PINCH_FLOOR: f32 = 24.0;

/// What a change in the fingers on the glass asks the view to do.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Gesture {
    /// Turn the world by a step in pixels - the same step a mouse drag produces, and handed
    /// to the same method.
    Turn(Step),
    /// Zoom, by the factor the fingers' separation changed by. Greater than one is fingers
    /// spreading apart.
    Pinch(f32),
}

/// The fingers on the glass, and where each of them was last seen.
///
/// Touch is tracked here rather than read from the engine's own touch state, because that
/// state only refreshes the previous position on frames that carried an event: with a
/// finger held still, its delta keeps reporting the last movement and the world would
/// drift on. Keeping the previous position here is the same shape as the pointer's drag,
/// which reads cursor positions for the same reason.
///
/// At most two are tracked. A third finger is ignored rather than queued, because there is
/// nothing for it to mean.
#[derive(Default)]
pub struct Fingers {
    down: Vec<(u64, Step)>,
}

impl Fingers {
    /// How many are being tracked, which is never more than two.
    pub fn count(&self) -> usize {
        self.down.len()
    }

    /// The separation between the pair, or `None` unless there are exactly two.
    fn gap(&self) -> Option<f32> {
        match self.down.as_slice() {
            [(_, first), (_, second)] => Some(first.distance(*second)),
            _ => None,
        }
    }

    pub fn began(&mut self, id: u64, at: Step) {
        if self.down.iter().any(|(known, _)| *known == id) {
            return;
        }
        if self.down.len() < 2 {
            self.down.push((id, at));
        }
    }

    pub fn ended(&mut self, id: u64) {
        self.down.retain(|(known, _)| *known != id);
    }

    /// Moves one finger and says what the hand as a whole is now asking for.
    ///
    /// One finger turns the world. Two zoom it, and **only** zoom it: the angle between
    /// them is computed nowhere, so a two-finger twist - which every gesture library offers
    /// for free - cannot reach anything. `spec/planet.md` fixes the roll for any point on
    /// the planet and says nothing the user does changes it, so a twist has to be discarded
    /// on purpose rather than left to be wired up by accident.
    pub fn moved(&mut self, id: u64, to: Step) -> Option<Gesture> {
        let index = self.down.iter().position(|(known, _)| *known == id)?;
        let separated = self.gap();
        let from = std::mem::replace(&mut self.down[index].1, to);
        match (separated, self.gap()) {
            (Some(before), Some(after)) if before >= PINCH_FLOOR => {
                Some(Gesture::Pinch(after / before))
            }
            (None, None) => Some(Gesture::Turn(to - from)),
            _ => None,
        }
    }
}

/// The two inks an id is ever drawn in, as sRGB.
pub const DARK_INK: [f32; 3] = [0.05, 0.06, 0.08];
pub const LIGHT_INK: [f32; 3] = [0.93, 0.95, 0.98];

/// Relative luminance of a linear colour, by the usual coefficients.
pub fn luminance(panel: [f32; 4]) -> f32 {
    0.2126 * panel[0] + 0.7152 * panel[1] + 0.0722 * panel[2]
}

/// The ink an id needs to stay legible on the panel it sits on.
///
/// The palette runs from pale sand to near-black navy, so any single ink is legible on some
/// territories and invisible on others - which was exactly what happened to the ids on the
/// darker panels. Relative luminance decides between the two. The panel colours are already
/// linear, which is the space those coefficients are defined in.
pub fn readable_on(panel: [f32; 4]) -> [f32; 3] {
    if luminance(panel) > 0.18 {
        DARK_INK
    } else {
        LIGHT_INK
    }
}

/// The heads-up line: what is being drawn, and what will move it.
///
/// `other_drawing` names the drawing the `T` key would switch to, and is `None` when there
/// is no game behind this globe. The bindings are listed from the same fact that installs
/// them, so the two cannot disagree - a detached globe used to advertise five keys that
/// started no game and a `T` that changed no drawing, on the first screen of the newest
/// prototype.
pub fn summary(world: &World, panels: &mesh::PlanetMesh, other_drawing: Option<&str>) -> String {
    let regions = world.tessellation.region_count();
    let shape = match sphere_tessellation::goldberg::arrangements_up_to(regions)
        .into_iter()
        .find(|&(m, n)| sphere_tessellation::goldberg::region_count(m, n) == regions)
    {
        Some((m, n)) => format!("GP({m},{n})"),
        None => "no Goldberg solid at this count".to_string(),
    };
    let bindings = match other_drawing {
        Some(other) => {
            format!("\n1-5 start a new game on a planet of that size - T for the {other} drawing")
        }
        None => String::new(),
    };
    // No name for the planet. What a count is *called* is the game's vocabulary, and this
    // draws solids the game has no word for; the count and the arrangement say more than a
    // name could, and saying neither keeps `PlanetSize` out of the drawing layer entirely.
    format!(
        "{regions} territories - {shape}\n{} - {} triangles\n\
         drag, a finger or the arrows to turn - wheel or pinch to zoom - R to reset{bindings}",
        world.degree_summary(),
        panels.triangle_count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resting() -> Orbit {
        Orbit::default()
    }

    /// Ink has to change with the panel under it, or the ids vanish on half the palette.
    #[test]
    fn ids_are_inked_against_the_panel_they_sit_on() {
        let dark = readable_on([0.01, 0.02, 0.06, 1.0]);
        let light = readable_on([0.75, 0.72, 0.5, 1.0]);
        assert_ne!(dark, light, "one ink cannot serve both");
        // Every colour in the real palette must get an ink that contrasts with it.
        for packed in planet_render::palette::REGION_COLORS {
            let panel = mesh::linear_rgba(packed);
            let ink = readable_on(panel);
            assert_eq!(
                ink == LIGHT_INK,
                luminance(panel) <= 0.18,
                "colour {packed:#08x} at luminance {} got the wrong ink",
                luminance(panel)
            );
        }
    }

    /// A finger is a drag. Not a similar thing that happens to look like one - the same
    /// step, through the same method, to the same result.
    #[test]
    fn a_finger_turns_the_world_exactly_as_a_mouse_drag_does() {
        let step = Step::new(37.0, -14.0);

        let mut dragged = resting();
        dragged.drag(step);

        let mut touched = resting();
        let mut fingers = Fingers::default();
        fingers.began(1, Step::new(100.0, 100.0));
        let gesture = fingers.moved(1, Step::new(100.0, 100.0) + step);
        assert_eq!(gesture, Some(Gesture::Turn(step)));
        touched.drag(step);

        assert_eq!(touched.yaw, dragged.yaw);
        assert_eq!(touched.pitch, dragged.pitch);
    }

    /// The first position after a finger lands only says where it landed. Turning on it
    /// would jerk the world by wherever the finger happened to be, which is the same bug
    /// the pointer's drag avoids.
    #[test]
    fn a_finger_landing_turns_nothing() {
        let mut fingers = Fingers::default();
        fingers.began(1, Step::new(400.0, 300.0));
        assert_eq!(
            fingers.moved(1, Step::new(400.0, 300.0)),
            Some(Gesture::Turn(Step::ZERO))
        );
        let mut orbit = resting();
        orbit.drag(Step::ZERO);
        assert_eq!(orbit.yaw, resting().yaw);
    }

    /// Fingers spreading apart bring the world closer, and pinching together push it away.
    #[test]
    fn spreading_two_fingers_zooms_in_and_pinching_zooms_out() {
        let mut fingers = Fingers::default();
        fingers.began(1, Step::new(300.0, 400.0));
        fingers.began(2, Step::new(500.0, 400.0));

        let Some(Gesture::Pinch(ratio)) = fingers.moved(2, Step::new(700.0, 400.0)) else {
            panic!("moving one of a pair is a pinch");
        };
        assert!(
            ratio > 1.0,
            "spreading apart is a ratio above one, got {ratio}"
        );
        let mut orbit = resting();
        orbit.scale_distance(1.0 / ratio);
        assert!(
            orbit.distance < RESTING_DISTANCE,
            "spreading did not come closer"
        );

        let Some(Gesture::Pinch(ratio)) = fingers.moved(2, Step::new(400.0, 400.0)) else {
            panic!("moving one of a pair is a pinch");
        };
        assert!(
            ratio < 1.0,
            "closing together is a ratio below one, got {ratio}"
        );
        let mut orbit = resting();
        orbit.scale_distance(1.0 / ratio);
        assert!(
            orbit.distance > RESTING_DISTANCE,
            "pinching did not go further out"
        );
    }

    /// `spec/planet.md` says *the roll for any point on the planet is fixed, and nothing
    /// the user does changes it.*
    ///
    /// A two-finger twist is the gesture that would reach roll, and it comes free with any
    /// pinch. Rotating the pair about its own centre must therefore leave the view exactly
    /// where it was - not approximately, and not only for the yaw.
    #[test]
    fn twisting_two_fingers_changes_nothing() {
        let centre = Step::new(400.0, 400.0);
        let arm = 150.0;
        let mut fingers = Fingers::default();
        fingers.began(1, centre + Step::new(arm, 0.0));
        fingers.began(2, centre - Step::new(arm, 0.0));

        let mut orbit = resting();
        let before = (orbit.yaw, orbit.pitch, orbit.distance);
        for step in 1..=32 {
            let angle = step as f32 * std::f32::consts::TAU / 32.0;
            let offset = Step::new(arm * angle.cos(), arm * angle.sin());
            for (id, at) in [(1, centre + offset), (2, centre - offset)] {
                match fingers.moved(id, at) {
                    Some(Gesture::Pinch(ratio)) => orbit.scale_distance(1.0 / ratio),
                    Some(Gesture::Turn(step)) => orbit.drag(step),
                    None => {}
                }
            }
        }
        // The separation never changed, so every ratio was one and nothing moved.
        assert!(
            (orbit.yaw - before.0).abs() < 1e-4,
            "a twist turned the world"
        );
        assert!(
            (orbit.pitch - before.1).abs() < 1e-4,
            "a twist tilted the world"
        );
        assert!(
            (orbit.distance - before.2).abs() < 1e-3,
            "a twist zoomed the world"
        );
    }

    /// A second finger landing mid-drag must not be read as an enormous jump.
    #[test]
    fn a_second_finger_landing_does_not_lurch_the_world() {
        let mut fingers = Fingers::default();
        fingers.began(1, Step::new(100.0, 100.0));
        assert!(matches!(
            fingers.moved(1, Step::new(140.0, 100.0)),
            Some(Gesture::Turn(_))
        ));
        fingers.began(2, Step::new(600.0, 100.0));
        // Now a pair. Moving either one measures separation, never the distance from
        // wherever the other finger happens to be.
        assert!(matches!(
            fingers.moved(1, Step::new(150.0, 100.0)),
            Some(Gesture::Pinch(_))
        ));
    }

    /// Two fingers landing on the same spot would divide by something near zero.
    #[test]
    fn a_pinch_that_starts_too_close_together_is_not_acted_on() {
        let mut fingers = Fingers::default();
        fingers.began(1, Step::new(400.0, 400.0));
        fingers.began(2, Step::new(401.0, 400.0));
        assert_eq!(fingers.moved(2, Step::new(460.0, 400.0)), None);
        // Once they are far enough apart it measures normally again.
        assert!(matches!(
            fingers.moved(2, Step::new(500.0, 400.0)),
            Some(Gesture::Pinch(_))
        ));
    }

    /// Lifting one of a pair leaves the other turning the world, rather than leaving a
    /// stale finger behind that makes every later move read as a pinch.
    #[test]
    fn lifting_one_of_two_fingers_goes_back_to_turning() {
        let mut fingers = Fingers::default();
        fingers.began(1, Step::new(300.0, 400.0));
        fingers.began(2, Step::new(500.0, 400.0));
        fingers.ended(2);
        assert_eq!(
            fingers.moved(1, Step::new(320.0, 400.0)),
            Some(Gesture::Turn(Step::new(20.0, 0.0)))
        );
    }

    /// A third finger is ignored, and ignoring it must not disturb the two that are already
    /// doing something.
    #[test]
    fn a_third_finger_is_ignored() {
        let mut fingers = Fingers::default();
        fingers.began(1, Step::new(300.0, 400.0));
        fingers.began(2, Step::new(500.0, 400.0));
        fingers.began(3, Step::new(700.0, 400.0));
        assert_eq!(fingers.count(), 2);
        assert_eq!(fingers.moved(3, Step::new(900.0, 400.0)), None);
        assert!(matches!(
            fingers.moved(2, Step::new(600.0, 400.0)),
            Some(Gesture::Pinch(_))
        ));
    }

    /// The pitch is clamped whichever device asks for it, or the world inverts at the pole
    /// where yaw and roll stop being separable.
    #[test]
    fn no_device_can_tilt_the_world_past_the_pole() {
        let mut orbit = resting();
        for _ in 0..500 {
            orbit.drag(Step::new(0.0, 100.0));
        }
        assert!(orbit.pitch <= PITCH_LIMIT);
        for _ in 0..1000 {
            orbit.drag(Step::new(0.0, -100.0));
        }
        assert!(orbit.pitch >= -PITCH_LIMIT);
    }

    /// Zoom is clamped the same way whether it came from a wheel or from a pinch.
    #[test]
    fn no_device_can_zoom_past_the_limits() {
        let mut orbit = resting();
        for _ in 0..200 {
            orbit.scale_distance(0.5);
        }
        assert_eq!(orbit.distance, CLOSEST);
        for _ in 0..200 {
            orbit.scale_distance(2.0);
        }
        assert_eq!(orbit.distance, FURTHEST);
    }

    /// A globe with no game behind it advertises no key that would do nothing.
    ///
    /// The five size keys and `T` are installed only when a game is being followed, so a
    /// detached globe must not list them. It listed them for one release.
    #[test]
    fn a_detached_globe_advertises_no_binding_it_does_not_have() {
        let world = World::canonical(92).expect("92 is a Goldberg count");
        let solid =
            sphere_tessellation::solid(&world.tessellation.seeds, &world.tessellation.neighbours);
        let panels = mesh::build(&solid, &world.coloring);

        let detached = summary(&world, &panels, None);
        assert!(!detached.contains("1-5"), "{detached}");
        assert!(!detached.contains(" T "), "{detached}");
        assert!(detached.contains("92 territories"), "{detached}");

        let following = summary(&world, &panels, Some("realistic"));
        assert!(following.contains("1-5"), "{following}");
        assert!(
            following.contains("T for the realistic drawing"),
            "{following}"
        );
    }
}
