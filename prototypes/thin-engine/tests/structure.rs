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
        "state",
        "role",
        "rule",
        "input",
        "clause",
        "binding",
        "literal",
        "command",
        "argument",
        "primitive",
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
    assert_eq!(checked, 13, "thirteen relations describe the structure");
    assert_eq!(
        game.schema().names().len(),
        17,
        "seventeen relations in all - those thirteen, and the game's four"
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
        with("{residency what:2 where:2 quantity:1}").expect_err("there is no thing 2"),
        Malformed::NoSuchRow {
            relation: "residency".to_string(),
            column: "what".to_string(),
            value: "2".to_string(),
            to: "thing".to_string()
        },
        "`what` points at `thing`, and no thing has the key 2"
    );

    // Territory 9 does not exist; thing 2 does now. **A second thing rather than a second
    // residency for thing 1**: `residency` is keyed by `what`, so reusing thing 1 is refused for
    // having a key already taken, and this test would pass on the wrong refusal.
    assert_eq!(
        with("{thing id:2 name:pioneer}\n{residency what:2 where:9 quantity:1}")
            .expect_err("there is no territory 9"),
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
    with("{thing id:3 name:runner}\n{residency what:3 where:2 quantity:1}")
        .expect("a second residency resolves both ways");
}

/// **A row is exactly its relation's columns**, and data stating anything else is refused.
#[test]
fn a_row_states_only_what_its_relation_declares() {
    for wrong in [
        "{residency what:1 where:1}",
        "{residency what:1 where:1 quantity:1 when:now}",
        "{settlement id:1 place:1}",
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
        "{command id:3 rule:1}\n\
         {argument id:11 command:3 input:1 value:1}\n\
         {argument id:12 command:3 input:2 value:1}\n\
         {argument id:13 command:3 input:3 value:9}",
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
        "{command id:4 rule:1}\n\
         {argument id:21 command:4 input:1 value:3}\n\
         {argument id:22 command:4 input:2 value:1}\n\
         {argument id:23 command:4 input:3 value:2}",
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

/// **The files the helpers load are the files `data/test.4x` loads**, and neither list is derived
/// from the other.
///
/// `common::LOADED` names four files so that these tests can assemble a game without running the
/// script; `test.4x` names them so the script can. **Two lists that must agree and nothing making
/// them** - so this is what would notice, rather than a fifth file being loaded by the script and
/// silently missing from every test here.
#[test]
fn the_helper_loads_what_the_script_loads() {
    // `into` names a store by id, and `script.4x` says which is which.
    let game_store = common::rows("data/foundation/script.4x")
        .iter()
        .find(|row| row.relation == "store" && row.value("name") == Some("game"))
        .and_then(|row| row.value("id").map(str::to_string))
        .expect("a store named `game`");
    let script: Vec<String> = common::rows("data/foundation/test.4x")
        .iter()
        .filter(|row| row.relation == "load" && row.value("into") == Some(game_store.as_str()))
        .filter_map(|row| row.value("file").map(|it| format!("data/foundation/{it}")))
        .collect();

    assert_eq!(script.len(), 5, "the script loads five files into the game");
    assert_eq!(
        script,
        common::LOADED.to_vec(),
        "and they are the five the helpers here assemble, in the same order"
    );
}

/// **A key names one row.** Two rows of a relation with the same key is a reference that names
/// neither, and nothing checked it until it was looked for.
#[test]
fn two_rows_of_one_relation_cannot_share_a_key() {
    assert_eq!(
        with("{thing id:1 name:pioneer}").expect_err("thing 1 is taken"),
        Malformed::TwoWithOneKey {
            relation: "thing".to_string(),
            key: vec![("id".to_string(), "1".to_string())]
        },
        "`{{residency what:1}}` would otherwise point at two things"
    );

    // The control: a thing with a key of its own is fine, so the refusal above is about the key
    // rather than about adding a thing at all.
    with("{thing id:2 name:pioneer}").expect("a thing with its own key");
}

/// **A description names one row, and a quantity is what made that matter.**
///
/// Two residencies for the same thing in the same place used to be legal - the surrogate `id` told
/// them apart - and merely *redundant*, because the store is a set and identical rows collapse.
/// **A quantity is the column that breaks that**: `-> 2` and `-> 3` are not identical, so the set
/// keeps both and the world says two things at once.
///
/// **This is the check that would have passed before**, which is the only reason it is worth
/// having. `{residency id:1 what:1 where:1 quantity:2}` beside `{residency id:4 what:1 where:1
/// quantity:3}` loaded clean, and nothing could say whether there were two scouts, three, or five.
#[test]
fn two_residencies_of_one_description_are_refused() {
    assert_eq!(
        with("{residency what:1 where:1 quantity:3}")
            .expect_err("scouts are already stated to be in territory 1"),
        Malformed::TwoWithOneKey {
            relation: "residency".to_string(),
            key: vec![
                ("what".to_string(), "1".to_string()),
                ("where".to_string(), "1".to_string())
            ]
        },
        "one scout, or three, or four - nothing could say"
    );

    // The control: the same description somewhere else is a different description, so the refusal
    // above is about the key and not about adding a residency at all.
    with("{residency what:1 where:2 quantity:3}").expect("another place is another description");
}

/// **Identified or counted, and never both.**
///
/// Sean, 2026-09-15: *it would make no sense to have both an id and a quantity in the same logical
/// model.* They are one slot - whether a row is one thing or a count of them - so a relation
/// carrying both says it is each at once. **Checked where the schema is read**, so it is refused
/// before any row of that relation is looked at.
#[test]
fn a_relation_cannot_carry_both_an_id_and_a_quantity() {
    assert_eq!(
        with("{column id:90 relation:16 seq:4 name:id}")
            .expect_err("a row is one thing or a count of them"),
        Malformed::IdAndQuantity {
            relation: "residency".to_string()
        }
    );

    // **The controls are a relation of their own**, because adding a column to one that has rows
    // makes every one of them stop fitting, and `WrongColumns` would then be the refusal whatever
    // the columns were called. A relation with no rows isolates the pair.
    let pile = "{relation id:90 name:pile}\n{column id:90 relation:90 seq:1 name:quantity}";
    with(pile).expect("counted alone is fine");
    with("{relation id:90 name:pile}\n{column id:90 relation:90 seq:1 name:id}")
        .expect("identified alone is fine");
    assert_eq!(
        with(&format!(
            "{pile}\n{{column id:91 relation:90 seq:2 name:id}}"
        ))
        .expect_err("and the two together are not"),
        Malformed::IdAndQuantity {
            relation: "pile".to_string()
        }
    );
}
