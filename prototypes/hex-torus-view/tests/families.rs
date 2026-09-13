//! The two families, and the fact that they are one lattice at two foldings.
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

use hex_torus_view::{Family, NEIGHBOURS, Torus, axis_aligned_sizes, sizes};

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
        hex_torus_view::both_families().len(),
        20,
        "ten of each, folded first"
    );
}
