//! The planet view as a plain object, with no graphics engine anywhere in sight.
//!
//! This is the whole application: it owns the world and the camera, it accepts
//! [`Command`]s and pointer motion, and it draws itself into a slice of pixels. What
//! it deliberately does not know is how a window is opened, where input comes from, or
//! how those pixels reach a screen.
//!
//! That boundary is the point. An engine adapter maps its own events onto these
//! methods and presents the buffer; swapping engines means rewriting the adapter and
//! nothing else. It also means all of this is testable without opening a window, which
//! is why the tests below can drive a whole session and assert on pixels.

use crate::camera::{GlobeView, Projection};
use crate::raster;
use planet_render::world::{World, WorldSpec};

/// Zoom per wheel notch. Gentle on purpose: at 1.18 a single flick overshot badly.
pub const ZOOM_PER_NOTCH: f64 = 1.07;
/// Backends disagree about how much scroll one notch reports, so cap a single event
/// rather than trusting the number.
pub const MAX_NOTCHES_PER_EVENT: f64 = 4.0;

/// Region count step when the shift key is held.
pub const COARSE_REGION_STEP: usize = 10;

/// How fast the arrow keys turn the sphere, in screen pixels per second.
///
/// Expressed in pixels rather than radians so that it feels the same at every zoom:
/// dragging and arrowing move the surface at the same rate under the hand.
pub const KEY_TURN_PIXELS_PER_SECOND: f64 = 420.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub borders: bool,
    pub labels: bool,
    pub dim_duplicates: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            borders: true,
            labels: true,
            dim_duplicates: true,
        }
    }
}

/// Everything the view can be asked to do that is not pointer motion.
///
/// Named for intent rather than for a key, so that the adapter decides which key means
/// what and this crate never has to care.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    ToggleLabels,
    ToggleBorders,
    ToggleDimming,
    ToggleProjection,
    /// Swap between the exact solid and a generated world.
    ToggleSource,
    NextSeed,
    MoreRegions(usize),
    FewerRegions(usize),
    ResetView,
}

pub struct PlanetView {
    world: World,
    view: GlobeView,
    settings: Settings,
    /// Who holds each region, indexed by region id. Owned by the model and pushed in
    /// from outside — this layer derives it from nothing and never writes back to it.
    owners: Vec<Option<u16>>,
}

impl PlanetView {
    pub fn new(spec: WorldSpec, width: usize, height: usize) -> Self {
        Self {
            world: World::build(spec),
            view: GlobeView::new(width.max(1), height.max(1)),
            settings: Settings::default(),
            owners: Vec::new(),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn view(&self) -> &GlobeView {
        &self.view
    }

    pub fn view_mut(&mut self) -> &mut GlobeView {
        &mut self.view
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Replaces the ownership the view draws. Strictly one-way: the view model reads
    /// the model and never writes to it.
    pub fn set_owners(&mut self, owners: Vec<Option<u16>>) {
        self.owners = owners;
    }

    pub fn owners(&self) -> &[Option<u16>] {
        &self.owners
    }

    pub fn size(&self) -> (usize, usize) {
        (self.view.width, self.view.height)
    }

    pub fn pixel_count(&self) -> usize {
        self.view.width * self.view.height
    }

    /// Returns true when the size actually changed, so a caller can avoid
    /// reallocating a buffer every frame.
    ///
    /// The zoom *factor* is preserved rather than the radius in pixels, so the world
    /// keeps its apparent size relative to the window. Without this an adapter that
    /// creates the view before it knows the window size — which is the normal case,
    /// since the size arrives with the first window event — would be stuck at whatever
    /// zoom that placeholder size implied.
    pub fn resize(&mut self, width: usize, height: usize) -> bool {
        let (width, height) = (width.max(1), height.max(1));
        if (width, height) == (self.view.width, self.view.height) {
            return false;
        }
        let before = crate::camera::default_radius(self.view.width, self.view.height);
        let after = crate::camera::default_radius(width, height);
        self.view.radius = (self.view.radius * after / before)
            .clamp(crate::camera::MINIMUM_RADIUS, crate::camera::MAXIMUM_RADIUS);
        self.view.width = width;
        self.view.height = height;
        true
    }

    /// Turns the sphere, following a drag in screen pixels.
    pub fn drag(&mut self, dx: f64, dy: f64) {
        self.view.drag(dx, dy);
    }

    /// Zooms about a screen position. `notches` is wheel clicks, and is clamped
    /// because backends disagree about the units.
    pub fn zoom(&mut self, x: f64, y: f64, notches: f64) {
        let notches = notches.clamp(-MAX_NOTCHES_PER_EVENT, MAX_NOTCHES_PER_EVENT);
        if notches != 0.0 {
            self.view.zoom_at(x, y, ZOOM_PER_NOTCH.powf(notches));
        }
    }

    pub fn apply(&mut self, command: Command) {
        let mut spec = self.world.spec;
        match command {
            Command::ToggleLabels => self.settings.labels = !self.settings.labels,
            Command::ToggleBorders => self.settings.borders = !self.settings.borders,
            Command::ToggleDimming => self.settings.dim_duplicates = !self.settings.dim_duplicates,
            Command::ToggleProjection => self.view.projection = self.view.projection.other(),
            Command::ResetView => self.view.reset(),
            Command::ToggleSource => {
                spec.soccer = !spec.soccer;
                self.rebuild(spec);
            }
            Command::NextSeed => {
                // Reseeding a fixed solid means nothing, so this leaves it.
                spec.soccer = false;
                spec.params.seed = spec.params.seed.wrapping_add(1);
                self.rebuild(spec);
            }
            Command::MoreRegions(step) => {
                spec.soccer = false;
                spec.params.region_count += step.max(1);
                self.rebuild(spec);
            }
            Command::FewerRegions(step) => {
                spec.soccer = false;
                let step = step.max(1).min(spec.params.region_count.saturating_sub(1));
                if step > 0 {
                    spec.params.region_count -= step;
                    self.rebuild(spec);
                }
            }
        }
    }

    fn rebuild(&mut self, spec: WorldSpec) {
        self.world = World::build(spec);
    }

    /// Which region is under a screen position.
    pub fn hovered(&self, cursor: Option<(f64, f64)>) -> Option<usize> {
        let (x, y) = cursor?;
        if x < 0.0 || y < 0.0 || x >= self.view.width as f64 || y >= self.view.height as f64 {
            return None;
        }
        let sample = self.view.screen_to_sample(x, y)?;
        Some(self.world.tessellation.region_at(sample.direction.vector()))
    }

    /// Draws a frame. The buffer must hold [`Self::pixel_count`] pixels, each
    /// `0x00RRGGBB`.
    pub fn draw(&self, buffer: &mut [u32], cursor: Option<(f64, f64)>) {
        let hovered = self.hovered(cursor);
        let scene = raster::Scene {
            seeds: &self.world.tessellation.seeds,
            directions: &self.world.directions,
            colors: &self.world.coloring.colors,
            owners: &self.owners,
            hovered,
            show_borders: self.settings.borders,
            show_labels: self.settings.labels,
            dim_duplicates: self.settings.dim_duplicates,
        };
        raster::render(buffer, &self.view, &scene, cursor);
        raster::draw_readout(
            buffer,
            self.view.width,
            self.view.height,
            &self.readout(cursor),
        );
    }

    /// Draws only the parts that sit on top of the planet, leaving the rest
    /// transparent. Used when the sphere itself is drawn by a shader.
    pub fn draw_overlay(&self, buffer: &mut [u32], cursor: Option<(f64, f64)>) {
        let hovered = self.hovered(cursor);
        let scene = raster::Scene {
            seeds: &self.world.tessellation.seeds,
            directions: &self.world.directions,
            colors: &self.world.coloring.colors,
            owners: &self.owners,
            hovered,
            show_borders: self.settings.borders,
            show_labels: self.settings.labels,
            dim_duplicates: self.settings.dim_duplicates,
        };
        raster::render_overlay(buffer, &self.view, &scene, cursor);
        raster::draw_readout(
            buffer,
            self.view.width,
            self.view.height,
            &self.readout(cursor),
        );
    }

    /// The lines shown in the corner.
    pub fn readout(&self, cursor: Option<(f64, f64)>) -> Vec<String> {
        let tessellation = &self.world.tessellation;
        let centre = self.view.centre_direction();

        let mut lines = vec![
            format!(
                "regions {}  edges {}  colors {} ({})",
                tessellation.region_count(),
                tessellation.edge_count(),
                self.world.coloring.color_count,
                self.world.coloring_method()
            ),
            format!("neighbours {}", self.world.degree_summary()),
        ];

        if self.world.spec.soccer {
            let check = &self.world.verification;
            lines.push(check.summary());
            lines.push(format!(
                "border pentagon-hexagon {:.6} rad, hexagon-hexagon {:.6} rad",
                check.pentagon_hexagon_border, check.hexagon_hexagon_border
            ));
            lines.push(format!(
                "spread within a kind: borders {:.1e}  seed angles {:.1e}",
                check.border_spread, check.angle_spread
            ));
        } else {
            let params = self.world.spec.params;
            lines.push(format!(
                "generated: jitter {:.2} relax {} seed {}{}",
                params.jitter,
                params.relaxation,
                params.seed,
                if tessellation.is_soccer_ball() {
                    "  and it landed on a soccer ball"
                } else {
                    ""
                }
            ));
        }

        lines.push(format!(
            "{}  zoom {:.2}x  world disc {:.0}px across",
            self.view.projection.name(),
            self.view.radius / crate::camera::default_radius(self.view.width, self.view.height),
            self.view.radius * 2.0
        ));
        lines.push(format!(
            "facing lon {:.1} lat {:.1}",
            centre.longitude().to_degrees(),
            centre.latitude().to_degrees()
        ));
        lines.push(match self.hovered(cursor) {
            Some(region) => format!(
                "region {}  neighbours {}  color {}",
                region,
                tessellation.neighbours[region].len(),
                self.world.coloring.colors[region]
            ),
            None => "region -".to_string(),
        });
        lines.push(self.world.quality.summary());
        let owned = self.owners.iter().filter(|owner| owner.is_some()).count();
        lines.push(format!(
            "owned {} of {}  (model: {} regions, {} borders)",
            owned,
            tessellation.region_count(),
            tessellation.region_count(),
            tessellation.edge_count()
        ));
        lines.push(format!(
            "labels {}  borders {}  dim repeats {}",
            on_off(self.settings.labels),
            on_off(self.settings.borders),
            on_off(self.settings.dim_duplicates)
        ));
        lines.push(
            "drag/arrows:turn wheel:zoom  P:fan/globe  S:solid/generated  L B D  R:reseed  \
             -/+:regions (shift x10)  0:reset"
                .to_string(),
        );
        lines
    }

    pub fn projection(&self) -> Projection {
        self.view.projection
    }
}

fn on_off(flag: bool) -> &'static str {
    if flag { "on" } else { "off" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sphere_tessellation::Params;

    fn view() -> PlanetView {
        PlanetView::new(WorldSpec::default(), 240, 200)
    }

    #[test]
    fn it_starts_on_the_perfect_solid() {
        let planet = view();
        assert!(planet.world().verification.is_perfect());
        assert!(
            planet
                .readout(None)
                .iter()
                .any(|line| line.contains("PERFECT")),
            "the readout should state the verification"
        );
    }

    #[test]
    fn drawing_fills_every_pixel() {
        let planet = view();
        let mut buffer = vec![0xDEAD_BEEF_u32; planet.pixel_count()];
        planet.draw(&mut buffer, Some((120.0, 100.0)));
        assert!(buffer.iter().all(|&pixel| pixel != 0xDEAD_BEEF));
    }

    /// An adapter cannot know the window size until the first window event, so the
    /// view is created at a placeholder size and resized. That must land on the same
    /// zoom as creating it at the real size in the first place.
    #[test]
    fn resizing_from_a_placeholder_size_lands_on_the_default_zoom() {
        let mut late = PlanetView::new(WorldSpec::default(), 1, 1);
        assert!(late.resize(900, 760));

        let direct = PlanetView::new(WorldSpec::default(), 900, 760);
        assert!(
            (late.view().radius - direct.view().radius).abs() < 1e-9,
            "placeholder start gave radius {} but creating at size gave {}",
            late.view().radius,
            direct.view().radius
        );
    }

    #[test]
    fn resizing_keeps_the_zoom_factor_rather_than_the_pixel_radius() {
        let mut planet = PlanetView::new(WorldSpec::default(), 800, 800);
        planet.zoom(400.0, 400.0, 4.0);
        let factor = planet.view().radius / crate::camera::default_radius(800, 800);

        planet.resize(1600, 1600);
        let after = planet.view().radius / crate::camera::default_radius(1600, 1600);
        assert!(
            (factor - after).abs() < 1e-9,
            "zoom factor drifted from {factor} to {after} on resize"
        );
    }

    #[test]
    fn resizing_reports_whether_it_changed() {
        let mut planet = view();
        assert!(planet.resize(320, 240));
        assert_eq!(planet.size(), (320, 240));
        assert!(
            !planet.resize(320, 240),
            "an unchanged size is not a change"
        );
        assert!(planet.resize(0, 0), "a zero size is clamped, not accepted");
        assert_eq!(planet.size(), (1, 1));
    }

    #[test]
    fn dragging_turns_the_sphere() {
        let mut planet = view();
        let before = planet.view().centre_direction();
        planet.drag(40.0, 15.0);
        assert!(before.angle_to(planet.view().centre_direction()) > 0.01);
    }

    #[test]
    fn zooming_is_clamped_and_ignores_nothing_events() {
        let mut planet = view();
        let before = planet.view().radius;
        planet.zoom(120.0, 100.0, 0.0);
        assert_eq!(planet.view().radius, before, "a zero notch must do nothing");

        planet.zoom(120.0, 100.0, 1000.0);
        let after_huge = planet.view().radius;
        planet.zoom(120.0, 100.0, 1000.0);
        assert!(
            planet.view().radius > after_huge,
            "a huge event is clamped, not saturating"
        );
    }

    /// Region count is fixed on the solid, so asking for more has to leave it.
    #[test]
    fn changing_the_region_count_leaves_the_fixed_solid() {
        let mut planet = view();
        assert!(planet.world().spec.soccer);
        planet.apply(Command::MoreRegions(10));
        assert!(!planet.world().spec.soccer);
        assert_eq!(planet.world().tessellation.region_count(), 42);

        planet.apply(Command::FewerRegions(10));
        assert_eq!(planet.world().tessellation.region_count(), 32);
    }

    #[test]
    fn the_region_count_never_goes_below_one() {
        let mut planet = PlanetView::new(
            WorldSpec {
                params: Params {
                    region_count: 3,
                    ..Default::default()
                },
                soccer: false,
            },
            120,
            100,
        );
        for _ in 0..10 {
            planet.apply(Command::FewerRegions(10));
        }
        assert_eq!(planet.world().tessellation.region_count(), 1);
    }

    #[test]
    fn toggles_flip_and_flip_back() {
        let mut planet = view();
        let before = *planet.settings();
        for command in [
            Command::ToggleLabels,
            Command::ToggleBorders,
            Command::ToggleDimming,
        ] {
            planet.apply(command);
        }
        assert_ne!(*planet.settings(), before);
        for command in [
            Command::ToggleLabels,
            Command::ToggleBorders,
            Command::ToggleDimming,
        ] {
            planet.apply(command);
        }
        assert_eq!(*planet.settings(), before);
    }

    /// Toggling swaps which *source* the world comes from. At 32 regions with no
    /// jitter both sources agree on the answer, because there is only one good answer —
    /// so this uses a jittered world to tell them apart.
    #[test]
    fn toggling_the_source_swaps_between_the_solid_and_a_generated_world() {
        let mut planet = PlanetView::new(
            WorldSpec {
                params: Params {
                    jitter: 0.25,
                    ..Default::default()
                },
                soccer: true,
            },
            240,
            200,
        );
        assert!(planet.world().verification.is_perfect());
        assert!(planet.world().spec.soccer);

        planet.apply(Command::ToggleSource);
        assert!(!planet.world().spec.soccer);
        assert!(!planet.world().verification.is_perfect());

        planet.apply(Command::ToggleSource);
        assert!(planet.world().verification.is_perfect());
    }

    #[test]
    fn reseeding_leaves_the_solid_because_a_fixed_shape_has_no_seed() {
        let mut planet = view();
        planet.apply(Command::NextSeed);
        assert!(!planet.world().spec.soccer);
    }

    #[test]
    fn resetting_the_view_undoes_turning_and_zooming() {
        let mut planet = view();
        let before = planet.view().centre_direction();
        let radius = planet.view().radius;
        planet.drag(90.0, 60.0);
        planet.zoom(100.0, 100.0, 3.0);
        planet.apply(Command::ResetView);
        assert!(before.angle_to(planet.view().centre_direction()) < 1e-9);
        assert!((planet.view().radius - radius).abs() < 1e-9);
    }

    #[test]
    fn hovering_finds_a_region_inside_the_window_and_nothing_outside() {
        let planet = view();
        assert!(planet.hovered(Some((120.0, 100.0))).is_some());
        assert!(planet.hovered(Some((-1.0, 100.0))).is_none());
        assert!(planet.hovered(Some((240.0, 100.0))).is_none());
        assert!(planet.hovered(None).is_none());
    }

    /// A whole session driven without a window, which is the point of this boundary.
    #[test]
    fn a_session_can_be_driven_with_no_engine_at_all() {
        let mut planet = view();
        let mut buffer = vec![0u32; planet.pixel_count()];

        planet.drag(30.0, 20.0);
        planet.zoom(120.0, 100.0, 2.0);
        planet.apply(Command::ToggleProjection);
        planet.apply(Command::MoreRegions(8));
        planet.apply(Command::NextSeed);
        planet.apply(Command::ToggleLabels);
        if planet.resize(200, 160) {
            buffer.resize(planet.pixel_count(), 0);
        }
        planet.draw(&mut buffer, Some((100.0, 80.0)));

        assert_eq!(buffer.len(), 200 * 160);
        assert!(buffer.iter().any(|&pixel| pixel != 0));
        assert_eq!(planet.projection(), Projection::Globe);
    }
}
