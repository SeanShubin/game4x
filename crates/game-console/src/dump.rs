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

use game_model::{Game, Location, Phase, Resource, StructureKind, UnitKind};

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
    let mut node = Table::new("node", &["territory", "resource", "density"]);
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
            let density: u32 = place
                .nodes
                .iter()
                .filter(|n| n.resource == resource)
                .map(|n| n.density)
                .sum();
            node.push(vec![
                place.id.0.to_string(),
                resource.name().to_string(),
                density.to_string(),
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
    let mut out = format!("# {title}\n\n");
    out.push_str(
        "**Generated. Do not edit.** Every table and every column is named whether or not \
         anything\nis in it, because the names are what this is for.\n\n",
    );
    for table in tables(game) {
        out.push_str(&format!("## {}\n\n", table.name));
        out.push_str(&format!("| {} |\n", table.columns.join(" | ")));
        out.push_str(&format!(
            "|{}\n",
            table
                .columns
                .iter()
                .map(|_| " --- |")
                .collect::<Vec<_>>()
                .join("")
        ));
        if table.rows.is_empty() {
            out.push_str("\n*(empty) 0 rows*\n\n");
            continue;
        }
        for row in &table.rows {
            out.push_str(&format!("| {} |\n", row.join(" | ")));
        }
        out.push_str(&format!("\n{} row(s)\n\n", table.rows.len()));
    }
    out
}
