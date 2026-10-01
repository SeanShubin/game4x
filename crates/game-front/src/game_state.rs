//! The game state as an interface, and the two narrow surfaces over it.
//!
//! **Sean, 2026-09-30**: *in an object oriented language, I would have wrapped access to game
//! state in an interface, hooked up the implementation in the composition roots, and either
//! wired up that interface or implemented smaller interfaces as needed. Smaller interfaces if I
//! wanted to expose smaller surfaces to different implementations that delegate to state.*
//!
//! `S-227`, answering `C-188`, which measured what each crate actually reaches.
//!
//! # Why two rather than one
//!
//! **The sets barely overlap, and that was measured before these were written.**
//!
//! ```text
//! game-globe     generation, territory_count, resets, drawing_changes, submit
//! game-inspect   submit, change_drawing, browser, and one command answered as text
//! game4x         territory_count
//! ```
//!
//! **A single wide trait would hand `game-globe` the submit-and-read path it never calls**,
//! which is the thing narrow interfaces are for. `game4x` needs one method and gets it from
//! [`Watches`], which it already has in hand to pass on.
//!
//! # What this does not change
//!
//! **The one console is still one console.** `shell.rs` guarantees that *nothing else in the
//! program holds a `Console` of its own*, and it kept that guarantee with a process-wide value.
//! **It is kept here by the root handing out one implementation** - the same fact with a
//! different enforcer, which `C-188` said it would be.
//!
//! **The `thread_local` does not go and stops being load-bearing.** On the web the page calls in
//! through free `#[wasm_bindgen]` functions, which have nowhere to receive a handle, so
//! [`TheOneConsole`] reaches `shell::with` - **an implementation detail behind the interface
//! rather than the shape every caller adopts.**

/// What the globe needs to follow the one game.
///
/// **It watches a counter and never reaches into the state**, which `game4x`'s own wiring
/// comment already claims: *it follows the one Session by watching a counter; it never reaches
/// into it.* This is that sentence as a type.
pub trait Watches: Send + Sync + 'static {
    /// How many commands a person has typed. The globe redraws when this moves.
    fn generation(&self) -> u64;
    /// How many territories the planet has, or none if there is no planet yet.
    fn territory_count(&self) -> Option<usize>;
    /// How many times a view reset has been asked for.
    fn resets(&self) -> u64;
    /// How many times the drawing has been changed.
    fn drawing_changes(&self) -> u64;
    /// Type a line at the console, exactly as a person would.
    fn submit(&self, line: &str) -> String;
}

/// What the remote control needs to drive a run and photograph it.
///
/// **`game-inspect` is the one caller that reads the game rather than watching it**, and it is
/// the harness - so the wider of the two surfaces is the one nothing ships to a player depends
/// on.
pub trait Drives: Send + Sync + 'static {
    /// Type a line at the console.
    fn submit(&self, line: &str) -> String;
    /// Ask for the next drawing.
    fn change_drawing(&self);
    /// Every entity, as the data browser renders it.
    fn browser(&self) -> String;
    /// Run one command and give back only what it said.
    ///
    /// **Not `submit`**, which returns the whole transcript. The harness wants the answer to
    /// one question, which is what it writes into a dump beside the photograph.
    fn says(&self, command: &str) -> String;
}

/// The implementation over the one console, which is what a root hands down.
///
/// **It holds nothing.** Every method goes through [`crate::shell::with`], so this is a name for
/// the process-wide console rather than a second way to reach it - and constructing two of them
/// is harmless, because there is still one console.
///
/// **No `Resource` derive, because this crate has no engine in it** and that is the property
/// `docs/architecture.md` gives it. The wrapper a Bevy app needs is in the crate that needs it.
#[derive(Clone, Copy, Debug, Default)]
pub struct TheOneConsole;

impl Watches for TheOneConsole {
    fn generation(&self) -> u64 {
        crate::shell::generation()
    }
    fn territory_count(&self) -> Option<usize> {
        crate::shell::territory_count()
    }
    fn resets(&self) -> u64 {
        crate::shell::resets()
    }
    fn drawing_changes(&self) -> u64 {
        crate::shell::drawing_changes()
    }
    fn submit(&self, line: &str) -> String {
        crate::shell::submit(line)
    }
}

impl Drives for TheOneConsole {
    fn submit(&self, line: &str) -> String {
        crate::shell::submit(line)
    }
    fn change_drawing(&self) {
        crate::shell::change_drawing();
    }
    fn browser(&self) -> String {
        crate::shell::browser()
    }
    fn says(&self, command: &str) -> String {
        crate::shell::with(|console| {
            console
                .session
                .run(command, &crate::library())
                .map(|outcome| match outcome {
                    game_console::Outcome::Said(said) => said,
                    other => format!("{other:?}"),
                })
                .unwrap_or_else(|problem| problem.to_string())
        })
    }
}
