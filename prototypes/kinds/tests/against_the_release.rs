//! The compilable specification and the written one are the same specification.
//!
//! `S-4`. Two copies of fifteen recipes would be one copy and one guess: the data
//! in `src/lib.rs` is rendered back into the release's two tables and compared with
//! `releases/first-release.md` on disk, so neither can move without the other. That is the
//! habit `crates/game-console/tests/quotations.rs` already has - read the document at test
//! time rather than trusting a copy of it.
//!
//! **Cells are compared, not bytes.** `tools/pad-tables` owns the column widths and rewrites
//! them whenever anything else in the file changes, so a byte comparison would fail on
//! whitespace nobody wrote. Trimming each cell compares exactly what the table says and
//! nothing about how it is laid out.

use std::path::PathBuf;

fn release() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", path.display()))
}

/// The rows of the first table under this heading, as trimmed cells.
///
/// The separator row is dropped: it is punctuation, and the padder rewrites it.
fn table_under(document: &str, heading: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        let line = line.trim();
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line == heading;
            continue;
        }
        if !inside || !line.starts_with('|') {
            if inside && !rows.is_empty() && !line.starts_with('|') && !line.is_empty() {
                // Prose after the table ends it. A second table under one heading would be
                // a different document than this test was written for.
                break;
            }
            continue;
        }
        let cells: Vec<String> = line
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect();
        if cells.iter().all(|cell| cell.chars().all(|c| c == '-')) {
            continue;
        }
        rows.push(cells);
    }
    rows
}

fn compare(what: &str, written: &[Vec<String>], compiled: &[Vec<String>]) {
    let mut wrong = Vec::new();
    for (at, row) in compiled.iter().enumerate() {
        match written.get(at) {
            Some(theirs) if theirs == row => {}
            Some(theirs) => wrong.push(format!(
                "  row {at}\n    release: {}\n    crate:   {}",
                theirs.join(" | "),
                row.join(" | ")
            )),
            None => wrong.push(format!(
                "  row {at} is missing from the release: {}",
                row.join(" | ")
            )),
        }
    }
    for (at, theirs) in written.iter().enumerate().skip(compiled.len()) {
        wrong.push(format!(
            "  row {at} is in the release and not in the crate: {}",
            theirs.join(" | ")
        ));
    }
    assert!(
        wrong.is_empty(),
        "{what} in `releases/first-release.md` and in this crate are different:\n\n{}\n\n\
         The release is the specification. If it moved, move this crate to match; if this \n\
         crate is right, that is a proposal rather than an edit.",
        wrong.join("\n")
    );
}

/// Every table the release declares, against the data that renders it.
///
/// Six now rather than two. The release declares its kinds, its families, its bins and its
/// traits, which is what this crate used to infer from the recipes and say so.
/// Every table the release declares, against the data that renders it.
///
/// **The only test in this file that is worth anything on its own.** The others read this
/// crate and check it against numbers written in this crate; on the day the release moved,
/// seven of them passed against data that had stopped matching hours earlier. One asserted
/// eighteen recipes while there were sixteen, and another that `revert` names a place while
/// `revert` no longer existed. A test that reads one artifact can only say it has not
/// changed.
#[test]
fn the_release_tables_are_the_ones_in_this_crate() {
    let document = release();
    let tables: [(&str, usize, Vec<Vec<String>>); 7] = [
        ("## Kinds", 10, kinds::kinds_table()),
        ("## Families", 3, kinds::families_table()),
        ("## Where things are", 3, kinds::rooms_table()),
        ("## Traits", 16, kinds::traits_table()),
        ("## What a territory has room for", 10, kinds::room_table()),
        ("## Units and structures", 6, kinds::units_table()),
        ("## Recipes", 40, kinds::recipes_table()),
    ];
    for (heading, least, compiled) in tables {
        let written = table_under(&document, heading);
        // A parser that found nothing would agree with anything.
        assert!(
            written.len() > least,
            "only {} rows parsed under {heading}; its shape has changed",
            written.len()
        );
        compare(heading, &written, &compiled);
    }
}

/// Sixteen. Twenty became sixteen by collapsing, not by cutting: `launch` was `move` with a
/// different destination, `land` became `deploy ark`, `eat` was `upkeep` with a citizen's
/// upkeep assumed rather than written, `depart` was `perish`, and `revert` could never fire.
#[test]
fn there_are_sixteen_recipes() {
    assert_eq!(kinds::RECIPES.len(), 16);
}

/// A family names several kinds at once, with no hierarchy anywhere.
#[test]
fn a_recipe_naming_a_family_matches_every_kind_in_it() {
    use kinds::{Family, Kind};

    assert!(Family::Unit.covers(Kind::Ark));
    assert!(Family::Unit.covers(Kind::Pioneer));
    assert!(!Family::Unit.covers(Kind::Yard), "a yard does not move");
    for resource in [Kind::Food, Kind::Metal, Kind::Energy] {
        assert!(Family::Resource.covers(resource));
    }
    assert!(!Family::Resource.covers(Kind::Labor), "labor is not stored");
    for kind in kinds::KINDS {
        assert!(Family::Thing.covers(kind), "{} is a thing", kind.name());
    }
}

/// Consumed is worked out, not written down.
///
/// The column is gone: *an ingredient is consumed exactly when the same thing, with the same
/// traits, does not appear among the results*. So four recipes gained an echo row, and being
/// consumed became a fact about the table rather than a second statement that could disagree
/// with it.
#[test]
fn an_ingredient_is_consumed_unless_it_is_echoed() {
    let kept: Vec<&str> = kinds::RECIPES
        .iter()
        .filter(|recipe| {
            recipe
                .ports
                .iter()
                .filter(|port| port.is_ingredient())
                .any(|port| !recipe.consumes(port))
        })
        .map(|recipe| recipe.name)
        .collect();

    // A territory is never consumed by acting in it, a garrison survives producing a
    // pioneer, a yard survives producing an ark, and upkeep gives back what it fed.
    //
    // The two claiming recipes are here for a subtler reason, and it is the rule working
    // rather than an accident: each takes `garrison` at most 0 and gives `garrison` 1, so
    // the same thing with the same traits does appear among the results. Nothing was
    // consumed because there was nothing there - which is exactly what *at most zero* asks.
    assert_eq!(
        kept,
        [
            "deploy ark",
            "move",
            "found by land",
            "produce pioneer",
            "produce ark",
            "work",
            "grow",
            "upkeep"
        ]
    );

    // And the metal a build spends is gone, which is the ordinary case.
    let build = kinds::RECIPES
        .iter()
        .find(|recipe| recipe.name == "build yard")
        .expect("the release builds yards");
    for port in build.ports.iter().filter(|port| port.is_ingredient()) {
        assert!(build.consumes(port));
    }
}

/// Every qualifier a recipe uses names a trait the release declares.
///
/// **The check that holds the two halves of the specification against each other.** The
/// Traits table says what a thing can be distinguished by; the recipes distinguish things;
/// nothing else compares them, so a rewrite can take a trait out from under a recipe still
/// using it - which is how `control`, `force of nature` and `unpaid` came to be missing.
///
/// It reads recipe qualifiers, so it is blind to a trait no recipe qualifies by. A green
/// result here is not evidence that nothing is missing.
#[test]
fn every_qualifier_names_a_declared_trait() {
    let declared: Vec<&str> = kinds::TRAITS.iter().map(|row| row.name).collect();
    let mut undeclared: Vec<&str> = Vec::new();
    let mut seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();

    for recipe in kinds::RECIPES {
        for port in recipe.ports {
            for qualifier in port.subject().qualifiers {
                seen.insert(qualifier.written);
                match qualifier.of_trait {
                    Some(named) => assert!(
                        declared.contains(&named),
                        "{} distinguishes by `{}`, which the Traits table does not declare",
                        recipe.name,
                        named
                    ),
                    None => undeclared.push(qualifier.written),
                }
            }
        }
    }
    undeclared.sort_unstable();
    undeclared.dedup();
    assert_eq!(undeclared, [] as [&str; 0]);

    // A universal check over an empty set passes, so this says how much it looked at.
    assert!(
        seen.len() >= 10,
        "only {} distinct qualifiers seen: {seen:?}",
        seen.len()
    );
}

/// A name is how a recipe reaches past what it consumes.
///
/// `$where`, `$from` and `$to` are bound by one ingredient and referred to by others, which
/// is what lets `work` yield the territory's density without the territory being consumed
/// and without the table having a column for where a recipe acts.
#[test]
fn a_named_ingredient_is_bound_before_it_is_referred_to() {
    for recipe in kinds::RECIPES {
        let bound: Vec<&str> = recipe
            .ports
            .iter()
            .filter_map(|port| port.subject().bound_as)
            .collect();
        for port in recipe.ports {
            let text = format!("{} {}", port.subject().written(), port.quantity().written());
            for name in ["where", "from", "to"] {
                if text.contains(&format!("`${name}`")) {
                    assert!(
                        bound.contains(&name),
                        "{} refers to `${name}` and never binds it",
                        recipe.name
                    );
                }
            }
        }
    }

    // Three recipes name something, and they are the three that act somewhere in particular.
    let naming: Vec<&str> = kinds::RECIPES
        .iter()
        .filter(|recipe| {
            recipe
                .ports
                .iter()
                .any(|port| port.subject().bound_as.is_some())
        })
        .map(|recipe| recipe.name)
        .collect();
    assert_eq!(naming, ["deploy ark", "move", "work"]);
}

/// `Metal in it` is derived, and the release says so.
#[test]
fn metal_in_it_is_its_binding_plus_its_parts() {
    for thing in kinds::PRODUCIBLE {
        assert_eq!(thing.metal_in_it(), thing.binding, "{}", thing.kind.name());
    }
}
