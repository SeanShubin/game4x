//! An index over everything that says what the game does, and the pages under it.
//!
//! `cargo run --example index`, or `scripts/reports.sh`.
//!
//! **Sean, 2026-09-28**: *I want to re-create a structure that allows me to navigate all
//! information about my tests and supporting data indexed from an html file.* The old index went
//! with the old ruleset's reports under `D-4` - **the data was right to delete and the structure
//! was not**, and he said so: *the index.html would just be pointing at different things.*
//!
//! # `R-9`'s three clauses, which never mentioned the data
//!
//! **Every reference is a link**, **every generated view has a diffable sibling**, and **no page
//! needs JavaScript to be read**. What `D-4` deleted was the evidence line naming twelve
//! territory pages; the clauses are about shape and survive the ruleset they were written beside.
//!
//! **So each page here is written twice** - `.html` to read and `.md` to diff - from one pass, so
//! the two cannot disagree about what they say.
//!
//! # Uninteresting things get their own page rather than a shorter row
//!
//! **Sean, the same day**: *they don't need to be compact, just not in the way. Uninteresting
//! data relegated to its own page costs me nothing unless and until I find it interesting, in
//! which case I will want it to have just as much detail as any other page.* So `types` and
//! `primitives` are pages of their own with the same detail as the rest, reached from one line on
//! the index.
//!
//! # What it links is what is in the repository
//!
//! **A link that resolves in a clone**, which the old index's did not: it pointed at `.txt` twins
//! the pipeline writes at deploy and commits nowhere, so every one of them 404'd locally. `R-11`
//! records that confusion. The deploy still makes twins for the published copy; these links are
//! to the files as they sit.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "render.rs"]
#[allow(dead_code)]
mod render;

/// The repository root.
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// One thing a page lists: where it is, what it is called, and what it says of itself.
struct Entry {
    /// Path from the repository root, which is also the link.
    at: String,
    /// The first comment line of the file, which every generated and hand-written `.4x` carries.
    said: String,
    /// Anything else worth seeing on the row - a sibling, a record, a count.
    beside: Vec<(String, String)>,
}

/// The first comment line of a `.4x`, which is where each of them says what it is.
fn title_of(at: &PathBuf) -> String {
    let text = std::fs::read_to_string(at).unwrap_or_default();
    text.lines()
        .find(|line| line.trim_start().starts_with('#'))
        .map(|line| line.trim_start_matches(['#', ' ']).trim().to_string())
        .filter(|it| !it.is_empty())
        .unwrap_or_else(|| "-".to_string())
}

fn named_files(under: &str) -> Vec<String> {
    let at = root().join(under);
    let Ok(entries) = std::fs::read_dir(&at) else {
        return Vec::new();
    };
    let mut out: Vec<String> = entries
        .filter_map(|it| it.ok())
        .map(|it| it.path())
        .filter(|it| it.extension().and_then(|e| e.to_str()) == Some("4x"))
        .filter_map(|it| it.file_name().and_then(|n| n.to_str()).map(str::to_string))
        .collect();
    out.sort();
    out
}

fn escaped(raw: &str) -> String {
    raw.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// A `.4x` line, marked up so the notation reads as what it is.
///
/// **Sean, 2026-09-28**: *I like the html can be used to make text clean, readable, and sometimes
/// syntax highlighting.* **The marking is emitted here and coloured by the stylesheet**, so the
/// page needs no JavaScript and reads plainly with the stylesheet missing - which is `R-9`'s
/// third clause rather than a preference.
///
/// **Four things a line can be.** A comment, which carries the prose; a section marker, which is
/// a row with no values; a row, which is a relation and its named values; and a blank.
fn lit(line: &str) -> String {
    let trimmed = line.trim_end();
    if trimmed.trim().is_empty() {
        return String::new();
    }
    if let Some(said) = trimmed.trim_start().strip_prefix('#') {
        return format!("<span class=\"prose\">{}</span>", prose(said.trim()));
    }
    let Some(open) = trimmed.find('{') else {
        return escaped(trimmed);
    };
    let Some(close) = trimmed.rfind('}') else {
        return escaped(trimmed);
    };
    let body = &trimmed[open + 1..close];
    let after = &trimmed[close + 1..];
    let mut parts = body.split_whitespace();
    let Some(relation) = parts.next() else {
        return escaped(trimmed);
    };
    let mut out = format!(
        "<span class=\"brace\">{{</span><span class=\"relation\">{}</span>",
        escaped(relation)
    );
    for part in parts {
        match part.split_once(':') {
            Some((key, value)) => out.push_str(&format!(
                " <span class=\"key\">{}</span><span class=\"colon\">:</span><span class=\"value\">{}</span>",
                escaped(key),
                escaped(value)
            )),
            None => out.push_str(&format!(" <span class=\"value\">{}</span>", escaped(part))),
        }
    }
    out.push_str("<span class=\"brace\">}</span>");
    if !after.trim().is_empty() {
        // **The arrow and its number**, which is how a quantity is written beside a row.
        out.push_str(&format!("<span class=\"arrow\">{}</span>", escaped(after)));
    }
    out
}

/// The little markup a comment uses, so prose reads as prose.
///
/// **Bold, emphasis and code, and nothing else.** The comments in these files are written for a
/// person and use exactly those three; anything cleverer would be a markdown parser, which is a
/// thing to depend on rather than a thing to write here.
fn prose(said: &str) -> String {
    let mut out = String::new();
    let mut rest = escaped(said);
    // **Bold before emphasis**, or `**x**` is read as an empty emphasis wrapping one.
    for (open, close, tag) in [("**", "**", "strong"), ("`", "`", "code"), ("*", "*", "em")] {
        let mut made = String::new();
        loop {
            let Some(at) = rest.find(open) else {
                made.push_str(&rest);
                break;
            };
            let after = &rest[at + open.len()..];
            let Some(end) = after.find(close) else {
                made.push_str(&rest);
                break;
            };
            made.push_str(&rest[..at]);
            made.push_str(&format!("<{tag}>{}</{tag}>", &after[..end]));
            rest = after[end + close.len()..].to_string();
        }
        rest = made;
    }
    out.push_str(&rest);
    out
}

/// Write one `.4x` as a page, and give back the link to it from `reports/`.
///
/// **HTML is the browsable form and the text is a copy beside it** - Sean, 2026-09-28: *lets
/// prefer html as the browsable standard... We can still have links to copies rendered as text.*
/// So every reference on a list page goes to the rendering, and the rendering says where the file
/// itself is.
///
/// **Laid out under `reports/` as the repository lays it out**, so a reader who knows where a
/// file lives knows where its page is. A path already under `reports/` loses that prefix rather
/// than gaining a second one.
fn rendered(rel: &str) -> String {
    let under = rel.strip_prefix("reports/").unwrap_or(rel);
    let at = root().join("reports").join(format!("{under}.html"));
    if let Some(dir) = at.parent() {
        std::fs::create_dir_all(dir).unwrap_or_else(|why| panic!("{}: {why}", dir.display()));
    }
    let depth = under.matches('/').count();
    let up = "../".repeat(depth);
    let out_of_reports = "../".repeat(depth + 1);
    let text = std::fs::read_to_string(root().join(rel)).unwrap_or_default();
    let title = title_of(&root().join(rel));

    let mut page = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <title>{}</title>\n<link rel=\"stylesheet\" href=\"{up}report.css\">\n</head>\n<body>\n\
         <h1><code>{}</code></h1>\n<p class=\"note\">{}</p>\n\
         <p class=\"where\"><a href=\"{up}index.html\">The index</a> \u{b7} \
         <a href=\"{out_of_reports}{}\">the file itself, as text</a></p>\n<pre class=\"four-x\">",
        escaped(rel),
        escaped(rel),
        escaped(&title),
        escaped(rel)
    );
    for line in text.lines() {
        page.push_str(&lit(line));
        page.push('\n');
    }
    page.push_str("</pre>\n</body>\n</html>\n");

    let before = std::fs::read_to_string(&at).unwrap_or_default();
    if before != page {
        std::fs::write(&at, &page).unwrap_or_else(|why| panic!("{}: {why}", at.display()));
        RENDERED.fetch_add(1, Ordering::Relaxed);
    }
    format!("{under}.html")
}

/// How many renderings this run rewrote, so the tally says what moved.
static RENDERED: AtomicUsize = AtomicUsize::new(0);

/// A page, written twice from one list so the two forms cannot disagree.
struct Page {
    slug: String,
    title: String,
    note: String,
    sections: Vec<(String, Vec<Entry>)>,
}

impl Page {
    fn markdown(&self) -> String {
        let mut out = format!("# {}\n\n{}\n", self.title, self.note);
        out.push_str("\n[The index](index.md)\n");
        for (heading, entries) in &self.sections {
            out.push_str(&format!("\n## {heading}  ({})\n\n", entries.len()));
            for one in entries {
                let beside: Vec<String> = one
                    .beside
                    .iter()
                    .map(|(what, at)| format!("[{what}]({at})"))
                    .collect();
                let beside = if beside.is_empty() {
                    String::new()
                } else {
                    format!(" - {}", beside.join(" \u{b7} "))
                };
                out.push_str(&format!(
                    "- [{}]({}){beside}\n  {}\n",
                    one.at, one.at, one.said
                ));
            }
        }
        out
    }

    fn html(&self) -> String {
        let mut out = format!(
            "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
             <title>{}</title>\n<link rel=\"stylesheet\" href=\"report.css\">\n</head>\n<body>\n\
             <h1>{}</h1>\n<p class=\"note\">{}</p>\n\
             <p class=\"where\"><a href=\"index.html\">The index</a> \u{b7} \
             <a href=\"{}.md\">this page as markdown</a></p>\n",
            escaped(&self.title),
            escaped(&self.title),
            escaped(&self.note),
            self.slug
        );
        for (heading, entries) in &self.sections {
            out.push_str(&format!(
                "<h2>{} <span class=\"count\">{}</span></h2>\n<ul>\n",
                escaped(heading),
                entries.len()
            ));
            for one in entries {
                let beside: Vec<String> = one
                    .beside
                    .iter()
                    .map(|(what, at)| format!("<a href=\"{}\">{}</a>", escaped(at), escaped(what)))
                    .collect();
                let beside = if beside.is_empty() {
                    String::new()
                } else {
                    format!(" <span class=\"beside\">{}</span>", beside.join(" \u{b7} "))
                };
                out.push_str(&format!(
                    "<li><a href=\"{}\"><code>{}</code></a>{beside}<br><span class=\"said\">{}</span></li>\n",
                    escaped(&one.at),
                    escaped(&one.at),
                    escaped(&one.said)
                ));
            }
            out.push_str("</ul>\n");
        }
        out.push_str("</body>\n</html>\n");
        out
    }
}

/// Every rule, with how many reviewed tests reach it - by command, and through a `{part}`.
///
/// # One number said `perish` was fired by nothing, and it is fired by two
///
/// **Counting the tests whose `when` names the rule is a true count of a narrower population.**
/// `the-hungry-perish-after-upkeep-and-not-before` fires `{end-turn}`, and `end-turn` runs
/// `perish` as one of its ten parts - so the rule is exercised by a test that never names it.
///
/// **Sean asked whether this page is where an unused rule shows**, naming `perish` as his
/// example. It was, and it was wrong: the page said nothing fires it. **Both numbers are
/// reported now**, because the difference is real - a rule nothing commands is not the same as a
/// rule nothing reaches, and only the second is unused.
///
/// **Expanded to a fixed point**, since a part may name a rule that has parts of its own.
fn rules_with_cover() -> Vec<(String, usize, usize)> {
    let rules_text = std::fs::read_to_string(root().join("spec/data/rules.4x")).unwrap_or_default();
    let named = |line: &str, key: &str| -> Option<String> {
        line.split_whitespace()
            .find_map(|it| it.strip_prefix(key))
            .map(|it| it.trim_end_matches('}').to_string())
    };

    let mut rules = Vec::new();
    let mut parts: Vec<(String, String)> = Vec::new();
    for line in rules_text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("{rule ")
            && let Some(name) = named(trimmed, "name:")
        {
            rules.push(name);
        }
        if trimmed.starts_with("{part ")
            && let (Some(of), Some(is)) = (named(trimmed, "of:"), named(trimmed, "is:"))
        {
            parts.push((of, is));
        }
    }

    let reaches = |from: &BTreeMap<String, ()>| -> BTreeMap<String, ()> {
        let mut all = from.clone();
        loop {
            let before = all.len();
            for (of, is) in &parts {
                if all.contains_key(of) {
                    all.insert(is.clone(), ());
                }
            }
            if all.len() == before {
                return all;
            }
        }
    };

    // **What each test commands**, read from the rows under its `{when}`.
    let mut commanded: Vec<BTreeMap<String, ()>> = Vec::new();
    for test in named_files("reviewed") {
        let text = std::fs::read_to_string(root().join("reviewed").join(&test)).unwrap_or_default();
        let mut inside = false;
        let mut mine = BTreeMap::new();
        for line in text.lines() {
            let trimmed = line.trim();
            match trimmed {
                "{when}" => inside = true,
                "{then}" | "{refused}" | "{given}" => inside = false,
                _ => {
                    if inside && let Some(rest) = trimmed.strip_prefix('{') {
                        let relation = rest
                            .split([' ', '}'])
                            .next()
                            .unwrap_or_default()
                            .to_string();
                        mine.insert(relation, ());
                    }
                }
            }
        }
        commanded.push(mine);
    }

    rules
        .into_iter()
        .map(|name| {
            let direct = commanded.iter().filter(|it| it.contains_key(&name)).count();
            let through = commanded
                .iter()
                .filter(|it| !it.contains_key(&name) && reaches(it).contains_key(&name))
                .count();
            (name, direct, through)
        })
        .collect()
}

fn up(from: &str, to: &str) -> String {
    // **Relative from `reports/`, because every page sits there and every target does not.**
    // `R-9` asks that a reference be a link followed, and a link out of the directory has to say
    // how far out.
    let _ = from;
    format!("../{to}")
}

/// Write every page and every rendering, and say how many of each moved.
///
/// **A function rather than only a `main`, so the check can call it** - the way
/// `tests/directories.rs` borrows `examples/render.rs`. **The check used to shell out to
/// `cargo run`**, which is a second cargo contending for the same target directory lock: it
/// passed alone and failed under `--workspace`, which is the worst way for a check to be wrong.
pub fn write_all() -> (usize, usize) {
    let mut pages: Vec<Page> = Vec::new();

    // -- The tests Sean has read ------------------------------------------------------------
    let mut tests = Vec::new();
    for name in named_files("spec/tests") {
        let stem = name.trim_end_matches(".4x");
        let record = root().join("reviewed").join(&name);
        let mut beside = vec![
            ("as text".to_string(), up("", &format!("spec/tests/{name}"))),
            (
                "foundation form".to_string(),
                rendered(&format!("reports/foundation/{name}")),
            ),
        ];
        if record.is_file() {
            // **The record is linked as text and not rendered**, because a rendering of it would
            // sit at `reports/reviewed/...` - and `hooks/pre-commit` puts `*/reviewed/*` in
            // Sean's column, **both spellings, so that the rule does not stop applying the day
            // the directory moves.** A generated page landing there would make every run of this
            // generator a commit spanning two columns.
            //
            // **The rule is right and the mirror was wrong.** What a rendering of a record would
            // add is *see what he approved*, and the source is rendered already; that they agree
            // is `no_test_differs_from_what_sean_read`'s job rather than a reader's.
            beside.push((
                "the record he read it, as text".to_string(),
                up("", &format!("reviewed/{name}")),
            ));
        } else {
            beside.push(("not read yet".to_string(), "tests.html".to_string()));
        }
        tests.push(Entry {
            at: rendered(&format!("spec/tests/{name}")),
            said: format!(
                "{} - {}",
                stem,
                title_of(&root().join("spec/tests").join(&name))
            ),
            beside,
        });
    }
    pages.push(Page {
        slug: "tests".to_string(),
        title: "The tests, and what he has read".to_string(),
        note: "One file per test, in the form it is written in. The foundation form beside each \
               is what the engine runs, generated from the record rather than from the source - \
               `spec/README.md` rule 3."
            .to_string(),
        sections: vec![("Tests".to_string(), tests)],
    });

    // -- The ruleset ------------------------------------------------------------------------
    let mut ruleset = Vec::new();
    for (name, direct, through) in rules_with_cover() {
        ruleset.push(Entry {
            at: rendered(&format!("regression/rules/{name}.4x")),
            said: match (direct, through) {
                (0, 0) => format!("the rule `{name}` - NO reviewed test reaches it"),
                (0, n) => {
                    format!("the rule `{name}` - no test commands it; {n} reach it through a part")
                }
                (n, 0) => format!("the rule `{name}` - {n} reviewed test(s) command it"),
                (n, m) => format!(
                    "the rule `{name}` - {n} test(s) command it, {m} more reach it through a part"
                ),
            },
            beside: vec![
                (
                    "as text".to_string(),
                    up("", &format!("regression/rules/{name}.4x")),
                ),
                ("in the ruleset".to_string(), rendered("spec/data/rules.4x")),
            ],
        });
    }
    let mut data = Vec::new();
    for (at, what) in [
        ("spec/data/schema.4x", "every kind of thing the game has"),
        ("spec/data/rules.4x", "every rule, as data"),
        (
            "crates/game-model/data/friendly/engine.4x",
            "every word the engine implements",
        ),
        (
            "crates/game-model/data/friendly/script.4x",
            "what a test script may say",
        ),
        (
            "crates/game-model/data/friendly/setup.4x",
            "what every test starts from",
        ),
    ] {
        data.push(Entry {
            at: rendered(at),
            said: what.to_string(),
            beside: vec![("as text".to_string(), up("", at))],
        });
    }
    pages.push(Page {
        slug: "ruleset".to_string(),
        title: "The ruleset, and the data under it".to_string(),
        note:
            "Each rule links to its generated case, which holds the lines of `spec/data/rules.4x` \
               that define it. The count beside it is how many tests he has read fire that rule."
                .to_string(),
        sections: vec![
            ("Rules".to_string(), ruleset),
            ("The data files".to_string(), data),
        ],
    });

    // -- The scenario -----------------------------------------------------------------------
    let mut turns: BTreeMap<String, Vec<Entry>> = BTreeMap::new();
    let at = root().join("regression/scenario");
    if let Ok(dirs) = std::fs::read_dir(&at) {
        for turn in dirs.filter_map(|it| it.ok()).map(|it| it.path()) {
            if !turn.is_dir() {
                continue;
            }
            let which = turn
                .file_name()
                .and_then(|it| it.to_str())
                .unwrap_or_default()
                .to_string();
            for case in named_files(&format!("regression/scenario/{which}")) {
                turns
                    .entry(format!("Turn {which}"))
                    .or_default()
                    .push(Entry {
                        at: rendered(&format!("regression/scenario/{which}/{case}")),
                        said: title_of(&turn.join(&case)),
                        beside: vec![(
                            "as text".to_string(),
                            up("", &format!("regression/scenario/{which}/{case}")),
                        )],
                    });
            }
        }
    }
    let mut sections = vec![(
        "The scenario itself".to_string(),
        vec![
            Entry {
                at: rendered("scenario/main.4x"),
                said: "the world it starts in and every command it plays".to_string(),
                beside: vec![("as text".to_string(), up("", "scenario/main.4x"))],
            },
            Entry {
                at: up("", "scenario/played.md"),
                said: "the whole world at the end of every turn, and what fired".to_string(),
                beside: Vec::new(),
            },
        ],
    )];
    sections.extend(turns);
    pages.push(Page {
        slug: "scenario".to_string(),
        title: "The main scenario, command by command".to_string(),
        note: "One case per command, generated. `{given}` is what the command took and `{then}` \
               is what it made. Deleting one accepts what it does now."
            .to_string(),
        sections,
    });

    // -- The suites that are not about a scenario --------------------------------------------
    for (slug, under, title, note) in [
        (
            "types",
            "regression/types",
            "The definitions of the things",
            "One case per relation, holding the lines of `spec/data/schema.4x` that define it.",
        ),
        (
            "primitives",
            "regression/primitives",
            "The words the engine implements",
            "One case per word a rule may delegate to. A word that is not here cannot be written \
             in data without writing Rust.",
        ),
    ] {
        let entries = named_files(under)
            .into_iter()
            .map(|name| Entry {
                at: rendered(&format!("{under}/{name}")),
                said: title_of(&root().join(under).join(&name)),
                beside: vec![("as text".to_string(), up("", &format!("{under}/{name}")))],
            })
            .collect();
        pages.push(Page {
            slug: slug.to_string(),
            title: title.to_string(),
            note: note.to_string(),
            sections: vec![(title.to_string(), entries)],
        });
    }

    // -- The index ---------------------------------------------------------------------------
    let mut hub = Vec::new();
    for (slug, what) in [
        (
            "tests",
            "every test, its foundation form, and whether he has read it",
        ),
        (
            "ruleset",
            "every rule, how many read tests fire it, and the data under them",
        ),
        (
            "scenario",
            "the main scenario, one case per command, by turn",
        ),
        ("types", "one case per relation the game declares"),
        ("primitives", "one case per word the engine implements"),
    ] {
        hub.push(Entry {
            at: format!("{slug}.html"),
            said: what.to_string(),
            beside: vec![("as markdown".to_string(), format!("{slug}.md"))],
        });
    }
    // **What no reviewed behaviour depends on**, written by the mutation sweeps when they run.
    for (slug, what) in [
        (
            "unused-rows",
            "rows that can be deleted and nothing a read test asserts would notice",
        ),
        (
            "unused-values",
            "values that can be changed and nothing a read test asserts would notice",
        ),
    ] {
        if root().join("reports").join(format!("{slug}.md")).is_file() {
            hub.push(Entry {
                at: format!("{slug}.md"),
                said: format!("{what} - written by `cargo test -p game-model -- --ignored`"),
                beside: Vec::new(),
            });
        }
    }
    pages.push(Page {
        slug: "index".to_string(),
        title: "Everything that says what the game does".to_string(),
        note: "Generated. Every reference is a link, every page has a markdown sibling to diff, \
               and no page needs JavaScript to be read - `R-9`."
            .to_string(),
        sections: vec![("Pages".to_string(), hub)],
    });

    let out = root().join("reports");
    std::fs::create_dir_all(&out).expect("reports/");
    let mut written = 0;
    for page in &pages {
        for (name, text) in [
            (format!("{}.html", page.slug), page.html()),
            (format!("{}.md", page.slug), page.markdown()),
        ] {
            let at = out.join(&name);
            let before = std::fs::read_to_string(&at).unwrap_or_default();
            if before != text {
                std::fs::write(&at, &text).unwrap_or_else(|why| panic!("{name}: {why}"));
                written += 1;
            }
        }
    }
    let css = out.join("report.css");
    let style = include_str!("report.css");
    if std::fs::read_to_string(&css).unwrap_or_default() != style {
        std::fs::write(&css, style).expect("report.css");
        written += 1;
    }
    (written, RENDERED.load(Ordering::Relaxed))
}

fn main() {
    let (written, rendered) = write_all();
    println!("reports/: {rendered} rendering(s), {written} page file(s) rewritten");
}
