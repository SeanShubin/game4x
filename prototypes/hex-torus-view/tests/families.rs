//! The families against each other: two that are one lattice at two foldings, and one that
//! refutes the requirement both were built to satisfy.
//!
//! **`X-37` is the third.** Sean measured Solium Infernum rather than reasoning about it, and a
//! shipped game he finds perfectly legible closes in `2W`, `H`, `2W` - one axis twice another.
//! So isotropy was never what made the pathing sensible; what does is that the wrap happens in
//! the coordinates a person thinks in, rows and columns.
//!
//! **Sean asked what a wrapping square grid has that a hex grid cannot.** The answer is
//! nothing, and this is where that is checked rather than asserted: wrapping each axial
//! coordinate on its own is isotropic, closes in `C` in all six directions, and gives six
//! shifts that are `(±C, 0)`, `(0, ±C)`, `(±C, ∓C)` - coordinate-wise, which is the whole of
//! why square-grid pathing is easy.
//!
//! **And `3k²` is this lattice folded into three.** Same circumference, a third of the
//! territories, and the folding is what destroys the coordinate-wise wrap. Hexes were never
//! the cause.

use hex_torus_view::{Family, NEIGHBOURS, Torus, axis_aligned_sizes, offset_sizes, sizes};

/// Every size of the axis-aligned family wraps by subtracting `C` from one coordinate or both.
#[test]
fn the_axis_aligned_family_wraps_one_coordinate_at_a_time() {
    let mut checked = 0;
    for torus in axis_aligned_sizes() {
        let c = torus.k;
        let domain = torus.domain();
        assert_eq!(
            domain.len(),
            torus.cells(),
            "C = {c}: the population, first - a count that does not state one is how three \
             wrong numbers got out of this prototype in one evening"
        );
        assert_eq!(torus.cells(), (c * c) as usize, "C = {c}: N is C squared");

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
        let want: std::collections::BTreeSet<(i32, i32)> =
            [(c, 0), (-c, 0), (0, c), (0, -c), (c, -c), (-c, c)]
                .into_iter()
                .collect();
        assert_eq!(
            shifts
                .keys()
                .copied()
                .collect::<std::collections::BTreeSet<_>>(),
            want,
            "C = {c}: the wrap is not one coordinate at a time"
        );
        // **Every shift moves exactly one coordinate by `C`, or both by `C` in opposite
        // directions** - which is what *coordinate-wise* means and what a player can learn.
        for (q, r) in shifts.keys() {
            assert!(
                (*q == 0 || q.abs() == c) && (*r == 0 || r.abs() == c),
                "C = {c}: the shift ({q}, {r}) is not a whole circumference in a coordinate"
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 10, "ten sizes, C = 3..=12");
}

/// All six directions close in `C`, and none sooner - the same isotropy the other family has.
#[test]
fn the_axis_aligned_family_is_isotropic() {
    let mut checked = 0;
    for torus in axis_aligned_sizes() {
        assert!(
            torus.circumnavigations_agree(),
            "C = {}: the six directions do not all close at {}",
            torus.k,
            torus.circumference()
        );
        assert_eq!(torus.circumference(), torus.k);
        checked += 1;
    }
    assert_eq!(checked, 10, "ten sizes");
    assert_eq!(
        axis_aligned_sizes()
            .iter()
            .map(|t| t.cells())
            .collect::<Vec<_>>(),
        vec![9, 16, 25, 36, 49, 64, 81, 100, 121, 144],
        "C = 3..=12, and 1 and 2 are dropped as degenerate the way k = 1 is"
    );
}

/// The axis-aligned lattice sits inside the `3k²` lattice with index exactly three.
///
/// **This is the finding Sean's question produced**, and it is why *hexes make pathing hard*
/// is false: the two families are one lattice at two foldings. Same circumference at `C = 3k`,
/// a third of the territories, and **the folding is what destroys the coordinate-wise wrap**.
#[test]
fn the_folded_family_is_the_axis_aligned_one_folded_into_three() {
    let mut checked = 0;
    for folded in sizes() {
        let c = folded.circumference();
        let aligned = Torus::axis_aligned(c);
        assert_eq!(
            aligned.circumference(),
            folded.circumference(),
            "k = {}: the two families do not share a circumference at C = 3k",
            folded.k
        );
        assert_eq!(
            aligned.cells(),
            3 * folded.cells(),
            "k = {}: the index is not three - {} against {}",
            folded.k,
            aligned.cells(),
            folded.cells()
        );
        // **Both of the axis-aligned generators are in the folded lattice**, which is what
        // *sits inside* means and is the half a ratio of areas does not show.
        for generator in aligned.generators() {
            assert_eq!(
                folded.reduce(generator.0, generator.1),
                (0, 0),
                "k = {}: {generator:?} is not in the 3k^2 lattice, so the two are not nested",
                folded.k
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 10, "ten sizes");
}

/// Three colours suffice only where three divides `C`, which the ladder reports without being
/// told.
///
/// **A real difference from `3k²`, where three always sufficed.** The proper colouring of a hex
/// grid is `(q − r) mod 3`, and it survives a wrap when both generators have `q − r` divisible
/// by three. The folded family's are `0` and `−3k`, always. This family's are `C` and `−C`.
#[test]
fn the_axis_aligned_family_needs_four_colours_unless_three_divides_c() {
    let mut checked = 0;
    for torus in axis_aligned_sizes() {
        let coloured = graph_coloring::color_graph(&torus.adjacency());
        let want = if torus.k % 3 == 0 { 3 } else { 4 };
        assert_eq!(
            coloured.method,
            graph_coloring::Method::Exact(want),
            "C = {}: {} cells coloured by {:?} where three divides C is {}",
            torus.k,
            torus.cells(),
            coloured.method,
            torus.k % 3 == 0
        );
        for (at, mine) in torus.adjacency().iter().enumerate() {
            for other in mine {
                assert_ne!(
                    coloured.colors[at], coloured.colors[*other as usize],
                    "C = {}: cells {at} and {other} are neighbours and share a colour",
                    torus.k
                );
            }
        }
        checked += 1;
    }
    assert_eq!(checked, 10, "ten sizes");
    // **Four of the ten take three colours and six take four**, stated so that a run where
    // every size agreed would fail rather than read as a clean result.
    //
    // **This assertion said three and its own message said four**, which is the arithmetic
    // error caught by the check that carries it: `C = 3, 6, 9, 12` is four of `3..=12`. The
    // ten colour counts above were all correct; only the summary was wrong.
    let threes = axis_aligned_sizes().iter().filter(|t| t.k % 3 == 0).count();
    assert_eq!(threes, 4, "C = 3, 6, 9 and 12 is four of ten");
}

/// The two families are distinguishable by their own accessors, so nothing reads one as the
/// other.
#[test]
fn a_torus_says_which_family_it_is() {
    assert_eq!(Torus::folded(2).family, Family::Folded);
    assert_eq!(Torus::axis_aligned(6).family, Family::AxisAligned);
    assert_eq!(
        Torus::new(2),
        Torus::folded(2),
        "`new` is the folded family"
    );
    assert_eq!(
        Torus::offset(12, 12).family,
        Family::Offset { tall: 12 },
        "the offset family carries its own height, which the other two do not have"
    );
    assert_eq!(
        hex_torus_view::both_families().len(),
        30,
        "ten of each of the three, folded first"
    );
    assert_eq!(
        [
            Family::Folded,
            Family::AxisAligned,
            Family::Offset { tall: 4 }
        ]
        .map(|it| it.name()),
        ["folded", "axis-aligned", "offset"],
        "the names the page reads off a group come from here and from nowhere else"
    );
}

/// Sean's two walks generate the offset lattice, and nothing was fitted to them.
///
/// **`X-37`, and this is the derivation rather than a restatement of it.** He measured a
/// shipped game: *twelve up returns to start; twelve right - alternating up-right, down-right -
/// returns to start.* Twelve alternating rightward steps is `6·(1,0) + 6·(1,-1) = (12, -6)` and
/// twelve up is `(0, 12)`. The claim is that those two walks and the offset generators are the
/// same pair, so the measurement and the conversion agree without either being fitted to the
/// other.
#[test]
fn the_two_walks_sean_measured_are_the_offset_generators() {
    let mut checked = 0;
    for torus in offset_sizes() {
        let (wide, tall) = (torus.k, torus.tall());
        // **Walked rather than summed**, because *alternating up-right and down-right* is a
        // walk and writing it as `6a + 6b` is already half the conversion this test exists to
        // check. `NEIGHBOURS[5]` and `NEIGHBOURS[0]` are the two the walk alternates between -
        // in a flat-top drawing they are up-right and down-right, which is the orientation
        // Sean was reading.
        let mut rightward = (0, 0);
        for step in 0..wide {
            let (dq, dr) = NEIGHBOURS[if step % 2 == 0 { 5 } else { 0 }];
            rightward = (rightward.0 + dq, rightward.1 + dr);
        }
        let mut upward = (0, 0);
        for _ in 0..tall {
            let (dq, dr) = NEIGHBOURS[1];
            upward = (upward.0 + dq, upward.1 + dr);
        }
        assert_eq!(
            rightward,
            (wide, -wide / 2),
            "W = {wide}: the alternating walk is not (W, -W/2)"
        );
        assert_eq!(
            upward,
            (0, tall),
            "H = {tall}: the straight walk is not (0, H)"
        );
        assert_eq!(
            torus.generators(),
            [rightward, upward],
            "W = {wide}: the generators are not the two walks"
        );
        // **And each walk is genuinely a round trip of that many steps**, which is the half a
        // pair of vectors does not say.
        assert_eq!(
            torus.reduce(rightward.0, rightward.1),
            (0, 0),
            "W = {wide}: {wide} alternating rightward steps do not come back"
        );
        assert_eq!(
            torus.reduce(upward.0, upward.1),
            (0, 0),
            "W = {wide}: {tall} steps up do not come back"
        );
        assert_eq!(
            torus.cells(),
            (wide * tall) as usize,
            "W = {wide}: N is W·H"
        );
        checked += 1;
    }
    assert_eq!(checked, 10, "ten offset sizes");

    // The rung that matters, stated on its own so a ladder that stopped short would fail here.
    let sean = Torus::offset(12, 12);
    assert_eq!(sean.generators(), [(12, -6), (0, 12)]);
    assert_eq!(
        sean.cells(),
        144,
        "Solium Infernum's map is 144 territories"
    );
}

/// The offset family is not isotropic, which is the whole of why `X-37` overturns a requirement.
///
/// **Sean asked for equal circumference in all six directions and two families were built to
/// satisfy it.** A game he finds perfectly legible does not have it: one axis takes twice as
/// long as another, and he did not notice until he went looking. So the check is that this
/// family fails the property the other two were built for - **asserted, because a family that
/// quietly became isotropic would delete the finding without deleting a line of it.**
#[test]
fn the_offset_family_closes_in_two_w_h_two_w_and_is_not_isotropic() {
    let mut checked = 0;
    for torus in offset_sizes() {
        let (wide, tall) = (torus.k, torus.tall());
        assert_eq!(
            torus.circumferences(),
            [2 * wide, tall, 2 * wide, 2 * wide, tall, 2 * wide],
            "W = {wide}, H = {tall}: the six do not close in 2W, H, 2W"
        );
        assert!(
            !torus.circumnavigations_agree(),
            "W = {wide}: this family is isotropic, and X-37 is the finding that it is not"
        );
        checked += 1;
    }
    assert_eq!(checked, 10, "ten offset sizes");

    // **And the other two families still are**, so the assertion above is about this family
    // rather than about the instrument having broken.
    let mut isotropic = 0;
    for torus in sizes().into_iter().chain(axis_aligned_sizes()) {
        assert!(
            torus.circumnavigations_agree(),
            "{:?} k = {}: an isotropic family stopped being isotropic",
            torus.family,
            torus.k
        );
        isotropic += 1;
    }
    assert_eq!(isotropic, 20, "the folded ten and the axis-aligned ten");
}

/// The two routes to a circumference agree, at every size of every family.
///
/// **One solves over the generators and one walks the reduction.** `circumferences` does
/// Cramer over the integers and never calls `reduce`; `circumnavigations_agree` steps and
/// reduces. They are independent, which is what makes agreeing worth asserting - and it is the
/// habit that caught `X-37`'s arithmetic being right rather than taking it.
#[test]
fn the_two_routes_to_a_circumference_agree() {
    let mut checked = 0;
    for torus in hex_torus_view::both_families() {
        for (at, (dq, dr)) in NEIGHBOURS.iter().enumerate() {
            let solved = torus.circumferences()[at];
            assert_eq!(
                torus.reduce(dq * solved, dr * solved),
                (0, 0),
                "{:?} k = {}: solving says ({dq}, {dr}) closes in {solved} and reducing says no",
                torus.family,
                torus.k
            );
            assert!(
                (1..solved).all(|n| torus.reduce(dq * n, dr * n) != (0, 0)),
                "{:?} k = {}: ({dq}, {dr}) comes back sooner than the {solved} solved for",
                torus.family,
                torus.k
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 30 * 6, "six directions at each of thirty worlds");
}

/// At 144 cells the three families can be held against each other, which is the comparison asked for.
#[test]
fn the_three_families_meet_where_they_can() {
    let offset = Torus::offset(12, 12);
    let aligned = Torus::axis_aligned(12);
    let folded = Torus::folded(4);
    assert_eq!(offset.cells(), aligned.cells(), "both are 144 territories");
    assert_eq!(offset.cells(), 144);
    assert_eq!(
        [
            offset.circumferences()[0],
            offset.circumferences()[1],
            offset.circumferences()[2]
        ],
        [24, 12, 24],
        "the offset world at 144 is the one Sean measured"
    );
    assert_eq!(aligned.circumferences(), [12; 6]);
    assert_eq!(folded.circumferences(), [12; 6]);
    // **Folded has no world at 144 and this says so rather than leaving a reader to wonder**:
    // `3k² = 144` wants `k² = 48`, and 48 is not a square.
    assert!(
        sizes().iter().all(|it| it.cells() != 144),
        "the folded family has a 144 and the comment says it cannot"
    );
    assert_eq!(folded.cells(), 48, "what it has at circumference 12 is 48");
}

/// The offset domain is the `W × H` array, indexed by column and offset row.
///
/// **The claim of this family is *the wrap you get free from storing a hex map as a 2D array***,
/// so the bright world has to be that array. The axial rectangle `0..W × 0..H` is an equally
/// valid set of representatives and draws as a **parallelogram** - the lattice is identical
/// either way and only which copy is called bright moves, but the picture would then say
/// *sheared lattice* where the point is *the rectangle the array holds*. Found by drawing it.
#[test]
fn the_offset_domain_is_the_array_a_game_would_store() {
    let mut checked = 0;
    for torus in offset_sizes() {
        let (wide, tall) = (torus.k, torus.tall());
        let domain = torus.domain();
        assert_eq!(
            domain.len(),
            (wide * tall) as usize,
            "W = {wide}: W·H cells"
        );

        // Every (column, offset row) of the array appears exactly once.
        let mut seen: std::collections::BTreeSet<(i32, i32)> = std::collections::BTreeSet::new();
        for (q, r) in &domain {
            let row = r + q.div_euclid(2);
            assert!(
                (0..wide).contains(q) && (0..tall).contains(&row),
                "W = {wide}: ({q}, {r}) is column {q} row {row}, outside the {wide} by {tall} \
                 array"
            );
            assert!(
                seen.insert((*q, row)),
                "W = {wide}: column {q} row {row} is in the domain twice"
            );
        }
        assert_eq!(
            seen.len(),
            (wide * tall) as usize,
            "W = {wide}: the domain is not every cell of the array exactly once"
        );

        // **And reducing lands in it**, which is the half a set-of-cells claim does not make.
        for q in -2 * wide..=2 * wide {
            for r in -2 * tall..=2 * tall {
                let (cq, cr) = torus.reduce(q, r);
                let row = cr + cq.div_euclid(2);
                assert!(
                    (0..wide).contains(&cq) && (0..tall).contains(&row),
                    "W = {wide}: ({q}, {r}) reduces to column {cq} row {row}, off the array"
                );
            }
        }
        checked += 1;
    }
    assert_eq!(checked, 10, "ten offset sizes");
}
