//! What the application was asked to do, from the command line.
//!
//! Option parsing in the composition root is the arrangement
//! [`docs/architecture.md`](../../../docs/architecture.md) already describes for the planet
//! view prototype - *option parsing, a headless capture path, and a `main`*. It holds no
//! game logic; it decides what to build.
//!
//! # Why there is a remote control at all
//!
//! The globe is drawn by a graphics engine into a window, and neither a test nor an agent
//! working on this code can see a window. Everything below the engine is testable without
//! one - that is the whole point of the layering - but *the picture* is exactly the part
//! that is not, and the picture is what half of `spec/planet.md` is about.
//!
//! So the application can be driven from outside: put the camera somewhere, choose a
//! drawing, run some commands, then write a PNG and a text dump and exit. What comes back
//! is evidence rather than an opinion.

/// How the application was asked to start.
#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    /// Write a screenshot here and exit.
    pub shot: Option<String>,
    /// Write a text dump here. Combines with `shot`, or stands alone.
    pub dump: Option<String>,
    /// Which drawing to open on.
    pub realistic: bool,
    /// Where to put the camera, if it is not to be left at rest.
    pub yaw: Option<f32>,
    pub pitch: Option<f32>,
    pub distance: Option<f32>,
    /// Lines to type at the console before anything is drawn, in order.
    pub run: Vec<String>,
    pub width: u32,
    pub height: u32,
    /// How many frames to let pass before capturing.
    ///
    /// The first frames of a Bevy application have no world in them yet: meshes are built
    /// in a system that runs after startup, and the images they need are uploaded a frame
    /// or two later still. Capturing too early photographs an empty sky.
    pub settle: u32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            shot: None,
            dump: None,
            realistic: false,
            yaw: None,
            pitch: None,
            distance: None,
            run: Vec::new(),
            width: 1280,
            height: 800,
            settle: 24,
        }
    }
}

impl Options {
    /// Whether the application should quit once it has produced what was asked for.
    pub fn is_errand(&self) -> bool {
        self.shot.is_some() || self.dump.is_some()
    }
}

/// What went wrong reading the arguments, in the words of somebody who typed them.
#[derive(Clone, Debug, PartialEq)]
pub enum Misuse {
    Unknown(String),
    Missing(&'static str),
    NotANumber { option: &'static str, word: String },
}

impl std::fmt::Display for Misuse {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Misuse::Unknown(word) => write!(out, "there is no option {word}; try --help"),
            Misuse::Missing(option) => write!(out, "{option} needs a value"),
            Misuse::NotANumber { option, word } => {
                write!(out, "{option} wants a number, not {word}")
            }
        }
    }
}

pub const USAGE: &str = "\
game4x - the planet, its console, and its data browser

    --shot PATH          draw one frame to a PNG and exit
    --dump PATH          write what is on screen, as text, and exit
    --drawing WHICH      practical (default) or realistic
    --yaw RADIANS        turn the planet before drawing
    --pitch RADIANS      tilt it
    --distance RADII     how far back the camera sits
    --run LINE           type a line at the console first; repeatable
    --size WHICH         tiny, small, medium, large or huge
    --width N            window width, default 1280
    --height N           window height, default 800
    --settle FRAMES      frames to let pass before capturing, default 24
    --help               this

With no options it opens a window and plays.";

/// Reads the arguments, or says what was wrong with them.
///
/// `--size` and `--run` are the same mechanism: `--size huge` is `--run \"/new huge\"`, kept
/// separate only because it is the thing most often wanted. Everything that changes the
/// game goes through the console, here as everywhere else.
pub fn read(arguments: impl IntoIterator<Item = String>) -> Result<Option<Options>, Misuse> {
    let mut options = Options::default();
    let mut words = arguments.into_iter();
    while let Some(word) = words.next() {
        let mut value = |option: &'static str| words.next().ok_or(Misuse::Missing(option));
        let number = |option: &'static str, word: String| {
            word.parse::<f32>()
                .map_err(|_| Misuse::NotANumber { option, word })
        };
        match word.as_str() {
            "--help" | "-h" => return Ok(None),
            "--shot" => options.shot = Some(value("--shot")?),
            "--dump" => options.dump = Some(value("--dump")?),
            "--drawing" => options.realistic = value("--drawing")? == "realistic",
            "--yaw" => options.yaw = Some(number("--yaw", value("--yaw")?)?),
            "--pitch" => options.pitch = Some(number("--pitch", value("--pitch")?)?),
            "--distance" => options.distance = Some(number("--distance", value("--distance")?)?),
            "--run" => options.run.push(value("--run")?),
            "--size" => options.run.push(format!("/new {}", value("--size")?)),
            "--width" => options.width = number("--width", value("--width")?)? as u32,
            "--height" => options.height = number("--height", value("--height")?)? as u32,
            "--settle" => options.settle = number("--settle", value("--settle")?)? as u32,
            other => return Err(Misuse::Unknown(other.to_string())),
        }
    }
    Ok(Some(options))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_words(line: &str) -> Result<Option<Options>, Misuse> {
        read(line.split_whitespace().map(str::to_string))
    }

    #[test]
    fn no_arguments_opens_a_window_and_plays() {
        let options = read_words("").unwrap().unwrap();
        assert_eq!(options, Options::default());
        assert!(!options.is_errand(), "it should not quit on its own");
    }

    #[test]
    fn an_errand_is_anything_that_produces_a_file() {
        assert!(read_words("--shot a.png").unwrap().unwrap().is_errand());
        assert!(read_words("--dump a.txt").unwrap().unwrap().is_errand());
        assert!(!read_words("--yaw 1").unwrap().unwrap().is_errand());
    }

    #[test]
    fn the_camera_can_be_placed_before_anything_is_drawn() {
        let options = read_words("--yaw 0.5 --pitch -0.25 --distance 4")
            .unwrap()
            .unwrap();
        assert_eq!(options.yaw, Some(0.5));
        assert_eq!(options.pitch, Some(-0.25));
        assert_eq!(options.distance, Some(4.0));
    }

    /// `--size` is `--run` wearing a shorter name, so a planet is still chosen the only
    /// way a planet can be chosen.
    #[test]
    fn choosing_a_size_is_typing_at_the_console() {
        let options = read_words("--size huge").unwrap().unwrap();
        assert_eq!(options.run, ["/new huge"]);
    }

    #[test]
    fn lines_to_run_keep_their_order() {
        let options = read(
            ["--run", "land ark 1", "--run", "end turn"]
                .into_iter()
                .map(str::to_string),
        )
        .unwrap()
        .unwrap();
        assert_eq!(options.run, ["land ark 1", "end turn"]);
    }

    #[test]
    fn either_drawing_can_be_asked_for() {
        assert!(
            read_words("--drawing realistic")
                .unwrap()
                .unwrap()
                .realistic
        );
        assert!(
            !read_words("--drawing practical")
                .unwrap()
                .unwrap()
                .realistic
        );
        assert!(!read_words("").unwrap().unwrap().realistic);
    }

    #[test]
    fn help_asks_for_nothing_to_be_built() {
        assert_eq!(read_words("--help").unwrap(), None);
    }

    #[test]
    fn a_misuse_says_what_was_wrong() {
        assert_eq!(
            read_words("--nonsense").unwrap_err().to_string(),
            "there is no option --nonsense; try --help"
        );
        assert_eq!(
            read_words("--shot").unwrap_err().to_string(),
            "--shot needs a value"
        );
        assert_eq!(
            read_words("--yaw sideways").unwrap_err().to_string(),
            "--yaw wants a number, not sideways"
        );
    }
}
