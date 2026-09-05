//! The compilable specification and the written one are the same specification.
//!
//! `S-4`. Two copies of seventeen recipes would be one copy and one guess: the data
//! in `src/lib.rs` is rendered back into the release's seven tables and compared with
//! `releases/first-release.md` on disk, so neither can move without the other. That is the
//! habit `crates/game-console/tests/quotations.rs` already has - read the document at test
//! time rather than trusting a copy of it.
//!
//! **Cells are compared, not bytes.** `tools/pad-tables` owns the column widths and rewrites
//! them whenever anything else in the file changes, so a byte comparison would fail on
//! whitespace nobody wrote. Trimming each cell compares exactly what the table says and
//! nothing about how it is laid out.

use kinds::release::{release, table_under};

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
/// Seven now rather than two. The release declares its kinds, its families, its capacities
/// and its traits, which is what this crate used to infer from the recipes and say so.
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
        ("## Kinds", 12, kinds::kinds_table()),
        ("## Families", 3, kinds::families_table()),
        ("## Where things are", 3, kinds::capacities_table()),
        ("## Traits", 17, kinds::traits_table()),
        (
            "## What bounds a kind in a territory",
            10,
            kinds::bounds_table(),
        ),
        ("## Units and structures", 6, kinds::units_table()),
        ("## Recipes", 50, kinds::recipes_table()),
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

/// Fifteen. Twenty became sixteen by collapsing, not by cutting: `launch` was `move` with
/// a different destination, `land` became `deploy ark`, `eat` was `upkeep` with a citizen's
/// upkeep assumed rather than written, `depart` was `perish`, and `revert` could never fire.
/// Then food gained a `keeps` counter, and `age` is what counts it down.
#[test]
fn there_are_fifteen_recipes() {
    assert_eq!(kinds::RECIPES.len(), 15);
}

/// Every name in the recipes' `Kind` column is a kind or a family the release declares.
///
/// **The check that was missing, and the gap it sat in is the point.** The comparison above
/// holds this crate against the release; `a_recipe_naming_a_family_matches_every_kind_in_it`
/// holds a family against its members. Nothing asked whether the recipes use a name the
/// release declares at all - and `territory` appeared in four recipe rows, in neither the
/// Kinds table nor the Families table, for as long as the recipes have existed. `P-192`
/// fixed it; this is what would have found it.
///
/// **It reads the document rather than this crate, deliberately.** The crate's types now
/// make the bug unwritable, which is worth having and is not a check: a type stops this
/// crate from disagreeing with itself, and the defect was the release disagreeing with
/// itself. Only the document can answer that, so only the document is read.
#[test]
fn every_kind_a_recipe_names_is_declared() {
    let document = release();
    let (used, undeclared) = undeclared_kinds(&document);
    assert!(
        undeclared.is_empty(),
        "the recipes name {undeclared:?}, which the Kinds and Families tables do not declare"
    );

    // Over every case, and how many cases there were. A column that stopped being the fifth
    // would leave this checking an empty set and passing.
    assert_eq!(
        used.len(),
        15,
        "fifteen distinct names across the recipes' Kind column, and these are {used:?}"
    );
}

/// Every name the recipes use, and the ones no table declares.
///
/// Separated from the test so it can be run against a document written to be wrong. A check
/// that has never failed is a claim, not evidence.
fn undeclared_kinds(document: &str) -> (Vec<String>, Vec<String>) {
    let mut declared: Vec<String> = Vec::new();
    for heading in ["## Kinds", "## Families"] {
        for row in table_under(document, heading).iter().skip(1) {
            if let Some(cell) = row.first() {
                declared.push(cell.trim_matches('*').to_string());
            }
        }
    }

    let mut used: Vec<String> = Vec::new();
    for row in table_under(document, "## Recipes").iter().skip(1) {
        let named = row.get(4).map(|cell| cell.trim()).unwrap_or_default();
        if !named.is_empty() && !used.iter().any(|seen| seen == named) {
            used.push(named.to_string());
        }
    }
    used.sort();

    let undeclared = used
        .iter()
        .filter(|name| !declared.contains(name))
        .cloned()
        .collect();
    (used, undeclared)
}

/// The bug `P-192` fixed, as a document this check is run against.
///
/// **`territory` appeared in four recipe rows and in neither declaring table**, for as long
/// as the recipes have existed, and nothing asked. The comparison test holds this crate
/// against the release and the family test holds a family against its members; an
/// undeclared name fell exactly between them. This is that release in miniature - the
/// recipes name a territory, the Kinds table does not list one - and the check has to find
/// it, or it is not the check that would have found it.
#[test]
fn the_check_finds_the_bug_it_exists_for() {
    let wrong = "## Kinds

| Kind        | What it is    |
| ----------- | ------------- |
| **citizen** | a person      |
| **ark**     | carries a landing |

## Families

| Family    | Members          |
| --------- | ---------------- |
| **thing** | every kind above |

## Recipes

| Recipe         | Auto   | Role    | Qty | Kind      | Traits | Where    |
| -------------- | ------ | ------- | --- | --------- | ------ | -------- |
| **deploy ark** | player | require | 1   | territory |        | `$where` |
|                |        | consume | 1   | ark       |        | `$where` |
|                |        | produce | 1   | citizen   |        |          |
";
    let (used, undeclared) = undeclared_kinds(wrong);
    assert_eq!(used, ["ark", "citizen", "territory"], "it read the column");
    assert_eq!(
        undeclared,
        ["territory"],
        "an undeclared name in the Kind column has to be found"
    );

    // And a document that declares it is clean, so the check is not simply always red.
    let fixed = wrong.replace(
        "| **ark**     | carries a landing |",
        "| **ark**     | carries a landing |
| **territory** | a place things are in |",
    );
    let (_, undeclared) = undeclared_kinds(&fixed);
    assert!(
        undeclared.is_empty(),
        "declaring it is enough: {undeclared:?}"
    );
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

    // **A rule changed when the table grew, and this is where it shows.** `thing` is *every
    // kind above*, so declaring a territory put one inside the family - and `grow` requires
    // `thing, houses`, a trait *of a thing that contains things*. A territory houses its
    // citizens, and `grow` could not match one before `P-192`.
    //
    // `docs/recipes/README.md` has rendered `territory (houses)` in that recipe since it
    // was written. The rendering was right and the data could not say it.
    assert!(
        Family::Thing.covers(Kind::Territory),
        "a territory is a thing, so `grow` can require the thing that houses its citizens"
    );
    let grow = kinds::RECIPES
        .iter()
        .find(|recipe| recipe.name == "grow")
        .expect("the world grows citizens");
    let houses = grow
        .lines
        .iter()
        .find(|line| line.traits.iter().any(|t| t.written == "houses"))
        .expect("grow requires something that houses");
    assert_eq!(
        houses.noun.name(),
        "thing",
        "and it requires it by family, which is what makes a territory eligible"
    );
}

/// Every role the release writes is one of the four, and each is used.
///
/// **Consumed is a column now, and used to be worked out.** The rule was *an ingredient is
/// consumed exactly when the same thing, with the same traits, does not appear among the
/// results*, so four recipes carried an echo row that existed only to say something
/// survived. Saying it once is the better trade, and it bought `limit` - *at most none of
/// these* - which the echo scheme could only spell as zero given back.
///
/// So this checks the thing the column made checkable: that every role is exercised, and
/// that the recipes keeping something are the ones that should.
#[test]
fn a_role_says_what_becomes_of_what_a_recipe_names() {
    use kinds::Role;

    let mut seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for recipe in kinds::RECIPES {
        for line in recipe.lines {
            seen.insert(line.role.written());
        }
    }
    assert_eq!(
        seen.into_iter().collect::<Vec<_>>(),
        ["consume", "limit", "produce", "require"],
        "all four roles are used, and nothing else is"
    );

    // A territory is never consumed by acting in it, unheld ground is a limit rather than an
    // ingredient, a yard survives producing an ark, and upkeep and grow both need something
    // they do not eat.
    let keeps: Vec<&str> = kinds::RECIPES
        .iter()
        .filter(|recipe| {
            recipe
                .lines
                .iter()
                .any(|line| line.is_ingredient() && !line.role.consumes())
        })
        .map(|recipe| recipe.name)
        .collect();
    assert_eq!(
        keeps,
        [
            "deploy ark",
            "move",
            "found by land",
            "produce ark",
            "work",
            "upkeep",
            "grow"
        ]
    );

    // And the metal a build spends is gone, which is the ordinary case.
    let build = kinds::RECIPES
        .iter()
        .find(|recipe| recipe.name == "build yard")
        .expect("the release builds yards");
    for line in build.lines.iter().filter(|line| line.is_ingredient()) {
        assert_eq!(line.role, Role::Consume);
    }
}

/// Every trait a recipe distinguishes by is one the release declares.
///
/// **The check that holds the two halves of the specification against each other.** The
/// Traits table says what a thing can be distinguished by; the recipes distinguish things;
/// nothing else compares them, so a rewrite can take a trait out from under a recipe still
/// using it - which is how `control`, `force of nature` and `unpaid` came to be missing.
///
/// It reads the Traits column, so it is blind to a trait no recipe distinguishes by. A green
/// result here is not evidence that nothing is missing.
#[test]
fn every_qualifier_names_a_declared_trait() {
    let declared: Vec<&str> = kinds::TRAITS.iter().map(|row| row.name).collect();
    let mut undeclared: Vec<&str> = Vec::new();
    let mut seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();

    for recipe in kinds::RECIPES {
        for line in recipe.lines {
            for qualifier in line.traits {
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
/// `$where`, `$from` and `$to` are bound by requiring a territory and referred to by other
/// lines, which is what lets `work` yield the territory's density without the territory
/// being consumed, and lets `move` say *this one* and *that one*.
#[test]
fn a_named_ingredient_is_bound_before_it_is_referred_to() {
    for recipe in kinds::RECIPES {
        let bound = recipe.binds();
        for text in recipe.mentions() {
            for name in ["`$where`", "`$from`", "`$to`"] {
                if text.contains(name) {
                    assert!(
                        bound.contains(&name),
                        "{} refers to {name} and never binds it",
                        recipe.name
                    );
                }
            }
        }
    }

    // Three recipes name something, and they are the three that act somewhere in particular.
    let naming: Vec<(&str, Vec<&str>)> = kinds::RECIPES
        .iter()
        .filter(|recipe| !recipe.binds().is_empty())
        .map(|recipe| (recipe.name, recipe.binds()))
        .collect();
    assert_eq!(
        naming,
        [
            ("deploy ark", vec!["`$where`"]),
            ("move", vec!["`$from`", "`$to`"]),
            ("work", vec!["`$where`"]),
        ]
    );
}

/// `Metal in it` is derived, and the release says so.
#[test]
fn metal_in_it_is_its_binding_plus_its_parts() {
    for thing in kinds::PRODUCIBLE {
        assert_eq!(thing.metal_in_it(), thing.binding, "{}", thing.kind.name());
    }
}

/// What a trait says its values are is borne out by the table that lists them.
///
/// **This is the check the cell-by-cell comparison structurally cannot make.** That one
/// holds this crate against the release, and this crate *copies* the release - so when the
/// `kind` trait said *one of the twelve* while the Kinds table had fourteen rows, both said
/// twelve and both agreed. **A comparison of two things that agree cannot notice that they
/// are both wrong.** Only reading one cell against the count of rows in another does.
///
/// The count went stale twice in two days - *ten* until `P-192`, *twelve* until `P-206` -
/// and `P-209` and `P-210` then deleted it, which is the better fix and is why this now
/// reads *one of the kinds*.
///
/// **The general arm was here before that landed and did not work**, which is worth keeping
/// rather than quietly repairing: *one of the kinds* still contains the word **one**, so a
/// parser scanning for any number word answered 1 and compared it with 14. A count is the
/// word **after `of the`** and nowhere else, which is what this reads now.
#[test]
fn what_a_trait_says_its_values_are_is_borne_out_by_the_table() {
    let document = release();

    // The traits that name a closed set, and the table each set is written out in.
    let counted: [(&str, &str, &str); 2] = [
        ("kind", "## Kinds", "kinds"),
        ("biome", "## Biomes", "biomes"),
    ];

    let after_of_the = |said: &str| -> Option<String> {
        let at = said.find("of the ")? + "of the ".len();
        Some(
            said[at..]
                .split_whitespace()
                .next()?
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_string(),
        )
    };

    let number = |word: &str| -> Option<usize> {
        const WORDS: [&str; 20] = [
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
            "twenty",
        ];
        WORDS
            .iter()
            .position(|w| *w == word)
            .map(|at| at + 1)
            .or_else(|| word.parse().ok())
    };

    let mut named = 0usize;
    let mut with_a_count = 0usize;

    for (trait_name, heading, set) in counted {
        let row = table_under(&document, "## Traits")
            .into_iter()
            .find(|row| row.first().map(|c| c.trim_matches('*')) == Some(trait_name))
            .unwrap_or_else(|| panic!("the Traits table declares no `{trait_name}`"));
        let values = row.get(2).cloned().unwrap_or_default();

        let listed = table_under(&document, heading).len().saturating_sub(1);
        assert!(
            listed > 0,
            "{heading} lists nothing, so this would agree with anything at all"
        );

        let Some(word) = after_of_the(&values) else {
            panic!("the `{trait_name}` trait says {values:?}, which names no set of values")
        };

        match number(&word) {
            // A stated count has to match. The strong case, and the one that was wrong.
            Some(said) => {
                with_a_count += 1;
                assert_eq!(
                    said, listed,
                    "the `{trait_name}` trait says {values:?} and {heading} has {listed} \
                     rows. One moved and the other did not - and this crate copies the \
                     release, so the cell-by-cell comparison agrees with both."
                );
            }
            // No count, so nothing can be miscounted - but the set it names still has to be
            // the table that exists. `P-209`'s shape, and the reason it is the better fix.
            None => {
                named += 1;
                assert_eq!(
                    word, set,
                    "the `{trait_name}` trait says {values:?}, which names neither a count \
                     nor {set}, so nothing in this document lists what it admits"
                );
            }
        }
    }

    // Both arms are real code and only one of them runs today, so this says which - a run
    // where neither fired would be a run over no traits at all.
    assert_eq!(
        named + with_a_count,
        2,
        "two traits name a closed set; {named} named one and {with_a_count} counted one"
    );
}

/// Both arms of the check above work, over documents written to exercise each.
///
/// **The count arm is dead code against today's release** - `P-209` and `P-210` removed
/// both counts - so without this, the case that was wrong for two days would be tested by
/// nothing and could rot unnoticed until the next promotion reintroduced a number.
#[test]
fn the_check_catches_a_miscount_and_accepts_a_named_set() {
    let document = |values: &str| {
        format!(
            "## Kinds\n\n\
             | Kind        | What it is |\n\
             | ----------- | ---------- |\n\
             | **citizen** | a person   |\n\
             | **ark**     | it carries |\n\n\
             ## Traits\n\n\
             | Trait    | Of          | Values  | Stored or derived |\n\
             | -------- | ----------- | ------- | ----------------- |\n\
             | **kind** | every thing | {values} | stored           |\n"
        )
    };

    let two = document("one of the two");
    let wrong = document("one of the nine");
    let named = document("one of the kinds");

    let count_of = |text: &str| -> Option<usize> {
        let row = table_under(text, "## Traits")
            .into_iter()
            .find(|row| row.first().map(|c| c.trim_matches('*')) == Some("kind"))?;
        let values = row.get(2)?.clone();
        let at = values.find("of the ")? + "of the ".len();
        let word = values[at..].split_whitespace().next()?.to_string();
        const WORDS: [&str; 20] = [
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
            "twenty",
        ];
        WORDS.iter().position(|w| *w == word).map(|at| at + 1)
    };

    let kinds = |text: &str| table_under(text, "## Kinds").len() - 1;
    assert_eq!(kinds(&two), 2, "two kinds written");
    assert_eq!(count_of(&two), Some(2), "and a count that agrees");
    assert_eq!(count_of(&wrong), Some(9), "a count that does not");
    assert_ne!(
        count_of(&wrong),
        Some(kinds(&wrong)),
        "and it must disagree"
    );

    // The arm that runs today: a named set carries no number at all, which is exactly the
    // reading that defeated the first version of this.
    assert_eq!(
        count_of(&named),
        None,
        "`one of the kinds` states no count, and reading its leading `one` as one is the \
         bug this line exists to keep out"
    );
}
