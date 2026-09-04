//! Every recipe, gathered under its own name.
//!
//! **`S-23`, and Sean's split is what it is for**: context-free — what the game *is* — against
//! context-specific — what happened in one scenario. `catalog.md` is the context-free view of
//! the things. `commands/play.4x` looks like a view of the recipes and is not one; it is a
//! scenario, and reading it tells you what one run did rather than what a recipe is.
//!
//! **The release states a recipe across seven columns and many rows**, one line per
//! ingredient or result, with the recipe's name written only on its first. That is the right
//! shape for a table and the wrong one for reading: answering *what does `deploy ark` do*
//! means finding its first row, then reading down until the name changes, and holding the
//! column headings in your head throughout.
//!
//! This is the same rows with the recipe as the unit. Nothing here is a fact the release
//! does not state - `spec/invariants.md`: *every other form of it is derived, and a derived
//! form is generated rather than written.*

use crate::release::{body_under, plain};

/// The recipes, as markdown.
pub fn recipes(document: &str) -> String {
    let mut out = String::from("# Recipes\n\n");
    out.push_str(
        "**Generated. Do not edit.** `cargo run -p kinds -- recipes`, or `scripts/kinds.sh \
         recipes`.\n\n",
    );
    out.push_str(
        "Every recipe the release declares, with its own lines gathered under it. The release \
         states\nthese across seven columns, one row per line and the name written only on \
         the first - which\nis the right shape for a table and the wrong one for answering \
         *what does this recipe do*.\n\n",
    );

    let rows = body_under(document, "## Recipes");
    let gathered = gather(&rows);
    out.push_str(&format!(
        "{} recipes, {} lines between them.\n",
        gathered.len(),
        rows.len()
    ));

    for recipe in &gathered {
        out.push_str(&format!(
            "
## {}

",
            recipe.name
        ));
        out.push_str(&format!(
            "Run by the **{}**.

",
            recipe.owner
        ));
        for (role, said) in &recipe.lines {
            out.push_str(&format!("- **{role}** {said}\n"));
        }
    }
    out
}

/// One recipe, with the lines that belong to it.
pub struct Gathered {
    pub name: String,
    /// Whose recipe it is - the release's **Owner** column.
    pub owner: String,
    /// Each line's role and what it says, in the order the table gives them.
    pub lines: Vec<(String, String)>,
}

/// The rows of one recipe, with its name carried down from the row that has it.
///
/// **The name is on the first row only**, which is what makes the table hard to read and is
/// also the one thing a gatherer has to get right: a recipe whose name failed to carry would
/// silently absorb the next recipe's lines, and the result would look like a longer recipe
/// rather than like an error.
fn gather(rows: &[Vec<String>]) -> Vec<Gathered> {
    let mut out: Vec<Gathered> = Vec::new();
    for row in rows {
        let name = plain(row.first().map(String::as_str).unwrap_or_default());
        let owner = row.get(1).cloned().unwrap_or_default();
        if !name.is_empty() {
            out.push(Gathered {
                name,
                owner,
                lines: Vec::new(),
            });
        }
        let Some(recipe) = out.last_mut() else {
            continue; // a line before any name: the table's shape has changed
        };
        let role = row.get(2).cloned().unwrap_or_default();
        if role.is_empty() {
            continue;
        }
        recipe.lines.push((role, said(row)));
    }
    out
}

/// One line of a recipe, as a sentence rather than as cells.
fn said(row: &[String]) -> String {
    let quantity = row.get(3).cloned().unwrap_or_default();
    let kind = row.get(4).cloned().unwrap_or_default();
    let traits = row.get(5).cloned().unwrap_or_default();
    let place = row.get(6).cloned().unwrap_or_default();

    // **A quantity is not always a number.** `work` produces *`$where`'s density for that
    // resource* of a *resource*, and writing quantity-then-kind gives *...for that resource
    // resource*. When the quantity is not a bare count the kind leads instead, which reads
    // as the release means it: what is produced, then how much.
    let counted = quantity.chars().all(|c| c.is_ascii_digit()) && !quantity.is_empty();
    let mut out = if counted {
        format!("{quantity} {kind}")
    } else {
        format!("{kind} — {quantity}")
    };
    if !traits.is_empty() {
        out.push_str(&format!(", {traits}"));
    }
    if !place.is_empty() {
        out.push_str(&format!(", in {place}"));
    }
    out
}
