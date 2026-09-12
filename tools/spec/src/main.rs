//! Promoting a proposal, as one command rather than a script written fresh each time.
//!
//! ```text
//! spec show <id>                     the approved text, exactly as it will land
//! spec after <id> <file> <anchor>    put it in after that line, and assert it landed
//! spec replacing <id> <file> <old>   put it in where those lines are, and assert it landed
//! spec land <id> <after>             ledger row from the item's own `into`, then remove it
//! ```
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

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let root = root();
    let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let outcome = match borrowed.as_slice() {
        ["show", id] => show(&root, id),
        ["after", id, file, anchor] => after(&root, id, file, anchor),
        ["replacing", id, file, old] => replacing(&root, id, file, old),
        ["land", id, previous] => land(&root, id, previous),
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
        "spec show <id>                     the approved text, exactly as it will land",
        "spec after <id> <file> <anchor>    put it in after that line, and assert it landed",
        "spec replacing <id> <file> <old>   put it in where those lines are, and assert it landed",
        "spec land <id> <after-id>          ledger row from the item's own `into`, then remove it",
    ]
    .join("\n")
}

/// The one proposal with this id, from the queue that `tools/outbox` reads.
fn proposal(root: &Path, id: &str) -> Result<outbox::Item, String> {
    let queue = read(&root.join("docs/notes/proposals.md"))?;
    let items = outbox::parse(&queue, "docs/notes/proposals.md");
    let mut found: Vec<outbox::Item> = items.into_iter().filter(|item| item.id == id).collect();
    match found.len() {
        1 => Ok(found.remove(0)),
        0 => Err(format!("{id} is not an item in docs/notes/proposals.md")),
        several => Err(format!(
            "{id} appears {several} times, so which one is a guess"
        )),
    }
}

/// What the proposal offers, with its quoting stripped and no carriage return in it.
fn approved(item: &outbox::Item) -> Result<String, String> {
    let text = item
        .proposed_text()
        .map_err(|why| format!("{}: {why}", item.id))?;
    if text.contains('\r') {
        return Err(format!(
            "{}'s approved text holds a carriage return",
            item.id
        ));
    }
    Ok(text)
}

fn show(root: &Path, id: &str) -> Result<String, String> {
    let item = proposal(root, id)?;
    let text = approved(&item)?;
    Ok(format!(
        "{id}, {} line(s), into {}\n\n{text}",
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
fn after(root: &Path, id: &str, file: &str, anchor: &str) -> Result<String, String> {
    let item = proposal(root, id)?;
    let text = approved(&item)?;
    let path = root.join(file);
    let before = read(&path)?;
    let written = queue::insert_after(&before, anchor, &text).map_err(|why| why.to_string())?;
    write(&path, &written)?;

    let back = read(&path)?;
    queue::lands_once(&back, &text).map_err(|why| format!("{id} did not land: {why}"))?;
    if back.contains('\r') {
        return Err(format!("{file} holds a carriage return after the write"));
    }
    Ok(format!(
        "{id}: {} line(s) into {file} after {anchor:?}, present exactly once, no carriage return",
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
fn replacing(root: &Path, id: &str, file: &str, old: &str) -> Result<String, String> {
    let item = proposal(root, id)?;
    let text = approved(&item)?;
    let path = root.join(file);
    let before = read(&path)?;
    let written = queue::replace_run(&before, old, &text).map_err(|why| why.to_string())?;
    write(&path, &written)?;

    let back = read(&path)?;
    queue::lands_once(&back, &text).map_err(|why| format!("{id} did not land: {why}"))?;
    if back.contains('\r') {
        return Err(format!("{file} holds a carriage return after the write"));
    }
    if queue::collapse(&back).contains(&queue::collapse(old)) {
        return Err(format!("{file} still holds the text {id} replaced"));
    }
    Ok(format!(
        "{id}: {} line(s) into {file}, present exactly once, the replaced text gone, no carriage return",
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

    let path = root.join("docs/notes/proposals.md");
    let text = read(&path)?;
    let with_row = queue::insert_ledger_row(&text, previous, &row).map_err(|w| w.to_string())?;
    let without = queue::remove_block(&with_row, id).map_err(|why| why.to_string())?;
    let settled = queue::say_if_empty(&without).map_err(|why| why.to_string())?;
    write(&path, &settled)?;

    let back = read(&path)?;
    if back.contains(&format!("### {id} - ")) {
        return Err(format!(
            "{id}'s block is still in the file after landing it"
        ));
    }
    let rows = back
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
