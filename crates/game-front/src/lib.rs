//! The front end, with no engine in it.
//!
//! `spec/interface.md` asks for three surfaces - the game itself, the console, and the
//! data browser - all reachable from the front end, in every build, and adds that *how a
//! thing is presented, and how the user acts on it, may follow the platform it runs on*
//! while what the user can do stays the same.
//!
//! That is the seam this crate is cut along. What the console *does* is here, once. How
//! it is reached is a [`shell`], and there is one per platform:
//!
//! | Platform | Console and browser are                                    |
//! | -------- | ---------------------------------------------------------- |
//! | Web      | elements on the page, driven from JavaScript through wasm  |
//! | Desktop  | stdin and stdout                                           |
//!
//! # Why they are not drawn by the engine
//!
//! They were, and every one of these was broken by it at once: the clipboard, because
//! glyphs on a canvas are not text and cannot be copied or pasted; the soft keyboard,
//! because a canvas is not a text field and iOS will not raise one for it; selection, for
//! the same reason; and tapping a label, because a Bevy `Text` node has no picking
//! backend and a touch never reaches it.
//!
//! None of those is a bug to be fixed in a canvas. A text field, a scroll region and a
//! button already exist on both platforms, do all of this correctly, and are what the
//! user's own accessibility settings apply to.
//!
//! # One Session, one door
//!
//! There is exactly one [`Console`] in a running program, held by the [`shell`], and
//! `Session::run` is still the only way game state moves. The engine never reaches it:
//! it watches [`Console::generation`] and rebuilds when the number changes. That keeps
//! `spec/invariants.md`'s *the game is one function* true across a boundary where the
//! caller is a web page.

pub mod browser;
pub mod console;
pub mod library;
pub mod shell;

pub use browser::browse;
pub use console::Console;
pub use library::library;

/// The three surfaces `spec/interface.md` lists.
///
/// Which one is in front is a fact about the front end, not about the game, so it is not
/// a transition and it is not in the history.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Surface {
    Game,
    Console,
    Browser,
}

impl Surface {
    pub const ALL: [Self; 3] = [Self::Game, Self::Console, Self::Browser];

    pub fn name(self) -> &'static str {
        match self {
            Surface::Game => "game",
            Surface::Console => "console",
            Surface::Browser => "browser",
        }
    }

    /// The name a shell answers to, as the user types or taps it.
    ///
    /// Neither `spec/` nor `releases/` names a binding for reaching a surface, so this is
    /// the one thing here that is invented rather than followed. It is at least invented
    /// once: the page's buttons and the terminal's `/game` are the same three names.
    pub fn called(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|surface| surface.name() == word.trim().trim_start_matches('/'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every surface is reachable, so none can strand a player.
    #[test]
    fn the_three_surfaces_are_the_three_the_specification_lists() {
        let names: Vec<&str> = Surface::ALL.into_iter().map(Surface::name).collect();
        assert_eq!(names, ["game", "console", "browser"]);
    }

    /// A shell's way of naming a surface is the same on every platform, with or without
    /// the slash a terminal wants to keep it clear of the command language.
    #[test]
    fn a_surface_is_reached_by_its_own_name() {
        for surface in Surface::ALL {
            assert_eq!(Surface::called(surface.name()), Some(surface));
            assert_eq!(
                Surface::called(&format!("/{}", surface.name())),
                Some(surface)
            );
        }
        assert_eq!(Surface::called("elsewhere"), None);
    }
}
