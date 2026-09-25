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
    ///
    /// `spec/console.md`: *a rejection names what was wrong, where, and what was expected
    /// instead.* It named the first two - `S-155`, and the rule had been promoted and unkept.
    ///
    /// **Nothing bold sits between that file name and its quotation**, which is not a style
    /// choice: `CLAUDE.md` records that the checker reads the `**` closing a span as the start
    /// of the quoted text, and two drafts of this comment were reported wrong for it.
    ///
    /// **Every category here is a closed set**, so the list is read off the type rather than
    /// written beside it. A list in a message is a second copy of the thing it describes, and
    /// `P-542` renaming the planet sizes is what a copy would have survived wrongly.
    Unknown {
        what: &'static str,
        word: String,
        expected: Vec<String>,
    },
    /// The binding table asked the syntax tree for something it does not carry.
    Malformed(Failure),
}

impl std::fmt::Display for Misreading {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Misreading::Unknown {
                what,
                word,
                expected,
            } if expected.is_empty() => write!(out, "there is no {what} called {word}"),
            Misreading::Unknown {
                what,
                word,
                expected,
            } => write!(
                out,
                "there is no {what} called {word} - expected one of {}",
                expected.join(", ")
            ),
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
            expected: Biome::ALL.iter().map(|it| it.name().to_string()).collect(),
        })
    };
    let resource = |hole: &str| -> Result<Resource, Misreading> {
        let word = utterance.name(hole)?;
        Resource::named(word).ok_or_else(|| Misreading::Unknown {
            what: "resource",
            word: word.to_string(),
            expected: Resource::ALL
                .iter()
                .map(|it| it.name().to_string())
                .collect(),
        })
    };
    // **A word that has to name one of the two units, and it came back with `P-328`.**
    // `structure` and the optional-resource lookup are still gone, because `build-extractor`
    // and `build-yard` carry the kind in the name - but `move` is one word and one recipe,
    // so which unit moves is a value again.
    let unit = |hole: &str| -> Result<UnitKind, Misreading> {
        let word = utterance.name(hole)?;
        UnitKind::named(word).ok_or_else(|| Misreading::Unknown {
            what: "unit",
            word: word.to_string(),
            expected: UnitKind::ALL
                .iter()
                .map(|it| it.name().to_string())
                .collect(),
        })
    };

    // **`structure` and an optional-resource lookup used to sit here and are gone.**
    // Each turned a word the player had put in a positional hole into a kind, and each could
    // fail on a word that was simply not a kind - `land colonizer 1`. `P-323` made the kind
    // part of the command's name, so the grammar refuses `{deploy colonizer territory:1}`
    // before any of this runs, and the failure says which command was meant rather than which
    // word was unrecognised.
    //
    // **Three ways to be wrong became one**, and the one that survives is the parser's, which
    // already carries a line, a column and the command it was found inside - `P-215`.

    let meaning = match utterance.form {
        form::CREATE_PLANET => {
            let word = utterance.name("size")?;
            let size = size_named(word).ok_or_else(|| Misreading::Unknown {
                what: "planet size",
                word: word.to_string(),
                expected: PlanetSize::ALL.into_iter().map(|it| it.name()).collect(),
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
        form::ADD_ARK => Meaning::Change(Transition::AddUnitToOrbit {
            kind: UnitKind::Ark,
            above: territory("territory")?,
        }),
        form::ADD_PIONEER => Meaning::Change(Transition::AddUnitToOrbit {
            kind: UnitKind::Pioneer,
            above: territory("territory")?,
        }),
        form::START => Meaning::Change(Transition::Start),

        // **The kind comes from the form now, not from a hole.** `P-323`: a command is named
        // for the recipe it fires, and `deploy ark` is the recipe's own name - so which unit
        // is deployed is part of what the player said rather than a word this had to look up
        // and could fail to recognise.
        form::DEPLOY_ARK => Meaning::Change(Transition::Land {
            kind: UnitKind::Ark,
            territory: territory("territory")?,
        }),
        form::LAUNCH_ARK => Meaning::Change(Transition::Launch {
            territory: territory("territory")?,
        }),
        // **The orbit is worked out and not bound** - `spec/console.md`, and `P-552`'s rows say
        // `above $where` for the same reason.
        form::MINE_ENERGY => Meaning::Change(Transition::MineEnergy {
            territory: territory("territory")?,
        }),
        // **The one lookup that came back.** `P-328` makes a command's name one word, and
        // `move` is the recipe's whole name - so the kind cannot ride in it the way it does in
        // `deploy-ark`, and a word has to be read and turned into a kind again. `C-56` is the
        // question of whether `unit:ark` is the right field at all.
        form::MOVE => Meaning::Change(Transition::Move {
            kind: unit("unit")?,
            from: territory("from")?,
            to: territory("to")?,
        }),
        // `P-214`: one command per recipe, so the player says which of the two this is
        // rather than the model deciding by looking at the ground.
        form::FOUND_BY_LAND => Meaning::Change(Transition::FoundByLand {
            territory: territory("territory")?,
        }),
        form::BUILD_STORE => Meaning::Change(Transition::BuildStore {
            resource: resource("resource")?,
            territory: territory("territory")?,
        }),
        form::BUILD_EXTRACTOR => Meaning::Change(Transition::Build {
            structure: StructureKind::Extractor,
            territory: territory("territory")?,
            resource: Some(resource("resource")?),
        }),
        form::BUILD_YARD => Meaning::Change(Transition::Build {
            structure: StructureKind::Yard,
            territory: territory("territory")?,
            resource: None,
        }),
        form::PRODUCE_PIONEER => Meaning::Change(Transition::ProducePioneer {
            territory: territory("territory")?,
        }),
        // **The count is `repeat` now, and one firing makes one labor.** `P-323`: a repeat
        // is a count of firings rather than an argument of the recipe, so the transition is
        // what one firing does and `Session::run` applies it that many times.
        form::CREATE_LABOR => Meaning::Change(Transition::CreateLabor {
            count: 1,
            territory: territory("territory")?,
        }),
        form::WORK => Meaning::Change(Transition::Work {
            count: 1,
            structure: StructureKind::Extractor,
            territory: territory("territory")?,
            resource: Some(resource("resource")?),
        }),
        form::END_TURN => Meaning::Change(Transition::EndTurn),

        form::SHOW_TERRITORY => Meaning::Show(Subject::Territory(territory("id")?)),
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
            // **The one category that is not a closed set of values but of forms**, and
            // it is unreachable while the grammar and this table agree - `crate::tests` is
            // what keeps them agreeing. Left without a list rather than given an empty one,
            // because twenty-six command names in a refusal is not what a reader wants and
            // this refusal is a defect in the tables rather than in what was typed.
            return Err(Misreading::Unknown {
                what: "command",
                word: other.to_string(),
                expected: Vec::new(),
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
        form::ADD_ARK,
        form::ADD_PIONEER,
        form::START,
        form::DEPLOY_ARK,
        form::LAUNCH_ARK,
        form::MINE_ENERGY,
        form::MOVE,
        form::FOUND_BY_LAND,
        form::BUILD_STORE,
        form::BUILD_EXTRACTOR,
        form::BUILD_YARD,
        form::PRODUCE_PIONEER,
        form::CREATE_LABOR,
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
            meaning("{deploy-ark territory:1}"),
            Meaning::Change(Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1)
            })
        );
        // **A repeat is not part of the transition**, so a command carrying one becomes the
        // same transition as one without. `P-323`: it is a count of firings rather than an
        // argument of the recipe, and `Session::run` is what fires it that many times.
        let once = meaning("{work territory:3 resource:metal}");
        assert_eq!(
            once,
            Meaning::Change(Transition::Work {
                count: 1,
                structure: StructureKind::Extractor,
                territory: TerritoryId(3),
                resource: Some(Resource::Metal),
            })
        );
        assert_eq!(
            meaning("{work territory:3 resource:metal repeat:4}"),
            once,
            "a repeat changes how many times, not what"
        );
        assert_eq!(meaning("{end-turn}"), Meaning::Change(Transition::EndTurn));
    }

    #[test]
    fn asking_a_question_is_not_a_change() {
        assert_eq!(
            meaning("{show-territory id:5}"),
            Meaning::Show(Subject::Territory(TerritoryId(5)))
        );
        assert_eq!(
            meaning("{help command:move}"),
            Meaning::Help(Some("move".to_string()))
        );
        assert_eq!(meaning("{help}"), Meaning::Help(None));
        assert_eq!(meaning("{history}"), Meaning::History);
    }

    /// A word in the right place that names nothing is a mistake about the game, and is
    /// reported as one.
    ///
    /// **`P-323` moved one of these out of reach and this is the other.** `build refinery 3`
    /// used to reach here and be told *there is no structure called refinery*; the kind is
    /// part of the command's name now, so `{build refinery territory:3}` is refused by the
    /// grammar with a position and a list of what could have been written. **A resource is
    /// still a value**, so a word that names none is still the game's mistake to report.
    #[test]
    fn a_word_that_names_nothing_is_reported_in_the_games_terms() {
        let utterance = parse_line(&grammar(), "{build-extractor territory:3 resource:gold}", 1)
            .unwrap()
            .unwrap();
        let misread = interpret(&utterance).unwrap_err();
        assert_eq!(
            misread.to_string(),
            "there is no resource called gold - expected one of food, metal, energy"
        );
    }

    /// A kind that is not a kind never reaches the binding at all now.
    ///
    /// **The half that moved, checked where it moved to.** Without this the test above reads
    /// as though one of the two cases had been deleted rather than relocated.
    #[test]
    fn a_kind_that_names_nothing_is_refused_by_the_grammar() {
        let failure = parse_line(&grammar(), "{build-refinery territory:3}", 1)
            .expect_err("`build-refinery` is not a command");
        // **At the name, which is now one word.** `P-328` made a command's name a single
        // token, so a name that is not one fails at the name rather than partway through it.
        assert_eq!(failure.position.column, 2, "at the name, not at the brace");
        assert!(
            failure
                .expected
                .iter()
                .any(|what| what == "build-extractor"),
            "and it says what could have been written: {failure}"
        );
    }

    #[test]
    fn a_misspelled_optional_resource_is_reported_rather_than_ignored() {
        let utterance = parse_line(
            &grammar(),
            "{build-extractor territory:3 resource:metel}",
            1,
        )
        .unwrap()
        .unwrap();
        let misread = interpret(&utterance).unwrap_err();
        assert_eq!(
            misread.to_string(),
            "there is no resource called metel - expected one of food, metal, energy"
        );
    }

    /// The world is the same one every time, or a history would stop being a save file:
    /// the biomes are recomputed from `create planet <size>` rather than recorded.
    #[test]
    fn creating_a_planet_twice_describes_the_same_world() {
        assert_eq!(
            meaning("{create-planet size:small-32}"),
            meaning("{create-planet size:small-32}")
        );
    }

    /// A planet is not one biome painted over twelve faces. `spec/planet.md` asks that
    /// nothing in the terrain reveal how the sphere was divided, and a world with a single
    /// biome would reveal nothing because it would say nothing.
    #[test]
    fn a_planet_has_more_than_one_kind_of_ground() {
        let Meaning::Change(Transition::CreatePlanet { biomes, .. }) =
            meaning("{create-planet size:huge-92}")
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
        }) = meaning("{create-planet size:tiny-12}")
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
        let mut reached = [false; 12];
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
            meaning("{create-planet size:tiny-12}")
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

    /// Every refusal over a closed set names what was expected, which is the rule rather than
    /// the message.
    ///
    /// # Why the message test was not enough, measured
    ///
    /// **`spec/console.md`**: *a rejection names what was wrong, where, and what was expected
    /// instead.* `a_size_that_is_not_a_planet_size_is_reported` asserts the exact string the
    /// console produces, and it was green for as long as that string said only what was
    /// wrong. **A test that pins the current message cannot notice the message is missing
    /// something a document requires** - it is the strongest possible statement about what the
    /// code does and says nothing about what it should do.
    ///
    /// **And the rule had been promoted and unkept**, which is how `S-155` came to be filed
    /// against a variant nothing constructs: `Rejection::NoSuchPlanetSize` had been dead since
    /// `P-215` folded three ways of being wrong into the parser's one, so the obvious place to
    /// fix it was not the live path. The research lane measured that; this is what would have
    /// said so.
    ///
    /// # Over every category and the count with it
    ///
    /// Four closed sets reach this refusal - biome, resource, unit, planet size - and each is
    /// asked for a word it does not have. **`command` is the fifth and is excluded by name**:
    /// it is unreachable while the grammar and the binding table agree, and twenty-six command
    /// names in a refusal is not what a reader wants.
    #[test]
    fn every_refusal_over_a_closed_set_says_what_was_expected() {
        let cases = [
            ("{create-planet size:enormous}", "planet size", "tiny-12"),
            ("{set-biome territory:1 biome:swamp}", "biome", "grassland"),
            (
                "{set-resource territory:1 resource:gold extractors:1 density:1}",
                "resource",
                "food",
            ),
            ("{move unit:dragon from:1 to:2}", "unit", "ark"),
        ];
        let mut checked = 0;
        for (line, what, one_of) in cases {
            let Ok(Some(utterance)) = parse_line(&grammar(), line, 1) else {
                panic!("`{line}` does not parse, so this checks nothing");
            };
            let said = interpret(&utterance)
                .expect_err(&format!("`{line}` should be refused"))
                .to_string();
            assert!(
                said.contains(&format!("there is no {what} called")),
                "`{line}` said {said:?}, which does not name what was wrong"
            );
            assert!(
                said.contains("expected one of"),
                "`{line}` said {said:?}, and `spec/console.md` asks for what was expected"
            );
            assert!(
                said.contains(one_of),
                "`{line}` said {said:?}, which does not list {one_of}"
            );
            checked += 1;
        }
        // A count over nothing is the same failure with the sign flipped - `CLAUDE.md`.
        assert_eq!(checked, 4, "four closed sets reach this refusal");
    }

    #[test]
    fn a_size_that_is_not_a_planet_size_is_reported() {
        let utterance = parse_line(&grammar(), "{create-planet size:enormous}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(
            interpret(&utterance).unwrap_err().to_string(),
            "there is no planet size called enormous - expected one of tiny-12, small-32, \
             medium-42, large-72, huge-92"
        );
    }
}
