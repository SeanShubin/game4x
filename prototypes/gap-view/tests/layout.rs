//! What the gap projection claims, asked of every territory of every planet size.
//!
//! **Each claim is checked over its whole population with the population's size asserted.**
//! A claim that rigid tiles never overlap is satisfied by there being no tiles, and this
//! repository has recorded that failure often enough to name it.

use gap_view::{
    Layout, Point, Tile, flatten, overlaps, predicted_gap, seam, snug_scale, total_defect_degrees,
};
use sphere_tessellation::{Direction, Params, Solid, Tessellation, Vec3, solid};

/// The five sizes `spec/planet.md` allows and the game uses.
const SIZES: [usize; 5] = [12, 32, 42, 72, 92];

fn world(region_count: usize) -> (Solid, Vec<Vec3>) {
    let tessellation = Tessellation::generate(Params {
        region_count,
        ..Params::default()
    });
    let built = solid(&tessellation.seeds, &tessellation.neighbours);
    (built, tessellation.seeds.clone())
}

/// A focus that is a territory's own centre, which is what the game would use: the map is
/// centred on where the player is looking.
fn focus_of(seeds: &[Vec3], territory: usize) -> Direction {
    Direction::of(seeds[territory])
}

fn every_layout() -> Vec<(usize, Layout)> {
    SIZES
        .iter()
        .map(|&size| {
            let (built, seeds) = world(size);
            let focus = focus_of(&seeds, size / 2);
            let scale = snug_scale(&built, &seeds, focus_of(&seeds, size / 2));
            (size, flatten(&built, &seeds, focus, scale))
        })
        .collect()
}

/// Whether two convex outlines overlap, by the separating axis theorem.
///
/// **Convex is safe to assume here**: the cells are the faces of a convex solid, laid down
/// rigidly, so a tile is the same convex polygon the sphere had.
fn overlap(left: &[Point], right: &[Point]) -> bool {
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
            // A shared edge touches without overlapping, and at the focus two tiles do
            // exactly that. The tolerance is what separates touching from crossing.
            if left_high <= right_low + 1e-9 || right_high <= left_low + 1e-9 {
                return false;
            }
        }
    }
    true
}

/// No tile lands on another, at any planet size, for any pair.
///
/// **This is the claim the whole idea rests on.** The projection has a radial scale factor of
/// exactly 1, so rings keep their true spacing and rigid tiles cannot collide. If that were
/// wrong the proposal would be unbuildable rather than merely ugly, so it is asked of every
/// pair rather than of a sample.
#[test]
fn no_tile_overlaps_another() {
    let mut pairs = 0;
    for (size, layout) in every_layout() {
        for left in 0..layout.tiles.len() {
            for right in (left + 1)..layout.tiles.len() {
                pairs += 1;
                assert!(
                    !overlap(&layout.tiles[left].outline, &layout.tiles[right].outline),
                    "on a {size}-territory planet, territories {left} and {right} overlap - \
                     the projection is not free of collisions after all"
                );
            }
        }
    }
    // 12*11/2 + 32*31/2 + 42*41/2 + 72*71/2 + 92*91/2
    assert_eq!(
        pairs,
        66 + 496 + 861 + 2556 + 4186,
        "every pair of every size was asked, and a claim about no pairs is not a claim"
    );
}

/// Every territory is drawn at the size and shape it has on the sphere.
///
/// **This is the other half of the proposal**, and the one a picture cannot show: a tile that
/// had quietly been scaled would still look plausible. Each corner keeps its true distance
/// from its own centre, so that is what is compared.
#[test]
fn every_tile_keeps_its_shape() {
    let mut checked = 0;
    let mut worst: f64 = 0.0;
    for &size in &SIZES {
        let (built, seeds) = world(size);
        let scale = snug_scale(&built, &seeds, focus_of(&seeds, size / 2));
        let layout = flatten(&built, &seeds, focus_of(&seeds, size / 2), scale);
        for tile in &layout.tiles {
            let centre = Direction::of(seeds[tile.territory]);
            let corners = &built.cells[tile.territory];
            assert_eq!(
                corners.len(),
                tile.outline.len(),
                "territory {} lost a corner on the way to the map",
                tile.territory
            );
            for (at, &corner) in corners.iter().enumerate() {
                let on_sphere = centre.angle_to(built.corners[corner as usize]) * scale;
                let on_map = tile.outline[at].distance_to(tile.centre);
                worst = worst.max((on_sphere - on_map).abs());
                assert!(
                    (on_sphere - on_map).abs() < 1e-9,
                    "on a {size}-territory planet, a corner of territory {} sits {on_map} \
                     from its centre on the map and {on_sphere} on the sphere",
                    tile.territory
                );
                checked += 1;
            }
        }
    }
    // A Goldberg solid of F faces has twelve pentagons and the rest hexagons, so its
    // corners number `6F - 12`. Summed over the five sizes: 6 * 250 - 60.
    assert_eq!(
        checked,
        6 * (12 + 32 + 42 + 72 + 92) - 12 * 5,
        "every corner of every territory of every size was asked; a world of F territories \
         has 6F - 12 of them"
    );
    assert!(worst < 1e-9, "the worst corner was out by {worst}");
}

/// Distance from the focus is exact, and so is bearing.
///
/// **This is what lets a player navigate without rotating.** If a territory is three hops
/// north-east on the sphere it is three hops north-east on the map, whatever the gaps are
/// doing, so the map can be read for direction as well as for adjacency.
#[test]
fn distance_and_bearing_from_the_focus_survive() {
    let mut checked = 0;
    for &size in &SIZES {
        let (built, seeds) = world(size);
        let focus = focus_of(&seeds, size / 2);
        let layout = flatten(
            &built,
            &seeds,
            focus,
            snug_scale(&built, &seeds, focus_of(&seeds, size / 2)),
        );
        for tile in &layout.tiles {
            let on_sphere = focus.angle_to(Direction::of(seeds[tile.territory]));
            let on_map = tile.centre.distance_to(Point { x: 0.0, y: 0.0 });
            // `acos` of a dot product close to one loses half the mantissa, so the focus's
            // own tile is out by about 1e-8 and nothing else is. That is the arithmetic
            // rather than the projection, and a tolerance below it would be a check on the
            // floating point library.
            assert!(
                (on_sphere - on_map).abs() < 1e-6,
                "on a {size}-territory planet, territory {} is {on_sphere} from the focus on \
                 the sphere and {on_map} on the map",
                tile.territory
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked,
        12 + 32 + 42 + 72 + 92,
        "every territory of every size was asked"
    );
}

/// The formula bounds the gap the layout opens between two neighbours, and is reached.
///
/// **Measured between neighbours rather than by area.** The first version summed the area of
/// every tile whose centre sat inside a disc and compared that with the disc - which counts
/// the whole of a tile hanging over the edge, and reported a *negative* gap out to thirty
/// degrees. The claim is about the space between two territories, so that is what is
/// measured: how much further apart two neighbours are on the map than on the sphere.
///
/// **Nothing ever comes closer**, which is the half that matters: it is why rigid tiles can
/// be laid down at all, and it is asserted over every adjacent pair rather than on average.
#[test]
fn the_gap_between_neighbours_is_bounded_by_the_formula_and_reaches_it() {
    let region_count = 92;
    let tessellation = Tessellation::generate(Params {
        region_count,
        ..Params::default()
    });
    let built = solid(&tessellation.seeds, &tessellation.neighbours);
    let seeds = &tessellation.seeds;
    let focus = focus_of(seeds, region_count / 2);
    let layout = flatten(&built, seeds, focus, 1.0);

    // **The formula is a bound, not a prediction for every pair.** `theta / sin(theta)` is
    // the scale factor *across* the line to the focus; along that line the scale is exactly
    // 1. So two neighbours lying radially open by nothing however far out they are, and two
    // lying across the line open by the full figure. Asserting the figure of every pair is
    // what the first version of this did, and a border past 120 degrees opening by 9 per
    // cent against a figure of 142 is the map behaving correctly.
    let mut pairs = 0;
    let mut worst_below: f64 = 0.0;
    let mut worst_above: f64 = 0.0;
    let mut attained: f64 = 0.0;
    let mut in_the_band = 0;
    let band = (50.0_f64.to_radians(), 70.0_f64.to_radians());

    for (territory, neighbours) in tessellation.neighbours.iter().enumerate() {
        for &neighbour in neighbours {
            let neighbour = neighbour as usize;
            if neighbour < territory {
                continue;
            }
            pairs += 1;

            let on_sphere =
                Direction::of(seeds[territory]).angle_to(Direction::of(seeds[neighbour]));
            let here = layout.tile(territory);
            let there = layout.tile(neighbour);
            let on_map = here.centre.distance_to(there.centre);
            let opened = on_map / on_sphere - 1.0;

            // Never closer. This is what makes rigid tiles safe to place, and it holds
            // everywhere including out at the seam.
            worst_below = worst_below.max((on_sphere - on_map).max(0.0));

            // Never more than the tangential figure at the further of the two, because the
            // map cannot stretch anything by more than its largest scale factor along the
            // way.
            let furthest = here.from_focus.max(there.from_focus);
            worst_above = worst_above.max(opened - predicted_gap(furthest));

            let midpoint = (here.from_focus + there.from_focus) / 2.0;
            if midpoint >= band.0 && midpoint <= band.1 {
                attained = attained.max(opened);
                in_the_band += 1;
            }
        }
    }

    assert_eq!(
        pairs,
        3 * region_count - 6,
        "a world of F territories has 3F - 6 borders, and every one was asked"
    );
    assert!(
        worst_below < 1e-9,
        "a pair of neighbours ended up {worst_below} closer on the map than on the sphere, \
         so the projection does compress after all"
    );
    assert!(
        worst_above < 1e-9,
        "a pair opened by {worst_above} more than the figure at its own distance, and the \
         figure is supposed to bound it"
    );

    // **And the bound is reached, or it would be describing nothing.** Sixty degrees out the
    // figure is 21 per cent, and the borders lying across the line to the focus should be
    // near it.
    assert!(
        in_the_band > 10,
        "only {in_the_band} borders sit between 50 and 70 degrees, too few for the next \
         assertion to be about the map"
    );
    let figure = predicted_gap(60.0_f64.to_radians());
    assert!(
        attained > figure * 0.7,
        "the widest a border between 50 and 70 degrees opened was {attained}, against a \
         figure of {figure} - so nothing reaches the bound and it describes nothing"
    );

    // The formula itself, at the two ends that matter: nothing at the focus, and more gap
    // than territory by the time the map reaches half the world.
    assert!(predicted_gap(0.0) < 1e-9);
    assert!((predicted_gap(std::f64::consts::FRAC_PI_2) - 0.5708).abs() < 1e-3);
}

/// The seam is one territory, and it is the antipode of the focus.
///
/// **The cost of the projection is a place rather than a line**, which is what makes it worth
/// preferring: a caller choosing the focus is choosing which single territory reads wrongly.
#[test]
fn the_only_broken_territory_is_the_antipode() {
    for &size in &SIZES {
        let (built, seeds) = world(size);
        let focus = focus_of(&seeds, size / 2);
        let layout = flatten(
            &built,
            &seeds,
            focus,
            snug_scale(&built, &seeds, focus_of(&seeds, size / 2)),
        );

        let seam = seam(&layout);
        let seam_distance = focus.angle_to(Direction::of(seeds[seam]));
        for tile in &layout.tiles {
            let distance = focus.angle_to(Direction::of(seeds[tile.territory]));
            assert!(
                distance <= seam_distance + 1e-12,
                "territory {} is further from the focus than the seam is",
                tile.territory
            );
        }
        // Every other territory is inside the disc rather than on its rim, so the
        // degeneracy really is confined to one.
        let inside = layout
            .tiles
            .iter()
            .filter(|tile| tile.from_focus < seam_distance - 1e-12)
            .count();
        assert_eq!(
            inside,
            size - 1,
            "on a {size}-territory planet, {inside} territories are inside the seam and \
             {} are on it",
            size - inside
        );
    }
}

/// The whole world is on the map, so nothing has to be rotated to.
///
/// **This is the question the prototype exists for**, reduced to the part a test can hold:
/// every territory is drawn, and every one is within the disc of radius pi. Whether choosing
/// one is a good gesture is a judgement, and is Sean's.
#[test]
fn every_territory_is_on_the_map() {
    for (size, layout) in every_layout() {
        assert_eq!(
            layout.tiles.len(),
            size,
            "a {size}-territory planet drew {} tiles",
            layout.tiles.len()
        );
        for tile in &layout.tiles {
            assert!(
                tile.from_focus <= std::f64::consts::PI + 1e-12,
                "territory {} is off the map at {} radians",
                tile.territory,
                tile.from_focus
            );
        }
    }
}

/// Descartes: the tax is 720 degrees whatever the planet, so a layout only chooses where.
///
/// **By way of Euler, because that is the part the geometry can be asked.** The first version
/// of this summed the face angles at every corner and expected 720 to a millionth. It got
/// 733.7, and the excess was real: these cells are *spherical* polygons whose corners are not
/// coplanar, so a face angle measured between chords is not the face angle Descartes' theorem
/// is about. The number was measuring how far from flat a territory is.
///
/// So what is computed here is `chi`, which is exact and integral, and the defect follows
/// from it. **That the solid closes is also the thing worth checking**: if it did not, every
/// other claim in this file would be about something that is not a planet.
#[test]
fn the_total_defect_is_720_degrees_at_every_size() {
    let mut checked = 0;
    for &size in &SIZES {
        let (built, _) = world(size);
        assert_eq!(
            built.euler_characteristic(),
            2,
            "a {size}-territory planet does not close up, so it is not a sphere's division"
        );
        assert!(
            (total_defect_degrees(&built) - 720.0).abs() < 1e-12,
            "a {size}-territory planet owes {} degrees and every closed solid owes 720",
            total_defect_degrees(&built)
        );
        checked += 1;
    }
    assert_eq!(checked, SIZES.len(), "every size was asked");
}

/// A tile is a real polygon with real area, so the measurements above measure something.
///
/// **A zero-area tile would satisfy every claim above.** Nothing overlaps, every shape is
/// preserved, every distance survives - all true of nothing at all, which is why this is
/// here.
#[test]
fn a_tile_has_area() {
    let (built, seeds) = world(42);
    let layout = flatten(&built, &seeds, focus_of(&seeds, 21), 1.0);
    let total: f64 = layout.tiles.iter().map(Tile::area).sum();
    // The sphere's area is 4 pi in these units, and rigid tiles carry all of it apart from
    // what flattening each cell adds.
    assert!(
        (total - 4.0 * std::f64::consts::PI).abs() < 0.2,
        "the tiles carry {total} of area and the sphere has {}",
        4.0 * std::f64::consts::PI
    );
    assert!(
        layout.tiles.iter().all(|tile| tile.area() > 0.0),
        "a territory was drawn with no area"
    );
}

/// The gap flattening forces is under one per cent, and at one size it is nothing at all.
///
/// **This began as a claim that it is never zero, and that was wrong.** Flattening a
/// spherical polygon at its true radii does make it bigger than it was, so two territories
/// that touch *along the line to the focus* cross - but whether any pair lies close enough to
/// that line is an accident of where the seeds fell. A twelve-territory planet has no such
/// pair and needs no gap; a ninety-two-territory planet samples the worst case better and
/// needs half a per cent.
///
/// **The useful half of the finding survives the correction and is the better news**: the gap
/// that flattening forces is far smaller than any gap chosen for looks, so choosing one for
/// looks costs nothing.
#[test]
fn the_gap_flattening_forces_is_under_one_per_cent() {
    let mut reported = 0;
    let mut ever_forced = false;
    for &size in &SIZES {
        let (built, seeds) = world(size);
        let focus = focus_of(&seeds, size / 2);
        let scale = snug_scale(&built, &seeds, focus);
        assert!(
            scale > 0.99,
            "a {size}-territory planet needs its territories shrunk to {scale}, which is far \
             more than flattening a cell should cost"
        );
        assert!(
            scale <= 1.0,
            "a scale above one would be a magnified territory"
        );

        if scale < 1.0 {
            ever_forced = true;
            // **Tight as well as safe.** The search returns a scale nothing crosses at; this
            // is what says it is not merely a small number - a little above it, something
            // does cross. Without this the search could return anything conservative and
            // every assertion above would still pass.
            let layout = flatten(&built, &seeds, focus, scale + 0.01);
            let mut crossed = false;
            for left in 0..layout.tiles.len() {
                for right in (left + 1)..layout.tiles.len() {
                    if overlaps(&layout.tiles[left].outline, &layout.tiles[right].outline) {
                        crossed = true;
                    }
                }
            }
            assert!(
                crossed,
                "on a {size}-territory planet nothing crosses at one per cent above the \
                 snug scale, so the search is returning something looser than it claims"
            );
        }
        reported += 1;
    }
    assert_eq!(reported, SIZES.len(), "every size was asked");
    assert!(
        ever_forced,
        "no planet size forces any gap at all, which would mean this whole effect is absent \
         rather than small"
    );
}
