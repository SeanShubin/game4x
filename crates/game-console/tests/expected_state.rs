//! The scenario's expected state, read from a data file rather than from Rust.
//!
//! **`S-29`, and half of `P-218`.** A data file the *test* reads is not a data file the
//! *game loads*; the kinds, recipes and costs are still Rust and markdown. What moves is one
//! scenario's expectations, out of ninety-six `assert_eq!` lines and into something Sean can
//! read and check.
//!
//! **The real expectation is not seeded here and that is deliberate.** `P-225` says changing
//! his mind means deleting the expected data, and absence means acceptance - so a missing
//! file is normally a request to seed. `P-227` is open because that rule is scoped to
//! *changing* his mind: **the first seed has no diff to read**, and accepting one means
//! accepting the program's output because the program produced it. That is the failure this
//! file exists to prevent, wearing the costume of a convenience. So the first expectation
//! comes from the hand derivation in `P-211`, and until it exists this compares nothing and
//! says so.
//!
//! **What is proven meanwhile is the mechanism**, over states written to disagree in each of
//! the three directions. A comparison nobody has seen fail is a claim.

use std::path::PathBuf;

use game_console::expected::{self, Row};
use game_console::{Library, Session};

/// Where the reviewed expectation will live once it is derived by hand.
const AT: &str = "expected/play.4x";

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
    let files = Files(root().join("commands"));
    let mut session = Session::new();
    for line in ["run setup", "start", "run play"] {
        session
            .run(line, &files)
            .unwrap_or_else(|why| panic!("`{line}` failed: {why}"));
    }
    session
}

/// A state written out and read back is the same state.
///
/// **The floor everything else stands on.** A comparison is worth nothing if the writing and
/// the reading disagree - it would report differences that are only the notation losing
/// something, and somebody would go looking for them in the game.
#[test]
fn a_state_survives_being_written_and_read() {
    let session = played();
    let written = expected::write(&session.game, "after `commands/play.4x`");
    let read = expected::read(&written).expect("what was just written must parse");
    let direct = expected::rows(&session.game);

    assert_eq!(read.len(), direct.len(), "every row survived");
    assert_eq!(read, direct, "and each is the row it was");

    assert!(
        direct.len() > 90,
        "only {} rows in a played game; the dump has probably changed shape",
        direct.len()
    );

    // **The awkward words are the ones a notation loses**, and they turned out to be the
    // names rather than the values: a table called *territory resource*, a column called
    // *force of nature*. My first version of this counted only values, found none, and
    // would have declared the quoting untested while three kinds of name depended on it.
    let awkward = direct
        .iter()
        .flat_map(|row| {
            std::iter::once(row.table.clone())
                .chain(row.fields.iter().flat_map(|(n, v)| [n.clone(), v.clone()]))
        })
        .filter(|word| word.contains(' ') || word.is_empty())
        .count();
    assert!(
        awkward > 2,
        "only {awkward} words here need quoting, so the quoting is barely tested"
    );
}

/// The comparison finds all three disagreements, and none where there are none.
#[test]
fn the_comparison_finds_missing_extra_and_different() {
    let session = played();
    let actual = expected::rows(&session.game);

    let same = expected::compare(&actual, &actual);
    assert_eq!(
        same.total(),
        0,
        "a state agrees with itself:\n{}",
        same.report()
    );

    // Different: change one value that is not the row's identity.
    //
    // **The first field *is* the identity**, so changing it makes the row a different row -
    // correctly reported as one missing and one extra. My first attempt changed `phase` on
    // the `game` row, which is its first field, and read the result as the comparison being
    // wrong rather than as it being right.
    let mut changed = actual.clone();
    let field = changed
        .iter_mut()
        .find_map(|row| row.fields.iter_mut().nth(1))
        .expect("a row with a second field");
    field.1 = format!("{}-nonsense", field.1);
    let wrong = expected::compare(&changed, &actual);
    assert_eq!(wrong.different.len(), 1, "{}", wrong.report());
    assert_eq!(
        wrong.missing.len() + wrong.extra.len(),
        0,
        "{}",
        wrong.report()
    );

    // Missing: expected a row that did not happen.
    let mut absent = actual.clone();
    absent.push(Row {
        table: "territory".to_string(),
        fields: vec![("territory".to_string(), "99".to_string())],
    });
    assert_eq!(
        expected::compare(&absent, &actual).missing.len(),
        1,
        "a row expected and not produced is missing"
    );

    // Extra: it happened and nobody expected it. **The direction ninety-six assertions
    // cannot have** - each is right about what it names, and names what somebody thought of.
    let mut fewer = actual.clone();
    fewer.pop();
    assert_eq!(
        expected::compare(&fewer, &actual).extra.len(),
        1,
        "a row produced and not expected is extra"
    );
}

/// The seeding branch, over a directory of its own.
///
/// **`P-225`: absence means acceptance**, so an update is a deletion rather than an edit -
/// deliberate, visible in `git status`, impossible by hand slip. This exercises that where
/// it is safe: a temporary directory, seeded from a state, then compared with it.
///
/// It is *not* run against `expected/play.4x`, and that is `P-227`.
#[test]
fn an_absent_expectation_is_seeded_and_then_compared() {
    let at = std::env::temp_dir().join("game4x-expected-seed");
    let _ = std::fs::remove_dir_all(&at);
    std::fs::create_dir_all(&at).expect("a directory to seed into");
    let file = at.join("play.4x");

    let session = played();
    let actual = expected::rows(&session.game);

    assert!(!file.exists(), "the point is that it is not there yet");
    std::fs::write(&file, expected::write(&session.game, "seeded")).expect("seeding writes it");

    let back = expected::read(&std::fs::read_to_string(&file).unwrap()).expect("parses");
    let wrong = expected::compare(&back, &actual);
    assert_eq!(
        wrong.total(),
        0,
        "a seed agrees with what seeded it:\n{}",
        wrong.report()
    );

    std::fs::remove_dir_all(&at).ok();
}

/// The reviewed expectation, once it exists.
///
/// Until `P-227` settles how a first expectation is made, this reports that there is none
/// rather than seeding one. **It says so out loud**: a test that skips quietly is the thing
/// this repository keeps finding, and the whole point of the file it waits for is that
/// somebody derived it rather than a program produced it.
#[test]
fn the_reviewed_expectation_holds() {
    let file = root().join(AT);
    let Ok(text) = std::fs::read_to_string(&file) else {
        println!(
            "no {AT} yet - the first expectation is derived by hand (`P-211`), not seeded \
             from the program (`P-227`). The mechanism is checked by the tests above."
        );
        return;
    };

    let want = expected::read(&text).unwrap_or_else(|why| panic!("{AT}: {why}"));
    assert!(!want.is_empty(), "{AT} exists and expects nothing");

    let session = played();
    let wrong = expected::compare(&want, &expected::rows(&session.game));
    assert_eq!(
        wrong.total(),
        0,
        "the scenario and {AT} disagree:\n{}\n\
         If the game is right and the expectation is out of date, delete the row - or the \
         file - rather than editing it. `docs/process.md`: absence means acceptance, and \
         what you review is the diff.",
        wrong.report()
    );
}
