//! A game state as a data file, in the map form `spec/console.md` specifies.
//!
//! > **What a thing contains is a map from a description to a quantity.** A description is a
//! > kind and **every trait of that thing**; a trait **of its kind** is not part of one,
//! > because naming the kind has already said it, and a derived trait is never part of one.
//! > **No trait of the thing may be left out** - `{citizen defending:1} -> 8` and `{citizen
//! > defending:0} -> 6`, never `{citizen} -> 14`. **Each distinct description is its own
//! > entry, and an entry is never zero.** A thing carrying an `id` has a description no
//! > other thing shares, so **its quantity is always one**. **Where a thing is, is where it
//! > appears**; nothing states its container. **Entries are in the order their descriptions
//! > sort in**, so the same state is always the same bytes.
//!
//! # What replaced what
//!
//! **This is `S-47`, and it is one change rather than sixteen renames.** The predecessor
//! wrote one flat row per thing - `{territory id:1 biome:grassland citizens:8 yards:1}` -
//! and built those rows by iterating `dump::tables`, the markdown presentation. So the file
//! a person validates inherited its whole vocabulary from a table's column names, which is
//! `C-37`: the arrow pointing the wrong way. Renaming the columns would have fixed the
//! symptom by editing the presentation until the data it generated looked right.
//!
//! It is built from [`game_model::containment::tree`] now, which is the state itself.
//!
//! # The form, and the part of it that is an assumption
//!
//! ```text
//! {game phase:play}
//!   {orbit id:1} -> 1
//!     {ark defending:1 id:1 moving:1} -> 1
//!   {territory biome:grassland id:1 nature:1} -> 1
//!     {citizen bearing:1 defending:1 laboring:1} -> 8
//! ```
//!
//! **Written the way the file writes it, and both entries moved twice this week.** `ready:yes`
//! was the trait `P-399` replaced with a held kind and `P-411` replaced again with a count per
//! action, so a citizen carries three and a unit two - each `0 or 1`, none of them omitted,
//! because *no trait of the thing may be left out*. `fuel` left the description with `P-407`,
//! which marks it **of the kind**: naming `ark` has already said it. The traits are in sorted
//! order for the reason the form gives - *entries are in the order their descriptions sort
//! in*, so the same state is always the same bytes.
//!
//! **`spec/console.md` gives the entry and not the nesting.** It says a thing appears inside
//! what holds it and never states its container, which fixes what the file may say and
//! leaves open how a reader is to see the inside. **Indentation, two spaces to the level**,
//! is the assumption proceeded under - it is diffable, it needs no closing token, and a line
//! carries one entry the way every other line in a `.4x` file carries one thing. Filed as
//! `C-46`.
//!
//! **The root carries no quantity**, because a quantity belongs to an entry in some map and
//! the game is in no map: `spec/logistics.md` makes it *the one thing that is in nothing*.

use command_language::token::{Token, tokenize};
use game_model::containment::{Description, Entry, tree};

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
/// both turn on whether a person has looked, so Sean edits this line when he has.
pub const REVIEW_LINE: &str =
    "# Expected. NOT YET REVIEWED - written from the program's own output, awaiting Sean.\n";

/// How many spaces one level of containment is written with.
const INDENT: usize = 2;

/// The state as a tree of entries, which is what this file writes and reads.
pub fn entries(game: &game_model::Game) -> Entry {
    tree(game)
}

/// The tree as a data file.
pub fn write(game: &game_model::Game, about: &str) -> String {
    let mut out = format!("# {about}\n");
    out.push_str(REVIEW_LINE);
    out.push_str(
        "# When you have read it and it is what the scenario should produce, replace the \
         line\n# above with: `# Expected. REVIEWED by Sean.` That edit is the review, and it \
         is the\n# only record of one.\n#\n# Deleting a row, or this file, is how changing \
         your mind is said - `docs/process.md`.\n#\n# A thing appears inside what holds it, \
         and nothing states its container. A line reads\n# `{description} -> quantity`, and \
         the description is the kind and every trait of that thing.\n\n",
    );
    out.push_str(&written(&entries(game)));
    out
}

/// The tree alone, with no header, which is what a comparison is made against.
pub fn written(root: &Entry) -> String {
    let mut out = String::new();
    out.push_str(&root.description.written());
    out.push('\n');
    for held in &root.contents {
        write_entry(&mut out, held, 1);
    }
    out
}

fn write_entry(out: &mut String, entry: &Entry, depth: usize) {
    out.push_str(&" ".repeat(depth * INDENT));
    out.push_str(&entry.description.written());
    out.push_str(&format!(" -> {}\n", entry.quantity));
    for held in &entry.contents {
        write_entry(out, held, depth + 1);
    }
}

/// Read a state back from its data file.
///
/// **The check the release asks for is that this is the inverse of [`written`]** -
/// *releases/first-release.md* -> *Where things are*: **the check is that the dump reads back
/// into the state it came from.** A count of fields is not, because a plausible subset
/// passes it.
///
/// **What it recovers is the containment tree and not the [`game_model::Game`].** Those are
/// the same information only once a territory's `density` and `total capacity` are in the
/// file, and they are not - see [`game_model::containment`]. So the round trip proved here is
/// text against tree, and saying which half is proved is the whole of `S-29`'s warning about
/// reporting half a rule as met.
pub fn read(text: &str) -> Result<Entry, String> {
    let mut stack: Vec<Entry> = Vec::new();
    for (at, line) in text.lines().enumerate() {
        let line_number = at + 1;
        // **One tokenizer for both readers** - `Q-67`. This used to split the line itself,
        // and the two implementations of the same lexical rules diverged exactly once before
        // anybody noticed. `spec/console.md` says *A `#` begins a comment. The rest of the
        // line is ignored* - unqualified, and `command_language::tokenize` honoured it anywhere
        // while this skipped a line only when it **started** with one. So a trailing comment
        // was a parse error here and whitespace there.
        //
        // Nobody wrote that divergence. One reader never learned a rule the other had, and
        // each went on passing its own tests - which is what two implementations of one
        // notation were always going to produce. `S-59` is where the duplication came from:
        // before it there were two notations and two readers, which was right.
        //
        // **The grammar is not shared and must not be.** `parse_line` is grammar-directed and
        // this is shape-only, and this deliberately does not resolve kinds - a data file may
        // name a kind the program does not have, and the honest failure is the comparison
        // rather than the read. What is shared is the lexical layer alone.
        let tokens = tokenize(line, line_number);
        if tokens.is_empty() {
            continue;
        }
        // **`P-252` is a rule only if the reader enforces it.** *Nothing in a data file is
        // quoted*, and a reader that lets a quote through leaves the door open for the next
        // generator that wants a two-word name.
        if let Some(quoted) = tokens.iter().find(|token| token.text.contains('"')) {
            return Err(format!(
                "line {line_number}: `{}` is quoted - `P-252`: nothing in a data file is, and \
                 a name that needs two words joins them with dashes",
                quoted.text
            ));
        }

        // Indentation is where the first token starts, which the tokenizer already knows.
        let indent = tokens[0].span.from.column - 1;
        if !indent.is_multiple_of(INDENT) {
            return Err(format!(
                "line {line_number}: indented {indent}, which is not a multiple of {INDENT}"
            ));
        }
        let depth = indent / INDENT;
        let (description, quantity) = parse(&tokens, line_number)?;

        if stack.is_empty() {
            if depth != 0 {
                return Err(format!("line {line_number}: the first entry is indented"));
            }
            if quantity.is_some() {
                return Err(format!(
                    "line {line_number}: the game is in nothing, so it has no quantity"
                ));
            }
            stack.push(Entry {
                description,
                quantity: 1,
                contents: Vec::new(),
                capacity: Vec::new(),
            });
            continue;
        }
        // **Depth is checked rather than trusted.** A line indented two levels deeper than
        // its predecessor names a container that is not there, and reading it as a child of
        // whatever happens to be on the stack would invent a containment nobody wrote.
        if depth > stack.len() {
            return Err(format!(
                "line {line_number}: indented {depth} inside a thing at depth {}",
                stack.len() - 1
            ));
        }
        let Some(quantity) = quantity else {
            return Err(format!(
                "line {line_number}: an entry has a quantity, written `-> n`"
            ));
        };
        if quantity == 0 {
            return Err(format!("line {line_number}: an entry is never zero"));
        }
        while stack.len() > depth {
            close(&mut stack);
        }
        stack.push(Entry {
            description,
            quantity,
            contents: Vec::new(),
            capacity: Vec::new(),
        });
    }
    if stack.is_empty() {
        return Err("no entries at all, so there is no game".to_string());
    }
    while stack.len() > 1 {
        close(&mut stack);
    }
    Ok(stack.pop().expect("the stack holds the root"))
}

fn close(stack: &mut Vec<Entry>) {
    let done = stack.pop().expect("close is only called on a deeper stack");
    stack
        .last_mut()
        .expect("something contains it, or it would be the root")
        .contents
        .push(done);
}

/// One line: `{kind trait:value ...}` and, unless it is the root, ` -> quantity`.
fn parse(tokens: &[Token], at: usize) -> Result<(Description, Option<u32>), String> {
    let written = |tokens: &[Token]| -> String {
        tokens
            .iter()
            .map(|token| token.text.clone())
            .collect::<Vec<_>>()
            .join(" ")
    };
    let mut read = tokens.iter();
    match read.next() {
        Some(token) if token.text == "{" => {}
        _ => {
            return Err(format!(
                "line {at}: not a `{{...}}` description: {}",
                written(tokens)
            ));
        }
    }
    let Some(kind) = read.next().filter(|token| token.text != "}") else {
        return Err(format!("line {at}: a description with no kind"));
    };
    let mut description = Description {
        // The kind is compared and written as text; it is not looked up, because a data file
        // may legitimately name a kind this program does not have and the honest failure is
        // the comparison rather than the read.
        kind: Box::leak(kind.text.clone().into_boxed_str()),
        traits: Default::default(),
    };

    let mut closed = false;
    for token in read.by_ref() {
        if token.text == "}" {
            closed = true;
            break;
        }
        let Some((name, value)) = token.text.split_once(':') else {
            return Err(format!("line {at}: `{}` is not `trait:value`", token.text));
        };
        if description
            .traits
            .insert(name.to_string(), value.to_string())
            .is_some()
        {
            return Err(format!(
                "line {at}: `{name}` twice in one description, and a description is a map"
            ));
        }
    }
    if !closed {
        return Err(format!(
            "line {at}: not a `{{...}}` description: {}",
            written(tokens)
        ));
    }

    let rest: Vec<&Token> = read.collect();
    if rest.is_empty() {
        return Ok((description, None));
    }
    if rest[0].text != "->" {
        return Err(format!(
            "line {at}: `{}` follows the description",
            written(tokens)
        ));
    }
    let Some(quantity) = rest.get(1) else {
        return Err(format!("line {at}: `->` with no quantity after it"));
    };
    if rest.len() > 2 {
        return Err(format!(
            "line {at}: `{}` follows the quantity",
            rest[2].text
        ));
    }
    let quantity: u32 = quantity
        .text
        .parse()
        .map_err(|_| format!("line {at}: `{}` is not a quantity", quantity.text))?;
    Ok((description, Some(quantity)))
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
    /// Both, and the quantity differs.
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

/// Every entry in a tree, by the path that says where it is.
///
/// **The path is the identity, because where a thing is, is where it appears.** The
/// predecessor matched a row on its table and its first column, which needed a `key` field
/// saying how many leading columns named a row rather than described it - a second
/// declaration that the writer and the reader both had to agree about. Containment answers it
/// instead: two entries are the same entry when they are the same description in the same
/// place.
fn by_path(root: &Entry) -> std::collections::BTreeMap<String, u32> {
    let mut out = std::collections::BTreeMap::new();
    // **The root contributes nothing to a path.** Every thing is in the game, so naming it
    // in every line says the same thing every time and distinguishes nothing. The root's own
    // description is compared separately, in `compare`, because it is the one entry no path
    // can reach.
    fn walk(entry: &Entry, above: &str, out: &mut std::collections::BTreeMap<String, u32>) {
        let here = if above.is_empty() {
            entry.description.written()
        } else {
            format!("{above} {}", entry.description.written())
        };
        out.insert(here.clone(), entry.quantity);
        for held in &entry.contents {
            walk(held, &here, out);
        }
    }
    for held in &root.contents {
        walk(held, "", &mut out);
    }
    out
}

/// Compare what was expected with what happened.
pub fn compare(expected: &Entry, actual: &Entry) -> Disagreement {
    let mut wrong = Disagreement::default();
    if expected.description != actual.description {
        wrong.different.push(format!(
            "{} → {}",
            expected.description.written(),
            actual.description.written()
        ));
    }
    let want = by_path(expected);
    let got = by_path(actual);
    for (path, quantity) in &want {
        match got.get(path) {
            None => wrong.missing.push(format!("{path} -> {quantity}")),
            Some(theirs) if theirs != quantity => wrong
                .different
                .push(format!("{path} · {quantity} → {theirs}")),
            Some(_) => {}
        }
    }
    for (path, quantity) in &got {
        if !want.contains_key(path) {
            wrong.extra.push(format!("{path} -> {quantity}"));
        }
    }
    wrong
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root(text: &str) -> Entry {
        read(text).unwrap_or_else(|why| panic!("{why}"))
    }

    #[test]
    /// **The descriptions are ones the game writes**, which a round trip does not require
    /// and which is worth the two minutes anyway. This fixture read `{ark fuel:2 id:1
    /// ready:yes}` and `{citizen ready:yes}` until today: `ready` is a trait the game has not
    /// had since `P-399`, and `fuel` left a description with `P-407`, which marks it of the
    /// kind. **The property holds over any traits at all** - so nothing failed, and the one
    /// place a reader looks to see what a tree looks like showed a shape the game cannot emit.
    fn a_tree_is_written_and_read_back_as_itself() {
        let text = "{game phase:play}\n  {orbit id:1} -> 1\n    {ark defending:1 id:1 moving:1} -> 1\n  \
                    {territory biome:ice id:1 nature:1} -> 1\n    {citizen bearing:1 defending:1 laboring:1} -> 8\n";
        let tree = root(text);
        assert_eq!(written(&tree), text, "the same bytes, both ways");
        assert_eq!(tree.contents.len(), 2, "an orbit and a territory");
        assert_eq!(tree.contents[0].contents[0].quantity, 1);
        assert_eq!(tree.contents[1].contents[0].quantity, 8);
    }

    /// One notation, two readers, and the same lexical rules under both.
    ///
    /// **`Q-67`, and the divergence had already happened.** `spec/console.md` says *A `#`
    /// begins a comment. The rest of the line is ignored* - unqualified, and in the section
    /// that governs the language. `command_language::tokenize` honoured it anywhere in a line and
    /// this reader skipped a line only when it **started** with one, so a trailing comment was
    /// whitespace to the console and a parse error to the data file.
    ///
    /// **Nobody wrote it.** One reader never learned a rule the other had, and each went on
    /// passing its own tests. `S-59` is where the duplication came from: before it there were
    /// two notations and two readers, which was correct.
    ///
    /// **So the rules are exercised through both, over the same text.** A test that only
    /// checked this reader would have passed on the day the two disagreed.
    #[test]
    fn a_comment_anywhere_in_a_line_is_ignored_by_both_readers() {
        let cases: [(&str, usize); 4] = [
            ("{game phase:play}", 4),
            ("{game phase:play} # a trailing comment", 4),
            ("# a whole line", 0),
            ("   # an indented one", 0),
        ];
        assert_eq!(cases.len(), 4, "four ways a comment can sit on a line");
        for (line, words) in cases {
            assert_eq!(
                tokenize(line, 1).len(),
                words,
                "the tokenizer reads `{line}` as {words} words"
            );
        }

        // And the reader agrees, which is the half that was wrong.
        let trailing = root(
            "{game phase:play} # the game, before anything happens\n  {territory id:1} -> 1 # one\n",
        );
        assert_eq!(trailing.description.written(), "{game phase:play}");
        assert_eq!(trailing.contents.len(), 1);
        assert_eq!(trailing.contents[0].quantity, 1);
        assert_eq!(
            written(&trailing),
            "{game phase:play}\n  {territory id:1} -> 1\n",
            "a comment is ignored rather than kept, so writing it back drops it"
        );
    }

    /// A `#` inside a value would begin a comment, which is worth knowing rather than finding.
    ///
    /// **Not a defect and not a decision this lane made** - it falls out of the rule being
    /// unqualified. Recorded because a value containing one is the case where the shared
    /// tokenizer changes what this reader accepts, and nothing in the game produces such a
    /// value: every value is a kind, a trait value or a number.
    #[test]
    fn a_hash_inside_a_value_begins_a_comment_like_anywhere_else() {
        let failure = read("{game phase:pl#ay}\n").expect_err("the brace is commented out");
        assert!(
            failure.contains("not a `{...}` description"),
            "the line ends at the hash: {failure}"
        );
    }

    /// Where a thing is, is where it appears - so a line's depth is the whole of it.
    #[test]
    fn depth_is_what_says_what_holds_what() {
        let inside = root("{game phase:play}\n  {territory id:1} -> 1\n    {citizen} -> 2\n");
        let beside = root("{game phase:play}\n  {territory id:1} -> 1\n  {citizen} -> 2\n");
        assert_eq!(
            inside.contents.len(),
            1,
            "the citizens are in the territory"
        );
        assert_eq!(beside.contents.len(), 2, "the citizens are in the game");
        assert_eq!(inside.contents[0].contents.len(), 1);
        assert!(beside.contents[0].contents.is_empty());
    }

    /// Every way a line can be wrong is a refusal that says which, rather than a guess.
    ///
    /// **Each arm asserted separately, and the count with them.** A loop over inputs that
    /// only checks `is_err` passes when every one of them fails for the same wrong reason.
    #[test]
    fn a_line_that_is_not_an_entry_is_refused_and_says_why() {
        let cases: [(&str, &str); 8] = [
            ("{game}\n  citizen -> 2\n", "not a `{...}` description"),
            ("{game}\n  {citizen}\n", "an entry has a quantity"),
            ("{game}\n  {citizen} -> 0\n", "an entry is never zero"),
            ("{game}\n  {citizen} -> many\n", "is not a quantity"),
            (
                "{game}\n  {citizen laboring} -> 1\n",
                "is not `trait:value`",
            ),
            (
                "{game}\n  {citizen laboring:1 laboring:0} -> 1\n",
                "twice in one description",
            ),
            ("{game}\n      {citizen} -> 1\n", "inside a thing at depth"),
            ("{game}\n   {citizen} -> 1\n", "not a multiple of"),
        ];
        assert_eq!(cases.len(), 8, "eight ways a line can be wrong");
        for (text, why) in cases {
            let refusal = read(text).expect_err(&format!("`{text}` must be refused"));
            assert!(
                refusal.contains(why),
                "reading `{text}` said `{refusal}`, which does not mention `{why}`"
            );
        }
        assert!(
            read("")
                .expect_err("an empty file is no game")
                .contains("no entries at all"),
            "and a file with nothing in it is refused too"
        );
        assert!(
            read("{game}\n  {citizen ready:\"a b\"} -> 1\n")
                .expect_err("a quoted value is refused")
                .contains("P-252"),
            "nothing in a data file is quoted"
        );
    }

    /// The comparison finds each of the three, and says where rather than what table.
    #[test]
    fn a_comparison_finds_missing_extra_and_different() {
        let want = root(
            "{game phase:play}\n  {territory id:1} -> 1\n    {citizen} -> 8\n    {yard} -> 1\n",
        );
        let got = root(
            "{game phase:play}\n  {territory id:1} -> 1\n    {citizen} -> 6\n    {store resource:food} -> 1\n",
        );
        let wrong = compare(&want, &got);
        assert_eq!(wrong.missing, vec!["{territory id:1} {yard} -> 1"]);
        assert_eq!(
            wrong.extra,
            vec!["{territory id:1} {store resource:food} -> 1"]
        );
        assert_eq!(wrong.different, vec!["{territory id:1} {citizen} · 8 → 6"]);
        assert_eq!(wrong.total(), 3);
        assert_eq!(
            compare(&want, &want).total(),
            0,
            "and a state matches itself"
        );
    }

    /// The same description in two places is two entries, which a flat comparison cannot say.
    ///
    /// **This is what the `key` field existed to approximate and could not.** Eight citizens
    /// in territory 1 and eight in territory 2 were one identity under the old comparison
    /// unless a column happened to tell them apart.
    #[test]
    fn one_description_in_two_places_is_two_entries() {
        let want = root(
            "{game}\n  {territory id:1} -> 1\n    {citizen} -> 8\n  {territory id:2} -> 1\n    {citizen} -> 8\n",
        );
        let got = root(
            "{game}\n  {territory id:1} -> 1\n    {citizen} -> 8\n  {territory id:2} -> 1\n    {citizen} -> 3\n",
        );
        let wrong = compare(&want, &got);
        assert_eq!(
            wrong.different,
            vec!["{territory id:2} {citizen} · 8 → 3"],
            "territory 1 is untouched and territory 2 is named"
        );
    }
}
