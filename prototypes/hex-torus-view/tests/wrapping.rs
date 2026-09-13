//! The check `X-32` asks for, over all ten sizes with the count asserted.
//!
//! **A person looking is the vetted-when and this is not a substitute for it.** What it
//! catches is the thing a picture cannot be trusted about: whether *exactly N bright* is true
//! or merely looks true. `X-32` names three cases in this repository of a check going green
//! over a population of one, which is why every assertion here is followed by how many cases
//! there were.

use hex_torus_view::{NEIGHBOURS, norm, sizes};

/// Exactly `N` cells are bright, and every cell in the plane is an echo of exactly one.
///
/// **Both halves, because either alone is satisfied by something wrong.** A domain of the
/// right size proves nothing if some cell reduces outside it, and every cell reducing into the
/// domain proves nothing if the domain is the whole plane.
#[test]
fn exactly_n_cells_are_bright_and_every_cell_echoes_one_of_them() {
    let mut checked = 0;
    for torus in sizes() {
        let domain = torus.domain();
        assert_eq!(
            domain.len(),
            torus.cells(),
            "k = {}: the fundamental domain holds {} cells and the family says 3k^2 = {}",
            torus.k,
            domain.len(),
            torus.cells()
        );

        // **Every cell of three whole periods**, which is more than the viewer draws, reduces
        // into the domain - and a cell already in the domain reduces to itself, which is what
        // makes *bright* a property of the cell rather than of the order they were visited in.
        let reach = 3 * torus.k;
        let mut seen = 0;
        for q in -reach..=reach {
            for r in -reach..=reach {
                let cell = torus.reduce(q, r);
                assert!(
                    domain.contains(&cell),
                    "k = {}: ({q}, {r}) reduces to {cell:?}, which is not a bright cell",
                    torus.k
                );
                assert_eq!(
                    torus.reduce(cell.0, cell.1),
                    cell,
                    "k = {}: reducing {cell:?} again moves it, so the domain is not fixed",
                    torus.k
                );
                assert_eq!(
                    torus.is_canonical(q, r),
                    (q, r) == cell,
                    "k = {}: ({q}, {r}) disagrees with itself about being bright",
                    torus.k
                );
                seen += 1;
            }
        }
        assert_eq!(
            seen,
            (2 * reach + 1) * (2 * reach + 1),
            "k = {}: the sweep skipped a cell",
            torus.k
        );
        checked += 1;
    }
    assert_eq!(
        checked, 10,
        "ten sizes, and the count so one cannot become ten"
    );
}

/// The bright region is a hexagon, which is what makes the six-fold symmetry visible.
///
/// **Asserted as symmetry rather than as a shape**, because a shape written here would be the
/// domain stated twice and the second copy is what rots. A Voronoi cell of this lattice is
/// symmetric under 60-degree rotation, so rotating every bright cell gives the bright set
/// back - which a rhombus or a rectangle would not do.
#[test]
fn the_bright_region_has_the_six_fold_symmetry_of_the_grid() {
    // Rotating a hex 60 degrees about the origin, in axial coordinates.
    let turn = |(q, r): (i32, i32)| (-r, q + r);
    let mut checked = 0;
    for torus in sizes() {
        let domain = torus.domain();
        let rotated: Vec<(i32, i32)> = domain
            .iter()
            .map(|cell| torus.reduce(turn(*cell).0, turn(*cell).1))
            .collect();
        let mut sorted = rotated.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            domain.len(),
            "k = {}: rotating the bright set sends two cells to one",
            torus.k
        );
        let mut want = domain.clone();
        want.sort();
        assert_eq!(
            sorted, want,
            "k = {}: the bright set is not carried to itself by a sixth turn, so it is not the \
             hexagonal domain",
            torus.k
        );
        checked += 1;
    }
    assert_eq!(checked, 10, "ten sizes");
}

/// All six directions circumnavigate in the same number of steps, and no fewer.
///
/// **This is the definition of the family and the thing a wrong generator breaks quietly.**
/// The other coordinate convention's generator for the same written formula gives `7k^2` cells
/// and six circumnavigations that do not agree - a picture that looks almost right.
#[test]
fn every_direction_wraps_in_the_same_number_of_steps() {
    let mut checked = 0;
    for torus in sizes() {
        assert!(
            torus.circumnavigations_agree(),
            "k = {}: the six directions do not all wrap at {} steps",
            torus.k,
            torus.circumference()
        );
        // And the circumference is the one the table gives, rather than whatever came out.
        assert_eq!(torus.circumference(), 3 * torus.k);
        checked += 1;
    }
    assert_eq!(checked, 10, "ten sizes");
    assert_eq!(
        sizes().iter().map(|t| t.cells()).collect::<Vec<_>>(),
        vec![12, 27, 48, 75, 108, 147, 192, 243, 300, 363],
        "the ten sizes are the ones `X-32` names, and Sean chose the list"
    );
}

/// Every cell has six distinct neighbours, which a torus has and a patch does not.
///
/// **The wrap is what makes this true at the edge**, so a domain that failed to wrap would
/// show up here as a cell neighbouring itself or as five neighbours rather than six.
#[test]
fn every_cell_has_six_distinct_neighbours() {
    let mut checked = 0;
    for torus in sizes() {
        for (at, neighbours) in torus.adjacency().iter().enumerate() {
            let mut sorted = neighbours.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(
                sorted.len(),
                6,
                "k = {}: cell {at} has {:?} for neighbours",
                torus.k,
                neighbours
            );
            assert!(
                !neighbours.contains(&(at as u32)),
                "k = {}: cell {at} is its own neighbour, so the wrap has folded it onto itself",
                torus.k
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked,
        sizes().iter().map(|t| t.cells()).sum::<usize>(),
        "every cell of every size"
    );
}

/// The neighbour step is one step, which is what `norm` and `NEIGHBOURS` have to agree about.
#[test]
fn a_neighbour_is_one_step_away() {
    for (q, r) in NEIGHBOURS {
        assert_eq!(norm(q, r), 1, "({q}, {r}) is not one step from the origin");
    }
    assert_eq!(norm(0, 0), 0);
    assert_eq!(norm(2, -1), 2, "two steps out and one back is two");
    assert_eq!(NEIGHBOURS.len(), 6);
}
