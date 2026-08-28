//! The one console: a session, what has been said, and what is being typed.
//!
//! No engine, no platform, no widgets. What a shell adds is a way to put characters in
//! and get text out; everything about *what happens* is here, so the desktop and the web
//! cannot quietly diverge on it.

use game_console::{Outcome, Session};

use crate::Surface;
use crate::library::library;

/// How many lines of transcript are kept.
///
/// A console is a window on what just happened. `history` is the record that does not
/// forget, and unlike this one it is a game command rather than a convenience.
const KEPT: usize = 400;

/// What a line typed at the console turned out to be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Said {
    /// The console spoke. Every line is in the transcript too.
    Spoke(Vec<String>),
    /// The line named a surface to go to. Nothing was said and nothing changed.
    ///
    /// What *going there* looks like is the shell's business: a terminal prints the
    /// surface, and a page shows a panel. Both are correct, and neither belongs here.
    Reach(Surface),
}

/// `spec/console.md`: *a line beginning with `/` is not a command. It names a surface to
/// go to.*
///
/// `None` means the line is a command and belongs to the parser.
///
/// The rule is here rather than in a shell because it is a fact about a line typed at the
/// console, and the console is one thing however a platform presents it. It lived in the
/// terminal for one commit, which quietly made `/browser` a parse error on the web -
/// exactly the divergence this crate exists to prevent.
fn names_a_surface(line: &str) -> Option<Result<Surface, String>> {
    let word = line.trim().strip_prefix('/')?;
    if let Some(surface) = Surface::named(word) {
        return Some(Ok(surface));
    }
    // A bare `/` is not a mistake to be scolded for. `help` deliberately does not list
    // the surfaces, so this is where somebody who read that a slash names one finds out
    // which ones there are.
    Some(Err(if word.is_empty() {
        format!("a `/` names a surface; there is {}", Surface::names())
    } else {
        format!(
            "there is no surface called {word}; there is {}",
            Surface::names()
        )
    }))
}

/// The nudge for somebody who typed a surface's name and left the slash off.
///
/// `spec/console.md` asks that a rejection name what was wrong, where, and what was
/// expected instead. The parser can only ever expect commands - it has never heard of a
/// surface - so `browser` gets a list of fifteen verbs that does not contain the one
/// thing it was nearly. The word most likely to have been meant is added here, by the
/// layer that knows there is such a thing as a surface.
fn meant_a_surface(line: &str) -> Option<String> {
    let surface = line.split_whitespace().next().and_then(Surface::named)?;
    Some(format!(
        "`{name}` is a surface rather than a command; type `/{name}` to go there",
        name = surface.name()
    ))
}

pub struct Console {
    pub session: Session,
    /// What has been said, oldest first.
    transcript: Vec<String>,
    /// How many times the game state has moved.
    ///
    /// The engine is on the other side of a wall from all of this - on the web it is not
    /// even on the same call stack, because the page calls in - so it cannot be handed
    /// the new state when it changes. It watches this instead, and rebuilds when the
    /// number it last saw is not the number it sees now.
    generation: u64,
    /// The surface the most recent line named, if it named one.
    ///
    /// Not "the surface you are on": the console has no idea which one is in front, and
    /// on a terminal there is no such thing to know. It is only what the last line asked
    /// for, which is what a shell reached through a narrow doorway needs to read back.
    reached: Option<Surface>,
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    /// Opens on a world rather than on nothing, so every surface has something in it.
    ///
    /// The world is built by running the release's own setup file through the console,
    /// because `spec/invariants.md` leaves no other way to build one: every change to
    /// game state is a console command, and that includes designing it.
    pub fn new() -> Self {
        let mut console = Self {
            session: Session::new(),
            transcript: vec![
                "game4x.".to_string(),
                "`help` lists every command. `run setup` builds the world of the first release."
                    .to_string(),
            ],
            generation: 0,
            reached: None,
        };
        for line in ["run setup", "start"] {
            console.submit(line);
        }
        console
    }

    /// Runs a line, records what happened whether it worked or not, and says what the
    /// line turned out to be.
    ///
    /// What was said is returned rather than left to be sliced back off the transcript. A
    /// terminal wants only the new lines, and the transcript is trimmed from the front
    /// once it is full - so counting lines before and after would quietly go wrong on the
    /// four hundredth command and not before.
    pub fn submit(&mut self, line: &str) -> Said {
        self.reached = None;
        if line.trim().is_empty() {
            return Said::Spoke(Vec::new());
        }
        self.say(format!("> {line}"));

        // The slash is settled before the parser is reached, because a line beginning
        // with one is not a command and handing it to a command parser would produce an
        // error about the wrong thing entirely.
        if let Some(named) = names_a_surface(line) {
            return match named {
                Ok(surface) => {
                    self.reached = Some(surface);
                    Said::Reach(surface)
                }
                Err(problem) => {
                    self.say(problem.clone());
                    Said::Spoke(vec![problem])
                }
            };
        }

        let said = match self.session.run(line, &library()) {
            Ok(Outcome::Said(said)) => said.lines().map(str::to_string).collect(),
            Ok(Outcome::Changed) => {
                self.generation += 1;
                vec!["done".to_string()]
            }
            Ok(Outcome::Nothing) => Vec::new(),
            // A problem is shown exactly as the layer that found it phrased it. A parse
            // failure says where and what was expected; a rejection talks about the game.
            Err(problem) => {
                let mut said = vec![problem.to_string()];
                said.extend(meant_a_surface(line));
                said
            }
        };
        for line in &said {
            self.say(line.clone());
        }
        Said::Spoke(said)
    }

    fn say(&mut self, line: String) {
        self.transcript.push(line);
        let overflow = self.transcript.len().saturating_sub(KEPT);
        self.transcript.drain(..overflow);
    }

    /// Everything still being kept, oldest first.
    pub fn transcript(&self) -> String {
        self.transcript.join("\n")
    }

    /// The last few lines, for a shell with a fixed number of them to fill.
    pub fn tail(&self, lines: usize) -> String {
        let from = self.transcript.len().saturating_sub(lines);
        self.transcript[from..].join("\n")
    }

    /// How many times the game state has moved. Never goes down.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// The surface the most recent line named, if it named one.
    pub fn reached(&self) -> Option<Surface> {
        self.reached
    }

    /// How many territories the planet has, or none if there is no planet yet.
    ///
    /// This is the one number the engine needs in order to draw the right world, and
    /// asking for it is a question: it changes nothing, and it goes through no command.
    pub fn territory_count(&self) -> Option<usize> {
        match self.session.game.territories.len() {
            0 => None,
            count => Some(count),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spoke(console: &mut Console, line: &str) -> String {
        match console.submit(line) {
            Said::Spoke(said) => said.join("\n"),
            Said::Reach(surface) => panic!("`{line}` reached {}", surface.name()),
        }
    }

    #[test]
    fn the_console_opens_on_a_world_that_was_built_by_commands() {
        let console = Console::new();
        assert_eq!(console.session.game.territories.len(), 12);
        assert_eq!(console.session.game.phase, game_model::Phase::Play);
        // And the history says so: every command that built it, in order.
        assert!(
            console
                .session
                .history()
                .contains(&"create planet tiny".to_string()),
            "{:?}",
            console.session.history()
        );
    }

    #[test]
    fn typing_a_command_records_what_it_said() {
        let mut console = Console::new();
        console.submit("show territory 1");
        let tail = console.tail(12);
        assert!(tail.contains("> show territory 1"), "{tail}");
        assert!(tail.contains("territory 1"), "{tail}");
    }

    /// A refused command is shown, not swallowed, and in the terms of whichever layer
    /// refused it.
    #[test]
    fn a_refused_command_is_shown_to_the_player() {
        let mut console = Console::new();
        console.submit("land ark somewhere");
        assert!(
            console.tail(3).contains("expected a number"),
            "{}",
            console.tail(3)
        );

        console.submit("land ark 99");
        assert!(
            console.tail(3).contains("no territory 99"),
            "{}",
            console.tail(3)
        );
    }

    /// `spec/console.md`: a line beginning with `/` is not a command. It names a surface.
    ///
    /// On the web this is asserted for the first time here. The rule lived in the
    /// terminal shell until now, which made `/browser` work on the desktop and fail as an
    /// unknown command on the page - the two builds disagreeing about what the user can
    /// do, which is the one thing `spec/interface.md` does not allow.
    #[test]
    fn a_slash_line_names_a_surface_on_every_platform() {
        let mut console = Console::new();
        for surface in Surface::ALL {
            assert_eq!(
                console.submit(&format!("/{}", surface.name())),
                Said::Reach(surface)
            );
            assert_eq!(console.reached(), Some(surface));
        }
    }

    /// Reaching a surface is not a change to the game, so nothing about the game moves.
    #[test]
    fn reaching_a_surface_changes_no_game_state() {
        let mut console = Console::new();
        let before = console.session.game.clone();
        let generation = console.generation();
        let history = console.session.history().len();

        console.submit("/browser");

        assert_eq!(console.session.game, before);
        assert_eq!(console.generation(), generation);
        assert_eq!(
            console.session.history().len(),
            history,
            "history recorded a line that is not a command"
        );
    }

    /// It is still echoed, though. A console shows what was typed at it whether or not the
    /// game heard about it.
    #[test]
    fn reaching_a_surface_is_still_echoed() {
        let mut console = Console::new();
        console.submit("/browser");
        assert!(
            console.tail(1).contains("> /browser"),
            "{}",
            console.tail(1)
        );
    }

    /// A slash that names nothing says which surfaces there are - and so does a bare
    /// slash, which is the whole discovery path for somebody who missed the greeting.
    #[test]
    fn a_slash_that_names_no_surface_says_which_ones_there_are() {
        let mut console = Console::new();
        for line in ["/nowhere", "/"] {
            let said = spoke(&mut console, line);
            for surface in Surface::ALL {
                assert!(
                    said.contains(surface.name()),
                    "`{line}` did not mention {}: {said}",
                    surface.name()
                );
            }
            assert!(
                !said.contains("expected create"),
                "`{line}` was answered as though it were a command: {said}"
            );
        }
    }

    /// The near miss. `spec/console.md` asks a rejection to say what was expected
    /// instead, and the parser can only expect commands - it has never heard of a surface.
    #[test]
    fn a_surface_name_without_its_slash_is_told_about_the_slash() {
        let mut console = Console::new();
        for surface in Surface::ALL {
            let said = spoke(&mut console, surface.name());
            assert!(
                said.contains(&format!("/{}", surface.name())),
                "typing `{}` never mentioned `/{}`: {said}",
                surface.name(),
                surface.name()
            );
        }
        // And it is still a rejection: the bare word is not a command and does not become
        // one by being close to something.
        let said = spoke(&mut console, "browser");
        assert!(said.contains("expected create"), "{said}");
    }

    /// The suggestion is for near misses only. A command that fails for its own reasons
    /// must not collect advice about surfaces.
    #[test]
    fn an_ordinary_failure_is_not_given_advice_about_surfaces() {
        let mut console = Console::new();
        let said = spoke(&mut console, "land ark 99");
        assert!(!said.contains('/'), "{said}");
    }

    /// The counter the engine watches moves when the game moves, and only then.
    ///
    /// Asking a question must not bump it, or the globe would be rebuilt every time
    /// somebody typed `show turn`.
    #[test]
    fn the_generation_counts_changes_and_not_questions() {
        let mut console = Console::new();
        let opened = console.generation();
        assert!(opened > 0, "building the world was a change");

        for question in [
            "show turn",
            "show planet",
            "help",
            "history",
            "",
            "/browser",
        ] {
            console.submit(question);
        }
        assert_eq!(console.generation(), opened, "a question moved nothing");

        console.submit("end turn");
        assert_eq!(console.generation(), opened + 1);
    }

    /// A refused command changes nothing, so it must not look like a change either.
    #[test]
    fn a_refused_command_does_not_move_the_generation() {
        let mut console = Console::new();
        let before = console.generation();
        console.submit("land ark 99");
        assert_eq!(console.generation(), before);
    }

    /// What the last line named is about the last line, not a mode to be left in.
    #[test]
    fn what_a_line_reached_does_not_outlive_the_next_line() {
        let mut console = Console::new();
        console.submit("/browser");
        assert_eq!(console.reached(), Some(Surface::Browser));
        console.submit("show turn");
        assert_eq!(console.reached(), None);
    }

    #[test]
    fn the_territory_count_is_the_planet_the_commands_built() {
        assert_eq!(Console::new().territory_count(), Some(12));
    }

    /// The transcript is a window, and a window has a size. Without this the browser tab
    /// grows without bound over a long game.
    #[test]
    fn the_transcript_stops_growing() {
        let mut console = Console::new();
        for _ in 0..KEPT * 2 {
            console.submit("show turn");
        }
        assert!(
            console.transcript().lines().count() <= KEPT,
            "{} lines kept",
            console.transcript().lines().count()
        );
    }
}
