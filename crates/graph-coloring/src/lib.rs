//! Assigns colors to a graph so that no two adjacent vertices share one, using as few
//! as practical.
//!
//! The theory is in `docs/theory/region-coloring.md`. The short version: a region
//! adjacency graph on a sphere is planar, so four colors always suffice — that is the
//! Four Color Theorem. Deciding 3-colorability is NP-complete, and *finding* a
//! four-coloring cheaply is best done with a heuristic search rather than the
//! quadratic algorithm from the proof.
//!
//! So this crate climbs a ladder: try 2 colors, then 3, then 4, each with a search
//! budget, and fall back to a greedy ordering that cannot fail. The fallback is a bug
//! detector — on a genuinely planar input the k = 4 step should always succeed.
//!
//! This crate knows nothing about spheres, geometry, or the game. It takes neighbour
//! lists and returns colors.

/// How the coloring was arrived at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    /// No edges to satisfy.
    Trivial,
    /// Exact search succeeded at this many colors.
    Exact(usize),
    /// The search budget ran out; this is a smallest-last greedy result.
    GreedyFallback,
}

#[derive(Clone, Debug)]
pub struct Coloring {
    /// One color index per vertex.
    pub colors: Vec<u8>,
    /// How many distinct colors are actually used.
    pub color_count: usize,
    pub method: Method,
}

/// The largest number of colors the exact search will attempt before giving up and
/// falling back. Four is a theorem for planar graphs; there is no point trying five.
const MAXIMUM_EXACT: usize = 4;

/// Default search effort, in backtracking steps. Generous: a few hundred planar
/// vertices normally resolve in far less.
pub const DEFAULT_BUDGET: u64 = 2_000_000;

/// Colors a graph given its neighbour lists.
pub fn color_graph(neighbours: &[Vec<u32>]) -> Coloring {
    color_graph_with_budget(neighbours, DEFAULT_BUDGET)
}

pub fn color_graph_with_budget(neighbours: &[Vec<u32>], budget: u64) -> Coloring {
    if neighbours.is_empty() {
        return Coloring {
            colors: Vec::new(),
            color_count: 0,
            method: Method::Trivial,
        };
    }
    if neighbours.iter().all(|list| list.is_empty()) {
        return Coloring {
            colors: vec![0; neighbours.len()],
            color_count: 1,
            method: Method::Trivial,
        };
    }

    for count in 2..=MAXIMUM_EXACT {
        let mut remaining = budget;
        if let Some(colors) = try_exactly(neighbours, count, &mut remaining) {
            let used = distinct(&colors);
            return Coloring {
                colors,
                color_count: used,
                method: Method::Exact(used),
            };
        }
    }

    let colors = greedy_smallest_last(neighbours);
    let used = distinct(&colors);
    Coloring {
        colors,
        color_count: used,
        method: Method::GreedyFallback,
    }
}

/// Backtracking search for a coloring using at most `count` colors.
///
/// Vertex choice is DSATUR: always take the vertex with the most distinct colors
/// already among its neighbours, so the search fails fast where it is going to fail.
/// A vertex may only introduce the next unused color, which discards the enormous
/// symmetry between equivalent color permutations.
fn try_exactly(neighbours: &[Vec<u32>], count: usize, budget: &mut u64) -> Option<Vec<u8>> {
    let vertices = neighbours.len();
    let mut colors = vec![u8::MAX; vertices];
    // Bit i is set when a neighbour already uses color i.
    let mut blocked = vec![0u32; vertices];
    if search(neighbours, count, &mut colors, &mut blocked, 0, 0, budget) {
        Some(colors)
    } else {
        None
    }
}

fn search(
    neighbours: &[Vec<u32>],
    count: usize,
    colors: &mut [u8],
    blocked: &mut [u32],
    assigned: usize,
    highest_used: usize,
    budget: &mut u64,
) -> bool {
    if assigned == colors.len() {
        return true;
    }
    if *budget == 0 {
        return false;
    }
    *budget -= 1;

    let Some(vertex) = most_saturated(neighbours, colors, blocked) else {
        return true;
    };

    // Never open more than one fresh color at a time.
    let ceiling = (highest_used + 1).min(count);
    for candidate in 0..ceiling {
        if blocked[vertex] & (1 << candidate) != 0 {
            continue;
        }
        colors[vertex] = candidate as u8;
        let mut touched = Vec::with_capacity(neighbours[vertex].len());
        for &neighbour in &neighbours[vertex] {
            let index = neighbour as usize;
            if blocked[index] & (1 << candidate) == 0 {
                blocked[index] |= 1 << candidate;
                touched.push(index);
            }
        }

        let next_highest = highest_used.max(candidate + 1);
        if search(
            neighbours,
            count,
            colors,
            blocked,
            assigned + 1,
            next_highest,
            budget,
        ) {
            return true;
        }

        for index in touched {
            blocked[index] &= !(1 << candidate);
        }
        colors[vertex] = u8::MAX;
    }
    false
}

/// The uncolored vertex whose neighbours already use the most distinct colors, with
/// ties broken by degree and then by index so the result is deterministic.
fn most_saturated(neighbours: &[Vec<u32>], colors: &[u8], blocked: &[u32]) -> Option<usize> {
    let mut best: Option<(u32, usize, usize)> = None;
    for vertex in 0..colors.len() {
        if colors[vertex] != u8::MAX {
            continue;
        }
        let key = (
            blocked[vertex].count_ones(),
            neighbours[vertex].len(),
            vertex,
        );
        match best {
            Some((saturation, degree, _)) if (key.0, key.1) <= (saturation, degree) => {}
            _ => best = Some(key),
        }
    }
    best.map(|(_, _, vertex)| vertex)
}

/// Greedy coloring in smallest-last order.
///
/// Every planar graph is 5-degenerate, so this uses at most six colors on any planar
/// input, in linear time, always. It exists so the renderer is never handed an
/// uncolored map.
pub fn greedy_smallest_last(neighbours: &[Vec<u32>]) -> Vec<u8> {
    let vertices = neighbours.len();
    let mut degree: Vec<usize> = neighbours.iter().map(|list| list.len()).collect();
    let mut removed = vec![false; vertices];
    let mut order = Vec::with_capacity(vertices);

    for _ in 0..vertices {
        let mut chosen = usize::MAX;
        let mut lowest = usize::MAX;
        for vertex in 0..vertices {
            if !removed[vertex] && degree[vertex] < lowest {
                lowest = degree[vertex];
                chosen = vertex;
            }
        }
        removed[chosen] = true;
        order.push(chosen);
        for &neighbour in &neighbours[chosen] {
            let index = neighbour as usize;
            if !removed[index] {
                degree[index] -= 1;
            }
        }
    }

    let mut colors = vec![u8::MAX; vertices];
    for &vertex in order.iter().rev() {
        let mut taken = 0u64;
        for &neighbour in &neighbours[vertex] {
            let color = colors[neighbour as usize];
            if color != u8::MAX {
                taken |= 1 << color;
            }
        }
        let mut candidate = 0u8;
        while taken & (1 << candidate) != 0 {
            candidate += 1;
        }
        colors[vertex] = candidate;
    }
    colors
}

fn distinct(colors: &[u8]) -> usize {
    let mut seen = 0u64;
    for &color in colors {
        seen |= 1 << color;
    }
    seen.count_ones() as usize
}

/// Returns the first edge whose endpoints share a color, if there is one.
pub fn find_conflict(neighbours: &[Vec<u32>], colors: &[u8]) -> Option<(usize, usize)> {
    for (vertex, list) in neighbours.iter().enumerate() {
        for &neighbour in list {
            if colors[vertex] == colors[neighbour as usize] {
                return Some((vertex, neighbour as usize));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn undirected(vertices: usize, edges: &[(u32, u32)]) -> Vec<Vec<u32>> {
        let mut neighbours = vec![Vec::new(); vertices];
        for &(a, b) in edges {
            neighbours[a as usize].push(b);
            neighbours[b as usize].push(a);
        }
        neighbours
    }

    #[test]
    fn no_vertices_is_not_a_problem() {
        let coloring = color_graph(&[]);
        assert_eq!(coloring.color_count, 0);
        assert_eq!(coloring.method, Method::Trivial);
    }

    #[test]
    fn a_graph_with_no_edges_needs_one_color() {
        let coloring = color_graph(&vec![Vec::new(); 5]);
        assert_eq!(coloring.color_count, 1);
        assert_eq!(coloring.method, Method::Trivial);
    }

    #[test]
    fn a_path_needs_two_colors() {
        let graph = undirected(4, &[(0, 1), (1, 2), (2, 3)]);
        let coloring = color_graph(&graph);
        assert_eq!(coloring.color_count, 2);
        assert!(find_conflict(&graph, &coloring.colors).is_none());
    }

    #[test]
    fn an_odd_cycle_needs_three_colors() {
        let graph = undirected(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
        let coloring = color_graph(&graph);
        assert_eq!(coloring.color_count, 3);
        assert!(find_conflict(&graph, &coloring.colors).is_none());
    }

    #[test]
    fn an_even_cycle_needs_two_colors() {
        let graph = undirected(6, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)]);
        assert_eq!(color_graph(&graph).color_count, 2);
    }

    #[test]
    fn a_complete_graph_of_four_needs_four_colors() {
        let graph = undirected(4, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
        let coloring = color_graph(&graph);
        assert_eq!(coloring.color_count, 4);
        assert!(find_conflict(&graph, &coloring.colors).is_none());
    }

    /// A real tessellation. Planar, so four colors must be enough, and the exact
    /// search must be the thing that finds them.
    #[test]
    fn a_sphere_tessellation_needs_at_most_four_colors() {
        for region_count in [4, 7, 20, 60, 137] {
            let tessellation =
                sphere_tessellation::Tessellation::generate(sphere_tessellation::Params {
                    region_count,
                    ..Default::default()
                });
            let coloring = color_graph(&tessellation.neighbours);
            assert!(
                find_conflict(&tessellation.neighbours, &coloring.colors).is_none(),
                "adjacent regions share a color at {region_count}"
            );
            assert!(
                coloring.color_count <= 4,
                "{region_count} regions took {} colors",
                coloring.color_count
            );
            assert_ne!(
                coloring.method,
                Method::GreedyFallback,
                "fell back to greedy at {region_count}, which means the exact search \
                 failed on a planar graph"
            );
        }
    }

    #[test]
    fn the_greedy_fallback_always_produces_a_valid_coloring() {
        let tessellation = sphere_tessellation::Tessellation::generate(Default::default());
        let colors = greedy_smallest_last(&tessellation.neighbours);
        assert!(find_conflict(&tessellation.neighbours, &colors).is_none());
        // Planar graphs are 5-degenerate, so smallest-last cannot exceed six.
        assert!(distinct(&colors) <= 6);
    }

    #[test]
    fn coloring_is_deterministic() {
        let tessellation = sphere_tessellation::Tessellation::generate(Default::default());
        let first = color_graph(&tessellation.neighbours);
        let second = color_graph(&tessellation.neighbours);
        assert_eq!(first.colors, second.colors);
    }
}
