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
#[test]
fn the_release_tables_are_the_ones_in_this_crate() {
    let document = release();
    let tables: [(&str, usize, Vec<Vec<String>>); 6] = [
        ("## Kinds", 10, kinds::kinds_table()),
        ("## Families", 3, kinds::families_table()),
        ("## Where things are", 3, kinds::bins_table()),
        ("## Traits", 13, kinds::traits_table()),
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

/// Eighteen: the fifteen `S-4` asked for, and `upkeep`, `perish` and `revert`.
#[test]
fn there_are_eighteen_recipes() {
    assert_eq!(kinds::RECIPES.len(), 18);
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

    // `thing` is every kind, which is what the release now says and what this crate once
    // had to report as a gap - `ready` named a family nothing defined.
    for kind in kinds::KINDS {
        assert!(Family::Thing.covers(kind), "{} is a thing", kind.name());
    }
}

/// The measurements the answers rest on, so a later change makes them stale loudly.
#[test]
fn the_figures_the_answers_rest_on() {
    use kinds::{Quantity, Scope};

    let here = kinds::RECIPES
        .iter()
        .filter(|recipe| recipe.scope == Scope::Here)
        .count();
    assert_eq!((here, kinds::RECIPES.len() - here), (10, 8));

    // A quantity is written or read. Three are read, and one of those reads past the
    // recipe's own ingredients - which is the distinction the variants carry.
    let read: Vec<Quantity> = kinds::RECIPES
        .iter()
        .flat_map(|recipe| recipe.ports.iter())
        .map(|port| port.quantity())
        .filter(|quantity| !matches!(quantity, Quantity::Exactly(_)))
        .collect();
    assert_eq!(read.len(), 3, "upkeep, perish and work");
    assert_eq!(
        read.iter()
            .filter(|quantity| matches!(quantity, Quantity::OfThePlace(_)))
            .count(),
        1,
        "only `work` reads a trait of the place rather than of an ingredient"
    );
}

/// `work` is the one recipe whose quantity is not a trait of anything it names.
///
/// `releases/first-release.md` says a quantity is *written in the recipe, or read from a
/// trait of one of the ingredients*. `work`'s ingredients are labor and an extractor, and
/// it yields the **territory's** density - so the sentence is false of the row three lines
/// below it. Reported as `P-151`; this pins the fact the proposal rests on.
#[test]
fn only_work_reads_past_its_own_ingredients() {
    use kinds::Quantity;

    let odd: Vec<&str> = kinds::RECIPES
        .iter()
        .filter(|recipe| {
            recipe
                .ports
                .iter()
                .any(|port| matches!(port.quantity(), Quantity::OfThePlace(_)))
        })
        .map(|recipe| recipe.name)
        .collect();
    assert_eq!(odd, ["work"]);

    // And the two that do read an ingredient really do name it among their ingredients.
    for recipe in kinds::RECIPES {
        for port in recipe.ports {
            if matches!(port.quantity(), Quantity::OfAnIngredient(_)) {
                assert!(
                    recipe.ports.iter().any(|other| other.is_ingredient()
                        && other.subject().noun.name() == "unit"),
                    "{} reads a unit's trait without taking a unit",
                    recipe.name
                );
            }
        }
    }
}

/// A territory is a place, not a thing, and one recipe names one.
///
/// The release's *Where things are* says every thing is in a bin, and a territory **is** a
/// bin. `revert` names it in the Thing column anyway, which is the one place the table's
/// noun is not a thing.
#[test]
fn only_revert_names_a_place_where_a_thing_goes() {
    use kinds::Noun;

    let naming_a_place: Vec<&str> = kinds::RECIPES
        .iter()
        .filter(|recipe| {
            recipe
                .ports
                .iter()
                .any(|port| port.subject().noun == Noun::Territory)
        })
        .map(|recipe| recipe.name)
        .collect();
    assert_eq!(naming_a_place, ["revert"]);
}

/// The two halves of the specification, held against each other.
///
/// **This is the check neither lane had.** The Traits table says what a thing can be
/// distinguished by; the recipes distinguish things. Nothing compared them, so a rewrite
/// could take a trait out from under a recipe that still uses it - which is what happened:
/// `P-143` declared a territory's traits, `P-148` rewrote that section around bins, and the
/// territory's own traits went with the column that stopped existing. `revert` still asks
/// whether a territory is claimed, and no declared trait says one can be.
///
/// Reported rather than asserted away. The list below is the release's, and shrinking it is
/// the specification lane's work; what this test guarantees is that it cannot grow in
/// silence.
#[test]
fn every_qualifier_names_a_declared_trait() {
    let declared: Vec<&str> = kinds::TRAITS.iter().map(|row| row.name).collect();

    let mut undeclared: Vec<&str> = kinds::RECIPES
        .iter()
        .flat_map(|recipe| recipe.ports.iter())
        .filter_map(|port| port.subject().qualified_by)
        .filter(|qualifier| qualifier.of_trait.is_none())
        .map(|qualifier| qualifier.written)
        .collect();
    undeclared.sort_unstable();
    undeclared.dedup();

    // Every qualifier that does name a trait names one the release declares.
    for recipe in kinds::RECIPES {
        for port in recipe.ports {
            if let Some(qualifier) = port.subject().qualified_by
                && let Some(named) = qualifier.of_trait
            {
                assert!(
                    declared.contains(&named),
                    "{} distinguishes by `{}`, which the Traits table does not declare",
                    recipe.name,
                    named
                );
            }
        }
    }

    assert_eq!(
        undeclared,
        [
            "force below its force of nature",
            "unclaimed",
            "whose upkeep is unpaid",
        ],
        "the qualifiers no declared trait accounts for. P-152 is about the first two - a \
         territory's control and its force of nature are traits the release uses and does \
         not declare. If this list has shrunk, the release answered one; if it has grown, a \
         recipe started asking something nothing can answer."
    );
}

/// Every noun a recipe names is a declared kind, a declared family, or the place itself.
#[test]
fn every_noun_is_declared_or_is_a_place() {
    use kinds::Noun;

    for recipe in kinds::RECIPES {
        for port in recipe.ports {
            match port.subject().noun {
                Noun::Of(kind) => assert!(
                    kinds::KINDS.contains(&kind),
                    "{} names {}, which is not one of the ten",
                    recipe.name,
                    kind.name()
                ),
                Noun::Any(family) => assert!(
                    kinds::FAMILIES.contains(&family),
                    "{} names the family {}, which is not declared",
                    recipe.name,
                    family.name()
                ),
                // `revert` names a territory, which is a bin rather than a thing in one.
                Noun::Territory => assert_eq!(recipe.name, "revert"),
            }
        }
    }
}
