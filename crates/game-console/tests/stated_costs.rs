//! A cost stated in prose agrees with the constant it restates.
//!
//! **`C-9`'s class, found three times in one file's worth of comments on 2026-09-25.** A pioneer
//! costs 3 metal, 2 energy and 2 citizens. Three places in this lane's own columns said something
//! else, and every one of them read perfectly well:
//!
//! | Where | Said | Wrong by |
//! | ----- | ---- | -------- |
//! | `crates/game-model/src/game.rs` | 3 metal, **6** energy, 2 citizens | `P-489` |
//! | `scenario/commands/play.4x` | three metal and **six** energy | `P-489` |
//! | `scenario/commands/spread.4x` | **two** metal and **six** energy | the bind, and `P-489` |
//!
//! **The first was four lines above the comment explaining the change.** `PIONEER_ENERGY`'s own
//! doc says *a cost again, and it is two rather than the six it was* - and the sentence one
//! screen up, stating the same three numbers, was not edited by the promotion that moved them or
//! by the comment written about it.
//!
//! **The third was wrong in two ways at once, from two different changes**, neither of which
//! edited it: two metal predates the bind going to three, and six energy predates `P-489`.
//!
//! # Why a declared list rather than a search for numbers
//!
//! **A regex over prose asking *does this sentence contain a cost* is the narrow instrument this
//! repository keeps finding.** It would match sentences that are not costs, miss costs spelled in
//! words, and return a plausible number either way.
//!
//! So this is the shape `tools/spec`'s `every_number_the_documents_state_is_the_number_that_is_there`
//! already uses for `docs/`: **a declared list of (file, phrase, what it ought to be)**, where
//! the phrase is asserted *present* before its number is compared. A comment that is reworded
//! fails loudly - *no longer says this* - rather than dropping out of the population and leaving
//! a count that still passes.
//!
//! # What it does not do
//!
//! **It does not find a stated cost nobody added to the list.** No check can: that is `C-28`'s
//! wall, and `docs/process.md` says the habit is what covers it. What this does is make the list
//! **fail when it goes stale**, which is the half that was missing - all three of these were
//! found by chasing an unrelated item, and none by anything noticing.

use game_model::game::cost;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// One sentence that states a cost, and the cost it has to agree with.
struct Stated {
    file: &'static str,
    /// The words as they appear, with whitespace as written.
    says: &'static str,
    /// What the sentence's numbers have to be, in the order they appear in it.
    numbers: &'static [u32],
}

/// Every stated cost agrees with the constant it restates.
#[test]
fn every_cost_stated_in_prose_is_the_cost_the_model_charges() {
    let stated = [
        Stated {
            file: "crates/game-model/src/game.rs",
            says: "A Pioneer costs 3 metal, 2 energy and 2 citizens.",
            numbers: &[
                cost::PIONEER_METAL,
                cost::PIONEER_ENERGY,
                cost::PIONEER_CITIZENS,
            ],
        },
        Stated {
            file: "crates/game-model/src/game.rs",
            says: "An Ark costs 3 metal, 12 energy and 2 citizens, and needs a Yard to produce it.",
            numbers: &[cost::ARK_METAL, cost::ARK_ENERGY, cost::ARK_CITIZENS],
        },
        // **Spelled in words, which is why the phrase is asserted rather than the digits
        // searched for.** A scanner looking for numerals would not see this sentence at all,
        // and a scenario comment is exactly where a reader meets the figure first.
        Stated {
            file: "scenario/commands/play.4x",
            says: "a pioneer costs three metal and two energy",
            numbers: &[cost::PIONEER_METAL, cost::PIONEER_ENERGY],
        },
        Stated {
            file: "scenario/commands/spread.4x",
            says: "three metal and two energy against",
            numbers: &[cost::PIONEER_METAL, cost::PIONEER_ENERGY],
        },
    ];

    // Numbers as words, for the sentences that spell them. A closed set: a word this does not
    // know fails loudly below rather than reading as absent.
    let spelled = |value: u32| -> &'static str {
        match value {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            12 => "twelve",
            15 => "fifteen",
            _ => "",
        }
    };

    let mut checked = 0;
    for Stated {
        file,
        says,
        numbers,
    } in stated
    {
        let text = std::fs::read_to_string(root().join(file))
            .unwrap_or_else(|why| panic!("{file}: {why}"));
        // **Whitespace collapsed on both sides**, so a sentence that wraps cannot hide -
        // `CLAUDE.md`, and it is the failure that hid one of the two sentences `C-113` quotes.
        let flat = text.split_whitespace().collect::<Vec<&str>>().join(" ");
        let needle = says.split_whitespace().collect::<Vec<&str>>().join(" ");
        assert!(
            flat.contains(&needle),
            "{file} no longer says {says:?} - if the wording changed, change it here too, \
             because a sentence that has dropped out of this list is one nothing checks"
        );

        // Every number the sentence carries, in order, against what it has to be.
        for value in numbers {
            let digits = value.to_string();
            let word = spelled(*value);
            assert!(
                !word.is_empty(),
                "{value} has no spelling in this test, so a sentence spelling it would read as \
                 absent rather than failing"
            );
            assert!(
                needle.contains(&digits) || needle.contains(word),
                "{file} states a cost and {value} is not in it: {says:?}"
            );
            checked += 1;
        }
    }

    // **Over every case and how many there were.** A list that emptied would satisfy the loop
    // above by having nothing to run.
    assert_eq!(
        checked, 10,
        "four sentences carrying ten numbers between them; {checked} were compared"
    );
}
