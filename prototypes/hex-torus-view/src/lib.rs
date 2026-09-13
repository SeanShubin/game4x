//! A flat hex grid that wraps in all six directions equally, at the ten sizes that could ship.
//!
//! **The question**, which is the deliverable and not the code: at the sizes the game would
//! use, does a flat isotropic hex torus read as a world, and how visible is the wrapping?
//! `X-32`, asked for by Sean. `prototypes/goldberg-view` answers it for spheres, and the two
//! are meant to be run against each other.
//!
//! # The grid, and why this family
//!
//! A wrapping of the hex plane is an ideal of the Eisenstein integers, so the sizes that admit
//! one are `a^2 + ab + b^2`. Those fall into families by circumference-to-area, and a sphere's
//! ratio is 2.72. **The `3k^2` family sits at exactly 3** - it circumnavigates `1.73*sqrt(N)`
//! where a sphere takes `1.65*sqrt(N)`, 5% apart - and Sean chose it over every isotropic
//! size, seven of whose first ten rungs are too small to ship.
//!
//! # Coordinates, because the same family is written two ways
//!
//! These are **axial hex coordinates with 60 degrees between the axes**, where the six
//! neighbours are `(1,0) (0,1) (-1,1) (-1,0) (0,-1) (1,-1)` and the norm is `q^2 + qr + r^2`.
//! The generators of the `3k^2` lattice are then `(k, k)` and its rotation `(-k, 2k)`, each of
//! norm `3k^2`.
//!
//! **`X-32` states the same family as `k(2+w)` with `a + b = 3k`**, which is the 120-degree
//! convention where the norm is `a^2 - ab + b^2`. Both describe one lattice; a reader who
//! takes one convention's generator into the other's norm gets `7k^2` and a grid that does not
//! wrap evenly. Written down because the two look alike on the page.

pub mod draw;

/// A hex torus: the plane quotiented by the `3k^2` lattice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Torus {
    pub k: i32,
}

/// Axial hex coordinates, 60 degrees between the axes.
pub type Cell = (i32, i32);

/// The six neighbours of a hex, in axial coordinates.
pub const NEIGHBOURS: [Cell; 6] = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];

/// How far a cell is from the origin, in steps.
pub fn norm(q: i32, r: i32) -> i32 {
    (q.abs() + r.abs() + (q + r).abs()) / 2
}

impl Torus {
    pub fn new(k: i32) -> Self {
        assert!(k >= 1, "a torus needs k >= 1");
        Torus { k }
    }

    /// How many territories: `3k^2`.
    pub fn cells(&self) -> usize {
        (3 * self.k * self.k) as usize
    }

    /// How many steps to go all the way round, in any of the six directions: `3k`.
    ///
    /// **The same in all six, which is what isotropic means here** and what
    /// [`Torus::circumnavigations_agree`] asserts rather than assumes.
    pub fn circumference(&self) -> i32 {
        3 * self.k
    }

    /// The two generators of the wrapping lattice.
    pub fn generators(&self) -> [Cell; 2] {
        [(self.k, self.k), (-self.k, 2 * self.k)]
    }

    /// The canonical cell of the copy `(q, r)` belongs to.
    ///
    /// **The fundamental domain is the Voronoi cell of the lattice, which is a hexagon** - so
    /// the representative is the translate nearest the origin, and the bright region shows its
    /// six-fold symmetry in its shape rather than asserting it. `X-32`: a rectangular frame
    /// makes the wrap look like it is slipping.
    ///
    /// **Ties are broken by a total order and not by whichever came first.** A cell on the
    /// boundary is equidistant from two lattice points, and without a rule the bright count
    /// lands at `N` plus or minus a few with nothing in the picture saying which. The rule is
    /// *nearest, then largest `r`, then largest `q`* - arbitrary, and the only thing that
    /// matters is that it is a function of the class rather than of the route in.
    pub fn reduce(&self, q: i32, r: i32) -> Cell {
        let [a, b] = self.generators();
        // Three periods either way is more than enough for anything the viewer draws, and the
        // assertion below says so rather than trusting it.
        let reach = 3;
        let mut best: Option<Cell> = None;
        for m in -reach..=reach {
            for n in -reach..=reach {
                let candidate = (q - m * a.0 - n * b.0, r - m * a.1 - n * b.1);
                let key = (norm(candidate.0, candidate.1), -candidate.1, -candidate.0);
                let better = match best {
                    None => true,
                    Some(far) => key < (norm(far.0, far.1), -far.1, -far.0),
                };
                if better {
                    best = Some(candidate);
                }
            }
        }
        let found = best.expect("the window always contains m = n = 0");
        assert!(
            norm(found.0, found.1) <= self.circumference(),
            "reducing ({q}, {r}) reached the edge of the search window, so the window is too \
             small and the answer would be silently wrong rather than absent"
        );
        found
    }

    /// Every canonical cell, in a fixed order, which is what gives each an id.
    ///
    /// **Enumerated from a region big enough to contain the domain, then reduced** - so the
    /// domain is whatever the reduction says it is rather than a shape written here twice.
    pub fn domain(&self) -> Vec<Cell> {
        let reach = 2 * self.k;
        let mut found: Vec<Cell> = Vec::new();
        for q in -reach..=reach {
            for r in -reach..=reach {
                let cell = self.reduce(q, r);
                if !found.contains(&cell) {
                    found.push(cell);
                }
            }
        }
        found.sort_by_key(|(q, r)| (*r, *q));
        found
    }

    /// Which territory a cell is, as an index into [`Torus::domain`].
    pub fn id_of(&self, domain: &[Cell], q: i32, r: i32) -> usize {
        let cell = self.reduce(q, r);
        domain
            .iter()
            .position(|other| *other == cell)
            .unwrap_or_else(|| panic!("({q}, {r}) reduces to {cell:?}, which is not in the domain"))
    }

    /// Whether a cell is the bright copy - the one the others echo.
    pub fn is_canonical(&self, q: i32, r: i32) -> bool {
        self.reduce(q, r) == (q, r)
    }

    /// Every cell's six neighbours, as ids, which is what a colouring is run over.
    ///
    /// **The wrap is in here and nowhere else.** A neighbour off the edge of the domain
    /// reduces back into it, which is the whole of what makes this a torus rather than a
    /// patch.
    pub fn adjacency(&self) -> Vec<Vec<u32>> {
        let domain = self.domain();
        domain
            .iter()
            .map(|(q, r)| {
                NEIGHBOURS
                    .iter()
                    .map(|(dq, dr)| self.id_of(&domain, q + dq, r + dr) as u32)
                    .collect()
            })
            .collect()
    }

    /// Whether stepping `3k` times in each of the six directions returns to where it began,
    /// and no fewer steps do.
    ///
    /// **Asserted rather than assumed, because it is the definition of the family.** A
    /// generator from the other coordinate convention gives `7k^2` cells and six
    /// circumnavigations that do not agree, and the picture would look almost right.
    pub fn circumnavigations_agree(&self) -> bool {
        let steps = self.circumference();
        NEIGHBOURS.iter().all(|(dq, dr)| {
            let round = self.reduce(dq * steps, dr * steps) == (0, 0);
            let sooner = (1..steps).any(|n| self.reduce(dq * n, dr * n) == (0, 0));
            round && !sooner
        })
    }
}

/// The ten sizes `X-32` names, which are `k = 2..=11`.
pub fn sizes() -> Vec<Torus> {
    (2..=11).map(Torus::new).collect()
}
