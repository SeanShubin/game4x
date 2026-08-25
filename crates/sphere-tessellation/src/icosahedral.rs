//! The truncated icosahedron — a soccer ball.
//!
//! # What it is
//!
//! 32 regions: **12 pentagons and 20 hexagons**, meeting at 60 vertices along 90
//! borders. Euler's formula checks out, `60 - 90 + 32 = 2`, and the twelve pentagons
//! are the same twelve that
//! [section 2 of the theory](../../../docs/theory/region-splitting.md) proves are
//! unavoidable on any sphere. It is the smallest Goldberg polyhedron with both face
//! kinds, `GP(1, 1)`, and the most familiar object in the whole subject.
//!
//! Its defining feature is that **no two pentagons touch** — every pentagon is ringed
//! entirely by hexagons. That is worth testing for, because it is exactly what makes a
//! soccer ball look like a soccer ball rather than like a lumpy sphere.
//!
//! # Why it is here, given that the design rejects it
//!
//! This is the icosahedral subdivision that
//! [the vision](../../../docs/vision.md) rules out for real worlds, and nothing here
//! changes that. Its twelve pentagons sit at perfect five-fold symmetry points, so once
//! a player finds one the other eleven are predictable — which is precisely the
//! objection.
//!
//! It earns its place as a **reference**: a known-good tessellation with an exactly
//! known answer, which makes it a far better test fixture than any generated world, and
//! the clearest possible demonstration that twelve pentagons are mandatory rather than
//! a quirk of our pipeline.

use crate::vec3::Vec3;

/// The golden ratio, `(1 + sqrt 5) / 2`.
const GOLDEN: f64 = 1.618_033_988_749_895;

/// The twelve vertices of a regular icosahedron, as unit vectors.
///
/// They are the cyclic permutations of `(0, +/-1, +/-golden)`.
pub fn icosahedron_vertices() -> Vec<Vec3> {
    let mut points = Vec::with_capacity(12);
    for &first in &[1.0, -1.0] {
        for &second in &[1.0, -1.0] {
            points.push(Vec3::new(0.0, first, second * GOLDEN));
            points.push(Vec3::new(first, second * GOLDEN, 0.0));
            points.push(Vec3::new(second * GOLDEN, 0.0, first));
        }
    }
    points.into_iter().map(|point| point.normalized()).collect()
}

/// The centres of the icosahedron's twenty triangular faces, as unit vectors.
///
/// The faces are found rather than tabulated: on a regular icosahedron every triple of
/// mutually nearest-neighbour vertices is a face, so there is no coordinate table to
/// get wrong.
pub fn icosahedron_face_centres(vertices: &[Vec3]) -> Vec<Vec3> {
    let mut edge_length = f64::INFINITY;
    for (index, first) in vertices.iter().enumerate() {
        for second in &vertices[index + 1..] {
            edge_length = edge_length.min(first.angle_to(*second));
        }
    }
    // Generous, because on a regular icosahedron the next distance up is a long way
    // further: adjacent vertices are 63.4 degrees apart, the next ones 116.6.
    let limit = edge_length * 1.2;

    let mut centres = Vec::with_capacity(20);
    for first in 0..vertices.len() {
        for second in (first + 1)..vertices.len() {
            if vertices[first].angle_to(vertices[second]) > limit {
                continue;
            }
            for third in (second + 1)..vertices.len() {
                if vertices[first].angle_to(vertices[third]) > limit
                    || vertices[second].angle_to(vertices[third]) > limit
                {
                    continue;
                }
                centres.push(
                    vertices[first]
                        .add(vertices[second])
                        .add(vertices[third])
                        .normalized(),
                );
            }
        }
    }
    centres
}

/// The icosahedron's twenty faces, as triples of vertex indices.
pub fn icosahedron_faces(vertices: &[Vec3]) -> Vec<[usize; 3]> {
    let mut edge_length = f64::INFINITY;
    for (index, first) in vertices.iter().enumerate() {
        for second in &vertices[index + 1..] {
            edge_length = edge_length.min(first.angle_to(*second));
        }
    }
    let limit = edge_length * 1.2;

    let mut faces = Vec::with_capacity(20);
    for first in 0..vertices.len() {
        for second in (first + 1)..vertices.len() {
            if vertices[first].angle_to(vertices[second]) > limit {
                continue;
            }
            for third in (second + 1)..vertices.len() {
                if vertices[first].angle_to(vertices[third]) > limit
                    || vertices[second].angle_to(vertices[third]) > limit
                {
                    continue;
                }
                faces.push([first, second, third]);
            }
        }
    }

    // Wind every face the same way, seen from outside.
    //
    // Sorted index order is arbitrary, so half the faces come out mirrored. Class I and
    // class II patches are symmetric enough not to care, but class III solids are
    // *chiral*: laying a left-handed patch on some faces and a right-handed one on
    // others makes the lattice fail to meet along shared edges. GP(2,1) came out with
    // 56 pentagons instead of 12 before this.
    for face in &mut faces {
        let [a, b, c] = *face;
        let normal = vertices[b]
            .sub(vertices[a])
            .cross(vertices[c].sub(vertices[a]));
        let outward = vertices[a].add(vertices[b]).add(vertices[c]);
        if normal.dot(outward) < 0.0 {
            face.swap(1, 2);
        }
    }
    faces
}

/// The vertices of a class I geodesic polyhedron of the given frequency.
///
/// Kept as a name for the common case; the work is done by
/// [`crate::goldberg::seeds`], which handles any `(m, n)`.
pub fn geodesic_seeds(frequency: usize) -> Vec<Vec3> {
    crate::goldberg::seeds(frequency.max(1), 0)
}

/// How many regions a class I Goldberg polyhedron of this frequency has.
pub fn class_one_region_count(frequency: usize) -> usize {
    10 * frequency * frequency + 2
}

/// Seeds for a canonical, maximally symmetric arrangement at this region count, if one
/// exists.
///
/// The twelve pentagons sit at the icosahedron's vertices and the hexagons form a clean
/// lattice between them, which is only possible when the count is `10*T + 2` for
/// `T = m^2 + mn + n^2`. See [`crate::goldberg`].
///
/// All three classes are built. Class III needed one thing the others did not: every
/// icosahedral face wound the same way. Those solids are chiral, so a mirrored face lays
/// down a mirrored lattice patch and the two fail to meet along their shared edge —
/// `GP(2,1)` came out with 56 pentagons instead of 12 until the winding was fixed.
pub fn canonical_seeds(region_count: usize) -> Option<Vec<Vec3>> {
    if region_count == PENTAGON_COUNT + HEXAGON_COUNT {
        // GP(1,1), built directly so that all ninety of its edges come out equal.
        return Some(truncated_icosahedron_seeds());
    }
    crate::goldberg::arrangements_up_to(region_count)
        .into_iter()
        .find(|&(m, n)| crate::goldberg::region_count(m, n) == region_count)
        .map(|(m, n)| crate::goldberg::seeds(m, n))
}

/// Seeds whose cells form a truncated icosahedron with **all ninety edges equal**.
///
/// # Why the seeds are not unit vectors
///
/// A region is the set of directions for which a given face is hit first by a ray from
/// the centre. Face `i` has plane `n_i . p = h_i`, so a ray in direction `x` hits it at
/// `h_i / (x . n_i)`, and the nearest face is the one maximising `(x . n_i) / h_i`.
/// Using `m_i = n_i / h_i` as the seed therefore turns face lookup back into a plain
/// `argmax` of a dot product — and because the bisector `x . (m_i - m_j) = 0` still
/// passes through the origin, every border is still a great circle. Nothing downstream
/// has to change.
///
/// What this buys is the thing unit seeds cannot express. In the real solid the two
/// kinds of face sit at *different* distances from the centre — pentagons about 2.65%
/// further out than hexagons — and a diagram built from directions alone flattens that
/// away. The bisectors then land in the wrong place and every hexagon comes out with
/// three long sides and three short ones, 45% apart. Dividing by `h_i` restores it.
///
/// The twelve pentagon seeds come first, so regions 0 to 11 are the pentagons.
pub fn truncated_icosahedron_seeds() -> Vec<Vec3> {
    let vertices = icosahedron_vertices();
    let faces = icosahedron_faces(&vertices);
    let (pentagon_plane, hexagon_plane) = truncated_icosahedron_planes(&vertices, &faces);

    let mut seeds: Vec<Vec3> = vertices
        .iter()
        .map(|vertex| vertex.scaled(1.0 / pentagon_plane))
        .collect();
    seeds.extend(
        icosahedron_face_centres(&vertices)
            .iter()
            .map(|centre| centre.scaled(1.0 / hexagon_plane)),
    );
    seeds
}

/// How far the pentagon and hexagon face planes sit from the centre, for a truncated
/// icosahedron built on the unit icosahedron.
///
/// Derived by actually truncating rather than quoting a constant: each icosahedron edge
/// is cut a third of the way along, and the distance to each face plane is read off the
/// result.
fn truncated_icosahedron_planes(vertices: &[Vec3], faces: &[[usize; 3]]) -> (f64, f64) {
    let cut = |from: usize, towards: usize| {
        vertices[from].add(vertices[towards].sub(vertices[from]).scaled(1.0 / 3.0))
    };

    // A pentagon sits where a vertex was. Its plane is perpendicular to that vertex.
    let neighbours: Vec<usize> = (1..vertices.len())
        .filter(|&other| {
            let edge = vertices[0].angle_to(vertices[other]);
            edge < 1.2 * shortest_edge(vertices)
        })
        .collect();
    let pentagon_plane = vertices[0].normalized().dot(cut(0, neighbours[0]));

    // A hexagon sits where a face was, perpendicular to that face's centre.
    let [a, b, c] = faces[0];
    let normal = vertices[a].add(vertices[b]).add(vertices[c]).normalized();
    let hexagon_plane = normal.dot(cut(a, b));

    (pentagon_plane, hexagon_plane)
}

fn shortest_edge(vertices: &[Vec3]) -> f64 {
    let mut shortest = f64::INFINITY;
    for (index, first) in vertices.iter().enumerate() {
        for second in &vertices[index + 1..] {
            shortest = shortest.min(first.angle_to(*second));
        }
    }
    shortest
}

/// How many pentagons the truncated icosahedron has, and therefore how many of the
/// leading seeds are pentagon centres.
pub const PENTAGON_COUNT: usize = 12;

/// How many hexagons it has.
pub const HEXAGON_COUNT: usize = 20;

/// Everything worth checking about a claimed truncated icosahedron.
///
/// Reported rather than merely asserted, so the prototype can show the numbers and a
/// human can see *why* it is perfect rather than taking a boolean on trust.
#[derive(Debug, Clone)]
pub struct Verification {
    pub region_count: usize,
    pub pentagons: usize,
    pub hexagons: usize,
    pub edges: usize,
    /// No two pentagons share a border.
    pub pentagons_isolated: bool,
    /// Every hexagon touches exactly three pentagons.
    pub hexagons_touch_three_pentagons: bool,
    /// The two distinct border lengths, in radians.
    pub pentagon_hexagon_border: f64,
    pub hexagon_hexagon_border: f64,
    /// Largest relative spread found within either border kind. Zero means every
    /// border of that kind is identical.
    pub border_spread: f64,
    /// The two distinct seed-to-seed angles, in radians.
    pub pentagon_hexagon_angle: f64,
    pub hexagon_hexagon_angle: f64,
    /// Largest relative spread within either angle kind.
    pub angle_spread: f64,
}

/// How close a measurement has to be to count as exact. Well above floating point
/// noise, well below anything an irregular tessellation would produce.
pub const EXACT: f64 = 1e-9;

impl Verification {
    pub fn is_perfect(&self) -> bool {
        self.region_count == PENTAGON_COUNT + HEXAGON_COUNT
            && self.pentagons == PENTAGON_COUNT
            && self.hexagons == HEXAGON_COUNT
            && self.edges == 90
            && self.pentagons_isolated
            && self.hexagons_touch_three_pentagons
            && self.border_spread < EXACT
            && self.angle_spread < EXACT
    }

    /// One line, for a readout.
    pub fn summary(&self) -> String {
        if self.is_perfect() {
            format!(
                "PERFECT: {} pentagons {} hexagons {} borders, 2 exact lengths",
                self.pentagons, self.hexagons, self.edges
            )
        } else {
            format!(
                "NOT PERFECT: {} pentagons {} hexagons {} borders, spread {:.2e}",
                self.pentagons,
                self.hexagons,
                self.edges,
                self.border_spread.max(self.angle_spread)
            )
        }
    }
}

fn spread(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let smallest = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let largest = values.iter().cloned().fold(0.0, f64::max);
    if smallest <= 0.0 {
        return f64::INFINITY;
    }
    largest / smallest - 1.0
}

/// Measures a tessellation against the truncated icosahedron.
pub fn verify(seeds: &[Vec3], neighbours: &[Vec<u32>]) -> Verification {
    use crate::adjacency::{edge_count, shared_border_length};

    let is_pentagon = |region: usize| neighbours[region].len() == 5;
    let pentagons = (0..neighbours.len()).filter(|&r| is_pentagon(r)).count();
    let hexagons = (0..neighbours.len())
        .filter(|&r| neighbours[r].len() == 6)
        .count();

    let mut pentagon_hexagon_borders = Vec::new();
    let mut hexagon_hexagon_borders = Vec::new();
    let mut pentagon_hexagon_angles = Vec::new();
    let mut hexagon_hexagon_angles = Vec::new();
    let mut pentagons_isolated = true;

    for first in 0..neighbours.len() {
        for &second in &neighbours[first] {
            let second = second as usize;
            if second < first {
                continue;
            }
            let border = shared_border_length(seeds, first, second);
            let angle = seeds[first].angle_to(seeds[second]);
            match (is_pentagon(first), is_pentagon(second)) {
                (true, true) => pentagons_isolated = false,
                (false, false) => {
                    hexagon_hexagon_borders.push(border);
                    hexagon_hexagon_angles.push(angle);
                }
                _ => {
                    pentagon_hexagon_borders.push(border);
                    pentagon_hexagon_angles.push(angle);
                }
            }
        }
    }

    let hexagons_touch_three_pentagons = (0..neighbours.len())
        .filter(|&r| !is_pentagon(r))
        .all(|r| {
            neighbours[r]
                .iter()
                .filter(|&&n| is_pentagon(n as usize))
                .count()
                == 3
        });

    let mean = |values: &[f64]| {
        if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        }
    };

    Verification {
        region_count: neighbours.len(),
        pentagons,
        hexagons,
        edges: edge_count(neighbours),
        pentagons_isolated,
        hexagons_touch_three_pentagons,
        pentagon_hexagon_border: mean(&pentagon_hexagon_borders),
        hexagon_hexagon_border: mean(&hexagon_hexagon_borders),
        border_spread: spread(&pentagon_hexagon_borders).max(spread(&hexagon_hexagon_borders)),
        pentagon_hexagon_angle: mean(&pentagon_hexagon_angles),
        hexagon_hexagon_angle: mean(&hexagon_hexagon_angles),
        angle_spread: spread(&pentagon_hexagon_angles).max(spread(&hexagon_hexagon_angles)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adjacency::{adjacency, degree_histogram, edge_count, is_connected};

    #[test]
    fn there_are_twelve_vertices_and_they_are_unit_vectors() {
        let vertices = icosahedron_vertices();
        assert_eq!(vertices.len(), 12);
        for vertex in &vertices {
            assert!((vertex.length() - 1.0).abs() < 1e-12);
        }
    }

    /// Every vertex of a regular icosahedron has exactly five neighbours, all the same
    /// distance away. If that fails, the coordinates are wrong.
    #[test]
    fn every_vertex_has_five_equidistant_neighbours() {
        let vertices = icosahedron_vertices();
        let edge = vertices[0]
            .angle_to(vertices[1])
            .min(vertices[0].angle_to(vertices[2]));
        let shortest = vertices
            .iter()
            .enumerate()
            .flat_map(|(index, first)| {
                vertices[index + 1..].iter().map(move |second| first.angle_to(*second))
            })
            .fold(f64::INFINITY, f64::min);
        assert!(edge >= shortest - 1e-9);

        for vertex in &vertices {
            let close = vertices
                .iter()
                .filter(|other| {
                    let angle = vertex.angle_to(**other);
                    angle > 1e-9 && angle < shortest * 1.2
                })
                .count();
            assert_eq!(close, 5, "a vertex had {close} neighbours");
        }
        // The known value, as a guard against a plausible-but-wrong construction.
        assert!(
            (shortest.to_degrees() - 63.4349).abs() < 1e-3,
            "edge angle was {} degrees",
            shortest.to_degrees()
        );
    }

    #[test]
    fn the_icosahedron_has_twenty_faces() {
        let vertices = icosahedron_vertices();
        assert_eq!(icosahedron_faces(&vertices).len(), 20);
    }

    /// Every class I Goldberg polyhedron: the right count, twelve pentagons, the rest
    /// hexagons, and no two pentagons touching. This is the property that makes a
    /// soccer ball read as a soccer ball, generalised.
    #[test]
    fn class_one_goldberg_polyhedra_are_all_well_formed() {
        use crate::adjacency::{adjacency, degree_histogram, edge_count};
        println!("
  GP(m,0) | regions | pentagons | hexagons | touching pentagons");
        for frequency in 1..=7 {
            let seeds = geodesic_seeds(frequency);
            let expected = class_one_region_count(frequency);
            assert_eq!(
                seeds.len(),
                expected,
                "frequency {frequency} should give 10m^2 + 2 seeds"
            );

            let neighbours = adjacency(&seeds);
            let histogram = degree_histogram(&neighbours);
            let pentagons = histogram.get(5).copied().unwrap_or(0);
            let hexagons = histogram.get(6).copied().unwrap_or(0);
            let touching = crate::Quality::measure(&seeds, &neighbours).adjacent_pentagon_pairs;
            println!(
                "  GP({frequency},0) | {expected:>7} | {pentagons:>9} | {hexagons:>8} | {touching:>18}"
            );

            assert_eq!(pentagons, 12, "GP({frequency},0) must have twelve pentagons");
            assert_eq!(
                hexagons,
                expected - 12,
                "GP({frequency},0): everything else must be a hexagon"
            );
            // GP(1,0) is the dodecahedron: twelve pentagons and nothing else, so there
            // are no hexagons to separate them and isolation cannot apply. From GP(2,0)
            // onward it must hold, and that is what makes these read as soccer balls.
            if frequency > 1 {
                assert_eq!(touching, 0, "GP({frequency},0): pentagons must be isolated");
            }
            // Euler, on every one of them.
            assert_eq!(edge_count(&neighbours), 3 * expected - 6);
        }
        println!();
    }

    #[test]
    fn canonical_seeds_exist_exactly_at_the_goldberg_counts() {
        assert!(canonical_seeds(12).is_some(), "GP(1,0), the dodecahedron");
        assert!(canonical_seeds(32).is_some(), "GP(1,1), the truncated icosahedron");
        assert!(canonical_seeds(42).is_some(), "GP(2,0)");
        assert!(canonical_seeds(92).is_some(), "GP(3,0)");
        assert!(canonical_seeds(162).is_some(), "GP(4,0)");
        assert!(canonical_seeds(492).is_some(), "GP(7,0)");

        for count in [20, 33, 50, 100, 150, 300] {
            assert!(
                canonical_seeds(count).is_none(),
                "{count} is not a Goldberg count and should have no canonical shape"
            );
        }
    }

    #[test]
    fn there_are_twenty_faces() {
        let vertices = icosahedron_vertices();
        let centres = icosahedron_face_centres(&vertices);
        assert_eq!(centres.len(), HEXAGON_COUNT);
        for centre in &centres {
            assert!((centre.length() - 1.0).abs() < 1e-12);
        }
    }

    #[test]
    fn the_seed_set_is_thirty_two_points() {
        let seeds = truncated_icosahedron_seeds();
        assert_eq!(seeds.len(), PENTAGON_COUNT + HEXAGON_COUNT);
        assert_eq!(seeds.len(), 32);
    }

    /// The heart of it: twelve pentagons, twenty hexagons, ninety borders.
    #[test]
    fn the_voronoi_cells_are_twelve_pentagons_and_twenty_hexagons() {
        let seeds = truncated_icosahedron_seeds();
        let neighbours = adjacency(&seeds);
        let histogram = degree_histogram(&neighbours);

        assert_eq!(
            histogram.get(5).copied().unwrap_or(0),
            PENTAGON_COUNT,
            "expected twelve pentagons, got {histogram:?}"
        );
        assert_eq!(
            histogram.get(6).copied().unwrap_or(0),
            HEXAGON_COUNT,
            "expected twenty hexagons, got {histogram:?}"
        );
        assert_eq!(histogram.iter().sum::<usize>(), 32);
        assert_eq!(edge_count(&neighbours), 90);
        assert!(is_connected(&neighbours));
    }

    /// Euler's formula, on the canonical example: 60 - 90 + 32 = 2.
    #[test]
    fn euler_holds_for_the_soccer_ball() {
        let neighbours = adjacency(&truncated_icosahedron_seeds());
        let faces = 32;
        let edges = edge_count(&neighbours);
        // Three regions meet at every corner, so each border contributes two corner
        // slots and each corner uses three: vertices = 2 * edges / 3.
        let vertices = 2 * edges / 3;
        assert_eq!((vertices, edges, faces), (60, 90, 32));
        assert_eq!(vertices as i64 - edges as i64 + faces as i64, 2);
    }

    /// What makes a soccer ball look like a soccer ball: every pentagon is ringed
    /// entirely by hexagons, so no two pentagons ever touch.
    #[test]
    fn no_two_pentagons_are_adjacent() {
        let neighbours = adjacency(&truncated_icosahedron_seeds());
        for pentagon in 0..PENTAGON_COUNT {
            assert_eq!(neighbours[pentagon].len(), 5);
            for &neighbour in &neighbours[pentagon] {
                assert!(
                    neighbour as usize >= PENTAGON_COUNT,
                    "pentagon {pentagon} touches pentagon {neighbour}"
                );
            }
        }
    }

    /// Each hexagon is surrounded by three pentagons and three hexagons, alternating.
    #[test]
    fn each_hexagon_touches_three_pentagons() {
        let neighbours = adjacency(&truncated_icosahedron_seeds());
        for hexagon in PENTAGON_COUNT..32 {
            let pentagons = neighbours[hexagon]
                .iter()
                .filter(|&&neighbour| (neighbour as usize) < PENTAGON_COUNT)
                .count();
            assert_eq!(
                pentagons, 3,
                "hexagon {hexagon} touches {pentagons} pentagons"
            );
        }
    }

    /// Does asking for 32 regions naturally produce a soccer ball?
    ///
    /// Partly, and the "partly" is the interesting bit. The degree deficit is always
    /// 12, so *if* every cell is a pentagon or hexagon the counts are forced to be 12
    /// and 20 — that much comes free. What is not forced is the arrangement, and a
    /// soccer ball additionally requires that no two pentagons touch. This reports
    /// what actually happens across the parameter space.
    #[test]
    fn does_thirty_two_regions_make_a_soccer_ball() {
        use crate::{Params, Tessellation};
        let mut soccer = 0;
        let mut right_counts = 0;
        let mut total = 0;

        for jitter in [0.0, 0.15, 0.30] {
            for relaxation in [0, 3, 12, 60] {
                for seed in 1..=6u64 {
                    let tessellation = Tessellation::generate(Params {
                        region_count: 32,
                        jitter,
                        relaxation,
                        seed,
                    });
                    let histogram = tessellation.degree_histogram();
                    let counts_match = histogram.get(5).copied().unwrap_or(0) == 12
                        && histogram.get(6).copied().unwrap_or(0) == 20
                        && histogram.iter().sum::<usize>() == 32;
                    total += 1;
                    if counts_match {
                        right_counts += 1;
                    }
                    if tessellation.is_soccer_ball() {
                        soccer += 1;
                    }
                    if seed == 1 {
                        println!(
                            "  jitter {jitter:.2} relax {relaxation:>2}: degrees {:?}  counts {}  soccer ball {}",
                            histogram
                                .iter()
                                .enumerate()
                                .filter(|&(_, &n)| n > 0)
                                .map(|(d, n)| format!("{d}:{n}"))
                                .collect::<Vec<_>>()
                                .join(" "),
                            if counts_match { "yes" } else { "NO " },
                            tessellation.is_soccer_ball()
                        );
                    }
                }
            }
        }
        println!(
            "
  {right_counts}/{total} had 12 pentagons and 20 hexagons; {soccer}/{total} were actual soccer balls"
        );
        // The deficit theorem guarantees nothing about the arrangement, so this test
        // records behaviour rather than demanding it.
        assert!(total > 0);
    }

    #[test]
    fn how_long_does_generation_take() {
        use crate::{Params, Tessellation};
        println!("
  world generation at the current defaults (jitter 0, relax 0):");
        for region_count in [32, 100, 250, 500, 1000] {
            let started = std::time::Instant::now();
            let t = Tessellation::generate(Params { region_count, ..Default::default() });
            println!("    {region_count:>5} regions: {:>8.1} ms   edges {}",
                started.elapsed().as_secs_f64() * 1000.0, t.edge_count());
        }
        println!();
    }

    /// Why are a region's sides different lengths, even on the soccer ball?
    ///
    /// Because this is a *Voronoi* soccer ball, not the Archimedean one. The
    /// Archimedean truncated icosahedron has all 90 edges equal, but its two kinds of
    /// face centre sit at different distances from the middle of the solid. A
    /// spherical Voronoi diagram only sees directions, so it cannot reproduce that
    /// difference, and the pentagon-hexagon borders come out a different length from
    /// the hexagon-hexagon ones.
    #[test]
    fn how_long_are_the_soccer_balls_borders() {
        use crate::adjacency::{adjacency, shared_border_length};
        let seeds = truncated_icosahedron_seeds();
        let neighbours = adjacency(&seeds);

        let mut pentagon_hexagon = Vec::new();
        let mut hexagon_hexagon = Vec::new();
        for first in 0..seeds.len() {
            for &second in &neighbours[first] {
                let second = second as usize;
                if second < first {
                    continue;
                }
                let length = shared_border_length(&seeds, first, second);
                let pentagons = (first < PENTAGON_COUNT) as usize
                    + (second < PENTAGON_COUNT) as usize;
                match pentagons {
                    1 => pentagon_hexagon.push(length),
                    0 => hexagon_hexagon.push(length),
                    _ => panic!("two pentagons should never share a border"),
                }
            }
        }

        let report = |name: &str, lengths: &[f64]| {
            let smallest = lengths.iter().cloned().fold(f64::INFINITY, f64::min);
            let largest = lengths.iter().cloned().fold(0.0, f64::max);
            println!(
                "  {name:<20} {:>3} borders   {:.4} to {:.4} rad   spread {:.4}%",
                lengths.len(),
                smallest,
                largest,
                100.0 * (largest / smallest - 1.0)
            );
            (smallest, largest)
        };
        println!("
  soccer ball (Voronoi of 32 directions):");
        let (pentagon_min, _) = report("pentagon-hexagon", &pentagon_hexagon);
        let (hexagon_min, _) = report("hexagon-hexagon", &hexagon_hexagon);
        println!(
            "  the two kinds differ by {:.1}%
",
            100.0 * (hexagon_min / pentagon_min - 1.0).abs()
        );

        assert_eq!(pentagon_hexagon.len(), 60);
        assert_eq!(hexagon_hexagon.len(), 30);

        // All ninety edges equal, which is what makes it the Archimedean solid rather
        // than merely something with the right combinatorics. Unit seeds cannot do this:
        // they gave 0.4575 against 0.3151, 45% apart.
        let longest = pentagon_hexagon
            .iter()
            .chain(&hexagon_hexagon)
            .cloned()
            .fold(0.0, f64::max);
        let shortest = pentagon_hexagon
            .iter()
            .chain(&hexagon_hexagon)
            .cloned()
            .fold(f64::INFINITY, f64::min);
        assert!(
            longest / shortest - 1.0 < 1e-6,
            "edges should all be equal: {shortest} to {longest}"
        );
        // Within each kind, every border is identical - the solid is that symmetric.
        for lengths in [&pentagon_hexagon, &hexagon_hexagon] {
            let smallest = lengths.iter().cloned().fold(f64::INFINITY, f64::min);
            let largest = lengths.iter().cloned().fold(0.0, f64::max);
            assert!(largest / smallest - 1.0 < 1e-9);
        }
    }

    /// And in a generated world, where the seeds are deliberately irregular.
    #[test]
    fn how_long_are_a_generated_worlds_borders() {
        use crate::adjacency::shared_border_length;
        use crate::{Params, Tessellation};
        println!("
  generated worlds, spread of side lengths within one region:");
        for relaxation in [0, 3, 8, 16, 40] {
            let tessellation = Tessellation::generate(Params {
                region_count: 32,
                jitter: 0.20,
                relaxation,
                seed: 1,
            });
            let mut worst = 0.0f64;
            let mut ratios = Vec::new();
            for region in 0..tessellation.region_count() {
                let lengths: Vec<f64> = tessellation.neighbours[region]
                    .iter()
                    .map(|&other| {
                        shared_border_length(&tessellation.seeds, region, other as usize)
                    })
                    .collect();
                let smallest = lengths.iter().cloned().fold(f64::INFINITY, f64::min);
                let largest = lengths.iter().cloned().fold(0.0, f64::max);
                let ratio = largest / smallest;
                ratios.push(ratio);
                worst = worst.max(ratio);
            }
            let mean = ratios.iter().sum::<f64>() / ratios.len() as f64;
            let mut sorted = ratios.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];
            let histogram = tessellation.degree_histogram();
            println!(
                "    relax {relaxation:>2}: longest/shortest side  median {median:.2}x  mean {mean:.2}x  worst {worst:.2}x   degrees {}",
                histogram
                    .iter()
                    .enumerate()
                    .filter(|&(_, &n)| n > 0)
                    .map(|(d, n)| format!("{d}:{n}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        println!();
    }

    /// With randomness off, generation is a bare Fibonacci lattice — and that is
    /// emphatically *not* a truncated icosahedron. Recording it here so the
    /// distinction is impossible to lose: 32 is the right face count, the degree
    /// deficit is still exactly 12, and the arrangement is still wrong.
    #[test]
    fn the_zeroed_defaults_do_not_produce_the_solid() {
        use crate::{Params, Tessellation};
        let defaults = Params::default();
        assert_eq!(defaults.region_count, 32);
        assert_eq!(defaults.jitter, 0.0);
        assert_eq!(defaults.relaxation, 0);

        let generated = Tessellation::generate(defaults);
        let histogram = generated.degree_histogram();

        // Squares and heptagons, not twelve pentagons and twenty hexagons.
        assert_eq!(histogram.get(4).copied().unwrap_or(0), 4);
        assert_eq!(histogram.get(7).copied().unwrap_or(0), 4);
        assert!(!generated.is_soccer_ball());
        assert!(!generated.verify_truncated_icosahedron().is_perfect());

        // Euler still holds, because it always does.
        let deficit: i64 = generated
            .neighbours
            .iter()
            .map(|list| 6 - list.len() as i64)
            .sum();
        assert_eq!(deficit, 12);

        // Perfection comes from construction, not from generation.
        assert!(Tessellation::soccer_ball().verify_truncated_icosahedron().is_perfect());
    }

    /// The confirmation itself: every measurable property of a truncated icosahedron,
    /// checked on the constructed solid.
    #[test]
    fn the_default_world_is_a_perfect_truncated_icosahedron() {
        use crate::Tessellation;
        let check = Tessellation::soccer_ball().verify_truncated_icosahedron();
        println!("\n  {}", check.summary());
        println!("    regions            {}", check.region_count);
        println!("    pentagons          {}", check.pentagons);
        println!("    hexagons           {}", check.hexagons);
        println!("    borders            {}", check.edges);
        println!("    pentagons isolated {}", check.pentagons_isolated);
        println!(
            "    every hexagon touches three pentagons  {}",
            check.hexagons_touch_three_pentagons
        );
        println!(
            "    border  pentagon-hexagon {:.9} rad   hexagon-hexagon {:.9} rad",
            check.pentagon_hexagon_border, check.hexagon_hexagon_border
        );
        println!(
            "    angle   pentagon-hexagon {:.9} rad   hexagon-hexagon {:.9} rad",
            check.pentagon_hexagon_angle, check.hexagon_hexagon_angle
        );
        println!(
            "    spread within a kind: borders {:.3e}  angles {:.3e}\n",
            check.border_spread, check.angle_spread
        );

        assert!(check.is_perfect(), "{check:?}");
        assert_eq!(check.region_count, 32);
        assert_eq!(check.pentagons, PENTAGON_COUNT);
        assert_eq!(check.hexagons, HEXAGON_COUNT);
        assert_eq!(check.edges, 90);
        assert!(check.pentagons_isolated);
        assert!(check.hexagons_touch_three_pentagons);
        assert!(check.border_spread < EXACT);
        assert!(check.angle_spread < EXACT);

        // One border length, not two. Seeds are `n / h`, so the pentagon and hexagon
        // face planes sit at their true distances and every edge comes out equal.
        assert!(
            (check.pentagon_hexagon_border / check.hexagon_hexagon_border - 1.0).abs() < 1e-6,
            "all ninety edges should be equal: {} against {}",
            check.pentagon_hexagon_border,
            check.hexagon_hexagon_border
        );
    }

    /// An irregular world must not be mistaken for the solid.
    #[test]
    fn verification_rejects_a_generated_world() {
        use crate::{Params, Tessellation};
        for jitter in [0.0, 0.2] {
            for relaxation in [0, 16] {
                let generated = Tessellation::generate(Params {
                    region_count: 32,
                    jitter,
                    relaxation,
                    seed: 1,
                });
                let check = generated.verify_truncated_icosahedron();
                assert!(
                    !check.is_perfect(),
                    "jitter {jitter} relax {relaxation} was accepted as perfect: {check:?}"
                );
            }
        }
    }

    /// The whole reason it is rejected for real worlds: it is perfectly symmetric, so
    /// every pentagon looks exactly like every other and their positions are
    /// predictable. Measured as every pentagon having an identical neighbourhood.
    #[test]
    fn the_symmetry_that_disqualifies_it_is_real() {
        let seeds = truncated_icosahedron_seeds();
        let neighbours = adjacency(&seeds);
        let mut spreads = Vec::new();
        for pentagon in 0..PENTAGON_COUNT {
            let mut angles: Vec<f64> = neighbours[pentagon]
                .iter()
                .map(|&other| seeds[pentagon].angle_to(seeds[other as usize]))
                .collect();
            angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
            spreads.push(angles);
        }
        for spread in &spreads {
            for pair in spread.windows(2) {
                assert!(
                    (pair[1] - pair[0]).abs() < 1e-9,
                    "a pentagon's neighbours were not all equidistant"
                );
            }
            assert!(
                (spread[0] - spreads[0][0]).abs() < 1e-9,
                "pentagons differ from each other, so this is not the regular solid"
            );
        }
    }
}
