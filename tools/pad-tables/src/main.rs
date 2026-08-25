//! Walks paths and pads the markdown tables it finds, in place.
//!
//! ```text
//! pad-tables [--check] [PATH...]
//! ```
//!
//! Each PATH is a directory to walk or a single `.md` file; with none, walks the current
//! directory. Walking skips `.git`, `target`, `node_modules`, and any dot-prefixed
//! directory. Only files that actually change are written.
//!
//! `--check` writes nothing and exits non-zero if anything would change, so the tool can
//! back a CI gate or a pre-push hook without either existing yet.
//!
//! Taking explicit paths is what lets the pre-commit hook pass exactly the staged files
//! instead of reformatting the whole tree.

use pad_tables::pad_tables;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("pad-tables [--check] [PATH...]");
        println!();
        println!("  Aligns markdown table columns in place.");
        println!("  PATH may be a directory to walk or a single .md file.");
        println!("  With no arguments, walks the current directory.");
        println!("  --check  write nothing; exit 1 if any file would change.");
        return ExitCode::SUCCESS;
    }

    let check = args.iter().any(|a| a == "--check");
    let targets: Vec<String> = args.into_iter().filter(|a| a != "--check").collect();
    let targets = if targets.is_empty() {
        vec![".".to_string()]
    } else {
        targets
    };

    let mut changed = Vec::new();
    let mut missing = Vec::new();
    for target in &targets {
        let path = Path::new(target);
        if path.is_dir() {
            visit_dir(path, check, &mut changed);
        } else if path.is_file() {
            if path.extension().and_then(|e| e.to_str()) == Some("md") && process_file(path, check)
            {
                changed.push(path.display().to_string());
            }
        } else {
            missing.push(target.clone());
        }
    }

    for target in &missing {
        eprintln!("  Skipped, not found: {target}");
    }

    if changed.is_empty() {
        println!("All tables are already padded.");
        return ExitCode::SUCCESS;
    }

    let verb = if check { "Would pad" } else { "Padded" };
    for path in &changed {
        println!("  {verb}: {path}");
    }
    println!();
    println!("{} file(s).", changed.len());

    if check {
        eprintln!("Not padded. Run scripts/pad-tables to fix.");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn visit_dir(dir: &Path, check: bool, changed: &mut Vec<String>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
            visit_dir(&path, check, changed);
        } else if path.extension().and_then(|e| e.to_str()) == Some("md")
            && process_file(&path, check)
        {
            changed.push(path.display().to_string());
        }
    }
}

/// Returns true if the file's tables were not already padded.
/// Writes the padded content unless `check` is set.
fn process_file(path: &Path, check: bool) -> bool {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let padded = pad_tables(&content);
    if padded == content {
        return false;
    }
    if !check {
        fs::write(path, padded).expect("Failed to write file");
    }
    true
}
