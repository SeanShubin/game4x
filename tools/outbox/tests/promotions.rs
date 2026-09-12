//! A promotion put in the target file what Sean approved.
//!
//! `S-10`, from the quality lens's `Q-39`. **`CLAUDE.md` promises that approved text is
//! byte-identical to shipped text, and after a promotion nothing could check it**: the
//! ledger keeps a one-line row, the proposal's body is deleted, and the approved text is
//! retained nowhere. The guarantee became unverifiable at the moment it was asserted, which
//! is why every defect in the promotions of 2026-09-01 was caught by a person.
//!
//! **It is buildable only from git**, and that is the whole trick: a promotion deletes the
//! proposal, so the proposal is still in the **parent** commit, with its `**shape**` and
//! `**into**` fields intact. No prose is parsed - both fields are structured.
//!
//! **Future promotions only, deliberately.** The 182 rows already in the ledger record their
//! destination as prose and would need a one-off audit of a back catalogue that has had a
//! week of readers. Here that is implemented as *an item that declares a shape*: the field
//! arrived in `P-195`, so anything older is skipped by having nothing to skip on.
//!
//! `P-194` gives three shapes and they are checked differently:
//!
//! - **text** is copied verbatim, so it must appear in the destination - compared with
//!   whitespace collapsed, because line wrapping is one of the three things a promotion may
//!   change.
//! - **rows** are table rows whose widths `tools/pad-tables` rewrites, so every cell must
//!   appear rather than every byte.
//! - **an instruction** lands nowhere verbatim and carries its own assertion, which the
//!   promoting commit runs. **This checks that it declared itself one and nothing else** -
//!   the weakest of the three arms, and named as weak rather than left to look complete.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).to_string())
}

/// A field's value, from the line that carries `**name**`.
///
/// Stops at the next `**`, so it does not matter whether the fields are joined by a middot
/// or by a hyphen, both of which appear in the queue.
pub fn field(body: &str, name: &str) -> Option<String> {
    let marker = format!("**{name}**");
    let at = body.find(&marker)? + marker.len();
    let rest = &body[at..];
    let end = rest.find("**").unwrap_or(rest.len());
    Some(
        rest[..end]
            .trim()
            .trim_end_matches(['\u{b7}', '-'])
            .trim()
            .to_string(),
    )
}

/// The file a proposal names, from an `**into**` field.
pub fn destination_file(into: &str) -> Option<String> {
    destination_files(into).into_iter().next()
}

/// Every file an `into` field names, in the order it names them.
///
/// **`CLAUDE.md`: a proposal that lands in more than one file carries one quotation for each,
/// in the order the destinations are named.** So a proposal may legally name two, and reading
/// only the first is why one of them could not be checked at all.
///
/// **`P-365` is the case and it was reported as unreadable for being legal.** It names
/// `spec/orbit.md` -> Crossing between layers, then `spec/units.md` -> Every unit, and offers
/// a quotation for each - so the check saw more blocks than destinations and refused. A rule
/// the specification states, met by a proposal that obeyed it, and an instrument that could
/// not read it.
///
/// **A backticked path, which is how every `into` field names one.** Anything backticked that
/// is not a path would be picked up here, so the pairing below is by position and the
/// comparison still has to succeed against the file that was named.
pub fn destination_files(into: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = into;
    while let Some(start) = rest.find('`') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('`') else { break };
        let named = &after[..end];
        // A section is named after `->` and is not backticked; a path is.
        if named.contains('/') || named.ends_with(".md") {
            out.push(named.to_string());
        }
        rest = &after[end + 1..];
    }
    out
}

/// Every **place** an `into` field names, as the file each one is in.
///
/// **`P-404`, promoted in `ba2a373`**: *a proposal that lands in more than one place carries
/// one quotation for each, in the order the destinations are named - whether those places are
/// two files or two sections of one.* So the unit is the destination and not the file, and a
/// proposal landing in four sections of one file carries four quotations against one path.
///
/// **The rule moved because five proposals had already outgrown the old one.** `P-399` names
/// four sections of `releases/first-release.md` and carries four quotations; splitting it into
/// four proposals about one change would be worse for the one person who reads them. `P-404`
/// is that, and this is the check following it.
///
/// **A file carries forward and a section does not.** `A -> S1, S2, then S3` is three places
/// in one file; `A -> S1, then B -> S2` is two places in two. Both are one rule: a backticked
/// path sets the file, and every comma-or-`then` separated piece after it is a place.
///
/// **What is deliberately not read is which section.** The check asks whether the quoted words
/// are in the file, not whether they are under the right heading - so a quotation landing in
/// the wrong section of the right file passes. Named rather than hidden: checking the section
/// needs the heading structure of five different documents, and the words landing at all is
/// the half that has caught every failure so far.
pub fn destinations(into: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    // **The first line only.** `field` reads to the next `**`, which for an `into` that is the
    // last field on its line runs on into the prose below - and the prose is full of backticked
    // paths. `P-383` came back with more destinations than it has places for exactly that
    // reason. An `into` field is one line, so this says so.
    let into = into.lines().next().unwrap_or_default();
    for piece in into.replace(" then ", ",").split(',') {
        let piece = piece.trim();
        if piece.is_empty() {
            continue;
        }
        if let Some(file) = destination_files(piece).into_iter().next() {
            current = Some(file);
        }
        if let Some(file) = &current {
            out.push(file.clone());
        }
    }
    out
}

/// Every blockquote in an item, each with its `> ` markers off and its lines kept.
///
/// **The lines are kept because `sentences` reads them.** Joining them with a space was
/// enough while the comparison was a substring test, and it destroyed every structural
/// boundary before the parse could see one - a heading ran into the paragraph under it, and
/// four correct promotions were reported missing. The blank lines are kept for the same
/// reason: one inside a quotation is a paragraph boundary, not a line break.
///
/// **`Item::proposed_text` refuses when there are several and that is right for its
/// caller** - `tools/spec` promotes one block and must not choose between two. This is a
/// different question: *did all of the approved text land*. A proposal quoting two
/// paragraphs separately has two blocks and both should be there, so refusing to read them
/// would leave a text proposal unchecked for having said more.
///
/// `P-223` and `P-224` were reported as *no readable block* for exactly that reason - both
/// carried several, both landed correctly, and the check could not see either.
fn blocks(body: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix('>') {
            current.push(rest.trim().to_string());
        } else if trimmed.is_empty() {
            // A blank line inside a quotation does not end it; a blank line after one does.
            if !current.is_empty() && !current.last().is_some_and(|l| l.is_empty()) {
                current.push(String::new());
            }
        } else if !current.is_empty() {
            found.push(current.join("\n").trim().to_string());
            current.clear();
        }
    }
    if !current.is_empty() {
        found.push(current.join("\n").trim().to_string());
    }
    found.retain(|b| b.split_whitespace().count() >= 3);
    found
}

/// Every bare markdown table in an item, each as its own block.
///
/// **A `shape rows` proposal may offer its rows as a table rather than as a quotation, and
/// both forms are in use.** `P-286` and `P-288` wrote `> | … |`; `P-274` and `P-310` wrote
/// the table plainly. Reading only blockquotes made the second form *no block to land* -
/// which is not a promotion that failed, it is this check answering a narrower question than
/// the one asked and returning a confident verdict about it. `C-28`, in the instrument.
///
/// Only the rows arm asks for these, because a table appearing beside a text proposal is
/// context rather than the thing offered.
fn tables(body: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        // A quoted table belongs to `blocks`, which already reads it.
        if trimmed.starts_with('|') && !line.trim_start().starts_with('>') {
            current.push(trimmed.to_string());
        } else if !current.is_empty() {
            found.push(current.join("\n"));
            current.clear();
        }
    }
    if !current.is_empty() {
        found.push(current.join("\n"));
    }
    // A table is a header, its rule, and at least one row. Fewer is a fragment quoted inline.
    found.retain(|table| table.lines().count() >= 3);
    found
}

/// Whitespace collapsed, so that re-wrapping a paragraph is not a difference.
///
/// Wrapping is one of the three things `CLAUDE.md` allows a promotion to change, so a
/// comparison that saw it would report every correct promotion as wrong.
pub fn flat(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// What a promotion is checked to have done, for one proposal.
#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Landed,
    /// Declared a shape this checker does not know.
    UnknownShape(String),
    Missing {
        what: String,
    },
    /// It did not land as approved, and the destination says so now.
    ///
    /// **A repaired promotion is not a standing failure.** This compares against the
    /// destination as it stood in the promoting commit, which is the precise question at
    /// the time - and no later commit can change what a past one contained, so a deviation
    /// caught and then fixed would otherwise stay red forever. `P-214` and `P-216` dropped
    /// their emphasis and `3fba321` restored it: the guarantee - *approved text is what is
    /// shipped* - holds again, and holding again is the outcome this exists to produce.
    Repaired,
    /// Several blocks, and no way to tell which one is the proposal.
    ///
    /// **Not a pass and not a failure.** A proposal often quotes context - *rule 7 as it
    /// stands* - beside the text it proposes, and nothing marks which is which. Requiring
    /// every block to land flags the context; requiring any one to land is satisfied by the
    /// context alone, since context is quoted *because* it is already there. Either answer
    /// would be wrong in the direction that looks right.
    ///
    /// So these are counted and named rather than judged. **A check that cannot answer
    /// should say so**, because the alternative is a green light that means nothing - and
    /// `P-223` and `P-224` both landed correctly, so guessing would have been right twice
    /// and taught me to trust it.
    Ambiguous,
}

/// Both sides of the comparison, parsed to the one thing they have in common.
///
/// **`P-289`, in Sean's words: a check can fail when nothing is wrong - a comparison broken
/// by a line wrap, a table's padding, a capital letter - and the fix that comes to hand is
/// to loosen it.** So this normalizes instead. Everything a promotion may change is
/// structure; what survives structure is a sequence of sentences; prose and the bullets it
/// became parse to the same sequence, and a change to the words does not.
///
/// **Every structural rule here was found by running it, not guessed.** An earlier attempt
/// was written against the six-case harness at the foot of this file, passed it at every
/// stage, and failed four promotions that had landed correctly - which is worse than the
/// loose version, because reporting a correct promotion as missing is what made somebody
/// loosen it in the first place. The four cases it cost:
///
/// - a blank line ends a block, so two paragraphs are never one sentence
/// - a quotation may open `> ` and then `- ` on the same line, so one marker is not enough
/// - a heading is structure: it bounds the prose around it, and its own words are a
///   sentence - which is also how a promotion changing the heading level lands
/// - a numbered item is a bullet, or its `1. ` reads as a sentence ending in the number 1
/// - **an outbox addressing line is bookkeeping rather than approved prose**, and is dropped
///
/// # Why the addressing line is not a change to the words
///
/// **The fifth case cost a green gate and is the only one found by a promotion rather than by
/// this file.** `P-395` offered `R-10` as a release capability and landed it verbatim, and this
/// reported it missing: `CLAUDE.md` requires every capability in `releases/` to carry
/// `**to** ... **status** ...` so that `tools/outbox` can see it, and that line is written *by
/// the promotion*, between the heading the proposal offered and the bullets it offered. Every
/// capability from `R-1` on has one, so the shape is the established one rather than a slip.
///
/// **Dropped from both sides rather than allowed on one**, which is this repository's standing
/// answer to a comparison that breaks on structure: normalize, do not loosen. A proposal's
/// quotation carries no such line today, and comparing the two after the same removal is what
/// keeps that from mattering.
///
/// **What this cannot see, said rather than left to be found.** Words smuggled into an
/// addressing line are now invisible here. The exposure is bounded: that line holds an id, an
/// addressee, a status and hashes, all of which `tools/outbox`'s own parser reads and reports,
/// and none of which is where a proposal's approved prose goes. **`C-91` carries the question
/// to the specification lane** - whether adding this line is a promotion's business at all is
/// `CLAUDE.md`'s subject, and this lane has made the check match what visibly already happens
/// rather than decided it.
pub fn sentences(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut block = String::new();
    let mut after_bullet = false;
    for line in text.lines() {
        let mut rest = line.trim();
        // A quotation marker is not prose, and it may be followed by another marker.
        while let Some(inner) = rest.strip_prefix('>') {
            rest = inner.trim_start();
        }
        if rest.is_empty() {
            drain(&mut block, &mut out);
            after_bullet = false;
            continue;
        }
        // An outbox addressing line: `**to** ... **status** ...`, which a promotion writes and
        // a proposal does not offer. Matched on both markers so that ordinary prose opening
        // with a bold word is not swallowed.
        if rest.starts_with("**to**") && rest.contains("**status**") {
            drain(&mut block, &mut out);
            after_bullet = false;
            continue;
        }
        // A heading bounds the prose on both sides of it and is a sentence of its own, so
        // that a promotion which changed the heading level still matches.
        if let Some(heading) = rest.strip_prefix('#') {
            drain(&mut block, &mut out);
            block.push_str(heading.trim_start_matches('#').trim());
            drain(&mut block, &mut out);
            after_bullet = false;
            continue;
        }
        // **A dash that opens a line is a bullet only where a bullet could start.** `P-418`
        // offered a paragraph whose last clause wrapped so that ` - a selector in a recipe`
        // began a line, and the destination wrapped it one word earlier - so the same
        // paragraph parsed to four sentences on one side and five on the other, and a
        // promotion that had landed exactly was reported missing. **Wrapping is structure**,
        // which is the one thing `P-283` allows to move, so the parse may not depend on it.
        //
        // A bullet begins a block or follows another bullet; a dash inside a paragraph is
        // punctuation continuing the sentence it is in.
        // **A list is open from its first item until the block ends**, which is what the
        // three resets above say: a blank line, a heading or an addressing line closes it.
        // So consecutive items are each a bullet, and a dash wrapped into the middle of a
        // paragraph is not one.
        let opens = block.is_empty() || after_bullet;
        if opens && let Some(inner) = bullet(rest) {
            drain(&mut block, &mut out);
            rest = inner;
            after_bullet = true;
        }
        if !block.is_empty() {
            block.push(' ');
        }
        block.push_str(rest);
    }
    drain(&mut block, &mut out);
    out
}

/// The marker that opens a list item, and what follows it.
fn bullet(line: &str) -> Option<&str> {
    for marker in ["- ", "* ", "+ "] {
        if let Some(rest) = line.strip_prefix(marker) {
            return Some(rest.trim_start());
        }
    }
    // `1. ` and `2. `, whose marker would otherwise read as a sentence ending in a number.
    // A leading `**` is not a marker: `**1.` opens emphasis, and `*` needs its space.
    let digits = line.find(|c: char| !c.is_ascii_digit())?;
    (digits > 0 && line[digits..].starts_with(". ")).then(|| line[digits + 2..].trim_start())
}

/// One block of prose becomes its sentences, and the block is emptied.
fn drain(block: &mut String, out: &mut Vec<String>) {
    let text = flat(block);
    let bytes = text.as_bytes();
    let mut start = 0;
    for (at, c) in text.char_indices() {
        if c != '.' {
            continue;
        }
        // Emphasis, a backtick or a bracket may close after the period - `**like this.**` -
        // and the sentence still ends where the period is.
        let mut after = at + 1;
        while after < bytes.len() && matches!(bytes[after], b'*' | b'`' | b'"' | b')' | b']') {
            after += 1;
        }
        if after < bytes.len() && bytes[after] != b' ' {
            continue;
        }
        push_sentence(&text[start..after], out);
        start = after;
    }
    push_sentence(&text[start..], out);
    block.clear();
}

/// One sentence, less the closing period `P-283` lets bullet-versus-paragraph take.
///
/// The period may sit inside the emphasis that closes the sentence, so it is taken from
/// there too - `**like this.**` bulleted is `**like this**`, and both are the same words.
fn push_sentence(sentence: &str, out: &mut Vec<String>) {
    let sentence = sentence.trim().trim_end_matches("**");
    let sentence = sentence.trim_end_matches('.').trim();
    if !sentence.is_empty() {
        out.push(sentence.to_string());
    }
}

/// The rule, over strings, so it can be run on documents written to be wrong.
/// The shape a proposal declares, in the one form everything below compares against.
///
/// **`S-68`, and the repair belongs here rather than in the four proposals that tripped it.**
/// `P-339`, `P-342`, `P-343` and `P-344` wrote `**shape** an instruction`; every earlier
/// promotion wrote the bare word. This read `shape != "instruction"`, took all four for a
/// shape it had never heard of, demanded a block none of them has, and reported four correct
/// promotions as wrong.
///
/// **The article is not the specification lane's mistake.** `CLAUDE.md` names the three
/// shapes in its own prose - *text*, *rows*, and **an instruction** - so the form this tool
/// refused is the form the governing document uses. **A check that only accepts a spelling
/// its own rules do not use catches a lane that did nothing wrong**, which costs exactly what
/// a false finding costs anywhere: the time to prove it false.
///
/// So both are read, here, once. **Normalizing before comparing rather than matching more
/// carefully** is the same lesson this repository has recorded for wrapped sentences and
/// padded table rows; this is the third surface it has arrived on.
///
/// **An unrecognised shape returns `None` and is named** rather than falling through to a
/// branch meant for something else, which is what made one typo look like two failures.
pub fn shape_of(said: &str) -> Option<&'static str> {
    let said = said.trim();
    let bare = said
        .strip_prefix("an ")
        .or_else(|| said.strip_prefix("a "))
        .unwrap_or(said)
        .trim();
    match bare {
        "text" => Some("text"),
        "rows" => Some("rows"),
        "instruction" => Some("instruction"),
        _ => None,
    }
}

/// Every shape a proposal declares, in the order it names them.
///
/// **A proposal offers one block per destination and a shape describes a block, so a proposal
/// landing in two files may name two shapes.** `P-459` is the case: a paragraph into
/// `spec/console.md` and a table into `releases/first-release.md`, declared `text and rows`.
/// `CLAUDE.md` says as much - *each indented quotation is one block of text being offered, and
/// the proposal says where that one goes. There is no count* - so the field was always plural
/// and this read it as singular.
///
/// **Four correct promotions were reported wrong for it**, which is the same failure `S-68`
/// recorded one field earlier: a check accepting a narrower spelling than the governing
/// document uses catches a lane that did nothing wrong. The repair is the same too - read the
/// form that is written rather than ask for the form that is convenient.
///
/// **The count is not the destinations' count.** `P-463` names three files and two shapes,
/// because a shape says what kind of thing lands rather than where it goes. So these are a
/// set to check blocks against, not a list to pair with `into`.
pub fn shapes_of(said: &str) -> Option<Vec<&'static str>> {
    let mut found = Vec::new();
    for part in said.split(" and ").flat_map(|part| part.split(',')) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let shape = shape_of(part)?;
        if !found.contains(&shape) {
            found.push(shape);
        }
    }
    (!found.is_empty()).then_some(found)
}

/// The blocks a proposal offers, in the order they appear, each with the shape its own form
/// implies.
///
/// **Only a compound shape needs this, and that is deliberate.** A single-shape proposal is
/// read exactly as it was before - `blocks`, falling back to `tables` for `rows` - so nothing
/// that passes today is re-judged by a new rule. What a compound needs and that path cannot
/// give is **order**: two blocks of different kinds have to reach the two files in the order
/// `into` names them, and `blocks` and `tables` each read the whole body separately.
///
/// **A block's form decides its shape rather than the declaration.** A table is `rows` and
/// anything else is `text`, which is the same distinction `CLAUDE.md` draws - *if the file
/// will say these words, that is text; if it will say them as cells in a table, that is rows*.
/// The declared set is then a claim about which forms appear, and a form outside it is context
/// rather than an offer.
fn offered(body: &str) -> Vec<(&'static str, String)> {
    let mut found = Vec::new();
    let mut quotation: Vec<String> = Vec::new();
    let mut table: Vec<String> = Vec::new();

    fn shut(found: &mut Vec<(&'static str, String)>, quotation: &mut Vec<String>) {
        let block = quotation.join("\n").trim().to_string();
        quotation.clear();
        if block.split_whitespace().count() < 3 {
            return;
        }
        let rows = block.lines().all(|line| line.trim().starts_with('|'));
        found.push((if rows { "rows" } else { "text" }, block));
    }

    for line in body.lines() {
        let trimmed = line.trim();
        let quoted = trimmed.starts_with('>');
        if trimmed.starts_with('|') && !quoted {
            if !quotation.is_empty() {
                shut(&mut found, &mut quotation);
            }
            table.push(trimmed.to_string());
            continue;
        }
        if !table.is_empty() {
            // A table is a header, its rule, and at least one row; fewer is a fragment.
            if table.len() >= 3 {
                found.push(("rows", table.join("\n")));
            }
            table.clear();
        }
        if let Some(rest) = trimmed.strip_prefix('>') {
            quotation.push(rest.trim().to_string());
        } else if trimmed.is_empty() {
            if !quotation.is_empty() && !quotation.last().is_some_and(|l| l.is_empty()) {
                quotation.push(String::new());
            }
        } else if !quotation.is_empty() {
            shut(&mut found, &mut quotation);
        }
    }
    if !quotation.is_empty() {
        shut(&mut found, &mut quotation);
    }
    if table.len() >= 3 {
        found.push(("rows", table.join("\n")));
    }
    found
}

pub fn check(shape: &str, block: &str, destination: &str) -> Verdict {
    match shape {
        "text" => {
            let want = sentences(block);
            let there = sentences(destination);
            // **`P-283` says what may move**: the line breaks, the bullet-versus-paragraph
            // form - which takes each sentence's closing period with it - and the heading
            // level. Every one of those is structure, so both sides are parsed to the thing
            // structure does not touch, **a sequence of sentences**, and the sequences are
            // compared in order. A comma, a dash, an emphasis marker or a reordering is
            // still a difference, because none of them is structure.
            //
            // **What this replaces, and why `P-289` says it had to go.** The predecessor
            // deleted every `. ` and every `- **` from both strings and asked whether one
            // contained the other. It got there one step at a time: `P-257` landed
            // correctly as four bullets and was reported missing, and the fix that came to
            // hand was to loosen the comparison rather than to normalize the two sides. The
            // comment written at the time admitted the cost - *a period deliberately
            // deleted mid-paragraph would now pass* - which is recording a check becoming
            // unable to fail rather than not doing it. `C-35`.
            if want.is_empty() {
                return Verdict::Missing {
                    what: "a block with no sentence in it".to_string(),
                };
            }
            if there.windows(want.len()).any(|run| run == want.as_slice()) {
                Verdict::Landed
            } else {
                Verdict::Missing {
                    what: want[0].chars().take(70).collect(),
                }
            }
        }
        "rows" => {
            let flat_destination = flat(destination);
            for row in block.lines().filter(|line| line.trim().starts_with('|')) {
                for cell in row.trim_matches('|').split('|') {
                    let cell = cell.trim();
                    if cell.is_empty() || cell.chars().all(|c| c == '-') {
                        continue;
                    }
                    if !flat_destination.contains(&flat(cell)) {
                        return Verdict::Missing {
                            what: format!("cell {cell:?}"),
                        };
                    }
                }
            }
            Verdict::Landed
        }
        // Nothing lands verbatim, and the assertion it carries is prose that the promoting
        // commit runs. Checking the destination here would be checking the wrong thing.
        "instruction" => Verdict::Landed,
        other => Verdict::UnknownShape(other.to_string()),
    }
}

/// Promotions this check cannot pass and cannot fix, each with the reason.
///
/// **A named exception, because a silent skip is the disease.** A promotion is history and
/// history is not rewritten to make a check green, so the alternative to naming these is
/// scoping the check to start after them - which turns it off for a reason no reader can
/// see. One line each, and the test below requires every one of them to still be failing.
///
/// **And that requirement is not running today, which the test now prints rather than
/// implies.** It reads the last 80 commits to touch the queue; the queue has had 444, so the
/// window slides, and all four of these promotions - 2026-09-02 to 09-04 - are already
/// behind it. An exception nothing reaches is neither confirmed nor refuted, and it reads
/// exactly like one that holds. **The sentence above claimed a guard that had stopped
/// firing**, which is the same defect as `C-35` one function over, so it is stated here
/// instead of asserted.
const KNOWN: &[(&str, &str)] = &[
    (
        "P-466",
        "one cell superseded inside its own promoting commit, which is `P-456`'s case and not the label problem this item first recorded. Its table is the seven columns *Units and structures* has afterwards, and **`yes` is the only cell of it the release does not carry** - checked cell by cell rather than inferred from the first failure. `P-465`, landing in the same commit, made the two *Movable* cells `1`. So the proposal is correct, the label is correct, and what cannot be judged is a promotion against a commit that also contains the one that overwrote a cell of it. **`C-107` said this was a before-and-after table and it is not**; the item is corrected.",
    ),
    (
        "P-465",
        "declared `shape rows` and its table describes a change rather than being one: `| Trait | Values now | Values after |`. A column headed *Values now* is a description of what the file said before, and `rows` means every cell lands - so this correctly looks for `Values now` in the release and does not find it. `CLAUDE.md`'s own test says which shape that is: *if it will say something these words only described, that is an instruction*. **The promotion is correct and only the label is wrong**, and a label is what this check reads. `C-107`, and `b8cd150` is the carrier that refuses this at filing time. **It said `the same shape one proposal later` and `P-466` is not that shape** - the item overstated its own population, and is corrected.",
    ),
    (
        "P-456",
        "its `Values` cell was superseded inside its own promoting commit. `b7fc6a6` landed six proposals in the order the message gives them, and `P-463` - the first of the six - had already made `id` admit *an identity*, so the cell `P-456` offered, *a number, unique among things of its kind*, is not in the file the commit produced. **Both promotions are correct**: the later one supersedes a cell of the earlier, and the message says so. What cannot be judged is a promotion against a commit that also contains the promotion that replaced it, which is the same shape as `P-214` and `P-216` a week apart rather than a commit apart. Its text block landed and is checked; only the row is excepted.",
    ),
    (
        "P-214",
        "dropped its quotation's emphasis, was repaired in `3fba321`, and the passage has since been superseded - a later promotion made `$` name a trait value as well as an ingredient. **`Repaired` is recomputed against `HEAD`, so it is not stable**: once the destination moves on for an unrelated reason, a settled deviation starts failing again. Judged at its own commit it deviated; judged today it cannot be judged at all, because the approved text is no longer what the file should say.",
    ),
    (
        "P-216",
        "the same: emphasis dropped, repaired in `3fba321`, passage since superseded.",
    ),
    (
        "P-236",
        "declared `shape text` and its quotation is a table row - `**asks** - <value>`, which landed as `| **asks** | <value> |` in `CLAUDE.md`'s field table. The promotion is correct: a row landed as a row, repadded by `tools/pad-tables`, which is exactly what `shape rows` means and is compared cell for cell. Only the label is wrong, and a label is what this check reads. `C-19`.",
    ),
    (
        "P-195",
        "declared `shape text` and its block is an instruction - it says what four sentences in `CLAUDE.md` become and adds a template field, and nothing in it lands verbatim. The first proposal to carry the field mislabelled its own shape, which is what this check found on its first run. **No repair can clear this one**: the four sentences landed correctly, so there is nothing in the destination to fix - the wrong thing is one field in a deleted proposal. `C-17`, answered.",
    ),
];

/// Every promotion since the `shape` field existed put its block where it said.
#[test]
fn a_promotion_lands_what_was_approved() {
    let root = root();
    if git(&root, &["rev-parse", "--is-inside-work-tree"]).is_none() {
        return;
    }
    // A shallow clone has one commit and therefore no parent to read a proposal from.
    if git(&root, &["rev-parse", "--is-shallow-repository"])
        .map(|out| out.trim() == "true")
        .unwrap_or(false)
    {
        return;
    }

    let log = git(
        &root,
        &[
            "log",
            "--format=%H",
            "-n",
            "80",
            "--",
            "docs/notes/proposals.md",
        ],
    )
    .unwrap_or_default();

    // **The ledger as it stands now, because a row can arrive late.** Promotion is detected
    // per commit - a proposal left the queue *and* gained an Accepted row in the same one -
    // and `C-40` is the case that breaks: eleven promotions had their rows written into the
    // Withdrawn table, so each was skipped, and `8d03a73` moving them restored the record
    // without restoring the check. Judged at their own commits they still gained no row.
    //
    // So a proposal that left the queue and is in the ledger **today** is a promotion whose
    // row arrived late, and is checked against its destination at the commit it left. A
    // withdrawal is still excluded, because a withdrawal never gets an Accepted row at all -
    // which is the discriminator the misfiling had temporarily destroyed.
    let landed_at_head: std::collections::BTreeSet<String> =
        git(&root, &["show", "HEAD:docs/notes/proposals.md"])
            .map(|text| {
                outbox::accepted(&text)
                    .into_iter()
                    .map(|row| row.id)
                    .collect()
            })
            .unwrap_or_default();
    let mut late_rows: Vec<String> = Vec::new();
    let mut exercised: Vec<&str> = Vec::new();
    let mut checked = 0usize;
    let mut older = 0usize;
    let mut excepted = 0usize;
    let mut repaired = 0usize;
    let mut ambiguous: Vec<String> = Vec::new();
    let mut left_without_landing: Vec<String> = Vec::new();
    let mut wrong = Vec::new();

    for commit in log.lines() {
        let Some(before) = git(
            &root,
            &["show", &format!("{commit}^:docs/notes/proposals.md")],
        ) else {
            continue;
        };
        let Some(after) = git(
            &root,
            &["show", &format!("{commit}:docs/notes/proposals.md")],
        ) else {
            continue;
        };
        // **A promotion is located by the proposal disappearing, never by the ledger row
        // appearing.** `git log -S` on a row finds the commit where the padder last widened
        // that table, not the commit that added it: on `P-1`'s row, landed 2026-08-25, it
        // answers a commit from 2026-08-28 about naming surfaces. Every row looks added
        // whenever a column moves.
        //
        // **But a disappearance is not a promotion on its own** - a withdrawal removes an
        // item too, and would be checked here as though its text should have landed
        // somewhere. So the ledger has to have gained a row for it in the same commit.
        let landed_now: std::collections::BTreeSet<String> = outbox::accepted(&after)
            .into_iter()
            .map(|row| row.id)
            .collect();
        let landed_before: std::collections::BTreeSet<String> = outbox::accepted(&before)
            .into_iter()
            .map(|row| row.id)
            .collect();

        // **An item can leave this queue without landing anywhere, and one has.** `P-344`
        // stopped asking approval and started asking a decision, so `4b9264d` moved it to
        // `docs/notes/decisions.md` - out of the queue, no ledger row, and a row at `HEAD`
        // because it was promoted for real three commits later. That is exactly the
        // signature of *promoted, row arrived late*, so this checked a move as though its
        // text should have landed, and reported a correct promotion as wrong.
        //
        // **`S-68`, and the second half of it.** The first half was the article on `an
        // instruction`; this is the one the article did not explain. I recorded it as a
        // named exception first - `P-344 declared shape rows and its table is an
        // illustration` - which was a true sentence about the wrong occurrence, and **the
        // exception mechanism refused it**: `P-344` is promoted twice in this history, the
        // exception matched by id, and the real promotion tripped *is excepted and now
        // passes*. A guard that will not let a wrong diagnosis in is worth more here than
        // the diagnosis was.
        let moved_away = git(
            &root,
            &["show", &format!("{commit}:docs/notes/decisions.md")],
        )
        .unwrap_or_default();

        let gone: Vec<outbox::Item> = outbox::parse(&before, "docs/notes/proposals.md")
            .into_iter()
            .filter(|item| item.id.starts_with("P-") && item.is_outstanding())
            .filter(|item| !after.contains(&format!("### {} ", item.id)))
            .filter(|item| !moved_away.contains(&format!("### {} ", item.id)))
            .filter(|item| {
                let promoted = landed_now.contains(&item.id) && !landed_before.contains(&item.id);
                // Its row arrived in a later commit than the promotion it records.
                let late = !promoted
                    && !landed_before.contains(&item.id)
                    && landed_at_head.contains(&item.id);
                if late {
                    late_rows.push(item.id.clone());
                }
                if !promoted && !late {
                    // Withdrawn, rejected, or promoted without a ledger row. The three are
                    // not distinguishable from here, so this records them rather than
                    // guessing which.
                    //
                    // **Named rather than counted, because the count hid four promotions.**
                    // `C-40`: `P-292`, `P-299`, `P-300` and `P-301` landed with their
                    // Accepted rows written into the Withdrawn table, so each left the queue
                    // and gained no row here, and each was skipped instead of checked. The
                    // number went from 2 to 7 in a day and **a number that moves says
                    // nothing about which**. Whoever reads this output can now see whether a
                    // name in it is a withdrawal, which is fine, or a promotion, which is
                    // the guarantee not holding.
                    left_without_landing.push(item.id.clone());
                }
                promoted || late
            })
            .collect();

        for item in gone {
            let Some(said) = field(&item.body, "shape") else {
                older += 1;
                continue;
            };
            let Some(shapes) = shapes_of(&said) else {
                wrong.push(format!(
                    "{}: declares shape {said:?}, which is not text, rows or an instruction. \
                     The article is accepted; the word is not one of the three",
                    item.id
                ));
                continue;
            };
            let shape = shapes.join(" and ");
            let files = field(&item.body, "into")
                .map(|i| destinations(&i))
                .unwrap_or_default();
            let Some(into) = files.first().cloned() else {
                wrong.push(format!("{}: no readable **into** field", item.id));
                continue;
            };
            // **An instruction carries no text that lands**, so it needs no block at all -
            // `P-222` had none and `P-220` had a before and an after. The destination check
            // below already declines to ask anything of an instruction; requiring a block
            // first meant refusing to read the ones that were correct.
            //
            // **A compound shape reads its blocks in document order instead**, because the
            // order is what pairs them with the files `into` names, and each block is
            // checked under the shape its own form implies.
            let compound = shapes.len() > 1;
            let mut forms: Vec<(&'static str, String)> = if compound {
                offered(&item.body)
                    .into_iter()
                    .filter(|(form, _)| shapes.contains(form))
                    .collect()
            } else {
                let mut quoted = blocks(&item.body);
                if shapes[0] == "rows" && quoted.is_empty() {
                    quoted = tables(&item.body);
                }
                quoted.into_iter().map(|b| (shapes[0], b)).collect()
            };
            if shapes == ["instruction"] {
                forms.clear();
            }
            if !shapes.contains(&"instruction") && forms.is_empty() {
                wrong.push(format!("{}: shape {shape} and no block to land", item.id));
                continue;
            }
            let quoted: Vec<String> = forms.iter().map(|(_, block)| block.clone()).collect();
            let Some(destination) = git(&root, &["show", &format!("{commit}:{into}")]) else {
                wrong.push(format!("{}: {into} is not in {}", item.id, &commit[..7]));
                continue;
            };
            checked += 1;
            // **One quotation per destination, in the order the destinations are named** -
            // `CLAUDE.md`. A proposal landing in two files carries two, and each is checked
            // against its own file; only a count that matches neither is unreadable.
            //
            // **`P-365` was reported unreadable for obeying that rule.** It names two files
            // and offers a quotation for each, and a check reading one destination saw two
            // blocks it could not place. Anything else with more blocks than destinations is
            // still `Ambiguous`, which is what left the other five in the list - and those
            // turned out to be irregular rather than unread, which is `P-404`.
            let mut verdict = if forms.is_empty() {
                Verdict::Landed // an instruction; nothing lands verbatim
            } else if compound {
                // **A compound shape asks each block to have landed in one of the files the
                // proposal names, rather than in the one its position picks out.** The
                // shapes do not pair with the destinations - `P-463` names three files and
                // two shapes - and `P-457` names two files where only one of them receives
                // anything verbatim, because the other is an instruction. So position is not
                // available, and what is left is still a real question: these words are in
                // one of the places this proposal said they would be.
                let mut all = Verdict::Landed;
                for (form, block) in &forms {
                    let mut anywhere = Verdict::Missing {
                        what: format!("no destination holds this {form} block"),
                    };
                    for file in &files {
                        let Some(text) = git(&root, &["show", &format!("{commit}:{file}")]) else {
                            continue;
                        };
                        if matches!(check(form, block, &text), Verdict::Landed) {
                            anywhere = Verdict::Landed;
                            break;
                        }
                    }
                    if anywhere != Verdict::Landed {
                        all = anywhere;
                        break;
                    }
                }
                all
            } else if quoted.len() == 1 {
                check(&shape, &quoted[0], &destination)
            } else if quoted.len() == files.len() {
                let mut all = Verdict::Landed;
                for (block, file) in quoted.iter().zip(&files) {
                    let Some(text) = git(&root, &["show", &format!("{commit}:{file}")]) else {
                        all = Verdict::Missing {
                            what: format!("{file} is not in {}", &commit[..7]),
                        };
                        break;
                    };
                    match check(&shape, block, &text) {
                        Verdict::Landed => {}
                        other => {
                            all = other;
                            break;
                        }
                    }
                }
                all
            } else {
                Verdict::Ambiguous
            };
            // If it did not land then, ask whether it has landed since. Only a `Missing` is
            // worth re-asking: an unknown shape is unknown at every commit.
            if matches!(verdict, Verdict::Missing { .. })
                && let Some(now) = git(&root, &["show", &format!("HEAD:{into}")])
                && forms
                    .iter()
                    .all(|(form, block)| matches!(check(form, block, &now), Verdict::Landed))
            {
                verdict = Verdict::Repaired;
            }
            if let Some((id, why)) = KNOWN.iter().find(|(id, _)| *id == item.id) {
                // The exception has to still be needed, or it is hiding a passing case and
                // will hide a failing one later.
                assert_ne!(
                    verdict,
                    Verdict::Landed,
                    "{} is excepted and now passes; delete the exception. It said: {why}",
                    item.id
                );
                exercised.push(*id);
                excepted += 1;
                continue;
            }
            match verdict {
                Verdict::Landed => {}
                Verdict::Repaired => repaired += 1,
                Verdict::Ambiguous => ambiguous.push(item.id.clone()),
                Verdict::UnknownShape(what) => wrong.push(format!(
                    "{} declares shape {what:?}, which is not text, rows or an instruction",
                    item.id
                )),
                Verdict::Missing { what } => wrong.push(format!(
                    "{} promoted into {into} as {shape} at {}, and {what} is not there",
                    item.id,
                    &commit[..7]
                )),
            }
        }
    }

    // Said rather than asserted: zero checked and all correct are the same green, and an
    // empty queue is the good state, so a count cannot be required.
    println!(
        "{checked} promotion(s) checked, {repaired} repaired after the fact, {excepted} excepted by name, {} unreadable ({ambiguous:?}); \n         {older} older than the shape field, {} left the queue without a ledger row {left_without_landing:?}; \n         {} whose ledger row arrived in a later commit {late_rows:?}",
        ambiguous.len(),
        left_without_landing.len(),
        late_rows.len()
    );

    // **The window is a cap, so it says what it dropped.** This reads the last 80 commits
    // that touched the queue, and the queue has had 444 - so the window slides, and every
    // one of the four named exceptions has already fallen out the back of it. While that is
    // true the `assert_ne!` above cannot run for them, and `KNOWN` reads as four live
    // exceptions while being four dead ones.
    //
    // Reported rather than repaired, because widening the window is not free - each commit
    // costs three `git show` calls - and because which of the two to do is a judgement about
    // this check's purpose rather than a defect in it. What is not acceptable is the silent
    // version: an exception list nothing exercises looks exactly like one that holds.
    let reach = log
        .lines()
        .last()
        .and_then(|oldest| {
            git(
                &root,
                &["log", "-1", "--format=%h %ad", "--date=short", oldest],
            )
        })
        .unwrap_or_default();
    let unexercised: Vec<&str> = KNOWN
        .iter()
        .map(|(id, _)| *id)
        .filter(|id| !exercised.contains(id))
        .collect();
    println!(
        "         the window is the last 80 commits to the queue, reaching back to {}; \n         {} of {} named exceptions were inside it{}",
        reach.trim(),
        exercised.len(),
        KNOWN.len(),
        if unexercised.is_empty() {
            String::new()
        } else {
            format!(
                ", and {unexercised:?} are older than it - \n         nothing exercises them, so they are neither confirmed nor refuted here"
            )
        }
    );
    assert!(
        wrong.is_empty(),
        "a promotion did not land what was approved:\n  {}\n\n\
         The proposal is in the promoting commit's parent. Read it there.",
        wrong.join("\n  ")
    );
}

/// A proposal that offers two kinds of block declares two shapes, and both are read.
///
/// **This is the check that did not exist**, and the four promotions it was written for were
/// green the moment it was added rather than before. `P-456`, `P-459` and `P-463` declared
/// `text and rows` and `P-457` declared `an instruction and text`; `shape_of` read the whole
/// field as one word, found none of the three, and reported four correct promotions as having
/// declared a shape that is not a shape. **The old spelling still has to work**, so the
/// single-shape cases are here beside the compound ones and the count covers both.
#[test]
fn a_compound_shape_is_every_shape_it_names() {
    let mut read = 0;
    for (said, want) in [
        ("text", vec!["text"]),
        ("an instruction", vec!["instruction"]),
        ("text and rows", vec!["text", "rows"]),
        ("rows and text", vec!["rows", "text"]),
        ("an instruction and text", vec!["instruction", "text"]),
        (
            "text, rows and an instruction",
            vec!["text", "rows", "instruction"],
        ),
        // A word repeated says nothing twice.
        ("text and text", vec!["text"]),
    ] {
        assert_eq!(
            shapes_of(said),
            Some(want.clone()),
            "{said:?} names the shapes {want:?}, in the order it names them"
        );
        read += 1;
    }
    assert_eq!(
        read, 7,
        "seven fields across the three shapes and their pairs"
    );

    // **One word outside the three refuses the whole field**, rather than the field being
    // read as whatever part of it happened to parse. A shape nobody defined is the typo this
    // check exists to name, and a compound is where one is easiest to hide.
    let mut refused = 0;
    for said in ["text and prose", "and", "", "a row and text", "text and"] {
        assert_eq!(
            shapes_of(said),
            None,
            "{said:?} contains something that is not a shape, so the field is not readable"
        );
        refused += 1;
    }
    assert_eq!(refused, 5, "five unreadable fields, and none was accepted");

    // **What the check before this one did, still here and still doing it.** `shape_of` reads
    // one word and refuses `text and rows`, which is correct of a word and wrong of the
    // field. So this is the deviation named rather than a behaviour quietly replaced: the
    // single-word reader is unchanged, and what is new is that nothing asks it the plural
    // question any more.
    assert_eq!(
        shape_of("text and rows"),
        None,
        "the single-word reader refuses a compound, which is what reported four promotions wrong"
    );
}

/// The blocks a compound proposal offers come back in the order they were written, each under
/// the shape its own form implies.
///
/// **Order is the whole reason this exists.** `blocks` and `tables` each read the body from
/// the top, so asking both gives every quotation and then every table - which says nothing
/// about which came first, and a compound proposal's blocks have to reach the files `into`
/// names in the order it names them.
///
/// **Written as a fixture rather than run against the queue**, because a proposal that
/// exercises a case today is deleted when it lands.
#[test]
fn a_compound_proposal_offers_its_blocks_in_order() {
    let body = "\
Some prose introducing the first thing.

> A paragraph of text that lands in the specification, long enough to count.

## The rows, into the release

| Trait  | Of      |
| ------ | ------- |
| **id** | a place |

And a closing paragraph, which offers nothing.
";
    let found = offered(body);
    assert_eq!(
        found.iter().map(|(form, _)| *form).collect::<Vec<_>>(),
        vec!["text", "rows"],
        "the quotation is first and the table second, which is how they were written"
    );
    assert!(found[0].1.starts_with("A paragraph of text"));
    assert!(
        found[1].1.lines().count() == 3,
        "a header, a rule and a row"
    );

    // **The other order, so this is not one example.** A check shown on one arrangement stops
    // meaning anything the moment the arrangement changes.
    let flipped = "\
| Trait  | Of      |
| ------ | ------- |
| **id** | a place |

> A paragraph of text that lands in the specification, long enough to count.
";
    assert_eq!(
        offered(flipped)
            .iter()
            .map(|(form, _)| *form)
            .collect::<Vec<_>>(),
        vec!["rows", "text"],
        "the table is first here, and the reader follows the document rather than a habit"
    );

    // A quoted table is rows too - `P-286` and `P-288` wrote them that way - and a two-line
    // table is a fragment rather than an offer.
    assert_eq!(
        offered("> | Trait  | Of      |\n> | ------ | ------- |\n> | **id** | a place |\n")
            .iter()
            .map(|(form, _)| *form)
            .collect::<Vec<_>>(),
        vec!["rows"],
        "a table inside a blockquote is still a table"
    );
    assert!(
        offered("| Trait | Of |\n| ----- | -- |\n").is_empty(),
        "a header and its rule with no row is a fragment quoted inline"
    );
}

/// The article on `an instruction` is read, and a word that is not a shape still is not one.
///
/// **The check this did not have, and `S-68` is what it cost.** Four correct promotions were
/// reported as wrong because they spelled the shape the way `CLAUDE.md` spells it. Both forms
/// are exercised here for all three shapes, and the population is asserted - a normalizer
/// tested on one spelling of one shape says nothing about the other five.
///
/// **And the failing direction, which is the half that stops this from being a rubber stamp.**
/// A normalizer that returned `Some("instruction")` for everything would pass every line above
/// and hide every typo, so the last block requires unknown words to stay unknown.
#[test]
fn a_shape_is_read_with_or_without_its_article() {
    let mut read = 0;
    for (said, want) in [
        ("text", "text"),
        ("rows", "rows"),
        ("instruction", "instruction"),
        ("an instruction", "instruction"),
        ("a text", "text"),
        ("  an instruction  ", "instruction"),
    ] {
        assert_eq!(
            shape_of(said),
            Some(want),
            "{said:?} is the shape {want:?}, written one of the two ways it is written"
        );
        read += 1;
    }
    assert_eq!(read, 6, "six spellings across the three shapes");

    // **Not everything is a shape, and an article does not make one.** Without this, a
    // normalizer that stripped the first word of anything would pass every case above.
    let mut refused = 0;
    for said in [
        "an interpretation",
        "instructions",
        "a row",
        "",
        "an",
        "table",
    ] {
        assert_eq!(
            shape_of(said),
            None,
            "{said:?} is not one of the three shapes and must be named rather than guessed"
        );
        refused += 1;
    }
    assert_eq!(
        refused, 6,
        "six words that are not shapes, and none was accepted"
    );
}

/// The rule, over documents written to be wrong.
///
/// **The live test above walks real history and is green when history is clean**, which is
/// the same green as a rule that does nothing. Every arm is therefore made to fail here.
#[test]
fn each_shape_is_checked_differently_and_each_can_fail() {
    // Text: wrapping may change, and the words may not.
    let block = "The tables that define kinds are the data\nthe game loads.";
    let wrapped = "- The tables that define kinds\n  are the data the game loads.\n";
    assert_eq!(check("text", block, wrapped), Verdict::Landed, "re-wrapped");
    assert!(
        matches!(
            check("text", block, "- The tables are the data the game loads.\n"),
            Verdict::Missing { .. }
        ),
        "text that never arrived has to be caught"
    );

    // Rows: the padder rewrites widths, so cells travel and bytes do not.
    let rows = "| **orbit** | a place above one territory |";
    let padded = "| **orbit**   | a place above one territory   |\n";
    assert_eq!(check("rows", rows, padded), Verdict::Landed, "re-padded");
    assert!(
        matches!(
            check("rows", rows, "| **orbit** | a place |\n"),
            Verdict::Missing { .. }
        ),
        "a cell that never arrived has to be caught"
    );

    // An instruction lands nowhere verbatim, so the destination cannot answer for it.
    assert_eq!(check("instruction", block, ""), Verdict::Landed);

    // And a shape nobody defined is reported rather than ignored, which is how a typo in
    // the field would otherwise turn the check off for that proposal.
    assert_eq!(
        check("prose", block, ""),
        Verdict::UnknownShape("prose".to_string())
    );
}

#[test]
fn a_field_is_read_whatever_separates_it() {
    let middot = "**to** sean \u{b7} **kind** cleanup \u{b7} **shape** text \u{b7} **into** `CLAUDE.md` -> Promotion";
    let hyphen =
        "**to** sean - **kind** cleanup - **shape** rows - **into** `spec/planet.md` -> Shape";
    assert_eq!(field(middot, "shape").as_deref(), Some("text"));
    assert_eq!(field(hyphen, "shape").as_deref(), Some("rows"));
    assert_eq!(
        field(middot, "into").and_then(|i| destination_file(&i)),
        Some("CLAUDE.md".to_string())
    );
    assert_eq!(
        field(hyphen, "into").and_then(|i| destination_file(&i)),
        Some("spec/planet.md".to_string())
    );
    assert_eq!(field(middot, "nonesuch"), None);
}

/// A capability waiting on a person is outstanding, and shows up as such.
///
/// **`S-17`.** `releases/first-release.md` gives a capability three states, and the middle
/// one - `built`, addressed `to sean` - means *the code lane says it is done and nobody has
/// looked*. Reading only `open` made that item vanish from the index at exactly the moment
/// it started waiting on somebody. `CLAUDE.md` records the consequence: five capabilities
/// could never move while `pending.md` reported that nothing needed deciding.
///
/// **Written against text rather than against the live outboxes, deliberately.** Nothing
/// carries `built` today - all six capabilities are `vetted` or `open` - so a test reading
/// the real files would pass without exercising the case at all, and would go on passing
/// after somebody reintroduced the bug. The states are written out here instead.
#[test]
fn a_capability_that_is_built_is_still_waiting_on_somebody() {
    let release = "\
### R-9 - something not built yet

**to** code · **status** open · **vetted when** somebody looks

### R-10 - something built and unlooked-at

**to** sean · **status** **built** 2026-09-03 · **evidence** it exists

### R-11 - something a person has observed

**to** sean · **status** **vetted** 2026-09-03 · **evidence** it held
";
    let items = outbox::parse(release, "releases/first-release.md");
    assert_eq!(items.len(), 3, "three capabilities written, three read");

    let waiting: Vec<&str> = items
        .iter()
        .filter(|item| item.is_outstanding())
        .map(|item| item.id.as_str())
        .collect();
    assert_eq!(
        waiting,
        ["R-9", "R-10"],
        "a capability is outstanding while it is open and while it is built; only a person \
         setting it vetted ends that"
    );

    // The half that would have caught the original bug on its own.
    let built = items
        .iter()
        .find(|item| item.id == "R-10")
        .expect("written above");
    assert!(
        built.is_outstanding(),
        "`built` means nobody has looked yet, so it is the one state that most needs to be \
         visible - it is where every capability vetted by looking waits"
    );
    assert_eq!(
        built.to, "sean",
        "and it waits on a person, not on the code lane"
    );
}

/// The bullet allowance permits a bullet and a full stop, and nothing else.
///
/// **`P-257` was reported missing for a day while being correct**, because one approved
/// block became four bullets and the checker stripped only the last period. Widening it
/// risks the opposite failure - an allowance that swallows a real difference - so this
/// drives both sides of it rather than trusting that the real case passing means anything.
#[test]
fn a_bullet_and_its_full_stop_may_move_and_nothing_else_may() {
    let approved = "One sentence here. **Two** sentences here.";

    // The same words as one paragraph, as two bullets, and wrapped: all the same text.
    for landed in [
        "One sentence here. **Two** sentences here.",
        "- One sentence here\n- **Two** sentences here",
        "- One sentence here\n  wrapped oddly\n- **Two** sentences here",
    ] {
        let landed = landed.replace("wrapped oddly", "");
        assert_eq!(
            check("text", approved, &landed),
            Verdict::Landed,
            "bullet-versus-paragraph and wrapping are what a promotion may change: {landed:?}"
        );
    }

    // And what it may not: a word, a comma, or emphasis moving.
    for landed in [
        "- One sentence there\n- **Two** sentences here",
        "- One sentence, here\n- **Two** sentences here",
        "- One sentence here\n- Two **sentences** here",
    ] {
        assert!(
            matches!(check("text", approved, landed), Verdict::Missing { .. }),
            "this is a change to the words and should be reported: {landed:?}"
        );
    }
}

/// A dash wrapped into the middle of a paragraph is punctuation, not a bullet.
///
/// **`P-418` landed exactly and was reported missing**, which is the same failure `P-257`
/// had and a different cause. The offered paragraph ends *...what says which it is** - a
/// selector in a recipe, a description in a state.*, and the two sides wrapped it one word
/// apart: the proposal put ` - a selector` at the start of a line and `spec/console.md` did
/// not. One side therefore parsed to one more sentence than the other, and the comparison
/// - which is a sequence of sentences in order - could not match.
///
/// **Wrapping is the one thing `P-283` says may move**, so a parse that depends on it is
/// wrong however plausible its answer. Both directions are driven here, because the fix
/// could as easily have stopped seeing real bullets.
#[test]
fn a_dash_in_a_wrapped_line_is_not_a_bullet_and_a_real_bullet_still_is() {
    // The same paragraph, wrapped two ways. Only the line breaks differ.
    let one = "**Where the form stands is what says which it is** - a selector in a recipe, a\ndescription in a state.";
    let other = "**Where the form stands is what says which it\nis** - a selector in a recipe, a description in a state.";
    assert_eq!(
        sentences(one),
        sentences(other),
        "the same paragraph wrapped two ways parses two ways, so a promotion's verdict \
         depends on where a line broke"
    );
    assert_eq!(
        sentences(one).len(),
        1,
        "it is one sentence however it wraps: {:?}",
        sentences(one)
    );

    // And a list is still a list, however many items and however they wrap.
    let list = "- one thing\n- another thing\n  wrapped\n- a third";
    assert_eq!(
        sentences(list),
        vec![
            "one thing".to_string(),
            "another thing wrapped".to_string(),
            "a third".to_string()
        ],
        "consecutive bullets are separate sentences"
    );

    // A list that opens a block after a paragraph opens on its own line, not inside one.
    let mixed = "A paragraph that ends here.\n\n- one thing\n- another";
    assert_eq!(
        sentences(mixed),
        vec![
            "A paragraph that ends here".to_string(),
            "one thing".to_string(),
            "another".to_string()
        ],
        "a blank line closes the paragraph and the list that follows is a list"
    );
}

/// The one case the loose comparison admitted, which is `C-35`'s whole verification.
///
/// **Every other case in this file passed before the parse replaced the loosening.** This
/// one did not, and the second assertion is why: it runs the predecessor's normalization on
/// the same two strings and shows them coming out equal. A refactor with no new check is
/// unverified, and the check that says this work happened is a check that fails on the old
/// code and passes on the new one.
///
/// Written as its own test rather than folded into the six above so that deleting it is a
/// deliberate act rather than an edit to a list.
#[test]
fn a_period_deleted_mid_paragraph_is_a_change_to_the_words() {
    let approved = "One sentence here. **Two** sentences here.";
    // Nothing became a bullet, so nothing licensed the sentence break to go. The words are
    // identical and the sentences are not, which is the distinction `P-283` draws.
    let run_together = "One sentence here **Two** sentences here.";
    assert!(
        matches!(
            check("text", approved, run_together),
            Verdict::Missing { .. }
        ),
        "a closing period may only go where bullet-versus-paragraph took it"
    );

    // The predecessor, quoted as code so the claim is demonstrated rather than asserted: it
    // deleted every `. ` from both sides before comparing, which makes these two equal.
    let loosened = |text: &str| {
        flat(text)
            .replace(". ", " ")
            .trim_end_matches('.')
            .to_string()
    };
    assert_eq!(
        loosened(approved),
        loosened(run_together),
        "the comparison this replaced could not tell these apart"
    );
}

/// The structure the parse has to know about, all of it found by running the check.
///
/// **Each case is one correct promotion the first attempt reported as missing**, and each
/// is written as a pair rather than as prose so that the next rewrite has to keep it
/// passing. Without the rule under test, the second column would collapse into one entry -
/// which is what the earlier attempt's comparison saw.
#[test]
fn the_structure_a_promotion_may_change_is_parsed_rather_than_stripped() {
    let cases: [(&str, &[&str]); 5] = [
        // A blank line ends a block, so two paragraphs are never one sentence - and a
        // paragraph need not end in a period for that to be true.
        (
            "A paragraph with no full stop\n\nAnother one",
            &["A paragraph with no full stop", "Another one"],
        ),
        // A line break inside a paragraph is not a boundary, because wrapping is the first
        // thing a promotion may change.
        (
            "Two lines with\nno period between them",
            &["Two lines with no period between them"],
        ),
        // A quotation may open `> ` and then `- ` on one line. Stripping one marker leaves
        // the other, and the bullets read as a single run-on sentence.
        (
            "> - A bullet inside a quotation\n> - And a second one",
            &["A bullet inside a quotation", "And a second one"],
        ),
        // A heading bounds the prose under it and is a sentence itself, which is also how a
        // promotion that changed the heading level matches.
        (
            "### A heading\nProse under it",
            &["A heading", "Prose under it"],
        ),
        // A numbered marker is a bullet. Read as prose, `1. ` closes a sentence whose whole
        // content is the number.
        (
            "1. First item\n2. Second item",
            &["First item", "Second item"],
        ),
    ];
    assert_eq!(
        cases.len(),
        5,
        "five structural rules, every one of them found by running the check against the \
         real queue rather than by designing for it"
    );
    for (text, want) in cases {
        assert_eq!(sentences(text), want, "parsing {text:?}");
    }
}

/// An addressing line is dropped, and a word smuggled beside one is still caught.
///
/// **The narrowing has to be narrow, and this is what says so.** Dropping a line by shape is
/// exactly the move that turns a check into a formality, so the case that must still fail is
/// checked beside the case that must now pass.
#[test]
fn an_addressing_line_is_not_prose_and_everything_around_it_still_is() {
    let offered = "### R-10 - I can read a generated drawing\n\n\
                   - **In** - `docs/process.md`, applied to a drawing\n\
                   - **Vetted when** - every drawing is legible in both\n";

    // As a promotion writes it: the required addressing line between the two.
    let landed = "### R-10 - I can read a generated drawing\n\n\
                  **to** code - **status** open - **raised** 2026-09-11 - **cited** `7b4761f`\n\n\
                  - **In** - `docs/process.md`, applied to a drawing\n\
                  - **Vetted when** - every drawing is legible in both\n";
    assert_eq!(
        check("text", offered, landed),
        Verdict::Landed,
        "the approved words are all there and the addressing line is the format `CLAUDE.md` \
         requires of a capability"
    );

    // **A word changed in the approved prose is still a change**, addressing line or not.
    let altered = landed.replace("legible in both", "legible in one");
    assert!(
        matches!(check("text", offered, &altered), Verdict::Missing { .. }),
        "a word was changed beside an addressing line and the check no longer sees it"
    );

    // **And a line that merely opens bold is not an addressing line.** Matching on `**to**`
    // alone would swallow ordinary prose, which is how a narrow rule becomes a wide one.
    let prose = landed.replace(
        "**to** code - **status** open - **raised** 2026-09-11 - **cited** `7b4761f`",
        "**to** the reader this matters because the drawing is the artifact",
    );
    assert!(
        matches!(check("text", offered, &prose), Verdict::Missing { .. }),
        "a sentence opening `**to**` was dropped as bookkeeping; it carries no `**status**` \
         and is prose"
    );
}

/// A proposal landing in two files is read, and a word wrong in either is still caught.
///
/// **`CLAUDE.md`: a proposal that lands in more than one file carries one quotation for each,
/// in the order the destinations are named.** `P-365` obeyed that and was reported unreadable
/// for it - the check read one destination, saw two blocks, and refused.
///
/// **The danger in pairing is that it waves things through.** A version that checked the first
/// pair and stopped, or that asked only whether each block landed *somewhere*, would read as
/// fixed and be worth nothing. So each half is poisoned separately, and the second one matters
/// most: it is the one a loop that forgot to keep going would miss.
#[test]
fn a_proposal_landing_in_two_files_is_checked_against_both() {
    let into = "`spec/orbit.md` -> Crossing between layers, then `spec/units.md` -> Every unit";
    assert_eq!(
        destinations(into),
        ["spec/orbit.md", "spec/units.md"],
        "both places, in the order they are named"
    );

    // **`P-404`: two sections of one file are two places.** The file carries forward and the
    // section does not, so four sections of the release are four destinations against one path
    // - which is what `P-399` is and what made it unreadable under the old rule.
    assert_eq!(
        destinations(
            "`releases/first-release.md` -> Kinds, Traits, Where things are, then Recipes"
        ),
        ["releases/first-release.md"; 4],
        "four sections of one file are four places"
    );

    // A section named after `->` is not a file, and nothing backticked there should be read
    // as one.
    assert_eq!(
        destinations("`CLAUDE.md` -> Promotion"),
        ["CLAUDE.md"],
        "one file and one section is one place"
    );

    // **A backticked thing that is not a path is not a place.** `P-383`'s `into` ends
    // `· from `Q-59``, and reading that as a destination would ask for a third quotation.
    assert_eq!(
        destinations("`docs/process.md` -> Who writes what, then All lanes · from `Q-59`"),
        ["docs/process.md"; 2],
        "an item id in the field is not a file"
    );

    let first = "- An orbit boundary is one an orbit is on either side of\n";
    let second = "- A mobile unit that moves over the ground has a bin for fuel\n";

    // Both landed, each in its own file.
    assert_eq!(check("text", first, first), Verdict::Landed);
    assert_eq!(check("text", second, second), Verdict::Landed);

    // **The second half wrong is the case a short-circuiting pair would miss.** Checked
    // through `check` directly, because the loop above is what pairs them and this is what
    // says a wrong block is still a wrong block.
    assert!(
        matches!(
            check(
                "text",
                second,
                "- A mobile unit that moves over the ground has no fuel\n"
            ),
            Verdict::Missing { .. }
        ),
        "a word changed in the second file's quotation is not caught"
    );

    // **And a count that matches neither one nor the destinations stays unreadable**, rather
    // than being paired off against whatever happens to be there.
    assert_eq!(
        destinations("`spec/orbit.md` -> Crossing").len(),
        1,
        "one destination, so two blocks against it is not a pairing"
    );
}
