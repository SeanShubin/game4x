//! The scenario's expected state, as a data file.
//!
//! **`S-29`.** The expected values were ninety-six `assert_eq!` lines in
//! `tests/first_release.rs` - citizens, turns, densities, counts - and no file said what the
//! scenario should produce. Sean cannot read a test to check what it expects, and
//! `docs/process.md` asks that he can: *the scenario test reads the data files for its
//! input, reads the data files for what is expected, computes what actually happens, and
//! compares.*
//!
//! **This is half of `P-218` and only half.** A data file the *test* reads is not a data
//! file the *game loads*, and the rule wants the second. The kinds, recipes and costs are
//! still Rust and markdown.
//!
//! **And nothing has moved yet.** This is the mechanism; `tests/first_release.rs` still
//! carries all ninety-six assertions, and they are live scenario values. `S-34`: they come
//! out in the **same change** that puts the first reviewed expectation in. Not before, or
//! the scenario is checked by nothing. Not after, because *after* is a window in which the
//! scenario has two expectations - a reviewed file and ninety-six lines written by whoever
//! wrote the code - and **the one that is wrong is not the one that fails**. A stale
//! assertion fails loudly while being the thing nobody ever reviewed.
//!
//! # The notation
//!
//! `P-212`: a command is `{name field:value ...}`, the name is the words that open it, and
//! arguments are named. This is the same notation for data - one line per row, the name is
//! the table, the fields are its columns:
//!
//! ```text
//! {game phase:play turn:8 territories:12 units:0}
//! {territory territory:1 biome:grassland founded:yes citizens:12}
//! ```
//!
//! **A focused reader rather than the command grammar.** `command-language` matches an
//! utterance against declared forms and reports disagreement; expected data has no forms to
//! declare and every row is its own shape. The notation is shared so that `S-26` adopting
//! `{...}` in the console does not make this a second dialect.
//!
//! A value containing a space is quoted: `upkeep:"1 food per turn"`. Nothing else is
//! special, and a value is text - the comparison is between what was written and what was
//! produced, and turning `12` into a number first would only add a way to disagree.

use std::collections::BTreeMap;

/// One row: the table it belongs to, and its named values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub table: String,
    pub fields: Vec<(String, String)>,
}

/// A word as it must be written to survive being read back.
///
/// Quoted when it is empty or holds a space, because those are the two ways a bare word
/// stops being one word. A `"` inside becomes `'`: no value in a state has one, and a
/// quoting rule with an escape is a second thing to get wrong.
fn quoted(word: &str) -> String {
    if word.is_empty() || word.contains(' ') || word.contains('"') {
        format!("\"{}\"", word.replace('"', "'"))
    } else {
        word.to_string()
    }
}

impl Row {
    /// The row as one line, in the notation above.
    pub fn written(&self) -> String {
        // The table's name is quoted for the same reason - one of them is called
        // *territory resource*.
        let mut out = format!("{{{}", quoted(&self.table));
        // **Names are quoted too, not only values.** A column is called *force of nature*,
        // and writing that bare made the row unreadable at three words rather than one
        // field. The round-trip test found it on its first run, which is what a round-trip
        // is for - the writer and the reader are the two halves most likely to agree with
        // each other and disagree with the truth.
        for (name, value) in &self.fields {
            out.push_str(&format!(" {}:{}", quoted(name), quoted(value)));
        }
        out.push('}');
        out
    }

    /// What makes this row *this* row rather than another of its table.
    ///
    /// The table and its first field. Every table the dump produces leads with what
    /// identifies a row - a territory number, a kind, a unit id - so a row can be matched
    /// with its counterpart before their fields are compared. Without that, a changed value
    /// reads as one row missing and another extra, which says where to look far less well.
    pub fn identity(&self) -> String {
        match self.fields.first() {
            Some((name, value)) => format!("{} {name}:{value}", self.table),
            None => self.table.clone(),
        }
    }
}

/// Every row of a state, in the order the dump lists them.
pub fn rows(game: &game_model::Game) -> Vec<Row> {
    let mut out = Vec::new();
    for table in crate::dump::tables(game) {
        for values in &table.rows {
            out.push(Row {
                table: table.name.to_string(),
                fields: table
                    .columns
                    .iter()
                    .zip(values)
                    .map(|(name, value)| (name.to_string(), value.clone()))
                    .collect(),
            });
        }
    }
    out
}

/// A state as a data file.
pub fn write(game: &game_model::Game, about: &str) -> String {
    let mut out = format!("# {about}\n");
    out.push_str(
        "# Expected. Reviewed by hand; the scenario test compares what it computes with \
         this.\n# Deleting a row, or this file, is how changing your mind is said - \
         `docs/process.md`.\n",
    );
    let mut last = String::new();
    for row in rows(game) {
        if row.table != last {
            out.push('\n');
            last = row.table.clone();
        }
        out.push_str(&row.written());
        out.push('\n');
    }
    out
}

/// What is wrong between what was expected and what happened.
///
/// **Three directions, and `extra` is the one a per-value assertion cannot have.** Ninety-six
/// `assert_eq!` lines can each be right while the game grows a territory nobody expected,
/// because an assertion checks what it names and names what somebody thought of. Comparing
/// whole states makes *unexpected* a finding rather than a blind spot.
#[derive(Debug, Default)]
pub struct Disagreement {
    /// Expected and did not happen.
    pub missing: Vec<String>,
    /// Happened and was not expected.
    pub extra: Vec<String>,
    /// Both, and a field differs.
    pub different: Vec<String>,
}

impl Disagreement {
    pub fn total(&self) -> usize {
        self.missing.len() + self.extra.len() + self.different.len()
    }

    pub fn report(&self) -> String {
        let mut out = String::new();
        for (what, lines) in [
            ("missing - expected and did not happen", &self.missing),
            ("extra - happened and was not expected", &self.extra),
            ("different", &self.different),
        ] {
            out.push_str(&format!("  {what} ({}):\n", lines.len()));
            for line in lines {
                out.push_str(&format!("    {line}\n"));
            }
        }
        out
    }
}

/// Compare what was expected with what happened.
pub fn compare(expected: &[Row], actual: &[Row]) -> Disagreement {
    let key = |row: &Row| row.identity();
    let by_identity = |rows: &[Row]| -> BTreeMap<String, Row> {
        rows.iter().map(|row| (key(row), row.clone())).collect()
    };
    let want = by_identity(expected);
    let got = by_identity(actual);

    let mut wrong = Disagreement::default();
    for (identity, row) in &want {
        match got.get(identity) {
            None => wrong.missing.push(row.written()),
            Some(theirs) if theirs == row => {}
            Some(theirs) => {
                for ((name, value), (_, other)) in row.fields.iter().zip(&theirs.fields) {
                    if value != other {
                        wrong.different.push(format!(
                            "{identity} · {name}: expected {value:?}, got {other:?}"
                        ));
                    }
                }
                // Same identity, different shape: the columns moved rather than a value.
                if row.fields.len() != theirs.fields.len() {
                    wrong.different.push(format!(
                        "{identity} · expected {} fields, got {}",
                        row.fields.len(),
                        theirs.fields.len()
                    ));
                }
            }
        }
    }
    for (identity, row) in &got {
        if !want.contains_key(identity) {
            wrong.extra.push(row.written());
        }
    }
    wrong
}

/// Read a state back from its data file.
///
/// Comments and blank lines are skipped; everything else must be a `{...}` row, because a
/// line that is neither is a typo rather than something to be lenient about.
pub fn read(text: &str) -> Result<Vec<Row>, String> {
    let mut out = Vec::new();
    for (at, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some(inner) = line.strip_prefix('{').and_then(|l| l.strip_suffix('}')) else {
            return Err(format!("line {}: not a `{{...}}` row: {line}", at + 1));
        };
        let mut words = split(inner);
        let Some(table) = words.next().map(|t| t.trim_matches('"').to_string()) else {
            return Err(format!("line {}: a row with no name", at + 1));
        };
        let mut fields = Vec::new();
        for word in words {
            let Some((name, value)) = word.split_once(':') else {
                return Err(format!("line {}: `{word}` is not `field:value`", at + 1));
            };
            fields.push((
                name.trim_matches('"').to_string(),
                value.trim_matches('"').to_string(),
            ));
        }
        out.push(Row { table, fields });
    }
    Ok(out)
}

/// Split on spaces, except inside quotes.
fn split(text: &str) -> impl Iterator<Item = String> + use<> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    for character in text.chars() {
        match character {
            '"' => {
                quoted = !quoted;
                current.push(character);
            }
            ' ' if !quoted => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(character),
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    words.into_iter()
}
