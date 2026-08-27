//! The sizes a planet comes in.
//!
//! There are five, and each is the territory count of one of the five smallest Goldberg
//! polyhedra. That is a stronger statement than it looks: territory counts are not free
//! numbers. Only `10T + 2` for `T = m^2 + mn + n^2` exists at all, which gives 12, 32,
//! 42, 72, 92, 122 and upward with nothing in between. So this list is a choice among
//! the counts that exist, not a set of round numbers picked for feel.
//!
//! The counts live here, in the model, rather than beside the geometry that produces
//! them. Nothing about "how big is a planet" needs to know what a sphere is - it is five
//! whole numbers - and putting them here keeps them on the integer side of the boundary
//! described in `docs/architecture.md`. That they really are Goldberg counts is asserted
//! in `planet-render`, which is the lowest crate that can see both facts at once.

/// One of the five planet sizes.
///
/// Ordered smallest to largest, so comparisons and sorting mean what they read as.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlanetSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

impl PlanetSize {
    /// Every size, smallest first.
    pub const ALL: [Self; 5] = [
        Self::Tiny,
        Self::Small,
        Self::Medium,
        Self::Large,
        Self::Huge,
    ];

    /// How many territories a planet of this size has.
    pub fn territory_count(self) -> usize {
        match self {
            Self::Tiny => 12,
            Self::Small => 32,
            Self::Medium => 42,
            Self::Large => 72,
            Self::Huge => 92,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Huge => "huge",
        }
    }

    /// The size with this many territories, if any. Most counts are not a planet size,
    /// including most counts that are perfectly good Goldberg numbers.
    pub fn with_territory_count(count: usize) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|size| size.territory_count() == count)
    }

    /// The next size up, or this one if it is already the largest. Saturating rather
    /// than wrapping: stepping through sizes should stop at the ends rather than jump
    /// from huge back to tiny, which would look like a glitch.
    pub fn larger(self) -> Self {
        Self::ALL
            .into_iter()
            .find(|size| *size > self)
            .unwrap_or(self)
    }

    /// The next size down, or this one if it is already the smallest.
    pub fn smaller(self) -> Self {
        Self::ALL
            .into_iter()
            .rev()
            .find(|size| *size < self)
            .unwrap_or(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_five_sizes_are_the_counts_the_specification_lists() {
        assert_eq!(PlanetSize::Tiny.territory_count(), 12);
        assert_eq!(PlanetSize::Small.territory_count(), 32);
        assert_eq!(PlanetSize::Medium.territory_count(), 42);
        assert_eq!(PlanetSize::Large.territory_count(), 72);
        assert_eq!(PlanetSize::Huge.territory_count(), 92);
        assert_eq!(PlanetSize::ALL.len(), 5);
    }

    /// The order of the enum is load-bearing - `larger` and `smaller` rely on it - so it
    /// is asserted rather than assumed.
    #[test]
    fn the_sizes_ascend() {
        let counts: Vec<usize> = PlanetSize::ALL
            .iter()
            .map(|size| size.territory_count())
            .collect();
        let mut sorted = counts.clone();
        sorted.sort_unstable();
        assert_eq!(counts, sorted, "declared order must match size order");
        sorted.dedup();
        assert_eq!(sorted.len(), 5, "no two sizes may share a count");
    }

    #[test]
    fn stepping_through_the_sizes_stops_at_both_ends() {
        assert_eq!(PlanetSize::Tiny.smaller(), PlanetSize::Tiny);
        assert_eq!(PlanetSize::Huge.larger(), PlanetSize::Huge);
        assert_eq!(PlanetSize::Tiny.larger(), PlanetSize::Small);
        assert_eq!(PlanetSize::Huge.smaller(), PlanetSize::Large);

        // Walking up from the smallest must reach every size in turn.
        let mut walked = vec![PlanetSize::Tiny];
        while *walked.last().unwrap() != PlanetSize::Huge {
            walked.push(walked.last().unwrap().larger());
        }
        assert_eq!(walked, PlanetSize::ALL.to_vec());
    }

    #[test]
    fn a_count_maps_back_to_its_size() {
        for size in PlanetSize::ALL {
            assert_eq!(
                PlanetSize::with_territory_count(size.territory_count()),
                Some(size)
            );
        }
        // 122 is a perfectly good Goldberg count and still not a planet size.
        assert_eq!(PlanetSize::with_territory_count(122), None);
        assert_eq!(PlanetSize::with_territory_count(0), None);
        assert_eq!(PlanetSize::with_territory_count(13), None);
    }
}
