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
//! **`reports/state.md` already solves the same problem for a different subject** - one table
//! per relation over the state - so this is that, over the rules.
//!
//! # What it reads
//!
//! **The relation is the row's opening word and the file is only where one was first met.** A
//! file holds as many relations as it likes: `spec/data/rules.4x` opens thirteen and
//! `schema.4x` the rest.
//!
//! **That changed on 2026-09-26 and the old shape is worth one line, because the check that
//! broke is the lesson.** `spec/data/` used to hold eleven files generated one per relation from
//! `releases/first-release.md`, and this asserted one relation per file. **The assertion was a
//! proxy for *no nulls*** - two relations in one table give a table with blank cells - and it
//! held right up until `P-563` put the foundation form there. `S-194`. **Keep the invariant, drop
//! the proxy**: `no_rendered_table_has_an_empty_cell` asks the rule itself, over every cell.
//!
//! Sean, 2026-09-26: *we are not giving up on the relational model, so there will be no nulls.*
//! `spec/invariants.md` carries the reason - the relational model guarantees coherence, and an
//! incoherent model cannot be accurate.
//!
//! **A relation's columns are the keys its own rows carry**, in the order first seen, so one
//! table per relation has no empty cell to render. **A row is a map, so within one row the order
//! is the map's** - every column the first row carries, alphabetically, then each later shape's
//! new columns after them. Stable, and deliberately not the order the file writes them in.

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

/// Every relation in a directory of `.4x` files.
///
/// **A relation is the word its rows open with, and a file may hold many.** `P-563` put the
/// foundation form in `spec/data/`, where `rules.4x` opens thirteen relations and `schema.4x`
/// thirteen more - so rows are gathered by name and the file is only where one was first met.
///
/// **It used to be one relation per file and that was a proxy**, standing in for *no nulls*
/// through the filename: rows of two relations rendered as one table have blank cells, and one
/// file per relation made that impossible. `S-194` is where it failed. The invariant is kept and
/// asserted directly now - `no_rendered_table_has_an_empty_cell` - and a relation's columns are
/// the keys its own rows carry, so gathering by name has nothing left to guard.
pub fn read(directory: &Path) -> Vec<Relation> {
    let mut found: Vec<Relation> = Vec::new();

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
            let row = fields(words, &shown, at + 1);

            // **A relation is gathered by its name and not by its file**, which is `S-194`.
            // `P-563` moved the foundation form into `spec/data/`, where `rules.4x` opens
            // thirteen relations and `schema.4x` thirteen more - so the file stopped being the
            // unit and a reader that assumed it was went red.
            //
            // **Two rows of one relation in different files join**, which is what makes this a
            // lookup by name rather than a reset per file. `file` then records where the relation
            // was first met rather than the only place it lives.
            let relation = match found.iter_mut().position(|it| it.name == name) {
                Some(at) => &mut found[at],
                None => {
                    found.push(Relation {
                        name: name.to_string(),
                        file: shown.clone(),
                        columns: Vec::new(),
                        rows: Vec::new(),
                    });
                    found.last_mut().expect("just pushed")
                }
            };
            for column in row.keys() {
                if !relation.columns.contains(column) {
                    relation.columns.push(column.clone());
                }
            }
            relation.rows.push(row);
        }
    }
    found
}

/// One row's fields.
///
/// # It read a value as running to the next `key:` token, and does not need to
///
/// **That was `C-120`**: `spec/data/line.4x` carried `qty:`$where`'s density for that resource`,
/// a sentence where `spec/console.md` gives a field one token. This reader tolerated it and
/// counted it, so the defect stayed visible and stayed the data's rather than becoming a red gate
/// on a file this lane does not own.
///
/// **`P-563` deleted that file with the rest of the old rendering, and the population went to
/// zero.** What replaced it is stricter than any tolerance here: `spec/data/` now holds the
/// foundation form, `game_model::foundation` reads those same bytes with `game_model::notation::read`,
/// and that refuses a bare word outright - *every value is named*. **So a sentence-valued
/// quantity fails the build before this reader is ever called**, which is where a notation rule
/// belongs.
///
/// **It refuses rather than dropping, which the old version could not afford to.** A bare word
/// here would otherwise become a column with an empty cell - the null
/// `no_rendered_table_has_an_empty_cell` forbids - and a silent drop would lose a field. The
/// message names the file and the line, because this reads a directory and a file spec adds
/// without the engine loading it would arrive here unvalidated.
fn fields<'a>(
    words: impl Iterator<Item = &'a str>,
    file: &str,
    line: usize,
) -> BTreeMap<String, String> {
    let mut row: BTreeMap<String, String> = BTreeMap::new();
    for word in words {
        let (key, value) = word.split_once(':').unwrap_or_else(|| {
            panic!("{file} line {line}: `{word}` is a bare word and every value is named")
        });
        row.insert(key.to_string(), value.to_string());
    }
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
pub fn markdown(found: &[Relation]) -> String {
    let mut out = String::from("# The rules, as relations\n\n");
    out.push_str(
        "**Generated. Do not edit.** One table per relation in `spec/data/`, which is where the\n\
         rules are stated. `reports/state.md` is the same view over the state the scenario\n\
         leaves; this is the same view over the rules it played by.\n\n\
         **The relation is the word each row opens with, and a file holds as many as it likes** -\n\
         `rules.4x` opens thirteen of these and `schema.4x` the rest.\n\n",
    );
    out.push_str(&format!(
        "{} relations, {} rows.\n\n",
        found.len(),
        found.iter().map(|it| it.rows.len()).sum::<usize>()
    ));

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

    /// **No rendered table has an empty cell**, which is the invariant the old check guarded
    /// through a proxy.
    ///
    /// # What the proxy was, and why it stopped working
    ///
    /// **This asserted one relation per file.** That was true of the old `spec/data/` - eleven
    /// files, each generated from one table of `releases/first-release.md` - and it protected the
    /// real rule by standing in for it: rows of two relations in one table render as one table
    /// with a lot of blanks, and the file name was a reliable way to stop that happening.
    ///
    /// **`P-563` put the foundation form there and the proxy failed on the first read.**
    /// `rules.4x` opens thirteen relations and `schema.4x` thirteen more, and Sean, 2026-09-26:
    /// *we are not giving up on the relational model, so there will be no nulls.* **The invariant
    /// is kept and the proxy is gone** - `read` gathers by relation name now, so a table's columns
    /// are the keys its own rows carry and there is nothing left for a filename to guard.
    ///
    /// # So the invariant is asserted directly, over every cell there is
    ///
    /// **A proxy answers a narrower question than the one asked** - `C-28` - and the cost here was
    /// a check that went red on data that was correct. This asks the rule itself: every cell of
    /// every rendered row is non-empty. **Both populations are asserted**, because a read that
    /// found no relations, or relations with no rows, would report no empty cells for the same
    /// reason an empty directory reports no failures.
    #[test]
    fn no_rendered_table_has_an_empty_cell() {
        let found = read(&data());
        assert!(
            found.len() > 10,
            "only {} relations, so finding no empty cell means nothing",
            found.len()
        );

        let mut cells = 0;
        let mut empty: Vec<String> = Vec::new();
        for relation in &found {
            assert!(
                !relation.rows.is_empty(),
                "`{}` has no rows, so its columns are guarded by nothing",
                relation.name
            );
            for row in &relation.rows {
                for (column, cell) in relation.columns.iter().zip(relation.cells(row)) {
                    if cell.is_empty() {
                        empty.push(format!("{}.{column}", relation.name));
                    }
                    cells += 1;
                }
            }
        }

        assert!(
            empty.is_empty(),
            "{} cells are empty, so the relational model has nulls in it: {:?}",
            empty.len(),
            empty.iter().take(8).collect::<Vec<&String>>()
        );
        assert!(
            cells > 1000,
            "only {cells} cells were read, which is not this data"
        );
    }

    /// **A relation's name is the word its rows open with and not its file name**, and the cases
    /// where those differ are named rather than assumed away.
    ///
    /// **It is every relation whose file holds more than one**, now that `P-563` landed the
    /// foundation form: twenty-six of them across two files, none of which could be named after
    /// the file it sits in. **The four that differed before were plural file names** - `biomes.4x`
    /// holding `value`, `kinds.4x` holding `kind` - and they are gone with the rendering they came
    /// from.
    #[test]
    fn a_relation_is_named_by_its_rows_and_not_by_its_file() {
        let found = read(&data());
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
        // **Counted rather than listed**, because the list is the ruleset's and moves whenever
        // Sean adds a relation. What this has to catch is the reader going back to naming a
        // relation after its file, which would make this empty.
        assert_eq!(
            differing.len(),
            found.len(),
            "no relation in `spec/data/` is named after the file it sits in - {differing:?}"
        );
        assert!(
            differing.len() > 20,
            "only {} relations differ from their file name, which is not this data",
            differing.len()
        );
    }
}
