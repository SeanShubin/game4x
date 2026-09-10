//! The gap projection: every territory keeps its shape, and the curvature is paid as gaps.
//!
//! # The question
//!
//! **Can a player set a destination anywhere on the world with one mouse gesture, without
//! rotating anything?** On the globe the mouse does two jobs - drag to rotate, click to
//! choose - and reaching the far side needs both. A flat map needs only the click, and the
//! usual objection is that flattening a sphere distorts what it draws.
//!
//! Sean's proposal, 2026-09-09: **keep the territory shapes and put the distortion between
//! them.** A territory is drawn at its true shape wherever it lands; what stretches is the
//! space between territories, not the territories.
//!
//! # Why the tax cannot be avoided, only placed
//!
//! **Descartes' theorem: the total angular defect of a closed solid is `360 * chi` degrees,
//! and `chi` is 2, so it is 720 degrees** - whatever the planet's size. Flattening rigid
//! faces has to open that much angle somewhere. So the choice a layout makes is never *how
//! much* to pay, which is fixed, but *where*. Sean's framing is the right one and this is the
//! number behind it.
//!
//! # Why this particular projection
//!
//! Place each territory's centre by an **azimuthal equidistant** projection from a focus, and
//! draw the territory rigidly around it. That projection has principal scale factors **1
//! along the radius** and **theta / sin(theta) across it**, so:
//!
//! - the radial scale being exactly 1 means rings keep their true spacing, and tiles are
//!   never driven into each other;
//! - the tangential scale being at least 1 means the space between tiles only ever opens.
//!
//! **That is the whole of why the idea works**, and it is a property of this projection
//! rather than of flattening in general. An equal-area placement compresses radially near the
//! rim and rigid tiles would collide there; a gnomonic one tears them apart before the
//! horizon.
//!
//! # The gap is not optional, and that is a finding rather than a decoration
//!
//! A territory is a *spherical* polygon, and flattening one at its true radii makes it
//! slightly **larger** than it was: the same `theta / sin(theta)` stretch, applied inside a
//! single cell. Two territories that touch *along the line to the focus* therefore cross.
//!
//! **Measured rather than estimated, and it is small**: [`snug_scale`] reports nothing at all
//! at 12 territories, 0.1 per cent at 32 and 42, and half a per cent at 92. Whether any pair
//! lies close enough to that line is an accident of where the seeds fell, which is why it
//! does not grow with cell size and why one size needs no gap at all.
//!
//! **So the gap Sean wants for looks is free.** It is an order of magnitude more than
//! flattening forces, and nothing is given up by choosing it.
//!
//! # What it costs, in numbers rather than adjectives
//!
//! The gap between territories at angular distance `theta` from the focus is
//! `theta / sin(theta) - 1`:
//!
//! | from focus  | gap  | share of world within |
//! | ----------- | ---- | --------------------- |
//! | 30 degrees  | 5%   | 7%                    |
//! | 60 degrees  | 21%  | 25%                   |
//! | 90 degrees  | 57%  | 50%                   |
//! | 120 degrees | 142% | 75%                   |
//!
//! **The whole world fits in one disc** of radius pi, so no territory is off the map and none
//! has to be rotated to. What degenerates is a single point: the territory antipodal to the
//! focus lands on the rim, and its own neighbours are spread around the whole rim circle.
//! **The seam of this projection is a point rather than a line**, and which territory pays it
//! is chosen by choosing the focus.

use sphere_tessellation::{Direction, Solid, Vec3};

/// A point on the flat map, in units where the sphere has radius 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const ORIGIN: Self = Self { x: 0.0, y: 0.0 };

    pub fn distance_to(self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// One territory as it is drawn: its outline, and where it came from.
#[derive(Clone, Debug)]
pub struct Tile {
    /// Which territory this is.
    pub territory: usize,
    /// Where its centre landed on the map.
    pub centre: Point,
    /// Its outline, in order, at true shape.
    pub outline: Vec<Point>,
    /// How far its centre is from the focus, in radians. Zero at the focus, pi at the
    /// antipode.
    pub from_focus: f64,
}

impl Tile {
    /// The furthest any corner sits from the centre.
    pub fn radius(&self) -> f64 {
        self.outline
            .iter()
            .map(|corner| corner.distance_to(self.centre))
            .fold(0.0, f64::max)
    }

    /// The outline's area, by the shoelace formula.
    pub fn area(&self) -> f64 {
        let mut sum = 0.0;
        for at in 0..self.outline.len() {
            let here = self.outline[at];
            let next = self.outline[(at + 1) % self.outline.len()];
            sum += here.x * next.y - next.x * here.y;
        }
        (sum / 2.0).abs()
    }
}

/// The whole world, flattened around one focus.
#[derive(Clone, Debug)]
pub struct Layout {
    pub tiles: Vec<Tile>,
    /// The direction the map is centred on.
    pub focus: Direction,
    /// What every tile was scaled by. One would be true size; less than one is the gap.
    pub scale: f64,
}

impl Layout {
    /// The tile drawn for a territory.
    pub fn tile(&self, territory: usize) -> &Tile {
        &self.tiles[territory]
    }

    /// How far out the map reaches.
    pub fn extent(&self) -> f64 {
        self.tiles
            .iter()
            .flat_map(|tile| tile.outline.iter())
            .map(|corner| corner.distance_to(Point::ORIGIN))
            .fold(0.0, f64::max)
    }
}

/// The frame bearings are measured against, at the focus.
///
/// **A map needs an up**, and `spec/planet.md` already fixes one: *the roll for any point on
/// the planet is fixed, and nothing the user does changes it*. So north is the world's north
/// carried to the focus, and the map turns only when the focus moves.
fn frame_at(focus: Direction) -> (Vec3, Vec3) {
    let world_north = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let focus = focus.vector();
    // At a pole the world's north has no component to carry, so any perpendicular will do.
    // The choice is arbitrary there and only there.
    let along = world_north.sub(focus.scaled(world_north.dot(focus)));
    let north = if along.length() < 1e-9 {
        focus.any_perpendicular().normalized()
    } else {
        along.normalized()
    };
    let east = focus.cross(north);
    (north, east)
}

/// Where a direction lands: distance from the focus, and bearing around it.
fn polar(focus: Vec3, north: Vec3, east: Vec3, at: Vec3) -> (f64, f64) {
    let from_focus = focus.angle_to(at);
    let across = at.sub(focus.scaled(at.dot(focus)));
    if across.length() < 1e-12 {
        // The focus itself, or its antipode. The bearing is undefined; zero is exact at the
        // focus and is the one arbitrary choice this projection makes at the antipode.
        return (from_focus, 0.0);
    }
    let across = across.normalized();
    (from_focus, across.dot(east).atan2(across.dot(north)))
}

/// The frame at a territory's centre that the map turns it into.
///
/// **`away` is the direction the great circle from the focus is heading when it arrives**, so
/// the tile is laid down facing the way the sphere says it faces. Building the outline in any
/// other frame and then turning it by the bearing is wrong by the convergence angle - how
/// much a great circle's direction changes between its two ends - which is the spherical
/// excess and reaches tens of degrees. That was this function's first version, and it made
/// the gap the layout needed erratic rather than proportional to a cell's size.
fn facing(focus: Vec3, centre: Vec3, from_focus: f64) -> (Vec3, Vec3) {
    let across = centre.sub(focus.scaled(centre.dot(focus)));
    if across.length() < 1e-12 {
        // At the focus or its antipode there is no great circle to arrive along, so the
        // world's own frame stands in. At the focus that is exactly right, because the map is
        // centred there and its north is the world's.
        let (north, east) = frame_at(Direction::of(centre));
        return (north, east);
    }
    let away = focus
        .scaled(-from_focus.sin())
        .add(across.normalized().scaled(from_focus.cos()));
    (away, centre.cross(away))
}

/// A territory's outline in its own frame, at true radii, before it is placed.
///
/// **Each corner keeps its exact angular distance from the centre**, which is what makes the
/// tile a fixed shape rather than a projected one - the same territory is the same figure
/// wherever on the map it lands.
fn outline_of(
    solid: &Solid,
    centre: Vec3,
    corners: &[u32],
    facing: (Vec3, Vec3),
) -> Vec<(f64, f64)> {
    let (away, side) = facing;
    corners
        .iter()
        .map(|&corner| {
            let corner = solid.corners[corner as usize].vector();
            let (span, bearing) = polar(centre, away, side, corner);
            (span * bearing.cos(), span * bearing.sin())
        })
        .collect()
}

/// Flattens a solid around a focus, keeping every territory's shape.
///
/// `scale` shrinks every tile by the same factor, which is what opens the gap. One draws
/// territories at true size and lets neighbours cross slightly - see [`snug_scale`], which is
/// the largest scale that does not.
pub fn flatten(solid: &Solid, seeds: &[Vec3], focus: Direction, scale: f64) -> Layout {
    assert_eq!(
        solid.cells.len(),
        seeds.len(),
        "the solid and the seeds are from different worlds"
    );
    assert!(scale > 0.0, "a tile scaled by {scale} would not be drawn");
    let (north, east) = frame_at(focus);
    let focus_vector = focus.vector();

    let mut tiles = Vec::with_capacity(solid.cells.len());
    for (territory, corners) in solid.cells.iter().enumerate() {
        let centre = seeds[territory].normalized();
        let (from_focus, bearing) = polar(focus_vector, north, east, centre);

        // Where the centre lands: distance preserved exactly, bearing preserved exactly.
        let placed = Point {
            x: from_focus * bearing.cos(),
            y: from_focus * bearing.sin(),
        };

        // The tile, laid down rigidly and turned so that the direction away from the focus on
        // the sphere is the direction away from the centre of the map. Turning it by the
        // bearing is what parallel transport comes to for this projection.
        let outline = outline_of(
            solid,
            centre,
            corners,
            facing(focus_vector, centre, from_focus),
        )
        .into_iter()
        .map(|(x, y)| {
            let (x, y) = (x * scale, y * scale);
            Point {
                x: placed.x + x * bearing.cos() - y * bearing.sin(),
                y: placed.y + x * bearing.sin() + y * bearing.cos(),
            }
        })
        .collect();

        tiles.push(Tile {
            territory,
            centre: placed,
            outline,
            from_focus,
        });
    }

    Layout {
        tiles,
        focus,
        scale,
    }
}

/// Whether two outlines cross, by the separating axis theorem.
///
/// **Convex is safe to assume**: the cells are faces of a convex solid laid down rigidly, so
/// a tile is the same convex polygon the sphere had. Sharing an edge counts as apart rather
/// than as crossing, which is what the tolerance is for.
pub fn overlaps(left: &[Point], right: &[Point]) -> bool {
    for outline in [left, right] {
        for at in 0..outline.len() {
            let here = outline[at];
            let next = outline[(at + 1) % outline.len()];
            let axis = Point {
                x: -(next.y - here.y),
                y: next.x - here.x,
            };
            let project = |points: &[Point]| {
                points.iter().fold((f64::MAX, f64::MIN), |(low, high), it| {
                    let along = it.x * axis.x + it.y * axis.y;
                    (low.min(along), high.max(along))
                })
            };
            let (left_low, left_high) = project(left);
            let (right_low, right_high) = project(right);
            if left_high <= right_low + 1e-12 || right_high <= left_low + 1e-12 {
                return false;
            }
        }
    }
    true
}

/// Whether any two territories cross at this scale.
fn anything_crosses(solid: &Solid, seeds: &[Vec3], focus: Direction, scale: f64) -> bool {
    let layout = flatten(solid, seeds, focus, scale);
    for left in 0..layout.tiles.len() {
        for right in (left + 1)..layout.tiles.len() {
            if overlaps(&layout.tiles[left].outline, &layout.tiles[right].outline) {
                return true;
            }
        }
    }
    false
}

/// The largest scale at which no two territories cross - the gap that is not optional.
///
/// **A spherical polygon flattened at its true radii is bigger than it was.** The same
/// `theta / sin(theta)` stretch that opens the gaps between territories also acts inside one,
/// so two territories that touch on the sphere cross on the map. This finds how much has to
/// come off.
///
/// **Searched against [`overlaps`] rather than derived.** A formula was tried first and
/// measured the wrong thing: it compared a cell's circumradius with the distance to its
/// neighbour's centre, a ratio that a perfectly flat hexagon tiling also fails, and so it
/// asked for a 38 per cent shrink at every planet size including those where flattening costs
/// almost nothing. **A search cannot make that mistake**, because the thing it searches
/// against is the thing being claimed.
///
/// The result is a lower bound to within `tolerance`, so it is safe rather than exact. The
/// test beside it asserts that it is also *tight*: a little above this, something crosses.
pub fn snug_scale(solid: &Solid, seeds: &[Vec3], focus: Direction) -> f64 {
    let tolerance = 1e-4;
    if !anything_crosses(solid, seeds, focus, 1.0) {
        return 1.0;
    }
    let (mut safe, mut crossing) = (0.0, 1.0);
    while crossing - safe > tolerance {
        let middle = (safe + crossing) / 2.0;
        if anything_crosses(solid, seeds, focus, middle) {
            crossing = middle;
        } else {
            safe = middle;
        }
    }
    safe
}

/// The gap the projection opens at a given distance from the focus: `theta / sin(theta) - 1`.
///
/// Stated here so a test can compare a layout against it rather than against a typed table.
pub fn predicted_gap(from_focus: f64) -> f64 {
    if from_focus < 1e-9 {
        return 0.0;
    }
    from_focus / from_focus.sin() - 1.0
}

/// The total angular defect any flattening of a closed solid has to absorb, in degrees.
///
/// **Descartes, by way of Euler**: the total defect is `360 * chi`, and a closed solid has
/// `chi = 2`. Written as `360 * chi` rather than as 720 so that a test computes `chi` from
/// the solid and this stops being a number somebody typed.
pub fn total_defect_degrees(solid: &Solid) -> f64 {
    360.0 * solid.euler_characteristic() as f64
}

/// Which territory the map cannot draw honestly: the one antipodal to the focus.
///
/// **The seam of this projection is a point.** That territory lands on the rim and its own
/// neighbours are spread around the whole rim circle, so adjacency reads wrongly there and
/// nowhere else. Returned so a drawing can mark it, and so a caller can choose a focus that
/// puts it somewhere the player does not care about.
pub fn seam(layout: &Layout) -> usize {
    layout
        .tiles
        .iter()
        .max_by(|left, right| left.from_focus.total_cmp(&right.from_focus))
        .expect("a world has territories")
        .territory
}

pub mod draw;
