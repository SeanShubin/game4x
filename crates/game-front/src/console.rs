//! The one console: a session, what has been said, and what is being typed.
//!
//! No engine, no platform, no widgets. What a shell adds is a way to put characters in
//! and get text out; everything about *what happens* is here, so the desktop and the web
//! cannot quietly diverge on it.

use game_console::{Outcome, Session};

use crate::library::library;

/// How many lines of transcript are kept.
///
/// A console is a window on what just happened. `history` is the record that does not
/// forget, and unlike this one it is a game command rather than a convenience.
const KEPT: usize = 400;

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
        };
        for line in ["run setup", "start"] {
            console.submit(line);
        }
        console
    }

    /// Runs a line, records what happened whether it worked or not, and gives back what
    /// was said in answer to *this* line.
    ///
    /// The answer is returned rather than left to be sliced back off the transcript. A
    /// terminal wants only the new lines, and the transcript is trimmed from the front
    /// once it is full - so counting lines before and after would quietly go wrong on the
    /// four hundredth command and not before.
    pub fn submit(&mut self, line: &str) -> Vec<String> {
        if line.trim().is_empty() {
            return Vec::new();
        }
        self.say(format!("> {line}"));
        let said = match self.session.run(line, &library()) {
            Ok(Outcome::Said(said)) => said.lines().map(str::to_string).collect(),
            Ok(Outcome::Changed) => {
                self.generation += 1;
                vec!["done".to_string()]
            }
            Ok(Outcome::Nothing) => Vec::new(),
            // A problem is shown exactly as the layer that found it phrased it. A parse
            // failure says where and what was expected; a rejection talks about the game.
            Err(problem) => vec![problem.to_string()],
        };
        for line in &said {
            self.say(line.clone());
        }
        said
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

    /// The counter the engine watches moves when the game moves, and only then.
    ///
    /// Asking a question must not bump it, or the globe would be rebuilt every time
    /// somebody typed `show turn`.
    #[test]
    fn the_generation_counts_changes_and_not_questions() {
        let mut console = Console::new();
        let opened = console.generation();
        assert!(opened > 0, "building the world was a change");

        for question in ["show turn", "show planet", "help", "history", ""] {
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
