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

/// Whether the version of a test Sean read is the version that is there.
///
/// **Whole file, whitespace collapsed.** A comment reworded counts, because in the workflow this
/// is for it is this lane that rewords it - *I don't want to miss anything*. Only formatting is
/// ignored, which is what makes a difference a non-syntax one.
fn review_of(stem: &str) -> (&'static str, Vec<String>) {
    let now = std::fs::read_to_string(mine().join(format!("data/friendly/tests/{stem}.4x")))
        .unwrap_or_default();
    let Ok(read) = std::fs::read_to_string(mine().join(format!("reviewed/{stem}.4x"))) else {
        return ("never reviewed", Vec::new());
    };
    let bare = |text: &str| -> Vec<String> {
        text.lines()
            .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
            .filter(|l| !l.is_empty())
            .collect()
    };
    let (was, is) = (bare(&read), bare(&now));
    if was == is {
        return ("reviewed", Vec::new());
    }
    // **The lines are a hint and the status is the fact.** Equality above catches everything,
    // ordering included; this set difference is only to show a reader where to look, and a
    // reordering with no other change would leave it empty while the status still says drifted.
    let mut said: Vec<String> = is.iter().filter(|l| !was.contains(l)).cloned().collect();
    said.extend(
        was.iter()
            .filter(|l| !is.contains(l))
            .map(|l| format!("was: {l}")),
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
    let Ok(text) = std::fs::read_to_string(mine().join("reviewed/asked.md")) else {
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
                for line in &drift {
                    said.push_str(&format!("<span class=\"drift\">{}</span>", escaped(line)));
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
            "<details class=\"test {class}\"{open} data-test=\"{stem}\">\n<summary><span class=\"name\">{stem}</span> <span class=\"badge {class}\">{badge}</span> <span class=\"badge {seen_class}\" data-mark>{mark}</span>{chip}{acts}</summary>\n{said}{noted}<pre>{body}{seen}</pre>\n</details>\n"
        ));
    }

    let total = passed + red;
    let body_attribute = if live { " data-live" } else { "" };
    let script = if live {
        format!("<script>{SCRIPT}</script>\n")
    } else {
        String::new()
    };
    let page = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>thin-engine tests</title>\n<style>{STYLE}</style>\n</head>\n<body{body_attribute}>\n<h1>thin-engine</h1>\n<p class=\"tally\"><strong>{total}</strong> tests &middot; <span class=\"ok\">{passed} as expected</span> &middot; <span class=\"red\">{red} red</span> &middot; <span class=\"seen\" data-tally=\"seen\">{reviewed} reviewed</span> &middot; <span class=\"unseen\" data-tally=\"unseen\">{unreviewed} to read</span></p>\n<p class=\"note\">Generated by <code>cargo run --example report</code>. Each test is shown whole, in the friendly form. A line the run wanted and did not get is marked <span class=\"key missing\">so</span>; one it got and did not want is marked <span class=\"key extra\">so</span>.</p>\n{cards}{script}</body>\n</html>\n"
    );
    Built {
        page,
        log,
        total,
        passed,
        red,
    }
}

fn main() {
    let built = build(false);
    std::fs::write(mine().join("report.html"), &built.page).expect("report.html");
    std::fs::write(mine().join("report.txt"), &built.log).expect("report.txt");
    let (total, passed, red) = (built.total, built.passed, built.red);
    println!("report.html, report.txt: {total} tests, {passed} as expected, {red} red");
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
  bar.innerHTML = '<b>j</b>/<b>k</b> move &middot; <b>Enter</b> open &middot; <b>r</b> reviewed'
    + ' &middot; <b>x</b> needs changing &middot; <b>u</b> unreview';
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
    const keys = {
      j: () => { here = Math.min(here + 1, cards.length - 1); show(); },
      k: () => { here = Math.max(here - 1, 0); show(); },
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
.seen { color: rgb(70 110 160) }
.unseen { color: rgb(170 100 0) }
@media (prefers-color-scheme: dark) {
  .seen { color: rgb(140 180 230) }
  .unseen { color: rgb(230 180 90) }
}
.drift { background: rgba(120,120,200,.16); border-left-color: rgb(90 110 190) }
.drift::after { content: " <- not what you read"; opacity: .7; font-size: .8em }
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
