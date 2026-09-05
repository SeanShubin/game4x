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
    /// How many leading columns name a row rather than describe it.
    ///
    /// **Without this a comparison pairs the wrong rows.** `expected::compare` matched on
    /// the table and its first column, which is right for `territory` and wrong for
    /// `extractor`: every extractor in territory 1 shared one identity, so building two of
    /// them read as *one row changed* instead of *two rows appeared*. Turn 2's delta said
    /// `extractor territory:1 · node: 3 → 2` - true of nothing that happened.
    ///
    /// Found by the check that reconciles a printed delta against the printed states, on its
    /// first run, which is the check I had declined to build.
    pub key: usize,
}

impl Table {
    fn new(name: &'static str, columns: &'static [&'static str]) -> Self {
        Table {
            name,
            columns,
            rows: Vec::new(),
            key: key_of(name),
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

fn readiness(exhausted: bool) -> String {
    if exhausted { "exhausted" } else { "ready" }.to_string()
}

/// How many leading columns name a row of this table.
///
/// **One declaration, read by the writer and by the reader.** A row written to a file does
/// not carry its key - that would be noise in the one artifact a person reads - so reading
/// one back has to ask the same question the writer asked. Asking a different place would be
/// two declarations, and a row that round-tripped into a different identity.
pub fn key_of(table: &str) -> usize {
    match table {
        "store" | "extractor" | "structure" | "territory-resource" => 2,
        _ => 1,
    }
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

    // **`P-254`: a thing's own identifier is `id`.** This said `territory`, and so did
    // every row referring to one from somewhere else - so `territory:1` meant *in territory
    // 1* in nine tables and *is territory 1* in this one, with only position telling them
    // apart. Two relations under one name.
    //
    // **`founded` is gone and nothing replaces it** - `P-255`. It was never a trait: the
    // release has declared `control` since it had traits at all and has never had a
    // `founded`, which entered the report in `8f69847` because a dump takes its columns
    // from the model while the release declares its traits somewhere else, and nothing has
    // ever compared the two. Sean chose to drop it rather than print `control` in its
    // place, on his own test - *if we actually need it I will notice when reviewing*.
    // `Territory::founded()` stays; `report.rs` branches on it.
    let mut territory = Table::new(
        "territory",
        &["id", "biome", "nature", "citizens", "labor-spent", "yards"],
    );
    // **Three facts, not one.** This was a single `density` column holding count times
    // density, so territories that are 3 x 4, 2 x 6 and 6 x 2 all read 12 - one number
    // standing for two, labelled with the name of the one it was not. `S-20`.
    //
    // `P-206` is what makes the split honest rather than invented: three extractor kinds
    // means the capacity is per kind, so *how many the ground has room for*, *what each
    // yields* and *how many are built* are each a fact the release already states.
    let mut node = Table::new(
        "territory-resource",
        &["territory", "resource", "capacity", "density", "built"],
    );
    let mut store = Table::new("store", &["territory", "resource", "amount"]);
    let mut garrison = Table::new("garrison", &["territory", "force"]);
    let mut extractor = Table::new("extractor", &["territory", "node", "resource", "readiness"]);
    let mut structure = Table::new("structure", &["territory", "structure", "count"]);
    // **`labor` is one of the fourteen kinds and had no table at all.** It existed only as a
    // `labor spent` column inside `territory`, so a reader looking for the kind found
    // nothing - and an absent table is the one thing that cannot be told from a wrong one.
    // `S-25`.
    //
    // It is derived rather than stored: a citizen provides one each turn and the model keeps
    // what was spent, so *made* and *left* are read out rather than held. That is a fact
    // about labor worth being able to see, and `S-21` is where whether it should be stored
    // at all gets decided.
    let mut labor = Table::new("labor", &["territory", "made", "spent", "left"]);

    for place in &game.territories {
        territory.push(vec![
            place.id.0.to_string(),
            format!("{:?}", place.biome).to_lowercase(),
            place.force_of_nature.to_string(),
            place.citizens().to_string(),
            place.labor_spent().to_string(),
            place.yards().to_string(),
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
                .extractors()
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

        if let Some(held) = &place.garrison() {
            garrison.push(vec![place.id.0.to_string(), held.force.to_string()]);
        }

        for built in &place.extractors() {
            extractor.push(vec![
                place.id.0.to_string(),
                built.node.to_string(),
                place.nodes[built.node].resource.name().to_string(),
                readiness(built.exhausted),
            ]);
        }

        labor.push(vec![
            place.id.0.to_string(),
            place.citizens().to_string(),
            place.labor_spent().to_string(),
            place.labor_available().to_string(),
        ]);

        for kind in StructureKind::ALL {
            let count = match kind {
                StructureKind::Extractor => place.extractors().len() as u32,
                StructureKind::Garrison => place.garrison().iter().count() as u32,
                StructureKind::Yard => place.yards(),
            };
            structure.push(vec![
                place.id.0.to_string(),
                kind.name().to_string(),
                count.to_string(),
            ]);
        }
    }

    let mut unit = Table::new("unit", &["id", "kind", "place", "fuel", "readiness"]);
    for flying in &game.units {
        unit.push(vec![
            flying.id.0.to_string(),
            flying.kind.name().to_string(),
            match flying.location {
                Location::Orbit => "orbit".to_string(),
                Location::On(at) => format!("territory-{}", at.0),
            },
            flying.cells.to_string(),
            readiness(flying.exhausted),
        ]);
    }

    // **Every kind the model knows, and how many there are.** This replaces a `unit kind`
    // table that named two of them.
    //
    // `S-25` found `labor` in no table at all, and the check written for it then found
    // `citizen` in the same state - both existed only as a column of `territory`, which is
    // not the same as the kind being represented. A table each would have answered those two
    // and left the next one to be found the same way. **One table naming every kind answers
    // the question the reader is actually asking**, which is Sean's: he reads these to find
    // the names that are wrong, and a name he cannot find is one he cannot judge.
    //
    // Counts are across the whole game, because *is there one anywhere* is what a missing
    // table leaves unanswerable. Where they sit is what the other tables are for.
    let mut kinds = Table::new("kind", &["id", "in-play"]);
    let total = |count: &dyn Fn(&game_model::Territory) -> u32| -> u32 {
        game.territories.iter().map(count).sum()
    };
    kinds.push(vec!["citizen".into(), total(&|t| t.citizens()).to_string()]);
    // **Labor things, not ready citizens.** This counted `labor_available` - how much
    // labor *could* be made - which was the only reading available while labor was a
    // counter. `P-231` made it a kind, so this counts the kind like every other row, and
    // reads zero at a turn boundary because `create labor` and `work` both happen inside a
    // turn. The `labor` table beside it is where the flow shows.
    kinds.push(vec![
        "labor".into(),
        total(&|t| t.count_of(game_model::thing::Kind::Labor)).to_string(),
    ]);
    for resource in Resource::ALL {
        kinds.push(vec![
            resource.name().to_string(),
            total(&|t| t.store(resource)).to_string(),
        ]);
    }
    for kind in StructureKind::ALL {
        kinds.push(vec![
            kind.name().to_string(),
            total(&|t| match kind {
                StructureKind::Extractor => t.extractors().len() as u32,
                StructureKind::Garrison => t.garrison().iter().count() as u32,
                StructureKind::Yard => t.yards(),
            })
            .to_string(),
        ]);
    }
    for kind in UnitKind::ALL {
        let count = game.units.iter().filter(|u| u.kind == kind).count();
        kinds.push(vec![kind.name().to_string(), count.to_string()]);
    }
    kinds.push(vec!["territory".into(), game.territories.len().to_string()]);

    vec![
        summary, territory, node, store, garrison, extractor, structure, labor, unit, kinds,
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
        "**Generated. Do not edit.** Every table and every column is named whether or not \
         anything\nis in it, because the names are what this is for.\n\n",
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
pub fn padded_rows(columns: &[String], rows: &[Vec<String>]) -> String {
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
/// lives in a data file, not in code and not in a presentation file.* Every table name,
/// column name and
/// value here comes from the state that was passed in. The only literals are structural -
/// tags, and a stylesheet that mentions no kind, no resource and no size.
pub fn html(sections: &[Section], title: &str) -> String {
    // **One opening for every page**, because there were two and they drifted. Extracting
    // the stylesheet out of this function took `</head><body>` with it, and both pages
    // shipped without either tag - well formed enough to render, and unparseable by the one
    // test that asks what is in the head. Sharing `head` means the tags cannot go missing
    // from one page and not the other, since there is no longer a second copy to forget.
    let mut out = head(title);
    out.push_str(&format!("<h1>{}</h1>\n", escaped(title)));
    out.push_str(
        "<p class=\"count\">Generated. Do not edit. Every table and every column is named whether or not \
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

/// One stylesheet for every page this crate writes.
///
/// It names no kind, no resource and no size - `spec/invariants.md` keeps the game's data
/// out of markup, and a rule reaching a colour by kind would be exactly that.
const STYLE: &str = "<style>\n\
     :root { color-scheme: light dark }\n\
     body { font: 15px/1.5 ui-monospace, SFMono-Regular, Menlo, monospace; margin: 2rem \
     auto; max-width: 70rem; padding: 0 1rem }\n\
     h1 { font-size: 1.3rem }\n\
     h2 { margin: 2rem 0 .25rem; font-size: 1.1rem }\n\
     h3 { margin: 1.25rem 0 .25rem; font-size: 1rem; opacity: .85 }\n\
     table { border-collapse: collapse; margin: .5rem 0 }\n\
     th, td { border: 1px solid currentColor; padding: .15rem .5rem; text-align: left }\n\
     th { font-weight: 600 }\n\
     pre { background: rgba(127,127,127,.12); padding: .6rem .8rem; overflow-x: auto }\n\
     ul { padding-left: 1.2rem }\n\
     .empty { opacity: .7; font-style: italic }\n\
     .count, .note { opacity: .7; font-size: .85rem }\n\
     .quiet { opacity: .55; font-size: .85rem }\n\
     .quiet a { font-weight: 400 }\n\
     </style>\n";

/// The page for a markdown report: `turns.md` becomes `turns.html`.
pub fn html_name(markdown: &str) -> &'static str {
    match markdown {
        "turns.md" => "turns.html",
        "commands.md" => "commands.html",
        "catalog.md" => "catalog.html",
        "recipes.md" => "recipes.html",
        // `state` and `entities` already have a page rendered from the model rather than
        // from their markdown, which is the better derivation and stays.
        other => panic!("no page name for {other}"),
    }
}

/// The reports another crate generates, which this one renders.
///
/// **`prototypes/kinds` writes these and cannot use this renderer** - nothing depends on
/// that crate and nothing should. So the page is made from the markdown on disk rather than
/// from the model that produced it, which is one derivation further away and the reason the
/// currency check has to cover both halves.
pub const RENDERED_ELSEWHERE: [&str; 2] = ["catalog.md", "recipes.md"];

/// The generated marker, on line two of every page.
///
/// **At a fixed line rather than wherever the prose puts it.** `tests/dumps_are_current.rs`
/// discovers its subjects by looking for this sentence near the top of a file, and reads
/// only the head so a file *discussing* the marker is not mistaken for one carrying it. That
/// window has been wrong twice. At twelve lines it found the three markdown dumps and
/// silently missed both pages; at twenty-four it missed the three pages `S-40` added, whose
/// marker falls at line 27 under a stylesheet. Both times the file was generated, marked,
/// and invisible - the check narrowing while looking unchanged.
///
/// A window measured against where the marker happens to land goes wrong whenever a page
/// grows. On line two it is a constant.
const MARKER: &str = "<!doctype html>\n<!-- Generated. Do not edit. -->\n";

/// The opening of every page, so one stylesheet serves all of them.
fn head(title: &str) -> String {
    let mut out = String::from(MARKER);
    out.push_str("<html lang=\"en\">\n<head>\n");
    out.push_str("<meta charset=\"utf-8\">\n");
    out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    out.push_str(&format!("<title>{}</title>\n", escaped(title)));
    out.push_str(STYLE);
    out.push_str("</head>\n<body>\n");
    out.push_str("<p class=\"note\"><a href=\"index.html\">all reports</a></p>\n");
    out
}

/// A markdown report as a page.
///
/// **`S-40`.** Five reports had markdown and only two had HTML, so three of the index's
/// links opened raw markdown in a browser - `turns.md` worst of all, being the longest and
/// the one read most while checking the state function.
///
/// **Both exist for reasons that do not overlap**, which `P-246` settled: markdown is the
/// surface a change is *reviewed* on, because a change is reviewed as a diff and HTML diffs
/// badly; HTML is the surface things are *browsed* on. Neither is canonical.
///
/// This handles what the reports actually contain - headings, paragraphs, tables, lists and
/// fenced blocks - rather than markdown at large. A general parser would be a large thing to
/// own for six generated files whose shapes this crate writes itself.
pub fn page(markdown: &str, title: &str) -> String {
    let mut out = head(title);
    let mut lines = markdown.lines().peekable();
    let mut fenced = false;
    let mut list = false;

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            out.push_str(if fenced {
                "</pre>
"
            } else {
                "<pre>
"
            });
            fenced = !fenced;
            continue;
        }
        if fenced {
            out.push_str(&format!(
                "{}
",
                escaped(line)
            ));
            continue;
        }

        if let Some(item) = trimmed.strip_prefix("- ") {
            if !list {
                out.push_str(
                    "<ul>
",
                );
                list = true;
            }
            out.push_str(&format!(
                "<li>{}</li>
",
                inline(item)
            ));
            continue;
        }
        if list {
            out.push_str(
                "</ul>
",
            );
            list = false;
        }

        if let Some(rest) = trimmed.strip_prefix("### ") {
            out.push_str(&format!(
                "<h3>{}</h3>
",
                inline(rest)
            ));
        } else if let Some(rest) = trimmed.strip_prefix("## ") {
            out.push_str(&format!(
                "<h2>{}</h2>
",
                inline(rest)
            ));
        } else if let Some(rest) = trimmed.strip_prefix("# ") {
            out.push_str(&format!(
                "<h1>{}</h1>
",
                inline(rest)
            ));
        } else if trimmed.starts_with('|') {
            // A table: this row, its separator, and every row after.
            let header = cells(trimmed);
            let separator = lines.peek().is_some_and(|next| next.contains("---"));
            if separator {
                lines.next();
            }
            out.push_str(
                "<table>
<thead>
<tr>",
            );
            for cell in &header {
                out.push_str(&format!("<th>{}</th>", inline(cell)));
            }
            out.push_str(
                "</tr>
</thead>
<tbody>
",
            );
            while lines
                .peek()
                .is_some_and(|next| next.trim().starts_with('|'))
            {
                let row = lines.next().unwrap_or_default();
                out.push_str("<tr>");
                for cell in cells(row.trim()) {
                    out.push_str(&format!("<td>{}</td>", inline(&cell)));
                }
                out.push_str(
                    "</tr>
",
                );
            }
            out.push_str(
                "</tbody>
</table>
",
            );
        } else if trimmed.is_empty() {
            // Blank lines separate blocks and carry nothing of their own.
        } else {
            // **A paragraph runs to the blank line, not to the newline.** Markdown wraps
            // prose at whatever column the file uses, and rendering each wrapped line as
            // its own `<p>` broke one paragraph of `catalog.md` into four - each ending
            // mid-sentence, and each looking deliberate. The joining is what makes the page
            // say the same thing as the markdown, which is the whole claim the pair rests
            // on.
            let mut paragraph = String::from(trimmed);
            while let Some(next) = lines.peek() {
                let next = next.trim();
                if next.is_empty()
                    || next.starts_with('|')
                    || next.starts_with("- ")
                    || next.starts_with('#')
                    || next.starts_with("```")
                {
                    break;
                }
                paragraph.push(' ');
                paragraph.push_str(next);
                lines.next();
            }
            out.push_str(&format!(
                "<p>{}</p>
",
                inline(&paragraph)
            ));
        }
    }
    if list {
        out.push_str(
            "</ul>
",
        );
    }
    out.push_str(
        "</body>
</html>
",
    );
    out
}

fn cells(row: &str) -> Vec<String> {
    row.trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

/// Bold, italic and code, which is all the reports use.
fn inline(text: &str) -> String {
    let mut out = escaped(text);
    for (mark, tag) in [("**", "strong"), ("`", "code"), ("*", "em")] {
        let mut open = true;
        while let Some(at) = out.find(mark) {
            let with = if open {
                format!("<{tag}>")
            } else {
                format!("</{tag}>")
            };
            out.replace_range(at..at + mark.len(), &with);
            open = !open;
        }
        if !open {
            // An odd number of markers: the last one opened nothing, so close it.
            out.push_str(&format!("</{tag}>"));
        }
    }
    out
}

/// The page that links every report and both scenario files.
///
/// **The two scenario files are linked as raw files, not as renderings** - `S-38`. They are
/// canonical data: `scenario/commands/play.4x` is what ran and `scenario/expected/play.4x`
/// is what a person reviewed. A rendering of either would be one more thing that can drift
/// from the thing it renders, and the whole point of them is that there is nothing between
/// the reader and the file.
///
/// So the page marks them apart from the generated views rather than listing them together.
/// Everything under *Reports* is derived and regenerated; everything under *The scenario* is
/// a source.
pub fn index(generated: &[(&str, String)]) -> String {
    let described = |name: &str| -> &str {
        match name {
            "catalog.md" => "every kind, with everything the release says about it in one place",
            "recipes.md" => "every recipe, with its own lines gathered under it",
            "state.md" => "the state after the scenario, one table per relation",
            "entities.md" => "the same state as entities and their components",
            "turns.md" => "every turn: the commands that ran, what changed, and what was there",
            "commands.md" => {
                "every command that ran, flattened out of its files, with the recipe it fired"
            }
            other => panic!("no description for {other}"),
        }
    };

    let mut out = String::from(MARKER);
    out.push_str(
        "<html lang=\"en\">
<head>
",
    );
    out.push_str(
        "<meta charset=\"utf-8\">
",
    );
    out.push_str(
        "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
",
    );
    out.push_str(
        "<title>game4x reports</title>
",
    );
    out.push_str(
        "<style>
         :root { color-scheme: light dark }
         body { font: 15px/1.6 ui-monospace, SFMono-Regular, Menlo, monospace; margin: 2rem          auto; max-width: 48rem; padding: 0 1rem }
         h2 { margin: 2rem 0 .25rem; font-size: 1.05rem }
         ul { list-style: none; padding: 0 }
         li { margin: .5rem 0 }
         a { font-weight: 600 }
         .what { opacity: .75 }
         .note { opacity: .75; font-size: .9rem }
         .quiet { opacity: .55; font-size: .85rem }
         .quiet a { font-weight: 400 }
         </style>
</head>
<body>
",
    );
    out.push_str("<h1>game4x</h1>\n");
    out.push_str(
        "<p class=\"note\">Generated. Do not edit. Made from the list of reports it \
         links, so one added or removed appears here without a second file being \
         edited.</p>\n",
    );
    out.push_str(
        "<h2>The scenario</h2>
",
    );
    out.push_str(
        "<p class=\"note\">Source, not a rendering. These are the files themselves - a view          of either would be one more thing that can drift from it.</p>
<ul>
",
    );
    for (path, what) in [
        (
            "../scenario/commands/play.4x",
            "the commands that ran, in order",
        ),
        (
            "../scenario/expected/play.4x",
            "what the scenario should produce, reviewed by hand",
        ),
    ] {
        out.push_str(&format!(
            "<li><a href=\"{path}\">{path}</a> <span class=\"what\">- {what}</span></li>
"
        ));
    }
    out.push_str(
        "</ul>
",
    );

    out.push_str("<h2>Reports</h2>\n");
    out.push_str(
        "<p class=\"note\">Generated. Every one is derived from the scenario or from the \
         release, and regenerated rather than written. <strong>The page is the link</strong>; \
         the markdown that made it is beside the name, because a change is reviewed as a diff \
         and a diff of HTML is not one.</p>\n<ul>\n",
    );

    // **A page is the default and its markdown is available beside it** - Sean's call, in
    // his words: *make it visually obvious that the html links are the default but the
    // markdown links are available*. Available, not hidden.
    //
    // Paired by stem rather than by a second list, so a report added to `generated` appears
    // here with both of its links and nothing else is edited. The pairing is asserted: a
    // markdown report with no page is a panic rather than a bare name on the page.
    let mut names: Vec<&str> = generated.iter().map(|(name, _)| *name).collect();
    names.push("catalog.md");
    names.push("catalog.html");
    names.push("recipes.md");
    names.push("recipes.html");
    names.sort_unstable();
    let mut listed = 0;
    for markdown in names.iter().filter(|name| name.ends_with(".md")) {
        let page = format!("{}.html", markdown.trim_end_matches(".md"));
        assert!(
            names.contains(&page.as_str()),
            "{markdown} has no page; every report gets both"
        );
        let name = markdown.trim_end_matches(".md");
        out.push_str(&format!(
            "<li><a href=\"{page}\">{name}</a> <span class=\"what\">- {}</span> \
             <span class=\"quiet\">(<a href=\"{markdown}\">markdown</a>)</span></li>\n",
            described(markdown)
        ));
        listed += 1;
    }
    assert_eq!(listed, 6, "six reports, each with a page and its markdown");
    out.push_str(
        "</ul>
</body>
</html>
",
    );
    out
}

/// One turn: what ran, what it changed, and what was there afterwards.
pub struct Turn {
    pub commands: Vec<String>,
    pub changed: crate::expected::Disagreement,
    pub state: String,
}

/// The five generated dump files, as names and contents.
///
/// **One producer for the writer and the check.** `bin/dump-state` writes these and
/// `tests/dumps_are_current.rs` compares them with what is committed - so the function that
/// knows how to run the scenario and render it lives here rather than in either. A test that
/// reproduced the scenario itself would be checking its own copy of the steps, which is the
/// arrangement `S-29` exists to end.
///
/// `catalog.md` is not among them: `prototypes/kinds` generates it and already holds it to
/// being current. Nor is `pending.md`, and that one is deliberate - `hooks/pre-commit`
/// **refuses** to rewrite it while any outbox has unstaged changes, so that it never renders
/// somebody's half-written finding. A test requiring it to be current would fail on a
/// correct refusal, and the fix for that would be making the hook unconditional, which is
/// the wrong direction.
pub fn generated(commands: &dyn crate::Library) -> Vec<(&'static str, String)> {
    let mut session = crate::Session::new();
    for line in ["run setup", "start"] {
        session
            .run(line, commands)
            .unwrap_or_else(|why| panic!("`{line}` failed: {why}"));
    }

    let scenario = commands
        .fetch("play")
        .unwrap_or_else(|| panic!("scenario/commands/play.4x is not there"));
    let boundaries = scenario
        .lines()
        .filter(|line| line.trim() == "end turn")
        .count();

    // **A turn is what ran, what changed, and what is there** - `S-38`. This used to be the
    // last of those alone: eight full states, eighteen hundred lines, and no command in any
    // of them. It showed the endpoints of a transformation and never the transformation, so
    // a reader validating the state function had to find the differences himself.
    let mut turns: Vec<Turn> = Vec::new();
    let mut ran: Vec<String> = Vec::new();
    let mut before = crate::expected::rows(&session.game);
    for line in scenario.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        ran.push(line.to_string());
        session
            .run(line, commands)
            .unwrap_or_else(|why| panic!("`{line}` failed: {why}"));
        if line == "end turn" {
            let after = crate::expected::rows(&session.game);
            turns.push(Turn {
                commands: std::mem::take(&mut ran),
                changed: crate::expected::compare(&before, &after),
                state: markdown(&session.game, ""),
            });
            before = after;
        }
    }

    // **The count, because a loop that stopped early produces a file that looks finished.**
    assert_eq!(
        turns.len(),
        boundaries,
        "scenario/commands/play.4x has {boundaries} `end turn` lines and {} states were taken",
        turns.len()
    );

    let state = "State after `scenario/commands/play.4x`";
    let things = "Entities after `scenario/commands/play.4x`";

    let mut per_turn = String::from("# Every turn of `scenario/commands/play.4x`\n\n");
    per_turn.push_str(&format!(
        "**Generated. Do not edit.** One section per `end turn` in the scenario - {} of \
         them.\nThe turn numbers are the scenario's own boundaries, so they line up with \
         its comments.\n\n",
        turns.len()
    ));
    for (at, turn) in turns.iter().enumerate() {
        per_turn.push_str(&format!(
            "# Turn {}

",
            at + 1
        ));

        per_turn.push_str(
            "## commands

```
",
        );
        for line in &turn.commands {
            per_turn.push_str(&format!(
                "{line}
"
            ));
        }
        per_turn.push_str(
            "```

",
        );

        per_turn.push_str(
            "## what changed

",
        );
        per_turn.push_str(&turn.changed.as_a_turn());

        per_turn.push_str("## what is there now\n\n");
        // **Every table demoted, not just the first.** A turn's three parts are `##`, so a
        // state's tables belong at `###` beneath them - demoting only the first left
        // `territory` sitting as a sibling of `commands`. It read wrong and parsed worse:
        // the check that counts rows per table saw one table of 156 rows.
        match turn.state.split_once("\n\n## ") {
            Some((_, rest)) => {
                per_turn.push_str(&format!("### {}", rest.replace("\n## ", "\n### ")))
            }
            None => per_turn.push_str(&turn.state),
        }
    }

    let mut written = vec![
        ("state.md", markdown(&session.game, state)),
        (
            "state.html",
            html(&normalized_sections(&session.game), state),
        ),
        ("entities.md", entities_markdown(&session.game, things)),
        (
            "entities.html",
            html(&entity_sections(&session.game), things),
        ),
        ("turns.md", per_turn),
        // `S-24`. Produced from its own replay rather than from the one above, because the
        // one above starts after `run setup` and this artifact is the whole of what ran -
        // the design included, which is where half the numbers in `state.md` come from.
        (
            "commands.md",
            crate::fired::markdown(&crate::fired::ran(commands)),
        ),
    ];
    // **Every markdown report gets a page** - `S-40`. Three of the index's links opened raw
    // markdown in a browser, `turns.md` worst of all, being the longest and the one read
    // most while checking the state function. Rendered from the markdown rather than from
    // the model, so the two cannot say different things.
    let pages: Vec<(&str, String)> = written
        .iter()
        .filter(|(name, _)| *name == "turns.md" || *name == "commands.md")
        .map(|(name, text)| (html_name(name), page(text, name)))
        .collect();
    written.extend(pages);

    // The page that links them, made from the list it links - so a report added here appears
    // on it, and one removed leaves it, without anybody editing a second file.
    let page = index(&written);
    written.push(("index.html", page));
    written
}
