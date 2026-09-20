//! The data structure, checked against the data that claims to fit it.
//!
//! **Sean typed the first test in relational notation to be clear about the structure**, so the
//! structure is as much under test as the move is. These are what declaring it buys.

use thin_engine::engine::fire;
use thin_engine::refusal::Refused;
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
        "trait",
        "carries",
        "part",
        "argument",
        "assigns",
        "capacity",
        "loose",
        "repeats",
        "scope",
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
    assert_eq!(checked, 29, "twenty-nine relations describe the structure");
    assert_eq!(
        game.schema().names().len(),
        42,
        "forty-two in all - those twenty-nine, and the game's thirteen: six kinds, two families,\n         a territory, an adjacency, a deposit, an extractor and a citizen"
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
        with("{extractor where:1 what:99 working:0 quantity:1}")
            .expect_err("there is no resource 99"),
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
        with("{scout where:9 moving:1 quantity:1}").expect_err("there is no territory 9"),
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
    with("{deposit where:2 what:31 density:6 quantity:1}\n{extractor where:2 what:31 working:0 quantity:1}")
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

    assert_eq!(
        offered,
        vec![
            // **Two offers, and twelve refreshes are not among them.** `refresh` is a part of
            // `end-turn`, so it is fired by `end-turn` rather than chosen by a player - and the
            // tree is the whole of what says so. **`fire` still takes it by name**, which is how
            // `refresh-makes-one-entry-of-a-spent-scout-and-a-fresh-one` tests the part on its
            // own. Sean, 2026-09-18: *it will be easier to test the end turn command itself if i
            // can test its parts.*
            "{end-turn}",
            "{move from:1 to:2 what:28}",
        ]
    );

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
    // **The scout is refreshed before asking**, because a scout that has moved has no move left
    // and nothing would be offered at all - which would hide the one-way corridor behind a spent
    // allowance rather than show it.
    let mut moved = game;
    for step in ["{move what:28 from:1 to:2}", "{end-turn}"] {
        let command = thin_engine::notation::read(step).expect("a command");
        moved = fire(&moved, &command[0], 1)
            .expect("the scout moves and is refreshed")
            .0;
    }
    let after: Vec<String> = thin_engine::engine::offered(&moved)
        .iter()
        .map(thin_engine::notation::write)
        .collect();
    assert_eq!(
        after,
        vec!["{end-turn}", "{move from:2 to:3 what:28}",],
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
        with("{scout where:1 moving:1 quantity:3}")
            .expect_err("scouts with a move are already stated to be in territory 1"),
        Malformed::TwoWithOneKey {
            relation: "scout".to_string(),
            key: vec![
                ("where".to_string(), "1".to_string()),
                ("moving".to_string(), "1".to_string())
            ]
        },
        "one scout, or three, or four - nothing could say"
    );

    // The control: the same description somewhere else is a different description, so the refusal
    // above is about the key and not about adding a scout at all.
    with("{scout where:2 moving:1 quantity:3}").expect("another place is another description");

    // And the allowance tells them apart in the same place, which is what makes the grouping
    // work: a scout that has moved is not the same description as one that has not.
    with("{scout where:1 moving:0 quantity:3}").expect("a spent scout is another description");
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
        with("{column id:900 relation:28 seq:4 name:id}")
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
        with("{extractor where:1 what:31 working:0 quantity:1}")
            .expect_err("no deposit of food here"),
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
    with("{deposit where:1 what:31 density:6 quantity:1}\n{extractor where:1 what:31 working:0 quantity:1}")
        .expect("one extractor in one deposit");

    // And one more than there is room for is refused by the number, not by the absence.
    assert_eq!(
        with(
            "{deposit where:1 what:31 density:6 quantity:1}\n{extractor where:1 what:31 working:0 quantity:2}"
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

/// **A relation with no id is keyed by its whole row, so a kind may belong to two families.**
///
/// **It could not before, and that was an artefact rather than a decision.** `key()` said the
/// first column wherever there was no quantity, so `member` was keyed by `kind` - and this lane
/// reported that as a property of the model. Sean, 2026-09-18: *Why not? Many languages have
/// multiple inheritance. Some languages have multiple trait inheritance. Some languages have duck
/// typing.*
///
/// **`limit` was the second casualty and nobody had noticed**: keyed by `held`, a thing could be
/// held by one container, which is a restriction no one chose either.
#[test]
fn a_kind_can_belong_to_two_families() {
    // **`30` is `metal`, already a `resource`, and `29` is `labor` made a family for this.**
    //
    // **It used to put a scout in `resource` and cannot any more**, which is a later rule
    // arriving rather than this one weakening: `{loose kind:resource}` makes every resource
    // loose, and a scout is told apart by `moving`, so it would be two numbers in one place.
    // `what_lies_loose_is_one_number` refuses that and is right to. **Metal is loose already and
    // keyed by `where` alone**, so nothing about this membership is in question.
    let extra = "{family relation:29}\n{member kind:30 family:29}";
    with(extra).expect("metal may also be a labor, however odd");

    // **The second membership is a different row and the first still stands**, which is what
    // being keyed by the pair means rather than by the kind.
    let game = with(extra).expect("two memberships");
    let how_many = game
        .rows()
        .rows()
        .iter()
        .filter(|row| row.relation == "member" && row.value("kind") == Some("30"))
        .count();
    assert_eq!(how_many, 2, "metal is a resource and a labor, in two rows");
}

/// **What the key rule was right about is kept right by marking a column.**
///
/// A column is an attribute of one relation, and a clause takes its relation from one input. Both
/// were free when the key was the first column; both are `{attribute ...}` rows now, which is the
/// same marker that keeps a deposit's density out of its key.
#[test]
fn a_column_is_an_attribute_of_one_relation() {
    assert_eq!(
        with("{attribute column:53 relation:19}")
            .expect_err("column 53 is already marked, and it belongs to `deposit`"),
        Malformed::TwoWithOneKey {
            relation: "attribute".to_string(),
            key: vec![("column".to_string(), "53".to_string())]
        },
        "`attribute.relation` is marked an attribute, so the key is the column alone"
    );

    // The control: marking a different column is fine, so the refusal is about the key rather
    // than about adding an attribute at all.
    //
    // **Not just any column, and the first attempt found out why.** Marking `extractor.what`
    // takes it out of the extractor's key, and the deposit limit then has nothing to compare -
    // `CannotLimit`. So the control marks a column of a relation keyed by an id, where the key
    // does not move.
    with("{attribute column:42 relation:15}").expect("a column of an identified relation");
}

/// **A kind is refreshed for a trait it carries, and refused for one it does not.**
///
/// **This is the pairing a type cannot state.** `refresh`'s `what` is typed as a relation and its
/// `trait` as a trait, so `{refresh what:extractor trait:moving}` is well typed in both arguments
/// and wrong in their combination - which is why it is refused when it fires rather than when it
/// is read. **`offered` shows only the pairs that would not be refused**, which is where a player
/// sees the difference.
#[test]
fn a_kind_is_refreshed_only_for_a_trait_it_carries() {
    let game = before();
    let fired = |text: &str| {
        let command = thin_engine::notation::read(text).expect("a command");
        fire(&game, &command[0], 1)
    };

    // **19 is `extractor` and 1 is `moving`.** An extractor has a `working` and no `moving`, so
    // there is no column for the put to assign and nothing to restore.
    assert_eq!(
        fired("{refresh where:1 what:19 trait:1}")
            .expect_err("an extractor has no move to give back"),
        Refused::DoesNotCarry {
            rule: "refresh".to_string(),
            relation: "extractor".to_string(),
            carried: "moving".to_string()
        }
    );

    // **The control, and it is the same command with the trait it does carry** - so the refusal
    // is about the pair rather than about extractors, or about `refresh` reaching them at all.
    fired("{refresh where:1 what:19 trait:2}").expect("an extractor carries `working`");

    // **And the other way round**, so neither half is the one doing all the work: a scout carries
    // `moving` and not `working`. **28 is `scout` and 2 is `working`.**
    assert_eq!(
        fired("{refresh where:1 what:28 trait:2}").expect_err("a scout does no work"),
        Refused::DoesNotCarry {
            rule: "refresh".to_string(),
            relation: "scout".to_string(),
            carried: "working".to_string()
        }
    );
}

/// **A trait and the column that holds it are checked in both directions.**
///
/// `{carries kind:scout trait:moving}` and `{column ... relation:scout name:moving}` are two
/// statements of one fact, and that is the shape that drifts. **Neither direction implies the
/// other**: without the first check a `carries` row could name a column nothing declares, and
/// without the second a column could hold an allowance no rule can reach - because `refresh`
/// finds a kind through `carries` rather than through its columns.
///
/// **Sean, 2026-09-18**, on why the duplication is allowed to stand at all: *One reason I resist
/// duplication is to guard against the inconsistency. Another reason is to keep the model simple.
/// Inconsistency can be mitigated by automated checks. Simplicity is more important from the
/// expression side that I audit than it is for the implementation details.*
#[test]
fn a_trait_and_the_column_that_holds_it_are_checked_both_ways() {
    // **13 is `territory` and 1 is `moving`.** A territory is one column, `id`, so a row saying
    // it carries a move names a place to keep one that does not exist.
    assert_eq!(
        with("{carries kind:13 trait:1}").expect_err("a territory has nowhere to keep a move"),
        Malformed::CarriesNothing {
            relation: "territory".to_string(),
            carried: "moving".to_string()
        }
    );

    // **The other direction, and `density` is the column to name it with** - `deposit` is the
    // only relation that declares one, so which relation the refusal names is not a question of
    // what order the structure happens to be walked in.
    assert_eq!(
        with("{trait id:990 name:density}").expect_err("a deposit does not spend its density"),
        Malformed::DoesNotCarry {
            relation: "deposit".to_string(),
            carried: "density".to_string()
        }
    );

    // **The control for the second, and it is the same row with a name nothing declares.** So
    // the refusal is about a trait meeting a column of that name, and not about a trait being
    // added - a trait nothing carries yet is how a new one would arrive.
    with("{trait id:991 name:dashing}").expect("a trait no relation declares a column for");

    // **The control for the first is the game**: five `{carries ...}` rows, each naming a column
    // its kind declares, and `before()` loading at all is what says a consistent pair is taken.
    assert_eq!(
        before()
            .rows()
            .rows()
            .iter()
            .filter(|row| row.relation == "carries")
            .count(),
        5,
        "unit, scout and transport carry `moving`; extractor carries `working`; citizen carries `hunger`"
    );
}

/// **A repetition draws from a pool that only shrinks, and this is the rule with no pool.**
///
/// **It is the one bound the engine does not carry itself.** A composite cannot reach itself
/// because `CycleOfParts` refuses it; a repetition cannot run forever because every firing takes
/// something out of a world that began finite - and a rule that takes nothing out has no such
/// argument to make.
#[test]
fn a_rule_that_repeats_and_removes_nothing_is_refused() {
    // **`4` is `refresh`**, whose one clause is a `put`: it assigns and takes nothing away, so it
    // would be able to fire again every time it fired.
    assert_eq!(
        with("{repeats rule:4}").expect_err("refresh consumes nothing"),
        Malformed::NeverStops {
            rule: "refresh".to_string()
        }
    );

    // **`10` is `adjust-population`**, a composite, which has no clauses of its own - so what it
    // consumes is its parts' business and a repetition of it would be reasoning about nothing.
    assert_eq!(
        with("{repeats rule:10}").expect_err("a composite removes nothing itself"),
        Malformed::NeverStops {
            rule: "adjust-population".to_string()
        }
    );

    // **The control is the game**: `upkeep` and `perish` both repeat and both remove, and
    // `before()` loading at all is what says a repetition with a pool is taken.
    assert_eq!(
        before()
            .rows()
            .rows()
            .iter()
            .filter(|row| row.relation == "repeats")
            .count(),
        2,
        "`upkeep` and `perish` are what repeat"
    );
}

/// **The rules are a tree, and the three ways they could stop being one are refused.**
///
/// Sean, 2026-09-18: *it must be able to organize the entirety of game rules is some type of
/// acyclic graph or tree. Otherwise it will be impossible for a human player to understand how to
/// play the game.* **This is also what makes the engine safe to recurse** - `run` walks parts with
/// no depth counter, because a cycle cannot be in a world that loaded.
///
/// **Ids in the 990s**, so that a poison never squats on one the schema grows into.
#[test]
fn the_rules_are_a_tree() {
    // **`4` is `refresh` and `1` is `move`.** Refresh is already a part of `end-turn`, so a part
    // of `move` naming it gives it a second parent.
    assert_eq!(
        with("{part id:990 of:1 is:4 seq:1}").expect_err("refresh belongs to end-turn"),
        Malformed::TwoParents {
            rule: "refresh".to_string(),
            // **In the order the parts are stated**, which is `end-turn`'s two rows and then the
            // poison's - so the message reads as the file does.
            parents: vec!["end-turn".to_string(), "move".to_string()]
        }
    );

    // **`5` is `end-turn`.** A part of refresh naming end-turn closes the loop, and the walk
    // upwards repeats rather than running forever.
    assert_eq!(
        with("{part id:991 of:4 is:5 seq:1}").expect_err("end-turn would reach itself"),
        Malformed::CycleOfParts {
            rules: vec!["end-turn".to_string(), "refresh".to_string()]
        }
    );

    // **A rule is a leaf or a composite.** `refresh` has clauses, so giving it a part as well
    // leaves *what does this rule do* with two answers and no order between them.
    assert_eq!(
        with("{part id:992 of:4 is:2 seq:1}").expect_err("refresh has clauses of its own"),
        Malformed::BothLeafAndComposite {
            rule: "refresh".to_string()
        }
    );

    // **The control, and it is a fourth part of the composite that already has two.** So none of
    // the three above is about adding a `{part ...}` row; each is about the shape it would make.
    // **Its arguments are missing**, which is a refusal when it fires and not when it is read -
    // the structure says what the tree is, and `run` says what a part is handed.
    with("{part id:993 of:5 is:4 seq:3}").expect("a composite may name a rule again");
}

/// **The tree is written down, so a rule added to it has to be read by somebody.**
///
/// **Sean, 2026-09-18**, on what went wrong with the specification: it *had no artifact whose whole
/// structure you could read at once*. `tree.txt` is that artifact here, and this is what makes it
/// impossible to change the shape of the rules without the change showing up in a diff of it.
///
/// **`cargo run --example tree` regenerates it.** The failure says to do that, because a test that
/// fails without saying what to do is a test somebody deletes.
#[test]
fn the_tree_is_what_the_file_says_it_is() {
    let shown = before().tree();
    let at = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tree.txt");
    let written = std::fs::read_to_string(&at).unwrap_or_default();
    assert_eq!(
        written, shown,
        "`tree.txt` is not what the rules render to - run `cargo run --example tree` and read it"
    );

    // **Every rule appears exactly once**, which is what makes it the whole of the rules rather
    // than a view of some of them. A rule missing from the tree is a rule nobody can find.
    let rules: Vec<String> = before()
        .rows()
        .rows()
        .iter()
        .filter(|row| row.relation == "rule")
        .filter_map(|row| row.value("name"))
        .map(str::to_string)
        .collect();
    assert_eq!(
        rules.len(),
        10,
        "move, build-extractor, work, refresh, end-turn, build-bin, lose-what-is-not-kept,
         upkeep, perish, adjust-population"
    );
    // **At least once, not exactly once.** `refresh` appears twice because `end-turn` names it
    // twice - two steps of one order - and asserting *once* said the tree was wrong when it was
    // the assertion that was. What matters is that no rule is missing: a rule absent from the
    // tree is a rule nobody reading this can find.
    for rule in &rules {
        let found = shown.matches(&format!("{rule} ")).count()
            + shown.matches(&format!("{rule}\n")).count();
        assert!(found >= 1, "`{rule}` is nowhere in the tree");
    }
    assert_eq!(
        shown.matches("refresh").count(),
        3,
        "and refresh is there three times, once per trait the turn restores"
    );

    // **And a step the engine fans out says so.** `upkeep` and `perish` are handed nothing, so
    // without this the turn would read as though each happened once - which is the one thing a
    // person reading it would get wrong.
    assert_eq!(
        shown.matches("per:territory").count(),
        2,
        "`upkeep` and `perish` each happen once per territory"
    );
}

/// **A family named where a member belongs is each member, and named twice it is the same one.**
///
/// Sean, 2026-09-18: *if we declared resource = [food, metal, energy], we could have bin[resource]
/// and transport[resource], which would need to be reified to a leaf resource by some mechanic.*
/// **This is that mechanic, and it is substitution**: bounded by the family, unable to recurse,
/// and leaving plain rows for every check downstream.
///
/// **27 is `resource`, 30 is `metal`, 31 is `food`, 42 is `bin` and 13 is `territory`.**
#[test]
fn a_family_named_where_a_member_belongs_is_each_member() {
    let game = with("{capacity of:42 for:27 what:27 per:13 quantity:10}")
        .expect("a bin holds ten of what it carries");
    let made: Vec<String> = game
        .rows()
        .rows()
        .iter()
        .filter(|row| row.relation == "capacity" && row.value("of") == Some("42"))
        .map(|row| game.schema().write(row))
        .collect();

    // **Two rows and not four**, because `for` and `what` name one family and so are one choice.
    // A row per pairing would say a metal bin holds food.
    assert_eq!(
        made,
        vec![
            "{capacity of:42 for:30 what:30 per:13 quantity:10}",
            "{capacity of:42 for:31 what:31 per:13 quantity:10}",
        ],
        "one row per resource, the same member on both sides"
    );
}

/// **A template and a row written out cannot disagree**, and the check that says so is the one
/// that was already there.
///
/// Sean, 2026-09-18: *I don't really care what time a contradiction like that is detected, so long
/// as we can write code to prevent it from entering a live game. I am imagining a live recipe
/// editor that will be able to reject invalid recipes and give the reason.* **Reifying before the
/// key check is what makes this free**: the expansion produces a row keyed exactly as the written
/// one, and two rows of one key is already refused.
#[test]
fn a_template_and_a_row_written_out_cannot_disagree() {
    assert_eq!(
        with("{capacity of:42 for:27 what:27 per:13 quantity:10}\n{capacity of:42 for:30 what:30 per:13 quantity:4}")
            .expect_err("the template already says what a metal bin holds"),
        Malformed::TwoWithOneKey {
            relation: "capacity".to_string(),
            // **In the relation's declared column order**, which is the order a person reads
            // the row in rather than the order the values happen to sort.
            key: vec![
                ("of".to_string(), "42".to_string()),
                ("for".to_string(), "30".to_string()),
                ("what".to_string(), "30".to_string()),
                ("per".to_string(), "13".to_string()),
            ]
        }
    );

    // **The control, and it is the same pair with the written-out row naming a resource the
    // template does not reach.** There is none - a family reaches all of its members - so the
    // control is a template alone, which is accepted, and that is what says the refusal above is
    // about the disagreement rather than about templates.
    with("{capacity of:42 for:27 what:27 per:13 quantity:10}").expect("a template on its own");

    // **And a row that is not the world's is not expanded.** `{member kind:30 family:27}` names
    // `resource` as the thing it is, so a second membership is one row and not two - which is
    // what restricting reification to what `{state ...}` declares buys. **`27` is `resource` and
    // would be two rows if it were.**
    let game = with("{family relation:29}\n{member kind:30 family:29}")
        .expect("metal may also be a labor");
    assert_eq!(
        game.rows()
            .rows()
            .iter()
            .filter(|row| row.relation == "member" && row.value("kind") == Some("30"))
            .count(),
        2,
        "two memberships, not one per member of the family named"
    );
}

/// **Only what is fungible may lie loose**, and a kind's key is what says whether it is.
///
/// `spec/logistics.md`: *What a place holds of a kind is one number.* A kind told apart by
/// anything beyond where it is and what it is of could hold two numbers in one place, and then
/// taking what is over capacity would have to choose which row to take from.
///
/// **Sean, 2026-09-19**: *I am expecting that we can compute the amount of room for something, we
/// can compute the excess, and discard the rest. I don't imagine we need to choose anything here.*
/// **He was right, and two earlier versions of this lane's were wrong in the same way** - a
/// refusal when `keep` fired, then a count of rows in a world. Both guarded the situation; this
/// refuses what allows it.
///
/// **19 is `extractor`, which is keyed by `where`, `what` and `working`.** `working` is the third,
/// so an extractor could be two numbers in one territory - one spent, one ready - and it may not
/// be loose.
#[test]
fn only_what_is_fungible_may_lie_loose() {
    assert_eq!(
        with("{loose kind:19}").expect_err("an extractor is told apart by more than where it is"),
        Malformed::LooseAndNotFungible {
            relation: "extractor".to_string(),
            by: "working".to_string()
        }
    );

    // **The control, and it is the kind that is already loose.** `42` is `bin`, keyed by `where`
    // and `what` - which are the two things a capacity groups by, so one bin row per group per
    // place and nothing to choose between.
    with("{loose kind:42}").expect("a bin is where it is and what it is of, and nothing more");

    // **And the loose kinds the game has are accepted**, which is `before()` loading at all:
    // `{loose kind:resource}` names a family, so `metal` and `food` are both checked and both
    // keyed by `where` alone.
    assert_eq!(
        before()
            .rows()
            .rows()
            .iter()
            .filter(|row| row.relation == "loose")
            .count(),
        1,
        "one row, naming the family, and not one per member"
    );
}
