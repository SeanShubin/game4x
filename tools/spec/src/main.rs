//! Promoting a proposal, as one command rather than a script written fresh each time.
//!
//! ```text
//! spec show <id>[:n]                     the approved text, exactly as it will land
//! spec after <id>[:n] <file> <anchor>    put it in after that line, and assert it landed
//! spec replacing <id>[:n] <file> <old>   put it in where those lines are, and assert it landed
//! spec land <id> <after>                 ledger row from the item's own `into`, then remove it
//! spec file <path>                       put a drafted item at the top of Open
//! spec touching <file>                   every open item in every outbox that names it
//! ```
//!
//! **`touching` is the promotion rule's other half**, which `CLAUDE.md` calls reading the
//! index: a rule that moves under an open item makes it wrong without touching it, and
//! nothing else will notice.
//!
//! **`file` was the last step still done by a script written fresh each time**, and it routes
//! by the item's own `asks`: a decision goes to `docs/notes/decisions.md` and approval to the
//! queue, because those two files are that field written down. It also makes the sentinel
//! agree afterwards, which is what a hand-written insert has no reason to think about.
//!
//! **`:n` names which block, and only a proposal offering more than one needs it.**
//! `CLAUDE.md`: *each indented quotation is one block of text being offered, and the proposal
//! says where that one goes* - so two sections is two quotations landing in two places, and the
//! tool refused every one of them until `P-458` was the first. Blocks are numbered from 1 in
//! the order they appear, and they come from [`outbox::Item::proposed_blocks`] - the reader
//! `proposed_text` is itself written in terms of, so there is still no second parser.
//!
//! **The approved text is never handled by a person between the proposal and the file.**
//! `docs/notes/tools-spec-design.md`: *every check compares the edit to what the script
//! intended, and nothing compares the intent to what Sean approved.* `show` and `after` read
//! the same `Item::proposed_text()` that `tools/outbox` parses with, so there is no second
//! parser to disagree with the first.
//!
//! Run from anywhere: the repository root is found from this file's own path.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use spec::queue;

/// Where an item that waits on Sean lives, and where one that waits on an instance does.
///
/// **`decide/` holds only what a person is asked to act on** - Sean, 2026-09-14: *move the
/// documents I need to make decisions on or approve to a directory not mixed in with what is
/// currently settled or historical*. The open queue was 127 lines of a 5,697-line file, and the
/// other 5,570 were the ledger, the closed items and the notices to other instances.
///
/// **A notice is not addressed to him and does not move.** `docs/notes/proposals.md` stays this
/// lane's outbox; `decide/` is not an outbox at all, and no instance files there to be read by
/// another instance.
const QUEUE: &str = "decide/proposals.md";
const QUESTIONS: &str = "decide/questions.md";
const OUTBOX: &str = "docs/notes/proposals.md";

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let root = root();
    let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let outcome = match borrowed.as_slice() {
        ["show", id] => show(&root, id),
        ["after", id, file, anchor] => after(&root, id, file, anchor),
        ["replacing", id, file, old] => replacing(&root, id, file, old),
        ["land", id, previous] => land(&root, id, previous),
        ["file", path] => file(&root, path),
        ["touching", what] => touching(&root, what),
        ["chains"] => chains(&root),
        _ => Err(usage()),
    };
    match outcome {
        Ok(said) => {
            println!("{said}");
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("spec: {why}");
            ExitCode::FAILURE
        }
    }
}

fn usage() -> String {
    [
        "spec show <id>[:n]                     the approved text, exactly as it will land",
        "spec after <id>[:n] <file> <anchor>    put it in after that line, and assert it landed",
        "spec replacing <id>[:n] <file> <old>   put it in where those lines are, and assert it landed",
        "spec land <id> <after-id>              ledger row from the item's own `into`, then remove it",
        "spec file <path>                       put a drafted item at the top of Open",
        "spec touching <file>                   every open item in every outbox that names it",
        "spec chains                            where one item closed by handing off to another",
        "",
        "<id>:n names one of several offered blocks, numbered from 1 in the order they appear.",
    ]
    .join("\n")
}

/// The one proposal with this id, from the queue that `tools/outbox` reads.
fn proposal(root: &Path, id: &str) -> Result<outbox::Item, String> {
    let queue = read(&root.join(QUEUE))?;
    let items = outbox::parse(&queue, QUEUE);
    let mut found: Vec<outbox::Item> = items.into_iter().filter(|item| item.id == id).collect();
    match found.len() {
        1 => Ok(found.remove(0)),
        0 => Err(format!("{id} is not an item in {QUEUE}")),
        several => Err(format!(
            "{id} appears {several} times, so which one is a guess"
        )),
    }
}

/// Split `P-458:2` into the id and which block was asked for.
///
/// **The ordinal rides on the id rather than being a separate argument**, so every verb takes
/// one without growing a parameter, and a caller that never needs one never sees it.
fn addressed(argument: &str) -> Result<(&str, Option<usize>), String> {
    match argument.split_once(':') {
        None => Ok((argument, None)),
        Some((id, which)) => {
            let at: usize = which
                .parse()
                .map_err(|_| format!("{which:?} is not a block number"))?;
            if at == 0 {
                return Err("blocks are numbered from 1".to_string());
            }
            Ok((id, Some(at)))
        }
    }
}

/// What the proposal offers, with its quoting stripped and no carriage return in it.
///
/// **An ordinal is required exactly when there is more than one block, in both directions.** A
/// proposal offering one refuses an ordinal, because a caller that passed one has miscounted
/// and would otherwise be handed the only block there is; and a proposal offering several
/// refuses to guess, which is what it did before it could be told.
fn approved(item: &outbox::Item, which: Option<usize>) -> Result<String, String> {
    let blocks = item
        .proposed_blocks()
        .map_err(|why| format!("{}: {why}", item.id))?;
    let text = match (blocks.len(), which) {
        (1, None) => blocks[0].clone(),
        (1, Some(_)) => return Err(format!("{} offers one block, so drop the `:n`", item.id)),
        (several, None) => {
            return Err(format!(
                "{} offers {several} blocks, so which one is a guess - say {}:1 through {}:{several}",
                item.id, item.id, item.id
            ));
        }
        (several, Some(at)) if at > several => {
            return Err(format!("{} offers {several} blocks, not {at}", item.id));
        }
        (_, Some(at)) => blocks[at - 1].clone(),
    };
    if text.contains('\r') {
        return Err(format!(
            "{}'s approved text holds a carriage return",
            item.id
        ));
    }
    Ok(text)
}

fn show(root: &Path, argument: &str) -> Result<String, String> {
    let (id, which) = addressed(argument)?;
    let item = proposal(root, id)?;
    let text = approved(&item, which)?;
    Ok(format!(
        "{argument}, {} line(s), into {}\n\n{text}",
        text.lines().count(),
        destination(&item).unwrap_or_else(|| "(no **into** field)".to_string())
    ))
}

/// Put the approved text in after one line, then assert it is there.
///
/// **The assertion reads the file back from disk rather than the string just written**,
/// which is the distinction `docs/process.md` draws: *a check that reads a copy of the
/// population is checking the copy.* This lane asserted a promotion against its own
/// intermediate file on 2026-09-11 and a truncated sentence passed.
fn after(root: &Path, argument: &str, file: &str, anchor: &str) -> Result<String, String> {
    let (id, which) = addressed(argument)?;
    let item = proposal(root, id)?;
    let text = approved(&item, which)?;
    let path = root.join(file);
    let before = read(&path)?;
    let written = queue::insert_after(&before, anchor, &text).map_err(|why| why.to_string())?;
    write(&path, &written)?;

    let back = read(&path)?;
    queue::lands_once(&back, &text).map_err(|why| format!("{argument} did not land: {why}"))?;
    if back.contains('\r') {
        return Err(format!("{file} holds a carriage return after the write"));
    }
    Ok(format!(
        "{argument}: {} line(s) into {file} after {anchor:?}, present exactly once, no carriage return",
        text.lines().count()
    ))
}

/// Put the approved text in where some existing lines are, then assert it is there.
///
/// **Every promotion today was a replacement and there was no verb for it**, so each went
/// through the anchor tool with a hand-written replacement file - and both of the day's
/// hygiene slips were in that path.
///
/// `old` is matched with whitespace collapsed, so the caller may give it on one line however
/// the file wrapped it.
fn replacing(root: &Path, argument: &str, file: &str, old: &str) -> Result<String, String> {
    let (id, which) = addressed(argument)?;
    let item = proposal(root, id)?;
    let text = approved(&item, which)?;
    let path = root.join(file);
    let before = read(&path)?;
    let written = queue::replace_run(&before, old, &text).map_err(|why| why.to_string())?;
    write(&path, &written)?;

    let back = read(&path)?;
    queue::lands_once(&back, &text).map_err(|why| format!("{argument} did not land: {why}"))?;
    if back.contains('\r') {
        return Err(format!("{file} holds a carriage return after the write"));
    }
    // **A replacement that keeps what it replaces is the ordinary case, not a failure.**
    // `P-470` offered a paragraph plus one sentence, so the old words are inside the new ones
    // and *the replaced text is gone* can never be true - it fired after writing correctly,
    // which is this tool's third false alarm and the second to cost a reader a doubt about a
    // file that was right. The check is only meaningful when the new text drops the old.
    if !queue::collapse(&text).contains(&queue::collapse(old))
        && queue::collapse(&back).contains(&queue::collapse(old))
    {
        return Err(format!("{file} still holds the text {argument} replaced"));
    }
    Ok(format!(
        "{argument}: {} line(s) into {file}, present exactly once, the replaced text gone, no carriage return",
        text.lines().count()
    ))
}

/// The `**into**` field, which is where the ledger row's destination comes from.
///
/// **Read from the field rather than typed again.** `docs/notes/tools-spec-design.md`: the
/// ledger row's destination *is prose typed by this lane from that field, and the field is
/// deleted with the body at promotion* - so writing it from the field closes a transcription
/// step of the same family as everything else.
fn destination(item: &outbox::Item) -> Option<String> {
    let fields = &item.fields;
    let at = fields.find("**into**")? + "**into**".len();
    let rest = &fields[at..];
    let end = rest.find(" · ").unwrap_or(rest.len());
    Some(rest[..end].trim().to_string())
}

fn land(root: &Path, id: &str, previous: &str) -> Result<String, String> {
    let item = proposal(root, id)?;
    let into = destination(&item)
        .ok_or_else(|| format!("{id} has no **into** field, so the ledger has nowhere to say"))?;
    let title = item
        .title
        .trim()
        .trim_start_matches(|c: char| c == '-' || c.is_whitespace())
        .to_string();
    let row = format!("| {id}, {title} | {into} | {} |", today());

    // **The block and the ledger row are in two files now.** `decide/proposals.md` holds only
    // what waits on Sean, and the ledger of everything accepted is the record rather than a
    // queue - so landing removes from the one and writes to the other, and asserts both.
    // **The row goes in before the block comes out.** Both halves are computed before either
    // file is written, and the ledger is written first - so a bad `previous` refuses with
    // nothing changed. Writing the queue first cost `P-540` its row on 2026-09-23: the item was
    // gone, the insert failed on an id that names no row, and the error spoke only of the ledger.
    let queue_at = root.join(QUEUE);
    let ledger_at = root.join(OUTBOX);
    let (settled, with_row) =
        queue::landing(&read(&queue_at)?, &read(&ledger_at)?, id, previous, &row)
            .map_err(|why| why.to_string())?;
    write(&ledger_at, &with_row)?;
    write(&queue_at, &settled)?;

    if read(&queue_at)?.contains(&format!("### {id} - ")) {
        return Err(format!(
            "{id}'s block is still in the queue after landing it"
        ));
    }
    let rows = read(&ledger_at)?
        .lines()
        .filter(|l| l.starts_with(&format!("| {id},")))
        .count();
    if rows != 1 {
        return Err(format!("{id} has {rows} ledger rows, expected one"));
    }
    Ok(format!(
        "{id} landed: one ledger row into {into}, block removed"
    ))
}

/// Every open item, in every outbox, whose body names this file.
///
/// **`CLAUDE.md` already requires this and calls it reading the index**: *after promoting,
/// check the index for open items that cite the destination file - `outbox` lists them - and
/// tell their owner. A rule that moves under an open item makes it wrong without touching it,
/// and nothing else will notice.* It also says the catch-up list is **read from the index
/// rather than assembled from memory**, which is the only version that stays complete after a
/// lane has been idle through forty promotions.
///
/// **This lane assembled it from a fresh script at every promotion instead**, which is the
/// same failure filing had. The script was right each time and existed only in that turn.
///
/// **What it cannot do is worth saying, because the case that prompted it is one it misses.**
/// `S-97` went stale when `P-421` landed in the release, and `S-97` names no path - it says
/// *the release's recipe table*. **A body that names its subject in prose is invisible to
/// this**, so what it makes findable is the citation and not the subject.
fn touching(root: &Path, what: &str) -> Result<String, String> {
    let all = outbox::read(root);
    let needle = what.trim_start_matches("./");
    let (found, looked) = spec::touching(&all.items, needle);
    if looked == 0 {
        return Err("no outstanding item in any outbox, so this counted over nothing".to_string());
    }
    if found.is_empty() {
        return Ok(format!(
            "nothing outstanding names {needle}, out of {looked} item(s) in {} outbox(es)",
            all.files.len()
        ));
    }
    let said: Vec<String> = found
        .iter()
        .map(|item| {
            format!(
                "  {} - to {} - {} - {}",
                item.id,
                if item.to.is_empty() {
                    "nobody"
                } else {
                    &item.to
                },
                item.outbox,
                item.title.trim().trim_start_matches(['-', ' '])
            )
        })
        .collect();
    Ok(format!(
        "{} of {looked} outstanding item(s) name {needle} - tell their owner:\n{}",
        said.len(),
        said.join("\n")
    ))
}
fn header_rows(text: &str) -> Vec<Vec<String>> {
    let bare = |line: &str| {
        line.trim_start()
            .trim_start_matches('>')
            .trim_start()
            .to_string()
    };
    let lines: Vec<String> = text.lines().map(bare).collect();
    let mut out = Vec::new();
    for (at, line) in lines.iter().enumerate() {
        if !line.starts_with('|') {
            continue;
        }
        let next = lines.get(at + 1).map(|l| l.trim()).unwrap_or("");
        if !next.starts_with("| -") && !next.starts_with("|-") {
            continue;
        }
        out.push(
            line.trim()
                .trim_matches('|')
                .split('|')
                .map(|c| c.trim().trim_matches('*').to_string())
                .filter(|c| !c.is_empty())
                .collect(),
        );
    }
    out
}

/// **A proposal saying `shape rows` offers cells that land. A before-and-after table does not.**
///
/// `CLAUDE.md`: *if it will say something these words only described, that is an instruction*. A
/// table headed `Values now | Values after` describes a change - no cell of it ever appears in
/// the destination, and the rows check correctly looks for `Values now` there and does not find
/// it.
///
/// **The check is that every column the proposal's table names exists in a table of the file it
/// names.** `Values now` exists nowhere, which is what makes it findable.
///
/// **This check covers one of three, and the three do not share a direction.** Re-derived by the
/// code lane as `C-107` after this was written, each read in the file rather than from the
/// exception text that names it:
///
/// - `P-195` declared `text` and offered an **instruction**
/// - `P-236` declared `text` and offered a **row**
/// - `P-465` declared `rows` and offered an **instruction**
///
/// **This sees `P-465`'s direction and only that one.** It cannot see a `text` proposal offering
/// a row or an instruction - and those are the two that recurred first. **So it is a case rather
/// than a fix**, and *four instances of one thing* - what this comment said first - is a
/// different claim from *three instances, one per direction* about whether the class is closed.
///
/// **`C-19` is the item about `P-236` and `P-466` is not an instance at all.** Counting an item
/// beside the proposal it reports is a population double-counted by its own record; `P-466`'s
/// table is the seven columns *Units and structures* has afterwards, and the one cell the release
/// lacked was written by `P-465` inside the same promoting commit.
///
/// **What would cover the other two is comparing the declared shape against the shape of what is
/// quoted** - a row opens with `|`, an instruction says *becomes* and appears nowhere verbatim.
/// `C-19` described it and did not build it, said so, and nobody has asked. Still true.
fn shape_is_rows_only_if_the_cells_land(root: &Path, id: &str, draft: &str) -> Result<(), String> {
    if !draft.contains("**shape** rows") {
        return Ok(());
    }
    let offered = header_rows(draft);
    if offered.is_empty() {
        return Ok(());
    }
    let fields = draft.lines().nth(2).unwrap_or("");
    let mut known: Vec<String> = Vec::new();
    let mut looked = 0;
    for piece in fields.split('`').skip(1).step_by(2) {
        if !piece.ends_with(".md") {
            continue;
        }
        let Ok(text) = read(&root.join(piece)) else {
            continue;
        };
        looked += 1;
        for row in header_rows(&text) {
            known.extend(row);
        }
    }
    if looked == 0 {
        return Ok(());
    }
    for row in &offered {
        for column in row {
            if !known.contains(column) {
                return Err(format!(
                    "{id} says `shape rows` and offers a column {column:?} that no table in the \
                     files it names has. A table describing a change is an instruction - \
                     CLAUDE.md: if it will say something these words only described, that is an \
                     instruction"
                ));
            }
        }
    }
    Ok(())
}

/// Put a notice at the top of *Addressed to other perspectives*.
///
/// **A notice carries no `asks` because only a proposal does**, and it is the commonest item this
/// lane writes - `S-121` was refused by a verb that had no case for it. The queue is the only
/// outbox this lane owns, so there is one destination and no routing to do.
fn file_notice(root: &Path, id: &str, draft: &str) -> Result<String, String> {
    let at = root.join(OUTBOX);
    let before = read(&at)?;
    if before.contains(&format!("### {id} - ")) {
        return Err(format!("{id} is already in the queue"));
    }
    let anchor = "## Addressed to other perspectives\n";
    let cut = before
        .find(anchor)
        .ok_or_else(|| "the queue has no section for notices".to_string())?
        + anchor.len();
    let body = draft.trim_end();
    let written = format!(
        "{}\n{body}\n\n{}",
        &before[..cut],
        before[cut..].trim_start()
    );
    write(&at, &written)?;

    let back = read(&at)?;
    if back.matches(&format!("### {id} - ")).count() != 1 {
        return Err(format!(
            "{id} is not in the queue exactly once after filing"
        ));
    }
    if back.contains('\r') {
        return Err("the queue holds a carriage return after the write".to_string());
    }
    Ok(format!(
        "{id} filed: {} line(s) under Addressed to other perspectives",
        body.lines().count()
    ))
}

/// Put a drafted item at the top of `## Open`, and make the sentinel agree afterwards.
///
/// **Filing was the one step still done by a script written fresh each time**, and
/// `CLAUDE.md` records what that costs: every hand-rolled edit to this file has eventually
/// reintroduced something the tool already guards. It cost the sentinel twice in one day -
/// `P-455`, then `P-456` an hour after the gate check for it existed - because a script that
/// inserts a block has no reason to think about a sentence three lines above it.
///
/// The draft is a file whose first line is the `### P-n - title` heading. It is checked the
/// way a promotion is: no carriage return, no `###` inside the body, and the id not already
/// in the queue.
fn file(root: &Path, path: &str) -> Result<String, String> {
    let draft = read(Path::new(path))?.replace("\r\n", "\n");
    let heading = draft
        .lines()
        .next()
        .ok_or_else(|| format!("{path} is empty"))?;
    let id = heading
        .strip_prefix("### ")
        .and_then(|rest| rest.split_once(" - "))
        .map(|(id, _)| id.trim().to_string())
        .ok_or_else(|| format!("{path} does not begin with `### P-n - title`"))?;
    if draft.lines().skip(1).any(|line| line.starts_with("### ")) {
        return Err(format!(
            "{id}'s body holds a `###`, which every tool reads as the next item - CLAUDE.md"
        ));
    }

    // **`asks` chooses the file, because the two files are that field written down.**
    // `docs/notes/decisions.md` holds choices only Sean can make and the queue holds words for
    // him to approve; an item lives in one at a time. `CLAUDE.md` already makes `asks`
    // checkable rather than descriptive, and this is the other thing it can check. Filed
    // `P-460` into the wrong one by hand within a minute of this verb existing.
    //
    // **And an item with no `asks` is a notice rather than a proposal**, which `CLAUDE.md` says
    // in as many words: *only a proposal carries it*. A notice goes under *Addressed to other
    // perspectives* in the queue. This verb refused `S-121` for a reason true of every notice
    // ever filed - it had no case for the commonest item this lane writes.
    let fields = draft.lines().nth(2).unwrap_or("");
    let to_sean = fields.contains("**to** sean");
    let asks_decision = match (
        draft.contains("**asks** a decision"),
        draft.contains("**asks** approval"),
    ) {
        (true, false) => true,
        (false, true) => false,
        (true, true) => return Err(format!("{id} says it asks both")),
        // **Nothing but a proposal is addressed to Sean** - `CLAUDE.md` - so an item with no
        // `asks` and no other reader has no home, and saying which of the two is wrong is more
        // use than saying it has none.
        (false, false) if to_sean => {
            return Err(format!(
                "{id} is addressed to sean and asks nothing. Nothing but a proposal is addressed                  to him, so it wants an `**asks**` or a different reader"
            ));
        }
        (false, false) => return file_notice(root, &id, &draft),
    };
    let into = if asks_decision { QUESTIONS } else { QUEUE };
    shape_is_rows_only_if_the_cells_land(root, &id, &draft)?;

    let at = root.join(into);
    let before = read(&at)?;
    if before.contains(&format!("### {id} - ")) {
        return Err(format!("{id} is already in the queue"));
    }
    let anchor = "## Open\n";
    let cut = before
        .find(anchor)
        .ok_or_else(|| "the queue has no Open section".to_string())?
        + anchor.len();
    let body = draft.trim_end();
    let written = format!("{}\n{body}\n{}", &before[..cut], before[cut..].trim_start());
    // Only the queue carries the sentinel; the decisions file says its own thing.
    let settled = if asks_decision {
        written
    } else {
        queue::say_if_empty(&written).map_err(|why| why.to_string())?
    };
    write(&at, &settled)?;

    let back = read(&at)?;
    if back.matches(&format!("### {id} - ")).count() != 1 {
        return Err(format!("{id} is not in {into} exactly once after filing"));
    }
    if !asks_decision && back.contains(queue::NOTHING_OPEN) {
        return Err("the queue still says nothing is open".to_string());
    }
    if back.contains('\r') {
        return Err(format!("{into} holds a carriage return after the write"));
    }
    let other = if asks_decision { QUEUE } else { QUESTIONS };
    if read(&root.join(other))?.contains(&format!("### {id} - ")) {
        return Err(format!(
            "{id} is in {other} too, and an item lives in one at a time"
        ));
    }
    Ok(format!(
        "{id} filed: {} line(s) at the top of {into} -> Open",
        body.lines().count()
    ))
}

/// Today, from the system clock, as `YYYY-MM-DD`.
///
/// Days since the epoch, converted by the civil-from-days algorithm rather than by pulling
/// in a date crate for one line - `tools/` has no dependencies outside this repository and
/// one field is not a reason to start.
fn today() -> String {
    let days = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs() / 86_400)
        .unwrap_or(0) as i64;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!("{year:04}-{m:02}-{d:02}")
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|why| format!("cannot read {}: {why}", path.display()))
}

/// Write with `\n`, always.
///
/// **The default put six carriage returns into `spec/invariants.md` on 2026-09-11**, and the
/// check that should have caught it collapsed whitespace, which `\r` is.
fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text.replace("\r\n", "\n"))
        .map_err(|why| format!("cannot write {}: {why}", path.display()))
}

fn root() -> PathBuf {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.parent()
        .and_then(Path::parent)
        .unwrap_or(here)
        .to_path_buf()
}

/// Where one item closed by naming another as carrying what it dropped.
///
/// **The failure is in `docs/postmortems/tracked-and-still-lost.md`.** `C-16` closed saying
/// *the gap is unchanged and is not being dropped - `S-30` is the item that carries it*;
/// `S-30` was withdrawn six days later saying *nothing is lost by this withdrawal* and handed
/// its live half to `C-102`. Each hop was honest and each carried a narrower question, so
/// *recipes are executed as match arms* fell out on the first hop and no later step could
/// recover it. The gap was tracked for ten days by an unbroken chain and was lost anyway.
///
/// **This is a detector and not a check, and the difference is the whole of what it claims.**
/// It says a chain exists and where to look. It cannot say whether the successor asks a
/// narrower question than its predecessor, because that is a comparison between two English
/// sentences - `P-245`'s wall, that no check can ask whether another check's predicate is
/// about its subject.
///
/// **Two things keep the false edges out, and both were measured rather than guessed.** A
/// first version counted any id mentioned anywhere in a closed item's body and produced two
/// false edges: `C-30` names `S-30` inside a table of counts, and `C-87` mentions it in
/// passing. So a handoff is never read from a table row, and the cue and the id must sit in
/// the same section. **The section rather than the sentence or the paragraph**, because
/// `S-30` puts its cue in a `## Where the live half is` heading and the successor's id in the
/// paragraph under it - a paragraph-scoped version returned a plausible zero and missed the
/// one chain that matters.
fn chains(root: &Path) -> Result<String, String> {
    let all = outbox::read(root);
    if all.items.is_empty() {
        return Err("no items in any outbox, so this counted over nothing".to_string());
    }

    let mut status: Vec<(String, String)> = Vec::new();
    let mut edges: Vec<(String, String)> = Vec::new();
    let mut closed = 0;
    for item in &all.items {
        status.push((item.id.clone(), item.status.clone()));
        if item.status == "open" {
            continue;
        }
        closed += 1;
        for named in spec::handed_to(&item.id, &item.body) {
            edges.push((item.id.clone(), named));
        }
    }

    let state = |id: &str| -> String {
        status
            .iter()
            .find(|(known, _)| known == id)
            .map(|(_, s)| s.clone())
            .unwrap_or_else(|| "unknown".to_string())
    };

    // **Only a chain whose tail is still open, and the narrowing is the point.** A first
    // version printed every two-hop chain in the history - forty of them, one of which
    // mattered - which is `CLAUDE.md`'s *a report where everything matters is a report where
    // nothing does*. A chain that ends in something closed is history; one that ends in
    // something open is a question somebody may still be answering more narrowly than it was
    // asked.
    let mut said: Vec<String> = Vec::new();
    let mut examined = 0;
    for (first, second) in &edges {
        for (from, third) in &edges {
            if from != second || third == first {
                continue;
            }
            examined += 1;
            if state(third) != "open" {
                continue;
            }
            let line = format!(
                "  {first} ({}) -> {second} ({}) -> {third} (OPEN)",
                state(first),
                state(second)
            );
            if !said.contains(&line) {
                said.push(line);
            }
        }
    }
    let hops = said.len();
    let live: Vec<String> = edges
        .iter()
        .filter(|(_, second)| state(second) == "open")
        .map(|(first, second)| format!("  {first} ({}) -> {second} is still open", state(first)))
        .collect();

    let mut report = vec![format!(
        "{} item(s), {closed} closed, {} handoff(s) found",
        all.items.len(),
        edges.len()
    )];
    report.push(String::new());
    report.push(format!(
        "Of {examined} two-hop chain(s), those still ending in an open item. Read the",
    ));
    report
        .push("first and the last together, and ask whether the last still asks what".to_string());
    report.push("the first one asked:".to_string());
    if hops == 0 {
        report.push("  none".to_string());
    } else {
        report.extend(said);
    }
    report.push(String::new());
    report.push("Closed items whose successor is still open:".to_string());
    if live.is_empty() {
        report.push("  none".to_string());
    } else {
        report.extend(live);
    }
    Ok(report.join("\n"))
}
