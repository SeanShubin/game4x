//! The three surfaces, and how a player moves between them.
//!
//! `spec/interface.md`: the game presents the game itself, the console, and a data
//! browser, all reachable from the front end, in every build. *Nothing is available in one
//! build and not another* - so there is no `cfg` in this file. What a browser shows and
//! what a desktop window shows are the same surfaces built by the same code, and the only
//! difference between the two builds is which target it was compiled for.
//!
//! Two rules from `docs/architecture.md` shape the browser in particular:
//!
//! - **It names things by their model id** (rule 8). A Bevy entity id is reused and is not
//!   stable across runs, so it could never be what `show territory 5` also names.
//! - **It does not write.** There is one function and therefore one path to it, so a
//!   surface that changed something directly would be a second way for state to move.
//!   Everything that changes the game goes through the console, as a command.

use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use game_console::{Embedded, Outcome, Session};

/// The command files carried in the binary.
///
/// A browser has no filesystem, so the files a desktop build could read off disk are
/// compiled in instead. Both builds get the same ones, which is what keeps `run setup`
/// meaning the same thing everywhere.
pub fn library() -> Embedded {
    Embedded::of(&[
        ("setup", include_str!("../../../commands/setup.4x")),
        ("nodes", include_str!("../../../commands/nodes.4x")),
        ("forces", include_str!("../../../commands/forces.4x")),
        ("play", include_str!("../../../commands/play.4x")),
    ])
}

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
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

    /// Which key reaches this surface. F1, F2, F3 - always available, on every surface,
    /// so no surface can trap a player on it.
    pub fn key(self) -> KeyCode {
        match self {
            Surface::Game => KeyCode::F1,
            Surface::Console => KeyCode::F2,
            Surface::Browser => KeyCode::F3,
        }
    }
}

/// The game, and everything typed at it.
#[derive(Resource)]
pub struct Console {
    pub session: Session,
    /// What has been said, oldest first.
    pub transcript: Vec<String>,
    /// The line being typed.
    pub typing: String,
}

impl Default for Console {
    fn default() -> Self {
        let mut console = Self {
            session: Session::new(),
            transcript: vec![
                "game4x. F1 game, F2 console, F3 browser.".to_string(),
                "`help` lists every command. `run setup` builds the world of the first release."
                    .to_string(),
            ],
            typing: String::new(),
        };
        // Open on a world rather than on nothing, so every surface has something in it.
        // This is the release's own setup file, run through the console like anything
        // else - there is no other way to build a world.
        for line in ["run setup", "start"] {
            console.submit(line.to_string());
        }
        console
    }
}

impl Console {
    /// Runs a line and records what happened, whether it worked or not.
    pub fn submit(&mut self, line: String) {
        if line.trim().is_empty() {
            return;
        }
        self.transcript.push(format!("> {line}"));
        match self.session.run(&line, &library()) {
            Ok(Outcome::Said(said)) => self.transcript.extend(said.lines().map(str::to_string)),
            Ok(Outcome::Changed) => self.transcript.push("done".to_string()),
            Ok(Outcome::Nothing) => {}
            // A problem is shown exactly as the layer that found it phrased it. A parse
            // failure says where and what was expected; a rejection talks about the game.
            Err(problem) => self.transcript.push(problem.to_string()),
        }
        // Keep the transcript from growing without bound; a console is a window on what
        // just happened, and `history` is the record that does not forget.
        let overflow = self.transcript.len().saturating_sub(400);
        self.transcript.drain(..overflow);
    }

    /// The last few lines, which is all that fits.
    pub fn tail(&self, lines: usize) -> String {
        let from = self.transcript.len().saturating_sub(lines);
        self.transcript[from..].join("\n")
    }
}

#[derive(Component)]
pub struct SurfacePanel(pub Surface);

#[derive(Component)]
struct TabLabel(Surface);

#[derive(Component)]
struct ConsoleText;

#[derive(Component)]
struct BrowserText;

pub struct SurfacesPlugin;

impl Plugin for SurfacesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Surface::Game)
            .insert_resource(Console::default())
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (choose_surface, type_at_console, show_surface).chain(),
            );
    }
}

const INK: Color = Color::srgb(0.85, 0.88, 0.94);
const DIM: Color = Color::srgb(0.48, 0.53, 0.62);
const PANEL: Color = Color::srgb(0.043, 0.047, 0.059);

fn spawn(mut commands: Commands) {
    // The tab bar, on every surface. Reaching any surface from any other is what
    // "all reachable from the front end" asks for.
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(12.0),
            top: Val::Px(10.0),
            column_gap: Val::Px(14.0),
            ..default()
        })
        .with_children(|bar| {
            for surface in Surface::ALL {
                bar.spawn((
                    TabLabel(surface),
                    Text::new(format!("F{} {}", surface as usize + 1, surface.name())),
                    TextFont {
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(DIM),
                ));
            }
        });

    for (surface, marker) in [(Surface::Console, "console"), (Surface::Browser, "browser")] {
        commands
            .spawn((
                SurfacePanel(surface),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(PANEL),
                Visibility::Hidden,
            ))
            .with_children(|panel| {
                let text = panel.spawn((
                    Text::new(String::new()),
                    TextFont {
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(INK),
                ));
                let mut text = text;
                if marker == "console" {
                    text.insert(ConsoleText);
                } else {
                    text.insert(BrowserText);
                }
            });
    }
}

fn choose_surface(keys: Res<ButtonInput<KeyCode>>, mut surface: ResMut<Surface>) {
    for wanted in Surface::ALL {
        if keys.just_pressed(wanted.key()) && *surface != wanted {
            *surface = wanted;
        }
    }
}

/// Collects what is typed while the console is up.
///
/// Bevy has no text field, so this reads key events directly. Only while the console is
/// the surface in front, so the game's own keys are not swallowed by it.
fn type_at_console(
    surface: Res<Surface>,
    mut keys: MessageReader<KeyboardInput>,
    mut console: ResMut<Console>,
) {
    if *surface != Surface::Console {
        keys.clear();
        return;
    }
    for event in keys.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }
        match &event.logical_key {
            Key::Character(typed) => {
                for character in typed.chars() {
                    if !character.is_control() {
                        console.typing.push(character);
                    }
                }
            }
            Key::Space => console.typing.push(' '),
            Key::Backspace => {
                console.typing.pop();
            }
            Key::Enter => {
                let line = std::mem::take(&mut console.typing);
                console.submit(line);
            }
            _ => {}
        }
    }
}

fn show_surface(
    surface: Res<Surface>,
    console: Res<Console>,
    mut panels: Query<(&SurfacePanel, &mut Visibility)>,
    mut tabs: Query<(&TabLabel, &mut TextColor)>,
    mut console_text: Query<&mut Text, (With<ConsoleText>, Without<BrowserText>)>,
    mut browser_text: Query<&mut Text, (With<BrowserText>, Without<ConsoleText>)>,
) {
    if !surface.is_changed() && !console.is_changed() {
        return;
    }
    for (panel, mut visibility) in &mut panels {
        *visibility = if panel.0 == *surface {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    for (tab, mut colour) in &mut tabs {
        *colour = TextColor(if tab.0 == *surface { INK } else { DIM });
    }

    if let Ok(mut text) = console_text.single_mut() {
        *text = Text::new(format!("{}\n\n> {}_", console.tail(34), console.typing));
    }
    if let Ok(mut text) = browser_text.single_mut() {
        *text = Text::new(browser(&console.session));
    }
}

/// Every entity and its components, named the way the console names them.
fn browser(session: &Session) -> String {
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
    // A browser is a window, not a report; what does not fit is not shown.
    lines.truncate(40);
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_console::Library;

    /// Every command file the release needs is carried in the binary, so a browser build
    /// can run exactly what a desktop build runs.
    #[test]
    fn the_release_command_files_travel_with_the_binary() {
        let library = library();
        for name in ["setup", "nodes", "forces", "play"] {
            assert!(library.fetch(name).is_some(), "`{name}` is not embedded");
        }
        assert!(
            library.fetch("setup").unwrap().contains("run nodes"),
            "setup should call its subroutines"
        );
    }

    /// The console opens on a designed, started world - built by running the release's own
    /// setup file, because there is no other way to build one.
    #[test]
    fn the_console_opens_on_a_world_that_was_built_by_commands() {
        let console = Console::default();
        assert_eq!(console.session.game.territories.len(), 12);
        assert_eq!(console.session.game.phase, game_model::Phase::Play);
        // And the history says so: every command that built it, in order.
        assert!(
            console
                .session
                .history()
                .contains(&"create planet tiny".to_string()),
            "{:?}",
            console.session.history()
        );
    }

    #[test]
    fn typing_a_command_records_what_it_said() {
        let mut console = Console::default();
        console.submit("show territory 1".to_string());
        let tail = console.tail(12);
        assert!(tail.contains("> show territory 1"), "{tail}");
        assert!(tail.contains("territory 1"), "{tail}");
    }

    /// A refused command is shown, not swallowed, and in the terms of whichever layer
    /// refused it.
    #[test]
    fn a_refused_command_is_shown_to_the_player() {
        let mut console = Console::default();
        console.submit("land ark somewhere".to_string());
        assert!(
            console.tail(3).contains("expected a number"),
            "{}",
            console.tail(3)
        );

        console.submit("land ark 99".to_string());
        assert!(
            console.tail(3).contains("no territory 99"),
            "{}",
            console.tail(3)
        );
    }

    /// The browser reads; it never writes. Rendering it cannot change the game.
    #[test]
    fn opening_the_browser_changes_nothing() {
        let console = Console::default();
        let before = console.session.game.clone();
        let _ = browser(&console.session);
        assert_eq!(console.session.game, before);
    }

    #[test]
    fn the_browser_names_territories_by_their_model_id() {
        let console = Console::default();
        let shown = browser(&console.session);
        assert!(shown.contains("territory 1"), "{shown}");
        assert!(shown.contains("model id"), "it says so, too");
    }

    /// Every surface is reachable from every other, so none can strand a player.
    #[test]
    fn the_three_surfaces_each_have_their_own_key() {
        let keys: Vec<KeyCode> = Surface::ALL.into_iter().map(Surface::key).collect();
        assert_eq!(keys, [KeyCode::F1, KeyCode::F2, KeyCode::F3]);
        let names: Vec<&str> = Surface::ALL.into_iter().map(Surface::name).collect();
        assert_eq!(names, ["game", "console", "browser"]);
    }
}
