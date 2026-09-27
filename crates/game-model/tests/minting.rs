//! `{minted}` and `{constant}` - the two rows `P-575` needs before a rule can make a world.
//!
//! **`S-200`, from `P-575`, which Sean approved as `N5`**: designing stays made of the same rules as
//! playing, and two notation pieces make that possible.
//!
//! ```text
//! {constant part:P input:i value:6}   a value supplied to an input a part invokes, where
//!                                    {argument} supplies a reference. The part-layer twin
//!                                    of {literal clause:C column:N value:1}
//! {minted clause:C column:N}         an `add` clause's column takes the next id unused by
//!                                    that relation
//! ```
//!
//! **`S-200` asks for the engine first, deliberately.** Four of the five design rules need one or
//! both, and *writing them first would be four intricate rules validated by nothing* - no test could
//! pass, so no test could tell a correct rule from a wrong one. So this is that half: the rows mean
//! something now, and the rules follow.
//!
//! # The world here states only what it needs, and the foundation is not it
//!
//! **Every other test in this crate loads the foundation and this one does not.** Two reasons, and
//! the second is the durable one.
//!
//! **`C-156`**: `P-577` gave `territory` a `biome` column and 380 territory rows do not carry one,
//! so `common::before()` is refused today. A test of an engine primitive that cannot run while a
//! game rule is being decided is a test coupled to something it has nothing to do with.
//!
//! **And a primitive is better shown over a world that states only what it needs.** `{minted}` is
//! about ids and `{constant}` about inputs; a territory, a place and fifteen rules are noise in both
//! sentences. The world below is 67 rows and every one of them is load-bearing - which is a claim
//! this file asserts rather than makes, by counting what it holds.
//!
//! # `minted` is per store, and the world here is one store
//!
//! **`S-201`, re-derived by the specification lane rather than agreed with**: `script.4x` uses
//! relation ids 1 to 6 and fourteen column ids, and all twenty collide with a different thing in
//! `schema.4x`. So the two stores have been separate id spaces since `script.4x` existed, and
//! `minted` inherits that scope rather than introducing it.

use game_model::engine::{Game, fire};
use game_model::notation::{Row, read, write};

/// A world with two rules, one that mints an id and one that hands it a constant.
///
/// **`thing` is deliberately not a game noun.** Naming it a territory would make every assertion
/// below read as a claim about the game, and none of them is.
///
/// **`thing id:1` and `thing id:3` are there and `2` is not**, because the gap is what tells *the
/// least unused* apart from *one past the largest* - and a world with no gap cannot.
///
/// **Each seeded thing labels itself.** One test below makes `label` a reference, and a seeded row
/// whose label were a word would then make `Game::of` refuse the world - which would test world
/// construction rather than what a constant does.
const WORLD: &str = r"
{relation id:1 name:relation}
{relation id:2 name:column}
{relation id:3 name:role}
{relation id:4 name:rule}
{relation id:5 name:input}
{relation id:6 name:clause}
{relation id:7 name:binding}
{relation id:8 name:minted}
{relation id:9 name:thing}
{relation id:10 name:part}
{relation id:11 name:constant}
{relation id:12 name:argument}

{column id:1 relation:1 seq:1 name:id}
{column id:2 relation:1 seq:2 name:name}
{column id:3 relation:2 seq:1 name:id}
{column id:4 relation:2 seq:2 name:relation}
{column id:5 relation:2 seq:3 name:seq}
{column id:6 relation:2 seq:4 name:name}
{column id:7 relation:3 seq:1 name:id}
{column id:8 relation:3 seq:2 name:name}
{column id:9 relation:4 seq:1 name:id}
{column id:10 relation:4 seq:2 name:name}
{column id:11 relation:5 seq:1 name:id}
{column id:12 relation:5 seq:2 name:rule}
{column id:13 relation:5 seq:3 name:seq}
{column id:14 relation:5 seq:4 name:name}
{column id:15 relation:5 seq:5 name:of}
{column id:16 relation:6 seq:1 name:id}
{column id:17 relation:6 seq:2 name:rule}
{column id:18 relation:6 seq:3 name:seq}
{column id:19 relation:6 seq:4 name:role}
{column id:20 relation:6 seq:5 name:relation}
{column id:21 relation:7 seq:1 name:id}
{column id:22 relation:7 seq:2 name:clause}
{column id:23 relation:7 seq:3 name:input}
{column id:24 relation:7 seq:4 name:column}
{column id:25 relation:8 seq:1 name:clause}
{column id:26 relation:8 seq:2 name:column}
{column id:27 relation:9 seq:1 name:id}
{column id:28 relation:9 seq:2 name:label}
{column id:29 relation:10 seq:1 name:id}
{column id:30 relation:10 seq:2 name:of}
{column id:31 relation:10 seq:3 name:is}
{column id:32 relation:10 seq:4 name:seq}
{column id:33 relation:11 seq:1 name:part}
{column id:34 relation:11 seq:2 name:input}
{column id:35 relation:11 seq:3 name:value}
{column id:36 relation:12 seq:1 name:id}
{column id:37 relation:12 seq:2 name:part}
{column id:38 relation:12 seq:3 name:input}
{column id:39 relation:12 seq:4 name:value}

{role id:1 name:require}
{role id:2 name:remove}
{role id:3 name:add}

{rule id:1 name:make-thing}
{input id:1 rule:1 seq:1 name:label of:9}
{clause id:1 rule:1 seq:1 role:3 relation:9}
{minted clause:1 column:27}
{binding id:1 clause:1 input:1 column:28}

{rule id:2 name:label-a-thing}
{part id:1 of:2 is:1 seq:1}
{constant part:1 input:1 value:hello}

{rule id:3 name:find-a-thing}
{clause id:2 rule:3 seq:1 role:1 relation:9}
{minted clause:2 column:27}

{thing id:1 label:1}
{thing id:3 label:3}
";

/// The world, with some rows added or replaced.
///
/// **Replaced by key**, so a test can say *this world but with that row different* without restating
/// sixty lines - and `the_world_states_only_what_it_needs` is what says a substitution substituted.
///
/// # A row without an id is keyed by its description, and that is not a nicety here
///
/// **This keyed on `id` alone and a `{constant}` has none.** `{constant part:1 input:1 value:1}`
/// replaced nothing, both constants sat in the world, and the engine took the first - so the control
/// in `a_constant_that_should_have_been_a_reference_is_caught_by_the_structure` refused for the
/// reason the test above it exists to show, which read as the engine being wrong.
///
/// **So the key is `id` where there is one and every other column where there is not.** `value` is
/// the payload rather than part of the key, which is how the notation already keys a described
/// relation - `residency` carries a quantity and is keyed by `(what, where)`.
fn world(extra: &str) -> Game {
    let mut rows = read(WORLD).expect("the world reads");
    let added = read(extra).unwrap_or_else(|why| panic!("{extra}: {why}"));
    for row in added {
        let same_key = |it: &Row| match row.value("id") {
            Some(id) => it.value("id") == Some(id),
            None => row
                .values
                .iter()
                .filter(|(column, _)| *column != "value")
                .all(|(column, value)| it.value(column) == Some(value.as_str())),
        };
        rows.retain(|it| it.relation != row.relation || !same_key(it));
        rows.push(row);
    }
    Game::of(rows).unwrap_or_else(|why| panic!("{why}"))
}

fn things(game: &Game) -> Vec<String> {
    game.rows()
        .rows()
        .iter()
        .filter(|row| row.relation == "thing")
        .map(write)
        .collect()
}

fn command(text: &str) -> Row {
    read(text).unwrap_or_else(|why| panic!("{text}: {why}"))[0].clone()
}

/// **The world is a world, and every row of it is load-bearing.**
///
/// **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. Every test
/// below fires a rule stated in `WORLD`, so a `WORLD` that had quietly lost a row would make them
/// fail rather than pass - but a `WORLD` that had grown one nobody needs would go unnoticed, and
/// this is where a reader finds out how big it is.
#[test]
fn the_world_states_only_what_it_needs() {
    let game = world("");
    assert_eq!(
        game.rows().rows().len(),
        67,
        "the world is 67 rows: {} declared relations, their columns, three roles, three rules \
         and two things",
        game.rows()
            .rows()
            .iter()
            .filter(|row| row.relation == "relation")
            .count()
    );
    assert_eq!(
        things(&game),
        vec!["{thing id:1 label:1}", "{thing id:3 label:3}"]
    );
}

/// **A minted id is the least one the relation is not using.**
///
/// `S-200`: *an `add` clause's column takes the next id unused by that relation.* **Next unused
/// rather than a counter**, so it is a fact about the world rather than state anybody keeps.
///
/// # The gap is the whole of the test
///
/// **`thing id:1` and `thing id:3` are there and `2` is not.** So *the least unused* gives `2` and
/// *one past the largest* gives `4`, and this is the assertion that says which the engine does. A
/// world with no gap would pass under either reading and say nothing.
///
/// **The reason for the least is that it makes the id a function of the world rather than of its
/// history**: a row removed and added back takes the id it had, where one past the largest would
/// give it a new one. `S-200` asks for a fact about the world, and that is the one that is.
#[test]
fn a_minted_id_fills_the_gap_rather_than_following_the_largest() {
    let (after, effect) = fire(&world(""), &command("{label-a-thing}"), 1).expect("it fires");
    assert_eq!(
        effect.made.iter().map(write).collect::<Vec<String>>(),
        vec!["{thing id:2 label:hello}"],
        "the least unused is 2, and one past the largest would be 4"
    );
    assert_eq!(
        things(&after).len(),
        3,
        "the two that were there, and the one made"
    );
}

/// **Each repeat mints a different id**, because the store one is minted against is the world as it
/// stands and not the world the command arrived in.
///
/// # This is the failure the parameter exists to prevent
///
/// **`row_of` is handed `game` everywhere else**, which is the world before the command. Minting
/// from that would have given every repeat the same id, and every `add` clause of one rule the same
/// id as every other - a row silently colliding with the one beside it. So `row_of` takes the store
/// as it stands, and this is what says it does.
///
/// **Three rather than two**, because two consecutive ids can be a coincidence of the gap: `2` then
/// `4` reads as *the gap, then one past the largest*. The third distinguishes them.
#[test]
fn each_repeat_mints_a_different_id() {
    let (after, effect) = fire(&world(""), &command("{label-a-thing}"), 3).expect("it fires");
    assert_eq!(
        effect.made.iter().map(write).collect::<Vec<String>>(),
        vec![
            "{thing id:2 label:hello}",
            "{thing id:4 label:hello}",
            "{thing id:5 label:hello}"
        ],
        "2 fills the gap, then 4 and 5 follow"
    );
    assert_eq!(
        things(&after).len(),
        5,
        "two were there and three were made"
    );
}

/// **A `{constant}` fills an input the part gives no `{argument}` for.**
///
/// `S-200`: *a value supplied to a part or a command's input, where `{argument}` supplies a
/// reference. It is the part-layer twin of `{literal clause:C column:N value:1}`.*
///
/// **`hello` is not a key of `thing`**, and that is the point rather than an oversight: an input is
/// typed as a relation and carries one of its keys, so a constant is exactly the case where asking
/// that question is asking the wrong one - as `{literal}` gives a quantity no reference check could
/// pass.
#[test]
fn a_constant_supplies_an_input_that_no_argument_does() {
    let (_, effect) = fire(&world(""), &command("{label-a-thing}"), 1).expect("it fires");
    assert_eq!(
        effect.made[0].value("label"),
        Some("hello"),
        "the constant reached the column its binding names"
    );
}

/// **The check a constant skips is not lost, it happens one layer down.**
///
/// A constant is not asked whether it is a key of the input's relation. **The row it ends up in is
/// still checked against the structure**, so a constant that should have been a reference is caught
/// there - by the world the rule would leave rather than by the argument it was given.
///
/// **This is the claim the engine's own comment makes, asserted rather than stated.** `CLAUDE.md`:
/// a check earns its place by a failure it could have produced, and *what moves is where the check
/// happens, not whether there is one* is a sentence that could have been wrong.
#[test]
fn a_constant_that_should_have_been_a_reference_is_caught_by_the_structure() {
    // `thing.label` points at a `thing` now, so `hello` is no longer a legal value for it.
    let game = world(
        "{relation id:13 name:reference}\n{column id:40 relation:13 seq:1 name:id}\n{column id:41 relation:13 seq:2 name:column}\n{column id:42 relation:13 seq:3 name:to}\n{reference id:1 column:28 to:9}",
    );
    let why = fire(&game, &command("{label-a-thing}"), 1).expect_err("`hello` is not a `thing`");
    assert_eq!(
        format!("{why}"),
        "`make-thing` would leave a world where `thing`.`label` is `hello`, and no `thing` has \
         that key"
    );

    // The control, so the refusal is about the constant and not about the reference existing: with
    // a constant that *is* a key, the same world fires.
    let game = world(
        "{relation id:13 name:reference}\n{column id:40 relation:13 seq:1 name:id}\n{column id:41 relation:13 seq:2 name:column}\n{column id:42 relation:13 seq:3 name:to}\n{reference id:1 column:28 to:9}\n{constant part:1 input:1 value:1}",
    );
    let (_, effect) = fire(&game, &command("{label-a-thing}"), 1).expect("`1` is a `thing`");
    assert_eq!(effect.made[0].value("label"), Some("1"));
}

/// **An `{argument}` wins over a `{constant}` for the same input**, because one is the caller
/// speaking and the other is the part.
///
/// **The same rule `{literal}` follows after `{binding}`**: one input, one source, and the order
/// says which wins where the data says two things. **And the argument is type-checked while the
/// constant is not**, so this is also what says the winner is checked.
#[test]
fn an_argument_beats_a_constant_for_one_input() {
    let game = world("{argument id:1 part:1 input:1 value:3}");
    let (_, effect) = fire(&game, &command("{label-a-thing}"), 1).expect("it fires");
    assert_eq!(
        effect.made[0].value("label"),
        Some("3"),
        "the argument won, and the constant said `hello`"
    );
}

/// **A `{minted}` on a clause that matches rather than makes is refused.**
///
/// Minting an id to look for asks the world for a row nobody has made. `require` and `remove` match
/// a pattern and only `add` builds a whole row, so this is the data saying something that cannot
/// mean anything - refused rather than answered with a number.
///
/// **It would otherwise be a `NotSo`**, which reads as *the world does not agree* about a row whose
/// id the rule invented. That names the symptom and sends a reader to the world.
#[test]
fn minting_where_nothing_is_made_is_refused() {
    let why = fire(&world(""), &command("{find-a-thing}"), 1).expect_err("nothing is made");
    assert_eq!(
        format!("{why}"),
        "`find-a-thing` mints `id` in clause `2`, which matches rather than makes"
    );
}
