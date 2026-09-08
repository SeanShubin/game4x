//! `anchor replace <file> <anchor-file> <replacement-file> [--strip <prefix>]`
//!
//! **The anchor and the replacement are files, and that is the point.** `CLAUDE.md`: *write a
//! script to a file before running it; never assemble one inside a shell string* - a file has
//! one level of quoting. Passing them as arguments would put the text back inside a shell
//! string, which is the failure this is a carrier for.
//!
//! `anchor find` does the same lookup and prints the byte range without changing anything,
//! which is what to reach for when checking an anchor before trusting it.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let strip = args
        .iter()
        .position(|a| a == "--strip")
        .and_then(|at| args.get(at + 1).cloned());
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
            match anchor::find(&text, &read(anchor_at), strip.as_deref()) {
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
            match anchor::replace(&text, &read(anchor_at), &read(with_at), strip.as_deref()) {
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
                "anchor find <file> <anchor-file> [--strip <prefix>]\n\
                 anchor replace <file> <anchor-file> <replacement-file> [--strip <prefix>]"
            );
            ExitCode::FAILURE
        }
    }
}
