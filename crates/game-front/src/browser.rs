//! The data browser: every entity in the game and its components, read directly.
//!
//! Two rules from `docs/architecture.md` shape it:
//!
//! - **It names things by their model id** (rule 8). A Bevy entity id is reused and is
//!   not stable across runs, so it could never be what `show territory 5` also names.
//! - **It does not write.** There is one function and therefore one path to it, so a
//!   surface that changed something directly would be a second way for state to move.
//!   Everything that changes the game goes through the console, as a command. The
//!   signature says so: a shared reference in, a string out.

use game_console::Session;

/// Every entity and its components, named the way the console names them.
pub fn browse(session: &Session) -> String {
    let mut lines = vec![
        "every entity in the game, by its model id".to_string(),
        "these are the ids `show` answers to, not the engine's - see architecture rule 8"
            .to_string(),
        String::new(),
    ];
    for entry in session.entities() {
        lines.push(format!("{} {}", entry.kind, entry.id));
        let parts: Vec<String> = entry
            .components
            .iter()
            .filter(|(_, value)| value != "none" && value != "0")
            .map(|(name, value)| format!("{name} {value}"))
            .collect();
        if parts.is_empty() {
            lines.push("    empty".to_string());
        } else {
            for chunk in parts.chunks(4) {
                lines.push(format!("    {}", chunk.join("   ")));
            }
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::Console;

    /// The browser reads; it never writes. Rendering it cannot change the game.
    #[test]
    fn opening_the_browser_changes_nothing() {
        let console = Console::new();
        let before = console.session.game.clone();
        let _ = browse(&console.session);
        assert_eq!(console.session.game, before);
        assert_eq!(console.generation(), console.generation());
    }

    #[test]
    fn the_browser_names_territories_by_their_model_id() {
        let console = Console::new();
        let shown = browse(&console.session);
        assert!(shown.contains("territory 1"), "{shown}");
        assert!(shown.contains("model id"), "it says so, too");
    }

    /// *Every* entity, not the first screenful.
    ///
    /// It used to be truncated to forty lines on the reasoning that a browser is a window
    /// and what does not fit is not shown. That was the Bevy panel's limit talking - text
    /// drawn as glyphs on a canvas does not scroll. Out of the engine it is an element on
    /// a page and a stream on a terminal, and both scroll, so the truncation was hiding
    /// nine territories out of twelve for no remaining reason. `spec/interface.md` asks
    /// for *every* entity in the game.
    #[test]
    fn every_territory_is_listed_and_not_just_the_first_few() {
        let console = Console::new();
        let shown = browse(&console.session);
        for id in 1..=12 {
            assert!(
                shown.contains(&format!("territory {id}\n")),
                "territory {id} is missing"
            );
        }
    }
}
