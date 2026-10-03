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
/// Every test or record under a directory of suites, as `suite/name` without the extension.
///
/// **A thin wrapper on `render::under`, which is the one enumeration** - `S-256`. This held the
/// fifth copy of *list the `.4x` files here* and read the directory flat, so `reviewed/interface/`
/// and `spec/tests/interface/` were invisible to every comparison below.
///
/// **The contract is unchanged**: a set, and stems rather than names - so every caller is
/// untouched and the stems are qualified now, which is what makes
/// `records_at().join(format!("{stem}.4x"))` still land on the right file.
fn stems(at: &Path) -> BTreeSet<String> {
    report::render::under(at)
        .into_iter()
        .map(|name| name.trim_end_matches(".4x").to_string())
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
        // **Drift is the first state with colour on it, not a third branch** - `P-611`. A test
        // whose rows changed is one he has not looked at in its present form, and `earlier` says
        // he looked at an earlier one. **So this reports the two separately and counts them as
        // one state**, which is the distinction the rule draws.
        let got = report::review_of(stem);
        match (got.state, got.earlier) {
            (report::APPROVED, _) | (report::DENIED, _) => compared += 1,
            (_, Some(_)) => {
                compared += 1;
                let said: Vec<&str> = got.lines.iter().map(|(_, it)| it.as_str()).collect();
                drifted.push(format!("{stem}\n      {}", said.join("\n      ")));
            }
            (_, None) => unread.push(stem),
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
    // **Printed, not asserted** - a drifted test is unread, and an unread test does not fail the
    // build. The lines that differ travel with it, because *two drifted* sends a reader looking
    // for which and for what.
    if !drifted.is_empty() {
        println!(
            "{} of {compared} test(s) changed after they were read, so their verdicts are cleared              and the engine still runs what was approved - read them again with              scripts/review.sh:
    {}",
            drifted.len(),
            drifted.join("
    ")
        );
    }
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

    // **The verdict, because a reading is no longer one thing** - `P-605`, and `X-46`. This compared
    // *every* record against the generated forms, which was right while a record meant *this
    // binds*; **a denied record is read and deliberately not run**, so the set that must reach the
    // suite is the approved one.
    //
    // **The assertion predated there being a third state and nothing re-read it.** The verdict had
    // reached every other reader in this file - `report::state_of`, the drift notice, the record
    // writer - and `E-2` could not be vetted without reddening the gate: *denying a test* was the
    // gesture, and denying a test broke the build the gesture is observed in.
    let mut approved: BTreeSet<String> = BTreeSet::new();
    let mut denied: BTreeSet<String> = BTreeSet::new();
    for name in &records {
        let at = report::records_at().join(format!("{name}.4x"));
        let said = std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("{name}: {why}"));
        match report::render::verdict_of(&said) {
            Ok(report::render::Verdict::Approved) => approved.insert(name.clone()),
            Ok(report::render::Verdict::Denied) => denied.insert(name.clone()),
            Err(why) => panic!("reviewed/{name}: {why}"),
        };
    }

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
    assert_eq!(
        approved.len() + denied.len(),
        records.len(),
        "a record carried neither verdict"
    );

    // **An approval that does not reach the suite.** This is the half the old assertion was right
    // about, said over the population it was always meant to be about.
    let not_running: Vec<&String> = approved.difference(&running).collect();
    assert!(
        not_running.is_empty(),
        "{} approved test(s) the suite does not run: {not_running:?}\n\
         Run `cargo run -p game-model --example foundation` to generate their foundation form. \
         An approval that constrains nothing is worse than no approval, because the record says \
         it does.",
        not_running.len()
    );

    // **A denial that still runs, which nothing checked and is the worse of the two.** `P-605`: a
    // test he has denied constrains nothing - so a generated form for one means **the engine is
    // held to something he has looked at and rejected**, which is further from his intent than
    // running something he never saw.
    let still_running: Vec<&String> = denied.intersection(&running).collect();
    assert!(
        still_running.is_empty(),
        "{} denied test(s) the suite still runs: {still_running:?} - the engine is held to \
         something he looked at and refused. Run the generator; it removes their forms.",
        still_running.len()
    );

    let unread: Vec<&String> = running.difference(&records).collect();
    assert!(
        unread.is_empty(),
        "{} test(s) the suite runs have no record of a reading: {unread:?} - the engine is held \
         to something nobody approved",
        unread.len()
    );

    // **The count over the approved population**, so the sets being equal is equality of a
    // population rather than of two empties - and the floors above are what stop it being short
    // rather than absent. **It was `records.len()` and that is what a denial broke.**
    assert_eq!(
        approved.len(),
        running.len(),
        "the approvals and the suite are different sizes and neither difference named it"
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

/// **Every state the code can reach is one of the three the rule names** - `P-611`, `S-241`.
///
/// `spec/README.md` rule 3: *there are three states and no others, for a test and for a case
/// alike. I have not looked at it; I have looked and approved it; I have looked and know it is
/// wrong.*
///
/// # Why this is built, when neither item asked for it
///
/// **A fourth state shipped and ran for days.** `review_of` returned `drifted`, the page rendered
/// it as a badge beside the other three, and `x` wrote a note the rule no longer treats as a
/// state. **Nothing compared what the page shows to what the rule says**, which is why it got that
/// far - and Sean found it by counting marks by hand, having said *three states* twice.
///
/// **So this asks the rule rather than pinning the output.** A check asserting the exact badges the
/// page emits today would be the strongest possible statement about what it does and would say
/// nothing about what it owes - and `CLAUDE.md` names that distinction: *what tells the two apart
/// is what the assertion names: the output, or the rule the output owes.*
///
/// # Driven, not counted
///
/// **Every record on disk says `approved`**, so a check that counted the states the page shows
/// would find one of three and pass. The states are driven over the combinations that can produce
/// them, and the count of combinations is asserted.
#[test]
fn every_state_the_code_can_reach_is_one_the_rule_names() {
    let at = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("spec")
        .join("README.md");
    let rule = std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));
    // **Collapsed on both sides**, because the rule is wrapped prose and a sentence drafted on
    // one line matches nothing in it - `CLAUDE.md`, and the reason `tools/anchor` exists.
    let said: String = rule.split_whitespace().collect::<Vec<_>>().join(" ");

    // **The rule's own number, so promoting a fourth state turns this red.** The literal is the
    // carrier: a sentence saying *four* no longer matches, and somebody has to look rather than
    // discovering it from a page.
    assert!(
        said.contains("There are three states and no others"),
        "rule 3 no longer says how many states there are, so this check has lost its subject"
    );
    assert_eq!(
        report::STATES.len(),
        3,
        "the code names a number of states the rule does not"
    );

    // **The three as the rule words them**, so a renamed state is caught here and not by a page
    // showing a word nobody recognises.
    for said in [
        "I have not looked at",
        "I have looked and approved it",
        "I have looked and know it is wrong",
    ] {
        assert!(said.contains(said), "rule 3 no longer says `{said}`");
    }

    // **Every combination that can produce a state**, over texts this test owns. A record is
    // absent, approved or denied; the rows either match the test or do not.
    let test = "{test name:x}\n\n{given}\n{place name:here}\n\n{when}\n{move}\n";
    let moved = "{test name:x}\n\n{given}\n{place name:there}\n\n{when}\n{move}\n";
    let approved = format!("{{verdict state:approved}}\n{test}");
    let denied = format!("{{verdict state:denied}}\n{test}");

    // **Named, because a five-tuple is a row nobody can read back.** Each is one combination
    // that can produce a state: what the record says, and whether the rows still match.
    struct Combination {
        what: &'static str,
        record: Option<String>,
        now: &'static str,
        state: &'static str,
        earlier: Option<bool>,
    }
    let cases = [
        Combination {
            what: "no record",
            record: None,
            now: test,
            state: report::NOT_LOOKED,
            earlier: None,
        },
        Combination {
            what: "approved, matching",
            record: Some(approved.clone()),
            now: test,
            state: report::APPROVED,
            earlier: None,
        },
        Combination {
            what: "denied, matching",
            record: Some(denied.clone()),
            now: test,
            state: report::DENIED,
            earlier: None,
        },
        // **Drift is the first state and keeps its colour** - *a test whose rows have changed
        // since I read it is in the first state, because somebody edited it and my approval was
        // of what it said.*
        Combination {
            what: "approved, drifted",
            record: Some(approved),
            now: moved,
            state: report::NOT_LOOKED,
            earlier: Some(false),
        },
        Combination {
            what: "denied, drifted",
            record: Some(denied),
            now: moved,
            state: report::NOT_LOOKED,
            earlier: Some(true),
        },
    ];
    assert_eq!(
        cases.len(),
        5,
        "three record states by whether the rows moved"
    );

    let mut reached = std::collections::BTreeSet::new();
    for one in &cases {
        let got = report::state_of(one.record.as_deref(), one.now);
        assert!(
            report::STATES.contains(&got.state),
            "`{}` reached `{}`, which rule 3 does not name",
            one.what,
            got.state
        );
        assert_eq!(got.state, one.state, "`{}` is in the wrong state", one.what);
        assert_eq!(
            got.earlier, one.earlier,
            "`{}` carries the wrong colour",
            one.what
        );
        reached.insert(got.state);
    }

    // **All three are reachable**, or this would pass over a state nothing can produce.
    assert_eq!(
        reached.len(),
        3,
        "only {} of the three states is reachable: {reached:?}",
        reached.len()
    );

    // **And the page renders whatever this returns**, which is the half that was wrong: the
    // fourth state was visible as a badge. Every badge the page can show for a state is one of
    // the three, and the drift is a separate attribute rather than a fourth word.
    // **Every page the application writes, not just the index** - `E-6` moved the cards onto a
    // page per category, so the index carries category lines and no marks at all. **The check said
    // *no state is rendered* and was right**, which is the floor doing its job rather than the
    // restructure breaking it.
    let built = report::build(true);
    let page: String = std::iter::once(built.page)
        .chain(built.pages.into_iter().map(|(_, it)| it))
        .collect::<Vec<String>>()
        .join(
            "
",
        );
    let shown: std::collections::BTreeSet<&str> = page
        .split("data-mark>")
        .skip(1)
        .filter_map(|rest| rest.split('<').next())
        .collect();
    assert!(
        !shown.is_empty(),
        "no state is rendered, so nothing is checked"
    );
    for one in &shown {
        assert!(
            report::STATES.contains(one),
            "the page shows `{one}`, which rule 3 does not name"
        );
    }
}

/// **The suites on disk are the two `spec/tests/README.md` names, and `reviewed/` mirrors them.**
///
/// `P-617` settled the interface test's form and `S-256` made everything walk both suites. **The
/// list is stated in two places - `spec/tests/README.md` in prose and `render::SUITES` in code -
/// and this is what stops them drifting.**
///
/// # Why a list at all, when `render::under` discovers the directories
///
/// **Discovery is what makes a test visible and says nothing about what ought to be there.** A
/// third suite appearing is a thing somebody decided; a suite disappearing is a thing nobody
/// decided. **`under` would report either without comment**, which is the shape of a check that
/// pins the present state: it reads the outcome where the question is the rule.
///
/// **And `reviewed/` mirroring `spec/tests/` is the answer to the question `S-256` left open.** A
/// record sits at the same path under `reviewed/` as its test does under `spec/tests/`, which is
/// what makes `records_at().join(name)` right for a name that carries its suite - **the page's
/// write path and the drift check's read path are then one expression rather than two that agree.**
#[test]
fn the_suites_on_disk_are_the_ones_that_are_stated() {
    let at = report::tests_at();
    let mut found: Vec<String> = std::fs::read_dir(&at)
        .unwrap_or_else(|why| panic!("{}: {why}", at.display()))
        .flatten()
        .filter(|it| it.path().is_dir())
        .map(|it| it.file_name().to_string_lossy().to_string())
        .collect();
    found.sort();
    let mut wanted: Vec<String> = report::SUITES.iter().map(|it| it.to_string()).collect();
    wanted.sort();
    assert_eq!(
        found, wanted,
        "the suites under spec/tests/ and the ones render::SUITES names have drifted"
    );

    // **And the prose says the same**, which is where the list is for a reader.
    let said = std::fs::read_to_string(at.join("README.md")).expect("spec/tests/README.md");
    for suite in report::SUITES {
        assert!(
            said.contains(suite),
            "spec/tests/README.md does not name the `{suite}` suite"
        );
    }

    // **`reviewed/` holds no suite that is not a test suite.** The other direction is allowed: a
    // suite nobody has approved a test in has no directory, which is `interface/` today.
    for suite in report::render::under(&report::records_at())
        .iter()
        .filter_map(|name| name.split('/').next().map(str::to_string))
        .filter(|it| !it.ends_with(".4x"))
    {
        assert!(
            report::SUITES.contains(&suite.as_str()),
            "`reviewed/{suite}/` is not a suite of spec/tests/"
        );
    }
}
