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

    /// The surface this word names, exactly.
    ///
    /// The word, not the line: stripping a leading `/` is the caller's job. Being strict
    /// here is what lets the same function serve two opposite questions - *is this line a
    /// surface* and *did somebody mean a surface and leave the slash off* - which a
    /// lenient version could not tell apart.
    pub fn named(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|surface| surface.name() == word.trim())
    }

    /// Every surface's name, for saying which ones there are.
    pub fn names() -> String {
        Self::ALL
            .into_iter()
            .map(Self::name)
            .collect::<Vec<_>>()
            .join(", ")
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

    /// A surface is named by its own name and by nothing else. The slash belongs to the
    /// line, not to the name, so this must not accept one.
    #[test]
    fn a_surface_is_named_by_its_own_name() {
        for surface in Surface::ALL {
            assert_eq!(Surface::named(surface.name()), Some(surface));
            assert_eq!(
                Surface::named(&format!("/{}", surface.name())),
                None,
                "the slash is the line's, not the name's"
            );
        }
        assert_eq!(Surface::named("elsewhere"), None);
    }

    /// What `/` answers with, and what `help` points at.
    #[test]
    fn the_surfaces_can_say_which_ones_there_are() {
        assert_eq!(Surface::names(), "game, console, browser");
    }
}
