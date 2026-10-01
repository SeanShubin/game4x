//! Every record of a reading is a record of reading something that is there.
//!
//! **A file in `reviewed/` says Sean has read the test of that name.** `examples/report.rs` looks
//! one up per test, so a record naming no test is walked by nobody and nothing notices it.
//!
//! # Untidy today, and a false approval tomorrow
//!
//! **Two records outlived their tests** - `a-transport-takes-two-berths-where-a-scout-takes-one`,
//! deleted in `4f89b719`, and `an-ark-gathers-an-energy-from-the-sun`. Neither cost an approval:
//! both successors carry records of their own, so what he read he read. **What it costs is ahead
//! of us.** A test given a name a stale record already holds opens as *reviewed* without anyone
//! having read a line of it - and the page would say so in the same words it uses when he has.
//!
//! **Nothing else in the suite can see it.** `review_of` is asked about each test in turn, so it
//! answers for the fifty-three that exist and is never asked about the records that answer to
//! nothing. **A check has to walk the other way**, which is the whole of this file.
//!
//! Found by the specification lane checking an argument of this lane's rather than accepting it -
//! `S-145`, and the argument was that `reviewed/` is the approval record an executable
//! specification would rest on. **A record that can drift is a poor thing to rest on**, which is
//! why this landed the day the argument was made rather than the day it mattered.

use std::collections::BTreeSet;
use std::path::Path;

/// **The comparison lives in the example and the gate borrows it**, so the page and the suite
/// cannot disagree about what drift is. `review-web.rs` includes the same file the same way.
///
/// **And `mod common` is not here, which is not a style choice.** It used to be that
/// `report.rs` reached `tests/common/friendly.rs` by path, so a file that loads both saw it
/// twice and clippy's `duplicate_mod` failed the gate, which runs `-D warnings`. **`R-12`
/// removed that hazard rather than this line working around it**: the translator is
/// `friendly-notation` now, a crate, and a crate cannot be included twice. **Nothing here needs
/// `common`** -
/// the two directories come from `report` itself, which is where they should have come from
/// anyway: `tests_at` and `records_at` exist so the move is spelled once.
#[path = "../examples/report.rs"]
#[allow(dead_code)]
mod report;

/// The stem of every `.4x` in a directory.
///
/// **Both of these left the prototype on 2026-09-21** - `P-532` put the tests in `spec/tests/`
/// and the records in `reviewed/`, at the repository root, where they are the specification and
/// the record of Sean having read it. **What this checks did not change**; only where it looks,
/// and it now asks `report` where rather than spelling it out a third time.
fn stems(at: &Path) -> BTreeSet<String> {
    let Ok(entries) = std::fs::read_dir(at) else {
        return BTreeSet::new();
    };
    entries
        .filter_map(|it| it.ok())
        .map(|it| it.path())
        .filter(|path| path.extension().and_then(|it| it.to_str()) == Some("4x"))
        .filter_map(|path| {
            path.file_stem()
                .and_then(|it| it.to_str())
                .map(str::to_string)
        })
        .collect()
}

#[test]
fn every_record_of_a_reading_names_a_test_that_is_there() {
    let tests = stems(&report::tests_at());
    let records = stems(&report::records_at());

    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. With no
    // records read, every record would name a test vacuously, and this would pass in the same
    // words it passes in now.
    assert!(
        tests.len() > 40,
        "only {} tests were found, so this is not about the suite",
        tests.len()
    );
    assert!(
        records.len() > 40,
        "only {} records were found, so this is not about the reading",
        records.len()
    );

    let orphaned: Vec<&String> = records.difference(&tests).collect();
    assert!(
        orphaned.is_empty(),
        "these say a test was read and there is no such test: {orphaned:?} - a test later given \
         one of those names would open as reviewed without anyone having read it"
    );
}

/// **A note asks for a change to a test, so it is addressed to one that exists.**
///
/// **The same failure with a quieter end.** An orphaned record claims a reading that did not
/// happen; an orphaned note asks for a change to nothing, and shows on no page - so it is a thing
/// Sean asked for that nobody will ever read.
#[test]
fn every_note_is_about_a_test_that_is_there() {
    let tests = stems(&report::tests_at());
    let Ok(text) = std::fs::read_to_string(report::records_at().join("asked.md")) else {
        return;
    };
    let named: Vec<String> = text
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .map(|name| name.trim().to_string())
        .collect();
    let orphaned: Vec<&String> = named.iter().filter(|it| !tests.contains(*it)).collect();
    assert!(
        orphaned.is_empty(),
        "these ask for a change to a test that is not there: {orphaned:?}"
    );
}

/// **What runs is what Sean read**, and this is the only thing that says so.
///
/// # What the suite proved before this, and it was less than it looked like
///
/// **`CLAUDE.md` says the suite runs `reviewed/`. It ran `spec/tests/`** - `S-149`, measured by
/// the specification lane on 2026-09-21. `reviewed/` was opened by one file, the check above,
/// and that one compares names. **So a test nobody had read ran exactly like one he had**, and
/// his reading changed what the gate did by nothing.
///
/// **It cost nothing while every test was read** - 54 records for 54 tests, the same bytes in
/// both directories - which is why it was invisible. It starts costing the first time this lane
/// drafts a test he has not seen, which is the next thing this lane does.
///
/// # This is a gate rather than a colour
///
/// **`examples/report.rs` already computed drift and showed it on a page**, where nothing stops a
/// build. The same function is called here, so a test that differs from its record fails, and it
/// fails with the lines that differ rather than with a name.
///
/// # Drifting is an error and being unread is not
///
/// **`CLAUDE.md`**: *a test nobody has read constrains nothing.* So an unread test is counted and
/// named here and fails nothing - `common::every_read_test` leaves it out of the run, which is
/// what *constrains nothing* means operationally.
///
/// **A test that drifted is the other case and it does fail.** He read it, so the engine is held
/// to it, and the bytes being held to are no longer the bytes he read. **That is the one state
/// the arrangement cannot carry**: what runs would be neither approved nor refused.
///
/// **This lane cannot fix it either way**, which is why it is red rather than repaired: putting
/// the record back is writing Sean's column, and he is the only one who can read the new version.
/// `scripts/review.sh` is the door.
#[test]
fn no_test_differs_from_what_sean_read() {
    let tests = stems(&report::tests_at());

    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. With no
    // tests found, every test would match its record vacuously.
    assert!(
        tests.len() > 40,
        "only {} tests were found, so this checked almost nothing",
        tests.len()
    );

    let mut unread: Vec<&String> = Vec::new();
    let mut drifted: Vec<String> = Vec::new();
    let mut compared = 0;
    for stem in &tests {
        let (status, lines) = report::review_of(stem);
        match status {
            "reviewed" => compared += 1,
            "never reviewed" => unread.push(stem),
            _ => {
                compared += 1;
                let said: Vec<&str> = lines.iter().map(|(_, it)| it.as_str()).collect();
                drifted.push(format!("{stem}\n      {}", said.join("\n      ")));
            }
        }
    }

    if !unread.is_empty() {
        println!(
            "{} of {} tests have no record and constrain nothing: {unread:?}",
            unread.len(),
            tests.len()
        );
    }
    // **A count over nothing again, and a different nothing.** Every test being unread would leave
    // nothing compared, and this would report no drift for the same reason a check over an empty
    // directory reports no failures.
    assert!(
        compared > 40,
        "only {compared} of {} tests have a record to compare against, so this said almost nothing",
        tests.len()
    );
    assert!(
        drifted.is_empty(),
        "{} of {compared} tests changed after they were read, so the engine is held to bytes that \
         are not the bytes that were approved - read them again with scripts/review.sh, or put \
         back what was approved:\n    {}",
        drifted.len(),
        drifted.join("\n    ")
    );
}

/// **The comparison notices what it is supposed to notice**, shown on text rather than on files.
///
/// **The gate above passes over fifty-four real tests, and that alone proves little** - a
/// comparison that answered *reviewed* for everything would pass it too. This is the half that
/// would fail if it did, and it writes nothing outside this lane: `spec/tests/` belongs to the
/// specification and `reviewed/` belongs to Sean, so demonstrating drift by editing one of them
/// is not available to this lane and is not needed.
///
/// **Three facts, because the gate reports three outcomes** and a reader acts on each
/// differently.
#[test]
fn the_comparison_tells_a_change_from_a_reformatting() {
    let read = "{test a}\n\n{given}\n{citizen at:home} -> 3\n";

    // **A changed quantity is drift**, and the lines say which row and which side it was on.
    let (status, lines) =
        report::drift(Some(read), "{test a}\n\n{given}\n{citizen at:home} -> 4\n");
    assert_eq!(status, "drifted");
    let said: Vec<&str> = lines.iter().map(|(_, it)| it.as_str()).collect();
    assert!(
        said.iter()
            .any(|it| it.contains("now:") && it.contains("-> 4")),
        "the row that changed is not named: {said:?}"
    );
    assert!(
        said.iter()
            .any(|it| it.contains("was:") && it.contains("-> 3")),
        "what was read is not named: {said:?}"
    );

    // **Whitespace is not drift**, which is why the gate and the page share one function: a gate
    // stricter than the display would call a test drifted on a page that says it is reviewed.
    let (status, _) = report::drift(
        Some(read),
        "{test   a}\n\n\n{given}\n  {citizen   at:home}  ->  3\n\n",
    );
    assert_eq!(status, "reviewed", "spacing is not a change to a test");

    // **No record is its own answer**, and not drift against an empty one.
    let (status, lines) = report::drift(None, read);
    assert_eq!(status, "never reviewed");
    assert!(lines.is_empty(), "nothing to show about a test nobody read");

    // **A row moved between sections is a change**, which is what carrying the section buys.
    let (status, _) = report::drift(
        Some(read),
        "{test a}\n\n{given}\n\n{then}\n{citizen at:home} -> 3\n",
    );
    assert_eq!(
        status, "drifted",
        "the same row in another section is a change"
    );
}

/// **Every reading reaches the suite that runs it, and everything the suite runs was read.**
///
/// # The gate was green while two approved tests did not run
///
/// **`S-215`.** Sean read `a-yard-is-built-from-labor-and-metal` and
/// `an-ark-cannot-launch-where-there-is-no-yard`, the application wrote both records, and
/// `cargo test --test first_test` stayed green over seven tests. **The two were not unread and
/// not red - they were absent.**
///
/// ```text
/// reviewed/                56   his readings
/// spec/tests/              56   the source
/// data/foundation/tests/   54   what the suite iterates
/// ```
///
/// **`every_read_test` asks which of the fifty-four have a record**, never which records have a
/// test to run, so a record with no generated form is in neither list it builds.
///
/// **And the guard beside it could not see this.** `first_test.rs` asserts `reading.len() > 40`,
/// which is a floor over the fifty-four: **a floor catches an empty population and not a short
/// one.** The check above compares `reviewed/` with `spec/tests/` and both are 56, so it passes
/// too - it asks whether a record names a test, never whether that test runs.
///
/// # Both directions, because either alone passes for the wrong reason
///
/// **A reading with no generated form is an approval that constrains nothing.** A generated form
/// with no reading is the opposite and worse: the suite holding the engine to something nobody
/// approved. **The first direction alone would pass over an empty suite; the second alone would
/// pass over a suite generated from the very records it is compared with** - which is what
/// `data/foundation/tests/` is, so that direction is nearly free and is asserted anyway, because
/// what makes it free is a generator that could stop being run.
#[test]
fn every_reading_reaches_the_suite_and_everything_the_suite_runs_was_read() {
    let records = stems(&report::records_at());
    let running = stems(&report::foundation_tests_at());

    assert!(
        records.len() > 40,
        "only {} records were found, so this is not about the reading",
        records.len()
    );
    assert!(
        running.len() > 40,
        "only {} tests are generated, so this is not about the suite",
        running.len()
    );

    let not_running: Vec<&String> = records.difference(&running).collect();
    assert!(
        not_running.is_empty(),
        "{} test(s) have been read and the suite does not run them: {not_running:?}\n\
         Run `cargo run -p game-model --example foundation` to generate their foundation form. \
         An approval that constrains nothing is worse than no approval, because the record says \
         it does.",
        not_running.len()
    );

    let unread: Vec<&String> = running.difference(&records).collect();
    assert!(
        unread.is_empty(),
        "{} test(s) the suite runs have no record of a reading: {unread:?} - the engine is held \
         to something nobody approved",
        unread.len()
    );

    // **The count, so the two sets being equal is equality of a population rather than of two
    // empties** - and the floors above are what stop it being short rather than absent.
    assert_eq!(
        records.len(),
        running.len(),
        "the readings and the suite are different sizes and neither difference named it"
    );
}

/// **Column order is not behaviour, so renumbering a `seq:` clears no verdict.**
///
/// `P-606`, Sean's answer and a third option neither lane offered: *the order of the columns is not
/// significant, so this should not make tests different, although we should be deterministic about
/// them either way.*
///
/// # The trap this closes, and which side had to close it
///
/// **The writer emits rows in the schema's declared order**, and a `seq:` is editable with no
/// meaning beyond order - so renumbering one changes every record's bytes. **If the comparison read
/// written bytes, that tidy-up would send all fifty-seven tests back to him.**
///
/// **It is loud rather than silent** - the verdicts clear and he notices - **which is why it was a
/// trap for a later session rather than a defect now.** The specification lane named it; the
/// requirement that followed is that the normalising has to be in the comparison, because the
/// writer cannot normalise away an order it is choosing.
///
/// **Driven over text rather than over the schema**, because renumbering a real `seq:` would edit
/// `spec/data/`, which is not this lane's column - and the property is about the comparison, not
/// about any particular column order.
#[test]
fn a_reordered_row_is_the_same_row_and_a_changed_value_is_not() {
    let read = "{test name:x}\n\n{given}\n{citizen where:place-1 hungry:1 bearing:1} -> 2\n";

    // **The same row with its columns in another order.** This is what a `seq:` renumber produces.
    let reordered = "{test name:x}\n\n{given}\n{citizen bearing:1 hungry:1 where:place-1} -> 2\n";
    assert_eq!(report::drift(Some(read), reordered).0, "reviewed");

    // **And alphabetical order, which is what `P-606` might have meant and now cannot matter.**
    let alphabetical =
        "{test name:x}\n\n{given}\n{citizen bearing:1 hungry:1 where:place-1} -> 2\n";
    assert_eq!(report::drift(Some(read), alphabetical).0, "reviewed");

    // **A reworded comment survives too** - `P-600`: *a comment explains and does not decide.*
    let recommented = "# said another way\n{test name:x}\n\n{given}\n{citizen where:place-1 hungry:1 bearing:1} -> 2\n";
    assert_eq!(report::drift(Some(read), recommented).0, "reviewed");

    // **Two entries of one description are one quantified row**, which is rule 3's coalescing.
    let split = "{test name:x}\n\n{given}\n{citizen where:place-1 hungry:1 bearing:1} -> 1\n{citizen where:place-1 hungry:1 bearing:1} -> 1\n";
    assert_eq!(report::drift(Some(read), split).0, "reviewed");

    // **And the comparison still bites.** A changed value, a changed quantity, a row moved to
    // another section, and a row removed are each a different behaviour.
    for (what, now) in [
        (
            "a changed value",
            "{test name:x}\n\n{given}\n{citizen where:place-1 hungry:0 bearing:1} -> 2\n",
        ),
        (
            "a changed quantity",
            "{test name:x}\n\n{given}\n{citizen where:place-1 hungry:1 bearing:1} -> 3\n",
        ),
        (
            "a row in another section",
            "{test name:x}\n\n{then}\n{citizen where:place-1 hungry:1 bearing:1} -> 2\n",
        ),
        ("a row removed", "{test name:x}\n\n{given}\n"),
        (
            "a column added",
            "{test name:x}\n\n{given}\n{citizen where:place-1 hungry:1 bearing:1 laboring:1} -> 2\n",
        ),
    ] {
        assert_eq!(
            report::drift(Some(read), now).0,
            "drifted",
            "{what} is a different behaviour and this did not say so"
        );
    }
}
