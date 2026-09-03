//! The state of a game, written out as tables a person can read the names of.
//!
//! **Sean's purpose, in his words:** *something like that is the only way I am going to be
//! able to actually identify the problems with names.* So this exists to be read rather than
//! to be complete, and the names are the product. Values are here to give the names
//! something to sit beside.
//!
//! Two things follow, and both are constraints rather than features.
//!
//! **Every table and every column is named whether or not anything is in it.** A dump that
//! omits what is empty hides exactly the names being reviewed - and the tables most likely
//! to be empty at the end of a run are the ones holding a thing that exists only in the
//! middle of one. A Pioneer is produced and then founds; an Ark is produced and then
//! deploys. `sql.html` prints *(empty) 0 rows* and so does this.
//!
//! **It takes a state rather than finding one.** Every change goes through the console and
//! history is complete, so any moment of a game is the state after replaying the first *n*
//! commands. Written this way, browsing turn *n* is a loop over states; written around the
//! final state, it is a rewrite.

use game_model::{Game, Location, Node, Phase, Resource, StructureKind, UnitKind};

/// One table: its name, its column names, and its rows.
///
/// The columns are carried separately from the rows so that a table with no rows still has
/// them. That is the whole reason this is a struct rather than a `Vec<Vec<String>>`.
pub struct Table {
    pub name: &'static str,
    pub columns: &'static [&'static str],
    pub rows: Vec<Vec<String>>,
}

impl Table {
    fn new(name: &'static str, columns: &'static [&'static str]) -> Self {
        Table {
            name,
            columns,
            rows: Vec::new(),
        }
    }

    fn push(&mut self, row: Vec<String>) {
        debug_assert_eq!(
            row.len(),
            self.columns.len(),
            "{} has {} columns and was given a row of {}",
            self.name,
            self.columns.len(),
            row.len()
        );
        self.rows.push(row);
    }
}

fn yes(value: bool) -> String {
    if value { "yes" } else { "no" }.to_string()
}

fn readiness(exhausted: bool) -> String {
    if exhausted { "exhausted" } else { "ready" }.to_string()
}

/// Every table, in a fixed order, for one moment of one game.
///
/// **The list is written here and not discovered from the state**, which is the difference
/// between a dump and a description of what happened to be present. `spec/invariants.md`
/// wants it in a data file rather than in code; `C-16` is that gap and is parked behind
/// `P-134`, so for now the enumerations come from the model's own `ALL` arrays wherever it
/// has one - which is what keeps a resource with nothing in it from vanishing.
pub fn tables(game: &Game) -> Vec<Table> {
    let mut summary = Table::new("game", &["phase", "turn", "territories", "units"]);
    summary.push(vec![
        match game.phase {
            Phase::Design => "design",
            Phase::Play => "play",
        }
        .to_string(),
        game.turn.to_string(),
        game.territories.len().to_string(),
        game.units.len().to_string(),
    ]);

    let mut territory = Table::new(
        "territory",
        &[
            "territory",
            "biome",
            "force of nature",
            "founded",
            "citizens",
            "labor spent",
            "yards",
        ],
    );
    // **Three facts, not one.** This was a single `density` column holding count times
    // density, so territories that are 3 x 4, 2 x 6 and 6 x 2 all read 12 - one number
    // standing for two, labelled with the name of the one it was not. `S-20`.
    //
    // `P-206` is what makes the split honest rather than invented: three extractor kinds
    // means the capacity is per kind, so *how many the ground has room for*, *what each
    // yields* and *how many are built* are each a fact the release already states.
    let mut node = Table::new(
        "territory resource",
        &["territory", "resource", "capacity", "density", "built"],
    );
    let mut store = Table::new("store", &["territory", "resource", "amount"]);
    let mut garrison = Table::new("garrison", &["territory", "force"]);
    let mut extractor = Table::new("extractor", &["territory", "node", "resource", "readiness"]);
    let mut structure = Table::new("structure", &["territory", "structure", "count"]);

    for place in &game.territories {
        territory.push(vec![
            place.id.0.to_string(),
            format!("{:?}", place.biome).to_lowercase(),
            place.force_of_nature.to_string(),
            yes(place.founded),
            place.citizens.to_string(),
            place.labor_spent.to_string(),
            place.yards.to_string(),
        ]);

        // Every resource, not every resource that has a node here. A territory with no
        // energy is a fact worth being able to see.
        for resource in Resource::ALL {
            let here: Vec<&Node> = place
                .nodes
                .iter()
                .filter(|n| n.resource == resource)
                .collect();
            // Every node of a resource is set to one density by `SetResource`, so this is
            // one number - but it is read out rather than assumed, and a ground that ever
            // held two would say both instead of quietly showing one.
            let mut densities: Vec<String> = here.iter().map(|n| n.density.to_string()).collect();
            densities.sort();
            densities.dedup();
            let built = place
                .extractors
                .iter()
                .filter(|e| place.nodes[e.node].resource == resource)
                .count();
            node.push(vec![
                place.id.0.to_string(),
                resource.name().to_string(),
                here.len().to_string(),
                if densities.is_empty() {
                    "0".to_string()
                } else {
                    densities.join(", ")
                },
                built.to_string(),
            ]);
            store.push(vec![
                place.id.0.to_string(),
                resource.name().to_string(),
                place.store(resource).to_string(),
            ]);
        }

        if let Some(held) = &place.garrison {
            garrison.push(vec![place.id.0.to_string(), held.force.to_string()]);
        }

        for built in &place.extractors {
            extractor.push(vec![
                place.id.0.to_string(),
                built.node.to_string(),
                place.nodes[built.node].resource.name().to_string(),
                readiness(built.exhausted),
            ]);
        }

        for kind in StructureKind::ALL {
            let count = match kind {
                StructureKind::Extractor => place.extractors.len() as u32,
                StructureKind::Garrison => place.garrison.iter().count() as u32,
                StructureKind::Yard => place.yards,
            };
            structure.push(vec![
                place.id.0.to_string(),
                kind.name().to_string(),
                count.to_string(),
            ]);
        }
    }

    let mut unit = Table::new("unit", &["unit", "kind", "place", "fuel", "readiness"]);
    for flying in &game.units {
        unit.push(vec![
            flying.id.0.to_string(),
            flying.kind.name().to_string(),
            match flying.location {
                Location::Orbit => "orbit".to_string(),
                Location::On(at) => format!("territory {}", at.0),
            },
            flying.cells.to_string(),
            readiness(flying.exhausted),
        ]);
    }

    // Named from the model's enumeration rather than from the units present, so a kind that
    // is nowhere still has a row saying so. This is the table that answers *is there an Ark
    // anywhere*, which `unit` can only answer by absence.
    let mut unit_kind = Table::new("unit kind", &["kind", "in play"]);
    for kind in UnitKind::ALL {
        let count = game.units.iter().filter(|u| u.kind == kind).count();
        unit_kind.push(vec![kind.name().to_string(), count.to_string()]);
    }

    vec![
        summary, territory, node, store, garrison, extractor, structure, unit, unit_kind,
    ]
}

/// The tables as markdown, which is the artifact of record.
///
/// **Markdown rather than only HTML, deliberately.** A committed markdown dump diffs, so a
/// rule change shows its consequences in the commit that caused it. `P-186` raised a
/// Pioneer's cost and the play-through got *shorter*, and that fact existed only because
/// somebody regenerated and noticed. HTML is rendered from these same tables for reading.
pub fn markdown(game: &Game, title: &str) -> String {
    let mut out = format!(
        "# {title}

"
    );
    out.push_str(
        "**Generated. Do not edit.** Every table and every column is named whether or not          anything
is in it, because the names are what this is for.

",
    );
    for table in tables(game) {
        out.push_str(&format!(
            "## {}

",
            table.name
        ));
        out.push_str(&padded(&table));
        if table.rows.is_empty() {
            out.push_str(
                "
*(empty) 0 rows*

",
            );
        } else {
            out.push_str(&format!(
                "
{} row(s)

",
                table.rows.len()
            ));
        }
    }
    out
}

/// A table with its columns already at the width `tools/pad-tables` would give them.
///
/// **Otherwise the generator and the padder fight over the file.** The padder rewrites
/// column widths whenever anything in a markdown file changes, and it runs in the gate - so
/// a generated table emitted narrow comes back padded, and the next generation makes it
/// narrow again. Every gate run would show a diff nobody wrote. `catalog.md` never hit this
/// because it uses bullets; this is the first generated file with tables in it.
///
/// So the widths are computed here to the same rule: each column as wide as its widest cell,
/// header included, and the separator filled to match.
fn padded(table: &Table) -> String {
    let columns: Vec<String> = table.columns.iter().map(|c| c.to_string()).collect();
    padded_rows(&columns, &table.rows)
}

/// The same, for a table whose column names were read rather than written down.
fn padded_rows(columns: &[String], rows: &[Vec<String>]) -> String {
    let mut width: Vec<usize> = columns.iter().map(|c| c.chars().count()).collect();
    for row in rows {
        for (at, cell) in row.iter().enumerate() {
            width[at] = width[at].max(cell.chars().count());
        }
    }
    // A minimum of three, so a separator is at least `---`. `tools/pad-tables` applies the
    // same floor, and without it a two-character column like `id` renders one space narrower
    // here than the padder would write it - which is the whole class of difference this
    // padding exists to remove.
    for at in &mut width {
        *at = (*at).max(3);
    }

    let line = |cells: &[String]| {
        let mut out = String::from("|");
        for (at, cell) in cells.iter().enumerate() {
            let pad = width[at] - cell.chars().count();
            out.push_str(&format!(" {cell}{} |", " ".repeat(pad)));
        }
        out.push('\n');
        out
    };

    let mut out = line(columns);
    out.push_str(&line(
        &width.iter().map(|w| "-".repeat(*w)).collect::<Vec<_>>(),
    ));
    for row in rows {
        out.push_str(&line(row));
    }
    out
}

/// The entity view: every thing, its id, and its components, one table per kind.
///
/// **The same rows the live browser shows.** `report::entities` is what F3 renders, and
/// `spec/interface.md` requires both views there as well as in a file - so this reads that
/// function rather than walking the model a second time. Built separately they would drift,
/// and this lane has already watched a view go on saying an orbit was named by no recipe.
///
/// **Columns come from the entities that exist, and the kinds do not.** A kind with nothing
/// in it still gets a table, because *is there an Ark anywhere* is a question the reader
/// should not have to answer from an absence. What it cannot get is column names - those
/// live in each entity's components, so a kind with no instances has none to show, and this
/// says so rather than printing an empty header and implying it knew.
pub fn entity_tables(game: &Game) -> Vec<EntityTable> {
    let entries = crate::report::entities(game);
    let mut out = Vec::new();

    for kind in ["game", "territory", "unit"] {
        let mine: Vec<&crate::report::Entry> =
            entries.iter().filter(|entry| entry.kind == kind).collect();

        // The union, in first-seen order, so a component only some entities carry still
        // gets a column rather than being dropped for the ones that have it.
        let mut columns: Vec<String> = Vec::new();
        for entry in &mine {
            for (name, _) in &entry.components {
                if !columns.iter().any(|seen| seen == name) {
                    columns.push(name.clone());
                }
            }
        }

        let rows = mine
            .iter()
            .map(|entry| {
                let mut row = vec![entry.id.clone()];
                for column in &columns {
                    row.push(
                        entry
                            .components
                            .iter()
                            .find(|(name, _)| name == column)
                            .map(|(_, value)| value.clone())
                            .unwrap_or_default(),
                    );
                }
                row
            })
            .collect();

        let mut named = vec!["id".to_string()];
        named.extend(columns);
        out.push(EntityTable {
            kind: kind.to_string(),
            columns: named,
            rows,
        });
    }
    out
}

/// A table whose columns are discovered rather than declared.
///
/// Separate from [`Table`] because that one owns `&'static` column names written down in
/// code, and these are read out of the entities themselves. Collapsing them would mean
/// pretending one of those two facts was the other.
pub struct EntityTable {
    pub kind: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// The entity view as markdown.
pub fn entities_markdown(game: &Game, title: &str) -> String {
    let mut out = format!("# {title}\n\n");
    out.push_str(
        "**Generated. Do not edit.** Every kind gets a table whether or not anything is of \
         that kind.\nThese are the rows the F3 browser shows, from the same function.\n\n",
    );
    for table in entity_tables(game) {
        out.push_str(&format!("## {}\n\n", table.kind));
        if table.rows.is_empty() {
            out.push_str(
                "*(empty) 0 rows* - and no columns either, because a component name lives \
                 on an entity\nand there is none of this kind to read one from.\n\n",
            );
            continue;
        }
        out.push_str(&padded_rows(&table.columns, &table.rows));
        out.push_str(&format!("\n{} row(s)\n\n", table.rows.len()));
    }
    out
}

/// One escaping rule, because a value that has never met `<` is not evidence of anything.
fn escaped(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The same rows as HTML, which is what Sean reads.
///
/// **Generated and never canonical.** `spec/invariants.md`: *a derived form is generated
/// rather than written*, and `P-198` makes the format part of it - specification and
/// presentation do not share one. So the markdown beside this is the artifact of record,
/// because it diffs and a rule change shows its consequences in the commit that caused it;
/// this is the same data in the form that is comfortable to read.
///
/// **No game data in the markup.** `spec/invariants.md` again: *what the game is made of
/// lives in a data file, not in code and not in markup.* Every table name, column name and
/// value here comes from the state that was passed in. The only literals are structural -
/// tags, and a stylesheet that mentions no kind, no resource and no size.
pub fn html(sections: &[Section], title: &str) -> String {
    let mut out = String::from("<!doctype html>\n<html lang=\"en\">\n<head>\n");
    out.push_str("<meta charset=\"utf-8\">\n");
    out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    out.push_str(&format!("<title>{}</title>\n", escaped(title)));
    out.push_str(
        "<style>\n\
         :root { color-scheme: light dark }\n\
         body { font: 15px/1.5 ui-monospace, SFMono-Regular, Menlo, monospace; margin: 2rem auto; \
         max-width: 70rem; padding: 0 1rem }\n\
         h2 { margin: 2rem 0 .25rem; font-size: 1.1rem }\n\
         table { border-collapse: collapse; margin: .5rem 0 }\n\
         th, td { border: 1px solid currentColor; padding: .15rem .5rem; text-align: left }\n\
         th { font-weight: 600 }\n\
         .empty { opacity: .7; font-style: italic }\n\
         .count { opacity: .7; font-size: .85rem }\n\
         </style>\n</head>\n<body>\n",
    );
    out.push_str(&format!("<h1>{}</h1>\n", escaped(title)));
    out.push_str(
        "<p class=\"count\">Generated. Every table and every column is named whether or not \
         anything is in it.</p>\n",
    );

    for Section {
        name,
        columns,
        rows,
    } in sections
    {
        out.push_str(&format!("<h2>{}</h2>\n", escaped(name)));
        if columns.is_empty() {
            out.push_str("<p class=\"empty\">(empty) 0 rows, and no columns to name</p>\n");
            continue;
        }
        out.push_str("<table>\n<thead>\n<tr>");
        for column in columns {
            out.push_str(&format!("<th>{}</th>", escaped(column)));
        }
        out.push_str("</tr>\n</thead>\n<tbody>\n");
        if rows.is_empty() {
            out.push_str(&format!(
                "<tr><td class=\"empty\" colspan=\"{}\">(empty) 0 rows</td></tr>\n",
                columns.len()
            ));
        }
        for row in rows {
            out.push_str("<tr>");
            for cell in row {
                out.push_str(&format!("<td>{}</td>", escaped(cell)));
            }
            out.push_str("</tr>\n");
        }
        out.push_str("</tbody>\n</table>\n");
        out.push_str(&format!("<p class=\"count\">{} row(s)</p>\n", rows.len()));
    }
    out.push_str("</body>\n</html>\n");
    out
}

/// One rendered table, in the one shape both views reduce to.
///
/// **The two views have different ideas of a table and the renderer must not.** A [`Table`]'s
/// columns are written down in code; an [`EntityTable`]'s are read off the entities. Both
/// become this before anything renders them, which is what *one producer, two destinations*
/// means in practice: one place knows how to draw a table, and it knows nothing about where
/// the rows came from.
pub struct Section {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// The normalized view, in the shape [`html`] takes.
pub fn normalized_sections(game: &Game) -> Vec<Section> {
    tables(game)
        .into_iter()
        .map(|t| Section {
            name: t.name.to_string(),
            columns: t.columns.iter().map(|c| c.to_string()).collect(),
            rows: t.rows,
        })
        .collect()
}

/// The entity view, in the shape [`html`] takes.
pub fn entity_sections(game: &Game) -> Vec<Section> {
    entity_tables(game)
        .into_iter()
        .map(|t| Section {
            name: t.kind,
            columns: t.columns,
            rows: t.rows,
        })
        .collect()
}
