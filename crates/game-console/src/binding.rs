//! What each command means.
//!
//! This is the only file in the project that knows both a word and a rule. Above it the
//! parser handles words and has never heard of a territory; below it the model handles
//! rules and has never heard of a word. Adding a command is a row here and a form in
//! [`crate::grammar`], and a test asserts the two tables cover the same set.

use command_language::{Failure, Utterance};
use game_model::{Biome, Resource, StructureKind, TerritoryId, Transition, UnitKind};
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
    let biome = |hole: &str| -> Result<Biome, Misreading> {
        let word = utterance.name(hole)?;
        Biome::named(word).ok_or_else(|| Misreading::Unknown {
            what: "biome",
            word: word.to_string(),
        })
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
            let seeds = seeds_for(size);
            let touching = sphere_tessellation::adjacency(&seeds);
            let near: Vec<Vec<usize>> = touching
                .iter()
                .map(|list| list.iter().map(|at| *at as usize).collect())
                .collect();
            // The terrain proposes and `biomes_of` disposes. Two of `spec/planet.md`'s
            // statements are about the arrangement rather than about any one point - a
            // biome is what covers most of a territory's ground, and oceans never isolate
            // land from land - so the solid and the adjacency are both in scope here, and
            // neither is anything the field itself knows about.
            let solid = sphere_tessellation::solid(&seeds, &touching);
            Meaning::Change(Transition::CreatePlanet {
                territories: size.territory_count(),
                adjacency: named(&touching),
                biomes: planet_terrain::biomes_of(&solid, &near, WORLD_SEED),
            })
        }
        form::SET_RESOURCE => Meaning::Change(Transition::SetResource {
            territory: territory("territory")?,
            resource: resource("resource")?,
            extractors: utterance.number("extractors")? as u32,
            density: utterance.number("density")? as u32,
        }),
        form::SET_FORCE => Meaning::Change(Transition::SetForceOfNature {
            territory: territory("territory")?,
            force: utterance.number("force")? as u32,
        }),
        form::SET_BIOME => Meaning::Change(Transition::SetBiome {
            territory: territory("territory")?,
            biome: biome("biome")?,
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
        form::SET_RESOURCE,
        form::SET_FORCE,
        form::SET_BIOME,
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
/// Which world `create planet` builds.
///
/// Taken from `planet-terrain` rather than declared here, because the realistic drawing
/// paints the same field and the two must not be seeded separately - a territory could
/// otherwise be ice in the model while the picture drew jungle over it.
///
/// It is what keeps `history` a save file: the biomes are not written into the history,
/// they are recomputed from `create planet <size>`, so the same commands rebuild the same
/// world on somebody else's machine.
const WORLD_SEED: u64 = planet_terrain::WORLD_SEED;

/// Where each territory sits on the sphere, in id order.
fn seeds_for(size: PlanetSize) -> Vec<sphere_tessellation::vec3::Vec3> {
    sphere_tessellation::icosahedral::canonical_seeds(size.territory_count())
        .expect("every planet size is a Goldberg count; planet-render asserts it")
}

/// The same adjacency, in the model's own names rather than in indices.
fn named(touching: &[Vec<u32>]) -> Vec<Vec<TerritoryId>> {
    touching
        .iter()
        .map(|near| {
            near.iter()
                .map(|at| TerritoryId::from_index(*at as usize))
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

    /// The world is the same one every time, or a history would stop being a save file:
    /// the biomes are recomputed from `create planet <size>` rather than recorded.
    #[test]
    fn creating_a_planet_twice_describes_the_same_world() {
        assert_eq!(
            meaning("create planet small"),
            meaning("create planet small")
        );
    }

    /// A planet is not one biome painted over twelve faces. `spec/planet.md` asks that
    /// nothing in the terrain reveal how the sphere was divided, and a world with a single
    /// biome would reveal nothing because it would say nothing.
    #[test]
    fn a_planet_has_more_than_one_kind_of_ground() {
        let Meaning::Change(Transition::CreatePlanet { biomes, .. }) =
            meaning("create planet huge")
        else {
            panic!("not a create planet");
        };
        let kinds: std::collections::BTreeSet<_> = biomes.iter().collect();
        assert!(kinds.len() > 2, "ninety-two territories and only {kinds:?}");
    }

    /// The planet the release calls for: tiny, which is twelve territories.
    #[test]
    fn creating_a_tiny_planet_makes_twelve_territories_that_all_touch_something() {
        let Meaning::Change(Transition::CreatePlanet {
            territories,
            adjacency,
            biomes,
        }) = meaning("create planet tiny")
        else {
            panic!("not a create planet");
        };
        assert_eq!(territories, 12);
        assert_eq!(adjacency.len(), 12);
        // `spec/planet.md`: each territory has a biome, and it is what the terrain gives
        // it - so there is one per territory, and this is not where it was decided.
        //
        // Not compared against `biome_at`, which is the raw field: a territory whose ground
        // is under water may still have to be land, because no two ocean territories may be
        // adjacent. That resolution is `planet-terrain`'s and is tested there. What this
        // asserts is that the binding asks for it rather than sampling on its own.
        assert_eq!(biomes.len(), 12);
        let seeds = seeds_for(PlanetSize::Tiny);
        let touching = sphere_tessellation::adjacency(&seeds);
        let near: Vec<Vec<usize>> = touching
            .iter()
            .map(|list| list.iter().map(|at| *at as usize).collect())
            .collect();
        let solid = sphere_tessellation::solid(&seeds, &touching);
        assert_eq!(biomes, planet_terrain::biomes_of(&solid, &near, WORLD_SEED));

        // `spec/planet.md`: *oceans never isolate land from land.* Asserted here as well
        // as in `planet-terrain`, because this is the only place that says which world the
        // command builds, and the rule is about the world rather than about the function.
        //
        // It used to assert that no two oceans touched. That is a stronger condition the
        // specification has since dropped: adjacent oceans are legal, an island is what is
        // forbidden, and water that cannot pool costs every coastline and every sea.
        let land: Vec<usize> = (0..12).filter(|at| biomes[*at].is_claimable()).collect();
        assert!(!land.is_empty(), "a planet with no land on it");
        let mut reached = vec![false; 12];
        let mut queue = vec![land[0]];
        reached[land[0]] = true;
        while let Some(at) = queue.pop() {
            for beside in &near[at] {
                if !reached[*beside] && biomes[*beside].is_claimable() {
                    reached[*beside] = true;
                    queue.push(*beside);
                }
            }
        }
        let stranded: Vec<usize> = land.into_iter().filter(|at| !reached[*at]).collect();
        assert!(stranded.is_empty(), "water cut off {stranded:?}");
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
