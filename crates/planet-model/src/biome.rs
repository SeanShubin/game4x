//! What kind of ground a territory is.
//!
//! Here rather than in `game-model` because three crates need it and only one of them is
//! the game. `planet-terrain` decides a territory's biome from the field, `planet-render`
//! paints it, and `game-model` holds it - and while it lived in the game, the two below
//! depended on the whole of `game-model` for a single `use` line.
//!
//! `spec/planet.md` is where every rule about it is written, which is the other half of the
//! argument: it is the planet's vocabulary, not the game's. [`PlanetSize`](crate::PlanetSize)
//! is here for the same reason and arrived first.

use std::fmt;

/// `spec/planet.md`: *each territory has a biome*, and *a territory's biome is what the
/// terrain gives it*.
///
/// `spec/planet.md`: *the biomes are ocean, ice, desert, grassland, jungle and mountain.*
///
/// Six, and chosen by **role rather than by appearance**: the test was whether knowing the
/// biome changes what a player does with a territory, or only how pleased they are to have
/// it. A planet may still draw tundra and savanna; they resolve to one of these until a
/// rule can tell them apart.
///
/// It is a fact of the model rather than of the picture so that the two cannot drift:
/// without that, a territory could be ice in the model while the realistic drawing paints
/// jungle across it, and nothing would be violated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Biome {
    Ocean,
    Ice,
    Desert,
    Grassland,
    Jungle,
    Mountain,
}

impl Biome {
    pub const ALL: [Self; 6] = [
        Self::Ocean,
        Self::Ice,
        Self::Desert,
        Self::Grassland,
        Self::Jungle,
        Self::Mountain,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Biome::Ocean => "ocean",
            Biome::Ice => "ice",
            Biome::Desert => "desert",
            Biome::Grassland => "grassland",
            Biome::Jungle => "jungle",
            Biome::Mountain => "mountain",
        }
    }

    pub fn named(word: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|biome| biome.name() == word)
    }

    /// `spec/planet.md`: *no territory can be claimed whose biome is ocean.*
    pub fn is_claimable(self) -> bool {
        self != Biome::Ocean
    }
}

impl fmt::Display for Biome {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(self.name())
    }
}
