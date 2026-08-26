//! Unit vectors on the sphere.
//!
//! Everything here is floating point. That is deliberate and confined to this crate:
//! the geometry is a generation-time and rendering-time concern, and only the integer
//! adjacency graph crosses into the game logic. See `docs/architecture.md`.

use std::f64::consts::TAU;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // `add` and `sub` are inherent rather than `std::ops` implementations on purpose.
    // Everything in this crate reads as a chain - `a.sub(b).cross(a.sub(c)).normalized()` -
    // and operators would break that chain in the middle of expressions where the order of
    // operations is the thing being checked. Naming them after the trait methods is what
    // clippy objects to; the alternative names would be worse.
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn scaled(self, factor: f64) -> Self {
        Self::new(self.x * factor, self.y * factor, self.z * factor)
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Returns the unit vector in the same direction. Degenerate input yields the
    /// north pole rather than a NaN, so callers never have to guard.
    pub fn normalized(self) -> Self {
        let length = self.length();
        if length < 1e-15 {
            Self::new(0.0, 0.0, 1.0)
        } else {
            self.scaled(1.0 / length)
        }
    }

    /// Longitude in `[0, TAU)`, measured east from the +x axis.
    pub fn longitude(self) -> f64 {
        self.y.atan2(self.x).rem_euclid(TAU)
    }

    /// Latitude in `[-PI/2, PI/2]`.
    pub fn latitude(self) -> f64 {
        self.z.clamp(-1.0, 1.0).asin()
    }

    pub fn from_lon_lat(longitude: f64, latitude: f64) -> Self {
        let (sin_lat, cos_lat) = latitude.sin_cos();
        let (sin_lon, cos_lon) = longitude.sin_cos();
        Self::new(cos_lat * cos_lon, cos_lat * sin_lon, sin_lat)
    }

    /// The angle between two vectors, of any length.
    ///
    /// Normalizing first matters: seeds are not always unit vectors. A region's seed is
    /// `n / h` — the face normal over its plane distance — so that a ray from the centre
    /// hits the nearest face plane first, and those have varying length by design. See
    /// [`crate::icosahedral::truncated_icosahedron_seeds`].
    pub fn angle_to(self, other: Self) -> f64 {
        self.normalized()
            .dot(other.normalized())
            .clamp(-1.0, 1.0)
            .acos()
    }

    /// Some unit vector perpendicular to this one. Which one is unspecified, but it is
    /// always well conditioned — the reference axis is chosen to avoid near-parallel
    /// cross products.
    pub fn any_perpendicular(self) -> Self {
        let reference = if self.z.abs() < 0.9 {
            Self::new(0.0, 0.0, 1.0)
        } else {
            Self::new(1.0, 0.0, 0.0)
        };
        self.cross(reference).normalized()
    }

    /// Moves `distance` radians along the great circle heading in direction
    /// `tangent`, which must be a unit vector perpendicular to `self`.
    pub fn moved_along(self, tangent: Self, distance: f64) -> Self {
        let (sin_d, cos_d) = distance.sin_cos();
        self.scaled(cos_d).add(tangent.scaled(sin_d)).normalized()
    }
}

/// A unit vector, guaranteed.
///
/// This type exists because of a bug that happened three times. Region seeds are not
/// unit vectors — a seed is `n / h`, the face normal over its plane distance, which is
/// what makes the edges of a Goldberg solid come out right. Any code that read a seed's
/// component as a cosine, or fed one to `acos`, was silently wrong: the adjacency cutoff,
/// the shader's border test, and the label projection each broke in turn, and each
/// surfaced as a different visual symptom rather than as an error.
///
/// Passing a [`Vec3`] where a `Direction` is wanted is now a compile error, and
/// constructing one normalizes. So a `Direction`'s components are always safe to read as
/// trigonometric values, and a seed can never be mistaken for one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Direction(Vec3);

impl Direction {
    /// Normalizes, so the result is a unit vector whatever came in.
    pub fn of(vector: Vec3) -> Self {
        Self(vector.normalized())
    }

    pub fn from_lon_lat(longitude: f64, latitude: f64) -> Self {
        Self(Vec3::from_lon_lat(longitude, latitude))
    }

    /// The underlying vector, which is always unit length.
    pub fn vector(self) -> Vec3 {
        self.0
    }

    pub fn x(self) -> f64 {
        self.0.x
    }

    pub fn y(self) -> f64 {
        self.0.y
    }

    pub fn z(self) -> f64 {
        self.0.z
    }

    pub fn longitude(self) -> f64 {
        self.0.longitude()
    }

    pub fn latitude(self) -> f64 {
        self.0.latitude()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.0.dot(other.0)
    }

    pub fn angle_to(self, other: Self) -> f64 {
        self.dot(other).clamp(-1.0, 1.0).acos()
    }

    pub fn scaled(self, factor: f64) -> Vec3 {
        self.0.scaled(factor)
    }

    pub fn opposite(self) -> Self {
        Self(self.0.scaled(-1.0))
    }
}

#[cfg(test)]
mod direction_tests {
    use super::*;

    #[test]
    fn constructing_one_always_gives_unit_length() {
        for vector in [
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::from_lon_lat(1.0, 0.3).scaled(0.9742),
        ] {
            assert!((Direction::of(vector).vector().length() - 1.0).abs() < 1e-12);
        }
    }

    /// The bug this type exists to prevent: length must not affect anything angular.
    #[test]
    fn length_cannot_affect_an_angle() {
        let base = Vec3::from_lon_lat(1.1, 0.4);
        let other = Vec3::from_lon_lat(2.0, -0.2);
        let expected = Direction::of(base).angle_to(Direction::of(other));
        for length in [0.5, 0.9742, 1.0, 4.0] {
            let scaled = Direction::of(base.scaled(length));
            assert!((scaled.angle_to(Direction::of(other)) - expected).abs() < 1e-12);
        }
    }

    #[test]
    fn latitude_and_longitude_survive_a_non_unit_input() {
        let direction = Direction::of(Vec3::from_lon_lat(2.3, -0.5).scaled(0.31));
        assert!((direction.longitude() - 2.3).abs() < 1e-9);
        assert!((direction.latitude() + 0.5).abs() < 1e-9);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, PI};

    #[test]
    fn lon_lat_round_trips() {
        for &(lon, lat) in &[(0.0, 0.0), (1.0, 0.5), (PI, -0.9), (5.5, FRAC_PI_2 - 0.01)] {
            let v = Vec3::from_lon_lat(lon, lat);
            assert!((v.longitude() - lon).abs() < 1e-9, "longitude {lon}");
            assert!((v.latitude() - lat).abs() < 1e-9, "latitude {lat}");
            assert!((v.length() - 1.0).abs() < 1e-12);
        }
    }

    #[test]
    fn perpendicular_is_perpendicular() {
        for &v in &[
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::from_lon_lat(2.0, 1.4),
        ] {
            let p = v.any_perpendicular();
            assert!(v.dot(p).abs() < 1e-12);
            assert!((p.length() - 1.0).abs() < 1e-12);
        }
    }

    #[test]
    fn moving_along_a_tangent_covers_the_requested_angle() {
        let start = Vec3::from_lon_lat(0.3, 0.2);
        let tangent = start.any_perpendicular();
        let moved = start.moved_along(tangent, 0.4);
        assert!((start.angle_to(moved) - 0.4).abs() < 1e-9);
    }
}
