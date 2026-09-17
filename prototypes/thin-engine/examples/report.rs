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

        let outcome = match run_test(&script, &data) {
            Err(why) => Outcome::Refused(format!("{why}")),
            Ok(report) if report.same() => Outcome::Passed,
            Ok(report) => Outcome::Differed {
                missing: report.missing.iter().map(|it| friendly(it)).collect(),
                extra: report.extra.iter().map(|it| friendly(it)).collect(),
            },
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
            body.push_str(&format!(
                "<span class=\"{kind}\">{}</span>\n",
                escaped(line)
            ));
        }
        if let Outcome::Differed { extra, .. } = &outcome {
            for one in extra {
                body.push_str(&format!("<span class=\"extra\">{}</span>\n", escaped(one)));
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
    println!("report.html: {total} tests, {passed} as expected, {red} red");
}

/// **Both themes, because a report nobody can read in their own is not one** - the same rule
/// `R-10` states for a generated drawing. Every mark is an alpha over whatever the page sits on,
/// so it lands on white and on black.
const STYLE: &str = r#"
:root { color-scheme: light dark }
body {
  font: 14px/1.35 ui-monospace, SFMono-Regular, Menlo, monospace;
  margin: 1.5rem auto; max-width: 62rem; padding: 0 1rem;
}
h1 { font-size: 1.2rem; margin: 0 0 .2rem }
p { margin: .3rem 0 }
.tally { font-size: .95rem }
.note { opacity: .7; font-size: .8rem; margin-bottom: 1rem }
.ok { color: rgb(30 130 60) }
.red { color: rgb(190 50 50) }
@media (prefers-color-scheme: dark) {
  .ok { color: rgb(110 200 140) }
  .red { color: rgb(255 130 130) }
}
.test {
  border: 1px solid rgba(127,127,127,.35);
  border-left: 4px solid rgba(127,127,127,.5);
  border-radius: .3rem; padding: .5rem .7rem; margin: .5rem 0;
}
.test.red { border-left-color: rgb(190 50 50) }
.test.ok { border-left-color: rgb(30 130 60) }
summary { cursor: pointer; font-weight: 600 }
summary::marker { opacity: .5 }
details[open] > summary { margin-bottom: .45rem }
.name { font-weight: 600 }
.badge {
  font-size: .72rem; font-weight: 600; letter-spacing: .02em;
  padding: .05rem .4rem; border-radius: .2rem; border: 1px solid currentColor;
}
.why { font-size: .85rem; margin: 0 0 .45rem }
pre {
  margin: 0; overflow-x: auto; background: rgba(127,127,127,.08);
  padding: .45rem .6rem; border-radius: .25rem; line-height: 1.3;
}
pre span { display: block; padding: 0 .3rem; border-left: 3px solid transparent }
.said { opacity: .55 }
.gap { height: .45em }
.mark { font-weight: 700 }
.missing { background: rgba(200,40,40,.16); border-left-color: rgb(190 50 50) }
.missing::after { content: " <- wanted, not got"; opacity: .7; font-size: .8em }
.extra { background: rgba(210,130,0,.18); border-left-color: rgb(200 120 0) }
.extra::after { content: " <- got, not wanted"; opacity: .7; font-size: .8em }
.key { display: inline; padding: 0 .3rem; border-left: 3px solid }
.key::after { content: "" }
"#;
