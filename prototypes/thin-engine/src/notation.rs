//! The notation, re-implemented.
//!
//! **A line is a row of a relation**: an opening word naming the relation, then named values.
//!
//! ```text
//! {at thing:scout place:1}
//! ```
//!
//! # This is a second parser and that is deliberate
//!
//! `crates/command-language` is the first, and `README.md` carries the argument. The short of
//! it: Sean asked for a prototype with no assumptions creeping in from existing code, **even the
//! notation**, because he may change the notation in here. A borrowed parser is a borrowed
//! decision about what a line may say.
//!
//! # What it does not do, and each absence is a concept not yet needed
//!
//! **No nesting.** `spec/console.md`'s notation lets a value be another command; nothing here
//! needs one, so a value is a word.
//!
//! **No quoting and no spaces in a value.** The release has three quantities that are English
//! phrases and they are `C-120` in the main tree; here a value is one token and a second token
//! would be a new key. **Refused rather than accepted**, so the day a value needs a space it is
//! a decision rather than a surprise.
//!
//! **No types.** Every value is a string. `1` and `scout` are the same sort of thing to this
//! parser, and whether that survives contact with arithmetic is a question for the concept that
//! needs arithmetic.

use std::collections::BTreeMap;

/// One row: the relation it belongs to, and its named values.
///
/// **A `BTreeMap` so that two rows written in a different key order are the same row.** The main
/// tree learned this the expensive way - `carries` writes `kind` before `trait` and `constraint`
/// writes them the other way round, and no single ordering can write both. Here the order a row
/// is *written* in is the relation's business and the order it is *held* in is nobody's.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    pub relation: String,
    pub values: BTreeMap<String, String>,
}

impl Row {
    pub fn value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

/// Why a line could not be read, in terms of the line rather than of the parser.
#[derive(Debug, PartialEq, Eq)]
pub struct Unreadable {
    /// One-based, so that it names the line a reader is looking at.
    pub line: usize,
    pub why: String,
}

impl std::fmt::Display for Unreadable {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "line {}: {}", self.line, self.why)
    }
}

/// Every row in a file of them.
///
/// **A blank line and a `#` comment are skipped and nothing else is.** A line that is not a row
/// is an error rather than something to step over: a file where a typo is silently ignored is a
/// file whose contents nobody can state.
pub fn read(text: &str) -> Result<Vec<Row>, Unreadable> {
    let mut rows = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        rows.push(row(line, number + 1)?);
    }
    Ok(rows)
}

fn row(line: &str, number: usize) -> Result<Row, Unreadable> {
    let complain = |why: &str| Unreadable {
        line: number,
        why: why.to_string(),
    };
    let inside = line
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
        .ok_or_else(|| complain("a row is written {relation key:value ...}"))?;

    let mut words = inside.split_whitespace();
    let relation = words
        .next()
        .ok_or_else(|| complain("a row names no relation"))?;
    if relation.contains(':') {
        return Err(complain("a row opens with a relation and not with a value"));
    }

    let mut values = BTreeMap::new();
    for word in words {
        let (key, value) = word
            .split_once(':')
            .ok_or_else(|| complain(&format!("`{word}` is a bare word; every value is named")))?;
        if key.is_empty() || value.is_empty() {
            return Err(complain(&format!("`{word}` has no key or no value")));
        }
        if values.insert(key.to_string(), value.to_string()).is_some() {
            // **A key twice is refused rather than taking the last.** Two values for one key is
            // a row that says two things, and picking one is inventing which.
            return Err(complain(&format!("`{key}` is given twice")));
        }
    }
    Ok(Row {
        relation: relation.to_string(),
        values,
    })
}

/// A row as the notation writes it, so that reading and writing are each other's inverse.
///
/// **Keys in the order they sort**, which is the only order this holds. A relation that wants a
/// column order of its own would state it, and none does yet.
pub fn write(row: &Row) -> String {
    let mut out = String::from("{");
    out.push_str(&row.relation);
    for (key, value) in &row.values {
        out.push(' ');
        out.push_str(key);
        out.push(':');
        out.push_str(value);
    }
    out.push('}');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_is_a_relation_and_its_named_values() {
        let rows = read("{at thing:scout place:1}").expect("one row");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].relation, "at");
        assert_eq!(rows[0].value("thing"), Some("scout"));
        assert_eq!(rows[0].value("place"), Some("1"));
    }

    /// **Writing is reading's inverse**, which is what lets the data be edited by hand and read
    /// back by the engine without either side owning the file.
    #[test]
    fn what_is_read_is_what_is_written() {
        let text = "{adjacent from:1 to:2}\n{at place:1 thing:scout}\n";
        let rows = read(text).expect("two rows");
        let again: Vec<String> = rows.iter().map(write).collect();
        assert_eq!(again.join("\n") + "\n", text);
        assert_eq!(read(&again.join("\n")).expect("re-read"), rows);
    }

    /// Every way a line can fail to be a row, and the count so that none is untested.
    #[test]
    fn a_line_that_is_not_a_row_says_which_line_and_why() {
        let refused = [
            "at thing:scout",
            "{}",
            "{at:x}",
            "{at scout}",
            "{at thing:}",
            "{at thing:a thing:b}",
        ];
        for line in refused {
            let why = read(line).expect_err(line);
            assert_eq!(why.line, 1, "{line}");
        }
        assert_eq!(refused.len(), 6, "six ways, and each is checked");

        // **The line number is the reader's**, so a file with a bad third line says three.
        let why = read("{a b:c}\n\n{a b}").expect_err("the third line is not a row");
        assert_eq!(why.line, 3);
    }

    /// **A relation with no values is a row**, found by a refusal test that used `{bad}` as its
    /// example of a line that is not one, and read it happily. Nothing says a row needs a value,
    /// so the rule is *a relation and any number of named values, including none* - and the
    /// sentence the parser was written from left the number open without noticing.
    #[test]
    fn a_relation_with_no_values_is_a_row() {
        let rows = read("{bad}").expect("a row with no values");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].relation, "bad");
        assert!(rows[0].values.is_empty());
        assert_eq!(
            write(&rows[0]),
            "{bad}",
            "and it writes back as it was read"
        );
    }

    #[test]
    fn a_blank_line_and_a_comment_are_not_rows_and_nothing_else_is_skipped() {
        let rows = read("# a note\n\n{at thing:scout place:1}\n").expect("one row");
        assert_eq!(rows.len(), 1);
    }
}
