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
use std::path::PathBuf;

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
    let named = |at: &str| -> BTreeSet<String> {
        std::fs::read_dir(mine().join(at))
            .unwrap_or_else(|why| panic!("{at}: {why}"))
            .filter_map(|it| it.ok())
            .filter_map(|it| it.file_name().to_str().map(str::to_string))
            .filter(|name| name.ends_with(".4x"))
            .collect()
    };
    // **`spec/tests/rule`, since the split on 2026-10-01.** The engine runs the rule suite and
    // `data/foundation/tests` is its rendering, so the two sides compared here are that one pair -
    // an interface test has no foundation form and is not the engine's to render.
    let source = named("../../spec/tests/rule");
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
            Some(name) => format!("../../spec/tests/rule/{name}"),
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
    let (names, schema) = table(true);
    let rows = friendly_notation::fold(text, &schema).map_err(|why| why.to_string())?;

    let mut out = format!("{{verdict state:{verdict}}}\n");
    out.push_str(&format!("{{test name:{name}}}\n"));
    let mut wrote = 0;
    for row in &rows {
        // **`{test name:}` is written once, above, from the file's own name.** A test states it
        // too and the two have always agreed; writing the row from the name rather than copying it
        // is what makes a disagreement impossible rather than unlikely.
        if row.relation == "test" {
            continue;
        }
        // **A section marker keeps its own line and takes a blank line before it**, so the record
        // reads the way a test reads. It is a row of no relation, which `Names::row` writes as it
        // is.
        let written = canonical(&names.row(row));
        if matches!(row.relation.as_str(), "given" | "when" | "then" | "refused") {
            out.push('\n');
        }
        out.push_str(&written);
        out.push('\n');
        wrote += 1;
    }
    // **A record with no rows would be a verdict about nothing**, and an approval of nothing is
    // the shape a reader would take for an approval of something.
    if wrote == 0 {
        return Err(format!(
            "`{name}` folded to no rows, so there is no behaviour to record"
        ));
    }
    Ok(out)
}
