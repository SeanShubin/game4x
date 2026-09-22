//! The board: where the territories are, and which routes between them are unambiguous.
//!
//! **No engine here and no game.** A board is a list of directions on the unit sphere and a
//! neighbour list, both from `sphere-tessellation`. Everything about moving is graph work over
//! that neighbour list, which is why it is in its own file: the interesting rule in this
//! prototype is *which destinations a player may click*, and that rule has to be checkable
//! without a window.

use sphere_tessellation::Vec3;

/// A territory, by the index `sphere-tessellation` gives it.
pub type Where = u32;

/// The twelve, thirty-two or forty-two faces of a Goldberg solid, and who touches whom.
pub struct Board {
    /// The centre of each territory, as a direction on the unit sphere.
    pub centres: Vec<Vec3>,
    /// Each territory's neighbours, sorted.
    pub neighbours: Vec<Vec<Where>>,
}

/// What clicking a destination would mean, given where the move has reached so far.
///
/// **The whole rule of this prototype is in this enum.** Sean, 2026-09-21: *I can only select
/// destinations that leave the path unambiguous, otherwise I have to click on the intermediate
/// destinations.* So a click is answered with the route it implies, or with a refusal that says
/// how many routes there were.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reach {
    /// The destination is where the move already is.
    Arrived,
    /// Exactly one shortest route, given as the territories stepped onto - the origin is not
    /// in it, and the destination is the last entry.
    One(Vec<Where>),
    /// More than one shortest route of the same length, so the click says where to end up and
    /// not how to get there.
    Many {
        /// How many shortest routes there are.
        ways: usize,
        /// How long every one of them is, in steps.
        steps: usize,
    },
    /// No route at all, which on a sphere means the board is broken rather than the click is.
    Nowhere,
}

impl Board {
    /// The `GP(m, n)` Goldberg solid, with its Voronoi neighbours.
    pub fn goldberg(m: usize, n: usize) -> Self {
        let centres = sphere_tessellation::goldberg::seeds(m, n);
        let neighbours = sphere_tessellation::adjacency(&centres);
        Self {
            centres,
            neighbours,
        }
    }

    pub fn territories(&self) -> usize {
        self.centres.len()
    }

    /// What a click on `to` means, for a move that has reached `from`.
    ///
    /// **Shortest routes and nothing else.** A longer route is never offered, because the whole
    /// point of clicking an intermediate territory is to pick one of several shortest routes -
    /// and a player who wants a detour says so by clicking the detour, which is then itself a
    /// shortest route from where the move has reached.
    ///
    /// **Counted before it is reconstructed.** A breadth-first sweep carries how many shortest
    /// routes reach each territory as well as how far it is, so *is it unambiguous* is answered
    /// by a number rather than by trying to build a route and noticing a second one. The two
    /// halves cannot disagree that way.
    pub fn reach(&self, from: Where, to: Where) -> Reach {
        if from == to {
            return Reach::Arrived;
        }
        let count = self.territories();
        let (mut far, mut ways) = (vec![usize::MAX; count], vec![0usize; count]);
        far[from as usize] = 0;
        ways[from as usize] = 1;

        let mut front = std::collections::VecDeque::from([from]);
        while let Some(here) = front.pop_front() {
            let next = far[here as usize] + 1;
            for &other in &self.neighbours[here as usize] {
                let at = other as usize;
                if far[at] == usize::MAX {
                    far[at] = next;
                    front.push_back(other);
                }
                // **Added to rather than assigned**, so a territory two routes reach at the
                // same distance carries two - which is what makes the destination ambiguous.
                if far[at] == next {
                    ways[at] = ways[at].saturating_add(ways[here as usize]);
                }
            }
        }

        let steps = far[to as usize];
        if steps == usize::MAX {
            return Reach::Nowhere;
        }
        if ways[to as usize] != 1 {
            return Reach::Many {
                ways: ways[to as usize],
                steps,
            };
        }

        // One route, so walking back from the destination has exactly one candidate at every
        // stage: the neighbour one step nearer the origin. **`expect` rather than a fallback**,
        // because a count of one that cannot be reconstructed is a defect in the sweep above
        // and silence about it would be the worst of the three outcomes.
        let mut route = Vec::with_capacity(steps);
        let mut here = to;
        while here != from {
            route.push(here);
            here = *self.neighbours[here as usize]
                .iter()
                .find(|&&other| far[other as usize] + 1 == far[here as usize])
                .expect("a territory one step nearer the origin");
        }
        route.reverse();
        Reach::One(route)
    }
}
