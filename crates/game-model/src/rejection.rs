//! Why a transition could not be applied.
//!
//! `spec/console.md`: *a command that cannot be run says why, and says it in terms of the
//! game rather than the parser*. So nothing here mentions a token, a form or a column.
//! Where a parser failure says "expected a number at line 3 column 12", a rejection says
//! "territory 7 is not adjacent to territory 1".
//!
//! A rejection is data, like a parse failure, and for the same reason: the layer above
//! has to be able to show it, log it, or count it, and none of that is possible with an
//! unwind.

use std::fmt;

use crate::identity::{Resource, StructureKind, TerritoryId, UnitKind};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rejection {
    /// The command belongs to the other phase.
    WrongPhase {
        wanted: &'static str,
    },
    NoPlanet,
    /// `spec/planet.md`: no territory can be claimed whose biome is ocean.
    ///
    /// Refused at the moment of founding rather than at the moment of moving, because
    /// founding is where a territory is taken and the rule is about taking one.
    CannotClaimOcean(TerritoryId),
    /// A planet was described with a different number of biomes than territories.
    ///
    /// A malformed transition rather than a move the rules forbid, but it arrives by the
    /// same door as everything else and so it is refused by the same one. Silently padding
    /// would put a biome in the world that nothing chose.
    BiomesDoNotCoverThePlanet {
        territories: usize,
        biomes: usize,
    },
    NoSuchTerritory(TerritoryId),
    NoSuchResource(String),
    NoSuchUnitKind(String),
    NoSuchStructure(String),
    /// Nothing of that kind is anywhere it could act from.
    NoUnitAvailable {
        kind: UnitKind,
        where_from: &'static str,
    },
    NotAdjacent {
        from: TerritoryId,
        to: TerritoryId,
    },
    /// `move` onto ground nobody holds. `P-214`: that is `found by land`, and the player
    /// says which rather than the model deciding by looking.
    NotFoundedYet {
        territory: TerritoryId,
    },
    /// `found by land` onto ground somebody already holds.
    AlreadyFounded {
        territory: TerritoryId,
    },
    /// The unit is where it should be and has already acted this turn.
    AlreadyUsed(UnitKind),
    /// The territory is already holding as many of that kind as it can.
    NoRoomForAnother {
        territory: TerritoryId,
        kind: crate::thing::Kind,
    },
    NotControlled(TerritoryId),
    AlreadyControlled(TerritoryId),
    NoCells(UnitKind),
    /// Not enough force to take a territory from whoever holds it.
    NotEnoughForce {
        territory: TerritoryId,
        force: u32,
        needed: u32,
    },
    NotEnoughResource {
        territory: TerritoryId,
        resource: Resource,
        held: u32,
        needed: u32,
    },
    NotEnoughLabor {
        territory: TerritoryId,
        available: u32,
        needed: u32,
    },
    NotEnoughCitizens {
        territory: TerritoryId,
        held: u32,
        needed: u32,
    },
    /// Every node of that resource already has an extractor on it.
    /// A unit is in orbit, and not above the territory it was told to land on.
    ///
    /// **`S-55`.** An orbit is above one territory, and landing is a move between adjacent
    /// places, so the ground an Ark can reach is the ground beneath it.
    NotAboveThatTerritory {
        kind: UnitKind,
        above: TerritoryId,
        asked: TerritoryId,
    },
    /// The territory's total capacity for extractors of that resource is already used.
    NoRoomForExtractor {
        territory: TerritoryId,
        resource: Resource,
    },
    /// A resource has to be named to know what an extractor is built for.
    ResourceNotNamed(StructureKind),
    AlreadyHasGarrison(TerritoryId),
    NoGarrison(TerritoryId),
    NoYard(TerritoryId),
    /// Nothing of that kind is left to work at.
    NothingToWorkAt {
        territory: TerritoryId,
        structure: StructureKind,
    },
    /// More work was asked for than there are structures to do it.
    NotThatManyToWork {
        territory: TerritoryId,
        structure: StructureKind,
        available: u32,
        asked: u32,
    },
    /// A garrison is what a founding unit becomes; there is no cost to build one.
    GarrisonIsNotBuilt,
    /// The unit cannot come down from orbit.
    CannotLand(UnitKind),
    /// The unit is not on the planet, so it cannot go up.
    NotOnThePlanet(UnitKind),
    PlanetAlreadyCreated,
    NoSuchPlanetSize(String),
}

impl fmt::Display for Rejection {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rejection::WrongPhase { wanted } => {
                write!(out, "that can only be done {wanted}")
            }
            Rejection::NoPlanet => write!(out, "there is no planet yet"),
            Rejection::CannotClaimOcean(id) => {
                write!(out, "territory {id} is ocean, and ocean cannot be claimed")
            }
            Rejection::BiomesDoNotCoverThePlanet {
                territories,
                biomes,
            } => write!(
                out,
                "a planet of {territories} territories was given {biomes} biomes"
            ),
            Rejection::NoSuchTerritory(id) => write!(out, "there is no territory {id}"),
            Rejection::NoSuchResource(word) => write!(out, "there is no resource called {word}"),
            Rejection::NoSuchUnitKind(word) => write!(out, "there is no unit called {word}"),
            Rejection::NoSuchStructure(word) => write!(out, "there is no structure called {word}"),
            Rejection::NoUnitAvailable { kind, where_from } => {
                write!(out, "there is no {kind} {where_from}")
            }
            Rejection::NotAdjacent { from, to } => {
                write!(out, "territory {to} is not adjacent to territory {from}")
            }
            // **Names the other command rather than merely refusing.** The player asked for
            // something reasonable in the wrong words, and the words they wanted are one
            // line away.
            Rejection::NotFoundedYet { territory } => write!(
                out,
                "nobody holds territory {territory}, so moving there is \
                 `{{found-by-land territory:{territory}}}`"
            ),
            Rejection::AlreadyFounded { territory } => write!(
                out,
                "territory {territory} is already founded, so getting there is `move <unit> {territory}`"
            ),
            Rejection::AlreadyUsed(kind) => write!(
                out,
                "that {kind} has already been used this turn; it can go on the next one"
            ),
            Rejection::NoRoomForAnother { territory, kind } => write!(
                out,
                "territory {territory} has as many {} things as it can hold",
                kind.name()
            ),
            Rejection::NotControlled(id) => write!(out, "you do not control territory {id}"),
            Rejection::AlreadyControlled(id) => write!(out, "you already control territory {id}"),
            Rejection::NoCells(kind) => write!(out, "that {kind} has no energy cells left"),
            Rejection::NotEnoughForce {
                territory,
                force,
                needed,
            } => write!(
                out,
                "taking territory {territory} needs more than {needed} force, and you bring {force}"
            ),
            Rejection::NotEnoughResource {
                territory,
                resource,
                held,
                needed,
            } => write!(
                out,
                "territory {territory} has {held} {resource} and that needs {needed}"
            ),
            Rejection::NotEnoughLabor {
                territory,
                available,
                needed,
            } => write!(
                out,
                "territory {territory} has {available} labor left this turn and that needs {needed}"
            ),
            Rejection::NotEnoughCitizens {
                territory,
                held,
                needed,
            } => write!(
                out,
                "territory {territory} has {held} citizens and that needs {needed}"
            ),
            Rejection::NotAboveThatTerritory { kind, above, asked } => write!(
                out,
                "that {kind} is above territory {above}, so it cannot land on territory {asked}"
            ),
            Rejection::NoRoomForExtractor {
                territory,
                resource,
            } => write!(
                out,
                "territory {territory} has no room for another {resource} extractor"
            ),
            Rejection::ResourceNotNamed(kind) => {
                write!(out, "say which resource the {kind} works")
            }
            Rejection::AlreadyHasGarrison(id) => {
                write!(
                    out,
                    "territory {id} already has a garrison, and may have only one"
                )
            }
            Rejection::NoGarrison(id) => write!(out, "territory {id} has no garrison"),
            Rejection::NoYard(id) => write!(out, "territory {id} has no yard"),
            Rejection::NothingToWorkAt {
                territory,
                structure,
            } => write!(out, "territory {territory} has no {structure} to work at"),
            Rejection::NotThatManyToWork {
                territory,
                structure,
                available,
                asked,
            } => write!(
                out,
                "territory {territory} has {available} {structure} able to be worked and you asked for {asked}"
            ),
            Rejection::GarrisonIsNotBuilt => write!(
                out,
                "a garrison is what a founding unit becomes; it is not built"
            ),
            Rejection::CannotLand(kind) => {
                write!(out, "a {kind} cannot come down from orbit")
            }
            Rejection::NotOnThePlanet(kind) => write!(out, "that {kind} is not on the planet"),
            Rejection::PlanetAlreadyCreated => write!(out, "there is already a planet"),
            Rejection::NoSuchPlanetSize(word) => {
                write!(out, "there is no planet size called {word}")
            }
        }
    }
}

impl std::error::Error for Rejection {}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rule this file exists for: a rejection is phrased in the game's terms. If one
    /// of these ever mentions a column or a token, the layers have leaked into each other.
    #[test]
    fn a_rejection_reads_as_the_game_and_never_as_the_parser() {
        let samples = [
            Rejection::NotAdjacent {
                from: TerritoryId(1),
                to: TerritoryId(7),
            },
            Rejection::NotEnoughResource {
                territory: TerritoryId(3),
                resource: Resource::Metal,
                held: 4,
                needed: 30,
            },
            Rejection::NoUnitAvailable {
                kind: UnitKind::Ark,
                where_from: "in orbit",
            },
        ];
        for rejection in samples {
            let said = rejection.to_string();
            for parser_word in ["token", "column", "expected", "parse", "syntax", "grammar"] {
                assert!(
                    !said.contains(parser_word),
                    "`{said}` talks about the parser"
                );
            }
            assert!(!said.is_empty());
        }
    }

    #[test]
    fn a_rejection_names_the_thing_that_was_wrong() {
        assert_eq!(
            Rejection::NotAdjacent {
                from: TerritoryId(1),
                to: TerritoryId(7)
            }
            .to_string(),
            "territory 7 is not adjacent to territory 1"
        );
        assert_eq!(
            Rejection::NotEnoughResource {
                territory: TerritoryId(3),
                resource: Resource::Metal,
                held: 4,
                needed: 30
            }
            .to_string(),
            "territory 3 has 4 metal and that needs 30"
        );
    }
}
