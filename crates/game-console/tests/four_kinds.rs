//! What a line changes says which kind of line it is, over every command the grammar declares.
//!
//! **`spec/console.md`, and the sentence this file exists for**: *the game knows nothing of the
//! interface. Local state is not game state, no rule reads it, and no local command is a
//! transition - so a replay of the history is a replay of the game and not of the clicking.*
//!
//! That file names **four kinds of line**, each identified by what it changes:
//!
//! | Kind | Transition | In the history | Logged |
//! | ---- | ---------- | -------------- | ------ |
//! | a game command | yes | **yes** | yes |
//! | a local command | no | **no** | yes |
//! | a query | no | **no** | yes |
//! | a front-end line | no | **no** | **no** |
//!
//! # Why this is a check and not a paragraph
//!
//! **The rule was promoted on 2026-09-24 and nothing held it.** The quality lens grepped
//! `crates/` for `local state`, `local_state` and `LocalState` and found nothing at all - *a
//! rule, a vocabulary and no code* - which it reported rather than counting as a violation,
//! because **a count of zero over a population of zero says nothing.** Re-run here and still
//! empty: `Meaning` has no local-command variant, so the second row of that table is a kind
//! the console cannot yet produce.
//!
//! **So what is checkable today is the first, third and fourth rows**, and the assertions below
//! say which rows they are about rather than implying the table is covered.
//!
//! # What was here before, and why a hand list was not enough
//!
//! `no_question_ever_changes_the_game` runs **nine** named questions and compares the state
//! either side. That is right and it is a list: a form added to the grammar is not in it, and
//! nothing says so. **This iterates the grammar**, so a tenth query is covered the moment it
//! exists - the same instinct as `every_crate_has_a_row_and_every_row_has_a_crate` walking the
//! workspace.
//!
//! # How a line is built for a form nobody wrote a line for
//!
//! A form's holes have kinds - a name, a number, another command - and a name's *values* are
//! the game's vocabulary, which the grammar does not carry. **So the test asks rather than
//! knows**: it fills a name hole with a word that names nothing, reads the expectations off the
//! refusal, and fills the hole with the first thing the console itself said it expected.
//!
//! **That works because of `S-155`** - *a rejection names what was wrong, where, and what was
//! expected instead* - so this check is also a consumer of that rule, and would go red if a
//! refusal stopped carrying its expectations.

use game_console::{Library, Meaning, Outcome, Session};
use std::collections::BTreeMap;
use std::path::PathBuf;

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn files() -> Files {
    Files(root().join("scenario/commands"))
}

/// A designed planet with play begun, so every form has something real to act on.
fn playing() -> Session {
    let mut session = Session::new();
    for line in ["{run file:setup}", "{start}"] {
        session
            .run(line, &files())
            .unwrap_or_else(|why| panic!("`{line}` failed: {why}"));
    }
    session
}

/// A line for one form, with every hole filled - asking the console what it expected whenever
/// a word names nothing.
///
/// Returns `None` for a form whose holes cannot be filled this way, and which those are is
/// asserted by the caller rather than being allowed to grow quietly.
fn line_for(session: &Session, name: &str) -> Option<String> {
    let form = session.grammar().form(name)?;
    let mut filled: BTreeMap<&str, String> = BTreeMap::new();
    // Ten passes is more than any form needs and terminates rather than looping: each pass
    // either learns one hole's vocabulary or finishes the line.
    for _ in 0..10 {
        let mut words = vec![name.to_string()];
        for (hole, kind, required) in form.holes() {
            if !required && !filled.contains_key(hole) {
                continue;
            }
            let value = filled.get(hole).cloned().unwrap_or_else(|| match kind {
                command_language::Kind::Number => "1".to_string(),
                // A word that names nothing, so the refusal says what would have.
                command_language::Kind::Name => "nothing-names-this".to_string(),
                command_language::Kind::Command => "{show-turn}".to_string(),
            });
            words.push(format!("{hole}:{value}"));
        }
        let line = format!("{{{}}}", words.join(" "));

        // **A fresh game each time rather than a clone of this one.** `Session` is not
        // `Clone`, and deriving it on the shipped type so that a test could copy a state is
        // exactly the sort of thing `Q-99` is about - the check would have changed the code it
        // is checking.
        let mut trying = playing();
        match trying.run(&line, &files()) {
            Ok(_) => return Some(line),
            Err(problem) => {
                // **A rule refusing the command is fine and is not this function's business.**
                // It was understood, which is all a classification needs. What has to be fixed
                // is a word the console could not read.
                let text = format!("{problem:?}");
                if !text.contains("Unknown") {
                    return Some(line);
                }
                let learned = expected_word(&text)?;
                let (hole, _, _) = form.holes().find(|(hole, kind, _)| {
                    *kind == command_language::Kind::Name && !filled.contains_key(hole)
                })?;
                filled.insert(hole, learned);
            }
        }
    }
    None
}

/// What a line means, without running it.
///
/// **This is the console's own classification** - `Meaning` is the type `binding.rs` exists to
/// produce, and its doc says why the kinds are kept apart in the type: *keeping them apart is
/// what stops a query accidentally becoming a way to change state.* Asking it is asking the
/// code what kind of line this is, rather than guessing from what happened.
fn meaning_of(session: &Session, line: &str) -> Option<Meaning> {
    let utterance = command_language::parse_line(session.grammar(), line, 1).ok()??;
    game_console::interpret(&utterance).ok()
}

/// The first word a refusal says it expected, read out of its debug form.
fn expected_word(text: &str) -> Option<String> {
    let after = text.split("expected:").nth(1)?;
    let inside = after.split('[').nth(1)?.split(']').next()?;
    let first = inside.split(',').next()?.trim().trim_matches('"');
    (!first.is_empty()).then(|| first.to_string())
}

/// Every form the grammar declares is a game command or a query, and which it is decides
/// whether it enters the history.
///
/// **The first and third rows of the table above**, over every form rather than a list. A game
/// command puts exactly one line in the history; a query puts none and leaves the game equal.
#[test]
fn every_form_is_a_game_command_or_a_query_and_the_history_says_which() {
    let session = playing();
    let names: Vec<&str> = session.grammar().form_names();
    assert!(
        names.len() > 20,
        "only {} forms declared, so this would be about almost nothing",
        names.len()
    );

    let mut commands = Vec::new();
    let mut queries = Vec::new();
    let mut calls = Vec::new();
    let mut unreadable = Vec::new();

    for name in &names {
        let Some(line) = line_for(&session, name) else {
            unreadable.push((*name).to_string());
            continue;
        };
        // **Classified by what the line means, not by whether running it worked.** The first
        // version of this read the outcome: an `Err` was taken for a game command, because a
        // refused command is understood and refused. **That is the instrument answering a
        // narrower question than the one asked** - `{run file:nothing-names-this}` is also an
        // `Err`, and `run` was classified as a game command, which it is not. What a line
        // means is `Meaning`, and that is the classification the code actually has.
        let Some(meaning) = meaning_of(&session, &line) else {
            unreadable.push((*name).to_string());
            continue;
        };

        let mut trying = playing();
        let before = trying.game.clone();
        let started = trying.history().len();
        let outcome = trying.run(&line, &files());
        let grew = trying.history().len() - started;

        match meaning {
            Meaning::Change(_) => {
                match outcome {
                    // The rules refused it, which is legal and changes nothing -
                    // `spec/invariants.md` - so it records nothing either.
                    Err(_) => {
                        assert_eq!(
                            grew, 0,
                            "`{line}` was refused and still entered the history, so a command \
                             that changed nothing is in the account of how the game got here"
                        );
                        assert_eq!(
                            trying.game, before,
                            "`{line}` was refused and moved the game anyway"
                        );
                    }
                    Ok(_) => assert_eq!(
                        grew, 1,
                        "`{line}` is a game command and put {grew} line(s) in the history, \
                         and a game command is exactly one"
                    ),
                }
                commands.push((*name).to_string());
            }
            // **A query changes nothing and reports** - and the history is what a replay runs,
            // so a question in it would be a question the replay asks again.
            Meaning::Show(_) | Meaning::Help(_) | Meaning::History => {
                assert!(
                    matches!(outcome, Ok(Outcome::Said(_))),
                    "`{line}` means a question and running it did not answer one"
                );
                assert_eq!(
                    grew, 0,
                    "`{line}` answered a question and put {grew} line(s) in the history"
                );
                assert_eq!(trying.game, before, "`{line}` answered and moved the game");
                queries.push((*name).to_string());
            }
            // **`run` is none of the four kinds and that is the point of naming it.** It is a
            // line that types other lines: what it ran is the history, and it is not, so a
            // replay does not call the file again.
            Meaning::Run(_) => {
                assert_eq!(
                    grew, 0,
                    "`{line}` called a file and recorded itself, so a replay would run it twice"
                );
                calls.push((*name).to_string());
            }
        }
    }

    // **Both populations, because either alone proves nothing.** All queries and no commands
    // would satisfy every assertion in the query arm and say nothing about the other.
    assert!(
        commands.len() > 15,
        "only {} forms behaved as a game command: {commands:?}",
        commands.len()
    );
    assert!(
        queries.len() >= 5,
        "only {} forms behaved as a query: {queries:?}",
        queries.len()
    );
    assert_eq!(
        commands.len() + queries.len() + calls.len() + unreadable.len(),
        names.len(),
        "every declared form is accounted for exactly once"
    );
    // **`run` is the only form that is neither**, and it is named rather than left in a
    // bucket: it is the one line that types other lines.
    assert_eq!(
        calls,
        ["run"],
        "only `run` calls a file, and these forms did"
    );
    // **Nothing is unreadable, and the assertion is that rather than a tolerance.** A form this
    // test cannot build a line for is a form it says nothing about, so the number that may be
    // hidden that way is zero.
    assert!(
        unreadable.is_empty(),
        "no line could be built for these forms, so this check says nothing about them: \
         {unreadable:?}"
    );
}

/// Everything the scenario's history holds is a game command, over all of it.
///
/// **The replay half is `the_history_of_a_game_is_enough_to_rebuild_it`**, which shows the
/// history reaches the same state. This is the other half of the same sentence: that what is
/// in it is only the game. **A history carrying a query would still replay to the right
/// state** - the question would simply be asked again - so replaying correctly does not show
/// this, and nothing else did.
#[test]
fn nothing_in_the_history_is_anything_but_a_game_command() {
    let mut session = playing();
    session
        .run("{run file:play}", &files())
        .expect("the scenario plays");

    let asking = ["show-", "help", "history"];
    let mut checked = 0;
    for line in session.history() {
        let opening = line.trim_start_matches('{');
        let opening = opening.split([' ', '}']).next().unwrap_or("");
        assert!(
            !asking.iter().any(|kind| opening.starts_with(kind)),
            "`{line}` is in the history and it is a question, which a replay would ask again"
        );
        // **A front-end line begins with `/` and is not a command at all**, so one in the
        // history would be the application's business recorded as the game's.
        assert!(
            !line.trim_start().starts_with('/'),
            "`{line}` directs the front end and is in the game's history"
        );
        // **A `run` records what it did rather than that it was called**, so the history is
        // flat and replays on its own.
        assert_ne!(
            opening, "run",
            "`{line}` is in the history, and a replay of it would run the file twice"
        );
        checked += 1;
    }
    assert!(
        checked > 200,
        "only {checked} history lines examined, and the scenario is 211 commands - a run that \
         recorded almost nothing would satisfy every assertion above"
    );
}

/// No form the grammar declares begins with `/`.
///
/// **`spec/console.md`: a line beginning with `/` directs the front end rather than the game.
/// It is not a command, and it is not logged.** *Not a command* is checkable here and is the
/// half that would break silently: a form declared with that opening would make the console
/// the place the application is steered from, which is the boundary this whole file is about.
#[test]
fn no_command_steers_the_front_end() {
    let session = Session::new();
    let forms = session.grammar().forms();
    assert!(
        forms.len() > 20,
        "only {} forms declared, so a count of zero below would mean nothing",
        forms.len()
    );
    let steering: Vec<&str> = forms
        .iter()
        .map(|form| form.name)
        .filter(|name| name.starts_with('/'))
        .collect();
    assert!(
        steering.is_empty(),
        "these forms are commands and direct the front end: {steering:?}"
    );

    // **And the console does not read one as a command.** `/game` is a real front-end line
    // from that file's own list.
    let mut trying = Session::new();
    let before = trying.history().len();
    let outcome = trying.run("/game", &files());
    assert!(
        outcome.is_err() || matches!(outcome, Ok(Outcome::Nothing)),
        "the console read `/game` as a command"
    );
    assert_eq!(
        trying.history().len(),
        before,
        "`/game` directs the front end and was logged in the game's history"
    );
}
