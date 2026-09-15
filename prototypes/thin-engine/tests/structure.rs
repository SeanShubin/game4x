//! The data structure, checked against the data that claims to fit it.
//!
//! **Sean typed the first test in relational notation to be clear about the structure**, so the
//! structure is as much under test as the move is. These are what declaring it buys.

use thin_engine::engine::run;
use thin_engine::schema::Malformed;

mod common;
use common::{before, with};

/// **The structure describes itself, and is checked against its own description.**
///
/// Every relation the data uses is declared by a `{relation ...}` row - including `relation`,
/// `column` and `reference` themselves. `Game::of` refuses a row of an undeclared relation, so
/// the file loading at all is most of this; what is asserted here is that the meta relations are
/// among them rather than exempt.
#[test]
fn the_relations_that_describe_the_structure_are_declared_like_any_other() {
    let game = before();
    let mut checked = 0;
    for name in [
        "relation",
        "column",
        "reference",
        "role",
        "rule",
        "input",
        "clause",
        "binding",
        "command",
        "argument",
    ] {
        let declared = game
            .schema()
            .relation(name)
            .unwrap_or_else(|| panic!("`{name}` is not declared, and it is used"));
        assert!(
            !declared.columns.is_empty(),
            "`{name}` is declared with no columns"
        );
        checked += 1;
    }
    assert_eq!(checked, 10, "ten relations describe the structure");
    assert_eq!(
        game.schema().names().len(),
        14,
        "fourteen relations in all - those ten, and the game's four"
    );
}

/// **A reference points at the relation its column names, and not at whichever has that key.**
///
/// `{residency what:1 where:1}` reads `1` twice and they mean different things - `what:1` is the
/// scout, `where:1` is territory 1. **Both are `1` in the first test**, so a check that only ever
/// saw the real data would pass with the two references crossed. These use values that exist in
/// one relation and not the other, which is what tells them apart.
#[test]
fn a_reference_is_checked_against_the_relation_it_names() {
    // Thing 2 does not exist; territory 2 does.
    assert_eq!(
        with("{residency what:2 where:2}").expect_err("there is no thing 2"),
        Malformed::NoSuchRow {
            relation: "residency".to_string(),
            column: "what".to_string(),
            value: "2".to_string(),
            to: "thing".to_string()
        },
        "`what` points at `thing`, and no thing has the key 2"
    );

    // Territory 9 does not exist; thing 1 does.
    assert_eq!(
        with("{residency what:1 where:9}").expect_err("there is no territory 9"),
        Malformed::NoSuchRow {
            relation: "residency".to_string(),
            column: "where".to_string(),
            value: "9".to_string(),
            to: "territory".to_string()
        },
        "`where` points at `territory`, and no territory has the key 9"
    );

    // The control: a row whose references both resolve is accepted, so the two above fail for
    // their own reason rather than because nothing added to this data is ever allowed.
    with("{residency what:1 where:2}").expect("a second residency resolves both ways");
}

/// **A row is exactly its relation's columns**, and data stating anything else is refused.
#[test]
fn a_row_states_only_what_its_relation_declares() {
    for wrong in [
        "{residency what:1}",
        "{residency what:1 where:1 when:now}",
        "{settlement place:1}",
    ] {
        with(wrong).expect_err(wrong);
    }
    with("").expect(
        "the entirety fits, which is what makes the three above fail for their own reasons",
    );
}

/// **An input is typed, and that is where *no such place* is caught.**
///
/// `{input id:move.to rule:move seq:3 name:to of:territory}` says the value is a `territory`'s
/// key. **So no rule has to say the destination exists** - the three-concept version of this
/// prototype needed a `{needs ... relation:territory id:$to}` row for exactly this, and a declared
/// structure does it once for every rule at once.
#[test]
fn a_command_naming_something_that_does_not_exist_is_refused_by_the_type() {
    let start = with(
        "{command id:3 rule:move}\n\
                      {argument id:3.what command:3 input:move.what value:1}\n\
                      {argument id:3.from command:3 input:move.from value:1}\n\
                      {argument id:3.to command:3 input:move.to value:9}",
    )
    .expect("a command naming territory 9 is still well formed data");

    let why = run(&start, "3").expect_err("there is no territory 9");
    assert_eq!(
        format!("{why}"),
        "`move`.`to` is `9`, and no `territory` has that key"
    );
}

/// **A thing is not a territory**, even where both have the key 1.
#[test]
fn an_input_is_checked_against_its_own_relation() {
    let start = with(
        "{command id:4 rule:move}\n\
                      {argument id:4.what command:4 input:move.what value:3}\n\
                      {argument id:4.from command:4 input:move.from value:1}\n\
                      {argument id:4.to command:4 input:move.to value:2}",
    )
    .expect("well formed data");

    // 3 is a territory and is not a thing, so `what` refuses it.
    let why = run(&start, "4").expect_err("there is no thing 3");
    assert_eq!(
        format!("{why}"),
        "`move`.`what` is `3`, and no `thing` has that key"
    );
}

/// **A command nothing states cannot be run**, because a command is data like everything else.
#[test]
fn a_command_that_is_not_stated_is_not_a_command() {
    let why = run(&before(), "99").expect_err("nothing states command 99");
    assert_eq!(format!("{why}"), "no command is stated with id `99`");
}
