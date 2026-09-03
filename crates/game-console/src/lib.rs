//! The console: words in, transitions out.
//!
//! ```text
//!   text
//!     |  command-language     grammar and parser, no game nouns
//!   Utterance
//!     |  binding              the one place a word meets a rule
//!   Transition
//!     |  game-model           the one function
//!   Game
//! ```
//!
//! `spec/invariants.md` says every change to game state is representable and executable as
//! a console command. That makes this the only way in, and [`Session::run`] the only door.
//! A question asked of the game comes back through the same door and changes nothing,
//! because the type it produces has no way to say otherwise.

pub mod binding;
pub mod dump;
pub mod grammar;
pub mod report;

use command_language::{Failure, Grammar, parse_line};
use game_model::{Game, Rejection, Transition};

pub use binding::{Meaning, Misreading, Subject, interpret};
pub use grammar::grammar as command_grammar;
pub use report::Entry;

/// Where a command file comes from.
///
/// A trait because the two places that need one are very different: a test reads them off
/// disk, and a browser has no disk at all and carries them in the binary. Neither fact
/// belongs in the console.
pub trait Library {
    fn fetch(&self, name: &str) -> Option<String>;
    /// Every file available, for reporting what could have been run.
    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

/// No files at all, for a console typed at directly.
pub struct NoLibrary;

impl Library for NoLibrary {
    fn fetch(&self, _name: &str) -> Option<String> {
        None
    }
}

/// Files carried in the binary, which is what a browser has to use.
pub struct Embedded(pub Vec<(String, String)>);

impl Embedded {
    pub fn of(files: &[(&str, &str)]) -> Self {
        Self(
            files
                .iter()
                .map(|(name, text)| ((*name).to_string(), (*text).to_string()))
                .collect(),
        )
    }
}

impl Library for Embedded {
    fn fetch(&self, name: &str) -> Option<String> {
        self.0
            .iter()
            .find(|(known, _)| known == name)
            .map(|(_, text)| text.clone())
    }

    fn names(&self) -> Vec<String> {
        self.0.iter().map(|(name, _)| name.clone()).collect()
    }
}

/// What running a line did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The game state moved.
    Changed,
    /// A question was answered. Nothing moved.
    Said(String),
    /// The line held no command.
    Nothing,
}

/// Everything that can go wrong, in the order the layers are crossed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    /// The words could not be read. Says where and what was expected.
    Parse(Failure),
    /// The words were read but name nothing in the game.
    Misread(Misreading),
    /// The command was understood and the rules refused it.
    Rule(Rejection),
    NoSuchFile {
        name: String,
        known: Vec<String>,
    },
    /// Files calling each other without end.
    TooDeep,
}

impl std::fmt::Display for Problem {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Problem::Parse(failure) => write!(out, "{failure}"),
            Problem::Misread(misreading) => write!(out, "{misreading}"),
            Problem::Rule(rejection) => write!(out, "{rejection}"),
            Problem::NoSuchFile { name, known } if known.is_empty() => {
                write!(out, "there is no command file called {name}")
            }
            Problem::NoSuchFile { name, known } => write!(
                out,
                "there is no command file called {name}; there is {}",
                known.join(", ")
            ),
            Problem::TooDeep => write!(out, "command files are calling each other without end"),
        }
    }
}

impl std::error::Error for Problem {}

/// How deep one file may call another.
///
/// A limit rather than cycle detection: a file legitimately running the same subroutine
/// twice is not a loop, and telling the two apart needs the call stack rather than the
/// set of names. A depth this small is far past anything a person would write.
const DEEPEST: usize = 16;

pub struct Session {
    pub game: Game,
    grammar: Grammar,
    history: Vec<String>,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            grammar: grammar::grammar(),
            history: Vec::new(),
        }
    }

    pub fn grammar(&self) -> &Grammar {
        &self.grammar
    }

    /// Every command run so far, in order.
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// Runs one line.
    pub fn run(&mut self, line: &str, library: &dyn Library) -> Result<Outcome, Problem> {
        self.run_at(line, 1, library, 0)
    }

    /// Runs every line of a script, stopping at the first problem.
    ///
    /// The line number is reported, so a failure in a file that was called from another
    /// file still says where it actually is.
    pub fn run_script(
        &mut self,
        text: &str,
        library: &dyn Library,
    ) -> Result<Vec<Outcome>, Problem> {
        self.run_script_at(text, library, 0)
    }

    fn run_script_at(
        &mut self,
        text: &str,
        library: &dyn Library,
        depth: usize,
    ) -> Result<Vec<Outcome>, Problem> {
        let mut outcomes = Vec::new();
        for (offset, line) in text.lines().enumerate() {
            outcomes.push(self.run_at(line, offset + 1, library, depth)?);
        }
        Ok(outcomes)
    }

    fn run_at(
        &mut self,
        line: &str,
        line_number: usize,
        library: &dyn Library,
        depth: usize,
    ) -> Result<Outcome, Problem> {
        let Some(utterance) =
            parse_line(&self.grammar, line, line_number).map_err(Problem::Parse)?
        else {
            return Ok(Outcome::Nothing);
        };
        let meaning = interpret(&utterance).map_err(Problem::Misread)?;

        match meaning {
            Meaning::Change(transition) => {
                self.apply(&transition)?;
                self.history.push(utterance.source.clone());
                Ok(Outcome::Changed)
            }
            Meaning::Show(subject) => Ok(Outcome::Said(report::show(&self.game, &subject))),
            Meaning::Help(command) => Ok(Outcome::Said(report::help(&self.grammar, command))),
            Meaning::History => Ok(Outcome::Said(report::history(&self.history))),
            Meaning::Run(name) => {
                if depth >= DEEPEST {
                    return Err(Problem::TooDeep);
                }
                let text = library.fetch(&name).ok_or(Problem::NoSuchFile {
                    name: name.clone(),
                    known: library.names(),
                })?;
                // Calling a file is not itself a change, and the commands inside it
                // record themselves. Recording both would make the history do everything
                // twice when it is replayed - which is what a history is *for*, since it
                // is the only account of how a game got where it is.
                self.run_script_at(&text, library, depth + 1)?;
                Ok(Outcome::Changed)
            }
        }
    }

    /// The one function, reached from here and from nowhere else.
    fn apply(&mut self, transition: &Transition) -> Result<(), Problem> {
        self.game = self.game.after(transition).map_err(Problem::Rule)?;
        Ok(())
    }

    /// Every entity in the game and its components, named by model id.
    ///
    /// `docs/architecture.md` rule 8: a Bevy entity id is reused and is not stable across
    /// runs, so it can never be what a player is shown. These are the model's own ids -
    /// the same ones `show territory 5` uses - which is what lets the browser and the
    /// console name the same thing the same way.
    pub fn entities(&self) -> Vec<Entry> {
        report::entities(&self.game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use command_language::disagreements;

    /// The check the predecessor did not have.
    ///
    /// Two declarative tables define this language: the grammar and the binding. Nothing
    /// makes them agree except this. Without it, a form nobody wrote a handler for is an
    /// error the first time a player types that command - in a program that compiled and
    /// whose other tests passed.
    #[test]
    fn the_grammar_and_the_binding_describe_the_same_language() {
        let problems = disagreements(&grammar::grammar(), &binding::handled());
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    /// Every command `spec/console.md` lists has a form, so none is unreachable.
    #[test]
    fn every_command_the_specification_lists_can_be_typed() {
        let grammar = grammar::grammar();
        let verbs = [
            "land", "launch", "move", "build", "produce", "work", "end", "show", "help", "history",
            "create", "add", "set", "start",
        ];
        for verb in verbs {
            assert!(
                !grammar.forms_beginning(verb).is_empty(),
                "no command begins with `{verb}`"
            );
        }
    }

    /// `spec/console.md`: *a line beginning with `/` directs the front end rather than the
    /// game ... none of these is a command.*
    ///
    /// That rule is only true if no command can begin with a slash, and the whole
    /// separation between the game's language and the front end rests on it. Nothing
    /// enforces it except this: `Term::Keyword` takes any string, so a future verb could
    /// be spelled `/anything` and the front end would swallow it before the parser ever
    /// saw it - in a program that compiled and whose other tests passed.
    ///
    /// Two things are asserted, because the rule needs both. Every form opens with a
    /// fixed word, so what a line means is decided by its first token; and no such word
    /// begins with a slash, so a line that does begins no command. Together they make
    /// "a slash line is not a command" true by construction rather than by inspection.
    #[test]
    fn no_command_can_begin_with_a_slash() {
        for form in grammar::grammar().forms() {
            match form.terms.first() {
                Some(command_language::Term::Keyword(word)) => assert!(
                    !word.starts_with('/'),
                    "the form `{}` opens with `{word}`, which the front end would take \
                     for a surface before the parser saw it",
                    form.name
                ),
                other => panic!(
                    "the form `{}` opens with {other:?} rather than a fixed word, so a \
                     line beginning with `/` could match it",
                    form.name
                ),
            }
        }
    }

    /// The other half of the same rule, from the language's side: a slash line matches no
    /// form, so handing one to the parser is always a mistake rather than sometimes one.
    #[test]
    fn a_slash_line_matches_no_command() {
        let grammar = grammar::grammar();
        for line in [
            "/game",
            "/console",
            "/browser",
            "/",
            "/end turn",
            "/show turn",
        ] {
            let failure = parse_line(&grammar, line, 1)
                .expect_err("a slash line must not parse as a command");
            assert_eq!(failure.position.column, 1, "on `{line}`");
        }
    }

    /// Every planet size can actually be created, not only the one the release uses.
    ///
    /// `create planet tiny` was the only size any test had ever typed - the release is
    /// tiny, so nothing reached the other four. Five keys now depend on them, and a size
    /// that parses but fails to build would be a key that appears to do nothing while
    /// saying something about the wrong thing entirely.
    #[test]
    fn every_planet_size_can_be_created() {
        for size in planet_model::PlanetSize::ALL {
            let line = format!("create planet {}", size.name());
            let mut session = Session::new();
            let outcome = session
                .run(&line, &NoLibrary)
                .unwrap_or_else(|problem| panic!("`{line}` was refused: {problem}"));
            assert_eq!(outcome, Outcome::Changed, "`{line}`");
            assert_eq!(
                session.game.territories.len(),
                size.territory_count(),
                "`{line}` built the wrong number of territories"
            );
        }
    }

    #[test]
    fn a_blank_line_does_nothing_and_is_not_an_error() {
        let mut session = Session::new();
        assert_eq!(session.run("", &NoLibrary).unwrap(), Outcome::Nothing);
        assert_eq!(
            session.run("  # a note", &NoLibrary).unwrap(),
            Outcome::Nothing
        );
        assert!(session.history().is_empty());
    }

    #[test]
    fn a_command_that_changes_nothing_is_not_recorded_as_history() {
        let mut session = Session::new();
        session.run("help", &NoLibrary).unwrap();
        session.run("show turn", &NoLibrary).unwrap();
        assert!(
            session.history().is_empty(),
            "asking is not doing: {:?}",
            session.history()
        );
    }

    #[test]
    fn history_lists_what_was_done_in_order() {
        let mut session = Session::new();
        session.run("create planet tiny", &NoLibrary).unwrap();
        session.run("set force 1 1", &NoLibrary).unwrap();
        assert_eq!(session.history(), ["create planet tiny", "set force 1 1"]);
    }

    /// A history is the flat list of what changed the game, not an account of which file
    /// asked for it. That is what lets it be replayed on its own, with no files at all.
    #[test]
    fn history_records_what_a_subroutine_did_rather_than_the_call_to_it() {
        let library = Embedded::of(&[(
            "world",
            "create planet tiny
set force 1 1
",
        )]);
        let mut session = Session::new();
        session.run("run world", &library).unwrap();
        assert_eq!(session.history(), ["create planet tiny", "set force 1 1"]);

        let mut rebuilt = Session::new();
        rebuilt
            .run_script(
                &session.history().join(
                    "
",
                ),
                &NoLibrary,
            )
            .expect("a history replays without the files it came from");
        assert_eq!(rebuilt.game, session.game);
    }

    /// The three failures, each reported by the layer that found it and in that layer's
    /// terms.
    #[test]
    fn each_layer_reports_its_own_kind_of_problem() {
        let mut session = Session::new();

        let parse = session.run("land ark somewhere", &NoLibrary).unwrap_err();
        assert!(matches!(parse, Problem::Parse(_)), "{parse}");
        assert!(parse.to_string().contains("expected a number"), "{parse}");

        let misread = session
            .run("create planet enormous", &NoLibrary)
            .unwrap_err();
        assert!(matches!(misread, Problem::Misread(_)), "{misread}");

        session.run("create planet tiny", &NoLibrary).unwrap();
        session.run("start", &NoLibrary).unwrap();
        let rule = session.run("land ark 1", &NoLibrary).unwrap_err();
        assert!(matches!(rule, Problem::Rule(_)), "{rule}");
        assert!(rule.to_string().contains("no ark"), "{rule}");
    }

    /// A rejected command changes nothing. The game is exactly the transitions that were
    /// accepted, so one that was not must leave no trace.
    #[test]
    fn a_refused_command_leaves_the_game_untouched() {
        let mut session = Session::new();
        session.run("create planet tiny", &NoLibrary).unwrap();
        let before = session.game.clone();
        assert!(session.run("land ark 1", &NoLibrary).is_err());
        assert_eq!(session.game, before);
        assert_eq!(session.history(), ["create planet tiny"]);
    }

    /// `spec/console.md`: commands may be organized in a hierarchy of files, one file
    /// invoking another as a subroutine.
    #[test]
    fn a_file_may_call_another_file() {
        let library = Embedded::of(&[
            ("world", "create planet tiny\nrun forces\n"),
            ("forces", "set force 1 1\nset force 2 1\n"),
        ]);
        let mut session = Session::new();
        session.run("run world", &library).unwrap();
        assert_eq!(session.game.territories.len(), 12);
        assert_eq!(
            session
                .game
                .territory(game_model::TerritoryId(2))
                .unwrap()
                .force_of_nature,
            1
        );
    }

    #[test]
    fn a_file_that_is_not_there_says_which_ones_are() {
        let library = Embedded::of(&[("setup", "start\n")]);
        let mut session = Session::new();
        let problem = session.run("run missing", &library).unwrap_err();
        assert!(problem.to_string().contains("missing"), "{problem}");
        assert!(problem.to_string().contains("setup"), "{problem}");
    }

    #[test]
    fn files_that_call_each_other_without_end_are_stopped() {
        let library = Embedded::of(&[("a", "run b\n"), ("b", "run a\n")]);
        let mut session = Session::new();
        assert_eq!(
            session.run("run a", &library).unwrap_err(),
            Problem::TooDeep
        );
    }

    /// A failure inside a subroutine is reported against the line it is on.
    #[test]
    fn a_failure_inside_a_subroutine_names_its_own_line() {
        let library = Embedded::of(&[("setup", "create planet tiny\nland ark nowhere\n")]);
        let mut session = Session::new();
        let problem = session.run("run setup", &library).unwrap_err();
        match problem {
            Problem::Parse(failure) => assert_eq!(failure.position.line, 2),
            other => panic!("expected a parse failure, got {other}"),
        }
    }
}
