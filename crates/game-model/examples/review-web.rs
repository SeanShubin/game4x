//! Serve the report, and let Sean say what he thinks of a test without leaving it.
//!
//! **Sean, 2026-09-16**: *The goal is to be able to look through everything quickly and express my
//! decisions quickly.*
//!
//! ```text
//! cd crates/game-model
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

/// Where a test's address begins, under the repository root.
///
/// **`spec/tests/` split into `rule/` and `interface/` on 2026-10-01 and this file said it three
/// times.** One said `spec/tests/rule` and two said `spec/tests/`, so
/// `every_test_is_browsable` refused to start - *57 of 57 tests have no address* - and Sean could
/// not review a test. **`S-240`**, and the check did exactly what its own comment says: *loudly,
/// at startup, before the first page*, because *a server that answers 404 on every link looks
/// exactly like one that is working until somebody clicks.*
///
/// **Fixing only the call site that panicked would have broken the other direction.** Line 207
/// trimmed `spec/tests/` off an address and compared the remainder to a bare stem, so with the
/// first repaired it would have left `rule/name` and reported every test as unlisted - the half
/// `S-214` added, failing for the opposite reason.
const UNDER: &str = "spec/tests/rule/";

/// A record: the verdict, and the behaviour that verdict is about.
///
/// **`spec/README.md` rule 3**: *a record names its verdict and carries the behaviour that verdict
/// is about - the rows, canonical, without the prose.* And `P-600`: *my approval is about what a
/// test says, not where it is or how it is written... the behaviour is every `{...}` row, including
/// a `{load}`, and nothing else - a comment explains and does not decide, and `{test name:}`
/// identifies rather than states.*
///
/// # Why `{test name:}` is written although it is not behaviour
///
/// **An approval is never inferred for a test that has not been given one.** If a record's identity
/// were only its filename, `git mv reviewed/rule/x.4x y.4x` would hand test `y` the verdict given
/// to `x`, with nothing to notice - the silent misattribution `P-600` names as the one failure no
/// convenience is worth. **The row is what makes a rename detectable.**
///
/// # Canonical is the schema's declared order, and writing it is compatibility
///
/// **Rule 3's *the order this specification already gives them* names two orders**, and the tree is
/// in one of them: measured over all 57 records, 449 rows are not in alphabetical trait order and
/// 51 are by coincidence. Every row is in the schema's declared order, which is what
/// [`Names::row`] emits - for `citizen` that is `where, hungry, bearing, laboring, quantity`.
///
/// **So writing that order reproduces the rows that already exist**, which is compatibility rather
/// than a choice. `P-606` asks him which order rule 3 meant; **if it is `console.md`'s, the rewrite
/// is the same size whether this was written today or waited**, so waiting bought nothing.
///
/// # One trap worth knowing about, and it is loud
///
/// **Renumbering a `seq:` in the schema clears every verdict.** Those values are editable and carry
/// no meaning beyond order, so a tidy-up nobody thinks twice about changes the order this writer
/// emits, changes every record's bytes, and sends all 57 back to him. **It is the safe direction -
/// the verdicts clear and he notices rather than a stale one surviving** - but it is a trap laid for
/// a later session. Alphabetical order cannot do it, because names are not renumbered.
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
    every_test_is_browsable(&known, &browsable);
    println!("http://{at}  -  {} tests", known.len());
    println!("arrows move, Enter opens, r reviewed, x needs changing, u unreview. Ctrl-C to stop.");
    for coming in listening.incoming() {
        match coming {
            Ok(stream) => serve(stream, &known, &browsable),
            Err(why) => eprintln!("{why}"),
        }
    }
}

/// Every test the page shows has an address, and refuse to serve if one does not.
///
/// **This is the check that was missing, and it is about the outcome rather than the input.**
/// Counting the files under a directory says what is on the disk; this asks whether the link the
/// page is about to print will answer, which is the thing that was false for fifty-four of them
/// between `P-532` and now.
///
/// **It reads `browsable` rather than the disk**, so it fails for a directory that moved, a
/// directory that could not be read, and a root somebody forgot to list - all of which are the
/// same defect from the reader's side and none of which raises anything on its own.
///
/// **Loudly, at startup, before the first page.** A server that answers 404 on every link looks
/// exactly like one that is working until somebody clicks, and nothing in the suite is about what
/// an address answers.
fn every_test_is_browsable(known: &[String], browsable: &[String]) {
    let missing: Vec<&String> = known
        .iter()
        .filter(|name| {
            let wanted = format!("{UNDER}{name}.4x");
            !browsable.contains(&wanted)
        })
        .collect();
    assert!(
        missing.is_empty(),
        "{} of {} tests have no address, so the page would link at nothing: {:?}",
        missing.len(),
        known.len(),
        missing
    );
    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. With no
    // tests found, every test would be browsable vacuously.
    assert!(
        known.len() > 40,
        "only {} tests were found, so this checked almost nothing",
        known.len()
    );

    // **And the other direction, which is `S-214`.** The half above asks whether every test the
    // page lists has an address that answers; it cannot see a test on disk that the page never
    // listed. **Fifty-six files and fifty-four rows satisfied it**, and the two Sean was meant to
    // read were absent in silence.
    //
    // **Its own comment said it was about the outcome rather than the input, and it was** - but
    // the outcome it read was the one the page had already chosen. Asking both directions is the
    // property; either alone passes over the gap the other is for.
    let unlisted: Vec<&String> = browsable
        .iter()
        .filter(|at| at.starts_with(UNDER) && at.ends_with(".4x"))
        .filter(|at| {
            let name = at
                .trim_start_matches(UNDER)
                .trim_end_matches(".4x")
                .to_string();
            !known.contains(&name)
        })
        .collect();
    assert!(
        unlisted.is_empty(),
        "{} test(s) are on disk and not on the page, so they can be read and not approved: {:?} - \
         the list is what `r` writes a record from",
        unlisted.len(),
        unlisted
    );
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
        // **The conversion `E-1` asks for, run by him through the application** - which is what
        // makes it his hand on the records. `CLAUDE.md`: *a record is created and deleted only by
        // the review application, acting as Sean.*
        //
        // **It is not a change of approval.** Sean, 2026-10-01: *the order of the columns is not
        // significant, so this should not make tests different.* Every record keeps its verdict
        // and its behaviour; only the order the columns are written in moves.
        //
        // **Idempotent, so running it twice is running it once.** A record already canonical is
        // left alone and counted as such, which is what lets him press it without checking first.
        ("POST", "/convert") => {
            // **Every record is checked before any is written** - `S-242`: *one press for all 57,
            // stopping on any whose meaning would change rather than reporting partial success.*
            //
            // **It used to write as it went**, so a record that tripped the check left the ones
            // before it converted and the ones after it not - a mixed directory and a line of
            // output, which is the state he asked not to be left in. **Nothing trips it today**,
            // which is exactly why the order mattered: the safe version and the unsafe one are
            // indistinguishable until the day one does.
            let mut writing = Vec::new();
            let mut already = 0;
            for file in report::every_test() {
                let at = report::records_at().join(&file);
                let Ok(was) = std::fs::read_to_string(&at) else {
                    continue;
                };
                let name = file.trim_end_matches(".4x");
                let verdict = match report::render::verdict_of(&was) {
                    Ok(report::render::Verdict::Denied) => "denied",
                    Ok(report::render::Verdict::Approved) => "approved",
                    Err(why) => return ("500".to_string(), PLAIN, format!("{file}: {why}")),
                };
                // **Built from the test rather than rewritten from the record**, because the
                // record is what the writer produces and the test is what it produces it from.
                // **The verdict is carried over** - that is the one thing a conversion may not
                // change.
                let from = report::tests_at().join(&file);
                let Ok(text) = std::fs::read_to_string(&from) else {
                    continue;
                };
                let now = match report::render::record_for(name, &text, verdict) {
                    Ok(now) => now,
                    Err(why) => return ("500".to_string(), PLAIN, format!("{file}: {why}")),
                };
                if now == was {
                    already += 1;
                    continue;
                }
                // **Refused rather than written if it would change what he approved.** A
                // conversion that altered the behaviour would be this lane deciding something,
                // and `report::drift` is the thing that knows whether it did.
                if report::drift(Some(&now), &text).0 == "drifted" {
                    return (
                        "500".to_string(),
                        PLAIN,
                        format!(
                            "{file}: converting would change what it says; nothing was written"
                        ),
                    );
                }
                writing.push((at, now));
            }
            let converting = writing.len();
            for (at, now) in writing {
                if let Err(why) = std::fs::write(&at, &now) {
                    return ("500".to_string(), PLAIN, format!("{}: {why}", at.display()));
                }
            }
            ok(
                PLAIN,
                format!("converted {converting}, already canonical {already}"),
            )
        }
        ("GET", "/data") => ok(HTML, index(browsable)),
        // **Served at the path it has on disk**, which is the shortest answer to *where is this
        // file*: the address is the answer. **`text/plain` is why no `.txt` copy exists** - a
        // browser shows a `.4x` as text when something tells it to, and this is the something.
        ("GET", _) if path.starts_with("/data/") || path.starts_with("/spec/") => {
            let wanted = path.trim_start_matches('/');
            // **A path not on the list is refused rather than resolved.** Nothing here joins a
            // request to a directory, so `..` is not a case to handle - it is a string that
            // matches nothing.
            if !browsable.iter().any(|it| it == wanted) {
                return ("404 Not Found".to_string(), PLAIN, format!("no {wanted}"));
            }
            match std::fs::read_to_string(on_disk(wanted)) {
                Ok(text) => ok(PLAIN, text),
                Err(why) => ("500".to_string(), PLAIN, format!("{wanted}: {why}")),
            }
        }
        ("POST", "/reviewed")
        | ("POST", "/denied")
        | ("POST", "/unreview")
        | ("POST", "/asked") => {
            let Some(name) = field(body, "name") else {
                return ("400 Bad Request".to_string(), PLAIN, "no name".to_string());
            };
            // **A name that is not a test is refused rather than guessed at**, which is the same
            // refusal `review.rs` makes for the same reason: a copy of nothing is compared against
            // nothing, forever.
            //
            // **Taking a reading back is the exception, and it is the whole point of it.** A record
            // whose test is gone is exactly a name that is not a test, so refusing it here would
            // make the one thing that can remove an orphan the one thing that cannot -
            // `CLAUDE.md`, promoted 2026-09-21: *a record is added and removed only by the review
            // application, acting as Sean.* **What may be taken back is what is there**, so this
            // asks the disk rather than the list of tests.
            let recorded =
                path == "/unreview" && report::records_at().join(format!("{name}.4x")).is_file();
            if !recorded && !known.contains(&name) {
                return (
                    "400 Bad Request".to_string(),
                    PLAIN,
                    format!("`{name}` is not a test"),
                );
            }
            match path {
                // **Approving and denying are one gesture with two words** - `E-2`, and
                // `spec/README.md` rule 3: *a record saying `approved` means the code is bound by
                // it; a record saying `denied` means it is not, and that I owe the specification a
                // statement of what I want instead.*
                //
                // **The same record either way**, because a verdict is about behaviour and both
                // verdicts are about the same behaviour. **A denial carrying no rows would be a
                // verdict about nothing** - and could not tell a later reader which test he turned
                // down, once the test changed under it.
                "/reviewed" | "/denied" => {
                    let from = report::tests_at().join(format!("{name}.4x"));
                    let text = match std::fs::read_to_string(&from) {
                        Ok(text) => text,
                        Err(why) => return ("500".to_string(), PLAIN, format!("{name}: {why}")),
                    };
                    // **A record is a verdict and the rows, not a copy of the file** - `P-605`.
                    // This wrote the test byte for byte, prose and all, which is what a record was
                    // while presence meant both *I read this* and *this binds*.
                    let verdict = if path == "/denied" {
                        "denied"
                    } else {
                        "approved"
                    };
                    let said = match report::render::record_for(&name, &text, verdict) {
                        Ok(said) => said,
                        Err(why) => return ("500".to_string(), PLAIN, format!("{name}: {why}")),
                    };
                    // **The suite's directory, not `reviewed/`** - `S-256`. A name carries its
                    // suite now, so a record lands at `reviewed/<suite>/<name>.4x` and the
                    // directory above it may not exist: approving the first interface test
                    // creates `reviewed/interface/`. **Creating `reviewed/` alone would have
                    // failed the write** with the page saying only *not written*.
                    let at = report::records_at().join(format!("{name}.4x"));
                    if let Some(parent) = at.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    match std::fs::write(&at, said) {
                        Ok(()) => ok(PLAIN, verdict.to_string()),
                        Err(why) => ("500".to_string(), PLAIN, format!("{name}: {why}")),
                    }
                }
                "/unreview" => {
                    let at = report::records_at().join(format!("{name}.4x"));
                    // **Already gone is the answer, not an error.** The page and the disk can
                    // disagree for a moment; saying so would be reporting a race as a fault.
                    let _ = std::fs::remove_file(at);
                    ok(PLAIN, "never reviewed".to_string())
                }
                // **A note is an annotation on `denied`, not a state of its own** - `P-611`, and
                // `spec/README.md` rule 3: *denied is where I say what I want instead, when I have
                // words for it. That is the same state whether I have said it or not: this is
                // wrong and this needs changing are one thing.*
                //
                // **So this writes the verdict and then the words.** It used to write only the
                // words, which left *needs changing* reading as a fourth answer on the page while
                // the record said he had never looked.
                _ => {
                    let note = field(body, "note").unwrap_or_default();
                    let note = note.trim();
                    if note.is_empty() {
                        return ("400 Bad Request".to_string(), PLAIN, "no note".to_string());
                    }
                    let from = report::tests_at().join(format!("{name}.4x"));
                    let text = match std::fs::read_to_string(&from) {
                        Ok(text) => text,
                        Err(why) => return ("500".to_string(), PLAIN, format!("{name}: {why}")),
                    };
                    let said = match report::render::record_for(&name, &text, "denied") {
                        Ok(said) => said,
                        Err(why) => return ("500".to_string(), PLAIN, format!("{name}: {why}")),
                    };
                    let into = report::records_at();
                    let _ = std::fs::create_dir_all(&into);
                    if let Err(why) = std::fs::write(into.join(format!("{name}.4x")), said) {
                        return ("500".to_string(), PLAIN, format!("{name}: {why}"));
                    }
                    file(&name, note);
                    ok(PLAIN, "denied".to_string())
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

/// Where a browsable path is, given the root that owns it.
///
/// **Two roots, because the move gave the two forms two owners.** `data/` is this prototype's and
/// `spec/tests/` is the repository's - `P-532` - so an address is the path from whichever root
/// owns the file, and this is the only place that knows which.
fn on_disk(wanted: &str) -> std::path::PathBuf {
    match wanted.starts_with("spec/") {
        true => mine().join("../..").join(wanted),
        false => mine().join(wanted),
    }
}

/// Every `.4x` file this serves, as the path it has under the root that owns it.
///
/// **One directory each, and never recursively**, so the address of a file is the directory it
/// was listed from plus its name - no path is assembled from a request.
///
/// **Listed from the disk rather than written down**, so a test added tomorrow is browsable
/// without anyone remembering - and so the list cannot say a file is there when it is not.
///
/// # It stopped listing the friendly tests silently, and every link to one was dead
///
/// **This walked `data/friendly/tests` until 2026-09-21**, when `P-532` moved those files to
/// `spec/tests/` and made them the specification. The walk `continue`s on a directory it cannot
/// read, so nothing failed and nothing said anything: the browse index simply lost a heading, and
/// **all fifty-four of the report's *on disk* links answered 404** - the links Sean asked for when
/// he asked where the tests were on disk.
///
/// **Found by driving the launcher rather than by reading it.** The suite says the two notations
/// agree and `tests/reviewed.rs` says every record names a test; neither is about what an address
/// answers, and a server that 404s is not a test failure anywhere.
fn browsable() -> Vec<String> {
    let mut found = Vec::new();
    for (root, under) in [
        (mine(), "data/foundation"),
        (mine(), "data/foundation/tests"),
        // **The five shared files still have a friendly side and it is still generated** -
        // `examples/render.rs` writes them and only them. What left the prototype was the tests.
        (mine(), "data/friendly"),
        (mine().join("../.."), UNDER.trim_end_matches('/')),
    ] {
        let Ok(entries) = std::fs::read_dir(root.join(under)) else {
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
            found.push(format!("{under}/{name}"));
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
    // **`reviewed/asked.md`, above the suites rather than inside one** - `S-256`. It was
    // `reviewed/rule/asked.md`, because `records_at()` was that directory; **one file holds notes
    // for every test**, so the suites are the wrong level for it. Nothing is moved: no note has
    // been filed, so this is where the first one lands.
    //
    // **The reader agrees by construction** - `report::asked` joins the same name onto the same
    // helper, which is the one thing the specification lane asked be true of whichever path I
    // picked.
    let at = report::records_at().join("asked.md");
    let _ = std::fs::create_dir_all(report::records_at());
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

#[cfg(test)]
mod tests {
    use super::*;

    /// **A record states the verdict, the identifier, and the behaviour - and nothing else.**
    ///
    /// `spec/README.md` rule 3: *a record names its verdict and carries the behaviour that verdict
    /// is about - the rows, canonical, without the prose.*
    ///
    /// # Driven over every test in the tree, because one would prove the shape and not the rule
    ///
    /// **A single example would pass on a writer that dropped a section**, or that kept the prose
    /// of tests whose comments happen to be short. This folds all 57 and asserts the three
    /// properties of each.
    #[test]
    fn a_record_is_the_verdict_the_name_and_the_rows() {
        let mut built = 0;
        // **`every_test` returns the file name and the handler's `name` is the stem**, which is
        // the unit a record is identified by - `{test name:a-bin-is-built-from-labor-and-metal}`.
        // **The first version joined `.4x` onto a name that already had it**, and the panic named
        // `...taken.4x.4x` - which would also have put the extension inside the `{test name:}`
        // row, so the identifier the rename check turns on would have been wrong.
        for file in report::every_test() {
            let name = file.trim_end_matches(".4x").to_string();
            let from = report::tests_at().join(&file);
            let text = std::fs::read_to_string(&from)
                .unwrap_or_else(|why| panic!("{}: {why}", from.display()));
            let said = report::render::record_for(&name, &text, "approved")
                .unwrap_or_else(|why| panic!("{name}: {why}"));

            // **The verdict leads**, so a reader and `verdict_of` meet it before anything else.
            assert!(
                said.starts_with("{verdict state:approved}\n{test name:"),
                "`{name}` does not open with its verdict and its name:\n{said}"
            );
            // **One verdict and one name**, because two of either says more than one thing about
            // one test.
            assert_eq!(said.matches("{verdict ").count(), 1, "{name}");
            assert_eq!(said.matches("{test name:").count(), 1, "{name}");
            // **No prose.** Every line is a row, a section marker or blank - a comment explains
            // and does not decide, so it is not part of what he approved.
            let prose: Vec<&str> = said
                .lines()
                .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('{'))
                .collect();
            assert!(prose.is_empty(), "`{name}` carries prose: {prose:?}");
            // **And the behaviour is there**, which the three above do not say between them: a
            // record of a verdict and a name and nothing else would pass all of them.
            assert!(
                said.contains("{given}") && said.contains("{when}"),
                "`{name}` has no behaviour in it:\n{said}"
            );
            built += 1;
        }
        assert!(built >= 40, "only {built} record(s) were built");
    }

    /// **The startup check passes, driven by the suite rather than by a person starting a
    /// server.**
    ///
    /// `S-240` is why this exists. `spec/tests/` split and this file said the prefix three times -
    /// one with `rule/` and two without - so `every_test_is_browsable` refused to start and Sean
    /// could not review a test. **The check was working exactly as designed**: loudly, at startup,
    /// before the first page, because a server that answers 404 on every link looks like one that
    /// works until somebody clicks.
    ///
    /// **What had no check is that nothing drives the example.** `87dd8cc5` swept the thirteenth
    /// place out of twelve because the compiler and the suite could see twelve; this one is a
    /// runtime assertion in a program a person starts. **An example a person drives is the one
    /// thing no check drives** - the same gap as `the_suite_reads_what_the_writer_writes` having
    /// had no home, one file over.
    ///
    /// **So the suite drives the startup now**, with the same two inputs `main` builds.
    #[test]
    fn the_startup_check_passes_before_anybody_starts_the_server() {
        let known: Vec<String> = report::every_test()
            .into_iter()
            .map(|name| name.trim_end_matches(".4x").to_string())
            .collect();
        let browsable = browsable();
        assert!(known.len() > 40, "only {} test(s) known", known.len());
        assert!(
            browsable.len() > 40,
            "only {} address(es) found",
            browsable.len()
        );
        // **Panics with the names if it fails**, which is the whole of the startup behaviour.
        every_test_is_browsable(&known, &browsable);
    }

    /// **A record the writer writes agrees with the test it was written from.**
    ///
    /// **This is the half that had no check and the one that would have cost him every approval.**
    /// The writer drops the prose - `P-600`: *a comment explains and does not decide* - and
    /// `report::drift` compared whole lines including comments. **Measured before the comparison
    /// was changed: `status=drifted`, fifteen lines, on a record built from an unmodified test.**
    /// So the first test he approved would have read as drifted at once.
    ///
    /// **Over every test rather than one**, because a writer and a comparison can agree about one
    /// file's shape and disagree about a section only some tests have.
    #[test]
    fn a_record_the_writer_writes_agrees_with_its_test() {
        let mut agreed = 0;
        for file in report::every_test() {
            let name = file.trim_end_matches(".4x").to_string();
            let text = std::fs::read_to_string(report::tests_at().join(&file))
                .unwrap_or_else(|why| panic!("{file}: {why}"));
            let said = report::render::record_for(&name, &text, "approved")
                .unwrap_or_else(|why| panic!("{name}: {why}"));
            let (status, lines) = report::drift(Some(&said), &text);
            assert_eq!(
                status,
                "reviewed",
                "`{name}`: a record written from this test does not agree with it:
  {}",
                lines
                    .iter()
                    .map(|(_, it)| it.as_str())
                    .collect::<Vec<_>>()
                    .join(
                        "
  "
                    )
            );
            agreed += 1;
        }
        assert!(agreed >= 40, "only {agreed} compared");
    }

    /// **Denying writes a record the readers treat as not binding** - `E-2`.
    ///
    /// `spec/README.md` rule 3: *a record saying `denied` means it is not* bound. **The gesture is
    /// the whole of this capability** - the format and the reader already existed, so what was
    /// missing was a way for him to produce one.
    ///
    /// # The two halves are run against each other rather than each alone
    ///
    /// **A writer that produced a plausible denial no reader recognised would pass its own
    /// checks**, which is the shape that cost a near-miss on the approval side the same day: the
    /// writer dropped prose, the comparison compared prose, and neither test noticed.
    #[test]
    fn denying_writes_a_record_that_binds_nothing() {
        let file = report::every_test().into_iter().next().expect("a test");
        let name = file.trim_end_matches(".4x").to_string();
        let text = std::fs::read_to_string(report::tests_at().join(&file)).expect("the test");

        let denied = report::render::record_for(&name, &text, "denied").expect("a denial");
        let approved = report::render::record_for(&name, &text, "approved").expect("an approval");

        // **The reader the suite uses**, which is what decides whether the engine is held to it.
        assert_eq!(
            report::render::verdict_of(&denied),
            Ok(report::render::Verdict::Denied)
        );
        assert_eq!(
            report::render::verdict_of(&approved),
            Ok(report::render::Verdict::Approved)
        );

        // **The two differ in the verdict and in nothing else.** A denial carries the behaviour it
        // is about, so a later reader can tell which test he turned down once the test has moved
        // under it - and so that the only difference is the one word that means something.
        assert_eq!(
            denied.replace("state:denied", "state:approved"),
            approved,
            "a denial and an approval of one test differ by more than the verdict"
        );

        // **And the comparison still says the rows agree**, because a denial is about the same
        // behaviour: `report::drift` answers *do these say the same thing*, and the verdict is
        // layered on top of it by `review_of` rather than inside it.
        assert_eq!(report::drift(Some(&denied), &text).0, "reviewed");
    }

    /// **Every case is on the page once, the suites are told apart, and only two offer no
    /// control** - `E-4`.
    ///
    /// *A page that lists 222 rows flat is not this capability met - the rule tests, the
    /// interface tests and the four regression suites are distinguishable without my counting.*
    ///
    /// # Driven over every case rather than over one
    ///
    /// **A check that found one case on the page would go on passing when a suite stopped being
    /// rendered.** So this ranges over what is on disk and asserts the count, which is the pair
    /// `CLAUDE.md` asks for: *check the rule over every case, not on one case, and assert how
    /// many cases there were.*
    ///
    /// **And both populations are asserted non-empty**, because *shown and no control* is
    /// vacuously true of a suite with no cases, and *markable* is vacuously true of no suites.
    #[test]
    fn every_case_is_on_the_page_once_and_its_suite_says_what_it_offers() {
        let built = report::build(true).page;
        let suites = report::every_case();
        assert_eq!(suites.len(), 4, "four regression suites");

        let (mut listed, mut markable, mut informational) = (0, 0, 0);
        for (suite, cases) in &suites {
            assert!(
                built.contains(&format!("data-suite=\"{suite}\"")),
                "`{suite}` is not its own fold, so the suites are not distinguishable"
            );
            assert!(!cases.is_empty(), "`{suite}` has no cases to be shown");
            let shown_only = report::SHOWN_ONLY.contains(&suite.as_str());
            if shown_only {
                informational += cases.len();
            } else {
                markable += cases.len();
            }
            for case in cases {
                let name = format!("{suite}/{}", case.trim_end_matches(".4x"));
                let at = format!("data-case=\"{name}\"");
                assert_eq!(
                    built.matches(&at).count(),
                    1,
                    "`{name}` is on the page {} time(s)",
                    built.matches(&at).count()
                );
                // **Linked whether or not it is markable**, which is the half of his instruction
                // about the informational suites: *we can even link to them if it helps with
                // comprehensibility.*
                assert!(
                    built.contains(&format!("regression/{name}.4x")),
                    "`{name}` is listed and not linked"
                );
                listed += 1;
            }
        }

        assert!(
            markable > 0 && informational > 0,
            "one kind of row, not two"
        );

        // **An input is not a case, and `world.4x` is the one there is.** A verdict on it would
        // observe nothing - no command, no `{then}` - and `{regenerate}` on it would rewrite the
        // world the other thirty-six are compared against.
        //
        // **Asserted against the file being on disk**, so this cannot pass by the file having
        // been renamed: a check that only asked *is it absent from the page* would go green the
        // day `world.4x` stopped existing, which is a count over nothing.
        assert!(
            report::cases_at()
                .join("scenario")
                .join("world.4x")
                .exists(),
            "world.4x is gone, so excluding it from the page proves nothing"
        );
        assert!(
            !built.contains("data-case=\"scenario/world\""),
            "`world.4x` is offered as a case, and a verdict on it observes nothing"
        );
        assert_eq!(listed, 165, "the cases, with the world they load excluded");
        assert_eq!(listed, markable + informational);
        // **Three suites shown and one reviewed**, which is his *I was expecting to review 3
        // things... everything else was to be informational only* read against his earlier
        // answer about `types/` and `primitives/`.
        assert_eq!(
            informational, 129,
            "the three suites he asked to be shown without a control"
        );
        assert_eq!(
            markable, 36,
            "the regression scenario, which is the one he reads"
        );

        // **The control follows the suite and not the case.** A case in an informational suite
        // offers nothing to press; one in a markable suite offers all three.
        for (suite, cases) in &suites {
            let shown_only = report::SHOWN_ONLY.contains(&suite.as_str());
            let case = cases.first().expect("a case");
            let name = format!("{suite}/{}", case.trim_end_matches(".4x"));
            let at = built
                .find(&format!("data-case=\"{name}\""))
                .expect("listed");
            let row = &built[at..][..built[at..].find("</li>").expect("a row")];
            // **The three names as the page writes them**, not as a sentence says them: `deny`
            // is not a prefix of `denied`, and a predicate built from the English rather than
            // the attribute reported `rules` as offering nothing.
            // **Regenerate is unoffered and the gesture survives**, so the page has two
            // controls and `reviewed/cases.4x` still takes three relations.
            assert!(
                !row.contains("data-mark=\"regenerate\""),
                "`{suite}` offers a regenerate button, which he asked to drop"
            );
            for control in ["approved", "denied"] {
                assert_eq!(
                    row.contains(&format!("data-mark=\"{control}\"")),
                    !shown_only,
                    "`{suite}` offers the wrong controls: `{control}`"
                );
            }
        }
    }

    /// **The page he opens away from this machine can write, and carries no secret** - `E-4`.
    ///
    /// *I open a page away from this machine, mark a rule test and a regression case, and a
    /// terminal here sees both without my touching git.*
    ///
    /// # What this can and cannot check
    ///
    /// **Measured: the page carries the writer, every record it would write, and no token.** The
    /// round trip through GitHub is his observation and not a test's - which is what makes this
    /// capability one only a person can vet.
    ///
    /// **So this asserts the half that would be false if the work had not been done**, and says
    /// so rather than implying it covers the rest.
    #[test]
    fn the_page_that_has_no_server_writes_through_the_api_and_holds_no_secret() {
        let hosted = report::build(false).page;
        let served = report::build(true).page;

        // **One script or the other, never both** - two writers would re-send what the first
        // already wrote.
        assert!(
            hosted.contains("api.github.com"),
            "the hosted page cannot write"
        );
        assert!(
            !served.contains("api.github.com"),
            "the served page would write twice"
        );

        // **Every markable test carries the bytes it would commit**, because there is no server
        // to ask for them. The body is the record without its verdict line.
        let tests = report::every_test().len();
        assert!(
            tests > 0,
            "no tests, so carrying their records proves nothing"
        );
        assert_eq!(
            hosted.matches("data-record=").count(),
            tests,
            "a test offers a button and the page does not know what it would write"
        );

        // **No secret, now or by accident later.** The token is his, lives in his browser, and
        // nothing generates it into a file this repository tracks.
        for leak in ["ghp_", "github_pat_", "Authorization: Bearer ey"] {
            assert!(!hosted.contains(leak), "`{leak}` is in a generated page");
        }

        // **Two controls and not three**, which is him dropping the regenerate feature. The
        // gesture survives in `reviewed/cases.4x`; what goes is the button.
        assert_eq!(hosted.matches("data-mark=\"regenerate\"").count(), 0);
        // **Thirty-six buttons are in the markup and fifty-seven more are made by the script**,
        // so this number is about the cases alone and says so. The specification lane counted
        // these 72 and nearly refused the capability, reading *the rule tests have no control*
        // off a page whose rule-test controls do not exist until it is opened.
        assert_eq!(
            hosted.matches("data-mark=\"approved\"").count(),
            36,
            "the case buttons, which are the ones written into the markup"
        );

        // **The selector and the markup are probed against each other**, because the half that
        // creates a test's buttons is a string in a script and nothing else reads it. **A
        // renamed attribute would leave fifty-seven cards with no control and every count
        // above unchanged** - the page would be inert and green.
        //
        // **This is still reading the input and it says so.** What it rules out is the two
        // halves disagreeing, which is the failure that can happen without anyone touching the
        // script; what it cannot rule out is the script being wrong in a way the markup agrees
        // with, and that is his observation.
        for attribute in ["data-test", "data-record"] {
            assert!(
                hosted.contains(&format!("[{attribute}]")),
                "the writer does not select on `{attribute}`, so no test gets a control"
            );
            assert!(
                hosted.contains(&format!("{attribute}=")),
                "no card carries `{attribute}`, so the writer's selector matches nothing"
            );
        }
    }

    /// **The canonical order does not depend on anything editable, and converting changes no
    /// approval** - `E-1`.
    ///
    /// `P-606`: *the order that function puts columns in must not depend on anything editable...
    /// an order taken from the schema's `seq:` would mean renumbering those cleared every
    /// approval I have given.*
    ///
    /// # What a conversion may not do, asserted over all fifty-seven
    ///
    /// **It may not change what a record says.** Sean: *the order of the columns is not
    /// significant, so this should not make tests different* - so every converted record must
    /// still agree with its test, and the one the route refuses is the one where it would not.
    #[test]
    fn converting_changes_the_order_and_not_what_a_record_says() {
        let mut moved = 0;
        let mut compared = 0;
        for file in report::every_test() {
            let name = file.trim_end_matches(".4x").to_string();
            let text = std::fs::read_to_string(report::tests_at().join(&file))
                .unwrap_or_else(|why| panic!("{file}: {why}"));
            let now = report::render::record_for(&name, &text, "approved")
                .unwrap_or_else(|why| panic!("{name}: {why}"));

            // **Still agrees with the test**, which is the whole of what a conversion may not
            // break.
            assert_eq!(
                report::drift(Some(&now), &text).0,
                "reviewed",
                "`{name}` stops agreeing with its test once converted"
            );
            compared += 1;

            // **And the order is the one `spec/console.md` states**, not the schema's: wherever
            // a row carries an `id`, it leads.
            for line in now.lines().map(str::trim) {
                let Some(inside) = line.strip_prefix('{').and_then(|it| it.split('}').next())
                else {
                    continue;
                };
                let mut words = inside.split(' ').skip(1).filter(|it| !it.is_empty());
                if let Some(first) = words.next()
                    && inside.contains(" id:")
                {
                    assert!(
                        first.starts_with("id:") || first.starts_with("name:"),
                        "`{name}`: `{line}` carries an id and does not lead with it"
                    );
                }
            }
            if let Some(record) = std::fs::read_to_string(report::records_at().join(&file)).ok()
                && record != now
            {
                moved += 1;
            }
        }
        assert!(compared >= 40, "only {compared} compared");
        // **Said rather than asserted**: how many move depends on whether he has run the
        // conversion, and both numbers are correct states.
        println!("{moved} of {compared} record(s) would move");
    }

    /// **What the writer produces reads as approved**, which is the one thing the readers need of
    /// it.
    ///
    /// **The writer and `verdict_of` are the two halves of `P-605`** and nothing else makes them
    /// meet: one is in an example that a person drives through a browser, the other in the suite.
    /// **A record this writes that the suite could not read would be found by Sean losing an
    /// approval**, which is the expensive way.
    #[test]
    fn the_suite_reads_what_the_writer_writes() {
        let file = report::every_test()
            .into_iter()
            .next()
            .expect("a test to record");
        let name = file.trim_end_matches(".4x").to_string();
        let text = std::fs::read_to_string(report::tests_at().join(&file)).expect("the test");
        let said = report::render::record_for(&name, &text, "approved").expect("a record");
        assert_eq!(
            report::render::verdict_of(&said),
            Ok(report::render::Verdict::Approved)
        );
    }
}
