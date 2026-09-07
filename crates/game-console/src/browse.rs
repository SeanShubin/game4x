//! Every reference is a link, and every identified thing has a page - `R-9`, shaped by `S-64`.
//!
//! **`spec/console.md`: a field naming a kind is a reference to one.** That sentence is what
//! makes this generated rather than guessed. A cell under a column called `kind` holds the
//! name of a kind; a cell under `territory` holds the id of a territory; and each of those is
//! a thing with a page of its own. Nothing here reads the *value* to decide - it reads the
//! column, which is the schema saying what the cell means.
//!
//! **So the rule is per column, and the columns are listed.** A column this does not know is
//! left as text rather than guessed at, and [`unlinked`] is what says which those are, so a
//! new column arrives as a decision rather than as a silently-plain cell.
//!
//! # A page per thing, not a fragment
//!
//! Sean's model - `vote/generated/code-structure/browse/` - gives every code unit its own
//! page rather than an anchor inside a long one, and the reason is that a page is something
//! he can point at: a URL that survives the report growing. The twelve territories are the
//! identified things this release has, so each gets one, gathering every row of every table
//! that names it. **That is a join no report performs**, which is the same test the catalog
//! passes: it answers *what is territory 5* in one place instead of in nine tables.

use crate::dump::{Section, escaped, head, normalized_sections};
use game_model::Game;

/// The page a kind's name links to: the catalog's section for it.
///
/// **An anchor rather than a page**, and it is the one exception to a page per thing. A kind
/// is not a thing in the game - it is what a thing *is* - and the catalog is already one
/// section per kind, generated from the release. A second page per kind would be that section
/// rendered twice from two crates, and `prototypes/kinds` may not use this renderer.
fn kind_at(name: &str) -> String {
    format!("catalog.html#{}", slug(name))
}

/// A territory's own page.
fn territory_at(id: &str) -> String {
    format!("territory-{id}.html")
}

/// A heading's anchor, from its text.
///
/// **The same slug on both sides or the link goes nowhere.** The catalog writes `## citizen`
/// and this writes `catalog.html#citizen`, and the only thing keeping those equal is that
/// both are this function. A test follows every link it generates to the anchor it names.
pub fn slug(text: &str) -> String {
    let mut out = String::new();
    let mut dashed = false;
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            dashed = false;
        } else if !dashed && !out.is_empty() {
            out.push('-');
            dashed = true;
        }
    }
    out.trim_end_matches('-').to_string()
}

/// Where a cell points, given the table and column it is in - or nowhere.
///
/// **Decided by the column, never by the value.** A cell holding `1` is a territory under
/// `territory` and a count under `yards`, and nothing about the string `1` says which. The
/// schema is the only thing that knows, which is why this takes all three.
pub fn reference(table: &str, column: &str, cell: &str) -> Option<String> {
    if cell.is_empty() || cell == "-" {
        return None;
    }
    match (table, column) {
        // A territory, by id, wherever one is named.
        (_, "territory") | (_, "from") | (_, "to") => Some(territory_at(cell)),
        ("territory", "id") => Some(territory_at(cell)),
        // A kind, by name. `resource` and `structure` hold kind names too - food and yard
        // are kinds in the release's Kinds table, which is what the catalog is generated
        // from.
        (_, "resource") | (_, "structure") | (_, "kind") | (_, "in-kind") => Some(kind_at(cell)),
        ("kind", "id") => Some(kind_at(cell)),
        // A unit stands in a place, and which place is what `in-kind` says. So `in-id` is a
        // territory's id when it is a territory's, and this cannot tell from here.
        (_, "in-id") => None,
        _ => None,
    }
}

/// The columns this deliberately leaves as plain text, and why.
///
/// **A list rather than a silence.** Every column not in [`reference`] falls through to plain
/// text, and a column added tomorrow would fall through the same way - reading as *not a
/// reference* when it means *nobody has looked*. `tests/browse.rs` holds every column of
/// every table against this list, so a new one fails until it is either linked or named here.
pub const UNLINKED: [(&str, &str); 23] = [
    ("game", "phase"),
    ("game", "territories"),
    ("game", "units"),
    ("territory", "biome"),
    ("territory", "nature"),
    ("territory", "citizens"),
    ("territory", "labor-spent"),
    ("territory", "yards"),
    ("deposit", "capacity"),
    ("deposit", "density"),
    ("deposit", "built"),
    ("store", "amount"),
    ("garrison", "force"),
    ("extractor", "ready"),
    ("structure", "count"),
    ("labor", "made"),
    ("labor", "spent"),
    ("labor", "left"),
    // **A unit is an identified thing and has no page**, which is a decision rather than an
    // oversight. `S-64` names the twelve territories as the things to give pages to, and
    // this scenario ends with no unit at all - `P-342` made launching put nothing into
    // orbit - so a page per unit would be a page per nothing. **Named here so that it is
    // findable** the day a unit survives to the end.
    ("unit", "id"),
    ("unit", "fuel"),
    ("unit", "ready"),
    // Where a unit stands. `in-kind` says what kind of place it is and links to that kind;
    // `in-id` is that place's id, and which page it names depends on the cell beside it.
    // **Left plain rather than guessed**, because a link that goes to the wrong thing is
    // worse than no link: it is confidently wrong, and nothing about following it says so.
    ("unit", "in-id"),
    ("kind", "in-play"),
];

/// A cell, as a link where it is a reference and as text where it is not.
pub fn cell(table: &str, column: &str, text: &str) -> String {
    match reference(table, column, text) {
        Some(at) => format!("<a href=\"{at}\">{}</a>", escaped(text)),
        None => escaped(text),
    }
}

/// Every territory that has a page, in the order the state lists them.
fn ids(game: &Game) -> Vec<String> {
    normalized_sections(game)
        .into_iter()
        .find(|section| section.name == "territory")
        .map(|section| {
            section
                .rows
                .iter()
                .filter_map(|row| row.first().cloned())
                .collect()
        })
        .unwrap_or_default()
}

/// Every row of every table that names this territory, with the table it came from.
///
/// **Including the `territory` table's own row**, found by `id` rather than by `territory`,
/// which is the one place the column naming a territory is not called that.
fn about(sections: &[Section], id: &str) -> Vec<Section> {
    let mut out = Vec::new();
    for section in sections {
        let naming: Vec<usize> = section
            .columns
            .iter()
            .enumerate()
            .filter(|(_, column)| {
                let column = column.as_str();
                column == "territory"
                    || column == "from"
                    || column == "to"
                    || (section.name == "territory" && column == "id")
            })
            .map(|(at, _)| at)
            .collect();
        if naming.is_empty() {
            continue;
        }
        let rows: Vec<Vec<String>> = section
            .rows
            .iter()
            .filter(|row| {
                naming
                    .iter()
                    .any(|at| row.get(*at).map(String::as_str) == Some(id))
            })
            .cloned()
            .collect();
        if rows.is_empty() {
            continue;
        }
        out.push(Section {
            name: section.name.clone(),
            columns: section.columns.clone(),
            rows,
        });
    }
    out
}

/// The territories reachable from this one in one move, from the adjacency table.
fn neighbours(sections: &[Section], id: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for section in sections.iter().filter(|it| it.name == "adjacency") {
        for row in &section.rows {
            let (from, to) = (row.first(), row.get(1));
            if from.map(String::as_str) == Some(id) {
                out.extend(to.cloned());
            } else if to.map(String::as_str) == Some(id) {
                out.extend(from.cloned());
            }
        }
    }
    out.sort_by_key(|it| it.parse::<u32>().unwrap_or(u32::MAX));
    out.dedup();
    out
}

/// One territory's page.
fn page(sections: &[Section], id: &str) -> String {
    let mine = about(sections, id);
    let mut out = head(&format!("territory {id}"));
    out.push_str(&format!("<h1>territory {id}</h1>\n"));
    out.push_str(&format!(
        "<p class=\"note\">Generated. Do not edit. Every row of every table in \
         <a href=\"state.html\">state</a> that names this territory, gathered - {} of them \
         across {} tables. Nothing here is a fact the state does not carry.</p>\n",
        mine.iter().map(|section| section.rows.len()).sum::<usize>(),
        mine.len()
    ));

    let beside = neighbours(sections, id);
    out.push_str("<h2>Reached from here</h2>\n");
    if beside.is_empty() {
        // A territory nothing borders cannot be reached by land, which is a fact about the
        // planet rather than a gap in this page.
        out.push_str("<p class=\"blank\">Nothing borders it.</p>\n");
    } else {
        out.push_str("<ul class=\"neighbours\">\n");
        for other in &beside {
            out.push_str(&format!(
                "<li><a href=\"{}\">territory {other}</a></li>\n",
                territory_at(other)
            ));
        }
        out.push_str("</ul>\n");
    }

    for section in &mine {
        out.push_str(&format!(
            "<h2 id=\"{}\">{}</h2>\n",
            slug(&section.name),
            escaped(&section.name)
        ));
        out.push_str("<table class=\"traits\">\n<thead>\n<tr>");
        for column in &section.columns {
            out.push_str(&format!("<th>{}</th>", escaped(column)));
        }
        out.push_str("</tr>\n</thead>\n<tbody>\n");
        for row in &section.rows {
            out.push_str("<tr>");
            for (at, text) in row.iter().enumerate() {
                let column = section.columns.get(at).cloned().unwrap_or_default();
                out.push_str(&format!("<td>{}</td>", cell(&section.name, &column, text)));
            }
            out.push_str("</tr>\n");
        }
        out.push_str("</tbody>\n</table>\n");
    }
    out.push_str("</body>\n</html>\n");
    out
}

/// One territory's diffable sibling.
///
/// **The links are not in it, and that is what a sibling is for.** It exists so that a change
/// to the page is one line of a diff; markup in it would put a URL on every line and make
/// every diff carry the rendering as well as the fact. The page is browsed, the sibling is
/// diffed, and neither does both.
fn sibling(sections: &[Section], id: &str) -> String {
    let mine = about(sections, id);
    let mut out = format!("# territory {id}\n\n");
    out.push_str(
        "**Generated. Do not edit.** The diffable sibling of the page; it carries the same \
         rows without\nthe links, so that a change to the state is a change to one line here.\n\n",
    );
    let beside = neighbours(sections, id);
    out.push_str(&format!(
        "**Reached from here** {}\n\n",
        if beside.is_empty() {
            String::from("nothing borders it")
        } else {
            beside
                .iter()
                .map(|other| format!("territory {other}"))
                .collect::<Vec<String>>()
                .join(", ")
        }
    ));
    for section in &mine {
        out.push_str(&format!("## {}\n\n", section.name));
        out.push_str(&crate::dump::padded_rows(&section.columns, &section.rows));
        out.push('\n');
    }
    out
}

/// Every territory page and every sibling, as generated files.
pub fn pages(game: &Game) -> Vec<(String, String)> {
    let sections = normalized_sections(game);
    let mut out = Vec::new();
    for id in ids(game) {
        out.push((format!("territory-{id}.html"), page(&sections, &id)));
        out.push((format!("territory-{id}.md"), sibling(&sections, &id)));
    }
    assert!(
        !out.is_empty(),
        "no territory has a page, so `R-9`'s page-per-thing is satisfied by there being no \
         things - which is the count-over-nothing this repository keeps recording"
    );
    out
}
