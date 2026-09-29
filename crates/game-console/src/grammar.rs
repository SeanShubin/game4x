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
    pub const SET_RESOURCE: &str = "set-resource";
    pub const SET_BIOME: &str = "set-biome";
    pub const ADD_ARK: &str = "add-ark-orbit";
    pub const ADD_PIONEER: &str = "add-pioneer-orbit";
    pub const START: &str = "start";

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
/// # The form
///
/// **`P-321`: a command is written `{name field:value ...}`.** Its name is the words that open
/// it and its arguments are named, so a `Term::Keyword` here is a word of the name and a
/// `Term::Hole` is a field.
///
/// **`P-323`: a command is named for the recipe it fires**, which is why the kind is in the
/// name rather than in a hole. `build` used to be one form choosing between three recipes by
/// reading a word out of a position; it is `build extractor`, `build store` and `build yard`.
///
/// **`P-323`: a command may carry a `repeat`**, which is how many times it fires. It is not an
/// argument of the recipe, so `Session::run` applies the transition that many times rather
/// than handing a number to it. **It is on the twelve player commands and on nothing else** -
/// `end turn` fires the world's recipes and is not one of them, `show`, `help`, `history` and
/// `run` change nothing, and the design commands build a world before there is a game to
/// change, which `P-217` already says is not a recipe.
///
/// # Order
///
/// Order is load-bearing, because matching is first-wins, and **one of the two rules it used
/// to serve is retired.**
///
/// - **The specific comes before the general**, which still binds: each `show` subject is its
///   own form, and `move ark` and `move pioneer` are two.
/// - **A hole can no longer swallow a keyword**, which is what `P-321` retired. A field
///   carries its own name, so a word with no `name:` before it can only ever be part of a
///   name - and `{show-planet}` reaches the keyword form whichever way the two are listed.
pub fn grammar() -> Grammar {
    // **The written half and the read half.** What a player may state is fixed by what a world
    // is, so it is here; what a player may fire is whatever `spec/data/rules.4x` declares, so it
    // comes from [`crate::rules`]. `D-1`: *I change a recipe by editing a data file, with no Rust
    // edited, and the game fires the changed rule* - which a grammar with `build-yard` written
    // into it could not do.
    let mut forms = vec![
        // -- designing the world, before `start` --------------------------
        Form::new(
            form::CREATE_PLANET,
            vec![
                Term::Keyword("create-planet"),
                Term::required("size", Kind::Name),
            ],
            "make a planet and its territories",
        ),
        Form::new(
            form::SET_RESOURCE,
            vec![
                Term::Keyword("set-resource"),
                Term::required("territory", Kind::Number),
                Term::required("resource", Kind::Name),
                Term::required("extractors", Kind::Number),
                Term::required("density", Kind::Number),
            ],
            "say how many extractors a territory has room for, and what each yields",
        ),
        Form::new(
            form::SET_BIOME,
            vec![
                Term::Keyword("set-biome"),
                Term::required("territory", Kind::Number),
                Term::required("biome", Kind::Name),
            ],
            "give a territory its biome",
        ),
        // **Which orbit** - `S-55`. There are twelve, one above each territory, and
        // `add ark orbit` put a unit above nowhere.
        Form::new(
            form::ADD_ARK,
            vec![
                Term::Keyword("add-ark-orbit"),
                Term::required("territory", Kind::Number),
            ],
            "place an ark in the orbit above a territory before play begins",
        ),
        Form::new(
            form::ADD_PIONEER,
            vec![
                Term::Keyword("add-pioneer-orbit"),
                Term::required("territory", Kind::Number),
            ],
            "place a pioneer in the orbit above a territory before play begins",
        ),
        Form::new(
            form::START,
            vec![Term::Keyword("start")],
            "end the design phase and begin play",
        ),
        // -- asking, which changes nothing --------------------------------
        Form::new(
            form::SHOW_TERRITORY,
            vec![
                Term::Keyword("show-territory"),
                Term::required("id", Kind::Number),
            ],
            "what is in a territory, and what can be done there",
        ),
        Form::new(
            form::SHOW_PLANET,
            vec![Term::Keyword("show-planet")],
            "every territory at a glance",
        ),
        Form::new(
            form::SHOW_ORBIT,
            vec![Term::Keyword("show-orbit")],
            "what is in orbit",
        ),
        Form::new(
            form::SHOW_UNITS,
            vec![Term::Keyword("show-units")],
            "every unit and where it is",
        ),
        Form::new(
            form::SHOW_TURN,
            vec![Term::Keyword("show-turn")],
            "which turn it is",
        ),
        Form::new(
            form::HELP,
            vec![Term::Keyword("help"), Term::optional("command", Kind::Name)],
            "list every command, or give one command's syntax",
        ),
        Form::new(
            form::HISTORY,
            vec![Term::Keyword("history")],
            "every command run so far, in order",
        ),
        Form::new(
            form::RUN,
            vec![Term::Keyword("run"), Term::required("file", Kind::Name)],
            "run the commands in a file, as though they had been typed here",
        ),
    ];
    forms.extend(crate::rules::forms());
    Grammar::new(forms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use command_language::parse_line;

    /// Every player recipe has a command, and every command is named for its recipe.
    ///
    /// **`P-321` and `P-323` together.** The form is `{name field:value ...}` and the name is
    /// the recipe's own, so `build extractor` is two words of a name where it used to be a
    /// keyword and a word in a positional hole. The examples that used to be here were the
    /// eight lines `spec/console.md` carried in the old form, and `P-321` deleted them.
    #[test]
    fn every_example_in_the_specification_parses() {
        // **A rule's name is its command's name**, so these are the rules' own words rather
        // than a verb this lane chose - `{deploy ...}` where it used to be `{deploy ...}`.
        let examples = [
            ("{deploy where:2 what:ark}", "deploy"),
            ("{move what:pioneer from:3 to:7}", "move"),
            ("{build-extractor where:3 what:metal}", "build-extractor"),
            ("{build-pioneer where:11}", "build-pioneer"),
            ("{work where:3 what:metal}", "work"),
            ("{end-turn}", "end-turn"),
            ("{show-territory id:5}", form::SHOW_TERRITORY),
            ("{help command:move}", form::HELP),
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
            ("{create-planet size:tiny-12}", form::CREATE_PLANET),
            (
                "{set-resource territory:1 resource:food extractors:3 density:4}",
                form::SET_RESOURCE,
            ),
            ("{set-biome territory:1 biome:grassland}", form::SET_BIOME),
            ("{add-ark-orbit territory:1}", form::ADD_ARK),
            ("{start}", form::START),
        ];
        for (line, expected) in examples {
            let parsed = parse_line(&grammar(), line, 1).unwrap().unwrap();
            assert_eq!(parsed.form, expected, "`{line}`");
        }
    }

    /// A field carries its own name, so the order two of them are written in is not a rule.
    ///
    /// **This is the whole gain of `P-321` and it is what the predecessor could not do.** The
    /// old form bound `1` to `territory` by counting, so the same three words in another order
    /// were a different command or none.
    #[test]
    fn two_fields_mean_the_same_thing_in_either_order() {
        let one = parse_line(&grammar(), "{build-extractor where:3 what:metal}", 1)
            .unwrap()
            .unwrap();
        let other = parse_line(&grammar(), "{build-extractor what:metal where:3}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(one.form, other.form);
        assert_eq!(one.number("where").unwrap(), 3);
        assert_eq!(other.number("where").unwrap(), 3);
        assert_eq!(one.name("what").unwrap(), "metal");
        assert_eq!(other.name("what").unwrap(), "metal");
    }

    /// A field the command does not take is named, rather than reported as surplus.
    ///
    /// **A positional grammar could only say *end of line*.** It had no name to give back,
    /// because the thing that was wrong had never had a name - which is what `P-321` changes.
    #[test]
    fn a_field_a_command_does_not_take_says_which_field_it_is() {
        let failure = parse_line(&grammar(), "{build-yard where:3 what:metal}", 1)
            .expect_err("a yard is not built for a resource");
        let said = failure.to_string();
        assert!(
            said.contains("what:"),
            "the failure names the field that does not belong: {said}"
        );
    }

    /// A repeat is optional on the twelve player commands and on nothing else.
    ///
    /// **Over every form rather than on one**, because the interesting half is which forms do
    /// *not* take one: `end turn` fires the world's recipes and `show` changes nothing.
    #[test]
    fn only_a_command_that_fires_a_recipe_may_repeat() {
        let all = grammar();
        let takes_repeat: Vec<&str> = all
            .forms()
            .iter()
            .filter(|form| {
                form.terms
                    .iter()
                    .any(|term| matches!(term, Term::Hole { name: "repeat", .. }))
            })
            .map(|form| form.name)
            .collect();
        // **A rule takes a repeat and nothing else does**, which is the rule rather than a
        // number: `P-323` makes a repeat a count of firings, and only a rule fires.
        //
        // **`end-turn` takes one now and did not before.** It was excepted while the eleven
        // verbs were written here, on the grounds that it fired no recipe of the player's -
        // and under the data it is a rule like any other, so `{end-turn repeat:3}` ends three
        // turns. **The exception went with the list that needed it.**
        let rules: Vec<String> = crate::rules::playable()
            .into_iter()
            .map(|it| it.name)
            .collect();
        assert!(rules.len() >= 10, "only {} rule(s) read", rules.len());
        for rule in &rules {
            assert!(
                takes_repeat.contains(&rule.as_str()),
                "`{rule}` is a rule and does not take a repeat"
            );
        }
        assert_eq!(
            takes_repeat.len(),
            rules.len(),
            "only a rule takes a repeat: {takes_repeat:?}"
        );
        for named in [form::SHOW_PLANET, form::RUN, form::CREATE_PLANET] {
            assert!(
                !takes_repeat.contains(&named),
                "`{named}` fires no rule and must not take a repeat"
            );
        }
        let parsed = parse_line(&grammar(), "{toil where:1 repeat:2}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.optional_number("repeat"), Some(2));
        let once = parse_line(&grammar(), "{toil where:1}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(
            once.optional_number("repeat"),
            None,
            "a command without one fires once, and says so by carrying nothing"
        );
    }

    /// No two commands share an opening word, so ordered choice decides nothing.
    ///
    /// **This used to be the ordering rule and `P-328` retired it.** A command's name is one
    /// word, dashed where it needs more, so `build-store` and `build-extractor` share no
    /// token at all - and the first-wins order that used to be load-bearing now settles
    /// nothing, because at most one form can match any name.
    ///
    /// **The rule outlived two examples and now outlives its own population.** It was shown
    /// by `add node` against `add <unit> orbit` until `P-149` deleted `add node`; it was
    /// checked over every pair after that; and the pairs are gone. So this asserts the
    /// property that replaced it - **every name is distinct and every name is one word** -
    /// which is what makes the order irrelevant rather than merely unexercised.
    #[test]
    fn every_command_name_is_one_word_and_no_two_are_the_same() {
        let all = grammar();
        let mut names: Vec<String> = Vec::new();
        for form in all.forms() {
            let opening: Vec<&str> = form
                .terms
                .iter()
                .map_while(|term| match term {
                    Term::Keyword(word) => Some(*word),
                    Term::Hole { .. } => None,
                })
                .collect();
            assert_eq!(
                opening.len(),
                1,
                "`{}` opens with {} words and a name is one - `P-328`",
                form.name,
                opening.len()
            );
            let name = opening[0];
            assert!(
                !name.contains(' '),
                "`{name}` has a space in it, and a name that needs more than one word joins \
                 them with dashes"
            );
            names.push(name.to_string());
        }
        assert!(
            names.len() > 20,
            "only {} forms, so this agrees with almost anything",
            names.len()
        );
        let mut unique = names.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            names.len(),
            "two commands share a name, so one of them is unreachable whatever the order"
        );
    }

    #[test]
    fn an_optional_resource_may_be_left_off() {
        let parsed = parse_line(&grammar(), "{build-yard where:11}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.optional_name("what"), None);
        // A repeat is the optional field every player command carries, and one left off
        // means the command fires once.
        let parsed = parse_line(&grammar(), "{toil where:1 repeat:2}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.optional_number("repeat"), Some(2));
    }

    #[test]
    fn a_mistyped_command_is_told_what_was_expected_and_where() {
        let failure = parse_line(&grammar(), "{deploy where:somewhere what:ark}", 1).unwrap_err();
        assert!(
            failure.expected.contains(&"a number".to_string()),
            "{failure}"
        );
        // At the value rather than at the field that carried it: `where:` opens at 9 and
        // `somewhere` at 15, and the value is what has to change.
        assert_eq!(failure.position.column, 15);
    }
}
