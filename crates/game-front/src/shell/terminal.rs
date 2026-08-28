//! The console and the data browser on stdin and stdout.
//!
//! A desktop console is a terminal. That is not a compromise for want of widgets - it is
//! what `spec/interface.md` means by *a console is a terminal on the desktop and part of
//! the page on the web*. Everything a person wants from a console is already there:
//! history, selection, copy and paste, scrollback, and whatever their shell adds.
//!
//! The engine owns the main thread from the moment its event loop starts, so this reads
//! on a thread of its own and shares the one [`Console`](crate::Console) through
//! [`crate::shell::with`].

use std::io::{BufRead, Write};

use crate::Surface;

/// What a line typed at the terminal turned out to be.
///
/// Kept apart from running it so it can be tested without stdin: the whole of the
/// terminal's own language is this function, and it is three words long.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    /// Go to a surface, and show it.
    Reach(Surface),
    /// Give it to the console.
    Command(String),
    /// Nothing was typed.
    Blank,
}

/// Reads one line the way the terminal reads it.
///
/// A leading `/` is what keeps the three surface names clear of the command language:
/// `spec/console.md` says a command is a verb followed by arguments, and no verb can
/// begin with a slash, so `/browser` can never be mistaken for one - now or after the
/// language grows.
pub fn read(line: &str) -> Line {
    let line = line.trim();
    if line.is_empty() {
        return Line::Blank;
    }
    match line.strip_prefix('/').and_then(Surface::called) {
        Some(surface) => Line::Reach(surface),
        None => Line::Command(line.to_string()),
    }
}

/// What the terminal prints in answer to a line.
///
/// Separated from printing it for the same reason [`read`] is separated from reading:
/// so what the terminal says can be asserted on without a terminal.
pub fn answer(line: &str) -> String {
    match read(line) {
        Line::Blank => String::new(),
        Line::Reach(Surface::Browser) => crate::shell::browser(),
        Line::Reach(Surface::Game) => {
            "the game is in the window. /console and /browser are here.".to_string()
        }
        Line::Reach(Surface::Console) => "this is the console.".to_string(),
        // Only what this line said. A terminal keeps its own scrollback, so reprinting
        // the transcript would print the whole game again every time.
        Line::Command(command) => crate::shell::with(|console| console.submit(&command)).join("\n"),
    }
}

/// The greeting, printed once.
pub fn greeting() -> String {
    [
        "game4x. The planet is in the window; this is the console.",
        "`help` lists every command. /browser lists every entity, /game says where it is.",
    ]
    .join("\n")
}

/// Reads lines until stdin ends, answering each.
///
/// Started on its own thread by the composition root, because the engine's event loop
/// never returns.
pub fn serve() {
    let input = std::io::stdin();
    let mut output = std::io::stdout();
    let _ = writeln!(output, "{}", greeting());
    loop {
        let _ = write!(output, "> ");
        let _ = output.flush();
        let mut line = String::new();
        match input.lock().read_line(&mut line) {
            // End of input. The window is still there, so this ends the console rather
            // than the program.
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        let said = answer(&line);
        if !said.is_empty() {
            let _ = writeln!(output, "{said}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_leading_slash_reaches_a_surface_and_nothing_else_does() {
        assert_eq!(read("/browser"), Line::Reach(Surface::Browser));
        assert_eq!(read("  /game  "), Line::Reach(Surface::Game));
        assert_eq!(read("/console"), Line::Reach(Surface::Console));
        assert_eq!(read(""), Line::Blank);
        assert_eq!(read("   "), Line::Blank);
    }

    /// A word that happens to name a surface is still a command. Only the slash reaches
    /// the front end, so the command language can never lose a verb to it.
    #[test]
    fn a_surface_name_without_a_slash_is_a_command() {
        assert_eq!(read("browser"), Line::Command("browser".to_string()));
        assert_eq!(
            read("show territory 5"),
            Line::Command("show territory 5".to_string())
        );
    }

    /// A slash that names nothing is a command too, so it fails at the parser and says
    /// where, rather than being swallowed here.
    #[test]
    fn a_slash_that_names_no_surface_is_left_to_the_console() {
        assert_eq!(read("/nowhere"), Line::Command("/nowhere".to_string()));
    }

    /// Answering prints what the line said, not the whole game so far.
    #[test]
    fn a_command_prints_only_what_it_said() {
        let said = answer("show territory 1");
        assert!(said.contains("territory 1"), "{said}");
        assert!(
            !said.contains("> show territory 1"),
            "the echo belongs to the terminal, not to us: {said}"
        );
        assert!(
            said.lines().count() < 20,
            "the whole game came back: {said}"
        );
    }

    #[test]
    fn the_browser_is_reachable_from_the_terminal() {
        let said = answer("/browser");
        assert!(said.contains("territory 12"), "{said}");
    }

    /// Every surface says something, so none of the three is a dead end.
    #[test]
    fn every_surface_can_be_reached() {
        for surface in Surface::ALL {
            let said = answer(&format!("/{}", surface.name()));
            assert!(!said.is_empty(), "`/{}` said nothing", surface.name());
        }
    }
}
