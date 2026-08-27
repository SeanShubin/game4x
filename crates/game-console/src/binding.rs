//! What each command means.
//!
//! This is the only file in the project that knows both a word and a rule. Above it the
//! parser handles words and has never heard of a territory; below it the model handles
//! rules and has never heard of a word. Adding a command is a row here and a form in
//! [`crate::grammar`], and a test asserts the two tables cover the same set.

use command_language::{Failure, Utterance};
use game_model::{Resource, StructureKind, TerritoryId, Transition, UnitKind};
use planet_model::PlanetSize;

use crate::grammar::form;

/// What a command turns out to mean.
///
/// Most commands are a transition. The rest either ask a question, which changes nothing,
/// or call in another file. Keeping them apart in the type is what stops a query
/// accidentally becoming a way to change state - `spec/invariants.md` allows exactly one
/// of those, and it is [`Meaning::Change`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Meaning {
    Change(Transition),
    Show(Subject),
    Help(Option<String>),
    History,
    Run(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Subject {
    Territory(TerritoryId),
    Planet,
    Orbit,
    Units,
    Turn,
}

/// Why a command could not be understood, before the rules ever see it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Misreading {
    /// A word was in the right place but names nothing in the game.
    Unknown { what: &'static str, word: String },
    /// The binding table asked the syntax tree for something it does not carry.
    Malformed(Failure),
}

impl std::fmt::Display for Misreading {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Misreading::Unknown { what, word } => write!(out, "there is no {what} called {word}"),
            Misreading::Malformed(failure) => write!(out, "{failure}"),
        }
    }
}

impl From<Failure> for Misreading {
    fn from(failure: Failure) -> Self {
        Misreading::Malformed(failure)
    }
}

/// Reads one parsed command as a meaning.
pub fn interpret(utterance: &Utterance) -> Result<Meaning, Misreading> {
    let territory = |hole: &str| -> Result<TerritoryId, Misreading> {
        Ok(TerritoryId(utterance.number(hole)? as u32))
    };
    let resource = |hole: &str| -> Result<Resource, Misreading> {
        let word = utterance.name(hole)?;
        Resource::named(word).ok_or_else(|| Misreading::Unknown {
            what: "resource",
            word: word.to_string(),
        })
    };
    let unit = |hole: &str| -> Result<UnitKind, Misreading> {
        let word = utterance.name(hole)?;
        UnitKind::named(word).ok_or_else(|| Misreading::Unknown {
            what: "unit",
            word: word.to_string(),
        })
    };
    let structure = |hole: &str| -> Result<StructureKind, Misreading> {
        let word = utterance.name(hole)?;
        StructureKind::named(word).ok_or_else(|| Misreading::Unknown {
            what: "structure",
            word: word.to_string(),
        })
    };
    // An optional resource that is present but misspelled is a mistake worth reporting,
    // rather than one silently treated as absent.
    let optional_resource = || -> Result<Option<Resource>, Misreading> {
        match utterance.optional_name("resource") {
            None => Ok(None),
            Some(word) => Resource::named(word)
                .map(Some)
                .ok_or_else(|| Misreading::Unknown {
                    what: "resource",
                    word: word.to_string(),
                }),
        }
    };

    let meaning = match utterance.form {
        form::CREATE_PLANET => {
            let word = utterance.name("size")?;
            let size = size_named(word).ok_or_else(|| Misreading::Unknown {
                what: "planet size",
                word: word.to_string(),
            })?;
            Meaning::Change(Transition::CreatePlanet {
                territories: size.territory_count(),
                adjacency: adjacency_for(size),
            })
        }
        form::ADD_NODE => Meaning::Change(Transition::AddNode {
            territory: territory("territory")?,
            resource: resource("resource")?,
            density: utterance.number("density")? as u32,
        }),
        form::SET_FORCE => Meaning::Change(Transition::SetForceOfNature {
            territory: territory("territory")?,
            force: utterance.number("force")? as u32,
        }),
        form::ADD_UNIT => Meaning::Change(Transition::AddUnitToOrbit {
            kind: unit("unit")?,
        }),
        form::START => Meaning::Change(Transition::Start),

        form::LAND => Meaning::Change(Transition::Land {
            kind: unit("unit")?,
            territory: territory("territory")?,
        }),
        form::LAUNCH => Meaning::Change(Transition::Launch {
            kind: unit("unit")?,
        }),
        form::MOVE => Meaning::Change(Transition::Move {
            kind: unit("unit")?,
            territory: territory("territory")?,
        }),
        form::BUILD => Meaning::Change(Transition::Build {
            structure: structure("structure")?,
            territory: territory("territory")?,
            resource: optional_resource()?,
        }),
        form::PRODUCE => Meaning::Change(Transition::Produce {
            kind: unit("unit")?,
            territory: territory("territory")?,
        }),
        form::WORK => Meaning::Change(Transition::Work {
            count: utterance.number("count")? as u32,
            structure: structure("structure")?,
            territory: territory("territory")?,
            resource: optional_resource()?,
        }),
        form::END_TURN => Meaning::Change(Transition::EndTurn),

        form::SHOW_TERRITORY => Meaning::Show(Subject::Territory(territory("territory")?)),
        form::SHOW_PLANET => Meaning::Show(Subject::Planet),
        form::SHOW_ORBIT => Meaning::Show(Subject::Orbit),
        form::SHOW_UNITS => Meaning::Show(Subject::Units),
        form::SHOW_TURN => Meaning::Show(Subject::Turn),
        form::HELP => Meaning::Help(utterance.optional_name("command").map(str::to_string)),
        form::HISTORY => Meaning::History,
        form::RUN => Meaning::Run(utterance.name("file")?.to_string()),

        // Unreachable while the two tables agree, and `crate::tests` is what keeps them
        // agreeing. Reported as data rather than panicking, because this layer promises
        // never to unwind on input.
        other => {
            return Err(Misreading::Unknown {
                what: "command",
                word: other.to_string(),
            });
        }
    };
    Ok(meaning)
}

/// Every form this table handles. Compared against the grammar in one test.
pub fn handled() -> Vec<&'static str> {
    vec![
        form::CREATE_PLANET,
        form::ADD_NODE,
        form::SET_FORCE,
        form::ADD_UNIT,
        form::START,
        form::LAND,
        form::LAUNCH,
        form::MOVE,
        form::BUILD,
        form::PRODUCE,
        form::WORK,
        form::END_TURN,
        form::SHOW_TERRITORY,
        form::SHOW_PLANET,
        form::SHOW_ORBIT,
        form::SHOW_UNITS,
        form::SHOW_TURN,
        form::HELP,
        form::HISTORY,
        form::RUN,
    ]
}

fn size_named(word: &str) -> Option<PlanetSize> {
    PlanetSize::ALL.into_iter().find(|size| size.name() == word)
}

/// Which territories touch, worked out from the sphere and handed to the model as a
/// graph of ids.
///
/// The model has no geometry and no floating point, so this is where a planet stops being
/// a shape and becomes a set of integers. Everything below this line is whole numbers.
fn adjacency_for(size: PlanetSize) -> Vec<Vec<TerritoryId>> {
    let seeds = sphere_tessellation::icosahedral::canonical_seeds(size.territory_count())
        .expect("every planet size is a Goldberg count; planet-render asserts it");
    sphere_tessellation::adjacency(&seeds)
        .into_iter()
        .map(|near| {
            near.into_iter()
                .map(|at| TerritoryId::from_index(at as usize))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::grammar;
    use command_language::parse_line;

    fn meaning(line: &str) -> Meaning {
        let utterance = parse_line(&grammar(), line, 1).unwrap().unwrap();
        interpret(&utterance).unwrap()
    }

    #[test]
    fn a_command_becomes_the_transition_it_names() {
        assert_eq!(
            meaning("land ark 1"),
            Meaning::Change(Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1)
            })
        );
        assert_eq!(
            meaning("work 4 extractor 3 metal"),
            Meaning::Change(Transition::Work {
                count: 4,
                structure: StructureKind::Extractor,
                territory: TerritoryId(3),
                resource: Some(Resource::Metal),
            })
        );
        assert_eq!(meaning("end turn"), Meaning::Change(Transition::EndTurn));
    }

    #[test]
    fn asking_a_question_is_not_a_change() {
        assert_eq!(
            meaning("show territory 5"),
            Meaning::Show(Subject::Territory(TerritoryId(5)))
        );
        assert_eq!(
            meaning("help move"),
            Meaning::Help(Some("move".to_string()))
        );
        assert_eq!(meaning("help"), Meaning::Help(None));
        assert_eq!(meaning("history"), Meaning::History);
    }

    /// A word in the right place that names nothing is a mistake about the game, and is
    /// reported as one.
    #[test]
    fn a_word_that_names_nothing_is_reported_in_the_games_terms() {
        let utterance = parse_line(&grammar(), "build refinery 3", 1)
            .unwrap()
            .unwrap();
        let misread = interpret(&utterance).unwrap_err();
        assert_eq!(misread.to_string(), "there is no structure called refinery");
    }

    #[test]
    fn a_misspelled_optional_resource_is_reported_rather_than_ignored() {
        let utterance = parse_line(&grammar(), "build extractor 3 metel", 1)
            .unwrap()
            .unwrap();
        let misread = interpret(&utterance).unwrap_err();
        assert_eq!(misread.to_string(), "there is no resource called metel");
    }

    /// The planet the release calls for: tiny, which is twelve territories.
    #[test]
    fn creating_a_tiny_planet_makes_twelve_territories_that_all_touch_something() {
        let Meaning::Change(Transition::CreatePlanet {
            territories,
            adjacency,
        }) = meaning("create planet tiny")
        else {
            panic!("not a create planet");
        };
        assert_eq!(territories, 12);
        assert_eq!(adjacency.len(), 12);
        // A dodecahedron: every face touches exactly five others.
        for (at, near) in adjacency.iter().enumerate() {
            assert_eq!(
                near.len(),
                5,
                "territory {} has {} neighbours",
                at + 1,
                near.len()
            );
        }
    }

    /// Adjacency has to read the same from both ends, or moving somewhere would not let
    /// you move back.
    #[test]
    fn adjacency_agrees_with_itself() {
        let Meaning::Change(Transition::CreatePlanet { adjacency, .. }) =
            meaning("create planet tiny")
        else {
            panic!("not a create planet");
        };
        for (at, near) in adjacency.iter().enumerate() {
            let here = TerritoryId::from_index(at);
            for other in near {
                assert!(
                    adjacency[other.index()].contains(&here),
                    "{here} lists {other} but not the other way round"
                );
            }
        }
    }

    #[test]
    fn a_size_that_is_not_a_planet_size_is_reported() {
        let utterance = parse_line(&grammar(), "create planet enormous", 1)
            .unwrap()
            .unwrap();
        assert_eq!(
            interpret(&utterance).unwrap_err().to_string(),
            "there is no planet size called enormous"
        );
    }
}
