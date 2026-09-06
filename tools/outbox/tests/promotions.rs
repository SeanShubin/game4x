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
    let start = into.find('`')? + 1;
    let end = into[start..].find('`')? + start;
    Some(into[start..end].to_string())
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
pub fn sentences(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut block = String::new();
    for line in text.lines() {
        let mut rest = line.trim();
        // A quotation marker is not prose, and it may be followed by another marker.
        while let Some(inner) = rest.strip_prefix('>') {
            rest = inner.trim_start();
        }
        if rest.is_empty() {
            drain(&mut block, &mut out);
            continue;
        }
        // A heading bounds the prose on both sides of it and is a sentence of its own, so
        // that a promotion which changed the heading level still matches.
        if let Some(heading) = rest.strip_prefix('#') {
            drain(&mut block, &mut out);
            block.push_str(heading.trim_start_matches('#').trim());
            drain(&mut block, &mut out);
            continue;
        }
        if let Some(inner) = bullet(rest) {
            drain(&mut block, &mut out);
            rest = inner;
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

        let gone: Vec<outbox::Item> = outbox::parse(&before, "docs/notes/proposals.md")
            .into_iter()
            .filter(|item| item.id.starts_with("P-") && item.is_outstanding())
            .filter(|item| !after.contains(&format!("### {} ", item.id)))
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
            let Some(shape) = field(&item.body, "shape") else {
                older += 1;
                continue;
            };
            let Some(into) = field(&item.body, "into").and_then(|i| destination_file(&i)) else {
                wrong.push(format!("{}: no readable **into** field", item.id));
                continue;
            };
            // **An instruction carries no text that lands**, so it needs no block at all -
            // `P-222` had none and `P-220` had a before and an after. The destination check
            // below already declines to ask anything of an instruction; requiring a block
            // first meant refusing to read the ones that were correct.
            let quoted = blocks(&item.body);
            if shape != "instruction" && quoted.is_empty() {
                wrong.push(format!("{}: shape {shape} and no block to land", item.id));
                continue;
            }
            let Some(destination) = git(&root, &["show", &format!("{commit}:{into}")]) else {
                wrong.push(format!("{}: {into} is not in {}", item.id, &commit[..7]));
                continue;
            };
            checked += 1;
            let mut verdict = match quoted.len() {
                0 => Verdict::Landed, // an instruction; nothing lands verbatim
                1 => check(&shape, &quoted[0], &destination),
                _ => Verdict::Ambiguous,
            };
            // If it did not land then, ask whether it has landed since. Only a `Missing` is
            // worth re-asking: an unknown shape is unknown at every commit.
            if matches!(verdict, Verdict::Missing { .. })
                && let Some(now) = git(&root, &["show", &format!("HEAD:{into}")])
                && quoted
                    .iter()
                    .all(|block| matches!(check(&shape, block, &now), Verdict::Landed))
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
