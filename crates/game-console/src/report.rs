//! Answering questions about the game. Nothing here changes anything.

use command_language::Grammar;
use game_model::{Game, Phase, Resource, TerritoryId, unit::Location};

use crate::binding::Subject;

/// One thing in the game, with its parts, for the data browser.
///
/// Named by the model's own id rather than by anything the engine assigns.
/// `docs/architecture.md` rule 8: a Bevy entity id is reused and is not stable across
/// runs, so the browser and `show territory 5` would end up naming the same thing two
/// different ways, and neither would survive a save.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// What sort of thing this is.
    pub kind: String,
    /// Its identity in the model, as a player would say it.
    pub id: String,
    /// Its parts, in a fixed order so a browser never reorders under the reader.
    pub components: Vec<(String, String)>,
}

pub fn show(game: &Game, subject: &Subject) -> String {
    match subject {
        Subject::Turn => match game.phase {
            Phase::Design => "designing the world; the game has not started".to_string(),
            Phase::Play => format!("turn {}", game.turn),
        },
        Subject::Territory(id) => match game.territory(*id) {
            Ok(_) => territory(game, *id),
            Err(why) => why.to_string(),
        },
        Subject::Planet => {
            if game.territories.is_empty() {
                return "there is no planet yet".to_string();
            }
            let mut lines = vec![format!("{} territories", game.territories.len())];
            for place in &game.territories {
                lines.push(format!(
                    "  {:>2}  {:<9} citizens {:<3} force {:<3} nodes {}",
                    place.id,
                    if place.founded { "yours" } else { "unclaimed" },
                    place.citizens,
                    game.force_in(place.id),
                    place.nodes.len()
                ));
            }
            lines.join("\n")
        }
        Subject::Orbit => {
            let above = game.units_in_orbit();
            if above.is_empty() {
                return "there is nothing in orbit".to_string();
            }
            above
                .into_iter()
                .map(|unit| format!("{} {} in orbit", unit.kind, unit.id))
                .collect::<Vec<_>>()
                .join("\n")
        }
        Subject::Units => {
            if game.units.is_empty() {
                return "there are no units".to_string();
            }
            game.units
                .iter()
                .map(|unit| {
                    let place = match unit.location {
                        Location::Orbit => "in orbit".to_string(),
                        Location::On(id) => format!("on territory {id}"),
                    };
                    format!(
                        "{} {} {place}, {} cells{}",
                        unit.kind,
                        unit.id,
                        unit.cells,
                        if unit.usable { "" } else { ", unusable" }
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

fn territory(game: &Game, id: TerritoryId) -> String {
    let Ok(place) = game.territory(id) else {
        return format!("there is no territory {id}");
    };
    let mut lines = vec![format!(
        "territory {id}, {}",
        if place.founded { "yours" } else { "unclaimed" }
    )];
    lines.push(format!(
        "  citizens {}  labor left {}  force {} against nature {}",
        place.citizens,
        place.labor_available(),
        game.force_in(id),
        place.force_of_nature
    ));
    for resource in Resource::ALL {
        let nodes: Vec<String> = place
            .nodes_of(resource)
            .into_iter()
            .map(|(_, node)| node.density.to_string())
            .collect();
        lines.push(format!(
            "  {:<7} {:>3} held, {} extractors of {} nodes{}",
            resource.name(),
            place.store(resource),
            place.extractors_for(resource).len(),
            nodes.len(),
            if nodes.is_empty() {
                String::new()
            } else {
                format!(" at density {}", nodes.join(", "))
            }
        ));
    }
    match place.garrison {
        Some(garrison) => lines.push(format!(
            "  garrison force {} multiplier {}, manned by {}",
            garrison.force, garrison.multiplier, garrison.manned
        )),
        None => lines.push("  no garrison".to_string()),
    }
    if place.yards > 0 {
        lines.push(format!("  yards {}", place.yards));
    }
    for unit in game.units_on(id) {
        lines.push(format!(
            "  {} {} with {} cells",
            unit.kind, unit.id, unit.cells
        ));
    }
    lines.join("\n")
}

/// `spec/console.md`: list every command, or give one command's syntax.
/// What `help` says instead of listing the surfaces.
///
/// Kept as a constant so the test that `help` does not list them can name the one line
/// that is allowed to mention a slash at all.
pub const SURFACES_ARE_ELSEWHERE: &str = "a line beginning with `/` directs the front end rather than the game; type `/` on its own to see what it can direct";

pub fn help(grammar: &Grammar, command: Option<String>) -> String {
    match command {
        None => {
            let mut lines = vec!["commands:".to_string()];
            for form in grammar.forms() {
                lines.push(format!("  {:<44} {}", form.syntax(), form.summary));
            }
            // `spec/console.md`: a line beginning with `/` directs the front end, and
            // *help does not list them*. Saying the mechanism exists is not listing them,
            // and it
            // has to be said somewhere - on a build whose console is a terminal, typing
            // is the only way two of the three surfaces can be reached at all, and the
            // greeting that used to be the only announcement scrolls away. The names
            // themselves are left to a bare `/`, which is what keeps this from being a
            // list of them.
            lines.push(String::new());
            lines.push(SURFACES_ARE_ELSEWHERE.to_string());
            lines.join("\n")
        }
        Some(word) => {
            let matching = grammar.forms_beginning(&word);
            if matching.is_empty() {
                return format!("there is no command called {word}; `help` lists every command");
            }
            matching
                .into_iter()
                .map(|form| format!("{:<44} {}", form.syntax(), form.summary))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

pub fn history(commands: &[String]) -> String {
    if commands.is_empty() {
        return "nothing has been done yet".to_string();
    }
    commands
        .iter()
        .enumerate()
        .map(|(at, line)| format!("{:>4}  {line}", at + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every entity in the game and its components.
pub fn entities(game: &Game) -> Vec<Entry> {
    let mut entries = Vec::new();

    entries.push(Entry {
        kind: "game".to_string(),
        id: "the game".to_string(),
        components: vec![
            (
                "phase".to_string(),
                match game.phase {
                    Phase::Design => "design".to_string(),
                    Phase::Play => "play".to_string(),
                },
            ),
            ("turn".to_string(), game.turn.to_string()),
            (
                "territories".to_string(),
                game.territories.len().to_string(),
            ),
            ("units".to_string(), game.units.len().to_string()),
        ],
    });

    for place in &game.territories {
        let mut components = vec![
            ("founded".to_string(), place.founded.to_string()),
            ("citizens".to_string(), place.citizens.to_string()),
            ("labor spent".to_string(), place.labor_spent.to_string()),
            (
                "force of nature".to_string(),
                place.force_of_nature.to_string(),
            ),
            ("force".to_string(), game.force_in(place.id).to_string()),
        ];
        for resource in Resource::ALL {
            components.push((
                resource.name().to_string(),
                place.store(resource).to_string(),
            ));
            let nodes: Vec<String> = place
                .nodes_of(resource)
                .into_iter()
                .map(|(_, node)| node.density.to_string())
                .collect();
            components.push((
                format!("{} nodes", resource.name()),
                if nodes.is_empty() {
                    "none".to_string()
                } else {
                    nodes.join(", ")
                },
            ));
            components.push((
                format!("{} extractors", resource.name()),
                place.extractors_for(resource).len().to_string(),
            ));
        }
        components.push((
            "garrison".to_string(),
            match place.garrison {
                Some(garrison) => format!(
                    "force {} multiplier {} manned {}",
                    garrison.force, garrison.multiplier, garrison.manned
                ),
                None => "none".to_string(),
            },
        ));
        components.push(("yards".to_string(), place.yards.to_string()));

        entries.push(Entry {
            kind: "territory".to_string(),
            // The model's id, which is what `show territory 5` names too.
            id: place.id.to_string(),
            components,
        });
    }

    for unit in &game.units {
        entries.push(Entry {
            kind: "unit".to_string(),
            id: unit.id.to_string(),
            components: vec![
                ("kind".to_string(), unit.kind.to_string()),
                (
                    "location".to_string(),
                    match unit.location {
                        Location::Orbit => "orbit".to_string(),
                        Location::On(id) => format!("territory {id}"),
                    },
                ),
                ("cells".to_string(), unit.cells.to_string()),
                ("force".to_string(), unit.force().to_string()),
                ("exhausted".to_string(), unit.exhausted.to_string()),
                ("usable".to_string(), unit.usable.to_string()),
            ],
        });
    }

    entries
}

#[cfg(test)]
mod tests {
    use crate::{Library, NoLibrary, Outcome, Session};

    fn played(lines: &[&str]) -> Session {
        let mut session = Session::new();
        for line in lines {
            session
                .run(line, &NoLibrary)
                .unwrap_or_else(|why| panic!("`{line}`: {why}"));
        }
        session
    }

    fn tiny() -> Session {
        played(&[
            "create planet tiny",
            "add node 1 food 4",
            "add node 1 metal 4",
            "set force 1 1",
            "add ark orbit",
            "start",
        ])
    }

    #[test]
    fn help_lists_every_command_with_its_syntax() {
        let mut session = Session::new();
        let Outcome::Said(text) = session.run("help", &NoLibrary).unwrap() else {
            panic!("help said nothing");
        };
        for expected in [
            "land <unit> <territory>",
            "end turn",
            "show territory <territory>",
        ] {
            assert!(
                text.contains(expected),
                "help is missing `{expected}`:\n{text}"
            );
        }
    }

    #[test]
    fn help_for_one_command_gives_that_commands_syntax() {
        let mut session = Session::new();
        let Outcome::Said(text) = session.run("help move", &NoLibrary).unwrap() else {
            panic!();
        };
        assert!(text.contains("move <unit> <territory>"), "{text}");
        assert!(
            !text.contains("end turn"),
            "only the one asked for:\n{text}"
        );
    }

    #[test]
    fn help_for_something_that_is_not_a_command_says_so() {
        let mut session = Session::new();
        let Outcome::Said(text) = session.run("help fly", &NoLibrary).unwrap() else {
            panic!();
        };
        assert!(text.contains("no command called fly"), "{text}");
    }

    #[test]
    fn showing_a_territory_reports_what_is_there() {
        let mut session = tiny();
        let Outcome::Said(text) = session.run("show territory 1", &NoLibrary).unwrap() else {
            panic!();
        };
        assert!(text.contains("territory 1"), "{text}");
        assert!(text.contains("food"), "{text}");
        assert!(text.contains("no garrison"), "{text}");
    }

    #[test]
    fn showing_a_territory_that_is_not_there_says_so_in_the_games_terms() {
        let mut session = tiny();
        let Outcome::Said(text) = session.run("show territory 99", &NoLibrary).unwrap() else {
            panic!();
        };
        assert_eq!(text, "there is no territory 99");
    }

    #[test]
    fn showing_orbit_reports_what_is_up_there() {
        let mut session = tiny();
        let Outcome::Said(text) = session.run("show orbit", &NoLibrary).unwrap() else {
            panic!();
        };
        assert!(text.contains("ark"), "{text}");
    }

    /// The browser and the console have to name the same thing the same way, or a player
    /// reading one cannot type the other.
    #[test]
    fn the_browser_names_a_territory_the_way_the_console_does() {
        let session = tiny();
        let entries = session.entities();
        let fifth = entries
            .iter()
            .find(|entry| entry.kind == "territory" && entry.id == "5")
            .expect("territory 5 should be listed by its model id");
        assert!(fifth.components.iter().any(|(name, _)| name == "citizens"));

        // And that is the same id `show territory 5` answers to.
        let mut session = session;
        let Outcome::Said(text) = session.run("show territory 5", &NoLibrary).unwrap() else {
            panic!();
        };
        assert!(text.starts_with("territory 5"), "{text}");
    }

    #[test]
    fn the_browser_lists_every_territory_and_every_unit() {
        let session = tiny();
        let entries = session.entities();
        assert_eq!(entries.iter().filter(|e| e.kind == "territory").count(), 12);
        assert_eq!(entries.iter().filter(|e| e.kind == "unit").count(), 1);
        assert_eq!(entries.iter().filter(|e| e.kind == "game").count(), 1);
    }

    #[test]
    fn history_is_numbered_and_in_order() {
        let mut session = tiny();
        let Outcome::Said(text) = session.run("history", &NoLibrary).unwrap() else {
            panic!();
        };
        let lines: Vec<&str> = text.lines().collect();
        assert!(lines[0].contains("create planet tiny"), "{text}");
        assert!(lines.last().unwrap().contains("start"), "{text}");
    }

    #[test]
    fn a_library_reports_what_it_holds() {
        let library = crate::Embedded::of(&[("one", "start\n"), ("two", "start\n")]);
        assert_eq!(library.names(), ["one", "two"]);
        assert!(library.fetch("one").is_some());
        assert!(library.fetch("three").is_none());
    }
}
