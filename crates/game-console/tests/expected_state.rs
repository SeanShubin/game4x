//! The scenario's expected state, read from a data file rather than from Rust.
//!
//! **`S-29`, and half of `P-218`.** A data file the *test* reads is not a data file the
//! *game loads*; the kinds, recipes and costs are still Rust and markdown.
//!
//! **The assertions this replaces are gone, and they went in the change that seeded the
//! file.** `c37de2e`, 2026-09-04: fourteen of them - the landing site's citizens, its
//! extractors and stores, the turn, control, the second territory's garrison. `S-34` asked for
//! exactly that: *the same change that puts the first expectation in. Not before, or the
//! scenario is checked by nothing. Not after, because after is a window in which the scenario
//! has two expectations - and the one that is wrong is not the one that fails.*
//!
//! **This paragraph said the opposite for three days**, and an item reported from it - `C-49`
//! told the specification lane that a bullet of `S-29` was deliberately not done when it had
//! been done since the fourth. **A comment that describes the file it sits in goes stale
//! without the file changing under it**, and reading one feels identical to reading the code.
//! `S-57`.
//!
//! # The window is closed and nothing keeps it closed
//!
//! **`S-34`'s rule has no mechanism, and `C-58` records why one was built and deleted.** The
//! predicate available - *assertions after the line that runs the scenario* - is blunter than
//! the rule: eight lines in `first_release.rs` run it, only two are about what it leaves, and
//! three of the rest are determinism tests that cannot go stale against this file, because
//! whole-state equality moves with the game.
//!
//! **The distinction that would make it precise is not mechanisable**: the failure is a *stale*
//! assertion, one that disagrees, and whether two statements can drift is a fact about the
//! future. `C-28`'s wall. So the case is written here beside the file it is about, which is
//! what is available instead.
//!
//! **`S-47` changed what a state is written as, and this is what checks the new form.** It
//! is the map form of `spec/console.md`: a thing appears inside what holds it, an entry is a
//! description and a quantity, and nothing states its container. The predecessor wrote flat
//! rows whose vocabulary came from `dump::tables` - `C-37`.
//!
//! **What is proven here is the mechanism**, over states written to disagree in each of the
//! three directions. A comparison nobody has seen fail is a claim.

use game_console::containment::Entry;
use game_console::state;

/// Where the reviewed expectation lives.
mod common;

use common::played;

/// How many entries there are, root included.
fn size(tree: &Entry) -> usize {
    tree.walk().len()
}

/// A state written out and read back is the same state.
///
/// **This is the check `releases/first-release.md` -> *Where things are* names**: *the check
/// is that the dump reads back into the state it came from. A count of fields is not,
/// because a plausible subset passes it.*
///
/// **And it is the tree that round trips, not the [`game_model::Game`].** Those are the same
/// information only once a territory's `density` and `total capacity` are in the file, and
/// they are not - a description is a flat map and a territory has three densities. `C-46`.
/// Saying which half is proved is `S-29`'s own warning about reporting half a rule as met.
#[test]
fn a_state_survives_being_written_and_read() {
    let session = played();
    let direct = state::entries(&session.game);
    let written = state::write(&session.game, "after `scenario/commands/play.4x`");
    let read = state::read(&written).expect("what was just written must parse");

    assert_eq!(size(&read), size(&direct), "every entry survived");
    // **Against the containment alone, and that is a concession rather than a rule.**
    // `Q-66`: this said *capacity is derived*, which is true of `used` and false of `total` -
    // the release stores `total capacity`, and the derived trait in that neighbourhood is
    // `metal in it`. So the sentence next to the assertion made the omission sound
    // legitimate while the doc comment above called it a limitation, and only the false one
    // was where a reader would meet it.
    //
    // What is actually conceded: a territory's stored `total capacity` is not in the file,
    // because a description is a flat map and a territory has one per kind. `C-46`.
    assert_eq!(read, direct.contained(), "and each is the entry it was");
    assert_eq!(
        state::written(&read),
        state::written(&direct),
        "the same state is the same bytes"
    );

    assert!(
        size(&direct) > 40,
        "only {} entries in a played game; the state has probably changed shape",
        size(&direct)
    );

    // **`P-252`: nothing in a data file is quoted, so nothing in one may need to be.**
    //
    // Every word is checked and the number of them is asserted, because a run over no words
    // would satisfy a count of zero for the wrong reason.
    let mut words = 0;
    let mut unwritable = Vec::new();
    for entry in direct.walk() {
        for word in entry.description.words() {
            if word.contains(' ') || word.contains('"') || word.is_empty() {
                unwritable.push(word.clone());
            }
            words += 1;
        }
    }
    assert!(
        unwritable.is_empty(),
        "these would have to be quoted, and nothing in a data file is: {unwritable:?}"
    );
    assert!(
        words > 150,
        "only {words} words examined, so an empty run would pass the assertion above"
    );
}

/// The comparison finds all three disagreements, and none where there are none.
#[test]
fn the_comparison_finds_missing_extra_and_different() {
    let session = played();
    let actual = state::entries(&session.game);

    let same = state::compare(&actual, &actual);
    assert_eq!(
        same.total(),
        0,
        "a state disagrees with itself:\n{}",
        same.report()
    );

    // Different: one entry's quantity moved.
    let mut changed = actual.clone();
    let held = changed
        .contents
        .iter_mut()
        .find(|entry| !entry.contents.is_empty())
        .expect("something in the game contains something");
    held.contents[0].quantity += 1;
    let wrong = state::compare(&changed, &actual);
    assert_eq!(
        (
            wrong.different.len(),
            wrong.missing.len(),
            wrong.extra.len()
        ),
        (1, 0, 0),
        "a changed quantity is one difference and nothing else:\n{}",
        wrong.report()
    );

    // Missing: it was expected and did not happen.
    let mut absent = actual.clone();
    absent.contents.push(Entry {
        description: game_console::containment::Description::of(game_model::thing::Kind::Yard),
        quantity: 1,
        contents: Vec::new(),
        capacity: Vec::new(),
    });
    assert_eq!(
        state::compare(&absent, &actual).missing.len(),
        1,
        "an entry expected and not produced is missing"
    );

    // Extra: it happened and nobody expected it. **The direction a per-value assertion
    // cannot have** - each is right about what it names, and names what somebody thought of.
    let mut fewer = actual.clone();
    fewer.contents.pop();
    let extra = state::compare(&fewer, &actual);
    assert!(
        !extra.extra.is_empty(),
        "an entry produced and not expected is extra"
    );
}

/// The seeding branch, over a directory of its own.
///
/// **`P-225`: absence means acceptance**, so an update is a deletion rather than an edit -
/// deliberate, visible in `git status`, impossible by hand slip. This exercises that where
/// it is safe: a temporary directory, seeded from a state, then compared with it.
///
/// It is *not* run against `scenario/expected/play.4x`, and that is `P-227`.
#[test]
fn an_absent_expectation_is_seeded_and_then_compared() {
    let at = std::env::temp_dir().join("game4x-expected-seed");
    let _ = std::fs::remove_dir_all(&at);
    std::fs::create_dir_all(&at).expect("a directory to seed into");
    let file = at.join("play.4x");

    let session = played();
    let actual = state::entries(&session.game);

    assert!(!file.exists(), "the point is that it is not there yet");
    std::fs::write(&file, state::write(&session.game, "seeded")).expect("seeding writes it");

    let back = state::read(&std::fs::read_to_string(&file).unwrap()).expect("parses");
    let wrong = state::compare(&back, &actual);
    assert_eq!(
        wrong.total(),
        0,
        "a seed agrees with what seeded it:\n{}",
        wrong.report()
    );

    std::fs::remove_dir_all(&at).ok();
}
