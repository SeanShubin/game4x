//! Write `reports/review/index.html`: every test, whole, with whatever failed marked in it.
//!
//! **Sean, 2026-09-16**: *I want an aesthetically pleasing and informative test report. [...] Make
//! sure I can see the entirety of the test and the failures are highlighted somehow.*
//!
//! **It shows the friendly file and runs the foundation one.** The friendly form is how a person
//! reads a test - `{residency what:scout where:territory-1} -> 1` rather than `what:1 where:1` -
//! and the foundation is what the engine reads. A row the run says is missing is rendered back
//! into the friendly form to be found in the text.
//!
//! **One file, no stylesheet beside it, and no script in the file on disk.** It opens from disk,
//! which is what `R-9` asks of a generated view. `review-web` serves the same page with a script
//! added, so the reviewing controls exist only where something is listening to them - a button
//! that writes to the disk would be a lie in a file opened from it.
//!
//! **The page is built here and nowhere else.** `review-web` calls `build` rather than rendering
//! its own, so the served page and the written one cannot disagree about what a test says.
//!
//! `cargo run --example report`

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use friendly_notation as friendly;

use friendly::Names;
use game_model::notation::{Row, read};
use game_model::script::{Files, run_test};

// **`pub` so `review-web.rs` reaches the canonical writer through this module** rather than
// borrowing `render.rs` a second time, which would give a file that loads both two copies of it
// and fail clippy's `duplicate_mod` - the hazard `tests/reviewed.rs` already records.
#[path = "render.rs"]
#[allow(dead_code)]
pub mod render;

fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Where the approved tests live, which is no longer inside this prototype.
///
/// **`P-532`, 2026-09-21**: Sean moved them to `spec/tests/` and their approval records to
/// `reviewed/`, both at the repository root. **They are the specification now** - `CLAUDE.md`,
/// promoted the same day: a test in `spec/tests/` arrives the way everything in `spec/` arrives,
/// which is that he has read it.
///
/// **Named once rather than spelled out ten times**, which is what the move cost when they were
/// spelled out: ten files, four of them tests, and nothing to change in one place.
pub fn tests_at() -> PathBuf {
    mine()
        .join("..")
        .join("..")
        .join("spec")
        .join("tests")
        .join(SUITE)
}

/// Which suite of `spec/tests/` this engine runs.
///
/// **`spec/tests/` split into `rule/` and `interface/` on 2026-10-01**, and the engine runs the
/// first: a rule test is about the transformation function, and an interface test is about what a
/// person sees. **Named once here because the three paths below have to agree about it** - the
/// source he writes, the record that he read it, and the copy the suite iterates.
///
/// **Sean, on what made the move cheap**: *it was always the case that my approval was about what
/// the tests said, not where the tests were.* So moving a record is not creating or deleting one,
/// which `CLAUDE.md` now says.
pub const SUITE: &str = "rule";

/// Where the form the engine actually runs lives, generated from `reviewed/`.
///
/// **Named here with the other two** because the three together are the thing that has to line
/// up: the source he writes, the record that he read it, and the copy the suite iterates.
/// `S-215` is what happens when only two of the three are ever compared.
pub fn foundation_tests_at() -> PathBuf {
    mine().join("data").join("foundation").join("tests")
}

/// Where the record of what Sean has read lives.
///
/// **No instance writes it** - `CLAUDE.md`. The review application does, acting as him, and that
/// application is this prototype's.
pub fn records_at() -> PathBuf {
    mine().join("..").join("..").join("reviewed").join(SUITE)
}

fn text(at: &str) -> String {
    std::fs::read_to_string(mine().join(at)).unwrap_or_else(|why| panic!("{at}: {why}"))
}

fn rows(at: &str) -> Vec<Row> {
    read(&text(at)).unwrap_or_else(|why| panic!("{at}: {why}"))
}

struct Directory(PathBuf);

impl Files for Directory {
    fn read(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(name)).ok()
    }
}

/// Every test file, read rather than listed - one test per file, and nothing else in `tests/`.
///
/// # `spec/tests/` and not `data/foundation/tests/`, which is `S-214`
///
/// **This read the generated directory, and that directory is generated from `reviewed/`.** So a
/// test reached the page only after it had been read, and **the one interface by which a test
/// becomes part of `spec/` could never show a new one.** Sean ran the application an hour after
/// two tests were written for him and saw neither.
///
/// **`spec/README.md` rule 3** is the sentence it was on the wrong side of: the two directories
/// *hold the same tests only while I have read every one, and a test nobody has read is in the
/// first and not the second.* **The source is the first.**
///
/// **It never fired before because the directory arrived whole**: `b629b5e7` moved all fifty-four
/// at once and every one already had a foundation form. `ce755d39` is the first genuinely new
/// test since, and it is what found this.
pub fn every_test() -> Vec<String> {
    let mut found: Vec<String> = std::fs::read_dir(tests_at())
        .expect("spec/tests")
        .filter_map(|it| it.ok())
        .filter_map(|it| it.file_name().to_str().map(str::to_string))
        .filter(|name| name.ends_with(".4x"))
        .collect();
    found.sort();
    found
}

/// What converting a test's source needs: the two name tables and the schema that folds it.
///
/// **Built once rather than per test**, because each is read from every file of the store.
struct Converting {
    game: Names,
    script: Names,
    schema: game_model::schema::Schema,
}

impl Converting {
    fn ready() -> Self {
        let (game, _) = render::table(true);
        let (script, _) = render::table(false);
        let at = render::mine().join(render::friendly_at("schema.4x"));
        let said =
            std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));
        let rows = read(&said).unwrap_or_else(|why| panic!("spec/data/schema.4x: {why}"));
        let schema = game_model::schema::Schema::of(&rows).expect("a schema");
        Converting {
            game,
            script,
            schema,
        }
    }

    /// One test's rows in the form the engine reads, converted from the source it is written in.
    ///
    /// **A test that has never been read has no foundation copy**, which is the whole of `S-214`:
    /// four reads in `build` went to the generated directory and a new test has nothing there.
    ///
    /// **Folding is not converting, and this lane shipped the difference for a minute.** The
    /// friendly form says `relation:territory` and the foundation form says `relation:13`, so
    /// reading the source and folding its arrows gave rows the engine could parse and not match -
    /// fifty-six tests, **every one of them red, none of them changed**. `Names::foundation` is
    /// the step that was missing, and it is the same one `examples/render.rs` writes the
    /// generated copies with, so nothing here is a second opinion about what conversion means.
    ///
    /// **Which name table reads a row is a fact about where the row sits** - a script file's
    /// prologue is the script's and its sections are the game's, which `in_a_section` decides.
    fn rows_of(&self, file: &str) -> Result<Vec<Row>, String> {
        let at = tests_at().join(file);
        let said =
            std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));
        let parsed = friendly::fold(&said, &self.schema).map_err(|why| why.to_string())?;
        let of_the_game = friendly::in_a_section(&parsed);
        let mut out = Vec::new();
        for (at, row) in parsed.iter().enumerate() {
            let names = if of_the_game[at] {
                &self.game
            } else {
                &self.script
            };
            out.push(names.foundation(row).map_err(|why| why.to_string())?);
        }
        Ok(out)
    }
}

/// A record's bytes as base64, so the page can carry one in an attribute.
///
/// **The GitHub contents API takes a file that way**, and encoding here means the page never has
/// to encode text it did not generate - `E-4`. **Written out rather than taken from a crate**,
/// because a dependency for sixteen lines of table lookup is a dependency to keep current.
fn encoded(raw: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = raw.as_bytes();
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let mut block = [0u8; 3];
        block[..chunk.len()].copy_from_slice(chunk);
        let packed = u32::from(block[0]) << 16 | u32::from(block[1]) << 8 | u32::from(block[2]);
        for at in 0..4 {
            if at <= chunk.len() {
                let six = (packed >> (18 - 6 * at)) & 0b11_1111;
                out.push(ALPHABET[six as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

fn escaped(raw: &str) -> String {
    raw.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// One friendly line, with the cells that differ between the two worlds marked.
///
/// **Sean, 2026-09-20**, asking for this as the bonus half: *split a single line into what is
/// different about it, but only if it can be done deterministically.* **It is**, because the line
/// is already a list of cells - `{relation column:value ...}` and an arrow - and which of them
/// differ is what [`friendly::compared`] hands over.
///
/// **The quantity is a cell too, and is written as an arrow rather than as `quantity:n`.** So it
/// is marked by where it sits rather than by its name, which is the one place this has to know how
/// the friendly form writes a row.
/// A friendly line and the row it is, so a marking can be found by the line it belongs to.
type Stated = (String, Row);

fn celled(line: &str, differ: &[String], counted: Option<&str>) -> String {
    let (Some(open), Some(close)) = (line.find('{'), line.find('}')) else {
        return escaped(line);
    };
    let mut out = escaped(&line[..=open]);
    for (at, token) in line[open + 1..close].split(' ').enumerate() {
        if at > 0 {
            out.push(' ');
        }
        // **The relation is the first token and names no column**, so it is never a cell that
        // differs - two rows of different relations are never paired at all.
        let column = token.split_once(':').map(|(name, _)| name).unwrap_or("");
        if at > 0 && differ.iter().any(|it| it == column) {
            out.push_str(&format!("<span class=\"cell\">{}</span>", escaped(token)));
        } else {
            out.push_str(&escaped(token));
        }
    }
    let tail = &line[close..];
    match counted.filter(|it| differ.iter().any(|was| was == it)) {
        Some(_) => match tail.find("->") {
            Some(arrow) => {
                out.push_str(&escaped(&tail[..arrow]));
                out.push_str(&format!(
                    "<span class=\"cell\">{}</span>",
                    escaped(&tail[arrow..])
                ));
            }
            None => out.push_str(&escaped(tail)),
        },
        None => out.push_str(&escaped(tail)),
    }
    out
}

/// Records of a reading whose test is gone.
///
/// **`CLAUDE.md`, promoted 2026-09-21**: *the application shows a record whose test is gone, so
/// that a rename - which leaves an orphaned record and an unread test - is two things he can see
/// and act on rather than one thing nobody may touch.*
///
/// **Nothing else can reach one.** `review_of` is asked about each test in turn, so it answers for
/// the tests that exist and is never asked about a record that answers to nothing - and under the
/// rule that `reviewed/` is written by no instance, an orphan that cannot be reached is an orphan
/// that cannot be removed. **A test later given that name would open as reviewed** with nobody
/// having read a line of it.
///
/// **`tests/reviewed.rs` refuses one and this is what lets it be obeyed.** The check said the
/// record was wrong; until this, the only hand that could put it right was one the rule forbids.
fn orphaned() -> Vec<String> {
    let tests: BTreeMap<String, ()> = every_test()
        .iter()
        .map(|file| (file.trim_end_matches(".4x").to_string(), ()))
        .collect();
    let Ok(entries) = std::fs::read_dir(records_at()) else {
        return Vec::new();
    };
    let mut found: Vec<String> = entries
        .filter_map(|it| it.ok())
        .map(|it| it.path())
        .filter(|path| path.extension().and_then(|it| it.to_str()) == Some("4x"))
        .filter_map(|path| {
            path.file_stem()
                .and_then(|it| it.to_str())
                .map(str::to_string)
        })
        .filter(|stem| !tests.contains_key(stem))
        .collect();
    found.sort();
    found
}

/// Whether the version of a test Sean read is the version that is there.
///
/// **Whole file, whitespace collapsed.** A comment reworded counts, because in the workflow this
/// is for it is this lane that rewords it - *I don't want to miss anything*. Only formatting is
/// ignored, which is what makes a difference a non-syntax one.
///
/// # Both sides are labelled, and that is not a flourish
///
/// **Only the approved side used to be**, so the other had to be inferred - and with six
/// near-identical citizen rows on each side there was nothing to infer it from. Sean, 2026-09-20:
/// *why does the diff section list both sides as not what I read? Shouldn't I have read at least
/// one of them.* **He had read one of them**, and the report gave him no way to tell which.
///
/// # The gate reads this too, since 2026-09-21
///
/// **`tests/reviewed.rs` calls this, so the page and the gate cannot disagree about what drift
/// is.** `S-149`: the suite ran the working copies and nothing anywhere compared a test to its
/// record - so Sean's reading changed what the gate did by nothing at all.
///
/// **The alternative was to run `reviewed/` instead of `spec/tests/`, and comparing is stronger.**
/// A runner pointed at the records does not run a test that has no record, and the orphan check
/// walks records to tests rather than the other way - so an unread test would simply not run, and
/// the suite would be green while proving less. **This one is red and says which.**
///
/// **Normalized the same way for both, which is the point of it being one function.** A gate
/// stricter than the display would call a test drifted on a page that says it is reviewed, and a
/// reader would have no way to tell which was lying.
///
/// # And each line says which section it stands in
///
/// **A row in a `given` and the same row in a `then` differ only by a quantity**, so six lines of
/// citizens were six lines of citizens and telling them apart meant counting. The section is read
/// off the `{given}`, `{when}`, `{then}` and `{refused}` markers while the file is walked.
///
/// **Everything above the first marker is `note`** - the prose that says what the test is for,
/// which drifts as readily as the rows and matters as much.
pub fn review_of(stem: &str) -> Review {
    let now = std::fs::read_to_string(tests_at().join(format!("{stem}.4x"))).unwrap_or_default();
    let read = std::fs::read_to_string(records_at().join(format!("{stem}.4x"))).ok();
    state_of(read.as_deref(), &now)
}

// **The state logic is `render.rs`'s now**, re-exported so this file's callers and the page read
// the same names. See the note there for why it moved.
#[allow(unused_imports)]
pub use render::{APPROVED, DENIED, NOT_LOOKED, Review, STATES, drift, state_of};

/// What Sean has asked to be changed, per test, read from `reviewed/asked.md`.
///
/// **A note is addressed to this lane and the approval is not**, so the two are kept apart: a copy
/// in `reviewed/` says *I have read this*, and a bullet here says *change this*. **The file is
/// ordinary markdown** - `## <test>` opens a section and each `- ` line is one note - so it can be
/// written by the page, read here, and edited by hand without a format to learn.
pub fn asked() -> BTreeMap<String, Vec<String>> {
    let mut found: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let Ok(text) = std::fs::read_to_string(records_at().join("asked.md")) else {
        return found;
    };
    let mut stem = String::new();
    for line in text.lines() {
        let bare = line.trim();
        if let Some(name) = bare.strip_prefix("## ") {
            stem = name.trim().to_string();
        } else if let Some(note) = bare.strip_prefix("- ")
            && !stem.is_empty()
        {
            found
                .entry(stem.clone())
                .or_default()
                .push(note.to_string());
        }
    }
    found
}

/// What became of one test, in the words the page uses.
enum Outcome {
    Passed,
    /// The source cannot be folded against the data as it stands, so there is nothing to run.
    ///
    /// **A test arrives before the rule it is about**, which is the order `spec/README.md` asks
    /// for: he reads it, then it constrains. **So the page has to show one it cannot run** -
    /// panicking here took the whole application down and `S-214` is about exactly the test that
    /// cannot be shown.
    Unreadable(String),
    Refused(String),
    Differed {
        missing: Vec<String>,
        extra: Vec<String>,
    },
}

/// Where the generated cases live, which is beside `spec/` rather than inside it.
///
/// **A case is generated and a test is approved**, which is why they sit apart: `spec/tests/`
/// holds what Sean has read and `regression/` holds what the suite writes.
/// Where the generated reports live, which is beside `spec/` rather than inside this crate.
fn reports_at() -> PathBuf {
    mine().join("..").join("..").join("reports")
}

pub fn cases_at() -> PathBuf {
    mine().join("..").join("..").join("regression")
}

/// Every generated regression case, by suite.
///
/// **`E-4`**: *a page that lists 222 rows flat is not this capability met - the rule tests, the
/// interface tests and the four regression suites are distinguishable without my counting.*
///
/// **The rows are 223 and the cases are 166.** Measured over `regression/**/*.4x` at `HEAD` and
/// at three earlier commits, 166 every time - so the 222 and the 165 are a miscount rather than
/// drift, and `C-213` asks the specification lane to correct the two *vetted when* lines that
/// carry them. **This renders what is there** rather than what those lines predict, because a
/// page built to a wrong number would make the number look right.
pub fn every_case() -> Vec<(String, Vec<String>)> {
    let at = cases_at();
    let mut suites: Vec<(String, Vec<String>)> = Vec::new();
    let Ok(entries) = std::fs::read_dir(&at) else {
        return suites;
    };
    let mut named: Vec<PathBuf> = entries
        .flatten()
        .map(|it| it.path())
        .filter(|it| it.is_dir())
        .collect();
    named.sort();
    for suite in named {
        let Some(name) = suite.file_name().map(|it| it.to_string_lossy().to_string()) else {
            continue;
        };
        let mut cases = Vec::new();
        walk(&suite, &suite, &mut cases);
        cases.sort();
        let loaded = loaded_by(&suite, &cases);
        cases.retain(|case| !loaded.contains(case));
        if !cases.is_empty() {
            suites.push((name, cases));
        }
    }
    suites
}

/// Collect every `.4x` under a suite, however deeply it nests.
///
/// **`scenario/` nests a turn directory and the others do not**, so a reader that assumed one
/// level would have found the other three suites and none of scenario's thirty-seven - a
/// plausible number over a narrower population, which is the class `CLAUDE.md` names.
fn walk(root: &Path, at: &Path, into: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(at) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(root, &path, into);
        } else if path.extension().is_some_and(|it| it == "4x")
            && let Ok(under) = path.strip_prefix(root)
        {
            into.push(under.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// The files a case loads, which are inputs rather than cases.
///
/// **`regression/scenario/world.4x` is the scenario's world and not a case.** It has no command
/// and no `{then}`, so a verdict on it would observe nothing - and `{regenerate}` on it would
/// rewrite the thing the other thirty-six are compared against. **So it is not a row.**
///
/// # The predicate is the role, not the shape
///
/// **A structural test does not survive the four suites.** `{when}` and `{test name:}` are both
/// true of the thirty-six scenario cases and of none of the other hundred and twenty-nine -
/// `rules/`, `types/` and `primitives/` hold declarations rather than runs. **Either one would
/// have excluded 129 real cases to exclude one input**, which is the class of a predicate read
/// off a single example.
///
/// **What is actually true of `world.4x` is that a case loads it.** That holds over every suite,
/// says why it is not a case rather than what it happens to look like, and **excludes a second
/// input added tomorrow without anyone editing a list.**
///
/// **Comments are dropped before the line is read.** `world.4x` explains the load line in its own
/// header - *a case opens `{load file:world.4x into:game}` and the runner follows it* - so a
/// reader that took the whole file found thirty-seven loaders of a file that has thirty-six.
/// **Quoting a thing and doing it are the same bytes**, which `CLAUDE.md` names.
fn loaded_by(suite: &Path, cases: &[String]) -> BTreeSet<String> {
    let mut loaded = BTreeSet::new();
    for case in cases {
        let Ok(text) = std::fs::read_to_string(suite.join(case)) else {
            continue;
        };
        for line in text.lines().map(str::trim) {
            if line.starts_with('#') {
                continue;
            }
            let Some(at) = line.find("{load file:") else {
                continue;
            };
            let rest = &line[at + "{load file:".len()..];
            let named: String = rest
                .chars()
                .take_while(|it| !it.is_whitespace() && *it != '}')
                .collect();
            if !named.is_empty() {
                loaded.insert(named);
            }
        }
    }
    loaded
}

/// Which suites are shown and offer nothing to press.
///
/// **`E-4`, and it is three suites rather than two.** Sean, 2026-10-01, asked where
/// `regression/rules/`'s sixteen fall: *I was expecting to review 3 things. The tests I was
/// reviewing before. The new user interface tests. And the regression tests. Everything else was
/// to be informational only.*
///
/// **The three he reviews are the rule tests, the interface tests and `regression/scenario/`.**
/// *The regression tests* alone could have meant all four suites; his earlier instruction had
/// already made `types/` and `primitives/` informational, so it cannot - and `rules/` falls in
/// *everything else* rather than being singled out.
///
/// ```text
/// markable      57 rule tests + 0 interface + 36 scenario cases  =  93
/// shown only    16 rules + 53 types + 60 primitives              = 129
/// ```
///
/// **That is the interface declining to offer a control, not the notation forbidding one.**
/// `spec/README.md` rule 3 says *no suite is privileged* and `reviewed/cases.4x` takes a verdict
/// for any case, including one of these - so `render::cases_in` reads a verdict for all four and
/// only the page is narrower.
pub const SHOWN_ONLY: [&str; 3] = ["primitives", "rules", "types"];

/// The regression suites rendered as their own section, each one foldable.
///
/// **`E-4` asks that the suites be distinguishable without his counting**, so each is a fold
/// carrying its own count and saying in its own words whether it offers a control. A flat list of
/// everything is the thing that line rules out.
///
/// **A case he has denied says so whether or not its suite offers a control**, because
/// `reviewed/cases.4x` takes a verdict for any case and the page would otherwise hide one he had
/// made elsewhere - *no suite is privileged* is about what may be recorded.
fn cases_section(marks: &render::Cases, live: bool) -> String {
    let mut out = String::new();
    let suites = every_case();
    let total: usize = suites.iter().map(|(_, cases)| cases.len()).sum();
    out.push_str(&format!(
        "<h2>Regression cases</h2>
<p class=\"note\"><strong>{total}</strong> generated cases          in {} suites. A case is written by the test, so there is nothing to pin and nothing to          compare - a verdict on one records what you think of what it does, and a regeneration is          authorized separately.</p>
",
        suites.len()
    ));
    for (suite, cases) in &suites {
        let shown_only = SHOWN_ONLY.contains(&suite.as_str());
        let how = if shown_only {
            "shown for reading; no control, by your instruction"
        } else if live {
            "markable"
        } else {
            "markable when served"
        };
        out.push_str(&format!(
            "<details class=\"suite\" data-suite=\"{suite}\"><summary><b>{suite}</b> &middot;              {} case(s) &middot; <span class=\"note\">{how}</span></summary>
<ul class=\"cases\">
",
            cases.len()
        ));
        for case in cases {
            let name = format!("{suite}/{}", case.trim_end_matches(".4x"));
            let said = if marks.denied.contains(&name) {
                " <span class=\"red\">denied</span>"
            } else if marks.approved.contains(&name) {
                " <span class=\"ok\">approved</span>"
            } else {
                ""
            };
            let waiting = if marks.authorized.contains(&name) {
                " <span class=\"seen\">regeneration authorized</span>"
            } else {
                ""
            };
            // **Linked whether or not it is markable**, which is the half of his instruction that
            // is about the informational suites: *we can even link to them if it helps with
            // comprehensibility.*
            out.push_str(&format!(
                "<li data-case=\"{name}\"><a href=\"../../regression/{name}.4x\"><code>{case}</code></a>{said}{waiting}"
            ));
            // **The controls are on the page whether or not a server is listening.** Served,
            // the local script posts them; hosted, the writer script commits them with his
            // token. **A copy opened from disk offers them and says it is reading only**, which
            // is honest about where it is rather than pretending the row is unmarkable.
            if !shown_only {
                // **Two controls and not three.** Sean, 2026-10-01: *I can drop the regenerate
                // feature for now, does that make it simpler.* **The gesture is not dropped, only
                // unoffered** - `reviewed/cases.4x` still takes a `{regenerate}` row and the next
                // run still spends it, so what goes is the button rather than the capability.
                //
                // **And the saving is reach rather than code**: a verdict is a file write, so the
                // page needs `Contents: write` and nothing that starts a workflow.
                out.push_str(
                    " <button data-mark=\"approved\">approve</button>                     <button data-mark=\"denied\">deny</button>",
                );
            }
            out.push_str(
                "</li>
",
            );
        }
        out.push_str(
            "</ul></details>
",
        );
    }
    out
}

/// A rendering of the whole suite: the page, its diffable sibling, and the tally.
pub struct Built {
    pub page: String,
    pub log: String,
    pub total: usize,
    pub passed: usize,
    pub red: usize,
    /// How many tests the copy in `reviewed/` still matches.
    pub reviewed: usize,
    /// How many are drifted or were never read, which is what `scripts/reviewed` is for.
    pub unreviewed: usize,
}

/// Run every test and render it.
///
/// **`live` says whether anything is listening.** Served by `review-web` it is true and the page
/// carries the reviewing controls; written to `reports/review/` it is false and the page is what it
/// has always been - a button that writes to disk would be a lie in a file opened from disk.
pub fn build(live: bool) -> Built {
    let notes = asked();
    let data = Directory(mine().join("data").join("foundation"));
    let setup = rows("data/foundation/setup.4x");

    // **The shared ruleset**, which every test is read against: the schema, the rules and the
    // categories. A test's own rows are added per test, because its territories are its own.
    let mut shared = Vec::new();
    for file in ["schema.4x", "engine.4x", "rules.4x"] {
        shared.extend(rows(&format!("data/foundation/{file}")));
    }

    let mut cards = String::new();
    // **The diffable sibling** - `R-9`: every generated view has one. It is also the log:
    // (old state, commands) -> (new state, effects), written out per test.
    let mut log = String::from(
        "The thin engine, per test: the world it starts in, what each command took and made, and
the world it leaves. (old state, commands) -> (new state, effects).
",
    );
    let (mut passed, mut red) = (0usize, 0usize);
    let (mut reviewed, mut unreviewed) = (0usize, 0usize);

    let converting = Converting::ready();
    for file in every_test() {
        let stem = file.trim_end_matches(".4x").to_string();
        // **The source, folded, and read once rather than four times.** It was read from the
        // generated directory at each of the four uses below - which is what `S-214` is, and
        // re-reading a file to get the same rows was the shape that hid it.
        let folded = converting.rows_of(&file);
        let own = folded.clone().unwrap_or_default();

        let mut script = setup.clone();
        script.extend(own.clone());

        // **Names built per test**, because a territory is a test's own and a category is not.
        let mut whole = shared.clone();
        whole.extend(own.clone());
        let names = Names::of(&whole);
        let friendly = |written: &str| -> String {
            match read(written) {
                Ok(parsed) if !parsed.is_empty() => names.row(&parsed[0]),
                _ => written.to_string(),
            }
        };

        // **The world the test starts in**, built the way `run_test` builds it so the log shows
        // what the run saw rather than what the file said.
        let mut world = shared.clone();
        let mut inside = false;
        for row in own.clone() {
            if matches!(row.relation.as_str(), "given" | "when" | "then" | "refused") {
                inside = row.relation == "given";
                continue;
            }
            if inside {
                world.push(row);
            }
        }
        let before = game_model::engine::Game::of(world).ok();

        // **The commands the `when` states**, which is the middle of the fold.
        let mut commands = Vec::new();
        let mut inside = false;
        for row in own.clone() {
            if matches!(row.relation.as_str(), "given" | "when" | "then" | "refused") {
                inside = row.relation == "when";
                continue;
            }
            if inside {
                commands.push(row);
            }
        }

        let outcome = match &folded {
            // **Nothing to run, and the page says so rather than falling over.** The commonest
            // reason is the honest one: the test names a thing the data does not have yet,
            // because it was written to be read before the rule it is about exists.
            Err(why) => Outcome::Unreadable(why.clone()),
            Ok(_) => match run_test(&script, &data) {
                // **Said with names**, the same as every other row on this page. `Refused::told`
                // takes the writer because the engine has none - `tests/isolation.rs`.
                Err(why) => Outcome::Refused(why.told(&|row| names.row(row))),
                Ok(report) if report.same() => Outcome::Passed,
                Ok(report) => Outcome::Differed {
                    missing: report.missing.iter().map(|it| friendly(it)).collect(),
                    extra: report.extra.iter().map(|it| friendly(it)).collect(),
                },
            },
        };

        // **(old state, commands) -> (new state, effects), written out.** The log calls `play`
        // itself rather than reading it back off a report, so what it shows is the fold rather
        // than a reconstruction of it.
        let named = |outline: &str| -> String {
            outline
                .lines()
                .map(|line| match line.trim().strip_prefix("- {") {
                    Some(_) => {
                        let at = line.find("- ").unwrap_or(0) + 2;
                        format!("{}{}", &line[..at], friendly(line[at..].trim()))
                    }
                    None => line.to_string(),
                })
                .collect::<Vec<String>>()
                .join(
                    "
",
                )
        };

        log.push_str(&format!(
            "
{}
{stem}
",
            "=".repeat(78)
        ));
        match &before {
            None => log.push_str(
                "
  the given world does not fit the structure
",
            ),
            Some(before) => {
                log.push_str(
                    "
old state
",
                );
                for line in named(&before.outline()).lines() {
                    log.push_str(&format!(
                        "  {line}
"
                    ));
                }
                match game_model::engine::play(before, &commands) {
                    Err(why) => log.push_str(&format!(
                        "
refused
  {why}
"
                    )),
                    Ok((after, effects)) => {
                        for effect in &effects {
                            log.push_str(&format!(
                                "
command  {}
",
                                names.row(&effect.command)
                            ));
                            for row in &effect.took {
                                log.push_str(&format!(
                                    "  took   {}
",
                                    names.row(row)
                                ));
                            }
                            for row in &effect.made {
                                log.push_str(&format!(
                                    "  made   {}
",
                                    names.row(row)
                                ));
                            }
                        }
                        log.push_str(
                            "
new state
",
                        );
                        for line in named(&after.outline()).lines() {
                            log.push_str(&format!(
                                "  {line}
"
                            ));
                        }
                    }
                }
            }
        }
        log.push_str(&match &outcome {
            Outcome::Passed => "
as expected
"
            .to_string(),
            Outcome::Unreadable(said) => format!(
                "
cannot be read against the data as it stands
  {said}
"
            ),
            Outcome::Refused(said) => format!(
                "
not as expected
  refused  {said}
"
            ),
            Outcome::Differed { missing, extra } => {
                let mut said = String::from(
                    "
not as expected
",
                );
                for row in missing {
                    said.push_str(&format!(
                        "  wanted   {row}
"
                    ));
                }
                for row in extra {
                    said.push_str(&format!(
                        "  got      {row}
"
                    ));
                }
                said
            }
        });

        let Review {
            state: mark,
            earlier,
            lines: drift,
        } = review_of(&stem);
        let seen = match mark {
            APPROVED => {
                reviewed += 1;
                String::new()
            }
            _ => {
                unreviewed += 1;
                let mut said = String::new();
                if !drift.is_empty() {
                    said.push_str("<span class=\"gap\"></span>");
                }
                for (side, line) in &drift {
                    said.push_str(&format!(
                        "<span class=\"drift {side}\">{}</span>",
                        escaped(line)
                    ));
                }
                said
            }
        };

        let (badge, class, why) = match &outcome {
            Outcome::Passed => ("as expected", "ok", String::new()),
            Outcome::Unreadable(said) => ("cannot be read yet", "red", escaped(said)),
            Outcome::Refused(said) => ("refused", "red", escaped(said)),
            Outcome::Differed { .. } => ("not as expected", "red", String::new()),
        };
        if matches!(outcome, Outcome::Passed) {
            passed += 1;
        } else {
            red += 1;
        }

        // **The whole file, line for line**, so nothing about the test is off the page.
        let source = std::fs::read_to_string(tests_at().join(&file))
            .unwrap_or_else(|why| panic!("spec/tests/{file}: {why}"));
        let wanted: BTreeMap<String, ()> = match &outcome {
            Outcome::Differed { missing, .. } => {
                missing.iter().map(|it| (it.clone(), ())).collect()
            }
            _ => BTreeMap::new(),
        };

        // **What moved between the two worlds a test states.** A test says what was there and
        // what is there afterwards, and most of the second is the first - so the report says which
        // is which rather than leaving a reader to compare six citizen rows by eye.
        //
        // **Only where there are two worlds.** A `{refused}` test states one, and a line of it is
        // neither the same as nor different from anything.
        let schema = game_model::schema::Schema::of(&whole).ok();
        let mut marks: BTreeMap<(String, String), (friendly::Change, Option<String>)> =
            BTreeMap::new();
        if let Some(schema) = &schema {
            let (mut given, mut ends): (Vec<Stated>, Vec<Stated>) = (Vec::new(), Vec::new());
            let mut at = "";
            for line in source.lines() {
                let bare = line.trim();
                if matches!(bare, "{given}" | "{when}" | "{then}" | "{refused}") {
                    at = bare;
                    continue;
                }
                let holder = match at {
                    "{given}" => &mut given,
                    "{then}" => &mut ends,
                    _ => continue,
                };
                if bare.starts_with('{')
                    && let Ok(row) = names.parse(bare)
                {
                    holder.push((bare.to_string(), row));
                }
            }
            if !given.is_empty() && !ends.is_empty() {
                let only = |these: &[Stated]| -> Vec<Row> {
                    these.iter().map(|it| it.1.clone()).collect()
                };
                let (was, now) = friendly::compared(schema, &only(&given), &only(&ends));
                for (side, these, changes) in [("{given}", &given, was), ("{then}", &ends, now)] {
                    for ((line, row), change) in these.iter().zip(changes) {
                        let counted = schema
                            .relation(&row.relation)
                            .and_then(|it| it.quantity())
                            .map(str::to_string);
                        marks.insert((side.to_string(), line.clone()), (change, counted));
                    }
                }
            }
        }

        // **A row is marked in the section it is asserted in, and nowhere else.** Marking by text
        // alone put *wanted, not got* on a `{given}` line that happened to read the same as the
        // `{then}` line it was about - the given says what was there, and nothing about it can be
        // missing.
        let mut section = "";
        let mut body = String::new();
        for line in source.lines() {
            let bare = line.trim();
            if matches!(bare, "{given}" | "{when}" | "{then}" | "{refused}") {
                section = bare;
            }
            let kind = if bare.starts_with('#') {
                "said"
            } else if bare.is_empty() {
                "gap"
            } else if matches!(bare, "{given}" | "{when}" | "{then}" | "{refused}") {
                "mark"
            } else if matches!(section, "{then}" | "{refused}") && wanted.contains_key(bare) {
                "missing"
            } else {
                "row"
            };
            // **No newline after the span, and that is the whole of the spacing.** A `<pre>`
            // keeps the newlines in its text and these spans are `display: block`, so a `\n`
            // between them ended the line a second time and every row rendered with a blank one
            // beneath it. **The block is what ends the line**; the newline was a second ending.
            // **What became of this row, where there are two worlds to compare.** It is a second
            // class rather than a different one, so a line the run disagrees about keeps the mark
            // that says so - the run's quarrel with the test is the louder thing.
            let (mut moved, mut shown) = (String::new(), escaped(line));
            if kind == "row"
                && let Some((change, counted)) = marks.get(&(section.to_string(), bare.to_string()))
            {
                moved = match change {
                    friendly::Change::Same => " same",
                    friendly::Change::Changed(differ) => {
                        shown = celled(line, differ, counted.as_deref());
                        " changed"
                    }
                    friendly::Change::Gone => " gone",
                    friendly::Change::New => " new",
                }
                .to_string();
            }
            body.push_str(&format!("<span class=\"{kind}{moved}\">{shown}</span>"));
        }
        if let Outcome::Differed { extra, .. } = &outcome {
            for one in extra {
                body.push_str(&format!("<span class=\"extra\">{}</span>", escaped(one)));
            }
        }

        // **Where the test is on disk, as a link to it there.** Sean, 2026-09-21, asking for
        // the tests to be browsable from the deployment. **The address is the answer**: the path
        // in the URL is the path in the repository, so the link says where the file is as well as
        // showing it.
        //
        // **Only when something is serving**, for the same reason the review buttons are. A page
        // opened from disk would link at a server that is not there, and a relative link to a
        // `.4x` would be offered as a download rather than shown - which is the whole reason the
        // server declares `text/plain` and no `.txt` copy exists.
        // **The foundation link only where there is a foundation form** - `S-214`. A test that
        // has never been read has no generated copy, so the second link offered a 404 beside the
        // one file the reader actually needs. **Asked of the disk rather than of the outcome**,
        // because a test can be unread and still readable, and unreadable and still reviewed.
        let has_foundation = render::mine()
            .join(render::foundation_at(&format!("tests/{file}")))
            .exists();
        let raw = if live {
            let foundation = if has_foundation {
                format!(" · <a href=\"/data/foundation/tests/{stem}.4x\">foundation</a>")
            } else {
                " · not read yet, so there is no foundation form".to_string()
            };
            format!(
                "<p class=\"raw\">on disk: <a href=\"/spec/tests/rule/{stem}.4x\">spec/tests/rule/{stem}.4x</a>{foundation}</p>\n"
            )
        } else {
            // **A `.txt` twin, because a published `.4x` is a download.** Measured in the
            // mainline and ruled on by Sean as `S-134`: GitHub Pages picks the type from the
            // extension, does not know this one, and offers no way to override it. The twin is
            // written at deploy and committed nowhere, so **these two links resolve on the site
            // and not in a clone** - which the note under the tally says out loud.
            format!(
                "<p class=\"raw\">on disk: <a href=\"spec/tests/rule/{stem}.4x.txt\">spec/tests/rule/{stem}.4x</a> · <a href=\"data/foundation/tests/{stem}.4x.txt\">foundation</a></p>\n"
            )
        };
        let said = if why.is_empty() {
            String::new()
        } else {
            format!("<p class=\"why\">{why}</p>\n")
        };
        // **Open when red and folded when not**, so a run that is mostly green opens on what went
        // wrong. `<details>` is plain HTML and needs no script, which is what lets the page open
        // from disk - the same reason `R-9` gives for a report that filters being a page rather
        // than a click.
        let open = if matches!(outcome, Outcome::Passed) {
            ""
        } else {
            " open"
        };
        // **Denied is its own class and not *unseen*.** He has looked; the code is not bound.
        // Showing it as unread would ask him to read it again, which is the one thing a denial
        // says he has already done.
        let seen_class = match mark {
            APPROVED => "seen",
            DENIED => "denied",
            _ => "unseen",
        };
        // **The drift is colour beside the state, not a state of its own** - `P-611`. A test whose
        // rows changed is in the first state, and *you approved an earlier version* is worth
        // seeing while he is deciding whether to read it again.
        let was = match earlier {
            Some(true) => {
                " <span class=\"badge earlier\" data-earlier>you denied an earlier version</span>"
            }
            Some(false) => {
                " <span class=\"badge earlier\" data-earlier>you approved an earlier version</span>"
            }
            None => "",
        };
        // **The controls exist only when something is listening**, which is why `build` is told.
        let acts = if live {
            "<span class=\"acts\"><button data-do=\"reviewed\">reviewed</button><button data-do=\"denied\">deny</button><button data-do=\"asked\">needs changing</button><button data-do=\"unreview\">unreview</button></span>"
        } else {
            ""
        };
        let mut wants = String::new();
        for note in notes.get(&stem).into_iter().flatten() {
            wants.push_str(&format!("<li>{}</li>", escaped(note)));
        }
        // **What you asked for sits with the test it is about**, rather than in a list elsewhere.
        let how_many = notes.get(&stem).map(Vec::len).unwrap_or(0);
        let chip = if how_many == 0 {
            String::new()
        } else {
            format!(" <span class=\"badge noted\" data-noted>{how_many} asked</span>")
        };
        let noted = if wants.is_empty() {
            String::new()
        } else {
            format!("<ul class=\"asked\">{wants}</ul>\n")
        };
        // **The record this test would get, carried by the page that offers the button.**
        // `E-4`: the hosted page writes a verdict with a `Contents: write` token, so it has to
        // know the bytes - there is no server to ask. **The body is identical for both
        // verdicts**, so one copy travels and the script writes the state line above it.
        let would_write = render::record_for(&stem, &source, "approved")
            .ok()
            .and_then(|it| it.split_once('\n').map(|(_, rest)| rest.to_string()))
            .map(|body| format!(" data-record=\"{}\"", encoded(&body)))
            .unwrap_or_default();
        cards.push_str(&format!(
            "<details class=\"test {class}\"{open} data-test=\"{stem}\"{would_write}>\n<summary><span class=\"name\">{stem}</span> <span class=\"badge {class}\">{badge}</span> <span class=\"badge {seen_class}\" data-mark>{mark}</span>{was}{chip}{acts}</summary>\n{raw}{said}{noted}<pre>{body}{seen}</pre>\n</details>\n"
        ));
    }

    // **A record whose test is gone, shown so that it can be taken back.** It sits above the
    // tests rather than among them, because it is not one - there is nothing to read, and the
    // only thing to decide is whether the reading it records still means anything.
    let mut lost = String::new();
    for stem in orphaned() {
        let acts = if live {
            "<span class=\"acts\"><button data-do=\"unreview\">unreview</button></span>"
        } else {
            ""
        };
        lost.push_str(&format!(
            "<details class=\"test red\" open data-test=\"{stem}\">\n<summary><span class=\"name\">{stem}</span> <span class=\"badge red\">no such test</span>{acts}</summary>\n<p class=\"why\">This records a reading of a test that is not there. A test later given this name would open as reviewed with nobody having read it. <strong>Unreview takes the record back</strong>; if the test was renamed, its new name is somewhere below waiting to be read.</p>\n</details>\n"
        ));
    }

    let total = passed + red;
    // **A way in to the files themselves**, which only exists where something is serving them.
    let browse = if live {
        " <a href=\"/data\">Browse the data files</a>."
    } else {
        // **The same sentence `reports/index.html` carries**, for the same reason: a generated
        // copy is not canonical, and a link to one that is written at deploy is dead in a clone.
        " Each test links to its source as a <code>.txt</code> twin, which is written when the          site is deployed and committed nowhere - so those links resolve at          <code>/game4x/reports/review/</code> and not in a clone."
    };
    let body_attribute = if live { " data-live" } else { "" };
    // **Served, the local script posts; hosted, the writer commits with his token.** Never
    // both: one of them would re-send what the other already wrote.
    let script = if live {
        format!("<script>{SCRIPT}</script>\n")
    } else {
        format!("<script>{WRITER}</script>\n")
    };
    // **What he has said about the cases**, read the same way the suite reads it. An absent
    // file is no verdict on anything, which is the state before he has pressed anything.
    let marks = std::fs::read_to_string(records_at().join("cases.4x"))
        .ok()
        .and_then(|text| render::cases_in(&text).ok())
        .unwrap_or_default();
    let cases = cases_section(&marks, live);
    // **`S-242`: the conversion was a verb, a port and a path he had to know.** Every other
    // gesture in this application is a key or a button, and this one asked him to write a `curl` -
    // so `E-1` was built and unvettable at once.
    //
    // **It says he has to commit what it writes**, because the published page is generated from
    // `reviewed/` and an uncommitted conversion leaves the hosted page showing the old order with
    // nothing saying so.
    let convert = if live {
        // **A record is behind if rewriting it from its test, with its own verdict, differs.**
        // One question, asked once - the verdict is read off the record because carrying it over
        // is the thing a conversion may not lose.
        let behind = every_test()
            .iter()
            .filter(|file| {
                let name = file.trim_end_matches(".4x");
                let Ok(was) = std::fs::read_to_string(records_at().join(file)) else {
                    return false;
                };
                let Ok(text) = std::fs::read_to_string(tests_at().join(file)) else {
                    return false;
                };
                let verdict = match render::verdict_of(&was) {
                    Ok(render::Verdict::Denied) => "denied",
                    Ok(render::Verdict::Approved) => "approved",
                    Err(_) => return false,
                };
                render::record_for(name, &text, verdict).is_ok_and(|now| now != was)
            })
            .count();
        match behind {
            0 => "<p class=\"note\" data-convert=\"none\">Every record is in the canonical column order.</p>\n".to_string(),
            n => format!("<p class=\"note\" data-convert=\"{n}\"><strong>{n}</strong> record(s) are in the schema's column order rather than the canonical one. Converting changes the order and not what any record says, and stops without writing anything if it would. <button data-do=\"convert\">convert {n}</button> <b>Commit what it writes</b> - the published page is generated from <code>reviewed/</code>, so an uncommitted conversion leaves that page showing the old order and saying nothing.</p>\n"),
        }
    } else {
        String::new()
    };
    let case_count: usize = every_case().iter().map(|(_, it)| it.len()).sum();
    let rows = total + case_count;

    let page = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>Review</title>\n<style>{STYLE}</style>\n</head>\n<body{body_attribute}>\n<h1>Review</h1>\n<p class=\"tally\"><strong>{rows}</strong> rows &middot; <strong>{total}</strong> tests &middot; <span class=\"ok\">{passed} as expected</span> &middot; <span class=\"red\">{red} red</span> &middot; <span class=\"seen\" data-tally=\"seen\">{reviewed} reviewed</span> &middot; <span class=\"unseen\" data-tally=\"unseen\">{unreviewed} to read</span></p>\n<p class=\"note\">Generated by <code>cargo run --example report</code>.{browse} Each test is shown whole, in the friendly form. A line the run wanted and did not get is marked <span class=\"key missing\">so</span>; one it got and did not want is marked <span class=\"key extra\">so</span>.</p>\n<p class=\"note\">Between a test's two worlds: a row that did not change is <span class=\"key same\">dimmed</span>, a row that did has the cells that differ marked <span class=\"cell\">so</span>, and a row in one world and not the other says which. <b>Two rows that are the same thing are paired only where there is exactly one of them on each side</b> - so a spent extractor and a fresh one over one deposit are reported as gone and new rather than as one of them changing, because which became which has no answer.</p>\n{convert}{lost}{cards}{cases}{script}</body>\n</html>\n"
    );
    Built {
        page,
        log,
        total,
        passed,
        red,
        reviewed,
        unreviewed,
    }
}

fn main() {
    let built = build(false);
    // **Written under `reports/` so the root reports page can link it** - `S-249`. Sean went
    // looking for the page `E-4` is vetted against and could not find it: it was published at
    // `reports/thin-engine/`, named after a crate that stopped existing on 2026-09-25, and
    // `reports/index.md` linked twelve pages and not this one.
    //
    // **`reports/review/index.html`, which is the address he chose**: *reports review is good*,
    // and *it should be reachable from the root reports page.*
    //
    // **One level down, which is what makes the link checkable.** `tests/browsable.rs` reads the
    // top level of `reports/` and checks that every link resolves - so the link to this page is
    // held, and this page's own links are not descended into. **That is the right split**: its
    // links point at `data/` and `spec/tests/`, which the pipeline copies beside it in the
    // artifact and a clone does not have.
    //
    // **`review-web` is how it is read locally** and serves the page live with working links, so
    // nothing is lost by the committed copy being the deployed one.
    let into = reports_at().join("review");
    std::fs::create_dir_all(&into).expect("reports/review");
    std::fs::write(into.join("index.html"), &built.page).expect("index.html");
    std::fs::write(into.join("index.txt"), &built.log).expect("index.txt");
    let (total, passed, red) = (built.total, built.passed, built.red);
    let (seen, unseen) = (built.reviewed, built.unreviewed);
    println!(
        "reports/review/: {total} tests, {passed} as expected, {red} red, {seen} reviewed, {unseen} to read"
    );
}

/// **Both themes, because a report nobody can read in their own is not one** - the same rule
/// `R-10` states for a generated drawing. Every mark is an alpha over whatever the page sits on,
/// so it lands on white and on black.
const WRITER: &str = r#"
// **The hosted page writes a verdict, and starts nothing** - `E-4`. Sean, 2026-10-01: *I can
// drop the regenerate feature for now, does that make it simpler.* It does: a verdict is a file
// write, so this needs `Contents: write` and never `Actions: write`.
//
// **The token is his and this page is him writing.** It is held in `localStorage` on this origin,
// sent only as an `Authorization` header to `api.github.com`, and never put in a URL, a query
// string, a log line or the DOM. **Nothing here reads it back out for display.**
(() => {
  const cards = [...document.querySelectorAll('details[data-test][data-record]')];
  const cases = [...document.querySelectorAll('li[data-case] button[data-mark]')];
  if (!cards.length && !cases.length) return;

  // **Derived from where the page is, rather than written in.** A Pages URL says who owns it:
  // `https://<owner>.github.io/<repo>/...`. So the page carries no repository name to go stale,
  // and a copy served from anywhere else simply offers no writing.
  const host = location.hostname.match(/^([^.]+)\.github\.io$/);
  const repo = host && location.pathname.split('/').filter(Boolean)[0];
  const owner = host && host[1];
  const api = owner && repo ? `https://api.github.com/repos/${owner}/${repo}/contents` : null;

  const KEY = 'game4x-token';
  const bar = document.createElement('p');
  bar.className = 'note';
  document.querySelector('.tally').after(bar);

  const held = () => {
    try { return localStorage.getItem(KEY) || ''; } catch { return ''; }
  };

  // **A write that worked and a write that failed must not look alike** - `S-253`. Sean wrote a
  // record from his phone, the bar said *is approved.* at `opacity: .7`, the card went on reading
  // *never reviewed*, and he reported it as nothing having happened. **That is the one
  // distinction this surface exists to make.**
  const say = (what, how) => {
    bar.textContent = what;
    bar.className = how ? `note ${how}` : 'note';
  };

  const draw = () => {
    if (!api) {
      say('Reading only: this page writes just from its Pages URL, which says which repository to write to.');
      return;
    }
    bar.textContent = '';
    if (held()) {
      const out = document.createElement('button');
      out.textContent = 'forget token';
      out.onclick = () => {
        try { localStorage.removeItem(KEY); } catch {}
        draw();
      };
      bar.append(`Writing to ${owner}/${repo} as you. `, out);
      return;
    }
    const field = document.createElement('input');
    field.type = 'password';
    field.placeholder = 'fine-grained token, Contents: write';
    field.autocomplete = 'off';
    const save = document.createElement('button');
    save.textContent = 'hold it';
    save.onclick = () => {
      if (!field.value) return;
      try { localStorage.setItem(KEY, field.value); } catch {}
      field.value = '';
      draw();
    };
    bar.append('Reading only until you paste a token. It stays in this browser. ', field, ' ', save);
  };
  draw();

  // **Read before writing, because the API needs the blob it is replacing.** A file that is not
  // there has no sha and is created instead, which is what an unread test is.
  const put = async (path, content, message) => {
    const token = held();
    if (!token || !api) throw new Error('no token');
    const headers = { Authorization: `Bearer ${token}`, Accept: 'application/vnd.github+json' };
    let sha;
    const found = await fetch(`${api}/${path}`, { headers });
    if (found.ok) sha = (await found.json()).sha;
    else if (found.status !== 404) throw new Error(`reading ${path}: ${found.status}`);
    const sent = await fetch(`${api}/${path}`, {
      method: 'PUT',
      headers: { ...headers, 'Content-Type': 'application/json' },
      body: JSON.stringify({ message, content, sha }),
    });
    if (!sent.ok) throw new Error(`writing ${path}: ${sent.status}`);
  };

  const encode = (text) => btoa(String.fromCharCode(...new TextEncoder().encode(text)));

  // **The badge the page rendered, moved to what he just said.** One place, because a test's card
  // and a case's row carry the same three words and would otherwise drift apart.
  const WORD = { approved: 'reviewed', denied: 'denied' };
  const CLASS = { approved: 'seen', denied: 'denied' };
  const mark = (card, state) => {
    const badge = card.querySelector('[data-mark]');
    if (!badge) return;
    badge.textContent = WORD[state] || state;
    badge.className = `badge ${CLASS[state] || 'unseen'}`;
    // **The colour beside it goes, because it was about a verdict that is no longer the latest
    // one.** *You approved an earlier version* is true until he says something newer.
    card.querySelector('[data-earlier]')?.remove();
  };
  const said = (row, state) => {
    row.querySelector('.verdict')?.remove();
    const badge = document.createElement('span');
    badge.className = `verdict ${state === 'denied' ? 'red' : 'ok'}`;
    badge.textContent = ` ${state}`;
    row.append(badge);
  };

  // **A test's record is the verdict line and the body the page carries.** The body is identical
  // for both verdicts, which is why one copy travels.
  for (const card of cards) {
    const name = card.dataset.test;
    for (const [state, label] of [['approved', 'approve'], ['denied', 'deny']]) {
      const press = document.createElement('button');
      press.textContent = label;
      press.onclick = async () => {
        press.disabled = true;
        try {
          const body = atob(card.dataset.record);
          await put(
            `reviewed/rule/${name}.4x`,
            encode(`{verdict state:${state}}\n${body}`),
            `${state}: ${name}`,
          );
          // **Flipped from what this already knows, not fetched.** The page is static, so a
          // refresh cannot help - it is rendered at build time and would come back saying the
          // same thing until the pipeline runs again.
          mark(card, state);
          say(`${name} is ${state}.`, 'ok');
        } catch (why) {
          say(`${name} was not written: ${why.message}`, 'red');
        }
        press.disabled = false;
      };
      card.querySelector('summary').append(' ', press);
    }
  }

  // **A case's verdict is a row in one file**, so writing one is read, replace, write - and the
  // row for that case is replaced rather than appended, or a second press would say both things.
  for (const press of cases) {
    const row = press.closest('li[data-case]');
    const name = row.dataset.case;
    const state = press.dataset.mark;
    press.onclick = async () => {
      press.disabled = true;
      try {
        const token = held();
        let was = '';
        const found = await fetch(`${api}/reviewed/cases.4x`, {
          headers: { Authorization: `Bearer ${token}`, Accept: 'application/vnd.github+json' },
        });
        if (found.ok) was = atob((await found.json()).content.replace(/\s/g, ''));
        const kept = was
          .split('\n')
          .filter((line) => line.trim() && !line.includes(`case:${name} `) && !line.includes(`case:${name}}`));
        kept.push(`{verdict case:${name} state:${state}}`);
        await put('reviewed/cases.4x', encode(kept.join('\n') + '\n'), `${state}: ${name}`);
        said(row, state);
        say(`${name} is ${state}.`, 'ok');
      } catch (why) {
        say(`${name} was not written: ${why.message}`, 'red');
      }
      press.disabled = false;
    };
  }
})();
"#;

const SCRIPT: &str = r#"
// **Served, never written to disk.** `report.html` carries no script - see the top of `report.rs`.
(() => {
  const cards = [...document.querySelectorAll('details[data-test]')];
  if (!cards.length) return;
  let here = 0;

  const bar = document.createElement('p');
  bar.className = 'keys';
  bar.innerHTML = '<b>&uarr;</b>/<b>&darr;</b> move &middot; <b>Enter</b> open'
    + ' &middot; <b>r</b> reviewed &middot; <b>x</b> needs changing &middot; <b>u</b> unreview';
  document.querySelector('.note').after(bar);

  const flash = document.createElement('p');
  flash.className = 'flash';
  flash.hidden = true;
  bar.after(flash);
  const said = (why) => { flash.textContent = why; flash.hidden = false; };

  const show = () => {
    cards.forEach((c, i) => c.classList.toggle('here', i === here));
    cards[here].scrollIntoView({ block: 'nearest' });
  };

  const tally = () => {
    const seen = cards.filter(c => c.querySelector('[data-mark]').textContent === 'reviewed').length;
    document.querySelector('[data-tally="seen"]').textContent = seen + ' reviewed';
    document.querySelector('[data-tally="unseen"]').textContent = (cards.length - seen) + ' to read';
  };

  const post = async (what, name, note) => {
    try {
      const it = await fetch('/' + what, {
        method: 'POST',
        body: JSON.stringify({ name: name, note: note || '' })
      });
      const answer = (await it.text()).trim();
      if (!it.ok) { said(answer); return null; }
      flash.hidden = true;
      return answer;
    } catch (why) {
      // **The server is what makes a click true.** With nothing listening the page must say so
      // rather than flip a badge it cannot back up.
      said('nothing is listening - is `cargo run --example review-web` still running?');
      return null;
    }
  };

  const mark = async (card, what) => {
    const answer = await post(what, card.dataset.test);
    if (answer === null) return;
    const badge = card.querySelector('[data-mark]');
    badge.textContent = answer;
    badge.className = 'badge ' + (answer === 'reviewed' ? 'seen' : 'unseen');
    tally();
  };

  const note = (card, text) => {
    let list = card.querySelector('.asked');
    if (!list) {
      list = document.createElement('ul');
      list.className = 'asked';
      card.querySelector('summary').after(list);
    }
    const one = document.createElement('li');
    one.textContent = text;
    list.append(one);
    let chip = card.querySelector('[data-noted]');
    if (!chip) {
      chip = document.createElement('span');
      chip.className = 'badge noted';
      chip.setAttribute('data-noted', '');
      card.querySelector('[data-mark]').after(chip);
    }
    chip.textContent = list.children.length + ' asked';
  };

  // **An inline box rather than a `prompt`.** A modal dialog stops the page dead, and this page
  // is meant to be gone through quickly.
  const ask = (card) => {
    card.open = true;
    let box = card.querySelector('.ask');
    if (!box) {
      box = document.createElement('div');
      box.className = 'ask';
      box.innerHTML = '<input type="text" placeholder="what needs changing? Enter to file, Esc to drop">';
      card.querySelector('summary').after(box);
      box.querySelector('input').addEventListener('keydown', async (e) => {
        e.stopPropagation();
        const input = e.target;
        if (e.key === 'Enter' && input.value.trim()) {
          const answer = await post('asked', card.dataset.test, input.value.trim());
          if (answer === null) return;
          note(card, input.value.trim());
          input.value = '';
          box.hidden = true;
        } else if (e.key === 'Escape') {
          box.hidden = true;
          input.value = '';
        }
      });
    }
    box.hidden = false;
    box.querySelector('input').focus();
  };

  document.addEventListener('keydown', (e) => {
    if (e.target.tagName === 'INPUT' || e.metaKey || e.ctrlKey || e.altKey) return;
    // **The arrows are the keys and j/k are aliases.** Sean, 2026-09-18: *The j/k to move is
    // unintuitive on the review-web app.* It was vim muscle memory rather than anything a reader
    // would guess; the aliases cost a line and the bar advertises the arrows.
    const down = () => { here = Math.min(here + 1, cards.length - 1); show(); };
    const up = () => { here = Math.max(here - 1, 0); show(); };
    const keys = {
      ArrowDown: down,
      ArrowUp: up,
      j: down,
      k: up,
      r: () => mark(cards[here], 'reviewed'),
      u: () => mark(cards[here], 'unreview'),
      x: () => ask(cards[here]),
      Enter: () => { cards[here].open = !cards[here].open; }
    };
    const act = keys[e.key];
    if (!act) return;
    e.preventDefault();
    act();
  });

  document.addEventListener('click', (e) => {
    const button = e.target.closest('button[data-do]');
    if (!button) return;
    e.preventDefault();
    // **The conversion is not about one test**, so it is the one gesture with no card - `S-242`.
    if (button.dataset.do === 'convert') {
      button.disabled = true;
      button.textContent = 'converting';
      fetch('/convert', { method: 'POST' })
        .then((r) => r.text())
        .then((said) => {
          button.replaceWith(said + ' - now commit what it wrote, or the published page keeps the old order');
        })
        .catch((why) => { button.disabled = false; button.textContent = 'convert failed: ' + why.message; });
      return;
    }
    const card = button.closest('details[data-test]');
    here = cards.indexOf(card);
    show();
    if (button.dataset.do === 'asked') ask(card); else mark(card, button.dataset.do);
  });

  show();
})();
"#;

const STYLE: &str = r#"
:root { color-scheme: light dark }
body {
  font: 15px/1.6 ui-monospace, SFMono-Regular, Menlo, monospace;
  margin: 2rem auto; max-width: 62rem; padding: 0 1rem;
}
h1 { font-size: 1.3rem; margin: 0 0 .25rem }
p { margin: .4rem 0 }
.tally { font-size: 1rem }
.note { opacity: .7; font-size: .85rem; margin-bottom: 1.5rem }
/* **What the writer says is not a footnote** - `S-253`. The bar reporting a write sits at the
   same weight as the tally, because the one thing it says is whether what he pressed landed. */
.note.ok, .note.red { opacity: 1; font-size: 1rem; font-weight: bold }
.ok { color: rgb(30 130 60) }
.red { color: rgb(190 50 50) }
@media (prefers-color-scheme: dark) {
  .ok { color: rgb(110 200 140) }
  .red { color: rgb(255 130 130) }
}
.test {
  border: 1px solid rgba(127,127,127,.35);
  border-left: 4px solid rgba(127,127,127,.5);
  border-radius: .3rem; padding: .7rem .9rem; margin: .7rem 0;
}
.test.red { border-left-color: rgb(190 50 50) }
.test.ok { border-left-color: rgb(30 130 60) }
summary { cursor: pointer; font-weight: 600 }
summary::marker { opacity: .5 }
details[open] > summary { margin-bottom: .45rem }
.name { font-weight: 600 }
.badge {
  font-size: .75rem; font-weight: 600; letter-spacing: .02em;
  padding: .1rem .45rem; border-radius: .2rem; border: 1px solid currentColor;
}
.why { font-size: .9rem; margin: 0 0 .6rem }
.raw { font-size: .8rem; opacity: .7; margin: 0 0 .5rem }
pre {
  margin: 0; overflow-x: auto; background: rgba(127,127,127,.08);
  padding: .7rem .9rem; border-radius: .25rem;
}
pre > span { display: block; padding: 0 .3rem; border-left: 3px solid transparent }
.said { opacity: .55 }
.gap { height: .8em }
.mark { font-weight: 700 }
.same { opacity: .5 }
.changed { border-left-color: rgba(90,130,220,.75) }
.cell { background: rgba(90,130,220,.28); border-radius: .2rem; padding: 0 .15rem }
.gone { border-left-color: rgba(140,140,140,.9) }
.gone::after { content: " <- gone by then"; opacity: .55; font-size: .8em }
.new { border-left-color: rgba(40,150,110,.9) }
.new::after { content: " <- new in then"; opacity: .55; font-size: .8em }
@media (prefers-color-scheme: dark) {
  .cell { background: rgba(120,160,250,.3) }
}
.missing { background: rgba(200,40,40,.16); border-left-color: rgb(190 50 50) }
.missing::after { content: " <- wanted, not got"; opacity: .7; font-size: .8em }
.extra { background: rgba(210,130,0,.18); border-left-color: rgb(200 120 0) }
.extra::after { content: " <- got, not wanted"; opacity: .7; font-size: .8em }
.seen { color: rgb(70 110 160) }
.unseen { color: rgb(170 100 0) }
@media (prefers-color-scheme: dark) {
  .seen { color: rgb(140 180 230) }
  .unseen { color: rgb(230 180 90) }
}
.drift { background: rgba(120,120,200,.16); border-left-color: rgb(90 110 190) }
.drift::after { opacity: .7; font-size: .8em }
.drift.now::after { content: " <- not what you read" }
.drift.was::after { content: " <- what you read" }
.keys { font-size: .85rem; opacity: .8; margin: -1rem 0 1rem }
.keys b { font-weight: 700; opacity: 1 }
.flash { color: rgb(190 50 50); font-size: .9rem; margin-bottom: 1rem }
.test.here { outline: 2px solid rgba(90,130,220,.8); outline-offset: 2px }
.acts { float: right; font-weight: 400 }
.acts button {
  font: inherit; font-size: .75rem; cursor: pointer; margin-left: .3rem;
  padding: .1rem .5rem; border-radius: .2rem; color: inherit;
  border: 1px solid rgba(127,127,127,.5); background: rgba(127,127,127,.1);
}
.acts button:hover { background: rgba(127,127,127,.25) }
.asked { margin: .2rem 0 .6rem; padding-left: 1.2rem; font-size: .9rem }
.asked li, .noted { color: rgb(170 100 0) }
@media (prefers-color-scheme: dark) { .asked li, .noted { color: rgb(230 180 90) } }
.ask { margin: .3rem 0 .6rem }
.ask input { font: inherit; font-size: .85rem; width: 100%; padding: .3rem .4rem;
  border-radius: .2rem; border: 1px solid rgba(127,127,127,.5);
  background: rgba(127,127,127,.08); color: inherit }
.key { display: inline; padding: 0 .3rem; border-left: 3px solid }
.key::after { content: "" }
"#;
