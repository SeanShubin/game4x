//! `anchor replace <file> <anchor-file> <replacement-file> [--strip <prefix>] [--fold-case]`
//!
//! **The anchor and the replacement are files, and that is the point.** `CLAUDE.md`: *write a
//! script to a file before running it; never assemble one inside a shell string* - a file has
//! one level of quoting. Passing them as arguments would put the text back inside a shell
//! string, which is the failure this is a carrier for.
//!
//! `anchor edit <file> <edits-file>` applies several of them from one file, which is the shape a
//! real edit usually has - and the shape that used to send whoever was making it back to a
//! throwaway script, taking every failure this prevents with them.
//!
//! `anchor find` does the same lookup and prints the byte range without changing anything,
//! which is what to reach for when checking an anchor before trusting it.
//!
//! # Two things `--strip` does not do, both met on 2026-09-25
//!
//! **It is for matching and never for writing.** A replacement is written verbatim, so one that
//! drops the marker leaves a file whose lines are no longer comments. `--strip "//!"` with a
//! replacement of bare prose produced four lines of Rust that were not a doc comment and not
//! code, and it did it without complaint because the replacement is exactly what was asked for.
//!
//! **And a shell may eat the marker.** Git Bash rewrites an argument that looks like a path, so
//! `--strip "//!"` can arrive as something else entirely and strip nothing - after which the
//! anchor does not match and the refusal truthfully says the anchor is not in the file.
//! `MSYS_NO_PATHCONV=1` is the fix on that shell. **The tool now refuses a marker no line begins
//! with**, so the silent version of this is gone, and that refusal says a shell is the usual
//! reason.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let strip = args
        .iter()
        .position(|a| a == "--strip")
        .and_then(|at| args.get(at + 1).cloned());
    // **A flag with no value, so it is dropped from the positionals rather than skipping
    // the argument after it.** `X-40`: `spec/` capitalises kinds at the start of a sentence
    // and in every section heading, so the words most worth searching for are exactly the ones
    // a case-sensitive search misses - and it misses them by returning a clean, plausible
    // zero, which is what this tool exists to refuse.
    let how = anchor::How {
        fold_case: args.iter().any(|a| a == "--fold-case"),
    };
    let positional: Vec<&String> = {
        let mut out = Vec::new();
        let mut skip = false;
        for arg in &args {
            if skip {
                skip = false;
                continue;
            }
            if arg == "--strip" {
                skip = true;
                continue;
            }
            if arg == "--fold-case" {
                continue;
            }
            out.push(arg);
        }
        out
    };

    let read = |path: &str| -> String {
        std::fs::read_to_string(path).unwrap_or_else(|why| panic!("cannot read {path}: {why}"))
    };

    match positional.as_slice() {
        [command, file, anchor_at] if *command == "find" => {
            let text = read(file);
            match anchor::find(&text, &read(anchor_at), strip.as_deref(), how) {
                Ok((from, to)) => {
                    println!("{from}..{to}");
                    println!("{}", &text[from..to]);
                    ExitCode::SUCCESS
                }
                Err(why) => {
                    eprintln!("anchor: {file}: {why}");
                    ExitCode::FAILURE
                }
            }
        }
        [command, file, anchor_at, with_at] if *command == "replace" => {
            let text = read(file);
            match anchor::replace(
                &text,
                &read(anchor_at),
                &read(with_at),
                strip.as_deref(),
                how,
            ) {
                Ok(after) => {
                    std::fs::write(file, &after)
                        .unwrap_or_else(|why| panic!("cannot write {file}: {why}"));
                    println!("anchor: {file} rewritten");
                    ExitCode::SUCCESS
                }
                Err(why) => {
                    eprintln!("anchor: {file}: {why}");
                    ExitCode::FAILURE
                }
            }
        }
        [command, file, edits_at] if *command == "edit" => {
            let text = read(file);
            match anchor::edits(&text, &read(edits_at), strip.as_deref(), how) {
                Ok(after) => {
                    std::fs::write(file, &after)
                        .unwrap_or_else(|why| panic!("cannot write {file}: {why}"));
                    println!("anchor: {file} rewritten");
                    ExitCode::SUCCESS
                }
                Err(why) => {
                    eprintln!("anchor: {file}: {why}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => {
            eprintln!(
                "anchor find <file> <anchor-file> [--strip <prefix>] [--fold-case]\n\
                 anchor replace <file> <anchor-file> <replacement-file> [--strip <prefix>] [--fold-case]\n\
                 anchor edit <file> <edits-file> [--strip <prefix>] [--fold-case]"
            );
            ExitCode::FAILURE
        }
    }
}
