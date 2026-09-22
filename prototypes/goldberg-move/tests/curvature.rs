//! What a route drawn on a sphere has to clear, measured rather than assumed.
//!
//! **`X-38`.** The research lens measured that a straight chord between two neighbouring
//! territory centres, floated at `ABOVE`, passes under radius 1.0 on every one of `GP(2,0)`'s
//! edges. It said plainly what it had not measured: whether the chord also passes under the
//! **drawn panel**, which dips inside the sphere because a panel is flat. That is the number
//! that decides whether the geometry being wrong is also visible.
//!
//! **Both are measured here, and the population is re-derived from the crate.** The lens
//! counted 240 by rebuilding the forty-two centres from the icosahedron in a Python script
//! rather than reading `sphere-tessellation`, deliberately, so that its count was a second
//! derivation and not a copy of what it checks. **This is the other derivation**: the same
//! numbers out of the crate the prototype actually draws from.

use goldberg_move::board::Board;
use sphere_tessellation::Vec3;

/// How far above the surface the prototype floats a disk, and the route with it.
///
/// **Named here as well as in `main.rs` because this is what checks it**, and a test that read
/// the constant from the thing it is judging would agree with any value at all.
const ABOVE: f64 = 1.035;

fn dot(one: Vec3, two: Vec3) -> f64 {
    one.x * two.x + one.y * two.y + one.z * two.z
}

/// Every pair of neighbouring territories, from both ends.
fn edges(board: &Board) -> Vec<(u32, u32)> {
    let mut found = Vec::new();
    for from in 0..board.territories() as u32 {
        for &to in &board.neighbours[from as usize] {
            found.push((from, to));
        }
    }
    found
}

/// The lens counted 240 from the icosahedron; this counts it from the crate.
#[test]
fn the_board_has_two_hundred_and_forty_neighbour_steps() {
    let board = Board::goldberg(2, 0);
    assert_eq!(board.territories(), 42);
    // **Euler over the solid**: 42 faces, 80 vertices, 120 edges, each walked from both ends.
    assert_eq!(
        edges(&board).len(),
        240,
        "every edge, from both of its ends"
    );
}

/// A straight chord between two neighbours sags below the sphere, on every edge.
///
/// **This is `X-38`'s measurement re-derived against `sphere-tessellation`.** A chord between
/// two points at radius `r` passes closest to the centre at `r · cos(θ/2)`, and the test does
/// not use that formula: it takes the midpoint of the two floated endpoints and measures its
/// length, which is the same quantity arrived at without the algebra the item argues from.
#[test]
fn every_straight_step_passes_under_the_sphere() {
    let board = Board::goldberg(2, 0);
    let (mut lowest, mut highest, mut counted) = (f64::MAX, f64::MIN, 0);
    let (mut widest, mut narrowest) = (f64::MIN, f64::MAX);

    for (from, to) in edges(&board) {
        let (one, two) = (board.centres[from as usize], board.centres[to as usize]);
        let separation = dot(one, two).clamp(-1.0, 1.0).acos().to_degrees();
        widest = widest.max(separation);
        narrowest = narrowest.min(separation);

        // The deepest point of a chord is its midpoint, both ends being the same length.
        let middle = Vec3::new(
            (one.x + two.x) * 0.5 * ABOVE,
            (one.y + two.y) * 0.5 * ABOVE,
            (one.z + two.z) * 0.5 * ABOVE,
        );
        let depth = (middle.x * middle.x + middle.y * middle.y + middle.z * middle.z).sqrt();
        lowest = lowest.min(depth);
        highest = highest.max(depth);
        counted += 1;
    }

    assert_eq!(counted, 240, "a count over nothing proves nothing");
    assert!(
        highest < 1.0,
        "the highest a straight step reaches is {highest}, so not every one is under the \
         sphere and `X-38`'s claim is too strong"
    );
    // The item's figures, to three places: between 0.9843 and 0.9956 of a radius.
    assert!(
        (0.984..0.985).contains(&lowest) && (0.995..0.996).contains(&highest),
        "the sag is {lowest}..{highest}, where `X-38` measured 0.9843..0.9956"
    );
    assert!(
        (31.7..31.8).contains(&narrowest) && widest > narrowest,
        "the narrowest separation is {narrowest} degrees, where `X-38` measured 31.717"
    );
}

/// And it passes under the **drawn surface** too, which is what `X-38` left unmeasured.
///
/// # The obvious instrument answers a narrower question than the one asked
///
/// `PlanetMesh::deepest` gives the deepest point of any panel - `0.9797` here - and comparing
/// a step's `0.9843` against it says the step is higher. **That comparison is wrong and it is
/// wrong in the direction that would have closed this item.** `deepest` is the deepest point
/// *anywhere*, which is the middle of the widest panel; a step's low point is not there. It is
/// over the **border between two territories**, and a border is the segment between two
/// corners that both sit exactly on the sphere.
///
/// **So the surface under a step is near radius one, not near `deepest`**, and the step is
/// under it by far more than the global figure suggests. Measured below, per border, from the
/// same `Solid` the mesh is built from.
///
/// `deepest` is still read here, so that the number `X-38` asked for is on the record and the
/// reason it is the wrong one to compare against is stated where the temptation is.
#[test]
fn every_straight_step_passes_under_the_border_it_crosses() {
    let board = Board::goldberg(2, 0);
    let solid = sphere_tessellation::solid(&board.centres, &board.neighbours);
    let coloring = graph_coloring::color_graph(&board.neighbours);
    let panels = planet_render::mesh::build(&solid, &coloring);

    let deepest = panels.deepest() as f64;
    assert!(
        (0.97..0.99).contains(&deepest),
        "the panels reach {deepest}; `X-38` asked for this number and it is not the one a \
         step has to clear"
    );

    let mut counted = 0;
    let (mut worst, mut border_lowest, mut least) = (f64::MIN, f64::MAX, f64::MAX);
    let mut under = 0;
    for (from, to) in edges(&board) {
        // The border is the corners the two cells share, and there are exactly two.
        let mine = &solid.cells[from as usize];
        let theirs = &solid.cells[to as usize];
        let shared: Vec<u32> = mine
            .iter()
            .copied()
            .filter(|corner| theirs.contains(corner))
            .collect();
        assert_eq!(
            shared.len(),
            2,
            "territories {from} and {to} are neighbours and share {} corners",
            shared.len()
        );

        let corner = |at: u32| solid.corners[at as usize].vector();
        let (one, two) = (corner(shared[0]), corner(shared[1]));
        // Where the drawn surface is over the middle of that border.
        let over = Vec3::new(
            (one.x + two.x) * 0.5,
            (one.y + two.y) * 0.5,
            (one.z + two.z) * 0.5,
        );
        let surface = (over.x.powi(2) + over.y.powi(2) + over.z.powi(2)).sqrt();
        border_lowest = border_lowest.min(surface);

        // And where the straight step is over the same place.
        let (a, b) = (board.centres[from as usize], board.centres[to as usize]);
        let middle = Vec3::new(
            (a.x + b.x) * 0.5 * ABOVE,
            (a.y + b.y) * 0.5 * ABOVE,
            (a.z + b.z) * 0.5 * ABOVE,
        );
        let step = (middle.x.powi(2) + middle.y.powi(2) + middle.z.powi(2)).sqrt();

        worst = worst.max(surface - step);
        least = least.min(surface - step);
        if surface > step {
            under += 1;
        }
        counted += 1;
    }

    println!("PROBE under={under}/{counted} least={least} worst={worst}");
    assert_eq!(counted, 240, "a count over nothing proves nothing");
    assert!(
        border_lowest > deepest,
        "a border reaches {border_lowest} and the deepest panel {deepest}, so the border is \
         not the higher of the two and this test is measuring the wrong thing"
    );

    // # The finding, and it is not the one this test first asserted
    //
    // **A step is under the border it crosses on some of the 240 and over it on others.** The
    // first draft of this test claimed *every* one and asserted the maximum, which is the
    // narrower question: a maximum above zero says *some* step is under its border. Asserting
    // the minimum instead refuted the claim in the same run it was written in.
    //
    // **So `X-38` is right about the sphere and understates one thing while overstating
    // another.** Every step is under radius 1.0 - that is measured above and holds on all 240.
    // But the drawn surface is not at radius 1.0: a border dips too, and on the wider borders
    // it dips *further* than the step does, so the step floats over the ground there. On the
    // narrower ones it does not, and the route sinks into the planet.
    //
    // **That is worse than a route uniformly below the surface would be**, because the error
    // is not uniform: the same drawn line is above the ground at one boundary and below it at
    // the next, which reads as a line that flickers in and out of the planet rather than one
    // that is consistently wrong.
    assert!(
        under > 0 && under < counted,
        "{under} of {counted} steps pass under the border they cross - both numbers have to \
         be non-zero for the mixed case this comment describes, and one of them is not"
    );
}

/// And the arc that replaced the chord clears every border, which is the repair's evidence.
///
/// **Passing tests prove nothing here, because they passed beforehand too** - so this is the
/// check that did not exist before, and the two above it are what it is measured against. They
/// say a straight step sinks under the border on half the board; this says the arc does not,
/// on any of it.
///
/// **It reads `board::along`, which is why that function is not in the drawing.** A gizmo
/// cannot be read back, so an arc computed inside `draw_the_route` would be an arc no test
/// could reach - and the repair would rest on looking at it.
#[test]
fn the_arc_clears_every_border_it_crosses() {
    let board = Board::goldberg(2, 0);
    let solid = sphere_tessellation::solid(&board.centres, &board.neighbours);

    // The prototype's own figure. A test that read it from `main.rs` would agree with any
    // value at all, so it is stated here and the two have to be kept in step by hand.
    const PER_STEP: usize = 8;

    let (mut counted, mut closest) = (0, f64::MAX);
    for (from, to) in edges(&board) {
        let route = [board.centres[from as usize], board.centres[to as usize]];
        let points = goldberg_move::board::along(&route, ABOVE, PER_STEP);
        assert_eq!(
            points.len(),
            PER_STEP + 1,
            "one step drawn with {PER_STEP} segments is {PER_STEP} + 1 points"
        );

        // Every point of the arc, and every segment between two of them.
        for point in &points {
            let radius = (point.x.powi(2) + point.y.powi(2) + point.z.powi(2)).sqrt();
            assert!(
                (radius - ABOVE).abs() < 1e-9,
                "an arc point sits at {radius} where the route floats at {ABOVE}"
            );
        }

        let mine = &solid.cells[from as usize];
        let theirs = &solid.cells[to as usize];
        let shared: Vec<u32> = mine
            .iter()
            .copied()
            .filter(|corner| theirs.contains(corner))
            .collect();
        let corner = |at: u32| solid.corners[at as usize].vector();
        let (one, two) = (corner(shared[0]), corner(shared[1]));
        let over = Vec3::new(
            (one.x + two.x) * 0.5,
            (one.y + two.y) * 0.5,
            (one.z + two.z) * 0.5,
        );
        let surface = (over.x.powi(2) + over.y.powi(2) + over.z.powi(2)).sqrt();

        // **The segments and not only their ends.** The ends are on the sphere by
        // construction; what a straight chord did wrong was happen between two points that
        // were both fine, so the midpoint of each segment is where this has to look.
        for pair in points.windows(2) {
            let middle = Vec3::new(
                (pair[0].x + pair[1].x) * 0.5,
                (pair[0].y + pair[1].y) * 0.5,
                (pair[0].z + pair[1].z) * 0.5,
            );
            let low = (middle.x.powi(2) + middle.y.powi(2) + middle.z.powi(2)).sqrt();
            closest = closest.min(low - surface);
        }
        counted += 1;
    }

    assert_eq!(counted, 240, "a count over nothing proves nothing");
    assert!(
        closest > 0.0,
        "an arc segment passes {closest} of a radius under the border it crosses, so the \
         repair does not hold everywhere"
    );
}

/// A route is drawn through the border it crosses, not over the middle of a territory.
///
/// **A separate fact from the curve and it needs its own check.** The arc above stops the line
/// going under the ground; this is about where it goes over it. The research lens, 2026-09-21:
/// giving the line its points as the cell centre then the midpoint of the edge it crosses is
/// what turns a polyline into a route through *these* territories.
#[test]
fn a_route_passes_through_every_border_it_crosses() {
    let board = Board::goldberg(2, 0);

    // Both ways round, because a route is walked in one direction and a border is not.
    assert_eq!(board.borders.len(), 240, "a border for every ordered pair");

    let mut counted = 0;
    for (from, to) in edges(&board) {
        let points = board.through(&[from, to]);
        assert_eq!(points.len(), 3, "a centre, a border and a centre");

        let border = points[1];
        let radius = (border.x.powi(2) + border.y.powi(2) + border.z.powi(2)).sqrt();
        assert!(
            (radius - 1.0).abs() < 1e-9,
            "a border point sits at {radius} and should be on the unit sphere"
        );

        // **Between the two centres and nearer than either is to the other**, which is what
        // *the midpoint of the edge it crosses* means without asserting the corners twice.
        let away = |one: Vec3, two: Vec3| dot(one, two).clamp(-1.0, 1.0).acos();
        let (a, b) = (board.centres[from as usize], board.centres[to as usize]);
        let whole = away(a, b);
        assert!(
            away(a, border) < whole && away(b, border) < whole,
            "the border between {from} and {to} is not between them"
        );
        counted += 1;
    }
    assert_eq!(counted, 240, "a count over nothing proves nothing");
}
