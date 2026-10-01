//! The two narrow surfaces over the game, and the thing having them is for.
//!
//! `S-227`, answering `C-188`. Sean, 2026-09-30: *in an object oriented language, I would have
//! wrapped access to game state in an interface, hooked up the implementation in the composition
//! roots, and either wired up that interface or implemented smaller interfaces as needed.*
//!
//! **What an interface buys is a second implementation**, and that is what these check: a caller
//! written against [`Watches`] or [`Drives`] can be driven by something that is not the one
//! console. **Before `S-227` it could not** - `game-globe` and `game-inspect` named
//! `game_front::shell::` directly, so there was nothing to substitute.

use std::sync::atomic::{AtomicU64, Ordering};

use game_front::game_state::{Drives, TheOneConsole, Watches};

/// A game that is not the one console, which is the whole point of the trait.
#[derive(Default)]
struct APretendGame {
    generation: AtomicU64,
    territories: Option<usize>,
    said: std::sync::Mutex<Vec<String>>,
}

impl Watches for APretendGame {
    fn generation(&self) -> u64 {
        self.generation.load(Ordering::Relaxed)
    }
    fn territory_count(&self) -> Option<usize> {
        self.territories
    }
    fn resets(&self) -> u64 {
        0
    }
    fn drawing_changes(&self) -> u64 {
        0
    }
    fn submit(&self, line: &str) -> String {
        self.generation.fetch_add(1, Ordering::Relaxed);
        self.said.lock().expect("the lock").push(line.to_string());
        format!("pretended: {line}")
    }
}

/// **A caller written against the trait can be handed something else**, which is the property
/// the refactor exists to create.
///
/// This stands in for `follow_the_game`, which is the system the globe redraws from: it reads a
/// counter and a territory count and nothing else. **It is driven here by a game that has never
/// run a command**, which is not possible against a process-wide console.
#[test]
fn a_caller_against_the_interface_can_be_handed_a_different_game() {
    fn redraws_at(game: &dyn Watches) -> Option<usize> {
        if game.generation() == 0 {
            return None;
        }
        game.territory_count()
    }

    let pretend = APretendGame {
        territories: Some(42),
        ..Default::default()
    };
    assert_eq!(redraws_at(&pretend), None, "nothing typed yet");
    pretend.submit("/new tiny");
    assert_eq!(redraws_at(&pretend), Some(42));
    assert_eq!(pretend.said.lock().expect("the lock").len(), 1);
}

/// **The real implementation satisfies both traits**, so the root can hand one value to two
/// plugins and they are the same game by construction rather than by a static.
///
/// `shell.rs` guarantees that *nothing else in the program holds a `Console` of its own*, and
/// `C-188` said a handed-down value keeps that by the root handing out one - **the same fact
/// with a different enforcer**. This is that sentence as a compile-time check.
#[test]
fn the_one_console_is_both_surfaces_at_once() {
    fn watching(_: &dyn Watches) {}
    fn driving(_: &dyn Drives) {}
    let game = TheOneConsole;
    watching(&game);
    driving(&game);
}

/// **The surfaces are narrow, and narrow is the requirement rather than a preference.**
///
/// `C-188` measured what each crate reaches and the sets barely overlap. **A single wide trait
/// would hand `game-globe` the submit-and-read path it never calls**, which is what Sean's
/// *smaller interfaces if I wanted to expose smaller surfaces* asks against.
///
/// Counted here so that a method added to either one is a decision somebody makes rather than
/// a drift: five and four.
#[test]
fn neither_surface_has_grown() {
    // **Counted by hand against the trait, because nothing in Rust reports a trait's method
    // count at run time.** The count is what makes the sentence above checkable at all; if a
    // sixth method is right, change the number and say why in the commit.
    let watches = [
        "generation",
        "territory_count",
        "resets",
        "drawing_changes",
        "submit",
    ];
    let drives = ["submit", "change_drawing", "browser", "says"];

    // **`assert_eq!(watches.len(), 5)` was here and said nothing** - a literal array compared
    // against its own length, which cannot fail and cannot report anything. Found by the quality
    // lens sampling three assertions of this session's; this was the one that was vacuous.
    //
    // **What it meant to assert is the shape of the two lists**, so that is what is checked: no
    // name twice, and the one method both traits share named as shared rather than left to be
    // noticed.
    assert_eq!(
        watches
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        watches.len(),
        "a name is listed twice"
    );
    assert_eq!(
        drives
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        drives.len(),
        "a name is listed twice"
    );
    let shared: Vec<&&str> = watches.iter().filter(|it| drives.contains(it)).collect();
    assert_eq!(
        shared,
        vec![&"submit"],
        "the two surfaces share exactly one method, and which one is the point"
    );

    let source = include_str!("../src/game_state.rs");
    for name in watches.iter().chain(drives.iter()) {
        assert!(
            source.contains(&format!("fn {name}(")),
            "`{name}` is listed here and not in the traits"
        );
    }
    // And the other way: every `fn` declared in a trait is one of the names above.
    // **Counted over the signature rather than over the line**, because a signature rustfmt has
    // wrapped ends its first line on a `,` and this counted it as nothing - **a count that
    // silently becomes zero**, which is the failure this whole file is about. The quality lens
    // called it reachable rather than likely; the longest signature here is 47 characters.
    //
    // **So the source is collapsed to one line first**, and a declaration is `fn ...;` with no
    // `{` before the `;` - a trait method rather than one with a body.
    let tight = source.split_whitespace().collect::<Vec<_>>().join(" ");
    let declared: Vec<&str> = tight
        .match_indices("fn ")
        .filter_map(|(at, _)| {
            let rest = &tight[at..];
            let end = rest.find(';')?;
            let body = rest.find('{').unwrap_or(usize::MAX);
            (end < body).then_some(&rest[..end])
        })
        .collect();
    assert_eq!(
        declared.len(),
        watches.len() + drives.len(),
        "the traits declare {} method(s) and this names {}: {declared:?}",
        declared.len(),
        watches.len() + drives.len()
    );
}
