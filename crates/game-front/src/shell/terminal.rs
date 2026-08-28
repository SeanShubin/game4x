//! The console and the data browser on stdin and stdout.
//!
//! A desktop console is a terminal. That is not a compromise for want of widgets - it is
//! what `spec/interface.md` means by *a console is a terminal on the desktop and part of
//! the page on the web*. Everything a person wants from a console is already there:
//! history, selection, copy and paste, scrollback, and whatever their shell adds.
//!
//! It is also why `/browser` is not a convenience. `spec/interface.md` requires all three
//! surfaces reachable in every build, and that reaching one *never requires a gesture or a
//! key the platform may lack*. A terminal has neither a button to point at nor an F-key to
//! press, so typing is the only way any surface can be reached here. That is the whole
//! reason the slash rule exists.
//!
//! The engine owns the main thread from the moment its event loop starts, so this reads
//! on a thread of its own and shares the one [`Console`](crate::Console) through
//! [`crate::shell::with`].

use std::io::{BufRead, Write};

use crate::Surface;
use crate::console::Said;

/// What the terminal prints in answer to a line.
///
/// Separated from printing it so that what the terminal says can be asserted on without a
/// terminal. Reading the line is no longer done here: whether a line is a command or the
/// name of a surface is settled by [`crate::Console`], because it is the same question on
/// every platform.
///
/// Reaching a surface *prints* it, rather than leaving the terminal in a mode. A terminal
/// has no panels to switch between and nothing to switch back from - you ask, and it
/// answers. That difference is presentation, which `spec/interface.md` puts on this side
/// of the line.
pub fn answer(line: &str) -> String {
    crate::shell::with(|console| match console.submit(line) {
        Said::Reach(Surface::Browser) => crate::browse(&console.session),
        Said::Reach(Surface::Game) => {
            "the game is in the window; this terminal is the console.".to_string()
        }
        Said::Reach(Surface::Console) => "this is the console.".to_string(),
        Said::Spoke(said) => said.join("\n"),
    })
}

/// The greeting, printed once.
///
/// Once is the problem, which is why it is not the only place the slash forms are
/// announced: scrollback carries it away, and a person who never saw it would otherwise
/// have no route to two of the three surfaces. `help` says a slash names a surface, and a
/// bare `/` says which ones there are.
pub fn greeting() -> String {
    [
        "game4x. The planet is in the window; this is the console.",
        "`help` lists every command. A line beginning with `/` directs the front end: try `/`.",
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
    use crate::shell::exclusively;

    /// Answering prints what the line said, not the whole game so far.
    #[test]
    fn a_command_prints_only_what_it_said() {
        exclusively(|| {
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
        });
    }

    /// On a terminal, reaching the data browser is the only way it exists at all.
    #[test]
    fn the_browser_is_reachable_from_the_terminal() {
        exclusively(|| {
            let said = answer("/browser");
            assert!(said.contains("territory 12"), "{said}");
        });
    }

    /// Every surface says something, so none of the three is a dead end.
    #[test]
    fn every_surface_can_be_reached() {
        exclusively(|| {
            for surface in Surface::ALL {
                let said = answer(&format!("/{}", surface.name()));
                assert!(!said.is_empty(), "`/{}` said nothing", surface.name());
            }
        });
    }

    /// A word that happens to name a surface is still a command, and still fails - but it
    /// is told what it was nearly.
    #[test]
    fn a_surface_name_without_a_slash_is_told_about_the_slash() {
        exclusively(|| {
            let said = answer("browser");
            assert!(said.contains("/browser"), "{said}");
        });
    }

    /// The greeting scrolls away, so it must not be the only thing that mentions the
    /// slash - but while it is on screen it should still say so.
    #[test]
    fn the_greeting_says_a_slash_directs_the_front_end() {
        exclusively(|| {
            assert!(greeting().contains('/'), "{}", greeting());
        });
    }

    /// Starting a new game is reachable by typing, which on a terminal is the only way
    /// anything is reachable at all.
    #[test]
    fn a_new_game_can_be_started_from_the_terminal() {
        exclusively(|| {
            let said = answer("/new small");
            assert!(said.contains("small"), "{said}");
            // And back, so the rest of the suite finds the world it expects.
            answer("/new tiny");
        });
    }

    #[test]
    fn a_blank_line_is_answered_with_nothing() {
        exclusively(|| {
            assert_eq!(answer(""), "");
            assert_eq!(answer("   \n"), "");
        });
    }
}
