//! The five recipes and the closed form give the same population, at every pair in a range.
//!
//! **`Q-8`'s shape, and the reason it is a test rather than a deletion.** `P-373`'s saturating
//! rewrite replaced `grow` - which consumed *the lesser of the surplus food and the citizens
//! here* - with `upkeep`, `bear`, `breed`, `renew` and `perish`, each carrying a constant
//! quantity. The obvious move when the rewrite landed was to delete `population_after`, the
//! closed form it replaced. **Keeping it and comparing the two is worth more**: a refactor with
//! no new check is not done, it is unverified, and the passing tests that were already there
//! passed beforehand too.
//!
//! **Over every case, and the count is asserted.** A comparison at one pair stops meaning
//! anything the day that pair is edited away, and goes on passing. This runs every pair in a
//! range and says how many there were, so a version that quietly stopped iterating fails on the
//! count rather than passing on an empty loop.
//!
//! # What the two would have disagreed about, and why they do not
//!
//! `C-83`: `bear` made one fertility per fertile citizen whether or not there was food to breed
//! with, and nothing consumed the remainder - so a territory that starved to nobody ended the
//! turn with fertility in hand and repopulated from stock the next turn, which the closed form
//! forbids. **`P-380` is what closed it**, by giving `discard` a `fertility` row. The case is
//! asserted directly below, because *the two agree* is a weaker statement than *they agree
//! here, where they did not*.

use game_model::thing::Kind;
use game_model::{Biome, Resource, Territory, TerritoryId, territory::population_after};

/// A territory with this many citizens and this much food, and nothing else.
fn with(citizens: u32, food: u32) -> Territory {
    let mut territory = Territory::empty(TerritoryId(1), Biome::Grassland);
    territory.put(Kind::Citizen, citizens);
    territory.add(Resource::Food, food);
    territory
}

/// The recipes and the closed form agree, at every pair up to twelve by twelve.
#[test]
fn the_five_recipes_and_the_closed_form_agree_at_every_pair() {
    let mut compared = 0;
    let mut grew = 0;
    let mut starved = 0;

    for citizens in 0..=12 {
        for food in 0..=12 {
            let mut territory = with(citizens, food);
            territory.settle_population();
            let by_recipe = territory.citizens();
            let closed = population_after(citizens, food);
            assert_eq!(
                by_recipe, closed,
                "{citizens} citizens with {food} food: the recipes give {by_recipe} and \
                 `population_after` gives {closed}"
            );
            compared += 1;
            if by_recipe > citizens {
                grew += 1;
            }
            if by_recipe < citizens {
                starved += 1;
            }
        }
    }

    assert_eq!(
        compared,
        13 * 13,
        "every pair from 0 to 12 in both, and no fewer"
    );

    // **The population this ran against, said out loud.** Agreement over cases that all do
    // nothing is agreement about nothing, so both directions have to occur - and a range that
    // only starved, or only grew, would satisfy the equality above for the wrong reason.
    assert!(
        grew > 0 && starved > 0,
        "{grew} pairs grew and {starved} starved; the comparison needs both to mean anything"
    );
}

/// A population of none never grows, however much food there is.
///
/// **`C-83` is what this is here for.** The decomposition reproduced the closed form in every
/// case except one: `bear` makes a fertility per fertile citizen whether or not there is food
/// to breed with, `breed` is the only thing that consumes one, and nothing swept the
/// remainder - so two citizens with no food ended the turn dead and holding two fertility,
/// and bred from stock the next turn. `P-380` gave `discard` its `fertility` row.
#[test]
fn a_territory_that_starves_to_nobody_holds_nothing_to_breed_from() {
    let mut territory = with(2, 0);
    territory.settle_population();
    assert_eq!(
        territory.citizens(),
        0,
        "no food, so both go unpaid and perish"
    );

    // **Before the sweep this was 2**, which is the whole of `C-83`.
    territory.end_of_turn_losses();
    assert_eq!(
        territory.count_of(Kind::Fertility),
        0,
        "fertility is transient and does not survive the turn's end - `P-380`, `P-381`"
    );

    // And the next turn, with food, it stays dead.
    territory.add(Resource::Food, 10);
    territory.settle_population();
    assert_eq!(
        territory.citizens(),
        0,
        "a population of none never grows however much food there is"
    );
}

/// `bear` bounds the increase by the citizens, and `breed` bounds it by the food.
///
/// **The two halves of what `grow`'s expression did in one word**, checked apart so that
/// losing either is a failure rather than a number that still looks plausible.
#[test]
fn the_increase_is_bounded_at_both_ends() {
    // Bounded by the food: two citizens, three food. Two eat, one is left, one is born.
    let mut lean = with(2, 3);
    lean.settle_population();
    assert_eq!(lean.citizens(), 3, "one surplus food, so one more citizen");

    // Bounded by the citizens: two citizens, eight food. Two eat, six are left - and two
    // citizens can only bear twice, so the population doubles rather than reaching eight.
    let mut plenty = with(2, 8);
    plenty.settle_population();
    assert_eq!(
        plenty.citizens(),
        4,
        "six surplus food and only two citizens to bear, so it doubles and stops"
    );

    // **`spent` is what does it.** With the trait never set, `breed` would go on firing while
    // the food lasted and eight food would give eight citizens.
    assert!(
        plenty.citizens() < 2 + 6,
        "the food alone would allow six more; the `spent` trait is what stops at two"
    );
}
