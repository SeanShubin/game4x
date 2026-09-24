//! `spec/data/` as relations, one table each.
//!
//! # Why this exists
//!
//! **`S-135`, and Sean's own words are the whole of the case.** `P-497` normalized `spec/data/`
//! on his argument that *I find normalized data a lot easier to review because it makes the
//! relationships apparent and has less duplication* - and then he met the rows and could not
//! place them: *I am less familiar with "line", "block", and "constraint". Are these data rows
//! rather than commands?*
//!
//! **The rules had a source and a denormalized presentation and nothing in between.** The
//! source is twelve files of `{...}` rows; the presentation is
//! `releases/first-release.md` -> Recipes, one wide table with blank continuation cells. **The
//! relational model had no rendering at all**, which is what this is.
//!
//! **`reports/state.md` already solves the same problem for a different subject** - one table
//! per relation over the state - so this is that, over the rules.
//!
//! # What it reads and what it refuses
//!
//! **The relation is the row's opening word and is not the file name.** `biomes.4x` holds
//! `value`, `families.4x` holds `family`, `kinds.4x` holds `kind` and `traits.4x` holds
//! `trait`. **Naming the table after the file would have named four of twelve wrongly**, so the
//! table is named by what the rows say and the file is a column of the index.
//!
//! **One relation per file, asserted rather than assumed.** A file holding two would produce a
//! table whose columns are the union of two different shapes, which reads as one relation with
//! a lot of blanks - the exact thing a normalized view exists to stop.
//!
//! **A row does not have to carry every column.** Eighteen `constraint` rows come in two
//! shapes and sixty-eight `line` rows in six, because `n`, `place-bound` and `place-above` are
//! written only where they say something. The union is taken in the order the columns are
//! first seen and a row without a column leaves the cell empty.
//!
//! **A row is a map, so within one row the order is the map's.** What that gives is every
//! column a first row carries, alphabetically, then each later shape's new columns after them -
//! stable, and deliberately not the order the file writes them in. **The file's order is not
//! recoverable from a relation**, because six shapes of `line` do not agree on one.

use std::collections::BTreeMap;
use std::path::Path;

/// One relation, with every row of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    /// The word every row of it opens with.
    pub name: String,
    /// The file it was read from, as a path under the repository root.
    pub file: String,
    /// Every key any row carries, in the order they were first seen.
    pub columns: Vec<String>,
    /// One entry per row, keyed by column; a column a row does not carry is absent.
    pub rows: Vec<BTreeMap<String, String>>,
}

impl Relation {
    /// A row's cells in column order, with an absent one written as nothing.
    pub fn cells(&self, row: &BTreeMap<String, String>) -> Vec<String> {
        self.columns
            .iter()
            .map(|column| row.get(column).cloned().unwrap_or_default())
            .collect()
    }
}

/// What a file said that this reader would rather report than guess at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spilled {
    pub file: String,
    pub line: usize,
    pub key: String,
    pub value: String,
}

/// Every relation in a directory of `.4x` files, and every value that spilled its token.
///
/// **A value runs to the next `key:` token**, which is the reading that makes
/// `{line ... qty:density for that resource ...}` mean what it says. **`spec/console.md` gives
/// a field one token** and `C-120` is that a quantity in `line.4x` is a sentence - so this
/// reads it rather than refusing it, and hands back every one it read that way so a caller can
/// count them. **The item said three and says one now**: `P-522` cut `muster` and `stand` with
/// the force rule, and writing this reader is what found that its number had not followed.
///
/// **Reported rather than refused, and that is a choice.** Refusing would make a report
/// generator the thing that enforces the notation, and the gate would go red on a file this
/// lane does not own. Counting them keeps the defect visible and keeps it `C-120`'s.
pub fn read(directory: &Path) -> (Vec<Relation>, Vec<Spilled>) {
    let mut found: Vec<Relation> = Vec::new();
    let mut spilled: Vec<Spilled> = Vec::new();

    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(directory)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", directory.display()))
        .map(|entry| entry.expect("a readable entry").path())
        .filter(|path| path.extension().is_some_and(|it| it == "4x"))
        .collect();
    files.sort();

    for path in files {
        let shown = format!(
            "spec/data/{}",
            path.file_name().expect("a file").to_string_lossy()
        );
        let text = std::fs::read_to_string(&path).expect("a readable data file");
        let mut relation: Option<Relation> = None;
        for (at, line) in text.lines().enumerate() {
            let line = line.trim();
            if !line.starts_with('{') {
                continue;
            }
            let inner = line
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .to_string();
            let mut words = inner.split_whitespace();
            let Some(name) = words.next() else {
                continue;
            };
            let row = fields(words, &shown, at + 1, &mut spilled);

            let relation = relation.get_or_insert_with(|| Relation {
                name: name.to_string(),
                file: shown.clone(),
                columns: Vec::new(),
                rows: Vec::new(),
            });
            assert_eq!(
                relation.name,
                name,
                "{shown} line {} opens with `{name}` where the file's first row opened with \
                 `{}` - a file holding two relations renders as one with a lot of blanks, \
                 which is what a normalized view exists to stop",
                at + 1,
                relation.name
            );
            for column in row.keys() {
                if !relation.columns.contains(column) {
                    relation.columns.push(column.clone());
                }
            }
            relation.rows.push(row);
        }
        if let Some(relation) = relation {
            found.push(relation);
        }
    }
    (found, spilled)
}

/// One row's fields, with a value running to the next `key:` token.
fn fields<'a>(
    words: impl Iterator<Item = &'a str>,
    file: &str,
    line: usize,
    spilled: &mut Vec<Spilled>,
) -> BTreeMap<String, String> {
    let mut row: BTreeMap<String, String> = BTreeMap::new();
    let mut open: Option<(String, String)> = None;
    let mut shut = |open: &mut Option<(String, String)>, row: &mut BTreeMap<String, String>| {
        if let Some((key, value)) = open.take() {
            if value.contains(' ') {
                spilled.push(Spilled {
                    file: file.to_string(),
                    line,
                    key: key.clone(),
                    value: value.clone(),
                });
            }
            row.insert(key, value);
        }
    };
    for word in words {
        match word.split_once(':') {
            Some((key, value)) => {
                shut(&mut open, &mut row);
                open = Some((key.to_string(), value.to_string()));
            }
            None => match open.as_mut() {
                Some((_, value)) => {
                    value.push(' ');
                    value.push_str(word);
                }
                // A bare word before any field is a trait named without a value, which the
                // notation allows on a kind's line. It becomes a column with an empty cell.
                None => {
                    row.insert(word.to_string(), String::new());
                }
            },
        }
    }
    shut(&mut open, &mut row);
    row
}

/// The relational model as [`crate::dump::html`] takes it.
pub fn sections(found: &[Relation]) -> Vec<crate::dump::Section> {
    found
        .iter()
        .map(|relation| crate::dump::Section {
            name: format!("{} - {}", relation.name, relation.file),
            columns: relation.columns.clone(),
            rows: relation
                .rows
                .iter()
                .map(|row| relation.cells(row))
                .collect(),
        })
        .collect()
}

/// The relational model as markdown, one table per relation.
pub fn markdown(found: &[Relation], spilled: &[Spilled]) -> String {
    let mut out = String::from("# The rules, as relations\n\n");
    out.push_str(
        "**Generated. Do not edit.** One table per relation in `spec/data/`, which is where the\n\
         rules are stated. `reports/state.md` is the same view over the state the scenario\n\
         leaves; this is the same view over the rules it played by.\n\n\
         **The relation is the word each row opens with, and it is not always the file name** -\n\
         `biomes.4x` holds `value`, `families.4x` holds `family`, `kinds.4x` holds `kind` and\n\
         `traits.4x` holds `trait`.\n\n",
    );
    out.push_str(&format!(
        "{} relations, {} rows.\n\n",
        found.len(),
        found.iter().map(|it| it.rows.len()).sum::<usize>()
    ));

    if !spilled.is_empty() {
        out.push_str(
            "## Values that are sentences\n\n\
             `spec/console.md` gives a field one token and these carry several, so a reader has\n\
             to take the words up to the next `key:` as one value. **This is `C-120` and it is\n\
             shown rather than corrected here** - the data is not this report's to edit.\n\n",
        );
        let columns = [
            "Where".to_string(),
            "Field".to_string(),
            "Value".to_string(),
        ];
        let rows: Vec<Vec<String>> = spilled
            .iter()
            .map(|it| {
                vec![
                    format!("`{}` line {}", it.file, it.line),
                    format!("`{}`", it.key),
                    it.value.clone(),
                ]
            })
            .collect();
        out.push_str(&crate::dump::padded_rows(&columns, &rows));
        out.push('\n');
    }

    for relation in found {
        out.push_str(&format!("## {}\n\n", relation.name));
        out.push_str(&format!(
            "From `{}`. {} row(s), {} column(s).\n\n",
            relation.file,
            relation.rows.len(),
            relation.columns.len()
        ));
        // **Written as `tools/pad-tables` would leave it**, which `CLAUDE.md` requires of a
        // generated file: padding it has to change nothing, and `dump::padded_rows` is the
        // function `state.md` already writes through.
        let rows: Vec<Vec<String>> = relation
            .rows
            .iter()
            .map(|row| relation.cells(row))
            .collect();
        out.push_str(&crate::dump::padded_rows(&relation.columns, &rows));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data")
    }

    /// Every file in `spec/data/` becomes exactly one relation, named by its rows.
    ///
    /// # The check that would have failed before this
    ///
    /// **Nothing rendered `spec/data/` as relations at all** - `S-135` - so nothing asked
    /// whether a file holds one relation, and nothing would have noticed a second creeping in.
    ///
    /// **Over the directory rather than over a list**, because a list of twelve names is a copy
    /// of the population and `docs/process.md` says a check that reads a copy is checking the
    /// copy. The count is asserted so that a read returning nothing cannot pass.
    ///
    /// **And the four whose relation is not their file name are named**, because that is the
    /// case a reader would otherwise assume away.
    #[test]
    fn every_file_holds_one_relation_named_by_its_rows() {
        let (found, _) = read(&data());
        let files: usize = std::fs::read_dir(data())
            .expect("spec/data exists")
            .filter(|entry| {
                entry
                    .as_ref()
                    .is_ok_and(|it| it.path().extension().is_some_and(|it| it == "4x"))
            })
            .count();
        assert!(
            files > 0,
            "no data files, so finding no defect means nothing"
        );
        assert_eq!(
            found.len(),
            files,
            "one relation per file, and `read` asserts the other direction"
        );
        assert!(
            found.iter().all(|it| !it.rows.is_empty()),
            "a relation with no rows: {:?}",
            found
                .iter()
                .filter(|it| it.rows.is_empty())
                .map(|it| &it.file)
                .collect::<Vec<_>>()
        );

        let differing: Vec<(&str, &str)> = found
            .iter()
            .filter(|it| {
                let stem = it
                    .file
                    .trim_start_matches("spec/data/")
                    .trim_end_matches(".4x");
                stem != it.name
            })
            .map(|it| (it.file.as_str(), it.name.as_str()))
            .collect();
        assert_eq!(
            differing,
            [
                ("spec/data/biomes.4x", "value"),
                ("spec/data/families.4x", "family"),
                ("spec/data/kinds.4x", "kind"),
                ("spec/data/traits.4x", "trait"),
            ],
            "four file names are plural and their relations are not - naming a table after its \
             file would name these four wrongly"
        );
    }

    /// A row that does not carry every column leaves that cell empty rather than shifting.
    ///
    /// **`line` is the case, and it is the one a reader most needs right.** Sixty-eight rows in
    /// six shapes: `place-bound` and `place-above` appear on ten of them and `qty` is absent
    /// from eleven. **A reader that packed cells left** would put a `kind` under `qty` and the
    /// table would be wrong in a way that reads as data.
    #[test]
    fn a_row_without_a_column_leaves_the_cell_empty() {
        let (found, _) = read(&data());
        let line = found
            .iter()
            .find(|it| it.name == "line")
            .expect("`line` is a relation");
        assert!(
            line.columns.len() > 5,
            "`line` has {} columns, which is too few for a row to be missing one",
            line.columns.len()
        );
        let short = line
            .rows
            .iter()
            .filter(|row| row.len() < line.columns.len())
            .count();
        assert!(
            short > 0,
            "no `line` row is missing a column, so this checks nothing"
        );
        for row in &line.rows {
            let cells = line.cells(row);
            assert_eq!(cells.len(), line.columns.len(), "every row is as wide");
            for (at, column) in line.columns.iter().enumerate() {
                match row.get(column) {
                    Some(value) => assert_eq!(&cells[at], value, "`{column}` is in its column"),
                    None => assert!(cells[at].is_empty(), "`{column}` is absent and blank"),
                }
            }
        }
    }

    /// A value of several words is read whole and reported, rather than split or refused.
    ///
    /// **One, and `C-120`'s title says three.** `spec/console.md` gives a field one token and
    /// a quantity the release states as a phrase carries several. The item was raised over
    /// three - `work`, `muster` and `stand` - and **`P-522` cut `muster` and `stand` with the
    /// force rule**, so what is left is `work`'s `` `$where`'s density for that resource ``.
    /// `every_bare_word_in_every_data_file_is_a_declared_trait` already counts four bare words
    /// where it counted eight, and says `P-522`; **the item is what had not followed**, and
    /// correcting it is part of this commit.
    ///
    /// **The count is asserted in both directions** so this stops being an accepted shape the
    /// moment the data is repaired: a reader that went on silently joining words would hide a
    /// real typo once the known one is gone.
    #[test]
    fn a_value_of_several_words_is_read_whole_and_counted() {
        let (_, spilled) = read(&data());
        let where_from: Vec<String> = spilled
            .iter()
            .map(|it| format!("{} {}:{}", it.file, it.key, it.value))
            .collect();
        assert_eq!(
            where_from,
            ["spec/data/line.4x qty:`$where`'s density for that resource"],
            "one quantity in `spec/data/` is a sentence - `C-120`, corrected from three when \
             `P-522` cut `muster` and `stand`"
        );
    }
}
