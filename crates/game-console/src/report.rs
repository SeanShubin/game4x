//! Answering questions about the game. Nothing here changes anything.

use command_language::Grammar;
use game_model::engine::Game;

use crate::containment::{self, Entry as Held};
use crate::state;

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

pub fn show(game: Option<&Game>, subject: &Subject) -> String {
    let Some(game) = game else {
        return "designing the world; the game has not started".to_string();
    };
    let whole = containment::tree(game);
    match subject {
        // **The engine keeps no turn and this stopped pretending one exists.** `end-turn` is a
        // rule like any other; nothing counts how many times it has fired, because nothing in
        // `spec/data/` holds a count. **A report of a number the game does not have is the shape
        // `founded` had** - `P-288`, and the same reasoning that took `turn` out of the entity
        // view took it out of here.
        Subject::Turn => "playing".to_string(),
        Subject::Territory(id) => match under(&whole, "territory", &id.to_string()) {
            Some(found) => state::written(found),
            None => format!("there is no territory {id}"),
        },
        Subject::Planet => match whole
            .contents
            .iter()
            .find(|it| it.description.kind == "planet")
        {
            Some(planet) => state::written(planet),
            None => "there is no planet yet".to_string(),
        },
        // **An orbit is a place and not a kind**, so what is in orbit is what sits under a place
        // whose layer says so - which is `spec/orbit.md`'s own sentence rather than a list of
        // the kinds that can be up there.
        Subject::Orbit => {
            let found = every(&whole, &|entry: &Held| {
                entry.description.kind == "place"
                    && entry.description.traits.get("layer").map(String::as_str) == Some("orbit")
            });
            if found.iter().all(|it| it.contents.is_empty()) {
                return "there is nothing in orbit".to_string();
            }
            found
                .into_iter()
                .filter(|it| !it.contents.is_empty())
                .map(state::written)
                .collect::<Vec<_>>()
                .join("")
        }
        // **A unit is what the data calls one**, read from `{member family:unit}` rather than
        // from a list here - so a kind added to the family is shown without this being edited.
        Subject::Units => {
            let kinds = family(game, "unit");
            let found = every(&whole, &|entry: &Held| {
                kinds.contains(&entry.description.kind)
            });
            if found.is_empty() {
                return "there are no units".to_string();
            }
            found
                .into_iter()
                .map(state::written)
                .collect::<Vec<_>>()
                .join("")
        }
    }
}

/// Every entry the test holds, anywhere in the tree.
fn every<'a>(root: &'a Held, holds: &dyn Fn(&Held) -> bool) -> Vec<&'a Held> {
    root.walk().into_iter().filter(|it| holds(it)).collect()
}

/// The one entry of this kind carrying this id, wherever it sits.
fn under<'a>(root: &'a Held, kind: &str, id: &str) -> Option<&'a Held> {
    root.walk().into_iter().find(|it| {
        it.description.kind == kind
            && it.description.traits.get("id").map(String::as_str) == Some(id)
    })
}

/// The kinds in a family, as the data declares them.
fn family(game: &Game, of: &str) -> Vec<String> {
    let rows = game.rows().rows();
    let named: std::collections::BTreeMap<&str, &str> = rows
        .iter()
        .filter(|it| it.relation == "relation")
        .filter_map(|it| Some((it.value("id")?, it.value("name")?)))
        .collect();
    let family_id = named
        .iter()
        .find(|(_, name)| **name == of)
        .map(|(id, _)| *id);
    rows.iter()
        .filter(|it| it.relation == "member")
        .filter(|it| Some(it.value("family").unwrap_or_default()) == family_id)
        .filter_map(|it| named.get(it.value("kind").unwrap_or_default()).copied())
        .map(str::to_string)
        .collect()
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

/// Every thing in the game and its traits, for the data browser.
///
/// **Flattened from the tree rather than walked out of the model.** `docs/architecture.md` rule
/// 8: a Bevy entity id is reused and is not stable across runs, so what a browser shows is named
/// by the data's own ids - the same ones `show territory 5` uses.
///
/// **What a thing is, is its relation**, so a kind the data adds appears here with nothing
/// edited. It was a match over nine of them until the port, which is why a world holding a tenth
/// rendered as a world without one.
pub fn entities(game: Option<&Game>) -> Vec<Entry> {
    let Some(game) = game else {
        return Vec::new();
    };
    containment::tree(game)
        .walk()
        .into_iter()
        .map(|held| {
            let description = &held.description;
            let mut components: Vec<(String, String)> = description
                .traits
                .iter()
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect();
            // **How many is a component and not a trait**, which is the same line the tree
            // draws: `spec/console.md` says a description is a kind and every trait, and a
            // quantity is neither.
            if held.quantity != 1 {
                components.push(("quantity".to_string(), held.quantity.to_string()));
            }
            Entry {
                kind: description.kind.clone(),
                // **A thing with an `id` is named by it and one without is named by what it
                // says.** A capacity has no id and never will - it is a fact about kinds - so
                // naming it by its description is the only stable name it has.
                id: description
                    .traits
                    .get("id")
                    .cloned()
                    .unwrap_or_else(|| description.written()),
                components,
            }
        })
        .collect()
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
            "{create-planet size:tiny-12}",
            "{set-resource territory:1 resource:food extractors:1 density:4}",
            "{set-resource territory:1 resource:metal extractors:1 density:4}",
            "{set-biome territory:1 biome:grassland}",
            "{add-ark-orbit territory:1}",
            "{start}",
        ])
    }

    // **`the_turn_says_what_would_win_from_here` was here and went with the hint** - `S-174`.
    // It checked three branches with the count and asserted them distinct, which was the right
    // shape for a sentence that existed; the sentence does not exist now.
    //
    // **What it caught is worth keeping in words**: the hint had told the player the old rule
    // for three days after `P-520` replaced it, because no test named the string and
    // `is_fully_exploited` was false in every fixture that would have shown it.

    #[test]
    fn help_lists_every_command_with_its_syntax() {
        let mut session = Session::new();
        let Outcome::Said(text) = session.run("{help}", &NoLibrary).unwrap() else {
            panic!("help said nothing");
        };
        for expected in [
            "{deploy where:<value> what:<value> [repeat:<value>]}",
            "{end-turn [repeat:<value>]}",
            "{show-territory id:<value>}",
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
        let Outcome::Said(text) = session.run("{help command:move}", &NoLibrary).unwrap() else {
            panic!();
        };
        // **One, and it was two between `P-323` and `P-328`.** A command is named for its
        // recipe and a name is one word, so `move` is the whole name and which unit moves is
        // a field.
        assert!(
            text.contains("{move what:<value> from:<value> to:<value> [repeat:<value>]}"),
            "{text}"
        );
        assert!(
            !text.contains("{end-turn}"),
            "only the one asked for:\n{text}"
        );
    }

    #[test]
    fn help_for_something_that_is_not_a_command_says_so() {
        let mut session = Session::new();
        let Outcome::Said(text) = session.run("{help command:fly}", &NoLibrary).unwrap() else {
            panic!();
        };
        assert!(text.contains("no command called fly"), "{text}");
    }

    #[test]
    fn showing_a_territory_reports_what_is_there() {
        let mut session = tiny();
        let Outcome::Said(text) = session.run("{show-territory id:1}", &NoLibrary).unwrap() else {
            panic!();
        };
        // **What a territory holds, in the form the state file uses**, which is what `show`
        // reports now: the same bytes a reader would diff, scoped to one thing.
        assert!(text.contains("{territory id:1}"), "{text}");
        assert!(text.contains("place"), "{text}");
        assert!(text.contains("deposit"), "{text}");
    }

    #[test]
    fn showing_a_territory_that_is_not_there_says_so_in_the_games_terms() {
        let mut session = tiny();
        let Outcome::Said(text) = session.run("{show-territory id:99}", &NoLibrary).unwrap() else {
            panic!();
        };
        assert_eq!(text, "there is no territory 99");
    }

    #[test]
    fn showing_orbit_reports_what_is_up_there() {
        let mut session = tiny();
        let Outcome::Said(text) = session.run("{show-orbit}", &NoLibrary).unwrap() else {
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
        // **A territory's only trait is its id**, which is what the data gives it - a biome
        // is a `{terrain}` row about the ground rather than a column on the territory.
        assert!(fifth.components.iter().any(|(name, _)| name == "id"));

        // And that is the same id `show-territory id:5` answers to.
        let mut session = session;
        let Outcome::Said(text) = session.run("{show-territory id:5}", &NoLibrary).unwrap() else {
            panic!();
        };
        assert!(text.starts_with("{territory id:5}"), "{text}");
    }

    #[test]
    fn the_browser_lists_every_territory_and_every_unit() {
        let session = tiny();
        let entries = session.entities();
        // **A kind is a relation now**, so an ark is an `ark` and not a `unit` - which is the
        // browser naming things the way the data does rather than the way an enum did.
        assert_eq!(entries.iter().filter(|e| e.kind == "territory").count(), 12);
        assert_eq!(entries.iter().filter(|e| e.kind == "ark").count(), 1);
        assert_eq!(entries.iter().filter(|e| e.kind == "game").count(), 1);
    }

    #[test]
    fn history_is_numbered_and_in_order() {
        let mut session = tiny();
        let Outcome::Said(text) = session.run("{history}", &NoLibrary).unwrap() else {
            panic!();
        };
        let lines: Vec<&str> = text.lines().collect();
        assert!(lines[0].contains("{create-planet size:tiny-12}"), "{text}");
        assert!(lines.last().unwrap().contains("{start}"), "{text}");
    }

    #[test]
    fn a_library_reports_what_it_holds() {
        let library = crate::Embedded::of(&[("one", "start\n"), ("two", "start\n")]);
        assert_eq!(library.names(), ["one", "two"]);
        assert!(library.fetch("one").is_some());
        assert!(library.fetch("three").is_none());
    }
}
