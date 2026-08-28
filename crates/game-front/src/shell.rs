//! The one [`Console`], and the platform that reaches it.
//!
//! [`with`] is the only way to touch it, whichever platform this is. Above that sit two
//! shells, one per target, and nothing else in the program holds a `Console` of its own -
//! which is what makes "one Session outside the engine" a fact rather than an intention.
//!
//! The sharing differs because the platforms do:
//!
//! - **Web.** One thread, and the page calls in. A `thread_local` is enough, and a
//!   `Mutex` would be a lie about what is happening.
//! - **Desktop.** The engine owns the main thread once its event loop starts, so reading
//!   stdin needs a thread of its own and the two share a lock.
//!
//! Neither shell is chosen at run time. A build is for one target, so the other shell is
//! not compiled - which is also why `wasm-bindgen` is a dependency of one target and not
//! of the crate.

use crate::Console;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub mod terminal;

#[cfg(target_arch = "wasm32")]
mod held {
    use super::Console;
    use std::cell::RefCell;

    thread_local! {
        static CONSOLE: RefCell<Console> = RefCell::new(Console::new());
    }

    pub fn with<R>(act: impl FnOnce(&mut Console) -> R) -> R {
        CONSOLE.with(|console| act(&mut console.borrow_mut()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod held {
    use super::Console;
    use std::sync::{Mutex, OnceLock};

    static CONSOLE: OnceLock<Mutex<Console>> = OnceLock::new();

    pub fn with<R>(act: impl FnOnce(&mut Console) -> R) -> R {
        let console = CONSOLE.get_or_init(|| Mutex::new(Console::new()));
        // A poisoned lock means a panic in an earlier call, and the state it left behind
        // is still a game - every transition that succeeded, applied in order. Refusing
        // to look at it would turn one panic into a dead program.
        let mut held = console
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        act(&mut held)
    }
}

/// Reaches the one console. Every shell, and the engine's own polling, goes through here.
pub fn with<R>(act: impl FnOnce(&mut Console) -> R) -> R {
    held::with(act)
}

/// Runs a line at the console and gives back everything the transcript now holds.
pub fn submit(line: &str) -> String {
    with(|console| {
        console.submit(line);
        console.transcript()
    })
}

/// The whole transcript, without running anything.
pub fn transcript() -> String {
    with(|console| console.transcript())
}

/// The surface the most recent line named, if it named one.
///
/// Read straight after [`submit`] by a shell that has to act on it. The web shell reaches
/// this through a doorway that carries nothing but strings, so it cannot be given back as
/// part of the transcript.
pub fn reached() -> Option<crate::Surface> {
    with(|console| console.reached())
}

/// The data browser's text, without running anything.
pub fn browser() -> String {
    with(|console| crate::browse(&console.session))
}

/// How many times the game state has moved.
///
/// This is what the engine watches. It is deliberately a number rather than a callback:
/// on the web the change happens on the page's call stack, not the engine's, and there
/// is nothing safe to call back into from there.
pub fn generation() -> u64 {
    with(|console| console.generation())
}

/// How many territories the planet has, or none if there is no planet yet.
pub fn territory_count() -> Option<usize> {
    with(|console| console.territory_count())
}

/// Serialises the tests that touch the one console.
///
/// The console is a process-wide static and the test runner runs tests in parallel
/// threads of one process, so two tests asserting about it race over the very thing being
/// asserted - one appending to the transcript while another compares it, or resetting
/// what the last line reached while another reads it back.
///
/// A guard rather than one enormous test, so each thing being asserted keeps its own name
/// and its own reason for existing. Nothing outside a test takes it.
#[cfg(test)]
pub(crate) fn exclusively<R>(act: impl FnOnce() -> R) -> R {
    use std::sync::{Mutex, OnceLock};
    static ONE_AT_A_TIME: OnceLock<Mutex<()>> = OnceLock::new();
    let held = ONE_AT_A_TIME
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let outcome = act();
    drop(held);
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    /// There is one console, and every way in reaches it.
    ///
    /// One test rather than several, on purpose: the console is a process-wide static, and
    /// the test runner runs tests in parallel threads of one process. Split up, these
    /// would be four tests racing each other over the very thing being asserted about.
    #[test]
    fn the_shell_holds_exactly_one_console() {
        exclusively(|| {
            assert_eq!(
                territory_count(),
                Some(12),
                "it opens on the release's world"
            );

            // Asking is not doing. The browser reads through the same handle and moves nothing.
            let before = generation();
            let read = browser();
            assert!(read.contains("territory 1"), "{read}");
            assert_eq!(generation(), before, "reading the browser moved the game");

            // Running does move it, and both views of the transcript are of the one console.
            let shown = submit("end turn");
            assert!(shown.contains("> end turn"), "{shown}");
            assert!(generation() > before, "the state moved and nobody noticed");
            assert_eq!(transcript(), shown, "two views of one transcript disagreed");
            assert_eq!(reached(), None, "a command is not a surface");

            // And a slash line is readable back through the same handle, which is the only
            // way the web shell can learn about it.
            submit("/browser");
            assert_eq!(reached(), Some(crate::Surface::Browser));
        });
    }
}
