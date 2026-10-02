//! The remote review surface: what a tick means, and what it may not do.
//!
//! `S-228`, from Sean: *only I can say a test is approved and only I may say the current
//! regression expectation should be deleted, but I also want to do this remotely with a button
//! press.*
//!
//! **The thing being checked is that the list cannot lie about `reviewed/`.** It is a rendering
//! of that directory, so a tick means a record exists and nothing else - and if the rendering
//! and the reading of it ever disagree, a tick he did not make becomes an approval.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use outbox::{Gesture, carry_out, gestures, review_issue, ticked_in};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .expect("the repository root")
}

fn at(named: &str) -> PathBuf {
    let at = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/review")
        .join(named);
    let _ = std::fs::remove_dir_all(&at);
    std::fs::create_dir_all(at.join("spec/tests")).expect("spec/tests");
    std::fs::create_dir_all(at.join("reviewed")).expect("reviewed");
    at
}

fn put(root: &Path, rel: &str, said: &str) {
    std::fs::write(root.join(rel), said).expect("the file");
}

/// **The body and the directory say the same thing, over every test.**
///
/// This is the whole guarantee. The issue is regenerated from `reviewed/` and read back by
/// [`ticked_in`], so a mismatch between the two is a tick nobody made - and it would read as
/// an approval of a test Sean never opened.
///
/// **Round-tripped over the repository's own 57 rather than over an example**, with the count
/// asserted, because a body that rendered no rows at all would agree with an empty set.
#[test]
fn the_issue_round_trips_against_the_records() {
    let root = root();
    let body = review_issue(&root);
    let read_back = ticked_in(&body);

    // **Walked one level, because `reviewed/` holds suites** - `rule/` since 2026-10-01, and
    // `interface/` when there is one. **This read the directory flat and found nothing**, which
    // the floor below caught rather than letting an empty set agree with an empty set.
    let mut records: BTreeSet<String> = BTreeSet::new();
    for suite in std::fs::read_dir(root.join("reviewed"))
        .expect("reviewed/")
        .flatten()
    {
        let named = suite.file_name().to_string_lossy().to_string();
        if named.ends_with(".4x") {
            records.insert(named);
            continue;
        }
        if suite.path().is_dir() {
            for inner in std::fs::read_dir(suite.path()).expect("a suite").flatten() {
                let file = inner.file_name().to_string_lossy().to_string();
                if file.ends_with(".4x") {
                    records.insert(format!("{named}/{file}"));
                }
            }
        }
    }

    assert!(
        records.len() >= 40,
        "only {} record(s); this round-trip would prove little",
        records.len()
    );
    assert_eq!(
        read_back, records,
        "the body and `reviewed/` disagree about what has been read"
    );
    // **A row per test, not per record**, which is the point of the issue: a test he has not read
    // has no record and still needs a line he can tick.
    //
    // **This compared the rows to the records and passed only while every test had one.** On
    // 2026-10-02 he approved two of the four biome tests and left two; the body rendered its
    // correct 63 and `reviewed/` held 59, and the assertion read that as a defect. **The thing it
    // was for is the equality above** - the ticks and the records agree - and this half was
    // counting the wrong population.
    let mut tests: BTreeSet<String> = BTreeSet::new();
    for suite in std::fs::read_dir(root.join("spec/tests"))
        .expect("spec/tests/")
        .flatten()
    {
        if !suite.path().is_dir() {
            continue;
        }
        for file in std::fs::read_dir(suite.path()).expect("a suite").flatten() {
            let name = file.file_name().to_string_lossy().to_string();
            if name.ends_with(".4x") {
                tests.insert(name);
            }
        }
    }
    assert!(tests.len() >= records.len(), "more records than tests");
    assert_eq!(
        body.lines().filter(|l| l.starts_with("- [")).count(),
        tests.len(),
        "a row per test, so a test he has not read still has a line to tick"
    );
    // **And the unread are named rather than counted**, because *four of sixty-three* sends a
    // reader looking for which.
    let unread: Vec<&String> = tests
        .iter()
        .filter(|name| !records.contains(*name))
        .collect();
    println!("{} test(s) have no record: {unread:?}", unread.len());
}

/// **A tick with no record approves, an untick with one withdraws, and agreement does nothing.**
///
/// All three in one world, because the interesting thing is the difference between the two
/// sets rather than any single row.
#[test]
fn a_tick_approves_an_untick_withdraws_and_agreement_is_quiet() {
    let root = at("three-ways");
    put(&root, "spec/tests/wants-approving.4x", "{test name:a}\n");
    put(&root, "spec/tests/wants-withdrawing.4x", "{test name:b}\n");
    put(&root, "reviewed/wants-withdrawing.4x", "{test name:b}\n");
    put(&root, "spec/tests/already-read.4x", "{test name:c}\n");
    put(&root, "reviewed/already-read.4x", "{test name:c}\n");

    let ticked: BTreeSet<String> = ["wants-approving.4x", "already-read.4x"]
        .iter()
        .map(|it| it.to_string())
        .collect();
    let wanted = gestures(&root, &ticked);
    assert_eq!(
        wanted,
        vec![
            Gesture::Approve("wants-approving.4x".to_string()),
            Gesture::Withdraw("wants-withdrawing.4x".to_string()),
        ],
        "already-read is in both and asks for nothing"
    );
}

/// **A tick copies the bytes and changes none of them.**
///
/// `S-228`: *nothing reformats a copy. `reviewed/` records that he read those bytes, so a
/// writer that tidied whitespace would be recording a reading of something else.* **The test
/// file here is deliberately untidy** - trailing spaces, a tab, no final newline - so a
/// normalising writer fails rather than passes by the input being already tidy.
#[test]
fn approving_copies_the_bytes_exactly() {
    let root = at("bytes");
    let untidy = "{test name:x}  \n\t{given}\n\n\n{then} no final newline";
    put(&root, "spec/tests/untidy.4x", untidy);

    carry_out(&root, &Gesture::Approve("untidy.4x".to_string())).expect("the copy");
    let written = std::fs::read_to_string(root.join("reviewed/untidy.4x")).expect("the record");
    assert_eq!(written, untidy, "a record is the bytes he read");
}

/// **A ticked row naming no test writes nothing.**
///
/// That is the orphan case from the other side: a record whose test was renamed away renders
/// as a ticked row, and ticking it again must not create a record of a file that is not there.
/// **A record of nothing is worse than no record**, because it reads as a reading.
#[test]
fn a_tick_naming_no_test_is_ignored() {
    let root = at("orphan");
    put(&root, "spec/tests/real.4x", "{test name:real}\n");
    put(&root, "reviewed/renamed-away.4x", "{test name:gone}\n");

    let ticked: BTreeSet<String> = ["real.4x", "renamed-away.4x", "never-existed.4x"]
        .iter()
        .map(|it| it.to_string())
        .collect();
    assert_eq!(
        gestures(&root, &ticked),
        vec![Gesture::Approve("real.4x".to_string())],
        "only a test that exists can be approved"
    );
    // **And the record it answers to is left alone**, rather than being deleted for having no
    // test. Removing it is a reading Sean takes back, not housekeeping.
    assert!(root.join("reviewed/renamed-away.4x").is_file());
}

/// **An orphaned record is shown rather than dropped from the list.**
///
/// The review application's own rule, which `CLAUDE.md` states: a rename leaves an orphaned
/// record and an unread test, *which is two things he can see and act on rather than one thing
/// nobody may touch*.
#[test]
fn an_orphaned_record_is_named_in_the_body() {
    let root = at("shown");
    put(&root, "spec/tests/here.4x", "{test name:here}\n");
    put(&root, "reviewed/here.4x", "{test name:here}\n");
    put(&root, "reviewed/gone.4x", "{test name:gone}\n");

    let body = review_issue(&root);
    assert!(body.contains("Records with no test"), "{body}");
    assert!(body.contains("`gone.4x`"), "{body}");
    assert!(body.contains("1 of 1 read."), "{body}");
}

/// **Both spellings of a tick are read, and an unticked row is not.**
///
/// GitHub writes `- [x]` and some clients write `- [X]`. **A reader that saw one and not the
/// other would treat his tick as an untick and delete the record** - which is the one failure
/// here that destroys something rather than failing to create it.
#[test]
fn either_spelling_of_a_tick_counts_and_a_blank_does_not() {
    let body = "\
- [x] [lower](../blob/master/spec/tests/lower.4x)
- [X] [upper](../blob/master/spec/tests/upper.4x)
- [ ] [blank](../blob/master/spec/tests/blank.4x)
";
    let read = ticked_in(body);
    assert_eq!(
        read,
        ["lower.4x", "upper.4x"]
            .iter()
            .map(|it| it.to_string())
            .collect::<BTreeSet<String>>()
    );
}
