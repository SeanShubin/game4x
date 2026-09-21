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
//! **Then open `http://127.0.0.1:7878` in a browser.** That is an address to visit and not an
//! argument: this takes none, and the port is fixed because one is all it needs. Sean, 2026-09-20,
//! having pasted it onto the command line where it was silently ignored - the line above used to
//! read *Then `http://127.0.0.1:7878`*, which invites exactly that.
//!
//! The arrow keys move, `Enter` opens, `r` marks the test reviewed, `u` takes that back, and `x`
//! files a note saying what needs changing. Every key press is a write to the disk that has already
//! happened before the badge changes.
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
//! **Every `.4x` file is browsable at the path it has on disk**, as `text/plain`. Sean, 2026-09-21:
//! *lets also make sure the tests are browsable from the deployment, bearing in mind that the .4x
//! extension my not render in a browser as text without the proper mime type, we may need to create
//! .txt files for rendering purposes.* **No second copy was needed**: the media type is this
//! server's to declare, and a `.txt` beside every `.4x` would be a third representation of every
//! test on top of the two `tests/directories.rs` already keeps from drifting.
//!
//! **The whitelist is the traversal defence, and it is not a separate one.** The paths are listed
//! from the disk at startup and a request that is not one of them is refused - so no path is ever
//! built from what a request said, which is the same shape as refusing a name that is not a test.
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
    let browsable = browsable();
    println!("http://{at}  -  {} tests", known.len());
    println!("arrows move, Enter opens, r reviewed, x needs changing, u unreview. Ctrl-C to stop.");
    for coming in listening.incoming() {
        match coming {
            Ok(stream) => serve(stream, &known, &browsable),
            Err(why) => eprintln!("{why}"),
        }
    }
}

/// Read one request, answer it, and close.
///
/// **One at a time, and that is enough.** One person is reading one page; a thread pool would be
/// machinery in a prototype whose whole point is having as little as possible.
fn serve(mut stream: TcpStream, known: &[String], browsable: &[String]) {
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

    let (status, kind, said) = answer(&method, &path, &body, known, browsable);
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
    browsable: &[String],
) -> (String, &'static str, String) {
    let ok = |kind, said| ("200 OK".to_string(), kind, said);
    match (method, path) {
        ("GET", "/") => ok(HTML, report::build(true).page),
        ("GET", "/data") => ok(HTML, index(browsable)),
        // **Served at the path it has on disk**, which is the shortest answer to *where is this
        // file*: the address is the answer. **`text/plain` is why no `.txt` copy exists** - a
        // browser shows a `.4x` as text when something tells it to, and this is the something.
        ("GET", _) if path.starts_with("/data/") => {
            let wanted = path.trim_start_matches('/');
            // **A path not on the list is refused rather than resolved.** Nothing here joins a
            // request to a directory, so `..` is not a case to handle - it is a string that
            // matches nothing.
            if !browsable.iter().any(|it| it == wanted) {
                return ("404 Not Found".to_string(), PLAIN, format!("no {wanted}"));
            }
            match std::fs::read_to_string(mine().join(wanted)) {
                Ok(text) => ok(PLAIN, text),
                Err(why) => ("500".to_string(), PLAIN, format!("{wanted}: {why}")),
            }
        }
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

/// Every `.4x` file under `data/`, as the path it has on disk.
///
/// **Listed from the disk rather than written down**, so a test added tomorrow is browsable
/// without anyone remembering - and so the list cannot say a file is there when it is not.
fn browsable() -> Vec<String> {
    let mut found = Vec::new();
    for flavour in ["foundation", "friendly"] {
        for under in ["", "tests"] {
            let at = mine().join("data").join(flavour).join(under);
            let Ok(entries) = std::fs::read_dir(&at) else {
                continue;
            };
            for entry in entries.filter_map(|it| it.ok()) {
                let path = entry.path();
                if path.extension().and_then(|it| it.to_str()) != Some("4x") {
                    continue;
                }
                let Some(name) = path.file_name().and_then(|it| it.to_str()) else {
                    continue;
                };
                found.push(match under {
                    "" => format!("data/{flavour}/{name}"),
                    _ => format!("data/{flavour}/{under}/{name}"),
                });
            }
        }
    }
    found.sort();
    found
}

/// A page of links, so that browsing needs no directory listing from the file system.
fn index(browsable: &[String]) -> String {
    let mut out = String::from(
        "<!doctype html>\n<html lang=\"en\"><head><meta charset=\"utf-8\">\n<title>thin-engine data</title>\n<style>body{font:15px/1.7 ui-monospace,Menlo,monospace;margin:2rem auto;max-width:62rem;padding:0 1rem}a{display:block}h2{font-size:1rem;margin:1.2rem 0 .3rem}</style>\n</head><body>\n<h1>data</h1>\n<p><a href=\"/\">back to the report</a></p>\n",
    );
    let mut heading = "";
    for one in browsable {
        let under = match one.rsplit_once('/') {
            Some((at, _)) => at,
            None => "",
        };
        if under != heading {
            heading = under;
            out.push_str(&format!("<h2>{heading}</h2>\n"));
        }
        out.push_str(&format!("<a href=\"/{one}\">{one}</a>\n"));
    }
    out.push_str("</body></html>\n");
    out
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
