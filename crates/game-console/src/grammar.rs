//! The shape of every command, as data.
//!
//! This is one of the two tables the language is made of. The other is the binding in
//! [`crate::binding`], which says what each one means. Neither knows about the other, and
//! [`crate::tests`] asserts that together they cover exactly the same set of forms - the
//! check the predecessor did not have, where a form with no handler was an exception the
//! first time a player typed it.

use command_language::{Form, Grammar, Kind, Term};

/// Names used by both tables. Constants rather than literals so a typo cannot make a form
/// and its handler quietly disagree - they would fail to compile instead.
pub mod form {
    pub const CREATE_PLANET: &str = "create-planet";
    pub const ADD_NODE: &str = "add-node";
    pub const SET_FORCE: &str = "set-force";
    pub const ADD_UNIT: &str = "add-unit";
    pub const START: &str = "start";

    pub const LAND: &str = "land";
    pub const LAUNCH: &str = "launch";
    pub const MOVE: &str = "move";
    pub const BUILD: &str = "build";
    pub const PRODUCE: &str = "produce";
    pub const WORK: &str = "work";
    pub const END_TURN: &str = "end-turn";

    pub const SHOW_TERRITORY: &str = "show-territory";
    pub const SHOW_PLANET: &str = "show-planet";
    pub const SHOW_ORBIT: &str = "show-orbit";
    pub const SHOW_UNITS: &str = "show-units";
    pub const SHOW_TURN: &str = "show-turn";
    pub const HELP: &str = "help";
    pub const HISTORY: &str = "history";
    pub const RUN: &str = "run";
}

/// Every command, in the order they are tried.
///
/// Order is load-bearing, because matching is first-wins. Two rules follow from that and
/// both are obeyed below:
///
/// - A form whose opening words are a prefix of another's comes *after* it. `add node` is
///   listed before `add <unit> orbit`, or `node` would be read as a unit's name.
/// - The specific comes before the general. Each `show` subject is its own form.
pub fn grammar() -> Grammar {
    Grammar::new(vec![
        // -- designing the world, before `start` --------------------------
        Form::new(
            form::CREATE_PLANET,
            vec![
                Term::Keyword("create"),
                Term::Keyword("planet"),
                Term::required("size", Kind::Name),
            ],
            "make a planet and its territories",
        ),
        Form::new(
            form::ADD_NODE,
            vec![
                Term::Keyword("add"),
                Term::Keyword("node"),
                Term::required("territory", Kind::Number),
                Term::required("resource", Kind::Name),
                Term::required("density", Kind::Number),
            ],
            "give a territory one node",
        ),
        Form::new(
            form::SET_FORCE,
            vec![
                Term::Keyword("set"),
                Term::Keyword("force"),
                Term::required("territory", Kind::Number),
                Term::required("force", Kind::Number),
            ],
            "set a territory's force of nature",
        ),
        Form::new(
            form::ADD_UNIT,
            vec![
                Term::Keyword("add"),
                Term::required("unit", Kind::Name),
                Term::Keyword("orbit"),
            ],
            "place a unit in orbit before play begins",
        ),
        Form::new(
            form::START,
            vec![Term::Keyword("start")],
            "end the design phase and begin play",
        ),
        // -- playing ------------------------------------------------------
        Form::new(
            form::LAND,
            vec![
                Term::Keyword("land"),
                Term::required("unit", Kind::Name),
                Term::required("territory", Kind::Number),
            ],
            "bring a unit down from orbit; it founds the territory",
        ),
        Form::new(
            form::LAUNCH,
            vec![Term::Keyword("launch"), Term::required("unit", Kind::Name)],
            "send a unit from the territory it is in up to orbit",
        ),
        Form::new(
            form::MOVE,
            vec![
                Term::Keyword("move"),
                Term::required("unit", Kind::Name),
                Term::required("territory", Kind::Number),
            ],
            "move a unit to an adjacent territory, taking it if it is not yours",
        ),
        Form::new(
            form::BUILD,
            vec![
                Term::Keyword("build"),
                Term::required("structure", Kind::Name),
                Term::required("territory", Kind::Number),
                Term::optional("resource", Kind::Name),
            ],
            "build a structure, paying its cost there",
        ),
        Form::new(
            form::PRODUCE,
            vec![
                Term::Keyword("produce"),
                Term::required("unit", Kind::Name),
                Term::required("territory", Kind::Number),
            ],
            "produce a unit at a structure that allows it",
        ),
        Form::new(
            form::WORK,
            vec![
                Term::Keyword("work"),
                Term::required("count", Kind::Number),
                Term::required("structure", Kind::Name),
                Term::required("territory", Kind::Number),
                Term::optional("resource", Kind::Name),
            ],
            "spend that many citizens' labor at a structure this turn",
        ),
        Form::new(
            form::END_TURN,
            vec![Term::Keyword("end"), Term::Keyword("turn")],
            "consume, transform, and unspend everything",
        ),
        // -- asking, which changes nothing --------------------------------
        Form::new(
            form::SHOW_TERRITORY,
            vec![
                Term::Keyword("show"),
                Term::Keyword("territory"),
                Term::required("territory", Kind::Number),
            ],
            "report one territory",
        ),
        Form::new(
            form::SHOW_PLANET,
            vec![Term::Keyword("show"), Term::Keyword("planet")],
            "report every territory",
        ),
        Form::new(
            form::SHOW_ORBIT,
            vec![Term::Keyword("show"), Term::Keyword("orbit")],
            "report what is in orbit",
        ),
        Form::new(
            form::SHOW_UNITS,
            vec![Term::Keyword("show"), Term::Keyword("units")],
            "report every unit",
        ),
        Form::new(
            form::SHOW_TURN,
            vec![Term::Keyword("show"), Term::Keyword("turn")],
            "report the turn and the phase",
        ),
        Form::new(
            form::HELP,
            vec![Term::Keyword("help"), Term::optional("command", Kind::Name)],
            "list every command, or give one command's syntax",
        ),
        Form::new(
            form::HISTORY,
            vec![Term::Keyword("history")],
            "list every command executed so far, in order",
        ),
        // `spec/console.md` says commands may be organized in a hierarchy of files, one
        // invoking another as a subroutine, but names no command for doing it. This is
        // that command, and it is an addition to the language - see the release report.
        Form::new(
            form::RUN,
            vec![Term::Keyword("run"), Term::required("file", Kind::Name)],
            "run another command file as a subroutine",
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use command_language::parse_line;

    #[test]
    fn every_example_in_the_specification_parses() {
        // Verbatim from `spec/console.md`, The language.
        let examples = [
            ("land ark 1", form::LAND),
            ("move pioneer 7", form::MOVE),
            ("build extractor 3 metal", form::BUILD),
            ("produce pioneer 11", form::PRODUCE),
            ("work 4 extractor 3 metal", form::WORK),
            ("end turn", form::END_TURN),
            ("show territory 5", form::SHOW_TERRITORY),
            ("help move", form::HELP),
        ];
        for (line, expected) in examples {
            let parsed = parse_line(&grammar(), line, 1)
                .unwrap_or_else(|why| panic!("`{line}` did not parse: {why}"))
                .unwrap_or_else(|| panic!("`{line}` parsed as nothing"));
            assert_eq!(parsed.form, expected, "`{line}`");
        }
    }

    #[test]
    fn every_design_command_in_the_specification_parses() {
        let examples = [
            ("create planet tiny", form::CREATE_PLANET),
            ("add node 1 food 4", form::ADD_NODE),
            ("set force 1 1", form::SET_FORCE),
            ("add ark orbit", form::ADD_UNIT),
            ("start", form::START),
        ];
        for (line, expected) in examples {
            let parsed = parse_line(&grammar(), line, 1).unwrap().unwrap();
            assert_eq!(parsed.form, expected, "`{line}`");
        }
    }

    /// The ordering rule, made visible. `add node` opens with the same word as
    /// `add <unit> orbit`, and only its position in the list keeps `node` from being read
    /// as the name of a unit.
    #[test]
    fn add_node_is_not_mistaken_for_adding_a_unit_called_node() {
        let parsed = parse_line(&grammar(), "add node 1 food 4", 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.form, form::ADD_NODE);
        assert_eq!(parsed.number("territory").unwrap(), 1);
        assert_eq!(parsed.name("resource").unwrap(), "food");
        assert_eq!(parsed.number("density").unwrap(), 4);
    }

    #[test]
    fn an_optional_resource_may_be_left_off() {
        let parsed = parse_line(&grammar(), "build yard 11", 1).unwrap().unwrap();
        assert_eq!(parsed.optional_name("resource"), None);
        let parsed = parse_line(&grammar(), "work 2 garrison 1", 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.number("count").unwrap(), 2);
        assert_eq!(parsed.optional_name("resource"), None);
    }

    #[test]
    fn a_mistyped_command_is_told_what_was_expected_and_where() {
        let failure = parse_line(&grammar(), "land ark somewhere", 1).unwrap_err();
        assert!(
            failure.expected.contains(&"a number".to_string()),
            "{failure}"
        );
        assert_eq!(failure.position.column, 10);
    }
}
