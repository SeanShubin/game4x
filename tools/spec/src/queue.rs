//! The operations that move a proposal out of the queue, as named things rather than as a
//! script written fresh each time.
//!
//! **Every function here exists because this lane did it by hand and got it wrong on
//! 2026-09-11.** Three failures in one session, all of the same shape - a structural edit to
//! markdown, hand-rolled, with the boundary or the newline decided anew each time:
//!
//! - A block was removed up to the next heading *of either level*, and a proposal's own `##`
//!   sub-headings are not boundaries. Five orphaned headings and sixty-eight lines survived
//!   in the `Open` section, quotations included. **Fixed once, then reproduced two hours
//!   later by reusing the same logic.**
//! - Text was extracted to a file opened without `newline=""`, so on Windows every `\n`
//!   became `\r\n` and six carriage returns went into `spec/invariants.md`.
//! - A replacement ending in a newline was pasted after an anchor ending in a newline, and
//!   the blank line that produced had to be repaired by hand after nearly every edit.
//!
//! **None of the three was caught by a check**, because each check compared what was written
//! to what the script had just written - `docs/process.md`: *a check that reads a copy of the
//! population is checking the copy.*

use crate::Problem;

/// One `Problem`, from the crate root, because two error types for one tool would be two
/// things to agree about - the same reason `tools/spec` reads `outbox`'s parser rather than
/// writing a second one.
fn problem<T>(message: impl Into<String>) -> Result<T, Problem> {
    Err(Problem(message.into()))
}

/// The top-level sections of `docs/notes/proposals.md`, by name.
///
/// **This list is the fix for the boundary defect and the reason it is a list rather than a
/// rule.** A proposal's body uses `##` for its own sub-headings - *What this changes beyond
/// the word*, *Why this asks you rather than just landing* - so `##` cannot mark where an
/// item ends. Nothing in the text distinguishes the two, so the file's own sections are
/// named and everything else at that level belongs to whatever item it sits under.
pub const FILE_SECTIONS: [&str; 8] = [
    "## The files that need you",
    "## How this works",
    "## Open",
    "## Addressed to other perspectives",
    "## Accepted",
    "## Forecast cleanups that were checked and not filed",
    "## Rejected",
    "## Withdrawn",
];

/// Where an item's block starts and ends, in lines.
///
/// The end is the next `### ` heading, or the next of the file's own sections - never a
/// `##` heading belonging to the item itself.
pub fn block_of(text: &str, id: &str) -> Result<(usize, usize), Problem> {
    let lines: Vec<&str> = text.lines().collect();
    let heading = format!("### {id} - ");
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.starts_with(&heading))
        .map(|(at, _)| at)
        .collect();
    match starts.as_slice() {
        [] => return problem(format!("{id} has no heading in this file")),
        [_] => {}
        several => {
            return problem(format!(
                "{id} has {} headings, so which one to move is a guess",
                several.len()
            ));
        }
    }
    let start = starts[0];
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, line)| line.starts_with("### ") || FILE_SECTIONS.contains(&line.trim()))
        .map(|(at, _)| at)
        .unwrap_or(lines.len());
    Ok((start, end))
}

/// Remove one item's block, refusing if the range holds anything that is not its own.
///
/// **The refusal is the point.** `CLAUDE.md` records two proposals destroyed by deleting a
/// range between two markers without counting what was inside it.
pub fn remove_block(text: &str, id: &str) -> Result<String, Problem> {
    let (start, end) = block_of(text, id)?;
    let lines: Vec<&str> = text.lines().collect();
    let inside = &lines[start + 1..end];
    let other: Vec<&&str> = inside
        .iter()
        .filter(|line| line.starts_with("### "))
        .collect();
    if !other.is_empty() {
        return problem(format!(
            "{id}'s range holds {} other item(s): {:?}",
            other.len(),
            other
        ));
    }
    let mut kept: Vec<&str> = Vec::with_capacity(lines.len());
    kept.extend_from_slice(&lines[..start]);
    kept.extend_from_slice(&lines[end..]);
    Ok(join(&kept))
}

/// Insert a ledger row after the row whose first cell begins with `after`.
///
/// **The row is located by a prefix and rebuilt; it is never matched whole.** `CLAUDE.md`:
/// *never put a table row in a match string* - the padder rewrites column widths, so a
/// pattern that matched yesterday silently stops matching, and thirteen rows went missing
/// that way.
///
/// **The count is asserted rather than the write trusted**, because a `replace` that matches
/// nothing is a no-op rather than an error.
pub fn insert_ledger_row(text: &str, after: &str, row: &str) -> Result<String, Problem> {
    if !row.starts_with("| ") || !row.ends_with(" |") {
        return problem(format!("a ledger row is written with pipes: {row:?}"));
    }
    let lines: Vec<&str> = text.lines().collect();
    let before = lines.iter().filter(|line| line.starts_with("| P-")).count();
    let prefix = format!("| {after},");
    let at: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.starts_with(&prefix))
        .map(|(index, _)| index)
        .collect();
    match at.as_slice() {
        [] => return problem(format!("no ledger row begins {prefix:?}")),
        [_] => {}
        several => {
            return problem(format!(
                "{} ledger rows begin {prefix:?}, so where to insert is a guess",
                several.len()
            ));
        }
    }
    let mut out: Vec<&str> = lines.clone();
    out.insert(at[0] + 1, row);
    let after_count = out.iter().filter(|line| line.starts_with("| P-")).count();
    if after_count != before + 1 {
        return problem(format!(
            "the ledger went {before} -> {after_count}, expected one more"
        ));
    }
    Ok(join(&out))
}

/// Put `text` in after the one line that equals `anchor`, without the blank line.
///
/// **The blank line is what this is for.** Pasting a replacement that ends in a newline
/// after an anchor that ends in a newline gives two, and this lane repaired that by hand
/// after nearly every edit on 2026-09-11 before noticing it was the same defect each time.
/// Here the lines are joined, so there is no newline to double.
pub fn insert_after(text: &str, anchor: &str, inserted: &str) -> Result<String, Problem> {
    if inserted.contains('\r') {
        return problem("the text to insert holds a carriage return");
    }
    let lines: Vec<&str> = text.lines().collect();
    let at: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.trim_end() == anchor.trim_end())
        .map(|(index, _)| index)
        .collect();
    match at.as_slice() {
        [] => problem(format!("no line equals the anchor {anchor:?}")),
        [_] => {
            let mut out: Vec<&str> = lines.clone();
            // **If the anchor ends a block, what follows it starts one.** `P-443` went in
            // directly under the last line of a paragraph and ran into it, because this
            // inserted on the next line whatever was there. The line after the anchor says
            // which case it is: blank means the anchor closed a block, and a new block then
            // needs its own blank line before it.
            let mut offset = 1;
            if lines
                .get(at[0] + 1)
                .is_some_and(|next| next.trim().is_empty())
            {
                out.insert(at[0] + 1, "");
                offset = 2;
            }
            for (index, line) in inserted.lines().enumerate() {
                out.insert(at[0] + offset + index, line);
            }
            Ok(join(&out))
        }
        several => problem(format!(
            "{} lines equal the anchor, so where to insert is a guess",
            several.len()
        )),
    }
}

/// Replace a contiguous run of lines with `text`, matching on their collapsed whitespace.
///
/// **Every promotion on 2026-09-11 and 2026-09-12 was a replacement and this verb did not
/// exist**, so each went through the anchor tool with a hand-written replacement file - and
/// both of the day's hygiene slips were in that path, a trailing newline kept twice and a
/// blank line left behind twice.
///
/// **The run is matched with whitespace collapsed on both sides**, so a wrap can hide
/// nothing. `CLAUDE.md`, from Sean: *I typically resolve this by normalizing things before I
/// compare them.* A match string drafted as one sentence, meeting a file that broke it across
/// two lines, is the failure this removes rather than asks the caller to avoid.
pub fn replace_run(text: &str, old: &str, new: &str) -> Result<String, Problem> {
    if new.contains('\r') {
        return problem("the replacement holds a carriage return");
    }
    let lines: Vec<&str> = text.lines().collect();
    let wanted: Vec<&str> = old.lines().filter(|line| !line.trim().is_empty()).collect();
    if wanted.is_empty() {
        return problem("nothing to replace: the text to match is blank");
    }
    let target = collapse(&wanted.join(" "));

    // **How many lines the run takes in the file is not how many it takes in the proposal.**
    // A sentence offered on one line and wrapped onto two is the case this verb exists for,
    // so the run is grown a line at a time until its collapsed text matches - and abandoned
    // once it is longer than the target, which it can never shrink back under.
    let mut found: Vec<(usize, usize)> = Vec::new();
    for start in 0..lines.len() {
        for end in start + 1..=lines.len() {
            let run = collapse(&lines[start..end].join(" "));
            if run == target {
                found.push((start, end));
                break;
            }
            if run.len() >= target.len() {
                break;
            }
        }
    }
    match found.as_slice() {
        [] => problem(format!(
            "no run of lines matches the {} offered, with whitespace collapsed",
            wanted.len()
        )),
        [(start, end)] => {
            let mut out: Vec<&str> = Vec::with_capacity(lines.len());
            out.extend_from_slice(&lines[..*start]);
            out.extend(new.lines());
            out.extend_from_slice(&lines[*end..]);
            Ok(join(&out))
        }
        several => problem(format!(
            "{} runs match, so which one to replace is a guess",
            several.len()
        )),
    }
}

/// What an empty `Open` section says, so that emptiness is stated rather than inferred.
pub const NOTHING_OPEN: &str = "*Nothing is open. Everything filed has been decided.*";

/// Make the sentence agree with the section, in both directions.
///
/// **The section saying nothing and the section saying it is empty are different claims**,
/// and only the second one a reader can act on. Landing `P-434` left `## Open` followed by a
/// blank line and the next section, which reads as *this has not been looked at* rather than
/// *there is nothing here*.
///
/// **It wrote the sentence and never took it out, which made it a lie twice in one day.**
/// `P-455` was filed under a standing sentinel, and so was `P-456` an hour after the gate
/// check for it was added. **A sentence only ever written says something true once and then
/// goes on saying it**, which is the same shape as the absence assertions the code lane
/// retired the same morning - and the fix is the same one: derive it from the section rather
/// than remember to change it.
pub fn say_if_empty(text: &str) -> Result<String, Problem> {
    let lines: Vec<&str> = text.lines().collect();
    let Some(open) = lines.iter().position(|line| line.trim() == "## Open") else {
        return problem("this file has no Open section");
    };
    let Some(next) = lines
        .iter()
        .enumerate()
        .skip(open + 1)
        .find(|(_, line)| FILE_SECTIONS.contains(&line.trim()))
        .map(|(at, _)| at)
    else {
        return problem("no section follows Open");
    };
    let inside: Vec<&&str> = lines[open + 1..next]
        .iter()
        .filter(|line| !line.trim().is_empty())
        .collect();
    if !inside.is_empty() {
        // **And it takes the sentinel out again, which is the half that was missing.** This
        // only ever wrote it, so the sentence went in when the last item landed and stayed
        // when the next one was filed - and for an hour on 2026-09-12 the queue said nothing
        // was open with `P-456` standing three lines below the sentence. Twice in one day,
        // because filing is done by hand and no verb ran then.
        if !inside.iter().any(|line| line.trim() == NOTHING_OPEN) {
            return Ok(text.to_string());
        }
        let mut out: Vec<&str> = Vec::with_capacity(lines.len());
        out.extend_from_slice(&lines[..open + 1]);
        let mut kept: Vec<&str> = lines[open + 1..next]
            .iter()
            .copied()
            .filter(|line| line.trim() != NOTHING_OPEN)
            .collect();
        while kept.first().is_some_and(|line| line.trim().is_empty()) {
            kept.remove(0);
        }
        out.push("");
        out.extend_from_slice(&kept);
        out.extend_from_slice(&lines[next..]);
        return Ok(join(&out));
    }
    let mut out: Vec<&str> = Vec::with_capacity(lines.len() + 3);
    out.extend_from_slice(&lines[..open + 1]);
    out.push("");
    out.push(NOTHING_OPEN);
    out.push("");
    // From `next`, not `next - 1`: the blank line before the following section is the one
    // just pushed, and keeping both is how this verb would put back the defect it removes.
    out.extend_from_slice(&lines[next..]);
    Ok(join(&out))
}

/// Whether the approved text is in the destination, compared with whitespace collapsed.
///
/// **Normalized on both sides rather than matched carefully.** `CLAUDE.md`, from Sean:
/// *I typically resolve this by normalizing things before I compare them.* A wrap can then
/// hide nothing, which is what a match string drafted on one line and met by a file that
/// broke it across two cannot promise.
pub fn lands_once(destination: &str, approved: &str) -> Result<(), Problem> {
    let found = collapse(destination).matches(&collapse(approved)).count();
    match found {
        1 => Ok(()),
        0 => problem("the approved text is not in the destination"),
        several => problem(format!(
            "the approved text is in the destination {several} times"
        )),
    }
}

/// Collapse every run of whitespace to one space, so wrapping cannot hide a difference.
pub fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Join lines with `\n` and end with one, whatever the platform thinks.
///
/// **Named and used everywhere rather than left to the default**, because the default put
/// six carriage returns into `spec/invariants.md` on 2026-09-11 and the check that was
/// supposed to catch it normalized whitespace, so `\r` passed.
fn join(lines: &[&str]) -> String {
    let mut out = lines.join("\n");
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A proposal's own `##` sub-headings are not where its block ends.
    ///
    /// **This is the test that would have failed before**, twice: the hand-written removal
    /// cut at the first `## ` and left sixty-eight lines of two proposals in the `Open`
    /// section on 2026-09-11, quotations included, after the same defect had been found and
    /// fixed two hours earlier.
    #[test]
    fn a_block_ends_at_the_next_item_and_not_at_its_own_sub_heading() {
        let text = "\
## Open

### P-1 - a title

**to** sean

## Why this asks you rather than just landing

Some reasoning, which belongs to P-1.

> the approved words

### P-2 - another

**to** sean
";
        let (start, end) = block_of(text, "P-1").expect("P-1 is there");
        let block: Vec<&str> = text.lines().collect::<Vec<&str>>()[start..end].to_vec();
        assert!(
            block
                .iter()
                .any(|line| line.starts_with("## Why this asks")),
            "the sub-heading belongs to P-1's block: {block:?}"
        );
        assert!(
            block.iter().any(|line| line.starts_with("> ")),
            "and so does the quotation under it: {block:?}"
        );
        assert_eq!(
            block.iter().filter(|line| line.starts_with("### ")).count(),
            1,
            "and P-2 does not"
        );
    }

    /// And removing it leaves nothing of it behind.
    #[test]
    fn removing_a_block_takes_its_sub_headings_with_it() {
        let text = "\
## Open

### P-1 - a title

## Why this asks you

> the approved words

### P-2 - another
";
        let left = remove_block(text, "P-1").expect("removable");
        assert!(!left.contains("Why this asks you"), "{left}");
        assert!(!left.contains("> the approved words"), "{left}");
        assert!(left.contains("### P-2 - another"), "{left}");
    }

    /// Never delete a range without knowing what is inside it.
    #[test]
    fn a_range_holding_another_item_is_refused() {
        let text = "### P-1 - one\n\n### P-2 - two\n";
        // P-1's block ends at P-2, so this is well-formed; the guard fires when a heading
        // is *inside* the range, which happens when an id is duplicated.
        let twice = "### P-1 - one\n\n### P-1 - one again\n";
        assert!(block_of(twice, "P-1").is_err(), "two headings is a guess");
        assert!(remove_block(text, "P-1").is_ok());
    }

    /// The ledger grows by exactly one row, and the row is never matched whole.
    #[test]
    fn a_ledger_row_goes_in_after_the_one_named_and_the_count_says_so() {
        let text = "| P-1, a thing | `spec/x.md` | 2026-09-01 |\n| P-2, another | `spec/y.md` | 2026-09-02 |\n";
        let out = insert_ledger_row(text, "P-1", "| P-3, a third | `spec/z.md` | 2026-09-11 |")
            .expect("inserted");
        let rows: Vec<&str> = out.lines().filter(|l| l.starts_with("| P-")).collect();
        assert_eq!(rows.len(), 3, "{out}");
        assert_eq!(rows[1], "| P-3, a third | `spec/z.md` | 2026-09-11 |");
    }

    /// Padding changes a row's bytes, so a prefix locates it and the row is rebuilt.
    ///
    /// **`CLAUDE.md` records thirteen rows going missing** to a match string that held a
    /// padded row. This passes over a padded table because nothing is matched whole.
    #[test]
    fn padding_does_not_stop_a_ledger_row_being_found() {
        let padded = "| P-1, a thing   | `spec/x.md`   | 2026-09-01 |\n";
        assert!(insert_ledger_row(padded, "P-1", "| P-2, x | `y` | z |").is_ok());
    }

    /// Inserting after an anchor does not leave a blank line behind.
    #[test]
    fn inserting_after_a_line_adds_no_blank_line() {
        let text = "one\ntwo\nthree\n";
        let out = insert_after(text, "two", "TWO AND A HALF").expect("inserted");
        assert_eq!(out, "one\ntwo\nTWO AND A HALF\nthree\n", "{out:?}");
    }

    /// A block inserted after a line that ends one gets its own blank line.
    ///
    /// **`P-443` needed this and did not have it.** It went in directly under the last line
    /// of a paragraph in `spec/console.md` and ran into it, and the assertion passed because
    /// the approved text was all there - `docs/process.md`'s point about a check answering a
    /// narrower question than the one asked.
    #[test]
    fn a_block_after_a_paragraph_gets_a_blank_line_before_it() {
        let text = "a paragraph.

another one.
";
        let out = insert_after(text, "a paragraph.", "**new block.**").expect("inserted");
        assert_eq!(
            out,
            "a paragraph.

**new block.**

another one.
",
            "{out:?}"
        );
    }

    /// And a line inserted inside a block does not gain one.
    #[test]
    fn a_line_inside_a_block_gets_no_blank_line() {
        let text = "- one
- two
- three
";
        let out = insert_after(text, "- two", "- two and a half").expect("inserted");
        assert_eq!(
            out,
            "- one
- two
- two and a half
- three
",
            "{out:?}"
        );
    }

    /// Two anchors is a guess, not a first match.
    #[test]
    fn an_anchor_matching_twice_is_refused() {
        let text = "same\nsame\n";
        assert!(insert_after(text, "same", "x").is_err());
    }

    /// A carriage return never reaches a specification file through this tool.
    ///
    /// **The test the CRLF defect needed.** The check that was supposed to catch it
    /// collapsed whitespace, and `\r` is whitespace - so it passed while six carriage
    /// returns sat in `spec/invariants.md`.
    #[test]
    fn text_carrying_a_carriage_return_is_refused_although_a_whitespace_check_would_pass() {
        let text = "one\ntwo\n";
        let dirty = "a line\r\nand another";
        assert!(
            insert_after(text, "one", dirty).is_err(),
            "refused outright"
        );
        assert_eq!(
            collapse(dirty),
            collapse("a line\nand another"),
            "and a whitespace comparison cannot tell them apart, which is why it is refused rather than checked"
        );
    }

    /// An emptied queue says so, rather than falling silent.
    ///
    /// **This is the one the integration test found**, on the live file, after landing the
    /// last proposal left `## Open` followed by nothing at all.
    #[test]
    fn landing_the_last_item_leaves_the_section_saying_it_is_empty() {
        let emptied = "## Open

## Accepted
";
        let said = say_if_empty(emptied).expect("a section");
        assert!(said.contains(NOTHING_OPEN), "{said:?}");
        assert_eq!(
            said,
            format!(
                "## Open

{NOTHING_OPEN}

## Accepted
"
            ),
            "{said:?}"
        );
    }

    /// And a section with something in it is left exactly alone.
    #[test]
    fn a_queue_with_an_item_in_it_is_untouched() {
        let held = "## Open

### P-1 - a title

## Accepted
";
        assert_eq!(say_if_empty(held).expect("unchanged"), held);
    }

    /// And a section with both the sentinel and an item loses the sentinel.
    ///
    /// **This is the case that happened twice on 2026-09-12** and that the one-directional
    /// version could not fix: an item filed by hand under a sentence saying nothing is open.
    #[test]
    fn a_queue_that_says_it_is_empty_and_is_not_loses_the_sentence() {
        let lying = format!(
            "## Open

{NOTHING_OPEN}

### P-1 - a title

## Accepted
"
        );
        let said = say_if_empty(&lying).expect("a section");
        assert!(!said.contains(NOTHING_OPEN), "{said:?}");
        assert_eq!(
            said,
            "## Open

### P-1 - a title

## Accepted
",
            "{said:?}"
        );
    }

    /// Approved text is found however the destination wrapped it.
    #[test]
    fn a_wrap_cannot_hide_the_approved_text() {
        let approved = "a sentence that will be wrapped somewhere";
        let destination = "before\na sentence that will be\nwrapped somewhere\nafter\n";
        assert!(lands_once(destination, approved).is_ok());
    }

    /// A replacement leaves no blank line behind, which is what `after` had to be taught.
    ///
    /// **Both of 2026-09-12's hygiene slips were this**, in a path that had no verb: a
    /// trailing newline kept after the bullet, and another before `### Yard`.
    #[test]
    fn replacing_a_run_leaves_no_blank_line() {
        let text = "before
- old first
  old second
after
";
        let out = replace_run(
            text,
            "- old first
  old second",
            "- new one line",
        )
        .expect("replaced");
        assert_eq!(
            out,
            "before
- new one line
after
",
            "{out:?}"
        );
    }

    /// A wrap cannot stop a run being found, because both sides are collapsed.
    #[test]
    fn a_run_wrapped_differently_is_still_matched() {
        let text = "a
- one sentence that
  wrapped here
b
";
        let out = replace_run(text, "- one sentence that wrapped here", "- gone").expect("matched");
        assert_eq!(
            out,
            "a
- gone
b
",
            "{out:?}"
        );
    }

    /// Two matching runs is a guess, and a guess is refused.
    #[test]
    fn a_run_matching_twice_is_refused() {
        assert!(
            replace_run(
                "x
x
", "x", "y"
            )
            .is_err()
        );
    }

    /// A run that is not there fails loudly rather than doing nothing.
    ///
    /// **`CLAUDE.md`: a `str.replace` with no match is a no-op rather than an error**, and
    /// thirteen rows went missing that way.
    #[test]
    fn a_run_that_is_absent_is_an_error_rather_than_a_no_op() {
        assert!(
            replace_run(
                "a
b
", "not here", "z"
            )
            .is_err()
        );
    }

    /// And a carriage return never reaches a specification file through this verb either.
    #[test]
    fn a_replacement_carrying_a_carriage_return_is_refused() {
        let dirty = concat!("one", "\r\n", "two");
        assert!(dirty.contains('\r'), "the fixture really holds one");
        assert!(replace_run("a\nb\n", "a", dirty).is_err());
    }

    /// And text that is not there fails loudly rather than quietly.
    #[test]
    fn text_that_did_not_land_is_an_error_rather_than_a_no_op() {
        assert!(lands_once("nothing like it\n", "the approved words").is_err());
        assert!(
            lands_once("twice: x. and x.\n", "x.").is_err(),
            "twice is also wrong"
        );
    }
}
