//! The camera: an orientation of the sphere, and a way of fanning it out flat.
//!
//! # The model
//!
//! Hold a ball. Fan it out flat in front of you and you can see all of it at once: the
//! point facing you sits at the centre undistorted, and everything else is pushed
//! outward, stretching more the further round the back it was. The point directly
//! behind the ball has nowhere to go but the entire rim. To look somewhere else you
//! fold the ball back up, turn it, and fan it out again.
//!
//! That is the [`Projection::Fanned`] view, and it is the azimuthal equidistant
//! projection centred on the view direction. Distance from the centre of the screen is
//! exactly angular distance on the sphere, so the radial scale is truthful everywhere
//! and only the tangential scale stretches:
//!
//! | Angle from centre | Tangential stretch |
//! | --- | --- |
//! | 0 degrees | 1.00, exact |
//! | 30 degrees | 1.05 |
//! | 60 degrees | 1.21 |
//! | 90 degrees | 1.57 |
//! | 150 degrees | 4.53 |
//! | 180 degrees | infinite — the far point becomes the whole rim |
//!
//! # Why this and not a flat map
//!
//! A flat map of a sphere has to fold, and the folds land on the poles. That is not a
//! choice of projection: the sphere is simply connected, so it has no nontrivial
//! covering space, so no flat map of it can wrap by translation alone. Walking north
//! over the pole really does put you on the opposite meridian heading south, and any
//! flat map wrapping vertically has to show that as a mirror.
//!
//! Fanning out has no poles at all. The projection's axis is wherever you are looking,
//! so no location on the sphere is special, and panning is rotation rather than
//! translation — there is no edge to reach.
//!
//! # The repeating rings
//!
//! The whole sphere fits inside the disc of radius `pi`. Keep going outward and the
//! projection simply covers the sphere again: a geodesic that passes the far point
//! carries on and comes back. So the plane beyond the first disc is filled with
//! further copies of the world, in rings, alternating between upright and turned
//! inside out. Those are the duplicates, drawn dimmed, and they are why the view has
//! no background and no edge anywhere.
//!
//! ```text
//!   radius       0 .. pi     pi .. 2pi     2pi .. 3pi
//!   contents     the world   the world     the world
//!   drawn        full        dimmed        dimmed
//! ```

use sphere_tessellation::{Direction, Vec3};
use std::f64::consts::PI;

pub const MINIMUM_RADIUS: f64 = 60.0;
pub const MAXIMUM_RADIUS: f64 = 60_000.0;

/// A 3x3 matrix, stored as rows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3 {
    pub rows: [Vec3; 3],
}

impl Mat3 {
    /// Facing longitude 0, latitude 0, with north up. The rows are the view axes —
    /// right, up, and toward the viewer — written in world coordinates, so the third
    /// row is the point in the middle of the screen.
    pub const FACING_EQUATOR: Self = Self {
        rows: [
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ],
    };

    pub fn apply(&self, vector: Vec3) -> Vec3 {
        Vec3::new(
            self.rows[0].dot(vector),
            self.rows[1].dot(vector),
            self.rows[2].dot(vector),
        )
    }

    /// Applies the transpose, which for a rotation is the inverse.
    pub fn apply_inverse(&self, vector: Vec3) -> Vec3 {
        self.rows[0]
            .scaled(vector.x)
            .add(self.rows[1].scaled(vector.y))
            .add(self.rows[2].scaled(vector.z))
    }

    /// `self * other`, so that `result.apply(v) == self.apply(other.apply(v))`.
    pub fn multiply(&self, other: &Mat3) -> Mat3 {
        Mat3 {
            rows: [
                other.column_combination(self.rows[0]),
                other.column_combination(self.rows[1]),
                other.column_combination(self.rows[2]),
            ],
        }
    }

    fn column_combination(&self, row: Vec3) -> Vec3 {
        Vec3::new(
            row.x * self.rows[0].x + row.y * self.rows[1].x + row.z * self.rows[2].x,
            row.x * self.rows[0].y + row.y * self.rows[1].y + row.z * self.rows[2].y,
            row.x * self.rows[0].z + row.y * self.rows[1].z + row.z * self.rows[2].z,
        )
    }

    /// Rotation about a unit axis, by Rodrigues' formula.
    pub fn rotation(axis: Vec3, angle: f64) -> Mat3 {
        let (sin, cos) = angle.sin_cos();
        let rest = 1.0 - cos;
        let (x, y, z) = (axis.x, axis.y, axis.z);
        Mat3 {
            rows: [
                Vec3::new(cos + x * x * rest, x * y * rest - z * sin, x * z * rest + y * sin),
                Vec3::new(y * x * rest + z * sin, cos + y * y * rest, y * z * rest - x * sin),
                Vec3::new(z * x * rest - y * sin, z * y * rest + x * sin, cos + z * z * rest),
            ],
        }
    }

    /// Turns the sphere so that `target` faces the viewer.
    ///
    /// `orientation` maps world space to view space, and the point facing the viewer
    /// is recovered by the inverse, so a world-space turn composes on the *right* and
    /// runs backwards: `R' = R * Q^-1`. Getting that the wrong way round turns the
    /// sphere twice as far in the wrong direction, which is subtle enough to be worth
    /// naming once and reusing.
    pub fn turned_to_face(&self, from: Vec3, target: Vec3) -> Mat3 {
        let axis = from.cross(target);
        let length = axis.length();
        if length < 1e-12 {
            return *self;
        }
        let angle = from.dot(target).clamp(-1.0, 1.0).acos();
        self.multiply(&Mat3::rotation(axis.scaled(1.0 / length), -angle))
            .orthonormalized()
    }

    /// Gram-Schmidt, so thousands of incremental drags cannot accumulate drift.
    pub fn orthonormalized(&self) -> Mat3 {
        let first = self.rows[0].normalized();
        let second = self.rows[1]
            .sub(first.scaled(first.dot(self.rows[1])))
            .normalized();
        Mat3 {
            rows: [first, second, first.cross(second)],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Projection {
    /// The ball fanned out flat: the whole sphere at once, undistorted at the centre,
    /// stretched at the rim, repeating outward.
    Fanned,
    /// The ball held together: what the eye would see. One hemisphere, no repeats.
    Globe,
}

impl Projection {
    pub fn name(self) -> &'static str {
        match self {
            Projection::Fanned => "fanned",
            Projection::Globe => "globe",
        }
    }

    pub fn other(self) -> Self {
        match self {
            Projection::Fanned => Projection::Globe,
            Projection::Globe => Projection::Fanned,
        }
    }
}

/// A screen pixel resolved onto the sphere.
#[derive(Clone, Copy, Debug)]
pub struct Sample {
    pub direction: Direction,
    /// 0 for the first, full-strength copy of the world; higher for the repeating
    /// rings further out.
    pub copy: u32,
}

/// The camera.
#[derive(Clone, Copy, Debug)]
pub struct GlobeView {
    /// Rotates world space into view space, where +x is right, +y is up, and +z points
    /// at the viewer.
    pub orientation: Mat3,
    /// On-screen radius in pixels of the complete world: the disc containing every
    /// region exactly once when fanned, or the visible limb when held together.
    pub radius: f64,
    pub width: usize,
    pub height: usize,
    pub projection: Projection,
}

impl GlobeView {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            orientation: Mat3::FACING_EQUATOR,
            radius: default_radius(width, height),
            width,
            height,
            projection: Projection::Fanned,
        }
    }

    pub fn reset(&mut self) {
        self.orientation = Mat3::FACING_EQUATOR;
        self.radius = default_radius(self.width, self.height);
    }

    fn centre(&self) -> (f64, f64) {
        (self.width as f64 / 2.0, self.height as f64 / 2.0)
    }

    /// Screen pixels per radian of angular distance, at the centre of the view.
    fn pixels_per_radian(&self) -> f64 {
        match self.projection {
            Projection::Fanned => self.radius / PI,
            Projection::Globe => self.radius,
        }
    }

    /// Which point on the sphere a pixel shows.
    ///
    /// Fanned, this always succeeds: the rings repeat outward forever, so there is no
    /// pixel that shows nothing. Held together, pixels off the limb show space.
    pub fn screen_to_sample(&self, x: f64, y: f64) -> Option<Sample> {
        let (centre_x, centre_y) = self.centre();
        let across = x - centre_x;
        let up = -(y - centre_y);
        let pixels = (across * across + up * up).sqrt();

        let view = match self.projection {
            Projection::Globe => {
                let distance = pixels / self.radius;
                if distance > 1.0 {
                    return None;
                }
                Vec3::new(
                    across / self.radius,
                    up / self.radius,
                    (1.0 - distance * distance).max(0.0).sqrt(),
                )
            }
            Projection::Fanned => {
                let raw = pixels / self.pixels_per_radian();
                // Each ring of width PI holds one more copy of the world. Odd rings
                // are turned inside out: the geodesic has passed the far point and is
                // on its way back, so it arrives from the opposite direction.
                let ring = (raw / PI).floor();
                let into_ring = raw - ring * PI;
                let (angle, flipped) = if (ring as i64) % 2 == 0 {
                    (into_ring, false)
                } else {
                    (PI - into_ring, true)
                };
                let (sin, cos) = angle.sin_cos();
                if pixels < 1e-9 {
                    Vec3::new(0.0, 0.0, 1.0)
                } else {
                    let sign = if flipped { -1.0 } else { 1.0 };
                    Vec3::new(
                        sin * sign * across / pixels,
                        sin * sign * up / pixels,
                        cos,
                    )
                }
            }
        };

        let copy = match self.projection {
            Projection::Globe => 0,
            Projection::Fanned => {
                let raw = pixels / self.pixels_per_radian();
                (raw / PI).floor() as u32
            }
        };
        Some(Sample {
            direction: Direction::of(self.orientation.apply_inverse(view)),
            copy,
        })
    }

    /// Where a point on the sphere lands on screen, in its first copy.
    ///
    /// Takes a [`Direction`] rather than a [`Vec3`] on purpose. Reading `view.z` as a
    /// cosine is only valid for a unit vector, and region seeds are not unit vectors —
    /// they carry a plane distance. When this took a raw `Vec3`, the twelve pentagons'
    /// labels (whose seeds are 2.65% shorter) landed in the wrong place by an amount
    /// that changed as the sphere turned, so they slid about while panning. The type
    /// makes that mistake impossible to write.
    pub fn direction_to_screen(&self, direction: Direction) -> Option<(f64, f64)> {
        let view = self.orientation.apply(direction.vector());
        let (centre_x, centre_y) = self.centre();
        match self.projection {
            Projection::Globe => {
                if view.z <= 0.0 {
                    return None;
                }
                Some((
                    centre_x + view.x * self.radius,
                    centre_y - view.y * self.radius,
                ))
            }
            Projection::Fanned => {
                let angle = view.z.clamp(-1.0, 1.0).acos();
                let planar = (view.x * view.x + view.y * view.y).sqrt();
                let pixels = angle * self.pixels_per_radian();
                if planar < 1e-12 {
                    // Dead centre, or the far point smeared around the whole rim.
                    Some((centre_x, centre_y - pixels))
                } else {
                    Some((
                        centre_x + pixels * view.x / planar,
                        centre_y - pixels * view.y / planar,
                    ))
                }
            }
        }
    }

    /// Every on-screen copy of a point, with its ring number. The first entry is the
    /// full-strength copy when that one is on screen.
    pub fn copies_on_screen(&self, direction: Direction) -> Vec<((f64, f64), u32)> {
        let mut found = Vec::new();
        let (centre_x, centre_y) = self.centre();
        if self.projection == Projection::Globe {
            if let Some(position) = self.direction_to_screen(direction) {
                found.push((position, 0));
            }
            return found;
        }

        let view = self.orientation.apply(direction.vector());
        let angle = view.z.clamp(-1.0, 1.0).acos();
        let planar = (view.x * view.x + view.y * view.y).sqrt();
        let (unit_x, unit_y) = if planar < 1e-12 {
            (0.0, 1.0)
        } else {
            (view.x / planar, view.y / planar)
        };

        let reach = ((self.width as f64).hypot(self.height as f64) / 2.0 + 32.0)
            / self.pixels_per_radian();
        let mut ring = 0u32;
        loop {
            // Ring k sits at angle + k*PI going out, alternating which side it
            // arrives from.
            let (raw, sign) = if ring % 2 == 0 {
                (angle + ring as f64 * PI, 1.0)
            } else {
                ((ring + 1) as f64 * PI - angle, -1.0)
            };
            if raw > reach {
                break;
            }
            let pixels = raw * self.pixels_per_radian();
            let position = (
                centre_x + pixels * sign * unit_x,
                centre_y - pixels * sign * unit_y,
            );
            if position.0 > -32.0
                && position.1 > -32.0
                && position.0 < self.width as f64 + 32.0
                && position.1 < self.height as f64 + 32.0
            {
                found.push((position, ring));
            }
            ring += 1;
            if ring > 64 {
                break;
            }
        }
        found
    }

    /// How far a one-pixel drag turns the sphere.
    pub fn radians_per_pixel(&self) -> f64 {
        1.0 / self.pixels_per_radian()
    }

    /// Rotates the sphere so the surface follows a drag.
    ///
    /// The increments are applied about the *view* axes, not about any fixed world
    /// axis. That is what makes this free of gimbal lock and free of special
    /// behaviour at the poles: there is no "up" to lose.
    pub fn drag(&mut self, dx: f64, dy: f64) {
        let step = self.radians_per_pixel();
        let yaw = Mat3::rotation(Vec3::new(0.0, 1.0, 0.0), dx * step);
        let pitch = Mat3::rotation(Vec3::new(1.0, 0.0, 0.0), dy * step);
        self.orientation = pitch
            .multiply(&yaw)
            .multiply(&self.orientation)
            .orthonormalized();
    }

    /// Zooms about a screen position, keeping whatever is under it fixed.
    pub fn zoom_at(&mut self, x: f64, y: f64, factor: f64) {
        let before = self.screen_to_sample(x, y).map(|sample| sample.direction);
        self.radius = (self.radius * factor).clamp(MINIMUM_RADIUS, MAXIMUM_RADIUS);
        let after = self.screen_to_sample(x, y).map(|sample| sample.direction);
        if let (Some(before), Some(after)) = (before, after) {
            self.orientation = self
                .orientation
                .turned_to_face(after.vector(), before.vector());
        }
    }

    /// The point on the sphere at the centre of the view.
    pub fn centre_direction(&self) -> Direction {
        Direction::of(self.orientation.apply_inverse(Vec3::new(0.0, 0.0, 1.0)))
    }
}

pub fn default_radius(width: usize, height: usize) -> f64 {
    (width.min(height) as f64 * 0.46).clamp(MINIMUM_RADIUS, MAXIMUM_RADIUS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::TAU;

    fn close(a: Vec3, b: Vec3) -> bool {
        a.angle_to(b) < 1e-7
    }

    #[test]
    fn rotation_turns_by_the_requested_angle() {
        let rotation = Mat3::rotation(Vec3::new(0.0, 1.0, 0.0), 0.7);
        let start = Vec3::new(0.0, 0.0, 1.0);
        let turned = rotation.apply(start);
        assert!((start.angle_to(turned) - 0.7).abs() < 1e-9);
        assert!(turned.y.abs() < 1e-12, "rotating about y must leave y alone");
    }

    #[test]
    fn transpose_undoes_the_rotation() {
        let rotation = Mat3::rotation(Vec3::new(0.3, 0.5, 0.8).normalized(), 1.1);
        let v = Vec3::from_lon_lat(2.0, -0.3);
        assert!(close(rotation.apply_inverse(rotation.apply(v)), v));
    }

    #[test]
    fn multiplication_composes_in_order() {
        let first = Mat3::rotation(Vec3::new(0.0, 1.0, 0.0), 0.3);
        let second = Mat3::rotation(Vec3::new(1.0, 0.0, 0.0), 0.4);
        let combined = second.multiply(&first);
        let v = Vec3::from_lon_lat(0.9, 0.2);
        assert!(close(combined.apply(v), second.apply(first.apply(v))));
    }

    /// The tests that used to live here checked that projecting a seed gave the same
    /// answer as projecting its direction — the bug that made region labels slide about
    /// while panning. They cannot be written any more: `direction_to_screen` takes a
    /// [`Direction`], and there is no way to hand it something of the wrong length.
    ///
    /// What remains worth checking is that the type is doing its job at the boundary.
    #[test]
    fn a_seed_becomes_a_direction_without_its_length() {
        let seed = Vec3::from_lon_lat(1.3, 0.4).scaled(0.9742);
        let direction = Direction::of(seed);
        assert!((direction.vector().length() - 1.0).abs() < 1e-12);
        assert!((direction.longitude() - 1.3).abs() < 1e-9);
        assert!((direction.latitude() - 0.4).abs() < 1e-9);
    }

    #[test]
    fn projection_round_trips_in_both_modes() {
        for projection in [Projection::Fanned, Projection::Globe] {
            let mut view = GlobeView::new(400, 300);
            view.projection = projection;
            view.drag(37.0, -19.0);
            for &(x, y) in &[(200.0, 150.0), (230.0, 170.0), (160.0, 120.0)] {
                let sample = view.screen_to_sample(x, y).expect("on the sphere");
                let (back_x, back_y) =
                    view.direction_to_screen(sample.direction).expect("visible");
                assert!(
                    (back_x - x).abs() < 1e-6 && (back_y - y).abs() < 1e-6,
                    "{projection:?} failed to round trip ({x}, {y})"
                );
            }
        }
    }

    /// Fanned out, the entire sphere is inside the first disc, exactly once.
    #[test]
    fn the_whole_world_fits_in_the_first_copy() {
        let view = GlobeView::new(600, 600);
        for step in 0..2_000 {
            let direction = Direction::of(sphere_tessellation::fibonacci_lattice(2_000)[step]);
            let (x, y) = view.direction_to_screen(direction).expect("always visible");
            let distance = ((x - 300.0).powi(2) + (y - 300.0).powi(2)).sqrt();
            assert!(
                distance <= view.radius + 1e-6,
                "a point landed outside the world disc at {distance}"
            );
            let sample = view.screen_to_sample(x, y).expect("inside");
            assert_eq!(sample.copy, 0, "the first disc must be copy zero");
            assert!(close(sample.direction.vector(), direction.vector()));
        }
    }

    /// The far point has nowhere to go but the rim, so the rim is one single place on
    /// the sphere all the way round.
    #[test]
    fn the_far_point_becomes_the_entire_rim() {
        let view = GlobeView::new(600, 600);
        let far = view.centre_direction().opposite();
        for step in 0..64 {
            let angle = TAU * step as f64 / 64.0;
            let x = 300.0 + view.radius * angle.cos();
            let y = 300.0 + view.radius * angle.sin();
            let sample = view.screen_to_sample(x, y).expect("on the rim");
            assert!(
                sample.direction.angle_to(far) < 1e-6,
                "the rim should be the far point, off by {}",
                sample.direction.angle_to(far)
            );
        }
    }

    /// Past the rim the world starts again, which is where the dimmed duplicates come
    /// from — and why there is no background anywhere.
    #[test]
    fn the_world_repeats_beyond_the_rim() {
        let view = GlobeView::new(600, 600);
        let step = view.radius / 12.0;
        for index in 1..40 {
            let x = 300.0 + step * index as f64;
            let sample = view
                .screen_to_sample(x, 300.0)
                .expect("fanned out, every pixel shows something");
            let expected = (index as f64 / 12.0).floor() as u32;
            assert_eq!(sample.copy, expected, "at x offset {}", step * index as f64);
        }
    }

    #[test]
    fn a_point_and_its_first_repeat_are_the_same_place() {
        let view = GlobeView::new(600, 600);
        // Half way out is 90 degrees away; its first repeat sits at 270 degrees of
        // arc measured the other way round, which is the same point.
        let first = view.screen_to_sample(300.0 + view.radius * 0.5, 300.0).unwrap();
        let second = view.screen_to_sample(300.0 - view.radius * 1.5, 300.0).unwrap();
        assert_eq!(first.copy, 0);
        assert_eq!(second.copy, 1);
        assert!(
            close(first.direction.vector(), second.direction.vector()),
            "the repeat should show the same place"
        );
    }

    #[test]
    fn copies_on_screen_agree_with_the_pixels_under_them() {
        let mut view = GlobeView::new(500, 500);
        view.radius = 120.0;
        let direction = Direction::from_lon_lat(1.1, 0.4);
        let copies = view.copies_on_screen(direction);
        assert!(copies.len() > 1, "zoomed out this far there must be repeats");
        for ((x, y), ring) in copies {
            let sample = view.screen_to_sample(x, y).expect("on screen");
            assert_eq!(sample.copy, ring, "ring number disagreed at ({x}, {y})");
            assert!(
                close(sample.direction.vector(), direction.vector()),
                "copy {ring} showed the wrong place"
            );
        }
    }

    /// The point of the whole rewrite: dragging never reaches an edge, never folds,
    /// and never mirrors. Cross both poles many times.
    #[test]
    fn dragging_forever_never_hits_an_edge() {
        let mut view = GlobeView::new(400, 300);
        for step in 0..4_000 {
            view.drag(7.0, if step % 3 == 0 { 11.0 } else { -5.0 });
            let centre = view.centre_direction();
            assert!((centre.vector().length() - 1.0).abs() < 1e-6, "left the sphere at {step}");
            assert!(view.screen_to_sample(200.0, 150.0).is_some());
        }
    }

    #[test]
    fn the_orientation_stays_a_rotation_after_many_drags() {
        let mut view = GlobeView::new(400, 300);
        for _ in 0..10_000 {
            view.drag(3.0, 2.0);
        }
        let rows = view.orientation.rows;
        for row in rows {
            assert!((row.length() - 1.0).abs() < 1e-9);
        }
        assert!(rows[0].dot(rows[1]).abs() < 1e-9);
        assert!(rows[0].dot(rows[2]).abs() < 1e-9);
        assert!(rows[1].dot(rows[2]).abs() < 1e-9);
    }

    #[test]
    fn a_full_turn_comes_back_to_the_start() {
        let mut view = GlobeView::new(400, 300);
        let start = view.centre_direction();
        let total = TAU / view.radians_per_pixel();
        let steps = 2_000;
        for _ in 0..steps {
            view.drag(total / steps as f64, 0.0);
        }
        assert!(close(view.centre_direction().vector(), start.vector()));
    }

    #[test]
    fn dragging_right_moves_the_surface_right() {
        let mut view = GlobeView::new(400, 300);
        let marker = view.centre_direction();
        view.drag(30.0, 0.0);
        let (x, y) = view.direction_to_screen(marker).expect("still visible");
        assert!(x > 200.0, "surface moved the wrong way: x = {x}");
        assert!((y - 150.0).abs() < 1.0);
    }

    #[test]
    fn dragging_down_moves_the_surface_down() {
        let mut view = GlobeView::new(400, 300);
        let marker = view.centre_direction();
        view.drag(0.0, 30.0);
        let (x, y) = view.direction_to_screen(marker).expect("still visible");
        assert!(y > 150.0, "surface moved the wrong way: y = {y}");
        assert!((x - 200.0).abs() < 1.0);
    }

    /// No location is special. Centre the view on a pole, on the equator, anywhere —
    /// the region at the centre should occupy about the same area on screen. This is
    /// the complaint the flat map could never answer.
    #[test]
    fn nowhere_on_the_sphere_is_a_special_place() {
        let tessellation = sphere_tessellation::Tessellation::generate(Default::default());
        let mut measurements = Vec::new();

        for target in [
            Vec3::new(0.0, 0.0, 1.0),  // north pole
            Vec3::new(0.0, 0.0, -1.0), // south pole
            Vec3::from_lon_lat(0.0, 0.0),
            Vec3::from_lon_lat(2.2, 0.6),
            Vec3::from_lon_lat(4.0, -1.1),
        ] {
            let mut view = GlobeView::new(300, 300);
            view.orientation = view
                .orientation
                .turned_to_face(view.centre_direction().vector(), target);
            assert!(
                view.centre_direction().angle_to(Direction::of(target)) < 1e-6,
                "failed to aim at {target:?}"
            );
            let centre_region = tessellation.region_at(view.centre_direction().vector());
            let mut pixels = 0usize;
            for y in 0..300 {
                for x in 0..300 {
                    let sample = view.screen_to_sample(x as f64, y as f64).unwrap();
                    if sample.copy == 0
                        && tessellation.region_at(sample.direction.vector()) == centre_region
                    {
                        pixels += 1;
                    }
                }
            }
            measurements.push(pixels);
        }

        let largest = *measurements.iter().max().unwrap() as f64;
        let smallest = *measurements.iter().min().unwrap() as f64;
        assert!(
            largest / smallest < 1.6,
            "the centred region's on-screen size varies too much by location: \
             {measurements:?}"
        );
    }

    #[test]
    fn zooming_keeps_the_point_under_the_cursor_fixed() {
        for projection in [Projection::Fanned, Projection::Globe] {
            let mut view = GlobeView::new(400, 300);
            view.projection = projection;
            let before = view.screen_to_sample(240.0, 130.0).unwrap().direction;
            view.zoom_at(240.0, 130.0, 1.6);
            let after = view.screen_to_sample(240.0, 130.0).unwrap().direction;
            assert!(
                before.angle_to(after) < 1e-6,
                "{projection:?} drifted by {}",
                before.angle_to(after)
            );
        }
    }

    #[test]
    fn zoom_is_clamped() {
        let mut view = GlobeView::new(400, 300);
        for _ in 0..500 {
            view.zoom_at(200.0, 150.0, 0.5);
        }
        assert!(view.radius >= MINIMUM_RADIUS);
        for _ in 0..500 {
            view.zoom_at(200.0, 150.0, 2.0);
        }
        assert!(view.radius <= MAXIMUM_RADIUS);
    }

    #[test]
    fn holding_the_ball_together_hides_the_far_side() {
        let mut view = GlobeView::new(400, 400);
        view.projection = Projection::Globe;
        let behind = view.centre_direction().opposite();
        assert!(view.direction_to_screen(behind).is_none());
        assert!(view.screen_to_sample(0.0, 0.0).is_none(), "corners are space");
    }
}
