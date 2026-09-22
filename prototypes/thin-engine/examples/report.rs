//! Write `report.html`: every test, whole, with whatever failed marked in it.
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

use std::collections::BTreeMap;
use std::path::PathBuf;

#[path = "../tests/common/friendly.rs"]
#[allow(dead_code)]
mod friendly;

use friendly::Names;
use thin_engine::notation::{Row, read};
use thin_engine::script::{Files, run_test};

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
    mine().join("..").join("..").join("spec").join("tests")
}

/// Where the record of what Sean has read lives.
///
/// **No instance writes it** - `CLAUDE.md`. The review application does, acting as him, and that
/// application is this prototype's.
pub fn records_at() -> PathBuf {
    mine().join("..").join("..").join("reviewed")
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
pub fn every_test() -> Vec<String> {
    let mut found: Vec<String> =
        std::fs::read_dir(mine().join("data").join("foundation").join("tests"))
            .expect("data/foundation/tests")
            .filter_map(|it| it.ok())
            .filter_map(|it| it.file_name().to_str().map(str::to_string))
            .filter(|name| name.ends_with(".4x"))
            .collect();
    found.sort();
    found
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
pub fn review_of(stem: &str) -> (&'static str, Vec<(&'static str, String)>) {
    let now = std::fs::read_to_string(tests_at().join(format!("{stem}.4x"))).unwrap_or_default();
    let read = std::fs::read_to_string(records_at().join(format!("{stem}.4x"))).ok();
    drift(read.as_deref(), &now)
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
    let bare = |text: &str| -> Vec<(String, String)> {
        let mut section = "note";
        let mut out: Vec<(String, String)> = Vec::new();
        for line in text.lines() {
            let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
            if line.is_empty() {
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
            }
            out.push((section.to_string(), line));
        }
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
    Refused(String),
    Differed {
        missing: Vec<String>,
        extra: Vec<String>,
    },
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
/// carries the reviewing controls; written to `report.html` it is false and the page is what it
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

    for file in every_test() {
        let stem = file.trim_end_matches(".4x").to_string();
        let own = rows(&format!("data/foundation/tests/{file}"));

        let mut script = setup.clone();
        script.extend(own.clone());

        // **Names built per test**, because a territory is a test's own and a category is not.
        let mut whole = shared.clone();
        whole.extend(own);
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
        for row in rows(&format!("data/foundation/tests/{file}")) {
            if matches!(row.relation.as_str(), "given" | "when" | "then" | "refused") {
                inside = row.relation == "given";
                continue;
            }
            if inside {
                world.push(row);
            }
        }
        let before = thin_engine::engine::Game::of(world).ok();

        // **The commands the `when` states**, which is the middle of the fold.
        let mut commands = Vec::new();
        let mut inside = false;
        for row in rows(&format!("data/foundation/tests/{file}")) {
            if matches!(row.relation.as_str(), "given" | "when" | "then" | "refused") {
                inside = row.relation == "when";
                continue;
            }
            if inside {
                commands.push(row);
            }
        }

        let outcome = match run_test(&script, &data) {
            Err(why) => Outcome::Refused(format!("{why}")),
            Ok(report) if report.same() => Outcome::Passed,
            Ok(report) => Outcome::Differed {
                missing: report.missing.iter().map(|it| friendly(it)).collect(),
                extra: report.extra.iter().map(|it| friendly(it)).collect(),
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
                match thin_engine::engine::play(before, &commands) {
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

        let (mark, drift) = review_of(&stem);
        let seen = match mark {
            "reviewed" => {
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
        let schema = thin_engine::schema::Schema::of(&whole).ok();
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
        let raw = if live {
            format!(
                "<p class=\"raw\">on disk: <a href=\"/spec/tests/{stem}.4x\">spec/tests/{stem}.4x</a> · <a href=\"/data/foundation/tests/{stem}.4x\">foundation</a></p>\n"
            )
        } else {
            // **A `.txt` twin, because a published `.4x` is a download.** Measured in the
            // mainline and ruled on by Sean as `S-134`: GitHub Pages picks the type from the
            // extension, does not know this one, and offers no way to override it. The twin is
            // written at deploy and committed nowhere, so **these two links resolve on the site
            // and not in a clone** - which the note under the tally says out loud.
            format!(
                "<p class=\"raw\">on disk: <a href=\"spec/tests/{stem}.4x.txt\">spec/tests/{stem}.4x</a> · <a href=\"data/foundation/tests/{stem}.4x.txt\">foundation</a></p>\n"
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
        let seen_class = if mark == "reviewed" { "seen" } else { "unseen" };
        // **The controls exist only when something is listening**, which is why `build` is told.
        let acts = if live {
            "<span class=\"acts\"><button data-do=\"reviewed\">reviewed</button><button data-do=\"asked\">needs changing</button><button data-do=\"unreview\">unreview</button></span>"
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
        cards.push_str(&format!(
            "<details class=\"test {class}\"{open} data-test=\"{stem}\">\n<summary><span class=\"name\">{stem}</span> <span class=\"badge {class}\">{badge}</span> <span class=\"badge {seen_class}\" data-mark>{mark}</span>{chip}{acts}</summary>\n{raw}{said}{noted}<pre>{body}{seen}</pre>\n</details>\n"
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
        " Each test links to its source as a <code>.txt</code> twin, which is written when the          site is deployed and committed nowhere - so those links resolve at          <code>/game4x/reports/thin-engine/</code> and not in a clone."
    };
    let body_attribute = if live { " data-live" } else { "" };
    let script = if live {
        format!("<script>{SCRIPT}</script>\n")
    } else {
        String::new()
    };
    let page = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>thin-engine tests</title>\n<style>{STYLE}</style>\n</head>\n<body{body_attribute}>\n<h1>thin-engine</h1>\n<p class=\"tally\"><strong>{total}</strong> tests &middot; <span class=\"ok\">{passed} as expected</span> &middot; <span class=\"red\">{red} red</span> &middot; <span class=\"seen\" data-tally=\"seen\">{reviewed} reviewed</span> &middot; <span class=\"unseen\" data-tally=\"unseen\">{unreviewed} to read</span></p>\n<p class=\"note\">Generated by <code>cargo run --example report</code>.{browse} Each test is shown whole, in the friendly form. A line the run wanted and did not get is marked <span class=\"key missing\">so</span>; one it got and did not want is marked <span class=\"key extra\">so</span>.</p>\n<p class=\"note\">Between a test's two worlds: a row that did not change is <span class=\"key same\">dimmed</span>, a row that did has the cells that differ marked <span class=\"cell\">so</span>, and a row in one world and not the other says which. <b>Two rows that are the same thing are paired only where there is exactly one of them on each side</b> - so a spent extractor and a fresh one over one deposit are reported as gone and new rather than as one of them changing, because which became which has no answer.</p>\n{lost}{cards}{script}</body>\n</html>\n"
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
    std::fs::write(mine().join("report.html"), &built.page).expect("report.html");
    std::fs::write(mine().join("report.txt"), &built.log).expect("report.txt");
    let (total, passed, red) = (built.total, built.passed, built.red);
    let (seen, unseen) = (built.reviewed, built.unreviewed);
    println!(
        "report.html, report.txt: {total} tests, {passed} as expected, {red} red, {seen} reviewed, {unseen} to read"
    );
}

/// **Both themes, because a report nobody can read in their own is not one** - the same rule
/// `R-10` states for a generated drawing. Every mark is an alpha over whatever the page sits on,
/// so it lands on white and on black.
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
