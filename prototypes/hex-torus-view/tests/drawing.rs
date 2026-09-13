//! What the page actually contains, which is a different question from what the grid says.
//!
//! **`tests/wrapping.rs` checks the grid and this checks the drawing.** They would both pass
//! with the bright and dim colours swapped, or with the seven copies placed at the wrong
//! translates, or with the ids taken from the position rather than from the cell - none of
//! which is a fact about the torus and all of which is a fact about the picture a person is
//! asked to vet.
//!
//! **Counted from the emitted markup rather than from the data that produced it.** A count
//! over `Torus::domain` would agree with itself; `X-32`'s vetted-when is about what is on the
//! screen, so this reads the string the page is made of.

use graph_coloring::color_graph;
use hex_torus_view::{draw, sizes};

/// The bright colours, which is how a hex says it is the copy rather than an echo.
fn bright_fills() -> Vec<&'static str> {
    draw::COLOURS.iter().map(|(full, _)| *full).collect()
}

/// Exactly `N` bright hexes, `7N` in all, and every id drawn exactly seven times.
#[test]
fn the_page_draws_one_bright_world_and_six_echoes() {
    let bright = bright_fills();
    let mut checked = 0;
    for torus in sizes() {
        let coloured = color_graph(&torus.adjacency());
        let svg = draw::group(&torus, &coloured.colors);
        let want = torus.cells();

        let fills: Vec<&str> = svg
            .match_indices("fill=\"")
            .map(|(at, _)| &svg[at + 6..at + 13])
            .collect();
        let full = fills.iter().filter(|fill| bright.contains(fill)).count();
        // Every polygon carries a fill and so does every id, so the polygons are half of them.
        assert_eq!(
            fills.len(),
            2 * 7 * want,
            "k = {}: {} fills where seven copies of {want} hexes and their ids is {}",
            torus.k,
            fills.len(),
            2 * 7 * want
        );
        assert_eq!(
            full, want,
            "k = {}: {full} hexes are drawn bright and one world is {want}",
            torus.k
        );

        // **Every id appears exactly seven times** - once in the bright copy and once in each
        // echo. That is the detail `X-32` says does the work: turn the ids on and the same
        // number appears in every direction.
        let mut seen: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
        for (at, _) in svg.match_indices("class=\"id\"") {
            let text = &svg[at..];
            let from = text.find('>').expect("an id element closes its tag") + 1;
            let to = text.find("</text>").expect("an id element ends");
            *seen.entry(&text[from..to]).or_default() += 1;
        }
        assert_eq!(
            seen.len(),
            want,
            "k = {}: {} distinct ids drawn and the world has {want} territories",
            torus.k,
            seen.len()
        );
        assert!(
            seen.values().all(|times| *times == 7),
            "k = {}: an id is drawn {:?} times rather than seven - once bright and six echoes",
            torus.k,
            seen.values().collect::<Vec<_>>()
        );
        checked += 1;
    }
    assert_eq!(
        checked, 10,
        "ten sizes, and the count so one cannot become ten"
    );
}

/// The seven copies are one world and its six lattice neighbours, and no copy overlaps another.
///
/// **Overlap is the failure this picture would hide.** Two copies drawn on top of each other
/// look like one copy with a strange edge, and the bright count would still be right.
#[test]
fn the_seven_copies_do_not_overlap() {
    let mut checked = 0;
    for torus in sizes() {
        let domain = torus.domain();
        let mut placed: std::collections::BTreeSet<(i32, i32)> = std::collections::BTreeSet::new();
        let copies = draw::copies(&torus);
        assert_eq!(copies.len(), 7, "k = {}: one world and six echoes", torus.k);
        assert_eq!(
            copies.iter().filter(|(_, bright)| *bright).count(),
            1,
            "k = {}: exactly one copy is the bright one",
            torus.k
        );
        for (shift, _) in &copies {
            for (q, r) in &domain {
                assert!(
                    placed.insert((q + shift.0, r + shift.1)),
                    "k = {}: two copies both draw ({}, {})",
                    torus.k,
                    q + shift.0,
                    r + shift.1
                );
            }
        }
        assert_eq!(
            placed.len(),
            7 * torus.cells(),
            "k = {}: seven copies of {} cells",
            torus.k,
            torus.cells()
        );
        checked += 1;
    }
    assert_eq!(checked, 10, "ten sizes");
}
