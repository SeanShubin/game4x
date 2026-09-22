//! The board: where the territories are, and which routes between them are unambiguous.
//!
//! **No engine here and no game.** A board is a list of directions on the unit sphere and a
//! neighbour list, both from `sphere-tessellation`. Everything about moving is graph work over
//! that neighbour list, which is why it is in its own file: the interesting rule in this
//! prototype is *which destinations a player may click*, and that rule has to be checkable
//! without a window.

use sphere_tessellation::Vec3;

/// A territory, by the index `sphere-tessellation` gives it.
pub type Where = u32;

/// The twelve, thirty-two or forty-two faces of a Goldberg solid, and who touches whom.
pub struct Board {
    /// The centre of each territory, as a direction on the unit sphere.
    pub centres: Vec<Vec3>,
    /// Each territory's neighbours, sorted.
    pub neighbours: Vec<Vec<Where>>,
    /// The middle of the border between two territories, for each ordered pair that has one.
    ///
    /// **Held because a route is drawn through it** - the research lens, 2026-09-21: the
    /// practice for a path over a tiled map is to give the line its points as the cell centre,
    /// then the midpoint of the edge it crosses, alternating. **The geometry is the arc's job
    /// and this is a different fact**: it makes the route visibly *cross* each boundary rather
    /// than skip from centre to centre, which is what turns a polyline into a route through
    /// these particular territories.
    ///
    /// Keyed both ways round, because a route is walked in one direction and the border does
    /// not care which.
    pub borders: std::collections::BTreeMap<(Where, Where), Vec3>,
}

/// What clicking a destination would mean, given where the move has reached so far.
///
/// **The whole rule of this prototype is in this enum.** Sean, 2026-09-21: *I can only select
/// destinations that leave the path unambiguous, otherwise I have to click on the intermediate
/// destinations.* So a click is answered with the route it implies, or with a refusal that says
/// how many routes there were.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reach {
    /// The destination is where the move already is.
    Arrived,
    /// Exactly one shortest route, given as the territories stepped onto - the origin is not
    /// in it, and the destination is the last entry.
    One(Vec<Where>),
    /// More than one shortest route of the same length, so the click says where to end up and
    /// not how to get there.
    Many {
        /// How many shortest routes there are.
        ways: usize,
        /// How long every one of them is, in steps.
        steps: usize,
    },
    /// No route at all, which on a sphere means the board is broken rather than the click is.
    Nowhere,
}

impl Board {
    /// The `GP(m, n)` Goldberg solid, with its Voronoi neighbours and its borders.
    pub fn goldberg(m: usize, n: usize) -> Self {
        let centres = sphere_tessellation::goldberg::seeds(m, n);
        let neighbours = sphere_tessellation::adjacency(&centres);
        let solid = sphere_tessellation::solid(&centres, &neighbours);

        // **Two neighbours share exactly two corners**, which are the ends of the border
        // between them. Taken from the same `Solid` the planet's panels are built from, so the
        // point a route crosses at is the point the drawing puts the boundary at.
        let mut borders = std::collections::BTreeMap::new();
        for (from, touching) in neighbours.iter().enumerate() {
            for &to in touching {
                let theirs = &solid.cells[to as usize];
                let shared: Vec<u32> = solid.cells[from]
                    .iter()
                    .copied()
                    .filter(|corner| theirs.contains(corner))
                    .collect();
                let [one, two] = shared[..] else { continue };
                let (one, two) = (
                    solid.corners[one as usize].vector(),
                    solid.corners[two as usize].vector(),
                );
                borders.insert(
                    (from as Where, to),
                    Vec3::new(
                        (one.x + two.x) * 0.5,
                        (one.y + two.y) * 0.5,
                        (one.z + two.z) * 0.5,
                    ),
                );
            }
        }

        Self {
            centres,
            neighbours,
            borders,
        }
    }

    /// The route as points to draw through: each centre, and the border it crosses next.
    ///
    /// **A border's midpoint is inside the sphere**, both its corners being on it, so every
    /// point is put back on the unit sphere before `along` floats it. What the border gives is
    /// the *direction* the crossing happens in, not a radius.
    pub fn through(&self, route: &[Where]) -> Vec<Vec3> {
        let mut points = Vec::new();
        for (at, &face) in route.iter().enumerate() {
            points.push(self.centres[face as usize]);
            if let Some(&next) = route.get(at + 1)
                && let Some(border) = self.borders.get(&(face, next))
            {
                let length =
                    (border.x * border.x + border.y * border.y + border.z * border.z).sqrt();
                points.push(Vec3::new(
                    border.x / length,
                    border.y / length,
                    border.z / length,
                ));
            }
        }
        points
    }

    pub fn territories(&self) -> usize {
        self.centres.len()
    }

    /// What a click on `to` means, for a move that has reached `from`.
    ///
    /// **Shortest routes and nothing else.** A longer route is never offered, because the whole
    /// point of clicking an intermediate territory is to pick one of several shortest routes -
    /// and a player who wants a detour says so by clicking the detour, which is then itself a
    /// shortest route from where the move has reached.
    ///
    /// **Counted before it is reconstructed.** A breadth-first sweep carries how many shortest
    /// routes reach each territory as well as how far it is, so *is it unambiguous* is answered
    /// by a number rather than by trying to build a route and noticing a second one. The two
    /// halves cannot disagree that way.
    pub fn reach(&self, from: Where, to: Where) -> Reach {
        if from == to {
            return Reach::Arrived;
        }
        let count = self.territories();
        let (mut far, mut ways) = (vec![usize::MAX; count], vec![0usize; count]);
        far[from as usize] = 0;
        ways[from as usize] = 1;

        let mut front = std::collections::VecDeque::from([from]);
        while let Some(here) = front.pop_front() {
            let next = far[here as usize] + 1;
            for &other in &self.neighbours[here as usize] {
                let at = other as usize;
                if far[at] == usize::MAX {
                    far[at] = next;
                    front.push_back(other);
                }
                // **Added to rather than assigned**, so a territory two routes reach at the
                // same distance carries two - which is what makes the destination ambiguous.
                if far[at] == next {
                    ways[at] = ways[at].saturating_add(ways[here as usize]);
                }
            }
        }

        let steps = far[to as usize];
        if steps == usize::MAX {
            return Reach::Nowhere;
        }
        if ways[to as usize] != 1 {
            return Reach::Many {
                ways: ways[to as usize],
                steps,
            };
        }

        // One route, so walking back from the destination has exactly one candidate at every
        // stage: the neighbour one step nearer the origin. **`expect` rather than a fallback**,
        // because a count of one that cannot be reconstructed is a defect in the sweep above
        // and silence about it would be the worst of the three outcomes.
        let mut route = Vec::with_capacity(steps);
        let mut here = to;
        while here != from {
            route.push(here);
            here = *self.neighbours[here as usize]
                .iter()
                .find(|&&other| far[other as usize] + 1 == far[here as usize])
                .expect("a territory one step nearer the origin");
        }
        route.reverse();
        Reach::One(route)
    }
}

/// The points of a route, on the sphere rather than through it.
///
/// **`X-38`.** Joining two territory centres with a straight line in three dimensions draws a
/// chord, and a chord between two points at radius `r` passes closest to the centre at
/// `r · cos(θ/2)`. On `GP(2,0)` the neighbours are 31.7 degrees apart at the closest, so a
/// step floated at 1.035 sags to between 0.984 and 0.996 - under the sphere on every one of
/// the 240, and under the **drawn border** on half of them. `tests/curvature.rs` measures
/// both, and the second number is why it is visible: the same line floats over the ground at
/// one boundary and sinks into it at the next.
///
/// **Computed here rather than in the drawing, and that is the point.** `draw_the_route` takes
/// `Gizmos` and nothing can read a gizmo back, so geometry that lives in the drawing is
/// geometry no test can reach. The split is the one `reach` already uses for the routing rule.
///
/// **Spherical interpolation, not normalised linear.** Normalising the straight interpolation
/// would put every point on the sphere too, and would bunch them toward the ends - which is
/// invisible on one step and not on a route of twelve. This spaces them by angle.
///
/// **`each` is per step and not per route**, so a long route is drawn as smoothly as a short
/// one rather than more coarsely.
pub fn along(route: &[Vec3], at: f64, each: usize) -> Vec<Vec3> {
    assert!(each >= 1, "a step needs at least one segment");
    let mut points = Vec::new();
    if let Some(first) = route.first() {
        points.push(scaled(*first, at));
    }
    for pair in route.windows(2) {
        let (from, to) = (pair[0], pair[1]);
        let angle = dot(from, to).clamp(-1.0, 1.0).acos();
        for step in 1..=each {
            let how_far = step as f64 / each as f64;
            points.push(scaled(between(from, to, angle, how_far), at));
        }
    }
    points
}

fn dot(one: Vec3, two: Vec3) -> f64 {
    one.x * two.x + one.y * two.y + one.z * two.z
}

fn scaled(one: Vec3, by: f64) -> Vec3 {
    Vec3::new(one.x * by, one.y * by, one.z * by)
}

/// A point `how_far` of the way from `from` to `to` along the shorter great circle.
///
/// **Two points a whole turn apart have no shorter arc**, and two that coincide have no
/// direction between them; both degenerate to the straight interpolation, which is exact for
/// the second case and arbitrary for the first. **Neither happens between neighbours** - the
/// widest separation on this board is far short of a half turn - so the fallback is there to
/// be correct rather than to be reached.
fn between(from: Vec3, to: Vec3, angle: f64, how_far: f64) -> Vec3 {
    let sine = angle.sin();
    if sine.abs() < 1e-9 {
        return Vec3::new(
            from.x + (to.x - from.x) * how_far,
            from.y + (to.y - from.y) * how_far,
            from.z + (to.z - from.z) * how_far,
        );
    }
    let (near, far) = (
        ((1.0 - how_far) * angle).sin() / sine,
        (how_far * angle).sin() / sine,
    );
    Vec3::new(
        from.x * near + to.x * far,
        from.y * near + to.y * far,
        from.z * near + to.z * far,
    )
}
