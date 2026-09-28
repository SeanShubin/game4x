//! The generated suites, one per type of thing that is data.
//!
//! **`docs/process.md`**: *every type of thing that is data has a generated suite of its own, and
//! there are four: the commands a scenario ran, the transformations over the things, the
//! definitions of the things, and the words the engine implements.*
//!
//! The first is `regression/scenario/` and is `examples/scenario.rs`'s. The other three are here.
//!
//! # A case is the source's own lines, not a rendering of them
//!
//! **Each case holds the lines of the friendly source that define one thing, verbatim.** Nothing
//! is parsed and written back out, so a diff in a case is a diff in `spec/data/` and cannot be an
//! artifact of how this file renders. Rows are parsed to decide *which* case a line belongs to
//! and for nothing else.
//!
//! **Which is what makes the suites answer the question they are for.** Change one clause of
//! `move` and one file moves; add a column to `place` and one file moves. A case that re-rendered
//! would move whenever the renderer changed, which is noise indistinguishable from a real change.
//!
//! # Ownership is a table and every line is owned exactly once
//!
//! **A line whose sort is in no table is a failure rather than a line nobody files.** That is the
//! same shape as `tests/common/mod.rs`'s `BEING_REPLACED`: read the population, except by name,
//! and assert the count - so a sort arriving in the data fails here instead of quietly belonging
//! to nothing.

use std::collections::BTreeMap;
use std::path::PathBuf;

use game_model::notation::{Row, read};

#[path = "render.rs"]
#[allow(dead_code)]
mod render;

/// One case: a file name under its suite, and what belongs in it.
pub struct Case {
    pub name: String,
    pub text: String,
}

/// Where the suites live - beside `reviewed/`, which is `D-6`.
pub fn suites_at() -> PathBuf {
    render::mine().join("..").join("..").join("regression")
}

/// Each `{...}` line of a friendly source, parsed for filing and kept for writing.
fn lines_of(file: &str) -> Vec<(Row, String)> {
    let at = render::friendly_at(file);
    let text = std::fs::read_to_string(render::mine().join(&at))
        .unwrap_or_else(|why| panic!("{at}: {why}"));
    let mut out = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('{') {
            continue;
        }
        let rows = read(trimmed).unwrap_or_else(|why| panic!("{at}: `{trimmed}`: {why}"));
        let [row] = &rows[..] else {
            panic!("{at}: `{trimmed}` is {} rows and should be one", rows.len());
        };
        out.push((row.clone(), trimmed.to_string()));
    }
    assert!(
        out.len() > 50,
        "{at} parsed to {} lines, so a suite built from it would prove almost nothing",
        out.len()
    );
    out
}

/// The header every generated case carries, which is where the gesture is written down.
fn header(what: &str, from: &str) -> String {
    format!(
        "# {what}\n\
         #\n\
         # **Generated. Do not edit.** `scripts/suites.sh`, from `{from}`.\n\
         # **Delete this file to accept what it says now** - `docs/process.md`: *absent expected\n\
         # data means I accept what it does now, so the test writes it, and what I review is the\n\
         # diff in version control.*\n\
         #\n\
         # **The lines below are that file's own**, so a diff here is a diff there.\n\n"
    )
}

/// File the lines of one source under the thing each defines, and assert none was left over.
///
/// `owner` returns the case a line belongs to, or `None` for a sort the table does not know -
/// which is reported with the line rather than dropped.
fn filed(
    source: &str,
    lines: &[(Row, String)],
    owner: impl Fn(&Row) -> Option<String>,
) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut orphans = Vec::new();
    for (row, line) in lines {
        match owner(row) {
            Some(to) => out.entry(to).or_default().push(line.clone()),
            None => orphans.push(line.clone()),
        }
    }
    assert!(
        orphans.is_empty(),
        "{} line(s) of {source} belong to no case, so the suite would be silently incomplete:\n  \
         {}\nAdd the sort to the ownership table, or say why it belongs nowhere.",
        orphans.len(),
        orphans.join("\n  ")
    );
    let filed: usize = out.values().map(Vec::len).sum();
    assert_eq!(
        filed,
        lines.len(),
        "{source} has {} lines and {filed} were filed",
        lines.len()
    );
    out
}

/// **The transformations over the things** - one case per rule.
pub fn rules_cases() -> Vec<Case> {
    let lines = lines_of("rules.4x");

    // A clause and a part each name their rule, and the rows under them name only the clause or
    // the part - so those two indexes are what turns a clause name into a rule name.
    let mut rule_of_clause: BTreeMap<String, String> = BTreeMap::new();
    let mut rule_of_part: BTreeMap<String, String> = BTreeMap::new();
    for (row, _) in &lines {
        match row.relation.as_str() {
            "clause" => {
                if let (Some(name), Some(rule)) = (row.value("name"), row.value("rule")) {
                    rule_of_clause.insert(name.to_string(), rule.to_string());
                }
            }
            "part" => {
                if let (Some(name), Some(of)) = (row.value("name"), row.value("of")) {
                    rule_of_part.insert(name.to_string(), of.to_string());
                }
            }
            _ => {}
        }
    }

    let owner = |row: &Row| -> Option<String> {
        let by = |key: &str| row.value(key).map(str::to_string);
        match row.relation.as_str() {
            "rule" => by("name"),
            // **`part.of` and not `part.is`.** A part names the rule it belongs to and the rule it
            // calls, and only the first owns it - otherwise `upkeep` would carry a line about
            // `end-turn`'s sequence, and a change to that sequence would move two cases.
            "part" => by("of"),
            "input" | "clause" | "repeats" | "scope" => by("rule"),
            "argument" => by("part").and_then(|it| rule_of_part.get(&it).cloned()),
            "binding" | "literal" | "reading" | "relation-of" | "soft" | "assigns" => {
                by("clause").and_then(|it| rule_of_clause.get(&it).cloned())
            }
            _ => None,
        }
    };

    filed("spec/data/rules.4x", &lines, owner)
        .into_iter()
        .map(|(rule, said)| Case {
            name: format!("{rule}.4x"),
            text: format!(
                "{}{}\n",
                header(&format!("The rule `{rule}`"), "spec/data/rules.4x"),
                said.join("\n")
            ),
        })
        .collect()
}

/// **The definitions of the things** - one case per relation.
pub fn types_cases() -> Vec<Case> {
    let lines = lines_of("schema.4x");

    // A reference names its column by id, because a column's name is the token it is keyed by and
    // is not unique - `tests/friendly.rs`. So the column has to be looked up to reach a relation.
    let mut relation_of_column: BTreeMap<String, String> = BTreeMap::new();
    for (row, _) in &lines {
        if row.relation == "column"
            && let (Some(id), Some(relation)) = (row.value("id"), row.value("relation"))
        {
            relation_of_column.insert(id.to_string(), relation.to_string());
        }
    }

    // **A vocabulary table's rows belong to the table.** `{trait id:1 name:moving}` is a row of
    // `trait` the way `{biome id:1 name:ocean}` is a row of `biome` - the sort is the relation.
    const VOCABULARY: [&str; 5] = ["trait", "role", "biome", "supply", "primitive"];

    let owner = |row: &Row| -> Option<String> {
        let by = |key: &str| row.value(key).map(str::to_string);
        let sort = row.relation.as_str();
        match sort {
            "relation" => by("name"),
            "column" | "state" | "family" | "attribute" => by("relation"),
            "reference" => by("column").and_then(|it| relation_of_column.get(&it).cloned()),
            // A family's case is the set of its members, and a kind's is what it carries and
            // where it may stand.
            "member" => by("family"),
            "carries" | "loose" | "stands-in" => by("kind"),
            _ if VOCABULARY.contains(&sort) => Some(sort.to_string()),
            _ => None,
        }
    };

    filed("spec/data/schema.4x", &lines, owner)
        .into_iter()
        .map(|(relation, said)| Case {
            name: format!("{relation}.4x"),
            text: format!(
                "{}{}\n",
                header(&format!("The thing `{relation}`"), "spec/data/schema.4x"),
                said.join("\n")
            ),
        })
        .collect()
}

/// **The words the engine implements** - one case per word.
///
/// **The fourth suite is below the other three** - `docs/process.md`: *a word a rule delegates
/// to, where what moved is the code rather than the game.* A word arriving or leaving here is a
/// change to what can be written in data at all.
pub fn primitives_cases() -> Vec<Case> {
    let lines = lines_of("engine.4x");
    let owner = |row: &Row| -> Option<String> {
        match row.relation.as_str() {
            "primitive" => row.value("word").map(str::to_string),
            _ => None,
        }
    };
    filed("data/friendly/engine.4x", &lines, owner)
        .into_iter()
        .map(|(word, said)| Case {
            name: format!("{word}.4x"),
            text: format!(
                "{}{}\n",
                header(
                    &format!("The word `{word}`, which the engine implements"),
                    "crates/game-model/data/friendly/engine.4x"
                ),
                said.join("\n")
            ),
        })
        .collect()
}

/// Write what is absent, compare what is there, and never overwrite one that is.
///
/// **The three rules are `docs/process.md`'s and they are the whole of the pattern.** Returns the
/// failure a caller should panic with, or `None`. It returns rather than panics so that a test can
/// assert the message as well as the outcome.
///
/// **The deletion is printed at every grain the suite has** - `D-6`: *when a failure names stale
/// cases it prints the deletion for each grain, so I paste it rather than compose it.* A case name
/// with a `/` in it has a middle grain; one without has two grains rather than three.
pub fn check(suite: &str, cases: &[Case]) -> Option<String> {
    let at = suites_at().join(suite);
    std::fs::create_dir_all(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));

    let mut stale: Vec<(String, String)> = Vec::new();
    let (mut written, mut compared) = (Vec::new(), 0usize);
    for Case { name, text } in cases {
        let path = at.join(name);
        match std::fs::read_to_string(&path) {
            Ok(committed) => {
                compared += 1;
                if committed != *text {
                    let differs = committed
                        .lines()
                        .zip(text.lines())
                        .enumerate()
                        .find(|(_, (was, now))| was != now);
                    stale.push((
                        name.clone(),
                        match differs {
                            Some((line, (was, now))) => format!(
                                "{name} line {}:\n      was  {was}\n      now  {now}",
                                line + 1
                            ),
                            None => format!(
                                "{name}: {} line(s) committed, {} produced",
                                committed.lines().count(),
                                text.lines().count()
                            ),
                        },
                    ));
                }
            }
            Err(_) => {
                if let Some(under) = path.parent() {
                    std::fs::create_dir_all(under)
                        .unwrap_or_else(|why| panic!("{}: {why}", under.display()));
                }
                std::fs::write(&path, text)
                    .unwrap_or_else(|why| panic!("{}: {why}", path.display()));
                written.push(name.clone());
            }
        }
    }
    if !written.is_empty() {
        println!(
            "{suite}: wrote {} case(s) that were absent; read the diff",
            written.len()
        );
    }
    assert_eq!(
        compared + written.len(),
        cases.len(),
        "{suite}: every case was either compared or written"
    );

    if stale.is_empty() {
        return None;
    }
    let mut out = String::new();
    for (_, said) in &stale {
        out.push_str(&format!("    {said}\n"));
    }
    out.push_str("\n  accept one case:\n");
    for (name, _) in &stale {
        out.push_str(&format!("    Remove-Item regression/{suite}/{name}\n"));
    }
    out.push_str(&format!(
        "\n  accept the whole suite:\n    Remove-Item -Recurse regression/{suite}\n"
    ));
    Some(format!(
        "{} of {} case(s) in `{suite}` no longer say what the data does.\n\
         **Delete what you are happy with, run again, and read the diff in version control.**\n\n\
         {out}",
        stale.len(),
        cases.len()
    ))
}

/// Every `.4x` on disk under one suite, so a case nothing produces can be reported.
pub fn on_disk(suite: &str) -> Vec<String> {
    let at = suites_at().join(suite);
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

fn main() {
    for (suite, cases) in [
        ("rules", rules_cases()),
        ("types", types_cases()),
        ("primitives", primitives_cases()),
    ] {
        match check(suite, &cases) {
            None => println!("{suite}: {} case(s), all current", cases.len()),
            Some(why) => println!("{why}"),
        }
    }
}
