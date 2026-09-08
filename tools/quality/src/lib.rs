//! Find loops that assert over a population nothing has counted.
//!
//! **`Q-74`, and the three before it.** A loop whose assertions are all inside its body checks
//! nothing when the thing it iterates comes back empty, and passes - which reads exactly like a
//! check that ran. `Q-48`, `Q-51`, `Q-72` and `Q-74` are four instances in one repository, the last
//! two on one day and the last one in the test written to close the one before it.
//!
//! # Why this is mechanisable when `C-28` says the shape is not
//!
//! `CLAUDE.md` says **no check can ask whether another check's predicate is about its subject**,
//! and that is true and is not this. *Is this predicate about the right thing* is semantic and has
//! no instrument. ***Is a denominator stated at all*** is syntactic: a length or an emptiness
//! assertion on the population, somewhere between the enclosing `fn` and the loop, is a thing a
//! scanner can look for.
//!
//! **So this triages rather than judges.** It answers the narrow question and says which one it
//! answered, and a person reads the list. A candidate here is not a defect; it is a test worth
//! looking at, and this lens triaged its own output before handing any of it on - the scanner's
//! worst false positive was four correct tests in `poles.rs`. The instrument that would decide is
//! the one `C-28` says cannot exist.
//!
//! # What it deliberately does not report
//!
//! **A loop over a literal cannot be empty**, so `for x in ["a", "b"]` and `for i in 0..3` are
//! skipped. Their population is visible in the source and is not zero. This is the distinction
//! that keeps the list short enough to read - without it, most of the 193 loops in this
//! repository's tests would be listed and the report would be worthless.

use std::path::Path;

/// Why a loop is a candidate. The two differ in how they get fixed.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Missing {
    /// The loop iterates a call's result directly, so the population is never bound to a name.
    /// **Nothing could have asserted about it**, which makes this the stronger signal.
    NeverNamed,
    /// The population has a name and no length or emptiness assertion precedes the loop.
    NotAsserted,
    /// Every assertion in the body sits behind a `continue`, so the population that matters is
    /// **how many times the body ran**, not how long the collection was.
    ///
    /// **A different failure from an empty collection, and a worse one.** The collection can be
    /// full and the filter still match nothing - `the_pentagons_are_the_corners_and_are_isolated`
    /// passed over a graph with no pentagons in it, a test named for the pentagons checking
    /// none. Asserting the collection's length would not have caught it. Named by the code lane.
    FilteredAway,
}

impl std::fmt::Display for Missing {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Missing::NeverNamed => write!(out, "never named"),
            Missing::NotAsserted => write!(out, "not asserted"),
            Missing::FilteredAway => write!(out, "filtered, and the filter is uncounted"),
        }
    }
}

/// One loop worth looking at.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Candidate {
    pub file: String,
    /// One-based, so it can be pasted into an editor.
    pub line: usize,
    /// The iterated expression as written.
    pub over: String,
    /// The enclosing function, where one was found.
    pub in_fn: String,
    pub why: Missing,
    /// Where the `for` begins, as a character offset into the source it was scanned from.
    ///
    /// **Carried so that only *this* loop is blanked** when asking whether the test asserts
    /// anywhere else. Blanking every loop was the bug: a nested loop's enclosing loop asserts
    /// outside it, and removing both said the test asserted nowhere.
    pub at: usize,
}

/// The identifier a population would be asserted about, or `None` when there is not one.
///
/// The root of the expression: `files` in `files.iter().filter(..)`, `&things` gives `things`.
/// A call - `eol_attributes(&invented)` - yields `None`, because a temporary has no name to
/// assert about.
pub fn population_of(expression: &str) -> Option<String> {
    let text = expression.trim().trim_start_matches(['&', '*']).trim();
    let mut ident = String::new();
    let mut chars = text.chars().peekable();
    while let Some(character) = chars.next() {
        if character.is_alphanumeric() || character == '_' {
            ident.push(character);
        } else if character == ':' && chars.peek() == Some(&':') {
            // A path is not a binding either. `Direction::poles()` names a call and
            // `Resource::ALL` a constant, and neither is a local anything could assert about,
            // so the path is followed to whatever ends it.
            chars.next();
            ident.clear();
        } else {
            // A call on the root itself - `f(x).iter()` - names no binding.
            if character == '(' {
                return None;
            }
            break;
        }
    }
    if ident.is_empty() || is_keyword(&ident) {
        None
    } else {
        Some(ident)
    }
}

fn is_keyword(word: &str) -> bool {
    matches!(word, "self" | "Self" | "if" | "match" | "loop" | "while")
}

/// Whether the population is visible in the source and cannot be empty.
///
/// An array or slice literal, or a range with literal bounds. **A literal's size is not a
/// question**, which is exactly why asserting it would be noise.
pub fn cannot_be_empty(expression: &str) -> bool {
    let text = expression.trim();
    let root = text.trim_start_matches(['&', '*']).trim();
    // An array or slice literal, written out or borrowed: `["a"]` and `&[(1.0, 2.0)]` alike.
    // A string literal is a visible population too - `"ABC".chars()` cannot be empty.
    if root.starts_with('[') || root.starts_with('"') {
        return true;
    }
    // A constant is a literal with a name. `Resource::ALL` and `NOT_SHOWN` are written out in
    // the source exactly as an array literal is, so asserting their size is the same noise.
    let head: String = root
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == ':')
        .collect();
    if let Some(last) = head.rsplit("::").next()
        // At least one letter, or `0` in `0..files.len()` reads as a constant and the range
        // whose bound is a length - the one that genuinely can be empty - gets skipped.
        && last.chars().any(|c| c.is_ascii_uppercase())
        && last
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    {
        return true;
    }
    let bounds: Vec<&str> = text.splitn(2, "..").collect();
    if bounds.len() == 2 {
        // A bound is visible when it is a literal or a constant. `2_000` carries a separator
        // and `8u64` a type suffix, and `0..PENTAGON_COUNT` names a constant - all three are
        // written in the source, and none of them is a question a test should be asserting.
        let visible = |bound: &str| {
            let bound = bound.trim();
            if bound.is_empty() {
                return false;
            }
            // A type suffix ends in digits too - `8u64` - so the number is taken from the
            // front and whatever follows must begin with a letter to be one.
            let head: String = bound
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '_')
                .collect();
            let rest = &bound[head.len()..];
            let numeric = head.chars().any(|c| c.is_ascii_digit())
                && (rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_alphabetic()));
            let constant = bound.chars().any(|c| c.is_ascii_uppercase())
                && bound
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
            numeric || constant
        };
        if visible(bounds[0].trim()) && visible(bounds[1].trim_start_matches('=').trim()) {
            return true;
        }
    }
    false
}

/// Whether this text asserts how big `name` is.
pub fn states_a_denominator(text: &str, name: &str) -> bool {
    let sized = [
        format!("{name}.len()"),
        format!("{name}.is_empty()"),
        format!("{name}.count()"),
    ];
    // An assertion may wrap across lines, so the region is searched as one string and the
    // assertion and the size are required to be in the same statement rather than the same line.
    for statement in text.split(';') {
        let asserts = statement.contains("assert!")
            || statement.contains("assert_eq!")
            || statement.contains("assert_ne!")
            || statement.contains("panic!");
        if asserts && sized.iter().any(|size| statement.contains(size.as_str())) {
            return true;
        }
    }
    false
}

/// Whether every assertion in this loop body sits behind a `continue`.
///
/// **The denominator is then how many times the body ran.** A full collection whose filter
/// matches nothing runs the body zero times, and asserting the collection's length says nothing
/// about that - which is why this is reported even when the population *is* asserted.
pub fn body_is_behind_a_filter(body: &str) -> bool {
    // **Only this loop's own `continue` counts**, and only in code. A nested loop's guard is
    // not this one's, and a comment is prose: the code lane's repair of the pentagons test
    // *describes* the `continue` it removed, and reading that word reported the fix as the
    // defect. `planet-model`'s own source guard already says it - only code counts.
    let own = code_only(&without_nested_loops(body));
    let Some(guard) = own.find("continue") else {
        return false;
    };
    if !asserts_something(&own) {
        return false;
    }
    // Every assertion after the guard, and none before it.
    !asserts_something(&own[..guard])
}

/// The text with line comments removed, because prose may name what it is about.
fn code_only(text: &str) -> String {
    text.lines()
        .map(|line| line.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The body with any nested `for` loop blanked out, leaving what this loop does itself.
fn without_nested_loops(body: &str) -> String {
    let chars: Vec<char> = body.chars().collect();
    let mut kept = String::new();
    let mut at = 0usize;
    // The body's own opening brace, so the scan starts inside it.
    while at < chars.len() {
        let rest: String = chars[at..].iter().collect();
        let Some(found) = rest.find("for ") else {
            kept.push_str(&rest);
            break;
        };
        let start = at + rest[..found].chars().count();
        kept.push_str(&rest[..found]);
        match header_of(&chars, start)
            .and_then(|(_, brace)| block_after(&chars, brace).map(|b| brace + b.chars().count()))
        {
            Some(end) => at = end,
            None => {
                kept.push_str("for ");
                at = start + 4;
            }
        }
    }
    kept
}

/// Whether a block of code asserts anything.
pub fn asserts_something(text: &str) -> bool {
    text.contains("assert!")
        || text.contains("assert_eq!")
        || text.contains("assert_ne!")
        || text.contains("panic!(")
        || text.contains("unreachable!(")
}

/// The tests that can pass having checked nothing.
///
/// **This is the question worth asking, and `scan` alone is not it.** A loop with no denominator
/// is only a defect when the test around it has nothing else to say - a test that also asserts
/// outside the loop still fails when the loop's population is empty. Run over this repository,
/// the loop-level question returns 150 of 193 loops and the list is unreadable; this one returns
/// the tests where an empty population means a green run over nothing.
///
/// A test is reported when **every** assertion it makes is inside a loop whose population is
/// unasserted and could be empty.
pub fn vacuous_tests(path: &str, source: &str) -> Vec<Candidate> {
    let loops = scan(path, source);
    let mut out: Vec<Candidate> = Vec::new();
    for candidate in loops {
        if out.iter().any(|kept| kept.in_fn == candidate.in_fn) {
            continue;
        }
        let Some((from, to)) = test_body_span(source, &candidate.in_fn) else {
            continue;
        };
        if asserts_outside(source, (from, to), candidate.at) {
            continue;
        }
        out.push(candidate);
    }
    out
}

/// The span of the named function's body, if it is a `#[test]`, in character offsets.
fn test_body_span(source: &str, name: &str) -> Option<(usize, usize)> {
    let at = source.find(&format!("fn {name}("))?;
    if !source[..at].contains("#[test]") {
        return None;
    }
    let chars: Vec<char> = source.chars().collect();
    let brace = source[at..].find('{')? + at;
    let from = source[..brace].chars().count();
    let body = block_after(&chars, from)?;
    Some((from, from + body.chars().count()))
}

/// Whether the test asserts anywhere outside the one loop at `at`.
///
/// **Only that loop is blanked.** `the_derived_graph_is_a_goldberg_polyhedron` asserts its
/// region count before an inner loop and its pentagon count after it, both inside an enclosing
/// loop - so blanking every loop reported a test that fails at once on an empty collection.
/// Found by the code lane, and it is the same cause as this scanner's other three: not
/// following where the population came from.
fn asserts_outside(source: &str, body: (usize, usize), at: usize) -> bool {
    let chars: Vec<char> = source.chars().collect();
    let (from, to) = body;
    let Some((_, brace)) = header_of(&chars, at) else {
        return false;
    };
    let Some(block) = block_after(&chars, brace) else {
        return false;
    };
    let end = brace + block.chars().count();
    let before: String = chars[from..at.min(to)].iter().collect();
    let after: String = chars[end.min(to)..to].iter().collect();
    asserts_something(&before) || asserts_something(&after)
}

/// The candidate loops in one file's source.
///
/// `path` is only carried into the results; the source is what is read.
pub fn scan(path: &str, source: &str) -> Vec<Candidate> {
    let mut out = Vec::new();
    let bytes: Vec<char> = source.chars().collect();
    // In a `src/` file only the test module counts; a `tests/` file is all test.
    let from = if path.contains("/tests/") || path.contains("\\tests\\") {
        0
    } else {
        match source.find("#[cfg(test)]") {
            Some(at) => source[..at].chars().count(),
            None => return out,
        }
    };

    for (index, _) in source.match_indices("for ") {
        let start = source[..index].chars().count();
        if start < from {
            continue;
        }
        // `for` must begin a statement, so only whitespace sits before it on its line.
        let line_start = source[..index].rfind('\n').map_or(0, |at| at + 1);
        if !source[line_start..index].trim().is_empty() {
            continue;
        }
        let Some((over, body_at)) = header_of(&bytes, start) else {
            continue;
        };
        let Some(body) = block_after(&bytes, body_at) else {
            continue;
        };
        if !asserts_something(&body) || (cannot_be_empty(&over) && !body_is_behind_a_filter(&body))
        {
            continue;
        }
        let before: String = bytes[function_start(&bytes, start)..start].iter().collect();
        if body_is_behind_a_filter(&body) {
            out.push(Candidate {
                file: path.to_string(),
                line: source[..index].matches('\n').count() + 1,
                over: over.trim().to_string(),
                in_fn: enclosing_fn(&bytes, start),
                why: Missing::FilteredAway,
                at: start,
            });
            continue;
        }
        let why = match population_of(&over) {
            // **The population may be asserted where it is computed rather than where it is
            // used**, which is better practice and was this scanner's worst false positive:
            // `poles.rs` asserts `arrangements().len() >= 8` inside the helper, citing `Q-48`,
            // and four correct tests were reported until this followed the call.
            None if called_helper_asserts_its_size(source, &over) => continue,
            None => Missing::NeverNamed,
            Some(name) if states_a_denominator(&before, &name) => continue,
            Some(name) if bound_from_a_literal(&before, &name) => continue,
            Some(name) if bound_from_helper_that_asserts(source, &before, &name) => continue,
            Some(_) => Missing::NotAsserted,
        };
        out.push(Candidate {
            file: path.to_string(),
            line: source[..index].matches('\n').count() + 1,
            over: over.trim().to_string(),
            in_fn: enclosing_fn(&bytes, start),
            why,
            at: start,
        });
    }
    out
}

/// Whether `name` was bound from a literal, whose size is visible in the source.
///
/// **`let sources = [..]` is `for x in [..]` with a name on it.** The scanner's false positives
/// were all one thing - not following where the population came from - and this is the second
/// half of it, after the helper that asserts its own return.
pub fn bound_from_a_literal(before: &str, name: &str) -> bool {
    let Some(at) = before.rfind(&format!("let {name}")) else {
        return false;
    };
    let after = &before[at..];
    let Some(equals) = after.find('=') else {
        return false;
    };
    // Skip a type annotation and any `mut`, then look at what the value starts with.
    let value = after[equals + 1..].trim_start();
    value.starts_with('[') || value.starts_with("&[") || value.starts_with("vec![")
}

/// Whether the function this expression calls asserts its own size.
///
/// **A population asserted where it is computed is asserted**, and a scanner that only looks
/// between the enclosing `fn` and the loop cannot see it. Only a helper in the same file can be
/// followed; a call into another crate is reported and left to a reader.
pub fn called_helper_asserts_its_size(source: &str, expression: &str) -> bool {
    let text = expression.trim().trim_start_matches(['&', '*']).trim();
    let called: String = text
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if called.is_empty() {
        return false;
    }
    helper_asserts_its_size(source, &called)
}

/// Whether `name` was bound from a same-file helper that asserts its own size.
fn bound_from_helper_that_asserts(source: &str, before: &str, name: &str) -> bool {
    let Some(at) = before.rfind(&format!("let {name}")) else {
        return false;
    };
    let statement = &before[at..];
    let end = statement.find(';').unwrap_or(statement.len());
    let Some(equals) = statement[..end].find('=') else {
        return false;
    };
    called_helper_asserts_its_size(source, statement[equals + 1..end].trim())
}

fn helper_asserts_its_size(source: &str, called: &str) -> bool {
    let Some(at) = source.find(&format!("fn {called}(")) else {
        return false;
    };
    let chars: Vec<char> = source.chars().collect();
    let Some(brace) = source[at..].find('{').map(|offset| at + offset) else {
        return false;
    };
    let Some(body) = block_after(&chars, source[..brace].chars().count()) else {
        return false;
    };
    for statement in body.split(';') {
        let asserts = statement.contains("assert!")
            || statement.contains("assert_eq!")
            || statement.contains("panic!");
        if asserts && (statement.contains(".len()") || statement.contains(".is_empty()")) {
            return true;
        }
    }
    false
}

/// The iterated expression and where its body's `{` sits, given the offset of `for`.
fn header_of(source: &[char], at: usize) -> Option<(String, usize)> {
    let text: String = source[at..].iter().take(600).collect();
    let in_at = text.find(" in ")?;
    let rest = &text[in_at + 4..];
    // The brace that opens the body, skipping any that belong to the expression.
    let mut depth = 0i32;
    for (offset, character) in rest.char_indices() {
        match character {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            '{' if depth == 0 => {
                let expression = rest[..offset].trim().to_string();
                let brace = at + text[..in_at + 4 + offset].chars().count();
                return Some((expression, brace));
            }
            '\n' if depth < 0 => return None,
            _ => {}
        }
    }
    None
}

/// The text of the block whose `{` is at `at`.
fn block_after(source: &[char], at: usize) -> Option<String> {
    let mut depth = 0i32;
    for (offset, character) in source[at..].iter().enumerate() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(source[at..=at + offset].iter().collect());
                }
            }
            _ => {}
        }
    }
    None
}

/// Where the enclosing function begins, or the start of the file.
fn function_start(source: &[char], at: usize) -> usize {
    let text: String = source[..at].iter().collect();
    text.rfind("fn ")
        .map_or(0, |byte| text[..byte].chars().count())
}

/// The enclosing function's name, or `?` when there is none to find.
fn enclosing_fn(source: &[char], at: usize) -> String {
    let text: String = source[..at].iter().collect();
    let Some(byte) = text.rfind("fn ") else {
        return "?".to_string();
    };
    text[byte + 3..]
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// Every `.rs` file under `root`, in a stable order.
pub fn rust_files(root: &Path) -> Vec<String> {
    let mut found = Vec::new();
    walk(root, &mut found);
    found.sort();
    found
}

fn walk(at: &Path, found: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(at) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if name != "target" && name != ".git" {
                walk(&path, found);
            }
        } else if name.ends_with(".rs") {
            found.push(path.to_string_lossy().replace('\\', "/"));
        }
    }
}
