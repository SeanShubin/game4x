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

/// Reducing and turning a sixth commute, which is a fact about `reduce` and not about a shape.
///
/// # This replaces a test that could not fail, under a name that promised something impossible
///
/// **It was called `the_bright_region_has_the_six_fold_symmetry_of_the_grid`** and it rotated
/// every cell of the domain, reduced the result, and asserted the set came back. `X-34`:
/// `reduce` maps every cell to its canonical representative and the domain **is** the set of
/// canonical representatives, so the rotated-and-reduced set is that set whatever shape it
/// has. The sixth turn carries `(k, k)` to `(-k, 2k)` exactly and `(-k, 2k)` back into the
/// lattice, so it permutes cosets - verified, not assumed. **The assertion was about `reduce`
/// and was named for the region.**
///
/// **And the property it named is unavailable at every size.** A set symmetric under a sixth
/// turn about the origin is the origin plus whole orbits of six, so its size is `1 mod 6`. A
/// fundamental domain contains the origin, and `3k²` is `0 or 3 mod 6` at all ten sizes -
/// never `1`. So no implementation could have made that name true, which is worse than a test
/// that merely does not check: it is evidence for something that cannot hold, in a file whose
/// other tests are sound.
///
/// **The tie-break is not the defect.** Something has to break the ties on the boundary and
/// nothing symmetric is available, which is exactly what the arithmetic says.
///
/// Found by the research lane at `X-34`, while trying to work out why the picture looked
/// lopsided when a green test said otherwise. **It cost them the time and would have cost the
/// next reader more**, because `X-33` rests on the region's shape and this is the test they
/// would have consulted.
#[test]
fn reducing_and_turning_a_sixth_commute() {
    // Rotating a hex 60 degrees about the origin, in axial coordinates.
    let turn = |(q, r): (i32, i32)| (-r, q + r);
    let mut checked = 0;
    for torus in sizes() {
        // **The turn is an automorphism of the lattice**, which is what makes it permute
        // cosets - asserted here rather than left as the reason the assertion below is weak.
        let [a, b] = torus.generators();
        assert_eq!(
            turn(a),
            b,
            "k = {}: a sixth turn does not carry A to B",
            torus.k
        );
        assert_eq!(
            torus.reduce(turn(b).0, turn(b).1),
            (0, 0),
            "k = {}: a sixth turn takes B out of the lattice",
            torus.k
        );

        let domain = torus.domain();
        for cell in &domain {
            let turned = turn(*cell);
            assert_eq!(
                torus.reduce(turned.0, turned.1),
                torus.reduce(
                    turn(torus.reduce(cell.0, cell.1)).0,
                    turn(torus.reduce(cell.0, cell.1)).1
                ),
                "k = {}: reducing {cell:?} and turning it do not commute",
                torus.k
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 10, "ten sizes");
}

/// The drawn region is **not** six-fold symmetric, and no choice of tie-break could make it so.
///
/// **Asserted as the impossibility, so the claim cannot be re-added.** `X-34` found it stated
/// as a passing test; what stops that recurring is a check that fails if the region ever does
/// come back symmetric, with the arithmetic beside it.
#[test]
fn the_drawn_region_is_not_six_fold_symmetric_and_cannot_be() {
    let turn = |(q, r): (i32, i32)| (-r, q + r);
    let mut checked = 0;
    for torus in sizes() {
        // A symmetric set about the origin is the origin plus orbits of six, so `1 mod 6`.
        assert_ne!(
            torus.cells() % 6,
            1,
            "k = {}: {} cells is 1 mod 6, so a symmetric domain is arithmetically possible \
             after all and this test's reason has gone",
            torus.k,
            torus.cells()
        );

        let mut domain = torus.domain();
        domain.sort();
        let mut turned: Vec<(i32, i32)> = domain.iter().map(|cell| turn(*cell)).collect();
        turned.sort();
        assert_ne!(
            turned, domain,
            "k = {}: the drawn region is carried to itself by a sixth turn, which the `1 mod 6` \
             count says is impossible - so one of the two is wrong",
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
