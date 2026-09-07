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

use std::path::PathBuf;

use game_console::state;
use game_console::{Library, Session};
use game_model::containment::Entry;

/// Where the reviewed expectation lives.
const AT: &str = "scenario/expected/play.4x";

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn played() -> Session {
    let files = Files(root().join("scenario/commands"));
    let mut session = Session::new();
    for line in ["{run file:setup}", "{start}", "{run file:play}"] {
        session
            .run(line, &files)
            .unwrap_or_else(|why| panic!("`{line}` failed: {why}"));
    }
    session
}

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

/// Every rule `spec/console.md` states about the map form, over the whole played state.
///
/// **Over every entry rather than on an example, and the count with it.** A rule shown on
/// one entry stops showing anything the moment that entry is edited away, and goes on
/// passing - `docs/notes/checks-outlive-examples.md`. The count is what tells that apart
/// from a walk that found nothing.
#[test]
fn the_played_state_obeys_every_rule_the_map_form_has() {
    let session = played();
    let tree = state::entries(&session.game);
    let all = tree.walk();
    assert!(
        all.len() > 40,
        "only {} entries, so each assertion below would pass over almost nothing",
        all.len()
    );

    let mut with_an_id = 0;
    for entry in &all {
        assert!(
            entry.quantity > 0,
            "an entry is never zero: {}",
            entry.description.written()
        );
        if entry.description.traits.contains_key("id") {
            with_an_id += 1;
            assert_eq!(
                entry.quantity,
                1,
                "there is never a quantity of a thing with an `id`: {}",
                entry.description.written()
            );
        }
        // Entries are in the order their descriptions sort in, so the same state is the
        // same bytes.
        let written: Vec<String> = entry
            .contents
            .iter()
            .map(|held| held.description.written())
            .collect();
        let mut sorted = written.clone();
        sorted.sort();
        assert_eq!(
            written,
            sorted,
            "the contents of {} are not in description order",
            entry.description.written()
        );
        // Each distinct description is its own entry, so no description appears twice in
        // one map.
        let mut seen = std::collections::BTreeSet::new();
        for description in &written {
            assert!(
                seen.insert(description.clone()),
                "{description} is two entries of one map, and each distinct description is one"
            );
        }
    }
    assert!(
        with_an_id >= 24,
        "only {with_an_id} things carry an id, and twelve territories and twelve orbits do"
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
        description: game_model::containment::Description::of(game_model::thing::Kind::Yard),
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

/// A territory's own numbers are rebuilt from the file and compared with the model.
///
/// **This is the claim `S-62` asked to be tested rather than stated**, and it is the one
/// `C-53` said was half true. `releases/first-release.md` -> *Where things are*: **the check
/// is that the dump reads back into the state it came from.** Territory 3 is `6 x 2` for
/// food; before `P-331` the file said `density:2` and six was nowhere, so a reader with the
/// file alone could not say what that ground offered.
///
/// **Both numbers are on the deposit now**, so this rebuilds every territory's id, biome,
/// force of nature and per-resource pair out of the text and holds them against the model.
///
/// # What this does not claim, and what poisoning it taught
///
/// **It is the territories, not the whole state**, and saying which is the whole of `S-29`'s
/// warning about reporting half a rule as met. What a territory *holds* round trips as
/// entries and is checked by `a_state_survives_being_written_and_read`.
///
/// **And it cannot catch a wrong number.** The first poison added one to every capacity, the
/// file was regenerated from the poisoned model, and this passed - correctly. A round trip
/// compares a file with the state it was written from, so a value that is wrong in both is
/// wrong consistently. **What catches a wrong number is the release**, in
/// `first_release.rs::released_table`, which reads `6 x 2` out of Sean's own table.
///
/// **The poison that reaches this one is asymmetric**: stop the writer stating
/// `total-capacity`, reseed, and it fails naming the territory and the resource. Two poisons,
/// and only the second is about the property - which is `C-57`'s lesson arriving a day later
/// in a different test.
#[test]
fn every_territorys_own_numbers_survive_the_round_trip() {
    let session = played();
    let text = std::fs::read_to_string(root().join(AT)).expect("the expected state");
    let read = state::read(&text).unwrap_or_else(|why| panic!("{AT} does not parse: {why}"));

    let mut checked = 0;
    for place in &session.game.territories {
        let described = read
            .contents
            .iter()
            .find(|entry| {
                entry.description.kind == "territory"
                    && entry.description.traits.get("id") == Some(&place.id.0.to_string())
            })
            .unwrap_or_else(|| panic!("territory {} is not in the file", place.id));

        let says = |name: &str| -> String {
            described
                .description
                .traits
                .get(name)
                .cloned()
                .unwrap_or_else(|| panic!("territory {} states no {name}", place.id))
        };
        assert_eq!(says("biome"), place.biome.name(), "territory {}", place.id);
        assert_eq!(
            says("nature"),
            place.force_of_nature.to_string(),
            "territory {}",
            place.id
        );

        for resource in game_model::Resource::ALL {
            let offered = place.deposit(resource);
            let deposit = described.contents.iter().find(|entry| {
                entry.description.kind == "deposit"
                    && entry.description.traits.get("resource")
                        == Some(&resource.name().to_string())
            });
            match deposit {
                // **A ground that offers nothing has no deposit, and an entry is never
                // zero** - `spec/console.md`. So absence has to mean nothing offered, and
                // this is the arm that says so rather than skipping.
                None => assert_eq!(
                    (offered.capacity, offered.density),
                    (0, 0),
                    "territory {} offers {resource} and the file has no deposit for it",
                    place.id
                ),
                Some(entry) => {
                    let has = |name: &str| -> u32 {
                        entry
                            .description
                            .traits
                            .get(name)
                            .and_then(|value| value.parse().ok())
                            .unwrap_or_else(|| {
                                panic!("territory {}'s {resource} states no {name}", place.id)
                            })
                    };
                    assert_eq!(
                        (has("total-capacity"), has("density")),
                        (offered.capacity, offered.density),
                        "territory {}'s {resource}, which the release writes `{} x {}`",
                        place.id,
                        offered.capacity,
                        offered.density
                    );
                    checked += 1;
                }
            }
        }
    }

    // **Over every case, and how many.** Twelve territories and three resources, less the
    // ground that offers nothing - territory 6 has no metal and territory 7 no energy, which
    // the release's own table says. A run that compared nothing would satisfy every assertion
    // above it.
    assert_eq!(
        checked, 34,
        "twelve territories times three resources, less the two that offer none; {checked} \
         pairs were compared"
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

/// The reviewed expectation, seeded on absence and compared thereafter.
///
/// **`P-225`: absence means acceptance.** A missing file is how changing your mind is said,
/// so this writes one rather than failing. An update is therefore a deletion rather than an
/// edit - deliberate, visible in `git status`, and impossible by a hand slip.
///
/// **The first seed is a known false positive and Sean said so himself**, withdrawing
/// `P-227`: *as a human I can remember to vet the scenario test the first time, it is
/// remembering to do some mundane task each time that is impossible for a human, which is
/// why we need a test to fail for those times to remind the human.* `P-228` is that reason
/// as a rule - **a check earns its place by guarding the repetition, not the one-off.**
///
/// So this seeds, says loudly that it seeded, and from the next run onward it is the check.
#[test]
fn the_reviewed_expectation_holds() {
    let file = root().join(AT);
    let session = played();
    let actual = state::entries(&session.game);

    let Ok(text) = std::fs::read_to_string(&file) else {
        if let Some(directory) = file.parent() {
            std::fs::create_dir_all(directory).expect("a directory to seed into");
        }
        std::fs::write(
            &file,
            state::write(&session.game, "after `scenario/commands/play.4x`"),
        )
        .unwrap_or_else(|why| panic!("cannot seed {AT}: {why}"));
        panic!(
            "seeded {AT} from the program, which nobody has reviewed.\n\n\
             This is the one-off `P-228` describes: the first expectation cannot be checked \
             by a diff because there is nothing to diff it against. Read it, and if it is \
             what the scenario should produce, commit it. From the next run on, this fails \
             only when the game and the reviewed file disagree.\n\n\
             {} entries written.",
            size(&actual)
        );
    };

    let expected = state::read(&text).unwrap_or_else(|why| panic!("{AT} does not parse: {why}"));
    let wrong = state::compare(&expected, &actual);
    assert_eq!(
        wrong.total(),
        0,
        "{AT} and the game disagree. Either the game is wrong, or you changed your mind - \
         and `P-225` says changing your mind is deleting the file.\n{}",
        wrong.report()
    );
}
