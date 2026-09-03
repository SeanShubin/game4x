//! Everything the release says about one kind, gathered under that kind.
//!
//! `spec/invariants.md`: *every other form of it is derived, and a derived form is
//! generated rather than written.* This is such a form.
//!
//! **It is a view rather than a reprint.** Copying the tables into a second file would be
//! the restatement the same sentence forbids. What the release does not have anywhere is
//! *one place per kind*: its description, the families that contain it, the traits that are
//! of it, what a territory can hold, what it costs, and every recipe line that requires,
//! limits, consumes or produces it. Each of those facts lives in a different table, and the
//! reader assembling them is doing a join by hand.
//!
//! So this answers a question no table answers, which is the test of whether a derived form
//! is worth generating: *what is a pioneer?* takes six tables and gets one section.

use crate::release::{body_under, plain};

/// The catalog, as markdown, from the release document.
pub fn catalog(document: &str) -> String {
    let mut out = String::new();
    out.push_str("# Catalog\n\n");
    out.push_str(
        "**Generated. Do not edit.** `cargo run -p kinds -- catalog`, or `scripts/kinds.sh catalog`.\n\n",
    );
    out.push_str(
        "Every kind the release declares, with everything it says about that kind gathered in \
         one place.\n`spec/invariants.md` has the release's tables be the data, and every other \
         form of them derived\nand generated rather than written; this is one such form. It is a \
         view and not a copy - each\nsection is a join across six tables that the document does \
         not perform anywhere.\n\n",
    );

    let kinds = body_under(document, "## Kinds");
    out.push_str(&format!(
        "{} kinds, {} families, {} traits, {} recipes.\n\n",
        kinds.len(),
        body_under(document, "## Families").len(),
        body_under(document, "## Traits").len(),
        recipe_names(document).len()
    ));

    for row in &kinds {
        let name = plain(&row[0]);
        out.push_str(&section(document, &name, row.get(1).map(String::as_str)));
    }
    out
}

/// Every recipe name, in the order the table gives them.
fn recipe_names(document: &str) -> Vec<String> {
    let mut names = Vec::new();
    for row in body_under(document, "## Recipes") {
        let name = plain(&row[0]);
        if !name.is_empty() {
            names.push(name);
        }
    }
    names
}

/// The recipe each row belongs to, since only a recipe's first row carries its name.
fn rows_with_recipe(document: &str) -> Vec<(String, Vec<String>)> {
    let mut current = String::new();
    let mut out = Vec::new();
    for row in body_under(document, "## Recipes") {
        let name = plain(&row[0]);
        if !name.is_empty() {
            current = name;
        }
        out.push((current.clone(), row));
    }
    out
}

fn section(document: &str, kind: &str, what_it_is: Option<&str>) -> String {
    let mut out = format!("## {kind}\n\n");
    if let Some(said) = what_it_is {
        out.push_str(&format!("{said}.\n\n"));
    }

    let families: Vec<String> = body_under(document, "## Families")
        .iter()
        .filter(|row| {
            let members = row.get(1).map(String::as_str).unwrap_or_default();
            members == "every kind above" || members.split(',').any(|m| m.trim() == kind)
        })
        .map(|row| plain(&row[0]))
        .collect();
    if !families.is_empty() {
        out.push_str(&format!("**In families** {}\n\n", families.join(", ")));
    }

    let traits: Vec<String> = body_under(document, "## Traits")
        .iter()
        .filter(|row| mentions(row.get(1).map(String::as_str).unwrap_or_default(), kind))
        .map(|row| {
            format!(
                "`{}` ({})",
                plain(&row[0]),
                row.get(2).cloned().unwrap_or_default()
            )
        })
        .collect();
    if !traits.is_empty() {
        out.push_str(&format!("**Traits of it** {}\n\n", traits.join(", ")));
    }

    for (heading, label) in [
        ("## What bounds a kind in a territory", "Bounded by"),
        ("## Units and structures", "As a thing"),
    ] {
        if let Some(row) = body_under(document, heading)
            .iter()
            .find(|row| plain(&row[0]) == kind)
        {
            let header = crate::release::table_under(document, heading);
            let names = header.first().cloned().unwrap_or_default();
            let said: Vec<String> = row
                .iter()
                .enumerate()
                .skip(1)
                .filter(|(_, cell)| !cell.is_empty())
                .map(|(at, cell)| {
                    let column = names.get(at).cloned().unwrap_or_default();
                    // A one-column table whose column is named the same as the section
                    // label would otherwise read *Bounded by: Bounded by: a capacity of 2*.
                    if column == label {
                        cell.clone()
                    } else {
                        format!("{column}: {cell}")
                    }
                })
                .collect();
            if !said.is_empty() {
                out.push_str(&format!("**{label}** {}\n\n", said.join(" · ")));
            }
        }
    }

    // Which families name this kind, so a recipe taking `place` is found under `orbit`.
    //
    // **`P-196` is why this exists.** Before it, no recipe named an orbit and the catalog
    // said so - which is what raised `C-15` and became that proposal. `move` now takes a
    // `place`, so an orbit is named by a recipe through its family, and a join matching the
    // Kind cell literally would have gone on reporting *none name it* after the thing it
    // reported had been fixed. A view that stays wrong once the world moves is worse than
    // no view.
    let mut families_of: Vec<String> = body_under(document, "## Families")
        .iter()
        .filter(|row| {
            let members = row.get(1).map(String::as_str).unwrap_or_default();
            members.split(',').any(|m| m.trim() == kind)
        })
        .map(|row| plain(&row[0]))
        .collect();
    families_of.push(kind.to_string());

    let mut lines: Vec<String> = Vec::new();
    for (recipe, row) in rows_with_recipe(document) {
        let named = row.get(4).map(|c| plain(c)).unwrap_or_default();
        let place = row.get(6).cloned().unwrap_or_default();
        // The Where column names a place too: `deploy ark` takes its Ark from *the orbit
        // above `$where`*, which is the only line in the table that reaches an orbit.
        let in_where = place
            .split(|c: char| !c.is_alphanumeric())
            .any(|w| w == kind);
        if !families_of.iter().any(|f| f == &named) && !in_where {
            continue;
        }
        let via = if named == kind {
            String::new()
        } else if in_where && !families_of.iter().any(|f| f == &named) {
            format!(" (as the place holding {named})")
        } else {
            format!(" (as a {named})")
        };
        let role = row.get(2).cloned().unwrap_or_default();
        let qty = row.get(3).cloned().unwrap_or_default();
        let traits = row.get(5).cloned().unwrap_or_default();
        let mut said = format!("`{recipe}` {role}s {qty}{via}");
        if !traits.is_empty() {
            said.push_str(&format!(", {traits}"));
        }
        if !place.is_empty() {
            said.push_str(&format!(", in {place}"));
        }
        lines.push(said);
    }
    if lines.is_empty() {
        // Worth saying rather than leaving blank: a kind no recipe names is a kind nothing
        // in the game can make, use or destroy, which is a fact about the release.
        out.push_str("**In recipes** none name it.\n\n");
    } else {
        out.push_str("**In recipes**\n\n");
        for line in lines {
            out.push_str(&format!("- {line}\n"));
        }
        out.push('\n');
    }
    out
}

/// Whether a Traits table's *Of* column covers this kind.
///
/// The column is prose - *every thing*, *a unit*, *citizen, garrison, ark, pioneer*, *a
/// territory, per resource* - so this matches on the word rather than parsing English, and
/// says so. A trait of *whatever readies* is not attributed to anything, which is honest:
/// the release does not say which kinds those are, and guessing would put a fact in this
/// document that the tables do not carry.
fn mentions(of: &str, kind: &str) -> bool {
    if of.trim() == "every thing" {
        return true;
    }
    of.split(|c: char| !c.is_alphanumeric())
        .any(|word| word == kind)
}
