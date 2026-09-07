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
    pub const SET_FORCE: &str = "set-force";
    pub const SET_BIOME: &str = "set-biome";
    pub const ADD_ARK: &str = "add-ark-orbit";
    pub const ADD_PIONEER: &str = "add-pioneer-orbit";
    pub const START: &str = "start";

    // **One id per recipe, because a command is named for the recipe it fires** - `P-323`.
    // `BUILD` and `PRODUCE` each covered several, choosing between them by reading a word the
    // player had put in a positional hole; the word is part of the name now.
    pub const DEPLOY_ARK: &str = "deploy ark";
    pub const LAUNCH_ARK: &str = "launch ark";
    pub const MOVE_ARK: &str = "move ark";
    pub const MOVE_PIONEER: &str = "move pioneer";
    pub const FOUND_BY_LAND: &str = "found by land";
    pub const BUILD_STORE: &str = "build store";
    pub const BUILD_EXTRACTOR: &str = "build extractor";
    pub const BUILD_YARD: &str = "build yard";
    pub const PRODUCE_PIONEER: &str = "produce pioneer";
    pub const PRODUCE_ARK: &str = "produce ark";
    pub const CREATE_LABOR: &str = "create labor";
    pub const WORK_EXTRACTOR: &str = "work extractor";
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
///   name - and `{show planet}` reaches the keyword form whichever way the two are listed.
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
            form::SET_RESOURCE,
            vec![
                Term::Keyword("set"),
                Term::Keyword("resource"),
                Term::required("territory", Kind::Number),
                Term::required("resource", Kind::Name),
                Term::required("extractors", Kind::Number),
                Term::required("density", Kind::Number),
            ],
            "say how many extractors a territory has room for, and what each yields",
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
            form::SET_BIOME,
            vec![
                Term::Keyword("set"),
                Term::Keyword("biome"),
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
                Term::Keyword("add"),
                Term::Keyword("ark"),
                Term::Keyword("orbit"),
                Term::required("territory", Kind::Number),
            ],
            "place an ark in the orbit above a territory before play begins",
        ),
        Form::new(
            form::ADD_PIONEER,
            vec![
                Term::Keyword("add"),
                Term::Keyword("pioneer"),
                Term::Keyword("orbit"),
                Term::required("territory", Kind::Number),
            ],
            "place a pioneer in the orbit above a territory before play begins",
        ),
        Form::new(
            form::START,
            vec![Term::Keyword("start")],
            "end the design phase and begin play",
        ),
        // -- playing ------------------------------------------------------
        Form::new(
            form::DEPLOY_ARK,
            vec![
                Term::Keyword("deploy"),
                Term::Keyword("ark"),
                Term::required("territory", Kind::Number),
                Term::optional("repeat", Kind::Number),
            ],
            "bring an ark down from the orbit above a territory; it founds the territory",
        ),
        Form::new(
            form::LAUNCH_ARK,
            vec![
                Term::Keyword("launch"),
                Term::Keyword("ark"),
                Term::optional("repeat", Kind::Number),
            ],
            "send an ark from the territory it is in up to orbit",
        ),
        Form::new(
            form::MOVE_ARK,
            vec![
                Term::Keyword("move"),
                Term::Keyword("ark"),
                Term::required("territory", Kind::Number),
                Term::optional("repeat", Kind::Number),
            ],
            "move an ark to an adjacent territory that is already yours",
        ),
        Form::new(
            form::MOVE_PIONEER,
            vec![
                Term::Keyword("move"),
                Term::Keyword("pioneer"),
                Term::required("territory", Kind::Number),
                Term::optional("repeat", Kind::Number),
            ],
            "move a pioneer to an adjacent territory that is already yours",
        ),
        Form::new(
            form::FOUND_BY_LAND,
            vec![
                Term::Keyword("found"),
                Term::Keyword("by"),
                Term::Keyword("land"),
                Term::required("territory", Kind::Number),
                Term::optional("repeat", Kind::Number),
            ],
            "send a pioneer onto adjacent unclaimed ground and found it there",
        ),
        Form::new(
            form::BUILD_STORE,
            vec![
                Term::Keyword("build"),
                Term::Keyword("store"),
                Term::required("territory", Kind::Number),
                Term::required("resource", Kind::Name),
                Term::optional("repeat", Kind::Number),
            ],
            "build somewhere to keep one resource; a store holds ten of it",
        ),
        Form::new(
            form::BUILD_EXTRACTOR,
            vec![
                Term::Keyword("build"),
                Term::Keyword("extractor"),
                Term::required("territory", Kind::Number),
                Term::required("resource", Kind::Name),
                Term::optional("repeat", Kind::Number),
            ],
            "build an extractor for one resource, paying its cost there",
        ),
        Form::new(
            form::BUILD_YARD,
            vec![
                Term::Keyword("build"),
                Term::Keyword("yard"),
                Term::required("territory", Kind::Number),
                Term::optional("repeat", Kind::Number),
            ],
            "build a yard, paying its cost there",
        ),
        Form::new(
            form::PRODUCE_PIONEER,
            vec![
                Term::Keyword("produce"),
                Term::Keyword("pioneer"),
                Term::required("territory", Kind::Number),
                Term::optional("repeat", Kind::Number),
            ],
            "produce a pioneer, paying its cost there",
        ),
        Form::new(
            form::PRODUCE_ARK,
            vec![
                Term::Keyword("produce"),
                Term::Keyword("ark"),
                Term::required("territory", Kind::Number),
                Term::optional("repeat", Kind::Number),
            ],
            "produce an ark at a yard",
        ),
        Form::new(
            form::CREATE_LABOR,
            vec![
                Term::Keyword("create"),
                Term::Keyword("labor"),
                Term::required("territory", Kind::Number),
                Term::optional("repeat", Kind::Number),
            ],
            "turn a ready citizen into labor",
        ),
        Form::new(
            form::WORK_EXTRACTOR,
            vec![
                Term::Keyword("work"),
                Term::Keyword("extractor"),
                Term::required("territory", Kind::Number),
                Term::required("resource", Kind::Name),
                Term::optional("repeat", Kind::Number),
            ],
            "spend labor at an extractor this turn",
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
                Term::required("id", Kind::Number),
            ],
            "what is in a territory, and what can be done there",
        ),
        Form::new(
            form::SHOW_PLANET,
            vec![Term::Keyword("show"), Term::Keyword("planet")],
            "every territory at a glance",
        ),
        Form::new(
            form::SHOW_ORBIT,
            vec![Term::Keyword("show"), Term::Keyword("orbit")],
            "what is in orbit",
        ),
        Form::new(
            form::SHOW_UNITS,
            vec![Term::Keyword("show"), Term::Keyword("units")],
            "every unit and where it is",
        ),
        Form::new(
            form::SHOW_TURN,
            vec![Term::Keyword("show"), Term::Keyword("turn")],
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
    ])
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
        let examples = [
            ("{deploy ark territory:1}", form::DEPLOY_ARK),
            ("{move pioneer territory:7}", form::MOVE_PIONEER),
            (
                "{build extractor territory:3 resource:metal}",
                form::BUILD_EXTRACTOR,
            ),
            ("{produce pioneer territory:11}", form::PRODUCE_PIONEER),
            (
                "{work extractor territory:3 resource:metal}",
                form::WORK_EXTRACTOR,
            ),
            ("{end turn}", form::END_TURN),
            ("{show territory id:5}", form::SHOW_TERRITORY),
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
            ("{create planet size:tiny}", form::CREATE_PLANET),
            (
                "{set resource territory:1 resource:food extractors:3 density:4}",
                form::SET_RESOURCE,
            ),
            ("{set force territory:1 force:1}", form::SET_FORCE),
            ("{set biome territory:1 biome:grassland}", form::SET_BIOME),
            ("{add ark orbit territory:1}", form::ADD_ARK),
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
        let one = parse_line(
            &grammar(),
            "{build extractor territory:3 resource:metal}",
            1,
        )
        .unwrap()
        .unwrap();
        let other = parse_line(
            &grammar(),
            "{build extractor resource:metal territory:3}",
            1,
        )
        .unwrap()
        .unwrap();
        assert_eq!(one.form, other.form);
        assert_eq!(one.number("territory").unwrap(), 3);
        assert_eq!(other.number("territory").unwrap(), 3);
        assert_eq!(one.name("resource").unwrap(), "metal");
        assert_eq!(other.name("resource").unwrap(), "metal");
    }

    /// A field the command does not take is named, rather than reported as surplus.
    ///
    /// **A positional grammar could only say *end of line*.** It had no name to give back,
    /// because the thing that was wrong had never had a name - which is what `P-321` changes.
    #[test]
    fn a_field_a_command_does_not_take_says_which_field_it_is() {
        let failure = parse_line(&grammar(), "{build yard territory:3 resource:metal}", 1)
            .expect_err("a yard is not built for a resource");
        let said = failure.to_string();
        assert!(
            said.contains("resource:"),
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
        assert_eq!(
            takes_repeat.len(),
            12,
            "twelve player recipes have a command; these take a repeat: {takes_repeat:?}"
        );
        for named in [
            form::END_TURN,
            form::SHOW_PLANET,
            form::RUN,
            form::CREATE_PLANET,
        ] {
            assert!(
                !takes_repeat.contains(&named),
                "`{named}` fires no recipe of the player's and must not take a repeat"
            );
        }
        let parsed = parse_line(&grammar(), "{create labor territory:1 repeat:2}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.optional_number("repeat"), Some(2));
        let once = parse_line(&grammar(), "{create labor territory:1}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(
            once.optional_number("repeat"),
            None,
            "a command without one fires once, and says so by carrying nothing"
        );
    }

    /// The ordering rule, checked over the whole grammar rather than shown by one pair.
    ///
    /// It used to be shown by `add node` against `add <unit> orbit`: two forms opening on
    /// the same word, one continuing with a keyword and the other with a hole, where only
    /// the keyword form's position kept `node` from being read as a unit's name. `P-149`
    /// deleted `add node`, and **no pair in the grammar collides today** - so an example
    /// would have been a test that demonstrates nothing while still passing.
    ///
    /// The rule outlives its example, so this checks the rule. For every pair of forms
    /// sharing an opening word, if one continues with a keyword and the other with a hole,
    /// the keyword one must come first. Adding a form that breaks it fails here, which is
    /// what the example could no longer do.
    #[test]
    fn a_keyword_form_comes_before_a_hole_that_would_swallow_it() {
        let grammar = grammar();
        let opens_with = |form: &Form| match form.terms.first() {
            Some(Term::Keyword(word)) => Some(*word),
            _ => None,
        };
        let second_is_keyword = |form: &Form| matches!(form.terms.get(1), Some(Term::Keyword(_)));
        let second_is_hole = |form: &Form| matches!(form.terms.get(1), Some(Term::Hole { .. }));

        let forms = grammar.forms();
        let mut sharing = 0usize;
        for (at, earlier) in forms.iter().enumerate() {
            for later in forms.iter().skip(at + 1) {
                if opens_with(earlier).is_none() || opens_with(earlier) != opens_with(later) {
                    continue;
                }
                sharing += 1;
                assert!(
                    !(second_is_hole(earlier) && second_is_keyword(later)),
                    "`{}` opens with a hole and is listed before `{}`, which opens with a keyword on the same word - first-wins matching would read that keyword as a value",
                    earlier.name,
                    later.name
                );
            }
        }
        // A grammar where no two forms share an opening word would pass without looking at
        // anything, which is the failure mode of every scanner.
        assert!(sharing >= 5, "only {sharing} pairs share an opening word");

        // **And the hazard the rule exists for is gone, which is worth showing rather than
        // asserting.** A hole used to swallow a keyword because a value was whatever word sat
        // in that position; `P-321` makes a field carry its own name, so `planet` in
        // `{show planet}` can only ever be a word of a name. Listed either way round, the
        // keyword form is the only one that matches.
        let general = Form::new(
            "general",
            vec![Term::Keyword("show"), Term::required("subject", Kind::Name)],
            "show something",
        );
        let specific = Form::new(
            "specific",
            vec![Term::Keyword("show"), Term::Keyword("planet")],
            "show the planet",
        );

        for order in [
            vec![general.clone(), specific.clone()],
            vec![specific.clone(), general.clone()],
        ] {
            let parsed = parse_line(&Grammar::new(order), "{show planet}", 1)
                .unwrap()
                .unwrap();
            assert_eq!(
                parsed.form, "specific",
                "a field is named, so nothing reads `planet` as a value"
            );
        }

        // The general form is still reachable, by writing the field it takes.
        let either = Grammar::new(vec![general, specific]);
        let parsed = parse_line(&either, "{show subject:planet}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.form, "general");
        assert_eq!(parsed.name("subject").unwrap(), "planet");
    }

    #[test]
    fn an_optional_resource_may_be_left_off() {
        let parsed = parse_line(&grammar(), "{build yard territory:11}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.optional_name("resource"), None);
        // A repeat is the optional field every player command carries, and one left off
        // means the command fires once.
        let parsed = parse_line(&grammar(), "{create labor territory:1 repeat:2}", 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.optional_number("repeat"), Some(2));
    }

    #[test]
    fn a_mistyped_command_is_told_what_was_expected_and_where() {
        let failure = parse_line(&grammar(), "{deploy ark territory:somewhere}", 1).unwrap_err();
        assert!(
            failure.expected.contains(&"a number".to_string()),
            "{failure}"
        );
        // At the value rather than at the field that carried it: `territory:` opens at 13
        // and `somewhere` at 23, and the value is what has to change.
        assert_eq!(failure.position.column, 23);
    }
}
