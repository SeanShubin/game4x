//! Three colours suffice at every one of the ten sizes, and that is a fact about this family.
//!
//! **`X-32`, amended after Sean asked for the hexes coloured.** The count is an output of the
//! prototype rather than a setting: `crates/graph-coloring` climbs 2, then 3, then 4, and
//! reports which succeeded. Nothing here asks it for three.
//!
//! # Why the assertion is load-bearing rather than decorative
//!
//! A hex grid's proper 3-colouring in these coordinates is `(q - r) mod 3` - each of the six
//! neighbour steps changes it. **It survives the wrap exactly when both generators are
//! divisible by 3 under that difference**, and this family's are `(k, k)` with `q - r = 0` and
//! `(-k, 2k)` with `q - r = -3k`. Both, for every `k`.
//!
//! So a torus built with a generator from the wrong family needs four, and `Exact(3)` failing
//! here is a **wrapping** bug rather than a colouring one. `docs/theory/region-coloring.md`
//! has the other half of it: the twelve pentagons that make a perfect hex grid impossible on a
//! sphere are what push a Goldberg from three to four, and this grid has none.
//!
//! # And what the colouring costs the picture, which is not this file's to fix
//!
//! A 3-colouring of a hex grid is essentially rigid - diagonal bands that wrap - where a
//! Goldberg's four-colouring looks irregular. **So the colouring makes the repeating structure
//! more visible, not less**, which is right for a prototype about wrapping and wrong for
//! anything shipped. The research lane said so before it was built, which is the moment worth
//! saying it in.

use graph_coloring::{Method, color_graph};
use hex_torus_view::sizes;

/// Three colours, exactly, at all ten sizes.
#[test]
fn three_colours_suffice_at_every_size() {
    let mut checked = 0;
    for torus in sizes() {
        let neighbours = torus.adjacency();
        assert_eq!(
            neighbours.len(),
            torus.cells(),
            "k = {}: colouring a graph that is not the torus",
            torus.k
        );
        let coloured = color_graph(&neighbours);
        assert_eq!(
            coloured.method,
            Method::Exact(3),
            "k = {}: {} cells coloured by {:?} in {} colours - three is a property of this \
             family, so anything else is the wrapping and not the colouring",
            torus.k,
            torus.cells(),
            coloured.method,
            coloured.color_count
        );
        assert_eq!(coloured.color_count, 3, "k = {}", torus.k);

        // **And the colouring is proper, which `Exact(3)` alone does not say.** A method that
        // reported success while leaving two neighbours the same colour would pass above.
        for (at, mine) in neighbours.iter().enumerate() {
            for other in mine {
                assert_ne!(
                    coloured.colors[at], coloured.colors[*other as usize],
                    "k = {}: cells {at} and {other} are neighbours and share a colour",
                    torus.k
                );
            }
        }
        checked += 1;
    }
    assert_eq!(
        checked, 10,
        "ten sizes, and the count so one cannot become ten"
    );
}

/// Two colours never suffice, which is what makes three the answer rather than the ceiling.
///
/// **Without this, `Exact(3)` is satisfied by a grid that needed two.** A hex grid contains
/// triangles - any cell and two adjacent neighbours - so two is impossible, and the check is
/// that the impossibility is in the graph this prototype builds rather than in the theory.
#[test]
fn no_size_is_two_colourable() {
    let mut found = 0;
    for torus in sizes() {
        let neighbours = torus.adjacency();
        // Three mutually adjacent cells: a cell, one neighbour, and a neighbour of both.
        let triangle = neighbours.iter().enumerate().find_map(|(at, mine)| {
            mine.iter().find_map(|other| {
                let theirs = &neighbours[*other as usize];
                mine.iter()
                    .find(|third| theirs.contains(third) && **third != at as u32)
                    .map(|third| (at as u32, *other, *third))
            })
        });
        let (a, b, c) = triangle.unwrap_or_else(|| {
            panic!(
                "k = {}: no three mutually adjacent cells, so this grid is not a hex grid",
                torus.k
            )
        });
        assert!(
            neighbours[a as usize].contains(&b)
                && neighbours[b as usize].contains(&c)
                && neighbours[c as usize].contains(&a),
            "k = {}: {a}, {b}, {c} are not mutually adjacent after all",
            torus.k
        );
        found += 1;
    }
    assert_eq!(found, 10, "a triangle at every size");
}
