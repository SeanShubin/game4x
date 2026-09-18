//! Serve the report, and let Sean say what he thinks of a test without leaving it.
//!
//! **Sean, 2026-09-16**: *The goal is to be able to look through everything quickly and express my
//! decisions quickly.*
//!
//! ```text
//! cd prototypes/thin-engine
//! cargo run --example review-web
//! ```
//!
//! Then `http://127.0.0.1:7878`. The arrow keys move, `Enter` opens, `r` marks the test reviewed,
//! `u` takes that back, and `x` files a note saying what needs changing. Every key press is a write
//! to the disk that has already happened before the badge changes.
//!
//! **`j` and `k` still work and are not advertised.** They were the only way to move until Sean
//! said so: *the j/k to move is unintuitive* - which it is, being vim's and not anything a reader
//! would guess.
//!
//! **The page is `report`'s, not this file's.** This calls `report::build(true)` for every request,
//! so the served page and `report.html` cannot disagree about what a test says - and because it is
//! rebuilt per request, a test edited while this is running shows up on the next reload.
//!
//! **Three writes and nothing else.** A copy into `reviewed/` says *I have read this*, deleting it
//! takes that back, and a bullet in `reviewed/asked.md` says *change this* - which is addressed to
//! the code lane rather than being a record of reading, and is why the two are separate files.
//!
//! **It binds `127.0.0.1` and refuses a name that is not a test.** Both matter because this is a
//! thing that writes files when something asks it to: the first means only this machine can ask,
//! and the second means the worst a bad request can do is get a refusal, rather than leave a copy
//! in `reviewed/` that no test will ever be compared against.
//!
//! **This lane still never marks anything reviewed** - `review.rs` says why. Running the server is
//! Sean's, the same as running the command.

#[path = "report.rs"]
#[allow(dead_code)]
mod report;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

fn mine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// What `reviewed/asked.md` says about itself, written once when the first note is filed.
const HEAD: &str = "# What needs changing

Written by `review-web`, read by `report` so each note shows with its test, and cleared by whoever
acts on it. One `## <test>` section per test, one `- ` bullet per note. Editing it by hand is fine.
";

fn main() {
    let at = "127.0.0.1:7878";
    let listening = TcpListener::bind(at).unwrap_or_else(|why| panic!("{at}: {why}"));
    let known: Vec<String> = report::every_test()
        .iter()
        .map(|file| file.trim_end_matches(".4x").to_string())
        .collect();
    println!("http://{at}  -  {} tests", known.len());
    println!("arrows move, Enter opens, r reviewed, x needs changing, u unreview. Ctrl-C to stop.");
    for coming in listening.incoming() {
        match coming {
            Ok(stream) => serve(stream, &known),
            Err(why) => eprintln!("{why}"),
        }
    }
}

/// Read one request, answer it, and close.
///
/// **One at a time, and that is enough.** One person is reading one page; a thread pool would be
/// machinery in a prototype whose whole point is having as little as possible.
fn serve(mut stream: TcpStream, known: &[String]) {
    let Ok(peer) = stream.try_clone() else { return };
    let mut reading = BufReader::new(peer);

    let mut first = String::new();
    if reading.read_line(&mut first).is_err() {
        return;
    }
    let mut words = first.split_whitespace();
    let method = words.next().unwrap_or("").to_string();
    let path = words.next().unwrap_or("").to_string();

    let mut length = 0usize;
    loop {
        let mut line = String::new();
        match reading.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => return,
        }
        if line.trim().is_empty() {
            break;
        }
        if let Some(said) = line.to_ascii_lowercase().strip_prefix("content-length:") {
            length = said.trim().parse().unwrap_or(0);
        }
    }
    let mut body = vec![0u8; length];
    if length > 0 && reading.read_exact(&mut body).is_err() {
        return;
    }
    let body = String::from_utf8_lossy(&body).to_string();

    let (status, kind, said) = answer(&method, &path, &body, known);
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        said.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(said.as_bytes());
    let _ = stream.flush();
}

const HTML: &str = "text/html; charset=utf-8";
const PLAIN: &str = "text/plain; charset=utf-8";

fn answer(
    method: &str,
    path: &str,
    body: &str,
    known: &[String],
) -> (String, &'static str, String) {
    let ok = |kind, said| ("200 OK".to_string(), kind, said);
    match (method, path) {
        ("GET", "/") => ok(HTML, report::build(true).page),
        ("POST", "/reviewed") | ("POST", "/unreview") | ("POST", "/asked") => {
            let Some(name) = field(body, "name") else {
                return ("400 Bad Request".to_string(), PLAIN, "no name".to_string());
            };
            // **A name that is not a test is refused rather than guessed at**, which is the same
            // refusal `review.rs` makes for the same reason: a copy of nothing is compared against
            // nothing, forever.
            if !known.contains(&name) {
                return (
                    "400 Bad Request".to_string(),
                    PLAIN,
                    format!("`{name}` is not a test"),
                );
            }
            match path {
                "/reviewed" => {
                    let from = mine().join(format!("data/friendly/tests/{name}.4x"));
                    let text = match std::fs::read_to_string(&from) {
                        Ok(text) => text,
                        Err(why) => return ("500".to_string(), PLAIN, format!("{name}: {why}")),
                    };
                    let into = mine().join("reviewed");
                    let _ = std::fs::create_dir_all(&into);
                    match std::fs::write(into.join(format!("{name}.4x")), text) {
                        Ok(()) => ok(PLAIN, "reviewed".to_string()),
                        Err(why) => ("500".to_string(), PLAIN, format!("{name}: {why}")),
                    }
                }
                "/unreview" => {
                    let at = mine().join(format!("reviewed/{name}.4x"));
                    // **Already gone is the answer, not an error.** The page and the disk can
                    // disagree for a moment; saying so would be reporting a race as a fault.
                    let _ = std::fs::remove_file(at);
                    ok(PLAIN, "never reviewed".to_string())
                }
                _ => {
                    let note = field(body, "note").unwrap_or_default();
                    let note = note.trim();
                    if note.is_empty() {
                        return ("400 Bad Request".to_string(), PLAIN, "no note".to_string());
                    }
                    file(&name, note);
                    ok(PLAIN, "noted".to_string())
                }
            }
        }
        _ => (
            "404 Not Found".to_string(),
            PLAIN,
            format!("no {method} {path}"),
        ),
    }
}

/// One string field of a flat JSON object.
///
/// **Enough JSON for two fields and no more**, because the alternative is a dependency and this
/// crate has none on purpose. It handles the escapes a note can produce - a quote and a backslash -
/// and it would be wrong about a nested object, which nothing here sends.
fn field(body: &str, name: &str) -> Option<String> {
    let key = format!("\"{name}\"");
    let at = body.find(&key)? + key.len();
    let rest = body[at..].trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let mut letters = rest.strip_prefix('"')?.chars();
    let mut out = String::new();
    loop {
        match letters.next()? {
            '"' => return Some(out),
            '\\' => match letters.next()? {
                'n' => out.push('\n'),
                't' => out.push('\t'),
                other => out.push(other),
            },
            letter => out.push(letter),
        }
    }
}

/// Add one note to `reviewed/asked.md`, under the test it is about.
///
/// **Rebuilt from its own lines rather than appended to.** A second note about one test belongs in
/// that test's section, and a file that is also edited by hand cannot be written to blind.
fn file(name: &str, note: &str) {
    let at = mine().join("reviewed/asked.md");
    let _ = std::fs::create_dir_all(mine().join("reviewed"));
    let text = std::fs::read_to_string(&at).unwrap_or_else(|_| HEAD.to_string());
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let heading = format!("## {name}");
    let bullet = format!("- {}", note.replace('\n', " "));
    match lines.iter().position(|line| line.trim() == heading) {
        Some(start) => {
            let mut end = start + 1;
            while end < lines.len() && !lines[end].trim_start().starts_with("## ") {
                end += 1;
            }
            while end > start + 1 && lines[end - 1].trim().is_empty() {
                end -= 1;
            }
            lines.insert(end, bullet);
        }
        None => {
            if lines.last().is_some_and(|line| !line.trim().is_empty()) {
                lines.push(String::new());
            }
            lines.push(heading);
            lines.push(bullet);
        }
    }
    std::fs::write(&at, lines.join("\n") + "\n").expect("reviewed/asked.md");
}
