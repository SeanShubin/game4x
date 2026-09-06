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
    /// How many leading fields name this row rather than describe it.
    pub key: usize,
}

/// A word, checked to be one.
///
/// **`P-252`.** `spec/console.md`: *Nothing in a data file is quoted.* A name is one word,
/// and where it needs more than one the words are joined with dashes. This used to quote a
/// word holding a space, which made the rule a convention - the writer could always fall
/// back on quotes, so a name with a space in it never had to be fixed, and four of them
/// were not, in seventy-one places.
///
/// So it panics rather than quoting. **A writer that cannot express a bad name cannot write
/// one**, and the failure is at the moment the name is invented rather than in a file
/// somebody reads later.
fn one_word(word: &str) -> &str {
    assert!(
        !word.is_empty() && !word.contains(' ') && !word.contains('"'),
        "{word:?} is not one word - `P-252`: join the words with dashes, and never quote"
    );
    word
}

impl Row {
    /// The row as one line, in the notation above.
    pub fn written(&self) -> String {
        // The table's name is quoted for the same reason - one of them is called
        // *territory resource*.
        let mut out = format!("{{{}", one_word(&self.table));
        // **Names are quoted too, not only values.** A column is called *force of nature*,
        // and writing that bare made the row unreadable at three words rather than one
        // field. The round-trip test found it on its first run, which is what a round-trip
        // is for - the writer and the reader are the two halves most likely to agree with
        // each other and disagree with the truth.
        for (name, value) in &self.fields {
            out.push_str(&format!(" {}:{}", one_word(name), one_word(value)));
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
        if self.key == 0 || self.fields.is_empty() {
            return self.table.clone();
        }
        let named: Vec<String> = self
            .fields
            .iter()
            .take(self.key)
            .map(|(name, value)| format!("{name}:{value}"))
            .collect();
        format!("{} {}", self.table, named.join(" "))
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
                key: table.key,
            });
        }
    }
    out
}

/// What a freshly written file says about whether anybody has looked at it.
///
/// **`S-35`.** This line used to read *Expected. Reviewed by hand* on a file the program had
/// just written from its own output. **The one artifact whose entire value is that a person
/// checked it opened by claiming a person had checked it** - correct about every number and
/// false about the only thing that made the numbers mean anything.
///
/// It is not deleted, because a file with no status line reads as reviewed to anyone who
/// does not know the history, which is everyone later. It says the true thing instead, and
/// **the line is where the state lives**: `P-219`'s lock and `P-225`'s deletion protocol
/// both turn on whether a person has looked, so Sean edits this line when he has. One act,
/// in the file, visible in the diff, impossible by accident - the same shape as `vetted`,
/// which only he sets.
///
/// A re-seed writes it back to unreviewed, which is right: a file the program rewrote is one
/// nobody has checked, whatever was true of the version before it.
pub const REVIEW_LINE: &str =
    "# Expected. NOT YET REVIEWED - written from the program's own output, awaiting Sean.
";

/// A state as a data file.
pub fn write(game: &game_model::Game, about: &str) -> String {
    let mut out = format!("# {about}\n");
    out.push_str(REVIEW_LINE);
    out.push_str(
        "# When you have read it and it is what the scenario should produce, replace the \
         line\n# above with: `# Expected. REVIEWED by Sean.` That edit is the review, and it \
         is the\n# only record of one.\n#\n# Deleting a row, or this file, is how changing \
         your mind is said - `docs/process.md`.\n",
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
    // **A description maps to a quantity, so this groups rather than replaces** - `P-287`.
    //
    // It was `BTreeMap<String, Row>`, built by collecting pairs, so **two rows with one
    // description kept only the last**. Nothing noticed while every row carried something
    // that made it unique; `S-48` deleted `node` from the extractor row, three food
    // extractors in one territory became three identical descriptions, and a turn that built
    // one more produced **no difference at all**. The table went from two rows to three and
    // the delta accounted for none of it.
    //
    // That is the shape this repository keeps recording: the instrument answered a narrower
    // question - *which descriptions are present* - than the one asked, and returned a
    // plausible empty delta rather than an error.
    let by_identity = |rows: &[Row]| -> BTreeMap<String, Vec<Row>> {
        let mut out: BTreeMap<String, Vec<Row>> = BTreeMap::new();
        for row in rows {
            out.entry(row.identity()).or_default().push(row.clone());
        }
        out
    };
    let want = by_identity(expected);
    let got = by_identity(actual);

    let mut wrong = Disagreement::default();
    for (identity, mine) in &want {
        let theirs: &[Row] = got.get(identity).map(Vec::as_slice).unwrap_or(&[]);
        // How many there are is part of the entry, so a difference in the count is reported
        // as the rows that have no counterpart rather than as a changed value.
        for row in mine.iter().skip(theirs.len()) {
            wrong.missing.push(row.written());
        }
        for row in theirs.iter().skip(mine.len()) {
            wrong.extra.push(row.written());
        }
        for (row, other) in mine.iter().zip(theirs) {
            if row == other {
                continue;
            }
            for ((name, value), (_, alternative)) in row.fields.iter().zip(&other.fields) {
                if value != alternative {
                    // **Neutral, because two readers want opposite words.** Against a
                    // reviewed expectation this is *expected X, got Y*; between two
                    // turns it is *was X, now Y*, and neither is a failure. One arrow
                    // is true for both, and the reader supplies the sentence.
                    wrong
                        .different
                        .push(format!("{identity} · {name}: {value} → {alternative}"));
                }
            }
            // Same identity, different shape: the columns moved rather than a value.
            if row.fields.len() != other.fields.len() {
                wrong.different.push(format!(
                    "{identity} · {} fields → {}",
                    row.fields.len(),
                    other.fields.len()
                ));
            }
        }
    }
    for (identity, theirs) in &got {
        if !want.contains_key(identity) {
            for row in theirs {
                wrong.extra.push(row.written());
            }
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
        // **A row written to a file does not carry its key**, because that would be noise
        // in the one artifact a person reads. So reading one asks the same question the
        // writer asked, from the same declaration - two would be a row that round-trips
        // into a different identity.
        let key = crate::dump::key_of(&table);
        out.push(Row { table, fields, key });
    }
    Ok(out)
}

/// Split on spaces. A quote is not a grouping character and is not allowed at all.
///
/// **`P-252` is a rule only if the reader enforces it.** This used to group on quotes, so a
/// file with `"force of nature"` in it read perfectly - which meant the writer could be
/// fixed and the format would still accept the thing it was fixed to stop producing.
/// Nothing would have failed, and the next generator to want a two-word name would have
/// found the door open.
///
/// Returned rather than panicking would be better and is not what this function can do: it
/// yields words and has no error channel. The panic is acceptable because reading these
/// files happens in a test and in one binary, both of which should stop.
fn split(text: &str) -> impl Iterator<Item = String> + use<> {
    assert!(
        !text.contains('"'),
        "`{text}` is quoted - `P-252`: nothing in a data file is, and a name that needs two words joins them with dashes"
    );
    let mut words = Vec::new();
    let mut current = String::new();
    for character in text.chars() {
        match character {
            ' ' => {
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

impl Disagreement {
    /// The same three lists, read as a turn's changes rather than as a failure.
    ///
    /// **`S-38`.** `compare` was built to answer *did the scenario match what was expected*,
    /// where every entry is something wrong. Between two turns the same three lists mean
    /// something else entirely: `missing` is what stopped being there, `extra` is what
    /// appeared, `different` is what moved. One computation, two readings - so the wording
    /// belongs at the point of reading and not in the comparison.
    pub fn as_a_turn(&self) -> String {
        if self.total() == 0 {
            return "*Nothing changed.*\n\n".to_string();
        }
        let mut out = String::new();
        for (what, lines) in [
            ("gone", &self.missing),
            ("new", &self.extra),
            ("changed", &self.different),
        ] {
            if lines.is_empty() {
                continue;
            }
            out.push_str(&format!("**{what}** ({})\n\n", lines.len()));
            for line in lines {
                out.push_str(&format!("- {line}\n"));
            }
            out.push('\n');
        }
        out
    }
}
