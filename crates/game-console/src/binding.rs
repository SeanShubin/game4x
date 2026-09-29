//! What each command means.
//!
//! This is the only file in the project that knows both a word and a rule. Above it the
//! parser handles words and has never heard of a territory; below it the model handles
//! rules and has never heard of a word.
//!
//! # Two paths, since 2026-09-29
//!
//! **Sean**: *build rows before `start`, fire rules after.* A design command turns into rows -
//! [`crate::world`] is where they are made - and a play command turns into a command row the
//! engine fires. **They were one `Transition` and they are two operations**: one states what is
//! there, the other says what may be done about it.
//!
//! **Only the design forms are named here.** What a player may fire is read from the rules by
//! [`crate::rules`], so adding a rule adds a command and this file does not hear about it.

use std::collections::BTreeMap;

use command_language::{Failure, Utterance};
use game_model::notation::Row;
use planet_model::{Biome, PlanetSize};

use crate::grammar::form;
use crate::world;

/// What a command turns out to mean.
///
/// **Two of these change the game and they change it differently.** [`Meaning::Build`] states
/// rows into a world being designed; [`Meaning::Fire`] hands a command row to the engine. The
/// rest either ask a question, which changes nothing, or call in another file - and keeping them
/// apart in the type is what stops a query becoming a way to change state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Meaning {
    /// Rows to state into the world, before `{start}`.
    Build(Vec<Row>),
    /// A rule to fire against the world, after it.
    Fire(Row),
    /// End the design phase and hand what was built to the engine.
    Start,
    Show(Subject),
    Help(Option<String>),
    History,
    Run(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Subject {
    Territory(u32),
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
    let territory = |hole: &str| -> Result<u32, Misreading> { Ok(utterance.number(hole)? as u32) };
    let biome = |hole: &str| -> Result<Biome, Misreading> {
        let word = utterance.name(hole)?;
        Biome::named(word).ok_or_else(|| Misreading::Unknown {
            what: "biome",
            word: word.to_string(),
            expected: Biome::ALL.iter().map(|it| it.name().to_string()).collect(),
        })
    };

    let meaning = match utterance.form {
        // -- Before `start`: what is there ---------------------------------------------------
        form::CREATE_PLANET => {
            let word = utterance.name("size")?;
            let size = size_named(word).ok_or_else(|| Misreading::Unknown {
                what: "planet size",
                word: word.to_string(),
                expected: PlanetSize::ALL.into_iter().map(|it| it.name()).collect(),
            })?;
            Meaning::Build(world::planet(size))
        }
        // **The extractor count has nowhere to go**, which `crate::world::deposit` says at
        // length: `{capacity of:deposit for:extractor ...}` is one row for the whole world.
        // `C-176` removes this command rather than this lane inventing a home for it.
        form::SET_RESOURCE => {
            let word = utterance.name("resource")?;
            let what = kind_in("resource", word).ok_or_else(|| Misreading::Unknown {
                what: "kind",
                word: word.to_string(),
                expected: every_kind_in("resource"),
            })?;
            Meaning::Build(world::deposit(
                territory("territory")?,
                &what,
                utterance.number("density")? as u32,
            ))
        }
        form::SET_BIOME => Meaning::Build(world::terrain(territory("territory")?, biome("biome")?)),
        form::ADD_ARK => Meaning::Build(world::in_orbit(territory("territory")?, "ark")),
        form::ADD_PIONEER => Meaning::Build(world::in_orbit(territory("territory")?, "pioneer")),
        form::START => Meaning::Start,

        // -- Questions and files, which change nothing ---------------------------------------
        form::SHOW_TERRITORY => Meaning::Show(Subject::Territory(territory("id")?)),
        form::SHOW_PLANET => Meaning::Show(Subject::Planet),
        form::SHOW_ORBIT => Meaning::Show(Subject::Orbit),
        form::SHOW_UNITS => Meaning::Show(Subject::Units),
        form::SHOW_TURN => Meaning::Show(Subject::Turn),
        form::HELP => Meaning::Help(utterance.optional_name("command").map(str::to_string)),
        form::HISTORY => Meaning::History,
        form::RUN => Meaning::Run(utterance.name("file")?.to_string()),

        // -- After `start`: a rule, and this file does not know which ones there are ---------
        //
        // **Every other form is a rule's**, because `crate::grammar` builds one form per rule
        // out of the foundation. So a rule added to `spec/data/rules.4x` arrives here as a form
        // this has never heard of and is fired anyway, which is `D-1`.
        rule => Meaning::Fire(fired(rule, utterance)?),
    };
    Ok(meaning)
}

/// A rule's command, as the row the engine fires.
///
/// **The inputs come from the rule and the values from what was typed.** An input typed as a
/// place is a number and every other is a word - `crate::rules` is where that is decided, and
/// this reads it rather than repeating it.
fn fired(rule: &str, utterance: &Utterance) -> Result<Row, Misreading> {
    let known = crate::rules::playable();
    let Some(playable) = known.iter().find(|it| it.name == rule) else {
        // **Unreachable while the grammar is built from the same rules this reads.** Reported
        // as data rather than panicking, because this layer promises never to unwind on input.
        return Err(Misreading::Unknown {
            what: "command",
            word: rule.to_string(),
            expected: known.iter().map(|it| it.name.clone()).collect(),
        });
    };

    let mut values = BTreeMap::new();
    for (input, of) in &playable.inputs {
        // **A place is written as its number and a kind as its name**, and both arrive as an
        // id - because the foundation names nothing twice, so `{move what:28 ...}` is a scout
        // and `{deploy what:54 ...}` is a pioneer.
        //
        // **The word is resolved here and refused here.** A kind that is not in the family the
        // input is typed as is a misreading rather than a refusal from the engine, so the
        // player is told which words would have done - which is what `Misreading::Unknown`
        // carries and a rule's refusal does not.
        let said = if of == "place" {
            utterance.number(input)?.to_string()
        } else {
            let word = utterance.name(input)?;
            kind_in(of, word).ok_or_else(|| Misreading::Unknown {
                what: "kind",
                word: word.to_string(),
                expected: every_kind_in(of),
            })?
        };
        values.insert(input.clone(), said);
    }
    Ok(Row {
        relation: rule.to_string(),
        values,
    })
}

/// Every form this table names, which is the design phase and the questions.
///
/// **The play forms are not here and that is the point.** They come from the rules, so a table
/// of them would be a second list of what `spec/data/rules.4x` already says - and the test that
/// compares this with the grammar now compares the written half with the written half, and asks
/// `crate::rules` for the rest.
pub fn handled() -> Vec<&'static str> {
    vec![
        form::CREATE_PLANET,
        form::SET_RESOURCE,
        form::SET_BIOME,
        form::ADD_ARK,
        form::ADD_PIONEER,
        form::START,
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

/// The id of a kind, where the family an input is typed as admits it.
///
/// **A family admits its members and a kind admits itself.** `move`'s `what` is typed as `unit`
/// and a scout is one. So this asks the data rather than holding a table - which is what lets a
/// kind added to a family be typed without this being edited.
fn kind_in(family: &str, word: &str) -> Option<String> {
    let rows = game_model::foundation::rows();
    let id = |name: &str| {
        rows.iter()
            .find(|it| it.relation == "relation" && it.value("name") == Some(name))
            .and_then(|it| it.value("id").map(str::to_string))
    };
    let (wanted, family) = (id(word)?, id(family)?);
    if wanted == family {
        return Some(wanted);
    }
    rows.iter()
        .any(|it| {
            it.relation == "member"
                && it.value("kind") == Some(wanted.as_str())
                && it.value("family") == Some(family.as_str())
        })
        .then_some(wanted)
}

/// Every word that would have done, so a refusal says what was expected.
fn every_kind_in(family: &str) -> Vec<String> {
    let rows = game_model::foundation::rows();
    let named: BTreeMap<String, String> = rows
        .iter()
        .filter(|it| it.relation == "relation")
        .filter_map(|it| Some((it.value("id")?.to_string(), it.value("name")?.to_string())))
        .collect();
    let Some(family) = named
        .iter()
        .find(|(_, name)| name.as_str() == family)
        .map(|(id, _)| id)
    else {
        return Vec::new();
    };
    let mut found: Vec<String> = rows
        .iter()
        .filter(|it| it.relation == "member" && it.value("family") == Some(family.as_str()))
        .filter_map(|it| named.get(it.value("kind").unwrap_or_default()).cloned())
        .collect();
    if found.is_empty() {
        found.extend(named.get(family).cloned());
    }
    found.sort();
    found
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

    fn misread(line: &str) -> String {
        let utterance = parse_line(&grammar(), line, 1).unwrap().unwrap();
        interpret(&utterance).unwrap_err().to_string()
    }

    /// The id a name has in the foundation, so a test says `ark` and compares what the engine
    /// would read.
    fn id(name: &str) -> String {
        game_model::foundation::rows()
            .iter()
            .find(|it| it.relation == "relation" && it.value("name") == Some(name))
            .and_then(|it| it.value("id").map(str::to_string))
            .unwrap_or_else(|| panic!("no relation `{name}`"))
    }

    fn fired(relation: &str, cells: &[(&str, &str)]) -> Meaning {
        Meaning::Fire(Row {
            relation: relation.to_string(),
            values: cells
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        })
    }

    /// **A rule's command becomes the row that fires it**, and the row is what the data reads.
    ///
    /// **Every reference is an id, because the foundation names nothing twice.** `what:ark` is
    /// typed as `ark` and arrives as the relation's id - which is what the foundation tests
    /// already hold: `{deploy where:1 what:54}`.
    #[test]
    fn a_rule_becomes_the_row_that_fires_it() {
        assert_eq!(
            meaning("{deploy where:2 what:ark}"),
            fired("deploy", &[("where", "2"), ("what", &id("ark"))])
        );
        assert_eq!(
            meaning("{move what:scout from:1 to:3}"),
            fired(
                "move",
                &[("what", &id("scout")), ("from", "1"), ("to", "3")]
            )
        );
        assert_eq!(meaning("{end-turn}"), fired("end-turn", &[]));

        // **A repeat is not part of the row.** `P-323`: it is a count of firings rather than an
        // argument of the rule, and `Session::run` is what fires it that many times.
        let once = meaning("{work where:3 what:metal}");
        assert_eq!(
            once,
            fired("work", &[("where", "3"), ("what", &id("metal"))])
        );
        assert_eq!(
            meaning("{work where:3 what:metal repeat:4}"),
            once,
            "a repeat changes how many times, not what"
        );
    }

    /// **A rule the data declares is a command, and this lane wrote none of them down.**
    ///
    /// **`D-1` is what this is for**: a recipe changes by editing a data file, so a rule added
    /// there is sayable without Rust being edited. The eleven verbs that used to be written into
    /// `grammar.rs` are gone, and four of them had drifted from the rule they fired.
    #[test]
    fn every_rule_the_data_declares_can_be_said() {
        let known = crate::rules::playable();
        assert!(known.len() >= 10, "only {} rule(s)", known.len());
        let mut said = 0;
        for rule in &known {
            let line = if rule.inputs.is_empty() {
                format!("{{{}}}", rule.name)
            } else {
                let holes: Vec<String> = rule
                    .inputs
                    .iter()
                    .map(|(name, of)| {
                        // A place is a number; anything else is a kind, and the first member of
                        // the family is one that will do.
                        if of == "place" {
                            format!("{name}:1")
                        } else {
                            format!("{name}:{}", every_kind_in(of).first().expect("a kind"))
                        }
                    })
                    .collect();
                format!("{{{} {}}}", rule.name, holes.join(" "))
            };
            let Meaning::Fire(row) = meaning(&line) else {
                panic!("`{line}` is not a rule");
            };
            assert_eq!(row.relation, rule.name);
            assert_eq!(row.values.len(), rule.inputs.len(), "{line}");
            said += 1;
        }
        assert_eq!(said, known.len());
    }

    /// **Stating a thing is not firing a rule**, and the design commands are the ones that state.
    #[test]
    fn a_design_command_becomes_rows() {
        let Meaning::Build(rows) = meaning("{create-planet size:tiny-12}") else {
            panic!("not a build");
        };
        assert_eq!(rows.len(), 114, "a tiny planet is 114 rows");

        let Meaning::Build(rows) = meaning("{set-biome territory:2 biome:jungle}") else {
            panic!("not a build");
        };
        assert_eq!(rows.len(), 1, "a biome is one row");
        assert_eq!(rows[0].relation, "terrain");

        assert_eq!(meaning("{start}"), Meaning::Start);
    }

    #[test]
    fn asking_a_question_is_not_a_change() {
        assert_eq!(
            meaning("{show-territory id:5}"),
            Meaning::Show(Subject::Territory(5))
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
    /// **It is a kind now rather than a resource**, because a rule's input is typed as a
    /// relation and `work`'s `what` is one - so the closed set is the family's members, read
    /// from the data rather than from an enum.
    #[test]
    fn a_word_that_names_nothing_is_reported_in_the_games_terms() {
        assert_eq!(
            misread("{work where:3 what:gold}"),
            "there is no kind called gold - expected one of energy, food, metal"
        );
    }

    /// A command that is not a command never reaches the binding at all.
    ///
    /// **The half that moved, checked where it moved to.** Without this the test above reads as
    /// though one of the two cases had been deleted rather than relocated.
    #[test]
    fn a_command_that_names_nothing_is_refused_by_the_grammar() {
        let failure = parse_line(&grammar(), "{build-refinery where:3}", 1)
            .expect_err("`build-refinery` is not a command");
        // **At the name, which is one word.** `P-328` made a command's name a single token, so a
        // name that is not one fails at the name rather than partway through it.
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
    fn a_misspelled_kind_is_reported_rather_than_ignored() {
        assert_eq!(
            misread("{work where:3 what:metel}"),
            "there is no kind called metel - expected one of energy, food, metal"
        );
    }

    /// Every refusal over a closed set names what was expected, which is the rule rather than
    /// the message.
    ///
    /// # Why a message test is not enough, measured
    ///
    /// **`spec/console.md`**: *a rejection names what was wrong, where, and what was expected
    /// instead.* A test asserting the exact string the console produces was green for as long as
    /// that string said only what was wrong. **A test that pins the current message cannot
    /// notice the message is missing something a document requires** - it is the strongest
    /// possible statement about what the code does and says nothing about what it should do.
    ///
    /// # Over every category and the count with it
    ///
    /// **Asked of every closed set rather than of the one an item named**, and the count is
    /// asserted so that a set which stops being reachable is a failure rather than a silence.
    #[test]
    fn every_refusal_over_a_closed_set_says_what_was_expected() {
        let cases = [
            ("{create-planet size:enormous}", "planet size", "tiny-12"),
            ("{set-biome territory:1 biome:swamp}", "biome", "grassland"),
            ("{work where:1 what:gold}", "kind", "metal"),
        ];
        let mut checked = 0;
        for (line, what, one_of) in cases {
            let said = misread(line);
            assert!(said.contains(what), "`{line}` said `{said}`");
            assert!(
                said.contains("expected one of"),
                "`{line}` said `{said}`, and a refusal says what was expected"
            );
            assert!(said.contains(one_of), "`{line}` said `{said}`");
            checked += 1;
        }
        assert_eq!(checked, 3, "every closed set a command can refuse over");
    }

    #[test]
    fn a_size_that_is_not_a_planet_size_is_reported() {
        let said = misread("{create-planet size:enormous}");
        assert!(
            said.starts_with("there is no planet size called enormous"),
            "{said}"
        );
    }
}
