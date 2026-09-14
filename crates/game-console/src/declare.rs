//! The release's declaring tables, written in the notation - `P-443`, answering `C-97`.
//!
//! **Rule 7 puts the game's data in the specification**: *state the game's data in several
//! files in a directory of their own, in the notation rather than in a table*. This is what
//! one of those files says, generated from the release so that the transcription is checked
//! rather than trusted - which is what `S-110` asked for first and what makes Sean's *go with
//! the data we have been using* a verifiable sentence.
//!
//! **`P-443` is what made it writable at all.** `C-97` asked whether a notation whose every
//! word is *a kind, a trait, or one of a trait's values* can carry a **declaration** - a file
//! that says which kinds there are is not a state of a game. The answer was that it needs no
//! second form: `kind`, `trait` and `family` are themselves kinds, so `{kind name:citizen}` is
//! an ordinary description and [`crate::state::declarations`] reads it with the one parser.
//!
//! # One table, and why not yet the other seven
//!
//! **`Kinds` is here because `P-443` wrote the example**, and nothing about its shape is this
//! lane's invention: a row is a name, and the name is the whole of what the notation carries.
//! The *What it is* column stays where rule 7 puts it - *relationships in prose* - so this
//! file is the vocabulary and the sentences remain sentences.
//!
//! **`Families` and `Traits` declare too and are not here**, because each has a cell holding
//! **several values** - a family's members, a trait's `Of` and `Values` - and the notation
//! gives one value to a key. Whether that is several lines, a joined name, or something else
//! is a decision rather than work, and it is filed rather than guessed at.

use crate::containment::Description;
use crate::recipes::{body_under, plain};

/// The three words a file of kinds needs before it can use any of them.
///
/// **`P-443` puts them in the file rather than in the release's table**, and says why: under
/// `P-440` that table becomes a copy of the data file rather than its source, so adding them
/// there would be work done twice. **A file that used `kind` without declaring it would be
/// using a word it had not introduced**, which is the rule this notation has about every other
/// word.
pub const VOCABULARY: [&str; 4] = ["kind", "trait", "family", "value"];

/// One relation of `spec/data/`: its name and its columns, in the order it writes them.
///
/// **A relation is an ordered tuple of named attributes**, and the order is the relation's own
/// rather than a global one. [`game_model::containment::Description::ordered`] ranks a game
/// thing's traits by Sean's order of relevance - the type, then the id, capacity last - and
/// **it cannot write these files.**
///
/// **Measured across the eight relations `P-497` left**, against what sorting their keys would
/// give: `carries` and `above` happen to agree, and the other six do not - `member` writes
/// `kind family`, `limit` writes `container contained n`, `block` writes `id recipe owner`,
/// and `line`, `constraint` and `for` all lead with `block seq`.
///
/// **Two of eight agreeing is worse than none**, which is the thing worth seeing: a rule that
/// held everywhere would be a rule, and a rule that holds twice looks like one until something
/// leans on it.
///
/// **This argument used to be made with one example and the example died.** It was that
/// `carries` orders `kind` before `trait` while `constraint` ordered `trait` before `kind` -
/// true until `P-511` deleted `refuel`, whose `free energy at least 1` was the only constraint
/// row carrying a kind at all. The specification lane caught it. **A finding that rests on one
/// row is a finding one promotion can take away**, and this one was not: it is about every
/// relation, and it survived losing its illustration.
///
/// **A column a row does not have is left out**, which is how `qty` is absent from a `put`
/// line and `n` from a comparison naming no number. A row carrying a column the relation does
/// not declare fails loudly, because that is a word reaching a data file by a route nothing
/// checked.
pub struct Relation {
    pub name: &'static str,
    pub columns: &'static [&'static str],
}

impl Relation {
    /// The rows as the file writes them, one per line.
    pub fn written(&self, rows: &[Vec<(&str, String)>]) -> String {
        let mut out = String::new();
        for row in rows {
            for (held, _) in row {
                assert!(
                    self.columns.contains(held),
                    "`{}` has no column `{held}`, so this row would write a word the relation \
                     does not declare",
                    self.name
                );
            }
            out.push('{');
            out.push_str(self.name);
            for column in self.columns {
                if let Some((_, value)) = row.iter().find(|(held, _)| held == column) {
                    out.push_str(&format!(" {column}:{value}"));
                }
            }
            out.push_str("}\n");
        }
        out
    }
}

/// `spec/data/block.4x`: one row per block of recipe rows.
pub const BLOCK: Relation = Relation {
    name: "block",
    columns: &["id", "recipe", "owner"],
};

/// Every block the release states, as `spec/data/block.4x` writes them.
///
/// **A block is what the release states under one name**, and a name may be stated several
/// times - `discard` five, `refresh` six - because `P-373` makes a rule whose subject is a
/// family a rule for each member. So a block needs an id the recipe name cannot give.
pub fn blocks(document: &str) -> String {
    let rows: Vec<Vec<(&str, String)>> = gathered(document)
        .into_iter()
        .map(|block| {
            vec![
                ("id", block.id),
                ("recipe", slug(&block.name)),
                ("owner", block.owner),
            ]
        })
        .collect();
    assert!(
        !rows.is_empty(),
        "the Recipes table parsed to no block at all"
    );
    BLOCK.written(&rows)
}

/// One block of the Recipes table: its id, its name, whose it is, and its rows.
pub struct Block {
    pub id: String,
    pub name: String,
    pub owner: String,
    /// The table's own cells, one `Vec` per row, in the order the table gives them.
    pub rows: Vec<Vec<String>>,
}

/// The Recipes table as blocks, each with the id `spec/data/` gives it.
///
/// # The id is derived and not chosen
///
/// **The name, slugged; then the kind, where the name repeats; then the trait, where that
/// still repeats.** `refresh` is what needs all three - six blocks over three kinds - and
/// `discard` needs two. **Qualified only as far as it has to be**, so a recipe stated once
/// keeps its own name and a reader can find it.
///
/// **Uniqueness is asserted rather than assumed.** A third qualifier would mean the release
/// states two blocks nothing tells apart, which is a defect in the release rather than
/// something to invent a suffix for.
pub fn gathered(document: &str) -> Vec<Block> {
    let table = body_under(document, "## Recipes");
    let kind_at = crate::recipes::column_of(document, "## Recipes", "Kind");
    let traits_at = crate::recipes::column_of(document, "## Recipes", "Traits");

    let mut blocks: Vec<Block> = Vec::new();
    for row in &table {
        let name = plain(row.first().map(String::as_str).unwrap_or_default());
        if !name.is_empty() {
            blocks.push(Block {
                id: String::new(),
                name,
                owner: row.get(1).cloned().unwrap_or_default(),
                rows: Vec::new(),
            });
        }
        if let Some(block) = blocks.last_mut() {
            block.rows.push(row.clone());
        }
    }

    let kind_of = |block: &Block| -> String {
        block
            .rows
            .iter()
            .find_map(|row| row.get(kind_at))
            .map(|cell| slug(&plain(cell)))
            .unwrap_or_default()
    };
    let trait_of = |block: &Block| -> Option<String> {
        block
            .rows
            .iter()
            .filter_map(|row| row.get(traits_at))
            .map(|cell| plain(cell))
            .find(|cell| !cell.is_empty())
            .and_then(|cell| cell.split_whitespace().next().map(str::to_string))
    };

    // **The qualifier is chosen per name, not per block**, so every block of a repeated name is
    // spelled the same way. `refresh` is what says so: `extractor` is unique among its six and
    // the file still writes `refresh-extractor-working`, because two of the other five need the
    // trait and a reader should not have to know which. **One rule for a name beats a shorter
    // id for one of its blocks.**
    let mut ids: Vec<String> = Vec::new();
    for block in &blocks {
        let same: Vec<&Block> = blocks.iter().filter(|it| it.name == block.name).collect();
        if same.len() == 1 {
            ids.push(slug(&block.name));
            continue;
        }
        let kinds: std::collections::BTreeSet<String> = same.iter().map(|it| kind_of(it)).collect();
        let mut id = format!("{}-{}", slug(&block.name), kind_of(block));
        if kinds.len() < same.len()
            && let Some(named) = trait_of(block)
        {
            id.push('-');
            id.push_str(&named);
        }
        ids.push(id);
    }

    let unique: std::collections::BTreeSet<&String> = ids.iter().collect();
    assert_eq!(
        unique.len(),
        ids.len(),
        "two blocks share an id, so the release states two nothing tells apart: {ids:?}"
    );
    for (block, id) in blocks.iter_mut().zip(ids) {
        block.id = id;
    }
    blocks
}

/// `spec/data/line.4x`: one row per line of a recipe.
pub const LINE: Relation = Relation {
    name: "line",
    columns: &[
        "block",
        "seq",
        "role",
        "qty",
        "kind",
        "place-bound",
        "place-above",
    ],
};

/// `spec/data/constraint.4x`: what a line's `Traits` cell says about a count.
pub const CONSTRAINT: Relation = Relation {
    name: "constraint",
    columns: &["block", "seq", "trait", "compare", "n"],
};

/// `spec/data/for.4x`: which kind a line's `Traits` cell names, where it names one.
pub const FOR: Relation = Relation {
    name: "for",
    columns: &["block", "seq", "kind"],
};

/// The three quantities that are a sentence rather than a number.
///
/// **Named, because the same check one column over was what let them through.** `P-497`
/// asserted that exactly one cell of the release could not be represented - `move`'s adjacency
/// qualifier, below - and named it so a second would fail the run. **That assertion read the
/// Traits column and the quantities went past unclassified**, so three unrepresentable cells
/// passed a check built to catch exactly that.
///
/// **This generator had the same hole.** `lines` writes `qty` verbatim, which reproduces the
/// file faithfully and says nothing about what it is reproducing. A fourth sentence-quantity
/// would have been written as silently as these three.
///
/// `C-120` is the item, and `P-514` offers a shape for two of them. The third -
/// `` `$where`'s density for that resource `` - is a density indexed by which resource the
/// block is for rather than a trait of a row, and that proposal says outright that it does not
/// fix it.
const NOT_A_NUMBER: [&str; 3] = [
    "`$where`'s density for that resource",
    "that citizen's strength",
    "that unit's strength",
];

/// The one *Traits* cell `spec/data/` does not represent.
///
/// **Named rather than skipped**, and `S-131` names it too: *joined to `$from` by an edge the
/// unit crosses* is a relation between two places and the notation has no form for it. A
/// second cell arriving here would be a second thing the data cannot say, so this is asserted
/// to be the only one rather than filtered quietly.
const NOT_REPRESENTED: &str = "joined to `$from` by an edge the unit crosses";

/// Every line the release states, as `spec/data/line.4x` writes them.
pub fn lines(document: &str) -> String {
    let mut rows: Vec<Vec<(&str, String)>> = Vec::new();
    for (block, seq, row) in numbered(document) {
        let mut line = vec![
            ("block", block.clone()),
            ("seq", seq.to_string()),
            ("role", cell(&row, document, "Role")),
        ];
        let qty = cell(&row, document, "Qty");
        if !qty.is_empty() {
            // **A quantity is a number or one of the three the notation cannot hold**, and a
            // fourth fails here rather than being written as though it were a number. See
            // [`NOT_A_NUMBER`] for why this guard exists one column over from the one that
            // did not catch these.
            assert!(
                qty.chars().all(|c| c.is_ascii_digit()) || NOT_A_NUMBER.contains(&qty.as_str()),
                "`{qty}` is neither a number nor one of the three quantities `C-120` names"
            );
            line.push(("qty", qty));
        }
        line.push(("kind", cell(&row, document, "Kind")));
        if let Some((column, named)) = place(&cell(&row, document, "Where")) {
            line.push((column, named));
        }
        rows.push(line);
    }
    assert!(
        !rows.is_empty(),
        "the Recipes table parsed to no line at all"
    );
    LINE.written(&rows)
}

/// Every constraint the release states, as `spec/data/constraint.4x` writes them.
pub fn constraints(document: &str) -> String {
    let mut rows: Vec<Vec<(&str, String)>> = Vec::new();
    let mut unrepresented = 0;
    for (block, seq, row) in numbered(document) {
        let said = cell(&row, document, "Traits");
        if said.is_empty() || is_a_kind(&said) {
            continue;
        }
        if said == NOT_REPRESENTED {
            unrepresented += 1;
            continue;
        }
        let (named, compare, n) = comparison(&said);
        let mut line = vec![
            ("block", block.clone()),
            ("seq", seq.to_string()),
            ("trait", named),
            ("compare", compare),
        ];
        if let Some(n) = n {
            line.push(("n", n));
        }
        rows.push(line);
    }
    assert_eq!(
        unrepresented, 1,
        "one `Traits` cell has no form in `spec/data/` - `S-131` names it - and this found \
         {unrepresented}"
    );
    assert!(
        !rows.is_empty(),
        "the Recipes table parsed to no constraint"
    );
    CONSTRAINT.written(&rows)
}

/// Every `for` the release states, as `spec/data/for.4x` writes them.
pub fn fors(document: &str) -> String {
    let rows: Vec<Vec<(&str, String)>> = numbered(document)
        .into_iter()
        .filter_map(|(block, seq, row)| {
            let said = cell(&row, document, "Traits");
            is_a_kind(&said).then(|| {
                vec![
                    ("block", block),
                    ("seq", seq.to_string()),
                    ("kind", said.trim_matches('`').to_string()),
                ]
            })
        })
        .collect();
    assert!(!rows.is_empty(), "the Recipes table parsed to no `for` row");
    FOR.written(&rows)
}

/// Every row of the Recipes table, with the block it belongs to and its place in that block.
///
/// **`seq` counts within the block and from one**, which is what makes a constraint and its
/// line the same row seen twice - the pair `(block, seq)` is how `spec/data/` joins them, and
/// it is the natural key `P-497` gave those relations.
fn numbered(document: &str) -> Vec<(String, usize, Vec<String>)> {
    let mut out = Vec::new();
    for block in gathered(document) {
        for (at, row) in block.rows.iter().enumerate() {
            out.push((block.id.clone(), at + 1, row.clone()));
        }
    }
    out
}

/// One cell of a row, by the name of its column.
fn cell(row: &[String], document: &str, column: &str) -> String {
    let at = crate::recipes::column_of(document, "## Recipes", column);
    plain(row.get(at).map(String::as_str).unwrap_or_default())
}

/// Whether a *Traits* cell names a kind rather than constraining a count.
///
/// **A kind is one word and a comparison is several**, and the one-word cells are `food`,
/// `metal` and `` `$resource` `` - the resource a `build` or a landing's extractor is for.
fn is_a_kind(said: &str) -> bool {
    !said.is_empty() && !said.contains(char::is_whitespace)
}

/// A *Traits* cell as a trait, a comparison and the number it names.
///
/// **Four forms and no others**, which the release's own vocabulary bears out: *at its
/// maximum*, *at least n*, *one less*, and a bare number. A fifth fails here rather than
/// being written as something it is not.
fn comparison(said: &str) -> (String, String, Option<String>) {
    let (named, rest) = said
        .split_once(' ')
        .unwrap_or_else(|| panic!("`{said}` names no trait"));
    let named = named.trim_matches('`').to_string();
    match rest {
        "at its maximum" => (named, "at-maximum".to_string(), None),
        "one less" => (named, "one-less".to_string(), None),
        _ => match rest.strip_prefix("at least ") {
            Some(n) => (named, "at-least".to_string(), Some(n.to_string())),
            None => {
                assert!(
                    rest.chars().all(|c| c.is_ascii_digit()),
                    "`{said}` is not `at its maximum`, `one less`, `at least n` or a number"
                );
                (named, "exactly".to_string(), Some(rest.to_string()))
            }
        },
    }
}

/// A *Where* cell as the column and the name it binds.
///
/// **Two columns, because a place named and a place worked out from one are different
/// facts.** `$where` is bound by the command; *above `$where`* is the orbit the release says
/// is derived - `spec/console.md`: *a place worked out from another is not open*.
fn place(said: &str) -> Option<(&'static str, String)> {
    let said = said.trim();
    if said.is_empty() {
        return None;
    }
    if let Some(rest) = said.strip_prefix("above ") {
        return Some((
            "place-above",
            rest.trim_matches('`').trim_start_matches('$').to_string(),
        ));
    }
    Some((
        "place-bound",
        said.trim_matches('`').trim_start_matches('$').to_string(),
    ))
}

/// A name as a data file spells it: one word, dashes for the spaces.
fn slug(name: &str) -> String {
    name.replace(' ', "-")
}

/// Every kind the release declares, as the file that would declare them.
///
/// **Read from the release's *Kinds* table**, so this is the same data by a different route
/// rather than a second copy of it - which is the whole of why it can be compared.
pub fn kinds(document: &str) -> String {
    let mut rows: Vec<Description> = Vec::new();
    for name in VOCABULARY {
        rows.push(named("kind", name, None));
    }
    for name in every_kind(document) {
        rows.push(named("kind", &name, None));
    }
    crate::state::declared(&rows)
}

/// Which family each kind is in, one line per membership - `spec/data/member.4x`.
///
/// **`P-497` took this off the kind's own line**, and the reason is the shape rather than the
/// tidiness: a kind's line carried a repeating group - its family and every trait it carries -
/// and a repeating group is the one thing a relation may not hold. `{kind name:pioneer
/// family:unit binding defending fuel ...}` is now `{kind name:pioneer}` and one row per fact.
///
/// **Membership is a list and a key takes one value**, which is what made the old form
/// fragile: a kind in two families had nowhere to put the second and this generator refused
/// rather than picking. **One row per membership cannot have that problem at all** - which is
/// the normalization doing what it is for, rather than a check being added.
pub fn members(document: &str) -> String {
    let families = families_of(document);
    // **In the Kinds table's order, not the Families table's.** A relation has no order and a
    // file does, so the file needs one that is derivable rather than chosen - and reading the
    // kinds in the order they are declared gives the same bytes from the same document.
    let rows: Vec<Description> = every_kind(document)
        .into_iter()
        .filter_map(|kind| {
            families.get(&kind).map(|family| {
                let mut traits = std::collections::BTreeMap::new();
                traits.insert("kind".to_string(), kind.clone());
                traits.insert("family".to_string(), family.clone());
                Description {
                    kind: "member",
                    traits,
                }
            })
        })
        .collect();
    assert!(
        !rows.is_empty(),
        "no kind declares a family, so `member.4x` would be empty and say nothing about it"
    );
    crate::state::declared(&rows)
}

/// Every kind the release declares, in the order its table gives them.
fn every_kind(document: &str) -> Vec<String> {
    body_under(document, "## Kinds")
        .iter()
        .map(|row| plain(row.first().map(String::as_str).unwrap_or_default()))
        .inspect(|name| {
            assert!(
                !name.is_empty(),
                "a row of the Kinds table names no kind, so the file would declare a blank word"
            )
        })
        .collect()
}

/// One line: a declaration of `what`, named `name`, optionally in a family.
///
/// **Built as the struct rather than through `Description::of`**, which takes a
/// `game_model::thing::Kind` - and `kind` is not one of the model's kinds. That is the
/// distinction `S-112` warned about before this file was written: `citizen` can be in a game
/// state and `kind` cannot, so a declaration names a word the model has no variant for. The
/// model is right not to have one; nothing in a game is ever a `kind`.
fn named(what: &'static str, name: &str, family: Option<&str>) -> Description {
    let mut traits = std::collections::BTreeMap::new();
    traits.insert("name".to_string(), name.to_string());
    if let Some(family) = family {
        traits.insert("family".to_string(), family.to_string());
    }
    Description { kind: what, traits }
}

/// Which family each kind declares, from the *Families* table read backwards.
///
/// **`P-448` inverted the declaration**: a kind declares which family it is in, and a family
/// declares only its name. So the table's `Members` cell, which held a list and is what `C-98`
/// stopped on, is read here and written one value to a line - `{kind name:ark family:unit}`.
///
/// **`thing` is skipped and writes nothing.** `spec/console.md`: *`thing` is the family every
/// kind is in, and no line says so kind by kind - a kind added tomorrow is a `thing` because it
/// is a kind.* The table spells it *every kind above*, which is a rule about the table rather
/// than a list, and a rule belongs in prose where rule 7 puts it.
///
/// **A kind in two families would have nowhere to put the second**, because a key takes one
/// value - so this refuses rather than picking. It cannot happen in this release and the
/// release is not what makes it true.
fn families_of(document: &str) -> std::collections::BTreeMap<String, String> {
    let mut out: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    for row in body_under(document, "## Families") {
        let family = plain(row.first().map(String::as_str).unwrap_or_default());
        let members = row.get(1).cloned().unwrap_or_default();
        if family.is_empty() || members.contains("every kind above") {
            continue;
        }
        for member in members.split(',') {
            let member = plain(member);
            if member.is_empty() {
                continue;
            }
            if let Some(already) = out.insert(member.clone(), family.clone()) {
                panic!(
                    "`{member}` is in `{already}` and in `{family}`, and a kind's line has one \
                     `family` to give - which family it declares would be a choice this file \
                     is not entitled to make"
                );
            }
        }
    }
    assert!(
        !out.is_empty(),
        "no kind is in any family, so the Families table parsed to nothing and every kind \
         would be written as a `thing` alone"
    );
    out
}

/// Every family the release declares, as the file that would declare them.
///
/// **A name and nothing else** - `P-448`. What a family contains is on its members' lines, so
/// this file is the shortest of the three and is the whole of what a family declares.
pub fn families(document: &str) -> String {
    let mut rows: Vec<Description> = Vec::new();
    for row in body_under(document, "## Families") {
        let name = plain(row.first().map(String::as_str).unwrap_or_default());
        assert!(
            !name.is_empty(),
            "a row of the Families table names no family, so the file would declare a blank \
             word"
        );
        rows.push(named("family", &name, None));
    }
    assert!(
        !rows.is_empty(),
        "the Families table parsed to nothing, so this would write an empty file and the \
         comparison against it would agree for the wrong reason"
    );
    crate::state::declared(&rows)
}

/// Every biome the release declares, as the file that would declare them.
///
/// **A biome is a value rather than a kind** - `P-451`: *a value declares which trait it is one
/// of*. So each line is a `value`, named, `of:biome`, and `value` is the fourth declaring kind
/// beside `kind`, `trait` and `family`.
///
/// **`nature` rides on the value's line**, because a biome's force of nature is a fact about
/// the biome - `P-454`'s fourth sentence, and the release says which column that is: **the
/// numbers here guide and do not bind; a territory's own are in *Territory resources*. Force
/// of nature is the one column that binds.**
///
/// **So the three resource columns are not written here and that is the release's own
/// sentence rather than a choice made in this file.** `5 x 6` is advice to whoever picks a
/// territory's numbers - two numbers in one cell, guiding - and what binds is in *Territory
/// resources*, which is **this planet's** and not the game's. `C-97` is where that
/// distinction was measured.
///
/// **Ocean carries no `nature`**, because the release says it *is not claimable and carries
/// nothing* and its every cell is `-`. A line with a name and its trait is the whole of what
/// is true of it.
pub fn biomes(document: &str) -> String {
    let mut rows: Vec<Description> = Vec::new();
    for row in body_under(document, "## Biomes") {
        let name = plain(row.first().map(String::as_str).unwrap_or_default()).to_lowercase();
        assert!(
            !name.is_empty(),
            "a row of the Biomes table names no biome, so the file would declare a blank word"
        );
        let mut traits = std::collections::BTreeMap::new();
        traits.insert("name".to_string(), name);
        traits.insert("of".to_string(), "biome".to_string());
        let nature = row.get(4).map(String::as_str).unwrap_or_default().trim();
        if nature != "-" && !nature.is_empty() {
            nature.parse::<u32>().unwrap_or_else(|_| {
                panic!("`{nature}` is a force of nature and is not a number, so the line would carry a word the notation has no place for")
            });
            traits.insert("nature".to_string(), nature.to_string());
        }
        rows.push(Description {
            kind: "value",
            traits,
        });
    }
    assert!(
        !rows.is_empty(),
        "the Biomes table parsed to nothing, so this would write an empty file and a \
         comparison against it would agree for the wrong reason"
    );
    crate::state::declared(&rows)
}

/// Every trait the release declares that a data file needs, as the file that would declare them.
///
/// **`P-451`: a trait says what it admits and whether it is stored, and says nothing about
/// which kinds carry it.** So the *Of* column is not here - it is inverted onto the kinds -
/// and each line is the trait's name, what it admits, and how it is kept.
///
/// **The two keys are `admits` and `kept`, and they are the specification lane's proposal
/// rather than a promoted rule** - `P-457`, open to Sean, and `C-100` is where this lane said
/// the shape was not its to choose. `admits` is the release's own verb: *where a trait admits
/// a closed set of values*. `kept` is not `held`, because in this game holding is containment
/// and a store holds metal. **If either name changes, this function changes and nothing else
/// does.**
///
/// # What `admits` says, by what the Values cell is
///
/// **A number, however the cell describes what it counts.** *A number* five times, and four
/// more that are a number with a sentence about what it counts - *how much energy its tank
/// holds*, *food per turn*, *the number of turns it will last*, and `id`'s *unique among
/// things of its kind*. The sentence is a relationship and rule 7 leaves it in prose.
///
/// **Or the name of whatever already declares the values.** A family where they are kinds -
/// `one of the resources` is the `resource` family, `a place` is the `place` family - and the
/// trait's own name where they are values, because `{value name:ice of:biome}` says it there.
///
/// **Or `???`, which is the one open cell.** `0 or 1` five times and `yes or no` three times
/// are the same two-valued set spelled twice, and what the notation calls it is `P-457`'s
/// question. **Written as a word that cannot be mistaken for an answer**, so a reader of the
/// generated file cannot take it for one.
///
/// # Which traits are here
///
/// **Twenty-one of twenty-three.** A derived trait that a recipe names must be declared, since
/// every word in a data file is a kind, a trait or one of a trait's values - and `surplus` and
/// `unpaid` are named once each. **`metal in it` and `control` are named by nothing**, counted
/// over the *Recipes* table, so they are in no data file at all.
pub fn traits(document: &str) -> String {
    // **Found by name, because `P-476` renamed one of these and moved the other.** *Stored or
    // derived* is *Belongs to*, and it says where a value belongs rather than whether one is
    // held - `spec/console.md`: *it says where a value belongs and never whether one is held,
    // which is the layout and has one reader.*
    let values_at = crate::recipes::column_of(document, "## Traits", "Values");
    let belongs_at = crate::recipes::column_of(document, "## Traits", "Belongs to");
    let mut rows: Vec<Description> = Vec::new();
    for row in body_under(document, "## Traits") {
        let name = plain(row.first().map(String::as_str).unwrap_or_default()).replace(' ', "-");
        let values = plain(row.get(values_at).map(String::as_str).unwrap_or_default());
        let belongs = plain(row.get(belongs_at).map(String::as_str).unwrap_or_default());
        assert!(
            !name.is_empty() && !values.is_empty() && !belongs.is_empty(),
            "a row of the Traits table is missing a cell, so the line would be short a fact"
        );
        // **Two where there were three.** `nothing` is gone with the distinction between a
        // stored trait and a derived one: a derived trait's value still belongs somewhere,
        // and `P-477` moved the guarantee about deriving to where the one writer is stated.
        let kept = match belongs.as_str() {
            "each thing" => "thing",
            "the kind" => "kind",
            other => panic!(
                "`{name}` belongs to `{other}`, and the release has two: each thing, the kind"
            ),
        };
        let mut traits = std::collections::BTreeMap::new();
        traits.insert("name".to_string(), name);
        traits.insert("admits".to_string(), admits(&values));
        traits.insert("kept".to_string(), kept.to_string());
        // **One fact `spec/data/traits.4x` states is not in the release any more.**
        // `spec/console.md`: *a trait of every kind is the one exception, and says so with
        // `of:thing`* - because there is no kind for it to belong to and no family that could
        // hold it. It was read from the *Of* cell saying `thing`, and `P-473` deleted that
        // column.
        //
        // **So this writes twenty-three of the file's twenty-four lines and says so**, rather
        // than inventing the twenty-fourth. Which trait is of every kind is not derivable from
        // what is left of the release, and a generator that guessed would be the second source
        // `P-469` forbids. `the_traits_file_declares_what_a_data_file_needs` asserts the one
        // difference by name.
        rows.push(Description {
            kind: "trait",
            traits,
        });
    }
    assert!(
        rows.len() > 15,
        "only {} traits, so the table parsed to nearly nothing",
        rows.len()
    );
    crate::state::declared(&rows)
}

/// What a trait admits, read from its Values cell.
///
/// **Four things a trait may admit**, and the release's own cell says which.
///
/// **A number**, however the cell describes what it counts. `a number` outright; `0 or 1` and
/// `yes or no`, which are a maximum showing through rather than the trait's shape - `P-457`,
/// and `spec/turn.md` is why: *each kind declares how many of each action a thing of it may
/// take in a turn*; and four cells that are a number with a sentence about what it counts.
///
/// **An identity**, which is `id` alone. `P-462`: two are equal or they are not, nothing
/// orders or sums or aggregates one, and a guard compares with `=`. Sean's reason is the third
/// test in *When a primitive earns its place* and it outranks the two that were there -
/// *unifying it would create a lie*. `max id of {territory}` was well-formed and meaningless.
///
/// **A family**, where the values are kinds and the family already declares them -
/// `one of the resources` is `resource`, `a place` is `place`.
///
/// **`value`**, where they are declared values rather than kinds. Not the trait's own name:
/// `{value name:ice of:biome}` already says which set it is, so repeating it on the trait's
/// line would be a rule stated twice - `P-458`, and the specification lane applied it to its
/// own draft.
fn admits(values: &str) -> String {
    // **A *Values* cell may carry its derivation after a colon** since `P-476` - *a number:
    // its capacity less what it holds*. What a trait admits is the part before it; how a
    // value is arrived at is not something a data file says.
    let said = values
        .split_once(": ")
        .map_or(values, |(what, _)| what)
        .trim();
    if said == "an identity" {
        return "identity".to_string();
    }
    if said == "one of the biomes" || said == "design or play" {
        return "value".to_string();
    }
    if let Some(rest) = said.strip_prefix("one of the ") {
        return rest.trim_end_matches('s').to_string();
    }
    if said == "a place" {
        return "place".to_string();
    }
    // **One cell is read rather than derived, and it is named here so it cannot be forgotten.**
    // `control` says *held by a player, or unclaimed*, which describes a closed set of two -
    // and `spec/data/traits.4x` says `admits:number`. `P-457` landed *nothing in the game is
    // two-valued anywhere* and `P-465` corrected eight cells and two; this one states a range
    // in a sentence rather than in the `0 or 1` and `yes or no` those two had, so it was not
    // in the list and is still stating one.
    //
    // **This matches the file rather than deriving from the cell**, which is a reading, and
    // `C-106` is where it is filed. **Keyed on the exact words, so the day the cell changes
    // this stops matching and the assertion below says why** - a named exception that cannot
    // outlive its excuse, which is `C-61`'s shape.
    if said == "held by a player, or unclaimed" {
        return "number".to_string();
    }
    assert!(
        said == "0 or 1"
            || said == "yes or no"
            || said.contains("number")
            || said.contains("how much")
            || said.contains("per turn"),
        "`{said}` is a Values cell this does not read, and guessing at it would put a word in \
         the file that the release did not say"
    );
    "number".to_string()
}
