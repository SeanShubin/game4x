//! The check `X-32` asks for, over every case with the count asserted.
//!
//! **The population is per check and is stated by each.** Two of these run over both families,
//! twenty worlds; the rest are properties of the `3k²` lattice and run over its ten. The
//! axis-aligned family's own properties are in `tests/families.rs`. The two that were widened
//! were narrow only because they were written when there was one family - `Q-87` is the same
//! shape in the README, and neither was found by anything failing.
//!
//! **A person looking is the vetted-when and this is not a substitute for it.** What it
//! catches is the thing a picture cannot be trusted about: whether *exactly N bright* is true
//! or merely looks true. `X-32` names three cases in this repository of a check going green
//! over a population of one, which is why every assertion here is followed by how many cases
//! there were.

use hex_torus_view::{NEIGHBOURS, both_families, norm, sizes};

/// Exactly `N` cells are bright, and every cell in the plane is an echo of exactly one.
///
/// **Both halves, because either alone is satisfied by something wrong.** A domain of the
/// right size proves nothing if some cell reduces outside it, and every cell reducing into the
/// domain proves nothing if the domain is the whole plane.
#[test]
fn exactly_n_cells_are_bright_and_every_cell_echoes_one_of_them() {
    let mut checked = 0;
    // **Both families, because nothing here is about which lattice it is.** It ran over the
    // folded ten for as long as there was one family, and the axis-aligned family landed under
    // it without the loop being re-read - the same shape as `Q-87`, one file over. The quality
    // lens found the narrowness, poisoned it, and withdrew the finding because the drawing
    // tests already redden; the population is widened here anyway, because *covered somewhere
    // else* and *checked here* are different claims and only one of them is this file's.
    for torus in both_families() {
        let domain = torus.domain();
        assert_eq!(
            domain.len(),
            torus.cells(),
            "{:?} k = {}: the fundamental domain holds {} cells and the family says {}",
            torus.family,
            torus.k,
            domain.len(),
            torus.cells()
        );

        // **Every cell of three whole periods**, which is more than the viewer draws, reduces
        // into the domain - and a cell already in the domain reduces to itself, which is what
        // makes *bright* a property of the cell rather than of the order they were visited in.
        let reach = 3 * torus.k;
        let mut seen = 0;
        let mut reached: std::collections::BTreeSet<(i32, i32)> = std::collections::BTreeSet::new();
        for q in -reach..=reach {
            for r in -reach..=reach {
                let cell = torus.reduce(q, r);
                reached.insert(cell);
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
            "{:?} k = {}: the sweep skipped a cell",
            torus.family,
            torus.k
        );

        // **And the other direction: every cell of the domain is reached.** The two halves
        // above are *the domain is the right size* and *nothing reduces outside it*, and
        // together they still allow a domain cell that nothing ever reduces to - a bright hex
        // that is drawn and is not any territory.
        //
        // **For the folded family this could not fail**, because `domain` is built by
        // reducing; for the axis-aligned family it can, because `domain` is the square
        // `0..C x 0..C` written down independently of `reduce`. **So the missing direction was
        // not merely untested - it was vacuous over half the population**, and read as complete
        // because the docstring above says it checks both halves.
        //
        // Found by poisoning the axis-aligned reduction to `r mod (C-1)` while widening this
        // test to both families. **Four tests reddened elsewhere and this one stayed green** -
        // three in `tests/families.rs` and `every_cell_has_six_distinct_neighbours` here -
        // while `tests/drawing.rs` and `tests/colouring.rs` stayed green, which is what says
        // this assertion reaches something nothing else does.
        //
        // **That count was first written as six and sourced from nothing.** The quality lens
        // had measured five under a different poison; six was neither number, and it reached a
        // commit message, this comment and the README before anyone applied the poison and
        // counted. Re-measured at `5221933`.
        assert_eq!(
            reached.len(),
            domain.len(),
            "{:?} k = {}: {} cells are reduced to and the domain has {} - a bright hex that is \
             no territory",
            torus.family,
            torus.k,
            reached.len(),
            domain.len()
        );
        for cell in &domain {
            assert!(
                reached.contains(cell),
                "{:?} k = {}: {cell:?} is drawn bright and nothing reduces to it",
                torus.family,
                torus.k
            );
        }
        checked += 1;
    }
    assert_eq!(
        checked, 20,
        "both families, ten sizes each, and the count so one cannot become twenty"
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

/// Leaving the region subtracts one of six vectors, each the same number of times.
///
/// **This is why the wrapping is learnable, and it is the reason the region is a hexagon.**
/// A step that leaves the drawn region comes back translated by `±a`, `±b` or `±(a−b)` - the
/// six shortest lattice vectors, all of norm `2k` - and each is used exactly `2k` times. Three
/// opposite edge-pairs, each a constant translation, the same everywhere along its edge.
///
/// # The reason the hexagon was chosen did not exist, and this is the one that does
///
/// `X-32` specified a hexagonal domain for its six-fold symmetry, which `X-34` then proved
/// unavailable at every size. **Right answer, wrong reason.** The reason measured afterwards:
/// a rhombic domain over the same lattice subtracts `±a`, `±b`, `±(a+b)` instead, uses its
/// diagonal pair **once** and the other four `4k−1` times each, and wraps `(16k−2)/18k²`
/// against the hexagon's `2/(3k)` - about a third more often at every size.
///
/// So the hexagon wraps less and wraps evenly, and the rhombus has two seams a player would
/// meet once in a game and four they would meet constantly.
///
/// **`X-33` first reported this as *no single wrap rule, nothing for a player to learn*, from
/// id offsets** - indices into a list sorted by position, which say nothing about geometry.
/// Both lanes repeated it before anybody measured it. What is true is the opposite: six
/// constant translations, and the only reason the seam is hard to see is that the region is a
/// blob rather than that the rule varies.
#[test]
fn leaving_the_region_subtracts_one_of_six_vectors_evenly() {
    let mut checked = 0;
    for torus in sizes() {
        let domain = torus.domain();
        assert_eq!(
            domain.len(),
            torus.cells(),
            "k = {}: the population, first, because a count that does not state one is how \
             three wrong numbers got out of this prototype in one evening",
            torus.k
        );
        let [a, b] = torus.generators();
        let mut shifts: std::collections::BTreeMap<(i32, i32), usize> =
            std::collections::BTreeMap::new();
        for (q, r) in &domain {
            for (dq, dr) in NEIGHBOURS {
                let (tq, tr) = (q + dq, r + dr);
                let back = torus.reduce(tq, tr);
                let shift = (tq - back.0, tr - back.1);
                if shift != (0, 0) {
                    *shifts.entry(shift).or_default() += 1;
                }
            }
        }

        let want: std::collections::BTreeSet<(i32, i32)> = [
            a,
            (-a.0, -a.1),
            b,
            (-b.0, -b.1),
            (a.0 - b.0, a.1 - b.1),
            (b.0 - a.0, b.1 - a.1),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            shifts
                .keys()
                .copied()
                .collect::<std::collections::BTreeSet<_>>(),
            want,
            "k = {}: the wrap does not subtract the six shortest lattice vectors",
            torus.k
        );
        // **`a + b` is not among them and is not one of the shortest** - it has norm `3k`
        // where these six have `2k`. Named because it is the vector the first account of this
        // said was used, and nothing else here would notice its absence.
        assert!(
            !shifts.contains_key(&(a.0 + b.0, a.1 + b.1)),
            "k = {}: the wrap subtracts a+b, which is the rhombic domain's diagonal",
            torus.k
        );
        assert!(
            shifts.values().all(|times| *times == 2 * torus.k as usize),
            "k = {}: the six shifts are used {:?} times rather than {} each - evenly is what \
             makes each edge one rule",
            torus.k,
            shifts.values().collect::<Vec<_>>(),
            2 * torus.k
        );
        // And how often a step wraps at all: `6 * 2k` of `6N`, which is `2/(3k)`.
        assert_eq!(
            shifts.values().sum::<usize>() * 3 * torus.k as usize,
            2 * 6 * torus.cells(),
            "k = {}: the share of steps that wrap is not 2/(3k)",
            torus.k
        );
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
    // Both families: a cell neighbouring itself is a wrap folded onto itself whichever lattice
    // did the folding.
    for torus in both_families() {
        for (at, neighbours) in torus.adjacency().iter().enumerate() {
            let mut sorted = neighbours.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(
                sorted.len(),
                6,
                "{:?} k = {}: cell {at} has {:?} for neighbours",
                torus.family,
                torus.k,
                neighbours
            );
            assert!(
                !neighbours.contains(&(at as u32)),
                "{:?} k = {}: cell {at} is its own neighbour, so the wrap folded it onto itself",
                torus.family,
                torus.k
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked,
        both_families().iter().map(|t| t.cells()).sum::<usize>(),
        "every cell of every size in both families"
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
