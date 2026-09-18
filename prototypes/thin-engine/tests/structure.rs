//! The data structure, checked against the data that claims to fit it.
//!
//! **Sean typed the first test in relational notation to be clear about the structure**, so the
//! structure is as much under test as the move is. These are what declaring it buys.

use thin_engine::engine::fire;
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
        "reading",
        "limit",
        "attribute",
        "relation-of",
        "family",
        "member",
        "supply",
        "provides",
        "consumes",
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
    assert_eq!(checked, 20, "twenty relations describe the structure");
    assert_eq!(
        game.schema().names().len(),
        32,
        "thirty-two in all - those twenty, and the game's twelve: five kinds, two families,\n         a territory, an adjacency, a deposit and an extractor"
    );
}

/// **A reference points at the relation its column names, and not at whichever has that key.**
///
/// `{extractor where:1 what:31}` reads two ids that mean different things - `where:1` is
/// territory 1 and `what:31` is the relation `food`. **A check that only ever saw the real data
/// could pass with the two references crossed**, so these use values that exist in one relation
/// and not the other.
#[test]
fn a_reference_is_checked_against_the_relation_it_names() {
    // **There is no relation 99; territory 1 there is.** `extractor.what` points at the family
    // `resource`, so what it admits is a member of it rather than any row that happens to have
    // the key.
    assert_eq!(
        with("{extractor where:1 what:99 quantity:1}").expect_err("there is no resource 99"),
        Malformed::NoSuchRow {
            relation: "extractor".to_string(),
            column: "what".to_string(),
            value: "99".to_string(),
            to: "resource".to_string()
        },
        "`what` points at the family `resource`, and 99 belongs to nothing"
    );

    // **Territory 9 does not exist, and the row is a scout rather than a second extractor**,
    // because reusing a description already stated would be refused for having a key already
    // taken - and this test would then pass on the wrong refusal.
    assert_eq!(
        with("{scout where:9 quantity:1}").expect_err("there is no territory 9"),
        Malformed::NoSuchRow {
            relation: "scout".to_string(),
            column: "where".to_string(),
            value: "9".to_string(),
            to: "territory".to_string()
        },
        "`where` points at `territory`, and no territory has the key 9"
    );

    // The control: a row whose references both resolve is accepted, so the two above fail for
    // their own reason rather than because nothing added to this data is ever allowed.
    with("{deposit where:2 what:31 density:6 quantity:1}\n{extractor where:2 what:31 quantity:1}")
        .expect("a resource in a real territory, with a deposit to stand in");
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
    let command = thin_engine::notation::read("{move what:28 from:1 to:9}").expect("a command");
    let why = fire(&before(), &command[0], 1)
        .map(|(game, _)| game)
        .expect_err("there is no territory 9");
    assert_eq!(
        format!("{why}"),
        "`move`.`to` is `9`, and no `territory` has that key"
    );
}

/// **A thing is not a territory**, even where both have the key 1.
#[test]
fn an_input_is_checked_against_its_own_relation() {
    // **Territory 9 is a territory and is not a unit, so `what` refuses it.** It is added here
    // rather than taken from `before()`, because a value that is nothing at all would refuse for
    // a weaker reason than a value that is something else.
    let game = with("{territory id:9}").expect("a ninth territory");
    let command = thin_engine::notation::read("{move what:9 from:1 to:2}").expect("a command");
    let why = fire(&game, &command[0], 1)
        .map(|(game, _)| game)
        .expect_err("territory 9 is no unit");
    assert_eq!(
        format!("{why}"),
        "`move`.`what` is `9`, and no `unit` has that key"
    );
}

/// **A command names its rule, and a rule nothing declares is not a command.**
///
/// **This used to be about a `{command id:99}` nobody stated.** There is no `command` relation any
/// more - a command is a row of its own rule - so the way to write one nothing can fire is to name
/// a rule that does not exist.
#[test]
fn a_command_that_is_not_stated_is_not_a_command() {
    let command = thin_engine::notation::read("{fly what:1 from:1 to:2}").expect("a command");
    let why = fire(&before(), &command[0], 1)
        .map(|(game, _)| game)
        .expect_err("nothing declares a rule `fly`");
    assert_eq!(format!("{why}"), "no command is stated with id `fly`");
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
    let script: Vec<String> = common::rows("data/foundation/setup.4x")
        .iter()
        .filter(|row| row.relation == "load" && row.value("into") == Some(game_store.as_str()))
        .filter_map(|row| row.value("file").map(|it| format!("data/foundation/{it}")))
        .collect();

    assert_eq!(
        script.len(),
        3,
        "the script loads three files into the game"
    );
    assert_eq!(
        script,
        common::LOADED.to_vec(),
        "and they are the three the helpers here assemble, in the same order"
    );
}

/// **What the player may fire is derived from the world, not listed anywhere.**
///
/// `spec/invariants.md`, of a recipe: *The player's are offered wherever their inputs are present,
/// to take or to leave.* And of a choice: *What may be chosen is whatever the game holds, and the
/// offering is derived rather than listed.*
///
/// **Sean, 2026-09-15**, on why this matters more than a reference being tidy: *we won't want
/// executing an invalid command to even be possible in the user interface.*
///
/// **Checked against the world rather than against a number.** The scout is in territory 1, which
/// is next to territory 2 and to nothing else - so exactly one move is offered, and the check that
/// it is *one* is what makes the two controls below mean anything.
#[test]
fn only_the_moves_the_world_allows_are_offered() {
    let game = before();
    let offered: Vec<String> = thin_engine::engine::offered(&game)
        .iter()
        .map(thin_engine::notation::write)
        .collect();

    assert_eq!(offered, vec!["{move from:1 to:2 what:28}"]);

    // **The candidates it chose between**, so a single answer is not a walk that tried one thing.
    // Three territories to leave, three to enter, one thing to move: 9 bindings, 1 legal.
    let territories = game
        .rows()
        .rows()
        .iter()
        .filter(|row| row.relation == "territory")
        .count();
    assert_eq!(
        territories, 3,
        "three territories, so nine bindings were tried"
    );

    // **The control, and it found something.** With the scout in territory 2 it can reach 3 and
    // **cannot go back to 1**: `adjacency` is stated one way - `{from:1 to:2}`, `{from:2 to:3}` -
    // and `move`'s second clause requires a row in exactly that direction. **The world is a
    // one-way corridor and no test could see it**, because the only move in the scenario runs
    // downhill.
    //
    // **The game does not have this**: `crates/game-model/src/game.rs` holds adjacency as
    // `Vec<Vec<TerritoryId>>` and its own comment says *Symmetric*, so both directions are stored
    // and the dump halves them for writing. **The prototype kept the halved form and lost the
    // symmetry with it.**
    //
    // **Asserted as it is rather than as it should be.** Which way to fix it - state both
    // directions, or read the one that is stated from either end - is a modelling decision and
    // not this test's. This line goes red when it is made, which is what it is for.
    let command = thin_engine::notation::read("{move what:28 from:1 to:2}").expect("a command");
    let (moved, _) = fire(&game, &command[0], 1).expect("the scout moves");
    let after: Vec<String> = thin_engine::engine::offered(&moved)
        .iter()
        .map(thin_engine::notation::write)
        .collect();
    assert_eq!(
        after,
        vec!["{move from:2 to:3 what:28}"],
        "one way only, which is the finding rather than the intent"
    );
}

/// **A key names one row.** Two rows of a relation with the same key is a reference that names
/// neither, and nothing checked it until it was looked for.
#[test]
fn two_rows_of_one_relation_cannot_share_a_key() {
    assert_eq!(
        with("{territory id:1}").expect_err("territory 1 is taken"),
        Malformed::TwoWithOneKey {
            relation: "territory".to_string(),
            key: vec![("id".to_string(), "1".to_string())]
        },
        "`{{scout where:1}}` would otherwise point at two territories"
    );

    // The control: a territory with a key of its own is fine, so the refusal above is about the
    // key rather than about adding a territory at all.
    with("{territory id:9}").expect("a territory with its own key");
}

/// **A description names one row, and a quantity is what made that matter.**
///
/// Two scouts in one place used to be legal - a surrogate `id` told them apart - and merely
/// *redundant*, because the store is a set and identical rows collapse. **A quantity is the column
/// that breaks that**: `-> 2` and `-> 3` are not identical, so the set keeps both and the world
/// says two things at once.
///
/// **This is the check that would have passed before**, which is the only reason it is worth
/// having. Two rows saying how many scouts stand in territory 1 loaded clean, and nothing could
/// say whether there were two, three, or five.
///
/// **The relation it is asked of moved and the question did not.** It was `residency`, keyed by
/// `(what, where)`; a kind is a relation now, so it is `scout`, keyed by `where` alone.
#[test]
fn two_scouts_of_one_description_are_refused() {
    assert_eq!(
        with("{scout where:1 quantity:3}")
            .expect_err("scouts are already stated to be in territory 1"),
        Malformed::TwoWithOneKey {
            relation: "scout".to_string(),
            key: vec![("where".to_string(), "1".to_string())]
        },
        "one scout, or three, or four - nothing could say"
    );

    // The control: the same description somewhere else is a different description, so the refusal
    // above is about the key and not about adding a scout at all.
    with("{scout where:2 quantity:3}").expect("another place is another description");
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
        with("{column id:900 relation:28 seq:3 name:id}")
            .expect_err("a row is one thing or a count of them"),
        Malformed::IdAndQuantity {
            relation: "scout".to_string()
        }
    );

    // **The controls are a relation of their own**, because adding a column to one that has rows
    // makes every one of them stop fitting, and `WrongColumns` would then be the refusal whatever
    // the columns were called. A relation with no rows isolates the pair.
    let pile = "{relation id:990 name:pile}\n{column id:990 relation:990 seq:1 name:quantity}";
    with(pile).expect("counted alone is fine");
    with("{relation id:990 name:pile}\n{column id:990 relation:990 seq:1 name:id}")
        .expect("identified alone is fine");
    assert_eq!(
        with(&format!(
            "{pile}\n{{column id:991 relation:990 seq:2 name:id}}"
        ))
        .expect_err("and the two together are not"),
        Malformed::IdAndQuantity {
            relation: "pile".to_string()
        }
    );
}

/// **A territory has one density per resource, and stating a second is refused.**
///
/// **This is what marking `density` an attribute buys, and the only thing it buys.** Sean,
/// 2026-09-17, on wanting three things and being able to have two - specify nothing extra where
/// there is one choice, allow more than one choice, and keep the notation uniform: *we are
/// dropping (2)*. **Dropping it only means anything if the second grade is refused**, and with
/// density in the key it would be a legal state nothing could name.
///
/// `spec/planet.md` is what says one: *For each resource, a territory has capacity for some number
/// of extractors, and a density that each of them yields.*
#[test]
fn a_deposit_cannot_have_two_densities() {
    assert_eq!(
        with("{deposit where:1 what:31 density:6 quantity:3}\n{deposit where:1 what:31 density:9 quantity:1}")
            .expect_err("two grades of one resource in one territory"),
        Malformed::TwoWithOneKey {
            relation: "deposit".to_string(),
            key: vec![
                ("where".to_string(), "1".to_string()),
                ("what".to_string(), "31".to_string())
            ]
        },
        "density is a fact about the deposit, so it is not what tells two deposits apart"
    );

    // The control: two deposits of different resources in one territory are two descriptions, so
    // the refusal above is about the key and not about stating two deposits at all.
    with("{deposit where:1 what:31 density:6 quantity:3}\n{deposit where:1 what:30 density:9 quantity:1}")
        .expect("food and metal are different deposits");
}

/// **An extractor with no deposit under it is refused when the world is read**, not when something
/// tries to use it.
///
/// **Sean, 2026-09-17**: *the situation should be detectible and therefore preventable.* This is
/// the detectable half, and prevention is the same check running after a rule - which is why no
/// rule mentions capacity.
#[test]
fn an_extractor_needs_a_deposit_to_stand_in() {
    assert_eq!(
        with("{extractor where:1 what:31 quantity:1}").expect_err("no deposit of food here"),
        Malformed::Overfull {
            held: "extractor".to_string(),
            by: "deposit".to_string(),
            wanted: "{deposit where:1 what:31 quantity:1}".to_string(),
            room: "0".to_string()
        },
        "the refusal names the deposit that would have had to be there"
    );

    // The control: the same extractor over a deposit with room is fine, so the refusal is about
    // the room rather than about extractors.
    with("{deposit where:1 what:31 density:6 quantity:1}\n{extractor where:1 what:31 quantity:1}")
        .expect("one extractor in one deposit");

    // And one more than there is room for is refused by the number, not by the absence.
    assert_eq!(
        with(
            "{deposit where:1 what:31 density:6 quantity:1}\n{extractor where:1 what:31 quantity:2}"
        )
        .expect_err("two extractors in one deposit"),
        Malformed::Overfull {
            held: "extractor".to_string(),
            by: "deposit".to_string(),
            wanted: "{deposit where:1 what:31 quantity:2}".to_string(),
            room: "1".to_string()
        },
        "a full deposit and an absent one are one refusal with a different number"
    );
}

/// **An allowance cannot exceed the things that have it.**
///
/// **`{limit held:working by:extractor}` is what says so**, and without this test the row
/// declaring it could be deleted and nothing would notice - which the mutation check said the day
/// the limit was added. **A limit is only worth declaring where something would otherwise be
/// allowed**, and two works for one extractor is that something.
#[test]
fn an_allowance_cannot_exceed_the_things_that_have_it() {
    assert_eq!(
        with(
            "{deposit where:1 what:31 density:6 quantity:1}
{extractor where:1 what:31 quantity:1}
{working where:1 what:31 quantity:2}"
        )
        .expect_err("two works and one extractor to do them"),
        Malformed::Overfull {
            held: "working".to_string(),
            by: "extractor".to_string(),
            wanted: "{extractor where:1 what:31 quantity:2}".to_string(),
            room: "1".to_string()
        },
        "the refusal names the extractor that would have had to be there"
    );

    // The control: as many works as there are extractors is fine, so the refusal above is about
    // the number rather than about stating a readiness at all.
    with(
        "{deposit where:1 what:31 density:6 quantity:1}
{extractor where:1 what:31 quantity:1}
{working where:1 what:31 quantity:1}",
    )
    .expect("one work for one extractor");
}
