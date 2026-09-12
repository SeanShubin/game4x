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
pub mod browse;
pub mod declare;
pub mod dump;
pub mod fired;
pub mod grammar;
pub mod nogain;
pub mod petri;
pub mod petri_draw;
pub mod petri_page;
pub mod recipes;
pub mod report;
pub mod state;
pub mod style;
pub mod tree;
pub mod worked;

use command_language::{Failure, Grammar, parse_line};
use game_model::{Game, Rejection};

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

/// Where a problem was found, and what led there.
///
/// **`P-215`.** A failure used to say what was wrong and, for a parse failure alone, which
/// column. Nothing said which *line*, and nothing said which file - so a rejection raised on
/// line five of `world.4x`, reached by `setup.4x` saying `run world`, reached in turn by the
/// console saying `run setup`, was reported as a bare sentence about the game.
///
/// **The chain is the half that is easy to skip and is the half that makes it debuggable.**
/// A line number alone is worse than none when seven files are in play: it names a line in a
/// file the reader has to guess.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Where {
    /// Which line of the file it was in. One-based, as an editor counts.
    pub line: usize,
    /// Which column, where that means anything.
    ///
    /// **`None` rather than one**, because only a parse failure has a column: it failed at a
    /// character. A misreading is about a word the parser already accepted and a rejection
    /// is about the whole command, so pointing at column one would be inventing a precision
    /// neither of them has.
    pub column: Option<usize>,
    /// The `run` commands enclosing it, outermost first. Empty at the console.
    pub inside: Vec<String>,
}

impl std::fmt::Display for Where {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "line {}", self.line)?;
        if let Some(column) = self.column {
            write!(out, ", column {column}")?;
        }
        if !self.inside.is_empty() {
            write!(out, ", inside `{}`", self.chain())?;
        }
        Ok(())
    }
}

impl Where {
    /// The `run` commands that led here, innermost first.
    ///
    /// Innermost first because that is the order a reader wants: the file the line is in,
    /// then how it was reached. The list is stored outermost first because that is the order
    /// it is built in, and reversing it here keeps one truth about the order in each place.
    pub fn chain(&self) -> String {
        self.inside
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>()
            .join("`, called from `")
    }
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
    /// One of the above, and where it happened.
    ///
    /// **A wrapper rather than a field on each**, so that adding the location did not have
    /// to be repeated five times and cannot be forgotten on the sixth. It is added once, at
    /// the one place that knows both the line and the chain of files that reached it.
    At {
        found: Where,
        what: Box<Problem>,
    },
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
            // **A parse failure already says where it is**, because the parser knew the
            // character it stopped at and has printed the line and column for as long as it
            // has existed. Repeating them reads as two different positions that happen to
            // agree. So what is added to that one is the part it could not know: which file
            // it was in, and what called it.
            Problem::At { found, what } => match **what {
                Problem::Parse(_) if found.inside.is_empty() => write!(out, "{what}"),
                Problem::Parse(_) => write!(out, "{what} (inside `{}`)", found.chain()),
                _ => write!(out, "{what} ({found})"),
            },
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
        self.run_at(line, 1, library, &[])
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
        self.run_script_at(text, library, &[])
    }

    fn run_script_at(
        &mut self,
        text: &str,
        library: &dyn Library,
        inside: &[String],
    ) -> Result<Vec<Outcome>, Problem> {
        let mut outcomes = Vec::new();
        for (offset, line) in text.lines().enumerate() {
            outcomes.push(self.run_at(line, offset + 1, library, inside)?);
        }
        Ok(outcomes)
    }

    fn run_at(
        &mut self,
        line: &str,
        line_number: usize,
        library: &dyn Library,
        inside: &[String],
    ) -> Result<Outcome, Problem> {
        // **Added once, here, where both halves are known.** The line is this function's
        // argument and the chain is its caller's; nowhere below this knows either, and
        // nowhere above this knows what went wrong.
        let locate = |what: Problem, column: Option<usize>| Problem::At {
            found: Where {
                line: line_number,
                column,
                inside: inside.to_vec(),
            },
            what: Box::new(what),
        };

        let Some(utterance) = parse_line(&self.grammar, line, line_number).map_err(|failure| {
            let column = failure.position.column;
            locate(Problem::Parse(failure), Some(column))
        })?
        else {
            return Ok(Outcome::Nothing);
        };
        let meaning = interpret(&utterance).map_err(|why| locate(Problem::Misread(why), None))?;

        match meaning {
            Meaning::Change(transition) => {
                // **`P-323`: a command may carry a `repeat`, and one without it fires once.**
                // A count of firings rather than an argument of the recipe, so this loops over
                // the same transition rather than handing a number to it.
                //
                // **All of them or none.** `spec/invariants.md` says a command that cannot be
                // run changes nothing, and a repeat that fails on its third firing would
                // otherwise leave two behind - a command half executed, which is a state no
                // history could reproduce. So the firings are applied to a copy and the copy
                // replaces the game only once every one of them has succeeded.
                // **A negative repeat cannot be written**: `Kind::Number` reads digits
                // only, deliberately, because every quantity in this language is a count, a
                // density or an identifier and none of those is ever negative.
                //
                // **A repeat of zero fires nothing and is accepted**, because no rule says it
                // may not, and refusing it would be a rule this lane invented rather than
                // one it was given. `C-54`.
                let repeat = utterance.optional_number("repeat").unwrap_or(1);
                let mut next = self.game.clone();
                for _ in 0..repeat {
                    next = next
                        .after(&transition)
                        .map_err(|why| locate(Problem::Rule(why), None))?;
                }
                self.game = next;
                self.history.push(utterance.source.clone());
                Ok(Outcome::Changed)
            }
            Meaning::Show(subject) => Ok(Outcome::Said(report::show(&self.game, &subject))),
            Meaning::Help(command) => Ok(Outcome::Said(report::help(&self.grammar, command))),
            Meaning::History => Ok(Outcome::Said(report::history(&self.history))),
            Meaning::Run(name) => {
                // The chain's length is the depth, so one field does both jobs and the two
                // cannot disagree - a counter beside a list is a second account of the same
                // fact.
                if inside.len() >= DEEPEST {
                    return Err(locate(Problem::TooDeep, None));
                }
                let text = library.fetch(&name).ok_or_else(|| {
                    locate(
                        Problem::NoSuchFile {
                            name: name.clone(),
                            known: library.names(),
                        },
                        None,
                    )
                })?;
                // Calling a file is not itself a change, and the commands inside it
                // record themselves. Recording both would make the history do everything
                // twice when it is replayed - which is what a history is *for*, since it
                // is the only account of how a game got where it is.
                let mut deeper = inside.to_vec();
                deeper.push(utterance.source.clone());
                self.run_script_at(&text, library, &deeper)?;
                Ok(Outcome::Changed)
            }
        }
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
            "deploy-ark",
            "launch-ark",
            "move",
            "build-extractor",
            "build-store",
            "build-yard",
            "produce-pioneer",
            "work",
            "end-turn",
            "show-territory",
            "help",
            "history",
            "create-planet",
            "create-labor",
            "add-ark-orbit",
            "set-force",
            "start",
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
    /// **The population is asserted, because the doc above claims coverage - `Q-76`.** *Two
    /// things are asserted, because the rule needs both* is a claim about every form, and an
    /// empty `forms()` would satisfy it silently while checking no command at all. The grammar
    /// is the whole vocabulary, which is what makes this the one worth guarding first.
    #[test]
    fn no_command_can_begin_with_a_slash() {
        let grammar = grammar::grammar();
        let forms = grammar.forms();

        assert!(
            forms.len() >= 10,
            "the console declares {} forms, which is too few to be its grammar",
            forms.len()
        );
        for form in forms {
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
            let line = format!("{{create-planet size:{}}}", size.name());
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
        session.run("{help}", &NoLibrary).unwrap();
        session.run("{show-turn}", &NoLibrary).unwrap();
        assert!(
            session.history().is_empty(),
            "asking is not doing: {:?}",
            session.history()
        );
    }

    #[test]
    fn history_lists_what_was_done_in_order() {
        let mut session = Session::new();
        session
            .run("{create-planet size:tiny}", &NoLibrary)
            .unwrap();
        session
            .run("{set-force territory:1 force:1}", &NoLibrary)
            .unwrap();
        assert_eq!(
            session.history(),
            [
                "{create-planet size:tiny}",
                "{set-force territory:1 force:1}"
            ]
        );
    }

    /// A history is the flat list of what changed the game, not an account of which file
    /// asked for it. That is what lets it be replayed on its own, with no files at all.
    #[test]
    fn history_records_what_a_subroutine_did_rather_than_the_call_to_it() {
        let library = Embedded::of(&[(
            "world",
            "{create-planet size:tiny}
{set-force territory:1 force:1}
",
        )]);
        let mut session = Session::new();
        session.run("{run file:world}", &library).unwrap();
        assert_eq!(
            session.history(),
            [
                "{create-planet size:tiny}",
                "{set-force territory:1 force:1}"
            ]
        );

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

        // Every problem is located now - `P-215` - so each is unwrapped before its kind is
        // asked about. Where it was found is checked in its own tests; this one is still
        // about the three layers reporting three different kinds of thing.
        let at = |problem: &Problem| match problem {
            Problem::At { what, .. } => (**what).clone(),
            other => panic!("every problem carries where it was found; got {other}"),
        };

        let parse = session
            .run("{deploy-ark territory:somewhere}", &NoLibrary)
            .unwrap_err();
        assert!(matches!(at(&parse), Problem::Parse(_)), "{parse}");
        // And it says its position once rather than twice: the parser already knew the
        // column, so the wrapper adds only what the parser could not know.
        assert_eq!(parse.to_string().matches("column").count(), 1, "{parse}");
        assert!(parse.to_string().contains("expected a number"), "{parse}");

        let misread = session
            .run("{create-planet size:enormous}", &NoLibrary)
            .unwrap_err();
        assert!(matches!(at(&misread), Problem::Misread(_)), "{misread}");

        session
            .run("{create-planet size:tiny}", &NoLibrary)
            .unwrap();
        session.run("{start}", &NoLibrary).unwrap();
        let rule = session
            .run("{deploy-ark territory:1}", &NoLibrary)
            .unwrap_err();
        assert!(matches!(at(&rule), Problem::Rule(_)), "{rule}");
        assert!(rule.to_string().contains("no ark"), "{rule}");
        // A rejection is about the whole command, so it says the line and no column.
        assert!(rule.to_string().contains("line 1"), "{rule}");
        assert!(!rule.to_string().contains("column"), "{rule}");
    }

    /// A rejected command changes nothing. The game is exactly the transitions that were
    /// accepted, so one that was not must leave no trace.
    #[test]
    fn a_refused_command_leaves_the_game_untouched() {
        let mut session = Session::new();
        session
            .run("{create-planet size:tiny}", &NoLibrary)
            .unwrap();
        let before = session.game.clone();
        assert!(session.run("{deploy-ark territory:1}", &NoLibrary).is_err());
        assert_eq!(session.game, before);
        assert_eq!(session.history(), ["{create-planet size:tiny}"]);
    }

    /// `spec/console.md`: commands may be organized in a hierarchy of files, one file
    /// invoking another as a subroutine.
    #[test]
    fn a_file_may_call_another_file() {
        let library = Embedded::of(&[
            ("world", "{create-planet size:tiny}\n{run file:forces}\n"),
            (
                "forces",
                "{set-force territory:1 force:1}\n{set-force territory:2 force:1}\n",
            ),
        ]);
        let mut session = Session::new();
        session.run("{run file:world}", &library).unwrap();
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
        let problem = session.run("{run file:missing}", &library).unwrap_err();
        assert!(problem.to_string().contains("missing"), "{problem}");
        assert!(problem.to_string().contains("setup"), "{problem}");
    }

    #[test]
    fn files_that_call_each_other_without_end_are_stopped() {
        let library = Embedded::of(&[("a", "{run file:b}\n"), ("b", "{run file:a}\n")]);
        let mut session = Session::new();
        let problem = session.run("{run file:a}", &library).unwrap_err();
        let Problem::At { found, what } = problem else {
            panic!("expected a located problem")
        };
        assert_eq!(*what, Problem::TooDeep);
        // The chain is the depth, so one says the other rather than a counter agreeing with
        // a list. Sixteen calls deep is `DEEPEST`, which is what stopped it.
        assert_eq!(found.inside.len(), DEEPEST);
        assert!(found.to_string().contains("called from"), "{found}");
    }

    /// A failure inside a subroutine is reported against the line it is on.
    #[test]
    fn a_failure_inside_a_subroutine_names_its_own_line() {
        let library = Embedded::of(&[(
            "setup",
            "{create-planet size:tiny}\n{deploy-ark territory:nowhere}\n",
        )]);
        let mut session = Session::new();
        let problem = session.run("{run file:setup}", &library).unwrap_err();
        let Problem::At { found, what } = &problem else {
            panic!("expected a located problem, got {problem}")
        };
        match &**what {
            Problem::Parse(failure) => assert_eq!(failure.position.line, 2),
            other => panic!("expected a parse failure, got {other}"),
        }
        // **And which file that line is in** - `P-215`. A line number alone is worse than
        // none when seven files are in play, because it names a line in a file the reader
        // has to guess. That is the half that was missing.
        assert_eq!(found.line, 2);
        assert_eq!(found.inside, ["{run file:setup}"]);
        assert!(
            problem.to_string().contains("inside `{run file:setup}`"),
            "{problem}"
        );
    }
}
