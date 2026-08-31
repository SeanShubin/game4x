//! Command line parsing. Nothing here knows how the view works or how it is drawn.

use planet_flat::Renderer;
use planet_render::{Params, WorldSpec};
use std::error::Error;

pub const DEFAULT_WIDTH: usize = 1000;
pub const DEFAULT_HEIGHT: usize = 820;

pub struct Options {
    pub width: usize,
    pub height: usize,
    pub params: Params,
    /// Render one frame to this path and exit, instead of opening a window.
    pub capture: Option<String>,
    /// Where to write a photograph of what the *engine* drew, if asked for.
    pub shot: Option<String>,
    /// Frames to let pass before the shutter.
    pub settle: u32,
    /// Which path draws the sphere, when the window opens.
    pub renderer: Renderer,
    pub turn_right: f64,
    pub turn_up: f64,
    pub zoom: f64,
    pub globe: bool,
    /// `None` until the user says which world they want.
    soccer: Option<bool>,
    /// Set by any option only a generated world can honour.
    generation_requested: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            params: Params::default(),
            capture: None,
            shot: None,
            // Enough frames for the world to be built and the first draw to land.
            settle: 30,
            renderer: Renderer::default(),
            turn_right: 0.0,
            turn_up: 0.0,
            zoom: 1.0,
            globe: false,
            soccer: None,
            generation_requested: false,
        }
    }
}

impl Options {
    /// The exact solid is the default, but any option only a generated world can
    /// honour switches to one. Otherwise `--regions 60` would be accepted and then
    /// quietly ignored, which is worse than rejecting it.
    pub fn wants_soccer(&self) -> bool {
        self.soccer.unwrap_or(!self.generation_requested)
    }

    pub fn spec(&self) -> WorldSpec {
        WorldSpec {
            params: self.params,
            soccer: self.wants_soccer(),
        }
    }
}

pub fn parse() -> Result<Options, Box<dyn Error>> {
    let mut options = Options::default();
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let mut index = 0;
    while index < arguments.len() {
        let flag = arguments[index].as_str();
        let mut value = || -> Result<String, Box<dyn Error>> {
            index += 1;
            arguments
                .get(index)
                .cloned()
                .ok_or_else(|| format!("{flag} needs a value").into())
        };
        match flag {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--width" => options.width = value()?.parse()?,
            "--height" => options.height = value()?.parse()?,
            "--regions" => {
                options.params.region_count = value()?.parse()?;
                options.generation_requested = true;
            }
            "--seed" => {
                options.params.seed = value()?.parse()?;
                options.generation_requested = true;
            }
            "--jitter" => {
                options.params.jitter = value()?.parse()?;
                options.generation_requested = true;
            }
            "--relax" => {
                options.params.relaxation = value()?.parse()?;
                options.generation_requested = true;
            }
            "--capture" => options.capture = Some(value()?),
            "--shot" => options.shot = Some(value()?),
            "--settle" => options.settle = value()?.parse()?,
            "--renderer" => {
                let word = value()?;
                options.renderer = match word.as_str() {
                    "gpu" => Renderer::Gpu,
                    "cpu" => Renderer::Cpu,
                    other => return Err(format!("unknown renderer {other}").into()),
                };
            }
            "--turn-right" => options.turn_right = value()?.parse::<f64>()?.to_radians(),
            "--turn-up" => options.turn_up = value()?.parse::<f64>()?.to_radians(),
            "--zoom" => options.zoom = value()?.parse()?,
            "--globe" => options.globe = true,
            "--soccer" => options.soccer = Some(true),
            "--generated" => options.soccer = Some(false),
            other => return Err(format!("unknown option {other}").into()),
        }
        index += 1;
    }

    if options.params.region_count == 0 {
        return Err("--regions must be at least 1".into());
    }
    if options.soccer == Some(true) && options.generation_requested {
        return Err(
            "--soccer builds a fixed 32-region solid, so it cannot be combined with \
             --regions, --seed, --jitter or --relax"
                .into(),
        );
    }
    Ok(options)
}

fn print_usage() {
    println!(
        "planet-view - a sphere fanned out flat\n\
         \n\
         The whole world sits inside one disc, undistorted at the centre and stretched\n\
         at the rim. Past the rim it repeats, dimmed. Dragging turns the sphere, so\n\
         there is no edge and no pole to get stuck on.\n\
         \n\
         Options:\n\
         \x20 --regions N        region count; implies --generated\n\
         \x20 --seed N           world seed; implies --generated\n\
         \x20 --jitter F         seed displacement; implies --generated\n\
         \x20 --relax N          Lloyd relaxation passes; implies --generated\n\
         \x20 --soccer           the exact truncated icosahedron, 32 regions (default)\n\
         \x20 --generated        a generated world instead of the exact solid\n\
         \x20 --width N          window width\n\
         \x20 --height N         window height\n\
         \x20 --capture PATH     render one frame to a PNG and exit, without a window\n\
         \x20 --turn-right DEG   turn the sphere before capturing\n\
         \x20 --turn-up DEG      turn the sphere before capturing\n\
         \x20 --zoom F           zoom multiplier, for --capture\n\
         \x20 --globe            hold the ball together instead of fanning it out\n\
         \n\
         Controls:\n\
         \x20 drag               turn the sphere, in any direction, forever\n\
         \x20 wheel              zoom about the cursor\n\
         \x20 P                  fan out flat, or fold back into a globe\n\
         \x20 S                  the exact solid, or a generated world\n\
         \x20 L B D              toggle labels, borders, duplicate dimming\n\
         \x20 R                  new seed\n\
         \x20 - and +            fewer or more regions; hold shift for tens\n\
         \x20 0                  reset the view\n\
         \x20 Esc                quit"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Asking for a region count used to be accepted and then silently ignored,
    /// because the exact solid is the default and its size is fixed.
    #[test]
    fn asking_for_a_region_count_switches_away_from_the_fixed_solid() {
        assert!(
            Options::default().wants_soccer(),
            "the solid is the default"
        );

        let asked_for_regions = Options {
            generation_requested: true,
            ..Default::default()
        };
        assert!(
            !asked_for_regions.wants_soccer(),
            "--regions must not be quietly ignored"
        );

        assert!(
            Options {
                soccer: Some(true),
                ..Default::default()
            }
            .wants_soccer()
        );
        assert!(
            !Options {
                soccer: Some(false),
                ..Default::default()
            }
            .wants_soccer()
        );
    }

    #[test]
    fn the_spec_follows_the_resolved_choice() {
        let options = Options::default();
        assert!(options.spec().soccer);

        let generated = Options {
            generation_requested: true,
            ..Default::default()
        };
        assert!(!generated.spec().soccer);
    }
}
