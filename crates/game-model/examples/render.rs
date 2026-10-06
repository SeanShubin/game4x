//! Write `data/foundation/` from the friendly source, which is what `tests/directories.rs` asserts.
//!
//! **The two forms hold the same rows in two spellings, and one is generated from the other.**
//! Sean, 2026-09-15: *Lets make friendly the source and not omit anything. This presumes we can
//! reliably convert between friendly and foundation.* So the friendly side is authored and this
//! side is produced, and `tests/directories.rs` is what says they agree.
//!
//! # It ran the other way until `P-576`, and that was the wrong way
//!
//! **This wrote `data/friendly/` from `data/foundation/`** - the source from the rendering. It was
//! written when the foundation side was kept by hand and the friendly side was catching up, and it
//! outlived that: nothing failed, because `tests/directories.rs` holds the two equal row for row
//! and a check that they agree is not a check that the right one is the source.
//!
//! **What made it visible is a column boundary.** `P-563` moved `rules.4x` and `schema.4x` into
//! `spec/data/` and named the *converted* form, so Sean's column got a rendering and the rules he
//! authors stayed in this one. `P-576` corrects the direction; inverting this program is what makes
//! the direction something the build does rather than something a document claims.
//!
//! # Where each form of each file lives
//!
//! **One place knows, and it is [`friendly_at`] and [`foundation_at`].** `tests/directories.rs`
//! borrows this module rather than spelling any of it a second time - the way `tests/scenario.rs`
//! borrows `examples/scenario.rs` - so the thing that converts and the thing that checks the
//! conversion cannot disagree about what converts to what.
//!
//! # It writes the five shared files and no test
//!
//! **A test's foundation side is generated from `reviewed/` and not from here.** `spec/README.md`
//! rule 3: *the rendering is generated from `reviewed/` and never from `spec/tests/`, so that what
//! the engine runs is derived from what has been read rather than compared with it.*
//! `examples/foundation.rs` is that program. Generating a test's foundation side here would read
//! what was **typed** where the rule asks for what was **read**, which is the one substitution the
//! whole arrangement exists to forbid.
//!
//! `cargo run --example render`

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use friendly_notation as friendly;

use friendly::Names;
use game_model::notation::{Row, read, write};
use game_model::schema::Schema;

/// Every file the two forms share, then one entry per test.
///
/// **Read rather than listed.** Sean, 2026-09-15: *I intend to have one test per file*, so a list
/// here would be a second place to remember - and `data/foundation/tests/` holding only tests is
/// what makes reading it safe. The flag is whether the whole file is the game's store.
pub fn files() -> Vec<(String, bool)> {
    let mut all: Vec<(String, bool)> = vec![
        ("schema.4x".to_string(), true),
        ("engine.4x".to_string(), true),
        ("rules.4x".to_string(), true),
        ("script.4x".to_string(), false),
        ("setup.4x".to_string(), false),
    ];
    // **Listed from the source, because the source is what every caller then reads.**
    //
    // **This listed `data/foundation/tests/` and resolved each name to `spec/tests/`**, so a test
    // whose source was deleted and whose generated form survived took the whole program down:
    // *`../../spec/tests/a-second-settlement-launches-the-ark-the-first-could-not.4x`: The system
    // cannot find the file specified.* `51a13065` removed two sources under `P-584` and
    // `scripts/review.ps1` stopped opening at all.
    //
    // **Asking the generated directory which sources exist is `S-214` one layer down** - there it
    // was the page's list, here it is this one, and both answered *what has been rendered* where
    // the question was *what is there to render*.
    //
    // **A record whose test is gone is not lost by this.**
    // `every_record_of_a_reading_names_a_test_that_is_there` walks the records and is what reports
    // one, and `CLAUDE.md` says the application shows it so that a rename is two things Sean can
    // see rather than one nobody may touch.
    // **In both spellings, because every caller reads both.** Listing either one alone panics on
    // the other: from the generated side, a source Sean deleted is gone; from the source side, a
    // test he has not read yet has no generated form. **Both are states the process allows** -
    // `51a13065` made the first and `P-584`'s replacement test the second, within one commit of
    // each other.
    //
    // **Nothing is lost by the intersection.** A record whose test is gone is
    // `every_record_of_a_reading_names_a_test_that_is_there`'s, an unread test is
    // `foundation.rs`'s to name, and a reading that never reaches the suite is
    // `every_reading_reaches_the_suite_and_everything_the_suite_runs_was_read`'s. **This function
    // converts and builds name tables**; which files ought to exist is three other checks'
    // question and not one it can answer by falling over.
    let named = |at: &str| -> BTreeSet<String> { under(&mine().join(at)).into_iter().collect() };
    // **Both suites, walked by `under`** - `S-256`. This read `spec/tests/rule` flat and compared
    // it against a flat `data/foundation/tests`, which was right while there was one suite.
    //
    // **The names are qualified on both sides now**, so the intersection still compares a test
    // against its own rendering - and a test in a suite nothing has generated for appears in
    // `source` and not in `rendered`, which is the state an unread interface test is in.
    let source = named("../../spec/tests");
    let rendered = named("data/foundation/tests");
    let mut tests: Vec<String> = source.intersection(&rendered).cloned().collect();
    assert!(
        tests.len() * 2 > source.len() + rendered.len() - tests.len(),
        "{} test(s) are in both spellings against {} in one, which is too few to be the same \
         set of tests",
        tests.len(),
        source.len() + rendered.len() - 2 * tests.len()
    );
    tests.sort();
    all.extend(
        tests
            .into_iter()
            .map(|name| (format!("tests/{name}"), false)),
    );
    all
}

pub fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Where the friendly source of a file is, relative to this crate.
///
/// **Two of the five are Sean's and three are this lane's, and that is the whole of the split.**
/// `P-576`: the game's rules and kinds live in `spec/data/` and the engine's primitives stay in
/// `crates/` - so `schema.4x` and `rules.4x` are the specification's, and `engine.4x`, `script.4x`
/// and `setup.4x` name no game noun and stay here. A test's friendly side is `spec/tests/`'s, which
/// `P-532` settled.
/// A record's behaviour, with the verdict taken off the front.
///
/// **A verdict is a fact about his response and not a row of the game** - `P-605`,
/// `spec/README.md` rule 3: *a record names its verdict and carries the behaviour that verdict is
/// about.* **Two things in one file**, so anything reading the behaviour drops the first.
///
/// # Found by the first record in the new format reaching a reader that had not been told
///
/// **Fifty-six records predate `P-605` and carry no verdict line**, so every reader that folds a
/// record went on working while the format changed underneath it. `drift` was taught to drop the
/// line; `tests/generated.rs` was not, and nothing noticed because **there was no record with a
/// verdict in it to notice with.**
///
/// **Sean pressed convert and the suite went red the same minute** - *the record generates 13 rows
/// and the engine runs 12*, the thirteenth being the verdict. **A check over a population of
/// zero**, which is the shape `CLAUDE.md` names with the sign flipped.
pub fn behaviour_in(record: &str) -> &str {
    match record.strip_prefix("{verdict") {
        Some(rest) => match rest.find('\n') {
            Some(at) => &rest[at + 1..],
            None => "",
        },
        None => record,
    }
}

// **The state a record and a test are in lives here, beside the record's other readers.** It was
// `report.rs`'s, and `tests/directories.rs` cannot reach that file - so it compared the foundation
// against `spec/tests/` with no way to ask whether the test had drifted, and a drifted test failed
// it by definition. **Third time the same argument has moved something into this module**: the
// canonical column order, `folded_record`, and now this. A thing every reader of a record needs
// belongs where every reader can reach it.

/// The state a record and a test are in, over the two texts rather than over the disk.
///
/// **Split out so a check can drive every combination.** `review_of` reads two directories that
/// belong to other columns - `spec/tests/` is the specification's and `reviewed/` is Sean's - so a
/// check that made a state happen by writing one of them would be writing outside this lane.
///
/// **And every record on disk says `approved` today**, so a check that counted the states the page
/// shows would find one of three and pass. **That is a count over nothing wearing the other
/// sign**: the states are driven here, and the page is asserted to render whatever this returns.
pub fn state_of(read: Option<&str>, now: &str) -> Review {
    let (mark, lines) = drift(read, now);
    let denied = read.is_some_and(|it| verdict_of(it) == Ok(Verdict::Denied));

    // **Three states and no others** - `spec/README.md` rule 3, from `P-611`: *I have not looked
    // at it; I have looked and approved it; I have looked and know it is wrong.*
    //
    // **`drifted` was a fourth and is not a state.** A test whose rows have changed since he read
    // it *is in the first state*, because somebody edited it and his approval was of what it
    // said. **So drift collapses into *not looked at* and survives as colour** - the page may say
    // he approved an earlier version, which is worth knowing and is not a fourth answer.
    //
    // **A denial drifts the same way and for the same reason.** What is there now is not what he
    // denied, so the verdict is about a test that no longer exists in that form.
    match (mark, denied) {
        ("drifted", _) => Review {
            state: NOT_LOOKED,
            earlier: Some(denied),
            lines,
        },
        (_, true) => Review {
            state: DENIED,
            earlier: None,
            lines,
        },
        ("reviewed", false) => Review {
            state: APPROVED,
            earlier: None,
            lines,
        },
        _ => Review {
            state: NOT_LOOKED,
            earlier: None,
            lines,
        },
    }
}

/// The three states a test can be in, and nothing else.
///
/// **Named rather than written at each use**, so that
/// `the_page_shows_the_three_states_the_rule_names` can range over them instead of carrying its
/// own copy of the list - a check against a second copy is checking the copy.
pub const NOT_LOOKED: &str = "never reviewed";
/// He looked and approved it, and the code is bound where this test is concerned.
pub const APPROVED: &str = "reviewed";
/// He looked and knows it is wrong, which binds nothing and owes him a statement of what he wants.
pub const DENIED: &str = "denied";

/// Every state, so a check can range over them.
pub const STATES: [&str; 3] = [NOT_LOOKED, APPROVED, DENIED];

/// What he said about one test: a state, and colour that is not a state.
///
/// **`earlier` is the drift**, kept because it is worth seeing and separated because it is not an
/// answer to *what do you think of this*. `Some(false)` is *you approved an earlier version*,
/// `Some(true)` is *you denied one*, and `None` is no record or one that still matches.
pub struct Review {
    pub state: &'static str,
    pub earlier: Option<bool>,
    pub lines: Vec<(&'static str, String)>,
}

/// The same comparison over the text rather than over the disk.
///
/// **Split out so it can be shown failing.** `review_of` reads two directories that belong to
/// other columns - `spec/tests/` is the specification and `reviewed/` is Sean's - so a test that
/// demonstrated drift by editing one of them would be writing outside this lane. **This takes the
/// two texts**, and `tests/reviewed.rs` shows a changed row, a whitespace-only change and a
/// missing record on strings it owns.
///
/// **The wiring needs no such demonstration, because both ways of getting it wrong are red.** A
/// records directory that pointed at nothing would answer *never reviewed* for all fifty-four and
/// a tests directory that pointed at nothing would answer *drifted* for all fifty-four - so the
/// gate passing is already a statement that both are found and that they agree.
pub fn drift(read: Option<&str>, now: &str) -> (&'static str, Vec<(&'static str, String)>) {
    let Some(read) = read else {
        return ("never reviewed", Vec::new());
    };
    // **What a test says, with everything that is not behaviour taken off.**
    //
    // `P-600`: *my approval is about what a test says, not where it is or how it is written. The
    // behaviour is every `{...}` row, including a `{load}`, and nothing else - a comment explains
    // and does not decide, and `{test name:}` identifies rather than states. So two tests are the
    // same test when their rows say the same thing, however the text differs: entries coalesced to
    // one per description, traits and entries in the order this specification already gives them,
    // whitespace not significant.*
    //
    // **And `P-606`, which is Sean's answer and neither order either lane offered**: *the order of
    // the columns is not significant, so this should not make tests different, although we should
    // be deterministic about them either way.* So the writer picks an order and **this takes it
    // away**.
    //
    // # Four things dropped, and each is a verdict surviving something it should survive
    //
    // ```text
    // a comment            P-600: a comment explains and does not decide
    // {verdict state:...}  the record's own statement, about the test rather than behaviour
    // column order         P-606: not significant
    // a repeated entry     coalesced to one per description, quantities summed
    // ```
    //
    // **This compared whole lines including comments**, so a reworded comment read as drift - which
    // `P-600` names as a change an approval survives. It cost nothing while a record was a byte
    // copy of its test; **under `P-605` a record has no prose at all, so every record the writer
    // writes would have read as drifted.** Measured before this was changed: `status=drifted`,
    // fifteen lines, on a record built from an unmodified test.
    //
    // # Why the `seq:` trap dies here rather than in the writer
    //
    // **The written order follows the schema's declared columns, and `seq:` values are editable
    // with no meaning beyond order.** So renumbering one changes what the writer emits. **If this
    // compared written bytes, that would clear every verdict**; because it compares a row's
    // key-value set, the written form changes and the behaviour does not, and nothing clears.
    let bare = |text: &str| -> Vec<(String, String)> {
        let mut section = "note";
        let mut counted: BTreeMap<(String, String), i64> = BTreeMap::new();
        let mut plain: Vec<(String, String)> = Vec::new();
        for line in text.lines() {
            let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
            // **Only a row is behaviour.** A comment, a blank and anything else are dropped rather
            // than compared.
            if !line.starts_with('{') {
                continue;
            }
            if let Some(marker) = line.strip_prefix('{').and_then(|it| it.strip_suffix('}'))
                && matches!(marker, "given" | "when" | "then" | "refused")
            {
                section = match marker {
                    "given" => "given",
                    "when" => "when",
                    "then" => "then",
                    _ => "refused",
                };
                continue;
            }
            let (row, how_many) = match line.rsplit_once(" -> ") {
                Some((row, count)) => (row.to_string(), count.trim().parse::<i64>().ok()),
                None => (line.clone(), None),
            };
            let Some(inside) = row
                .trim()
                .strip_prefix('{')
                .and_then(|it| it.strip_suffix('}'))
            else {
                plain.push((section.to_string(), line));
                continue;
            };
            let mut words = inside.split(' ');
            let relation = words.next().unwrap_or_default().to_string();
            // **The record's own verdict is not the test's behaviour**, and the test it is about
            // does not carry one - so comparing it would report every record as drifted.
            if relation == "verdict" {
                continue;
            }
            // **`{test name:}` identifies rather than states**, so it is dropped on both sides.
            // The writer writes it from the file's name, which is what makes a rename detectable;
            // that is a different question from whether two tests say the same thing.
            if relation == "test" {
                continue;
            }
            // **Sorted, which is what takes the column order away.**
            let mut values: Vec<&str> = words.filter(|it| !it.is_empty()).collect();
            values.sort_unstable();
            let key = (
                section.to_string(),
                format!("{relation} {}", values.join(" ")),
            );
            match how_many {
                Some(n) => *counted.entry(key).or_insert(0) += n,
                // **A row with no quantity is one entry**, and two of them with one description
                // would be a store holding one thing twice, which the key refuses.
                None => *counted.entry(key).or_insert(0) += 1,
            }
        }
        let mut out = plain;
        out.extend(
            counted
                .into_iter()
                .map(|((section, row), n)| (section, format!("{{{row}}} -> {n}"))),
        );
        out.sort();
        out
    };
    let (was, is) = (bare(read), bare(now));
    if was == is {
        return ("reviewed", Vec::new());
    }
    // **The lines are a hint and the status is the fact.** Equality above catches everything,
    // ordering included; this difference is only to show a reader where to look, and a reordering
    // with no other change would leave it empty while the status still says drifted.
    //
    // **A line now carries its section, so the comparison is finer than it was.** A row deleted
    // from a `then` while an identical row stands in the `given` used to match and be hidden.
    // **Each line carries its side as well as saying it.** The page annotates one *what you read*
    // and the other *not what you read*, and a stylesheet cannot work that out from the text - so
    // the side travels with the line rather than being parsed back off the front of it.
    let shown = |side: &'static str, (section, line): &(String, String)| {
        (side, format!("{side}: {section:<7} {line}"))
    };
    let mut said: Vec<(&'static str, String)> = is
        .iter()
        .filter(|it| !was.contains(it))
        .map(|it| shown("now", it))
        .collect();
    said.extend(
        was.iter()
            .filter(|it| !is.contains(it))
            .map(|it| shown("was", it)),
    );
    ("drifted", said)
}

/// The suites `spec/tests/` holds, and `reviewed/` mirrors.
///
/// **`spec/tests/README.md` names the two and what each is for**, so this is stated rather than
/// invented - `P-617`, `S-256`.
pub const SUITES: [&str; 2] = ["rule", "interface"];

/// Every `.4x` under a directory of suites, as `suite/name.4x`.
///
/// **The suite moved out of the path and into the name** - `S-256`. `tests_at()` was
/// `spec/tests/rule` and `records_at()` was `reviewed/rule`, so **everything walked the one
/// subdirectory by name** and an interface test on disk was invisible: 63 tests on the page, and
/// the sixty-fourth in a directory nothing read.
///
/// **Joining a qualified name onto the parent gives the same path it always did**, which is why
/// this is one change rather than ninety: `tests_at().join("rule/x.4x")` is what
/// `tests_at().join("x.4x")` used to be.
///
/// # One definition rather than five
///
/// **This was `names_in` in `foundation.rs`, `names_in` in `tests/generated.rs`, `stems` in
/// `tests/reviewed.rs`, and `every_test` in both `report.rs` and `review.rs`** - five copies of
/// *list the `.4x` files here*, which is what made adding a suite five edits. **Fourth time this
/// argument has moved something into this module**, after the canonical column order,
/// `folded_record` and the record's state.
///
/// **A file sitting directly in `at` keeps its bare name**, because `regression/` and the
/// foundation's own `tests/` are flat and this is used on them too.
pub fn under(at: &Path) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    let Ok(entries) = std::fs::read_dir(at) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            for inside in std::fs::read_dir(&path).into_iter().flatten().flatten() {
                let leaf = inside.file_name().to_string_lossy().to_string();
                if leaf.ends_with(".4x") {
                    found.push(format!("{name}/{leaf}"));
                }
            }
        } else if name.ends_with(".4x") {
            found.push(name);
        }
    }
    found.sort();
    found
}

/// A record's rows, folded - the one way to read a record's behaviour.
///
/// **Four places folded a record and two of them folded the verdict row too**, so the foundation
/// the generator wrote carried a row the engine does not run and the suite that compares them
/// disagreed about every one of the fifty-nine. **`behaviour_in` existed and two callers did not
/// call it**, which is a rule stated without a carrier.
///
/// **So this is the carrier.** A caller that wants a record's rows asks for them, and there is no
/// raw `fold` of a record left to copy.
///
/// # Why it was one record and then fifty-nine
///
/// **Fifty-six records predated `P-605` and carried no verdict line**, so for as long as the
/// format has existed there was nothing for either reader to get wrong. The first approval made it
/// one; **his conversion made it fifty-nine**, and whatever the readers disagreed about they now
/// disagreed about everywhere.
///
/// **Both times the population moved and nothing was wrong with the code in between.** `C-216`
/// fixed one reader against one record - the only one there was - and verifying against a
/// population of one is what left the other reader unexamined.
pub fn folded_record(text: &str, schema: &Schema) -> Result<Vec<Row>, String> {
    friendly::fold(behaviour_in(text), schema).map_err(|why| why.to_string())
}

/// What Sean has said about the generated cases, and what he has authorized.
///
/// **`spec/README.md` rule 3**: *a case's verdict is a row in `reviewed/cases.4x` and pins no
/// behaviour... an authorization is a different relation from a verdict because it is consumed. A
/// verdict stands until I change it; `{regenerate}` is spent by the regeneration it asks for and
/// is gone afterwards.*
///
/// # Two columns this lane chose, and they are the only invention here
///
/// **The semantics are specified and the spelling is not.** `P-608` names the relations -
/// `verdict` and `regenerate` - and says what each means; no promoted text gives their columns.
///
/// **So these follow the notation's own precedent**: a record says `{verdict state:approved}` and
/// identifies its test with a separate `{test name:...}` row, because one file holds one test.
/// **One file holds every case here**, so each row names the case it is about:
///
/// ```text
/// {verdict case:scenario/01/02-gather state:denied}
/// {regenerate case:scenario/01/02-gather}
/// ```
///
/// **A case is named as the suite names it** - its path under `regression/` without the
/// extension - because that is the identifier `Case.name` already carries and the one the
/// failure message already prints.
///
/// **If either column is wrong, renaming it costs a rewrite of this file and nothing else.** No
/// behaviour is pinned here - that is the whole of `P-608`'s first half - so unlike a test's
/// record, a spelling change cannot cost an approval.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cases {
    /// The case names he has denied. **A verdict stands until he changes it.**
    pub denied: BTreeSet<String>,
    /// The case names he has approved, which is the ordinary state and is recorded so that
    /// *approved* and *never looked at* stay two facts, as they are for a test.
    pub approved: BTreeSet<String>,
    /// The case names he has authorized a regeneration for. **Spent by the run that acts on it.**
    pub authorized: BTreeSet<String>,
}

/// Read `reviewed/cases.4x`, or an empty set where there is none.
///
/// **An absent file is not an error.** No verdict on any case is the state before he has pressed
/// anything, and it is the state today.
pub fn cases_in(text: &str) -> Result<Cases, String> {
    let mut out = Cases::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || !line.starts_with('{') {
            continue;
        }
        let inside = line
            .strip_prefix('{')
            .and_then(|it| it.strip_suffix('}'))
            .ok_or_else(|| format!("`{line}` is not a row"))?;
        let mut words = inside.split_whitespace();
        let relation = words.next().unwrap_or_default();
        let values: BTreeMap<&str, &str> = words.filter_map(|word| word.split_once(':')).collect();
        let named = values
            .get("case")
            .ok_or_else(|| format!("`{line}` names no case"))?
            .to_string();
        match relation {
            "verdict" => match values.get("state").copied() {
                Some("denied") => {
                    out.denied.insert(named);
                }
                Some("approved") => {
                    out.approved.insert(named);
                }
                // **An unknown state is refused rather than guessed**, the same way a test's is:
                // treating it as approved would bind on a word nobody defined, and treating it as
                // denied would drop a case he accepted.
                other => {
                    return Err(format!(
                        "`{other:?}` is not a verdict; rule 3 names approved and denied"
                    ));
                }
            },
            "regenerate" => {
                out.authorized.insert(named);
            }
            other => return Err(format!("`{other}` is not a relation this file may hold")),
        }
    }
    // **A case cannot be both denied and approved**, which one file holding both rows would say.
    let both: Vec<&String> = out.denied.intersection(&out.approved).collect();
    if !both.is_empty() {
        return Err(format!("{both:?} carry two verdicts"));
    }
    Ok(out)
}

/// What Sean said about a test, read out of its record.
///
/// **`P-605`, `spec/README.md` rule 3**: *a record names its verdict and carries the behaviour
/// that verdict is about - the rows, canonical, without the prose. No record means I have not
/// looked; a record saying `approved` means the code is bound by it; a record saying `denied`
/// means it is not, and that I owe the specification a statement of what I want instead.*
///
/// # The weld this splits, which is why there was never room for a third state
///
/// **Presence used to mean both *I read this* and *this binds*.** Those are two facts now, so a
/// directory listing answers the first and only a verdict answers the second.
///
/// **The one to fear is the suite running a denied test** - a failure that looks exactly like
/// nothing being wrong. `foundation.rs` says reading `reviewed/` rather than `spec/tests/` *is
/// the whole of the rule*; **that rule has a second half now and the first half alone is not
/// sufficient.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The code is bound by it.
    Approved,
    /// It is not binding, and he owes the specification what he wants instead.
    Denied,
}

/// The verdict a record states, or `Approved` where it states none.
///
/// **A record with no `{verdict}` row is `Approved`, and that is backward compatibility rather
/// than a default.** The fifty-seven records written before `P-605` are byte copies, and under
/// the rule they were written under **presence meant both halves of the weld** - so reading them
/// as approved preserves exactly what they recorded.
///
/// **Nothing converts them and nothing here may.** `reviewed/` is written by the review
/// application acting as Sean, and a format migration is that application's gesture rather than
/// a reader's - `C-204` puts the question where he can answer it.
///
/// **An unknown state is refused rather than guessed.** A record saying `pending` or anything
/// else is a record this code does not understand, and treating it as approved would bind the
/// code on a word nobody defined.
pub fn verdict_of(text: &str) -> Result<Verdict, String> {
    let stated: Vec<&str> = text
        .lines()
        .map(str::trim)
        // **`{verdict}` as well as `{verdict ...}`**, because a row with no `state:` is a
        // malformed verdict and not the absence of one. **Matching only the spelling with a
        // trailing space read it as no verdict at all, which is `Approved`** - the unsafe
        // direction, and the one a reader would never notice. Found by driving the five
        // spellings rather than by reading the filter.
        .filter(|line| line == &"{verdict}" || line.starts_with("{verdict "))
        .collect();
    match stated.len() {
        0 => Ok(Verdict::Approved),
        1 => {
            let said = stated[0]
                .split_whitespace()
                .find_map(|part| part.strip_prefix("state:"))
                .map(|it| it.trim_end_matches('}'))
                .unwrap_or_default();
            match said {
                "approved" => Ok(Verdict::Approved),
                "denied" => Ok(Verdict::Denied),
                other => Err(format!(
                    "`{other}` is not a verdict this code knows; `spec/README.md` rule 3 names \
                     approved and denied, and a third would bind the code on a word nobody defined"
                )),
            }
        }
        many => Err(format!(
            "{many} `{{verdict}}` rows in one record, so it says more than one thing about one test"
        )),
    }
}

pub fn friendly_at(file: &str) -> String {
    match file {
        "schema.4x" | "rules.4x" => format!("../../spec/data/{file}"),
        _ => match file.strip_prefix("tests/") {
            // **The name carries its suite** - `S-256`. This joined `rule/`, which was right
            // while there was one suite and gives `spec/tests/rule/rule/...` for a qualified
            // name.
            Some(name) => format!("../../spec/tests/{name}"),
            None => format!("data/friendly/{file}"),
        },
    }
}

/// Where the generated foundation form of a file is, relative to this crate.
///
/// **All of it is here, because all of it is generated**, and `CLAUDE.md` gives a generated file no
/// owner. `game_model::foundation::PATHS` names the three the shipped binary carries and has to
/// agree with this; `tests/directories.rs` is what asserts that it does.
pub fn foundation_at(file: &str) -> String {
    format!("data/foundation/{file}")
}

pub fn rows(at: &str) -> Vec<Row> {
    let text = std::fs::read_to_string(mine().join(at)).unwrap_or_else(|why| panic!("{at}: {why}"));
    read(&text).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// One friendly file's rows, with any `-> n` folded back into the column that holds it.
///
/// **The schema is read first and from the friendly side**, because the fold has to know which
/// column a quantity belongs in - and a schema file carries no arrow itself, so reading it needs
/// nothing that is not already there.
pub fn friendly_rows(file: &str) -> Vec<Row> {
    let at = friendly_at(file);
    let schema = Schema::of(&rows(&friendly_at("schema.4x"))).expect("a schema");
    let text =
        std::fs::read_to_string(mine().join(&at)).unwrap_or_else(|why| panic!("{at}: {why}"));
    friendly::fold(&text, &schema).unwrap_or_else(|why| panic!("{at}: {why}"))
}

/// Every friendly row of one of the two stores, deduplicated.
///
/// **A merged test file spans two stores**, so the store a row belongs to is a fact about where it
/// sits: a section's rows are the game's, everything else in that file is the script's.
/// **Deduplicated**, because a `then` section repeats its `given` one and two rows named `scout`
/// would make `thing` non-nameable and take every thing's name away with it.
pub fn store(of_game: bool) -> Vec<Row> {
    let mut seen: Vec<Row> = Vec::new();
    for (file, game) in files() {
        let these = friendly_rows(&file);
        let section = friendly::states_a_world(&these);
        for (row, is_game) in these.into_iter().zip(section) {
            if (game || is_game) == of_game && !seen.contains(&row) {
                seen.push(row);
            }
        }
    }
    seen
}

/// What one store needs to be converted: the names ids come from, and the schema that orders the
/// columns of what comes out.
///
/// # Two schemas and not one, which `examples/foundation.rs` found the expensive way
///
/// **`Schema::write` puts a row's values in the order its relation declares them, and a relation it
/// cannot find falls back to the notation's alphabetical order without saying so.** So the writing
/// schema is built over every row of the store rather than over `schema.4x` alone - `schema.4x`
/// declares 49 relations and `move` is not among them, and `{move what:28 from:1 to:2}` came out
/// `{move from:1 to:2 what:28}`: the same row, a different file, nothing complaining.
///
/// **And the script's is a second schema rather than more of the first.** `script.4x` declares
/// `store`, `test` and `load` over ids `schema.4x` already uses for other things, which is why the
/// two stores exist at all - and `Schema::of` refuses the merge, which is what says they are two.
pub fn table(of_game: bool) -> (Names, Schema) {
    let rows = store(of_game);
    let schema = Schema::of(&rows).expect("a schema over the store's own rows");
    (Names::of(&rows), schema)
}

/// The foundation form of every shared file, as text, paired with where it belongs.
///
/// **Produced here and used twice**: `main` writes it, and `tests/directories.rs` compares it with
/// what is committed. **A generator and a check that the generated file is current are the same
/// derivation**, so they are one function - which is what stops the check being a second opinion
/// about what conversion means.
pub fn converted() -> Vec<(String, String)> {
    let (of_game, game_schema) = table(true);
    let (of_script, script_schema) = table(false);

    let mut all = Vec::new();
    for (file, game) in files() {
        // **A test's foundation side is not this program's to write** - see the note at the top.
        // Skipped rather than filtered out of `files()`, because the name tables above are built
        // from every file and a test's rows are most of what the names are drawn from.
        if file.starts_with("tests/") {
            continue;
        }
        let from = friendly_at(&file);
        let text = std::fs::read_to_string(mine().join(&from))
            .unwrap_or_else(|why| panic!("{from}: {why}"));
        // **Which name table reads a row is a fact about where the row sits**: a script file's
        // prologue is the script's and its sections are the game's.
        let parsed = friendly_rows(&file);
        let of_the_game = friendly::in_a_section(&parsed);
        let mut at = 0;

        let mut out = String::new();
        for line in text.lines() {
            let bare = line.trim();
            if bare.is_empty() || bare.starts_with('#') {
                // **Comments and blank lines are carried across**, so the generated file keeps the
                // prose that explains it rather than becoming a bare list of rows.
                out.push_str(line);
            } else {
                let (names, writing) = if game || of_the_game[at] {
                    (&of_game, &game_schema)
                } else {
                    (&of_script, &script_schema)
                };
                let row = &parsed[at];
                at += 1;
                let converted = names
                    .foundation(row)
                    .unwrap_or_else(|why| panic!("{from}: `{}`: {why}", write(row)));
                // **`Schema::write` rather than `notation::write`** - see [`table`]. The notation
                // sorts a row's columns and the relation declares an order, and 18 rows of
                // `script.4x` are how this lane found that out a second time.
                out.push_str(&writing.write(&converted));
            }
            out.push('\n');
        }
        assert_eq!(
            at,
            parsed.len(),
            "{from}: {at} of {} rows were converted, so a line was read as prose",
            parsed.len()
        );
        all.push((foundation_at(&file), out));
    }
    assert_eq!(
        all.len(),
        5,
        "five shared files convert, and this converted {}",
        all.len()
    );
    all
}

fn main() {
    let mut changed = 0;
    for (at, text) in converted() {
        // **Only what changed**, so a run that converts nothing new says so rather than touching
        // every file's timestamp.
        let to = mine().join(&at);
        let before = std::fs::read_to_string(&to).unwrap_or_default();
        if before != text {
            std::fs::write(&to, &text).unwrap_or_else(|why| panic!("{}: {why}", to.display()));
            println!("{at}");
            changed += 1;
        }
    }
    println!("{changed} rewritten");
}

/// One rendered row, with its columns in the order `spec/console.md` states.
///
/// **`P-606`**: *the order that function puts columns in must not depend on anything editable.
/// `id` first, then every other trait alphabetically, then `occupied`, `free` and `capacity`
/// last - an order taken from the schema's `seq:` would mean renumbering those cleared every
/// approval I have given, and renumbering is a tidy-up nobody thinks twice about.*
///
/// **`Names::row` writes the schema's declared order**, which is exactly the editable one. So the
/// columns are reordered after it rather than instead of it: the renderer knows how to name a
/// value and this knows where it goes.
pub fn canonical(written: &str) -> String {
    let said = written.trim();
    let (row, arrow) = match said.rsplit_once(" -> ") {
        Some((row, count)) => (row, Some(count)),
        None => (said, None),
    };
    let Some(inside) = row.strip_prefix('{').and_then(|it| it.strip_suffix('}')) else {
        return written.to_string();
    };
    let mut words = inside.split(' ').filter(|it| !it.is_empty());
    let Some(relation) = words.next() else {
        return written.to_string();
    };
    let columns: Vec<(String, Option<String>)> = words
        .map(|word| match word.split_once(':') {
            Some((key, value)) => (key.to_string(), Some(value.to_string())),
            // **A valueless word keeps its place by name**, which is how a declaration's `id`
            // sorts without displacing `name` - `P-481` and `P-483`.
            None => (word.to_string(), None),
        })
        .collect();
    let ordered = friendly_notation::in_canonical_order(columns);
    let mut out = format!("{{{relation}");
    for (key, value) in ordered {
        match value {
            Some(value) => out.push_str(&format!(" {key}:{value}")),
            None => out.push_str(&format!(" {key}")),
        }
    }
    out.push('}');
    if let Some(count) = arrow {
        out.push_str(&format!(" -> {count}"));
    }
    out
}

pub fn record_for(name: &str, text: &str, verdict: &str) -> Result<String, String> {
    // **The rows, by name, with no schema** - `S-259`, and `spec/README.md` rule 3: *the behaviour
    // is every `{...}` row, including a `{load}`, and nothing else*, in an order that *depends on
    // names* and *must not depend on anything editable*.
    //
    // **This folded against the schema, which is narrower than the rule it implements.** A schema
    // is the engine's business; a record is what he approved. So a test whose relations no schema
    // declares - the first is `interface/the-starting-menu-offers-new-game-and-exit` - **could not
    // be recorded at all**, and `P-621` says that is the normal way a thing he wants becomes a
    // thing that is built: *relations do not have to exist for a test I have not approved.*
    //
    // **Measured before changing it: all 63 records come out byte-identical**, so no approval of
    // his moves. The old path folded to rows and rendered them back to the friendly form; this
    // reads the friendly form it was going to reproduce.
    let leaf = name.rsplit('/').next().unwrap_or(name);
    let mut out = format!("{{verdict state:{verdict}}}\n{{test name:{leaf}}}\n");
    let mut wrote = 0;
    for line in text.lines().map(str::trim) {
        // **Prose is not behaviour** - a record carries the rows and not the words around them.
        if !line.starts_with('{') {
            continue;
        }
        let relation = line
            .trim_start_matches('{')
            .split([' ', '}'])
            .next()
            .unwrap_or_default();
        // **`{test name:}` is written once, above, from the file's own name.** A test states it
        // too and the two have always agreed; writing the row from the name rather than copying it
        // is what makes a disagreement impossible rather than unlikely.
        if relation == "test" {
            continue;
        }
        // **A section marker keeps its own line and takes a blank line before it**, so the record
        // reads the way a test reads.
        if matches!(relation, "given" | "when" | "then" | "refused") {
            out.push('\n');
        }
        out.push_str(&canonical(line));
        out.push('\n');
        wrote += 1;
    }
    // **A record with no rows would be a verdict about nothing**, and an approval of nothing is
    // the shape a reader would take for an approval of something.
    if wrote == 0 {
        return Err(format!(
            "`{name}` has no rows, so there is no behaviour to record"
        ));
    }
    Ok(out)
}
