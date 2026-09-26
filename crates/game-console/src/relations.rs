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
//! held right up until `P-563` put the foundation form there. `S-194`.
//!
//! Sean, 2026-09-26: *we are not giving up on the relational model, so there will be no nulls.*
//! `spec/invariants.md` carries the reason - the relational model guarantees coherence, and an
//! incoherent model cannot be accurate.
//!
//! # Where *no nulls* is actually held, which is not here
//!
//! **A relation's columns are the keys its own rows carry**, in the order first seen - so a table
//! has a blank cell exactly when two rows of one relation carry different key sets.
//!
//! **`game_model::schema::Schema::fits` forbids that outright**, requiring a row's key set to
//! equal its relation's declared columns *exactly rather than at least*, and `schema::check`
//! applies it to every row of everything `game_model::foundation` loads - which is these same two
//! files. **So the rule is held upstream and this module cannot break it.** `Q-101`, and this lane
//! re-derived it rather than accepting it.
//!
//! **This lane first replaced the proxy with a check that could not fail**, and said in its own
//! doc that it *asks the rule itself*. What is here now compares this reader's rows against the
//! engine's over the same bytes: two readers, one input, an equality rather than a floor somebody
//! chose.
//!
//! **A row is a map, so within one row the order is the map's** - alphabetical. **Stable, and
//! deliberately not the order the file writes them in**, which is not recoverable from a relation
//! once its rows have been gathered.

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
/// file per relation made that impossible. `S-194` is where it failed. **What holds the invariant
/// is `Schema::fits`**, upstream of this and over the same bytes - see the module header - so
/// gathering by name has nothing left to guard.
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
/// foundation form, `game_model::foundation` reads those same bytes with
/// `game_model::notation::read`, and that refuses a bare word outright - *every value is named*. **So a sentence-valued
/// quantity fails the build before this reader is ever called**, which is where a notation rule
/// belongs.
///
/// **It refuses rather than dropping, which the old version could not afford to.** A bare word
/// here would otherwise become a column with an empty cell, and a silent drop would lose a field.
/// The message names the file and the line, because this reads a directory: a file the
/// specification adds without the engine loading it would arrive here unvalidated, and that is the
/// one case `Schema::fits` upstream does not cover.
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

    /// **This reader and the engine's read the same files the same way**, which is what the
    /// one-relation-per-file assertion was replaced by.
    ///
    /// # The first replacement claimed more than it did, and `Q-101` measured it
    ///
    /// **The assertion this replaces was a proxy for *no nulls* through the filename.** That held
    /// while `spec/data/` was eleven files generated one per relation from
    /// `releases/first-release.md`; `P-563` put the foundation form there, where `rules.4x` opens
    /// thirteen relations, and it failed on the first read. Sean, 2026-09-26: *we are not giving up
    /// on the relational model, so there will be no nulls.*
    ///
    /// **This lane then wrote `no_rendered_table_has_an_empty_cell` and said it *asks the rule
    /// itself*. It could not fail.** A cell is empty exactly when two rows of one relation carry
    /// different key sets, and `game_model::schema::Schema::fits` forbids that outright - the key
    /// set must equal the declared columns, *exactly rather than at least* - with `schema::check`
    /// applying it to every row of everything `foundation.rs` loads.
    ///
    /// **The quality lens found it and this lane re-derived it rather than accepting it**: `fits`
    /// at `schema.rs:821` compares sorted key sets for equality, `check` at `:965` runs it over
    /// `rows.rows()`, and the one population that could have differed is empty -
    /// `relations::read` globs `spec/data/*.4x` and `foundation::FOUNDATION` names two, and the
    /// glob today returns those same two. **So the extra coverage was zero files**, which is
    /// `CLAUDE.md`'s count over nothing wearing the other sign.
    ///
    /// # What is worth checking is that the two readers agree
    ///
    /// **Two readers over one input, and an equality rather than a floor.** The engine's
    /// `notation::read` and this module's `read` parse the same bytes for different purposes, and
    /// nothing said they got the same rows. **A floor is a number this lane chose**; this is a
    /// comparison, and it fails if either reader drifts.
    ///
    /// **`Q-101` measured what the floors were worth and they were worse than feared**:
    /// `cells > 1000` against 2,004 tolerated the reader losing forty-nine per cent, and
    /// `schema.4x` alone would have passed both by one per cent. Neither guarded the data, because
    /// a file cannot vanish without `include_str!` failing to compile.
    #[test]
    fn this_reader_and_the_engines_agree_about_the_rows() {
        let found = read(&data());
        let mut compared = 0;

        for (_, at, text) in game_model::foundation::FOUNDATION {
            if !at.contains("spec/data") {
                continue;
            }
            // **The engine's reader, over the bytes it carries** - which
            // `foundation::what_is_carried_is_what_is_on_disk` holds equal to the file this module
            // read off disk, so the two are reading one input and not two copies of one.
            let theirs = game_model::notation::read(text).expect("the engine reads it");
            assert!(!theirs.is_empty(), "{at} parsed to nothing");

            let mut by_relation: BTreeMap<String, Vec<BTreeMap<String, String>>> = BTreeMap::new();
            for row in &theirs {
                by_relation
                    .entry(row.relation.clone())
                    .or_default()
                    .push(row.values.clone());
            }

            for (name, rows) in by_relation {
                let mine = found
                    .iter()
                    .find(|it| it.name == name)
                    .unwrap_or_else(|| panic!("{at} declares `{name}` and this reader lost it"));
                // **Compared per relation rather than in total**, so one relation gaining rows
                // while another loses them cannot cancel out.
                //
                // **`found` spans both files and `rows` is one file's**, which is exact only
                // because no relation appears in both - thirteen in `rules.4x`, thirteen in
                // `schema.4x`, twenty-six distinct. **A relation that ever spanned them would
                // compare all of this reader's rows against one file's** and fail on the count
                // below, loudly and with both numbers, rather than passing over the difference.
                // Reported by the quality lens; left as a sentence because the failure is the
                // right one.
                assert_eq!(
                    mine.rows.len(),
                    rows.len(),
                    "`{name}`: the engine read {} row(s) of it and this reader read {}",
                    rows.len(),
                    mine.rows.len()
                );
                for (at_row, theirs) in rows.iter().enumerate() {
                    assert_eq!(
                        &mine.rows[at_row], theirs,
                        "`{name}` row {at_row}: the two readers disagree about what it says"
                    );
                }
                compared += rows.len();
            }
        }

        // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. With
        // no `spec/data/` entry in `FOUNDATION` this loop would compare nothing and pass in
        // exactly these words. **Equality against the reader's own total**, so the number is
        // derived on both sides rather than chosen here: every relation this reader found was
        // matched, and every row of it.
        assert_eq!(
            compared,
            found.iter().map(|it| it.rows.len()).sum::<usize>(),
            "every row this reader found was compared against the engine's"
        );
        assert!(compared > 0, "no rows were compared");
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
