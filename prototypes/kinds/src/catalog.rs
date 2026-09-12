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

use crate::release::{body_under, column_of, plain};

/// The catalog, as markdown, from the release document.
pub fn catalog(document: &str) -> String {
    // **Read once, here, and handed down** - see [`Declared`] for why not inside the reader.
    let declared = Declared::from_spec();
    let mut out = String::new();
    out.push_str("# Catalog\n\n");
    out.push_str(
        "**Generated. Do not edit.** `cargo run -p kinds -- catalog`, or `scripts/kinds.sh catalog`.\n\n",
    );
    out.push_str(
        "Every kind the release declares, with everything it says about that kind gathered in \
         one place.\n**This is a derived form of the release's tables** - `spec/invariants.md`: \
         *a fact is stated\nonce and every other form of it is derived*, and a derived form is \
         generated rather than written.\nIt is a view and not a copy - each section is a join \
         across six tables that the document does not\nperform anywhere.\n\n**The release's \
         tables are not themselves canonical for all of this.** Four of them - Kinds,\nFamilies, \
         Biomes and Traits - have files in `spec/data/`, which is where those facts are stated; \
         the\nrelease's four are a second hand-written form, and `P-465` is about what that \
         costs. This reads\nthe release because that is where all six tables are today, and says \
         so rather than implying the\nrelease is the source.\n\n",
    );

    let kinds = body_under(document, "## Kinds");
    out.push_str(&format!(
        "{} kinds, {} families, {} traits, {} recipes.\n\n",
        kinds.len(),
        body_under(document, "## Families").len(),
        body_under(document, "## Traits").len(),
        recipe_names(document).len()
    ));

    out.push_str(&groups(document, &declared));

    for row in &kinds {
        let name = plain(&row[0]);
        out.push_str(&section(
            document,
            &declared,
            &name,
            row.get(1).map(String::as_str),
        ));
    }
    out
}

/// The kinds that behave alike, shown together - `R-8`.
///
/// **What a signature is for is the comparison, so the comparison is the section.** Each
/// kind's own section names its signature, which is the thing to grep for; this is where two
/// kinds sharing one becomes visible without reading fifteen sections and holding them in
/// mind.
///
/// **A group of one is listed too.** A page that showed only the collisions would answer
/// *which kinds behave alike* and leave *does this one behave like anything* unanswered, and
/// the second question is the one asked while reading a single kind.
fn groups(document: &str, declared: &Declared) -> String {
    let found = signatures(document, declared);
    let together = found.iter().filter(|(_, _, kinds)| kinds.len() > 1).count();
    let mut out = String::from("## Signatures\n\n");
    out.push_str(
        "**A kind's signature is the traits it carries and every *(recipe, role)* pair that \
         names it.**\nComputed from the tables above rather than written by anyone, so two \
         kinds share one exactly\nwhen the release says the same things about them. \
         Quantities are not part of it: two kinds that\nare produced in different numbers by \
         the same recipe still behave alike. **Being named through a\nfamily counts**, \
         because a family is how the release addresses several kinds at once.\n\n",
    );
    // **The headline is the finding, not the arithmetic - `S-75`.** *Sixteen kinds fall into
    // sixteen signatures, and 0 of those hold more than one* is true and reads as a broken
    // report; *no two of the sixteen kinds behave alike, over 120 pairs* is the same fact and
    // reads as an answer. The counting form is kept for the day something does collide,
    // because then the counts are what a reader wants.
    //
    // **The pair count is computed rather than written.** It was 105 at fifteen kinds and is
    // 120 at sixteen, and a number in generated prose that somebody has to remember to edit is
    // exactly what this file keeps finding stale.
    let counted = found.iter().map(|(_, _, kinds)| kinds.len()).sum::<usize>();
    let pairs = counted * counted.saturating_sub(1) / 2;
    if together == 0 {
        out.push_str(&format!(
            "**No two of the {counted} kinds behave alike**, over all {pairs} pairs of them.\n\n"
        ));
    } else {
        out.push_str(&format!(
            "{counted} kinds fall into {} signatures over {pairs} pairs, and {together} of those\nhold more than one kind.\n\n",
            found.len(),
        ));
    }

    // **A zero here is a finding, and a zero on its own reads as a statistic.** `R-8` is
    // vetted by scanning the groups and seeing that two kinds behave alike - and today there
    // is nothing to scan, because the signature separates all fifteen. That is a fact about
    // the release rather than a defect in this view, and the person vetting it has to be told
    // before he looks rather than left to conclude the report is broken. `C-64`.
    //
    // **Written as a condition rather than a paragraph**, so it disappears by itself the day
    // two kinds collide, and nothing has to remember to delete it.
    if together == 0 {
        // **The number here is computed, not counted by eye.** It was written by hand first
        // - *four kinds carry `force` and `kind`* - which is a derived number in generated
        // prose, and this file is full of reasons that goes stale without anyone editing it.
        let sharing = found
            .iter()
            .filter(|(_, mine, _)| {
                found
                    .iter()
                    .any(|(_, other, _)| !std::ptr::eq(mine, other) && other.traits == mine.traits)
            })
            .count();
        out.push_str(&format!(
            "**Every group below holds one kind**, which is what that sentence means when\nyou reach them. **The traits alone do collide** - {sharing} of the kinds carry exactly\nthe traits another one carries - and every such pair is then separated by the recipes\nthat name it. So the release has no two kinds it says *the same things* about, and Sean\nhas accepted that as the answer: he expects a small number of distinct things.\n\n"
        ));
    }

    for (name, signature, kinds) in &found {
        out.push_str(&format!("### `{name}` - {}\n\n", kinds.join(", ")));
        let traits = if signature.traits.is_empty() {
            String::from("none")
        } else {
            signature
                .traits
                .iter()
                .map(|it| format!("`{it}`"))
                .collect::<Vec<String>>()
                .join(", ")
        };
        out.push_str(&format!("**Traits** {traits}\n\n"));
        if signature.pairs.is_empty() {
            // A kind no recipe names cannot be made, used or destroyed, and two such kinds
            // share a signature by having nothing rather than by behaving alike. Worth
            // saying where it happens rather than leaving an empty line to be read as a bug.
            out.push_str("**Named by** no recipe at all.\n\n");
        } else {
            out.push_str(&format!(
                "**Named by** {}\n\n",
                signature
                    .pairs
                    .iter()
                    .map(|pair| format!("`{pair}`"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }
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
    let rows = body_under(document, "## Recipes");
    if rows.is_empty() {
        return Vec::new();
    }
    // **`C-70` again, and this was the one the first pass missed.** The recipe's own name was
    // `row[0]`, which is only true while *Recipe* is the first column.
    let name_at = column_of(document, "## Recipes", "Recipe");
    let mut current = String::new();
    let mut out = Vec::new();
    for row in rows {
        let name = plain(&row[name_at]);
        if !name.is_empty() {
            current = name;
        }
        out.push((current.clone(), row));
    }
    out
}

/// Whether `phrase` names `name`.
///
/// **A dash is part of a name and not a separator.** `spec/console.md`: *A name is one word.
/// Where it needs more than one, the words are joined with dashes.* So a hyphenated name is
/// one word here, and splitting on the dash would find neither it nor its halves.
///
/// **One implementation because it is one rule**, which is `Q-70`: this file asked *does this
/// phrase name this thing* in three places and answered it two different ways, and the two
/// that split on the dash outvoted the one that did not. The failure was silent in the
/// direction that matters, and this is measured against the code as it was rather than
/// argued: hyphenating all fifteen kinds merged `deposit` and `adjacency` onto one key, so
/// `R-8` reported two unrelated kinds as behaving alike because a matcher failed on both.
/// **Only kinds with no recipe pairs merged** - the family match below is an exact string
/// compare and survives a rename - so the cost was one false group rather than all of them.
fn names_it(phrase: &str, name: &str) -> bool {
    phrase
        .split(|c: char| !c.is_alphanumeric() && c != '-')
        .any(|word| word == name)
}

/// What each row of *Where things are* is about, and how much it holds.
///
/// **`S-58`, and Sean hit its absence rather than reading about it.** He asked whether
/// territory 1's twelve energy was disorder about to be wiped. Answering it needs the
/// capacity, which is the stores times what a store holds, and **ten was in none of the four
/// artifacts** - in his words, *it isn't necessarily 10, it just happened to be 10, which
/// means I needed that information explicitly.* The release states it;
/// `releases/first-release.md` -> *Where things are*, `| a store | the resource it was built
/// for | 10 |`. This function is why it was not being read.
///
/// # The first column is a container, not a kind
///
/// It reads *a store*, *a unit's tank*, *a territory's total capacity for a kind* - so the
/// `plain(&row[0]) == kind` matcher every other heading uses matches none of them. **A phrase
/// naming a kind or a family is what relates a row to a section**, and a family carries the
/// row to each of its members, so *a unit's tank* is found under `ark` and under `pioneer`.
///
/// **It panics when a row names neither.** `S-58` asks for that in as many words, and the
/// reason is what happened here: a matcher that finds nothing contributes no line, and a
/// missing line reads exactly like a kind that holds nothing. **A row that stops matching has
/// to be loud**, because the quiet version of this is the defect being fixed.
fn containers(document: &str) -> Vec<(String, Vec<String>)> {
    let kinds: Vec<String> = body_under(document, "## Kinds")
        .iter()
        .map(|row| plain(&row[0]))
        .collect();
    let families: Vec<(String, Vec<String>)> = body_under(document, "## Families")
        .iter()
        .map(|row| {
            let members = row.get(1).map(String::as_str).unwrap_or_default();
            let members = if members == "every kind above" {
                kinds.clone()
            } else {
                members.split(',').map(|m| m.trim().to_string()).collect()
            };
            (plain(&row[0]), members)
        })
        .collect();
    assert!(
        !kinds.is_empty() && !families.is_empty(),
        "the release lists no kinds or no families, so every row below would match nothing"
    );

    let mut out = Vec::new();
    for row in body_under(document, "## Where things are") {
        let phrase = plain(&row[0]);
        let mut covers: Vec<String> = kinds
            .iter()
            .filter(|kind| names_it(&phrase, kind))
            .cloned()
            .collect();
        for (family, members) in &families {
            if names_it(&phrase, family) {
                covers.extend(members.iter().cloned());
            }
        }
        covers.sort();
        covers.dedup();
        assert!(
            !covers.is_empty(),
            "`{phrase}` in *Where things are* names no kind and no family, so this row \
             contributes nothing to any section - which is exactly how `10` went missing. \
             Relate it to a kind or say here why it has none."
        );
        for kind in covers {
            out.push((kind, row.clone()));
        }
    }
    assert!(
        !out.is_empty(),
        "*Where things are* yielded no rows at all, so every section below would be silently \
         short one line"
    );
    out
}

/// How a Recipes row reaches a kind, which is the only part `section` phrases differently.
#[derive(Clone, PartialEq, Eq)]
enum Reach {
    /// The Kind cell is the kind itself.
    Directly,
    /// The Kind cell is a family the kind belongs to - `move` takes a `unit`.
    ViaFamily(String),
    /// The Where cell names the kind as the place something else is in - `deploy ark`
    /// takes its Ark from *the orbit above `$where`*.
    AsPlace(String),
}

/// Every family this kind belongs to, and the kind's own name with them.
///
/// **One implementation, because it is one rule - `C-71`, and `Q-70` one level up.** The two
/// halves of a signature disagreed about whether a family counts: `recipe_rows` built this list
/// and `trait_rows` matched the kind's own name alone, so a trait declared *of a unit* reached
/// neither ark nor pioneer. `Signature`'s own doc said reaching through a family counts as
/// naming - true of one half and false of the other, in the same struct's documentation.
///
/// **`every kind above` is a membership and not a name.** The *thing* family is written that
/// way, so splitting on commas finds nothing in it and `keeps`, declared *of thing*, reached
/// none of the sixteen kinds. `containers` had handled this literal and the two joins had not.
fn families_of(document: &str, kind: &str) -> Vec<String> {
    let rows = body_under(document, "## Families");
    let mut out: Vec<String> = Vec::new();
    if !rows.is_empty() {
        let family_at = column_of(document, "## Families", "Family");
        let members_at = column_of(document, "## Families", "Members");
        for row in &rows {
            let members = row.get(members_at).map(String::as_str).unwrap_or_default();
            let holds = members.trim() == "every kind above"
                || members.split(',').any(|one| one.trim() == kind);
            if holds {
                out.push(plain(&row[family_at]));
            }
        }
    }
    out.push(kind.to_string());
    out
}

/// Every Recipes row that names this kind, with the recipe it belongs to and how it got there.
///
/// **Extracted so that `signature` and `section` cannot disagree.** They are the same join
/// asked two questions - *what does this say* and *what shape is it* - and two copies of a
/// matcher this fiddly would drift on the first family that changed. `R-8` is what made the
/// second caller exist.
fn recipe_rows(document: &str, kind: &str) -> Vec<(String, Vec<String>, Reach)> {
    let families_of = families_of(document, kind);

    // **By name, not by position - `C-70`.** These were `4` and `6`, which is the shape of
    // read that broke when `P-346` deleted a column from another table.
    //
    // **Asked only when there are rows**, because a document with no Recipes table has no
    // recipe rows rather than a missing column - which is what the partial documents in
    // `tests/what_a_kind_holds.rs` are. A malformed table still panics.
    let rows = rows_with_recipe(document);
    if rows.is_empty() {
        return Vec::new();
    }
    let named_at = column_of(document, "## Recipes", "Kind");
    let where_at = column_of(document, "## Recipes", "Where");

    let mut out = Vec::new();
    for (recipe, row) in rows {
        let named = row.get(named_at).map(|c| plain(c)).unwrap_or_default();
        let place = row.get(where_at).cloned().unwrap_or_default();
        let in_where = names_it(&place, kind);
        let of_mine = families_of.iter().any(|f| f == &named);
        if !of_mine && !in_where {
            continue;
        }
        let reach = if named == kind {
            Reach::Directly
        } else if in_where && !of_mine {
            Reach::AsPlace(named)
        } else {
            Reach::ViaFamily(named)
        };
        out.push((recipe, row, reach));
    }
    out
}

/// Which traits a kind carries, from `spec/data/kinds.4x` and `spec/data/traits.4x`.
///
/// **`P-473` deleted the release's *Of* column and this read it.** `R-8`'s *vetted when* says a
/// signature is *the traits it carries*, and where that is stated moved: `spec/console.md` puts
/// it on the kind - *a kind declares which traits it has*, and a trait *says nothing about
/// which kinds carry it*.
///
/// **It also understated, which is `C-108` and is why `P-473` went further than a repoint.**
/// The reader here matched a kind against the *Of* cell two ways, by word and by the cell being
/// exactly a family, and left out a cell that describes rather than names. That was one cell
/// when it was written and four by tonight, so seven of the eighteen kinds showed fewer traits
/// than the specification states - an Ark showed four and carries eight.
///
/// **Two files, because `of:thing` is in the second one.** A kind's line names the traits it
/// carries, and a trait of *every* kind is on no line at all - `P-471`: *a trait of every kind
/// is the one exception, and says so with `of:thing`*. A signature reading only `kinds.4x`
/// would lose `keeps` from all eighteen, which is the same shape of understatement one file
/// over.
///
/// **The rows returned are still the release's**, because the *Values* cell is what a reader of
/// the page sees - `moving` (a number). What moved is which rows, not where they are rendered
/// from.
fn trait_rows(document: &str, declared: &Declared, kind: &str) -> Vec<Vec<String>> {
    let carried = declared.carried_by(kind);
    body_under(document, "## Traits")
        .into_iter()
        .filter(|row| {
            row.first()
                .map(|name| carried.contains(&plain(name).replace(' ', "-")))
                .unwrap_or(false)
        })
        .collect()
}

/// What `spec/data/` declares: each kind's line, and each trait's.
///
/// **Passed in rather than read inside the reader.** `trait_rows` read the two files from disk,
/// which made it a pure function of one argument and a file - so
/// `every_kind_keeps_its_signature_when_its_name_carries_a_dash`, which rewrites the names in a
/// document and asks whether the signature survives, was comparing a renamed document against
/// unrenamed declarations. **The test was right and the reader was wrong**: a function that
/// reads a file nobody handed it cannot be driven on a document written to be different.
#[derive(Clone, Debug, Default)]
pub struct Declared {
    lines: Vec<std::collections::BTreeMap<String, String>>,
    of_every_kind: Vec<String>,
}

impl Declared {
    /// The files as they sit in `spec/data/`, which is what the catalog is generated from.
    pub fn from_spec() -> Self {
        let at = |file: &str| {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../spec/data")
                .join(file);
            std::fs::read_to_string(&path)
                .unwrap_or_else(|why| panic!("cannot read {}: {why}", path.display()))
        };
        Self::from_text(&at("kinds.4x"), &at("traits.4x"))
    }

    /// The same two files as text, so a test can supply its own.
    pub fn from_text(kinds: &str, traits: &str) -> Self {
        let lines = read_lines(kinds);
        // **`of:thing` is the one thing a trait says about which kinds carry it, and it says
        // *all*** - `spec/console.md`: *a trait of every kind is the one exception, and says
        // so with `of:thing`*. A signature reading only `kinds.4x` would lose it from every
        // kind, which is the understatement `C-108` found one file over.
        let of_every_kind = read_lines(traits)
            .into_iter()
            .filter(|line| line.get("of").map(String::as_str) == Some("thing"))
            .filter_map(|line| line.get("name").cloned())
            .collect();
        Declared {
            lines,
            of_every_kind,
        }
    }

    /// Every trait a kind carries: the bare names on its own line, and those of every kind.
    pub fn carried_by(&self, kind: &str) -> Vec<String> {
        let mut out: Vec<String> = self
            .lines
            .iter()
            .find(|line| line.get("name").map(String::as_str) == Some(kind))
            .map(|line| {
                line.iter()
                    .filter(|(_, value)| value.is_empty())
                    .map(|(name, _)| name.clone())
                    .collect()
            })
            .unwrap_or_default();
        out.extend(self.of_every_kind.iter().cloned());
        out
    }
}

/// One `spec/data/` file, as a map per line.
///
/// **A reader of one line of the notation, and deliberately not a second reader of it.**
/// `game_console::state::declarations` is the reader; this crate has no dependencies by
/// design - *read it and compile it; it does not play* - and taking one to split on whitespace
/// and a colon would cost more than it saves. **The divergence is written down here rather
/// than discovered later**, which is the half `Q-67` says was missing when one notation last
/// had two readers.
fn read_lines(text: &str) -> Vec<std::collections::BTreeMap<String, String>> {
    text.lines()
        .map(str::trim)
        .filter(|line| line.starts_with('{'))
        .map(|line| {
            line.trim_matches(|c| c == '{' || c == '}')
                .split_whitespace()
                .skip(1)
                .map(|word| match word.split_once(':') {
                    Some((name, value)) => (name.to_string(), value.to_string()),
                    None => (word.to_string(), String::new()),
                })
                .collect()
        })
        .collect()
}

/// What a kind carries and what names it, in the one form two kinds can be compared by.
///
/// **`R-8`.** The traits it carries, and every *(recipe, role)* pair that names it. Nothing
/// else: not the quantities, which differ between two kinds that behave alike, and not the
/// prose, which is a rendering. **Reaching through a family counts as naming**, because a
/// family is how the release addresses several kinds at once - `move` names a `unit`, and an
/// ark and a pioneer are both moved by it whether or not either word appears in the row.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Signature {
    /// The names of the traits, sorted.
    pub traits: Vec<String>,
    /// `recipe role`, sorted and deduplicated.
    pub pairs: Vec<String>,
}

impl Signature {
    /// The one string two kinds are equal by, which is also what the test compares.
    pub fn key(&self) -> String {
        format!("{} | {}", self.traits.join(", "), self.pairs.join(", "))
    }
}

/// One kind's signature, computed from the release's tables and written by nobody.
pub fn signature(document: &str, declared: &Declared, kind: &str) -> Signature {
    let mut traits: Vec<String> = trait_rows(document, declared, kind)
        .iter()
        .map(|row| plain(&row[0]))
        .collect();
    traits.sort();
    traits.dedup();

    let rows = recipe_rows(document, kind);
    // Same reason as `recipe_rows`: no rows means no table to ask about.
    let role_at = if rows.is_empty() {
        0
    } else {
        column_of(document, "## Recipes", "Role")
    };
    let mut pairs: Vec<String> = rows
        .iter()
        .map(|(recipe, row, _)| {
            let role = row.get(role_at).cloned().unwrap_or_default();
            format!("{recipe} {role}")
        })
        .collect();
    pairs.sort();
    pairs.dedup();

    Signature { traits, pairs }
}

/// Every signature the release produces, each with the kinds that share it.
///
/// **Grouped in order of first appearance**, so the name of a signature is stable as long as
/// the Kinds table is - and it is generated from that table rather than chosen, which is what
/// `R-8` asks for. A group of one is a kind that behaves like nothing else and is listed
/// exactly like a group of several, because *it is alone* is a finding too.
pub fn signatures(document: &str, declared: &Declared) -> Vec<(String, Signature, Vec<String>)> {
    let mut out: Vec<(String, Signature, Vec<String>)> = Vec::new();
    for row in body_under(document, "## Kinds") {
        let kind = plain(&row[0]);
        let mine = signature(document, declared, &kind);
        match out.iter_mut().find(|(_, seen, _)| seen.key() == mine.key()) {
            Some((_, _, kinds)) => kinds.push(kind),
            None => {
                let name = format!("s-{}", out.len() + 1);
                out.push((name, mine, vec![kind]));
            }
        }
    }
    out
}

/// The name of the signature a kind is in, which is the thing to grep for.
fn signature_of(document: &str, declared: &Declared, kind: &str) -> String {
    signatures(document, declared)
        .into_iter()
        .find(|(_, _, kinds)| kinds.iter().any(|k| k == kind))
        .map(|(name, _, _)| name)
        .unwrap_or_else(|| panic!("`{kind}` is in the Kinds table and in no signature"))
}

fn section(document: &str, declared: &Declared, kind: &str, what_it_is: Option<&str>) -> String {
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

    // **The cell shown is *Values*, found by name.** It was cell 2 and `P-473` deleting the
    // *Of* column made it cell 1, so every trait on the page read `(stored)` - the *Stored or
    // derived* cell, which is true of the wrong column.
    // **A heading with no table under it has no columns**, and asking for one of them panics
    // - correctly. A document with no Traits table has no trait rows either, so the index is
    // never used and the question is not asked.
    let values_at = if body_under(document, "## Traits").is_empty() {
        0
    } else {
        column_of(document, "## Traits", "Values")
    };
    let traits: Vec<String> = trait_rows(document, declared, kind)
        .iter()
        .map(|row| {
            format!(
                "`{}` ({})",
                plain(&row[0]),
                row.get(values_at).cloned().unwrap_or_default()
            )
        })
        .collect();
    if !traits.is_empty() {
        out.push_str(&format!("**Traits of it** {}\n\n", traits.join(", ")));
    }

    // **`R-8`: the name to grep for.** One word, in every section, so that finding the kinds
    // that behave like this one is a search rather than a comparison by eye.
    out.push_str(&format!(
        "**Signature** `{}`\n\n",
        signature_of(document, declared, kind)
    ));

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

    // **What it holds, and whether the number is a fact about the kind.** `S-58`: two of the
    // three rows are not per-kind constants - a unit's tank is *the unit's fuel*, which is a
    // trait of each unit, and a territory's is its total capacity for that kind. **Only the
    // store's is a fact about the kind**, which is what `S-44` settled, so one label for all
    // three would be honest about one row and misleading about two.
    //
    // The discriminator is the *Up to* cell itself: a number is a constant of the kind and
    // anything else is a phrase pointing at the thing.
    for (_, row) in containers(document).iter().filter(|(it, _)| it == kind) {
        let holds = row.get(1).cloned().unwrap_or_default();
        let up_to = row.get(2).cloned().unwrap_or_default();
        let about = if up_to.parse::<u32>().is_ok() {
            "a fact about the kind, so every one of them holds that many"
        } else {
            "a fact about each one rather than about the kind"
        };
        out.push_str(&format!("**Holds** {holds}, up to {up_to} - *{about}*\n\n"));
    }

    // Which families name this kind, so a recipe taking `place` is found under `orbit`.
    //
    // **`P-196` is why this exists.** Before it, no recipe named an orbit and the catalog
    // said so - which is what raised `C-15` and became that proposal. `move` now takes a
    // `place`, so an orbit is named by a recipe through its family, and a join matching the
    // Kind cell literally would have gone on reporting *none name it* after the thing it
    // reported had been fixed. A view that stays wrong once the world moves is worse than
    // no view.
    let mut lines: Vec<String> = Vec::new();
    for (recipe, row, reach) in recipe_rows(document, kind) {
        let place = row.get(6).cloned().unwrap_or_default();
        let via = match &reach {
            Reach::Directly => String::new(),
            Reach::AsPlace(named) => format!(" (as the place holding {named})"),
            Reach::ViaFamily(named) => format!(" (as a {named})"),
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
