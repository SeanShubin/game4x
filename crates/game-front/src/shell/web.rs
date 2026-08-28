//! The console and the data browser as elements on the page.
//!
//! Nothing here draws anything. It is the doorway the page calls through, and every
//! function is a thin wrapper over [`crate::shell`] - the widgets themselves are in
//! `crates/game4x/index.html`, because a text field, a scroll region and a button are
//! things a browser already has and does correctly.
//!
//! What that buys, none of which a canvas can be made to do:
//!
//! - **The clipboard.** Copy and paste work because the transcript is text and the
//!   prompt is an `<input>`, not because anything here implements them.
//! - **The soft keyboard.** iOS raises one for a focused text field. It will not raise
//!   one for a canvas, however much the canvas would like to be typed at.
//! - **Selection.** Dragging across the transcript selects it.
//! - **Tapping.** A `<button>` is reachable by touch. A Bevy `Text` node has no picking
//!   backend, so a tap on it reached nothing at all.
//!
//! Strings cross the boundary rather than structures. There is little to say and it is
//! all text, so a `String` in each direction keeps the doorway narrow enough to read.

use wasm_bindgen::prelude::wasm_bindgen;

/// Runs one line at the console and gives back the whole transcript.
///
/// The transcript comes back rather than being appended to by the page, so the page
/// never has to know how a line was rendered - or that a single command can say many
/// lines, or nothing at all.
#[wasm_bindgen]
pub fn console_submit(line: &str) -> String {
    crate::shell::submit(line)
}

/// The transcript as it stands, for first paint.
#[wasm_bindgen]
pub fn console_transcript() -> String {
    crate::shell::transcript()
}

/// The surface the most recent line named, or an empty string if it named none.
///
/// `spec/console.md` says a line beginning with `/` names a surface to go to, and that
/// rule is about a line typed at *the* console rather than at a terminal - so the page
/// has to honour it too. It cannot come back inside the transcript, which is text the
/// user wrote and read; so the page calls this straight after [`console_submit`].
///
/// Empty rather than `null`, because every other value crossing here is a string and one
/// exception is not worth the reader's attention.
#[wasm_bindgen]
pub fn console_reached() -> String {
    crate::shell::reached()
        .map(|surface| surface.name().to_string())
        .unwrap_or_default()
}

/// Asks the view to go back to its default.
///
/// `spec/planet.md` says the user can reset the view, and `spec/interface.md` says none of
/// these actions may require a key the platform may lack. `R` is exactly such a key, so
/// the page carries a control and this is what it calls.
#[wasm_bindgen]
pub fn request_view_reset() {
    crate::shell::request_reset();
}

/// Every entity in the game and its components.
#[wasm_bindgen]
pub fn browser_text() -> String {
    crate::shell::browser()
}

/// How many times the game state has moved.
///
/// The page does not use this; the engine does, from the other side. It is exported so
/// that a person with the developer tools open can see the same number the globe is
/// watching, which is the difference between "the command did nothing" and "the command
/// worked and the globe did not notice".
#[wasm_bindgen]
pub fn game_generation() -> f64 {
    // JavaScript has one number type and it is a double, so a u64 would arrive as a
    // BigInt and compare badly against anything else on the page. The counter is one per
    // command typed by a person; a double holds that exactly for longer than the sun.
    crate::shell::generation() as f64
}
