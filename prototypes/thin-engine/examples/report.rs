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
//! **One file, no stylesheet beside it and no script in it.** It opens from disk.
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
fn every_test() -> Vec<String> {
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

/// What became of one test, in the words the page uses.
enum Outcome {
    Passed,
    Refused(String),
    Differed {
        missing: Vec<String>,
        extra: Vec<String>,
    },
}

fn main() {
    let data = Directory(mine().join("data").join("foundation"));
    let setup = rows("data/foundation/setup.4x");

    // **The shared ruleset**, which every test is read against: the schema, the rules and the
    // categories. A test's own rows are added per test, because its territories are its own.
    let mut shared = Vec::new();
    for file in ["schema.4x", "engine.4x", "rules.4x", "things.4x"] {
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
        let source = text(&format!("data/friendly/tests/{file}"));
        let wanted: BTreeMap<String, ()> = match &outcome {
            Outcome::Differed { missing, .. } => {
                missing.iter().map(|it| (it.clone(), ())).collect()
            }
            _ => BTreeMap::new(),
        };

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
            body.push_str(&format!("<span class=\"{kind}\">{}</span>", escaped(line)));
        }
        if let Outcome::Differed { extra, .. } = &outcome {
            for one in extra {
                body.push_str(&format!("<span class=\"extra\">{}</span>", escaped(one)));
            }
        }

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
        cards.push_str(&format!(
            "<details class=\"test {class}\"{open}>\n<summary><span class=\"name\">{stem}</span> <span class=\"badge {class}\">{badge}</span></summary>\n{said}<pre>{body}</pre>\n</details>\n"
        ));
    }

    let total = passed + red;
    let page = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>thin-engine tests</title>\n<style>{STYLE}</style>\n</head>\n<body>\n<h1>thin-engine</h1>\n<p class=\"tally\"><strong>{total}</strong> tests &middot; <span class=\"ok\">{passed} as expected</span> &middot; <span class=\"red\">{red} red</span></p>\n<p class=\"note\">Generated by <code>cargo run --example report</code>. Each test is shown whole, in the friendly form. A line the run wanted and did not get is marked <span class=\"key missing\">so</span>; one it got and did not want is marked <span class=\"key extra\">so</span>.</p>\n{cards}</body>\n</html>\n"
    );
    std::fs::write(mine().join("report.html"), &page).expect("report.html");
    std::fs::write(mine().join("report.txt"), &log).expect("report.txt");
    println!("report.html, report.txt: {total} tests, {passed} as expected, {red} red");
}

/// **Both themes, because a report nobody can read in their own is not one** - the same rule
/// `R-10` states for a generated drawing. Every mark is an alpha over whatever the page sits on,
/// so it lands on white and on black.
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
pre {
  margin: 0; overflow-x: auto; background: rgba(127,127,127,.08);
  padding: .7rem .9rem; border-radius: .25rem;
}
pre span { display: block; padding: 0 .3rem; border-left: 3px solid transparent }
.said { opacity: .55 }
.gap { height: .8em }
.mark { font-weight: 700 }
.missing { background: rgba(200,40,40,.16); border-left-color: rgb(190 50 50) }
.missing::after { content: " <- wanted, not got"; opacity: .7; font-size: .8em }
.extra { background: rgba(210,130,0,.18); border-left-color: rgb(200 120 0) }
.extra::after { content: " <- got, not wanted"; opacity: .7; font-size: .8em }
.key { display: inline; padding: 0 .3rem; border-left: 3px solid }
.key::after { content: "" }
"#;
