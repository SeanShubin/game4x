//! What `decide/attention.md` promises, asserted over every case rather than shown on one.
//!
//! **`decide/README.md` makes one promise and it is checkable**: *a thing that waits on me and is
//! not in it is a defect in whatever writes it.* So the test below is over **every** open item
//! addressed to Sean, with the count asserted - a test that shows the rule on one item stops
//! showing anything the moment that item is closed, and goes on passing.
//!
//! **`S-225`**, from `P-593`.

use std::path::{Path, PathBuf};

use outbox::{Reading, Unreading, attention, open_by_addressee, read, reading};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .expect("the repository root")
}

/// **Every item open to Sean is named, and the population is asserted.**
///
/// This is the whole of `decide/README.md`'s promise. **The count is what tells a real pass from
/// a pass over nothing** - with no item addressed to him, a loop over zero items would say the
/// rule holds in exactly these words.
#[test]
fn every_item_waiting_on_sean_is_in_the_file() {
    let all = read(&root());
    let empty = Vec::new();
    let grouped = open_by_addressee(&all.items);
    let his = grouped.get("sean").unwrap_or(&empty);
    let written = attention(&all, &reading(&root()));

    let missing: Vec<&str> = his
        .iter()
        .filter(|item| !written.contains(&format!("**{}**", item.id)))
        .map(|item| item.id.as_str())
        .collect();
    assert!(
        missing.is_empty(),
        "open and addressed to Sean, and not in attention.md: {missing:?} - \
         `decide/README.md` says that is a defect in this generator"
    );
    assert!(
        !his.is_empty(),
        "nothing is open to Sean, so the check above ran over no items and proved nothing - \
         when this is the true state, delete this assertion rather than letting the one above \
         pass over nothing"
    );
}

/// **Nothing that is not his gets in**, which is the other half and the one that makes the file
/// worth opening.
///
/// `decide/README.md`: *nothing addressed to another perspective, nothing already settled.*
#[test]
fn nothing_addressed_to_an_instance_is_in_the_file() {
    let all = read(&root());
    let written = attention(&all, &reading(&root()));
    let mut checked = 0;
    let leaked: Vec<&str> = all
        .items
        .iter()
        .filter(|item| item.to != "sean")
        .inspect(|_| checked += 1)
        .filter(|item| written.contains(&format!("**{}**", item.id)))
        .map(|item| item.id.as_str())
        .collect();
    assert!(
        leaked.is_empty(),
        "addressed to an instance and in the file a person opens: {leaked:?}"
    );
    assert!(checked > 50, "only {checked} item(s) were checked against");
}

/// **A test with no record is named, and so is one whose record differs.**
///
/// Both cases at once, over a directory built for it, because the repository's own two
/// directories agree today - and a check that can only pass by the tree being in one state says
/// nothing about the state it exists to report.
#[test]
fn a_test_waiting_on_a_reading_is_named_either_way() {
    let at = temporary("waiting");
    // **A record is a verdict since `P-605`, not a copy.** This wrote `changed.4x` in two
    // spellings and expected a `RecordDiffers` - the comparison that is gone, because
    // `spec/README.md` rule 3 says *an approval survives a change that does not change the
    // behaviour*, and a byte comparison reported a reworded comment as a reading owed.
    //
    // **The two states left are the two a reader can see**: no record at all, and a record
    // saying `denied`. Whether two tests say the same thing needs folding and is the engine's.
    write(&at, "spec/tests/read.4x", "{test name:read}\n");
    write(
        &at,
        "reviewed/read.4x",
        "{verdict state:approved}\n{test name:read}\n",
    );
    write(&at, "spec/tests/never-read.4x", "{test name:never}\n");
    write(
        &at,
        "spec/tests/turned-down.4x",
        "{test name:turned-down}\n",
    );
    write(
        &at,
        "reviewed/turned-down.4x",
        "{verdict state:denied}\n{test name:turned-down}\n",
    );

    let Reading::Compared {
        tests,
        records,
        waiting,
    } = reading(&at)
    else {
        panic!("both directories hold something, so this is a comparison");
    };
    assert_eq!((tests, records), (3, 2));
    assert_eq!(
        waiting.len(),
        2,
        "two of the three are waiting: {waiting:?}"
    );
    assert_eq!(waiting[0].name, "never-read.4x");
    assert_eq!(waiting[0].why, Unreading::NoRecord);
    assert_eq!(waiting[1].name, "turned-down.4x");
    assert_eq!(waiting[1].why, Unreading::Denied);

    // **A record with no verdict reads as approved**, which is every record written before
    // `P-605`: the fifty-seven in the tree have none.
    write(&at, "reviewed/read.4x", "{test name:read}\n");
    let Reading::Compared { waiting, .. } = reading(&at) else {
        panic!("a comparison");
    };
    assert_eq!(
        waiting.len(),
        2,
        "the old shape is still approved: {waiting:?}"
    );

    // **And a state nobody defined is blind rather than approved or denied**, because guessing
    // either way decides something about a test on a word with no meaning.
    write(
        &at,
        "reviewed/read.4x",
        "{verdict state:maybe}\n{test name:read}\n",
    );
    assert!(matches!(reading(&at), Reading::Blind(_)));
}

/// **An empty directory is blind rather than good news**, which is the failure with the sign
/// flipped.
///
/// `CLAUDE.md`: *a count over nothing is the same failure with the sign flipped - zero
/// occurrences proves something only against a population that is not also zero.* **Both
/// populations, because they fail in opposite directions**: no tests would report no reading owed
/// while orphaning every record, and no records would report every test as unread.
#[test]
fn an_empty_directory_is_blind_rather_than_nothing_waiting() {
    let no_tests = temporary("no-tests");
    write(&no_tests, "spec/tests/.keep", "");
    write(&no_tests, "reviewed/read.4x", "{test name:read}\n");
    let Reading::Blind(why) = reading(&no_tests) else {
        panic!("`spec/tests/` holds no test, so nothing can be concluded");
    };
    assert!(why.contains("spec/tests/"), "{why}");

    let no_records = temporary("no-records");
    write(&no_records, "spec/tests/read.4x", "{test name:read}\n");
    write(&no_records, "reviewed/.keep", "");
    let Reading::Blind(why) = reading(&no_records) else {
        panic!("`reviewed/` holds no record, so every test would be reported unread");
    };
    assert!(why.contains("reviewed/"), "{why}");

    // **And a missing directory is blind too, rather than an empty answer.** A repository
    // without `spec/tests/` at all is the state a bad path produces, and it must not read as
    // him being up to date.
    let neither = temporary("neither");
    write(&neither, "README.md", "nothing here\n");
    assert!(matches!(reading(&neither), Reading::Blind(_)));
}

/// **Both outcomes say which they are, in the file itself.**
///
/// The generator's words are what he reads, and *nothing is waiting* and *this could not be
/// derived* must not be the same sentence. That is exactly the confusion the `Blind` case exists
/// to prevent, so it is asserted in the rendering rather than only in the value.
#[test]
fn the_file_tells_nothing_waiting_apart_from_could_not_be_derived() {
    let all = read(&root());
    let up_to_date = attention(
        &all,
        &Reading::Compared {
            tests: 57,
            records: 57,
            waiting: Vec::new(),
        },
    );
    assert!(
        up_to_date.contains("Nothing. All 57 test(s)"),
        "{up_to_date}"
    );

    let blind = attention(&all, &Reading::Blind("the directory is gone".to_string()));
    assert!(blind.contains("could not be derived"), "{blind}");
    assert!(
        !blind.contains("Nothing. All"),
        "a blind derivation must not read as being up to date"
    );
}

/// **Each of the three gestures has a heading whether or not it has anything**, because a
/// heading that disappears when it is empty reads as a file that forgot it.
///
/// `decide/README.md`: *empty means nothing is waiting. A file here that says nothing is open is
/// the whole report.*
#[test]
fn every_gesture_has_a_heading_even_with_nothing_under_it() {
    let written = attention(&read(&root()), &reading(&root()));
    let mut found = 0;
    for heading in [
        "## Approve words",
        "## Answer a question",
        "## Vet a capability",
        "## Read a test",
        "## Accept a regression case",
    ] {
        assert!(written.contains(heading), "no `{heading}`");
        found += 1;
    }
    assert_eq!(found, 5, "five kinds of thing can wait on him");
}

/// **Padding the generated file changes nothing**, which `CLAUDE.md` requires of every generated
/// file and this one makes true the cheap way: it has no table.
///
/// *A generated file is written in the form the padder would leave it... what is required is
/// that padding a generated file changes nothing, and that a check says so.*
#[test]
fn the_generated_file_is_in_the_form_the_padder_would_leave_it() {
    let written = attention(&read(&root()), &reading(&root()));
    let rows: Vec<&str> = written
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
        .collect();
    assert!(
        rows.is_empty(),
        "this file has no table, so `pad-tables` has nothing to rewrite: {rows:?}"
    );
    assert!(written.lines().count() > 20, "and it is not empty");
}

/// A directory of this test's own, under the crate's target.
fn temporary(named: &str) -> PathBuf {
    let at = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/attention")
        .join(named);
    let _ = std::fs::remove_dir_all(&at);
    std::fs::create_dir_all(&at).expect("a directory to work in");
    at
}

fn write(root: &Path, at: &str, said: &str) {
    let path = root.join(at);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("the directory");
    std::fs::write(&path, said).expect("the file");
}
