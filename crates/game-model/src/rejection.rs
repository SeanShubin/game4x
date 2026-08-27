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
    NoFreeNode {
        territory: TerritoryId,
        resource: Resource,
    },
    /// A resource has to be named to know which node an extractor works.
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
            Rejection::NoFreeNode {
                territory,
                resource,
            } => write!(
                out,
                "every {resource} node in territory {territory} already has an extractor"
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
