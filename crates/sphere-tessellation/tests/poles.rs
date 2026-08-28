//! Where the poles fall on the solid.
//!
//! `spec/planet.md`: the north and south poles are at the centres of two pentagons, never
//! on a boundary between territories.
//!
//! This is a constraint on orientation, not on the graph. It matters because
//! `spec/planet.md` also fixes the roll for any point, which forces two places where the
//! camera convention breaks down, and asks for the poles to be visible. Putting them at
//! face centres means those two places are inside territories that have ids, so "what is
//! at the north pole" has an answer, and crossing a pole passes through ground rather than
//! along a seam belonging to two territories at once.

use sphere_tessellation::{Direction, Vec3, adjacency, goldberg, icosahedral};

/// How far the second-nearest face must be, beyond the nearest, for the pole to count as
/// strictly inside one.
///
/// A tolerance rather than an equality: the nearest face being closer than the next by
/// *some* margin is the thing being asserted, and the margin shrinks as territories do.
/// Even at 492 territories it is far larger than this.
const CLEAR_BY: f64 = 1e-6;

/// The face a direction falls in, and how much closer it is than the next.
fn face_under(seeds: &[Vec3], point: Direction) -> (usize, f64) {
    let mut sorted: Vec<(f64, usize)> = seeds
        .iter()
        .enumerate()
        .map(|(at, seed)| (point.vector().angle_to(*seed), at))
        .collect();
    sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("angles are finite"));
    (sorted[0].1, sorted[1].0 - sorted[0].0)
}

/// Every planet size, and every Goldberg arrangement small enough to check quickly -
/// which includes the chiral ones, where the pentagons stay put and only the hexagons
/// between them are rearranged.
fn arrangements() -> Vec<(usize, usize)> {
    goldberg::arrangements_up_to(200)
}

#[test]
fn both_poles_sit_at_the_centre_of_a_pentagon() {
    for (m, n) in arrangements() {
        let seeds = goldberg::seeds(m, n);
        let neighbours = adjacency(&seeds);

        for pole in Direction::poles() {
            let (face, clearance) = face_under(&seeds, pole);
            assert_eq!(
                neighbours[face].len(),
                5,
                "GP({m},{n}): a pole is on territory {} which has {} neighbours",
                face + 1,
                neighbours[face].len()
            );
            assert!(
                clearance > CLEAR_BY,
                "GP({m},{n}): a pole is {clearance} from the next face, which is a boundary"
            );
            // And it is not merely inside the face but exactly at its centre.
            assert!(
                pole.vector().angle_to(seeds[face]) < 1e-9,
                "GP({m},{n}): the pole is inside a pentagon but not at its centre"
            );
        }
    }
}

/// The assertion that catches an off-by-one in choosing the axis: it is easy to put one
/// pole on a pentagon and leave the other wherever it happened to land.
#[test]
fn the_two_poles_are_on_different_pentagons_facing_each_other() {
    for (m, n) in arrangements() {
        let seeds = goldberg::seeds(m, n);
        let (north, _) = face_under(&seeds, Direction::NORTH_POLE);
        let (south, _) = face_under(&seeds, Direction::SOUTH_POLE);
        assert_ne!(north, south, "GP({m},{n}): both poles on one territory");

        let apart = Vec3::angle_to(seeds[north], seeds[south]);
        assert!(
            (apart - std::f64::consts::PI).abs() < 1e-9,
            "GP({m},{n}): the pole territories are {apart} apart, not opposite"
        );
    }
}

/// The exact solid built for 32 territories is a separate construction - its seeds carry
/// a plane distance so that all ninety edges come out equal - so it needs its own check
/// rather than inheriting one.
#[test]
fn the_truncated_icosahedron_stands_on_its_poles_too() {
    let seeds = icosahedral::truncated_icosahedron_seeds();
    let neighbours = adjacency(&seeds);
    for pole in Direction::poles() {
        let (face, clearance) = face_under(&seeds, pole);
        assert_eq!(neighbours[face].len(), 5, "a pole is not on a pentagon");
        assert!(clearance > CLEAR_BY, "a pole is on a boundary");
    }
}

/// Standing the solid up is a rigid motion, so it moves every territory together and
/// changes nothing about which touches which. If this ever failed, the orientation would
/// have started renumbering the world.
#[test]
fn standing_the_solid_up_leaves_every_neighbour_where_it_was() {
    // A dodecahedron's faces each touch five others, and territory 1's neighbours are
    // recorded in the console's own fixture test. Adjacency is a fact about the graph and
    // must be unaffected by which way the thing is pointing.
    for (m, n) in arrangements() {
        let seeds = goldberg::seeds(m, n);
        let neighbours = adjacency(&seeds);
        for (at, near) in neighbours.iter().enumerate() {
            assert!(
                near.len() == 5 || near.len() == 6,
                "GP({m},{n}): territory {} has {} neighbours",
                at + 1,
                near.len()
            );
            for other in near {
                assert!(
                    neighbours[*other as usize].contains(&(at as u32)),
                    "GP({m},{n}): adjacency disagrees with itself"
                );
            }
        }
    }
}
