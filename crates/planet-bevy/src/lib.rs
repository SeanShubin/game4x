//! The Bevy adapter: a window, input, and vsync presentation.
//!
//! This is the only crate in the project that knows a graphics engine exists. It owns
//! no logic of its own — it maps Bevy's events onto [`PlanetView`]'s methods and
//! uploads the pixels that come back. Everything interesting happens on the other side
//! of that boundary, in `planet-render`, which can be driven and tested with no engine
//! at all.
//!
//! The reason for the split is not tidiness. It is that the engine is the part most
//! likely to be replaced: this project already replaced minifb with Bevy once, and the
//! model, camera and rasterizer did not change a line.
//!
//! # Why Bevy rather than a framebuffer
//!
//! minifb has no vsync — none, it is not a setting — so it blits whenever asked and
//! the blit lands mid-scanout, which tears. Bevy presents through wgpu with
//! [`PresentMode::AutoVsync`], so frames are swapped during the vertical blank and
//! there is no seam.

pub mod gpu;

use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::{PresentMode, PrimaryWindow, WindowResized};
use planet_ecs::{Owner, PendingIntents, Region, WorldTopology};
use planet_model::{Intent, PlayerId, RegionId};
use planet_render::app::{COARSE_REGION_STEP, KEY_TURN_PIXELS_PER_SECOND};
use planet_render::{Command, PlanetView, WorldSpec};

/// A window configured the way this prototype wants it, vsync included.
///
/// Handed to the composition root rather than applied here, so that assembling the app
/// stays visible in one place.
pub fn window_plugin(width: u32, height: u32) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "planet view".to_string(),
            resolution: (width, height).into(),
            // The whole reason for moving off minifb.
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }
}

/// Drives a [`PlanetView`] from a Bevy window.
pub struct PlanetViewPlugin {
    pub spec: WorldSpec,
}

impl PlanetViewPlugin {
    pub fn new(spec: WorldSpec) -> Self {
        Self { spec }
    }
}

/// Which path draws the sphere.
///
/// The CPU rasterizer stays in the tree as the reference: it is the one with pixel-level
/// tests, and when the two disagree it is the one that is right. `G` switches between
/// them so they can be compared on the same frame.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Renderer {
    #[default]
    Gpu,
    Cpu,
}

impl Renderer {
    pub fn other(self) -> Self {
        match self {
            Renderer::Gpu => Renderer::Cpu,
            Renderer::Cpu => Renderer::Gpu,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Renderer::Gpu => "gpu",
            Renderer::Cpu => "cpu",
        }
    }
}

/// The quad the shader draws on.
#[derive(Resource)]
struct PlanetSurface {
    material: Handle<gpu::PlanetMaterial>,
    entity: Entity,
}

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(gpu::PlanetGpuPlugin)
            .insert_resource(Planet::new(self.spec))
            .init_resource::<LocalPlayer>()
            .init_resource::<Renderer>()
            .add_systems(Startup, spawn_screen)
            .add_systems(
                Update,
                (
                    follow_window_size,
                    read_input,
                    read_ownership,
                    feed_shader,
                    present,
                    quit_on_escape,
                )
                    .chain(),
            );
    }
}

/// The view, plus the buffers used to get it onto the screen.
#[derive(Resource)]
struct Planet {
    view: PlanetView,
    /// One `0x00RRGGBB` per pixel, which is what the rasterizer writes.
    pixels: Vec<u32>,
    /// The same thing as RGBA bytes, which is what a texture wants.
    bytes: Vec<u8>,
    cursor: Option<(f64, f64)>,
    dragging_from: Option<(f64, f64)>,
}

impl Planet {
    fn new(spec: WorldSpec) -> Self {
        // The real size arrives with the first window event; anything works until then.
        let view = PlanetView::new(spec, 1, 1);
        Self {
            view,
            pixels: Vec::new(),
            bytes: Vec::new(),
            cursor: None,
            dragging_from: None,
        }
    }

    fn fit_buffers(&mut self) {
        let pixels = self.view.pixel_count();
        if self.pixels.len() != pixels {
            self.pixels = vec![0; pixels];
            self.bytes = vec![0; pixels * 4];
        }
    }
}

/// Marks the sprite the planet is drawn onto.
#[derive(Component)]
struct Screen;

/// The texture the planet is drawn into.
#[derive(Resource)]
struct ScreenImage(Handle<Image>);

fn spawn_screen(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Keeps the view, the buffers and the texture the same size as the window.
fn follow_window_size(
    mut commands: Commands,
    mut planet: ResMut<Planet>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<gpu::PlanetMaterial>>,
    surface: Option<Res<PlanetSurface>>,
    screen: Option<Res<ScreenImage>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut sprites: Query<&mut Sprite, With<Screen>>,
    mut resized: MessageReader<WindowResized>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let width = window.resolution.width().max(1.0) as usize;
    let height = window.resolution.height().max(1.0) as usize;

    let first_time = screen.is_none();
    let changed = planet.view.resize(width, height) || resized.read().count() > 0 || first_time;
    if !changed {
        return;
    }
    planet.fit_buffers();

    let image = blank_image(width as u32, height as u32);
    let handle = images.add(image);

    if let Ok(mut sprite) = sprites.single_mut() {
        sprite.image = handle.clone();
        sprite.custom_size = Some(Vec2::new(width as f32, height as f32));
    } else {
        commands.spawn((
            Sprite {
                image: handle.clone(),
                custom_size: Some(Vec2::new(width as f32, height as f32)),
                ..default()
            },
            // Above the shader quad, so labels and the readout sit on top of it.
            Transform::from_xyz(0.0, 0.0, 1.0),
            Screen,
        ));
    }
    commands.insert_resource(ScreenImage(handle));

    // The quad the shader draws on, at the same size and underneath.
    let mesh = gpu::screen_quad(&mut meshes, width as f32, height as f32);
    let material = materials.add(gpu::PlanetMaterial::default());
    if let Some(existing) = surface {
        commands.entity(existing.entity).despawn();
    }
    let entity = commands
        .spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    commands.insert_resource(PlanetSurface { material, entity });
}

fn blank_image(width: u32, height: u32) -> Image {
    // Four bytes per pixel, fixed by the format below.
    const BYTES_PER_PIXEL: usize = 4;
    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![0; (width * height) as usize * BYTES_PER_PIXEL],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    // It is a pixel buffer at 1:1, so filtering would only blur it.
    image.sampler = ImageSampler::nearest();
    image
}

fn read_input(
    mut planet: ResMut<Planet>,
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<MouseWheel>,
    mut pending: ResMut<PendingIntents>,
    mut local_player: ResMut<LocalPlayer>,
    mut renderer: ResMut<Renderer>,
    time: Res<Time>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    planet.cursor = window
        .cursor_position()
        .map(|position| (position.x as f64, position.y as f64));
    let hovered = planet.view.hovered(planet.cursor);

    // Drag by the change in cursor position rather than by raw mouse motion, so the
    // surface tracks the pointer exactly even when the system applies acceleration.
    if buttons.pressed(MouseButton::Left) {
        if let Some((x, y)) = planet.cursor {
            if let Some((last_x, last_y)) = planet.dragging_from {
                planet.view.drag(x - last_x, y - last_y);
            }
            planet.dragging_from = Some((x, y));
        }
    } else {
        planet.dragging_from = None;
    }

    let notches: f64 = wheel
        .read()
        .map(|event| match event.unit {
            MouseScrollUnit::Line => event.y as f64,
            // Trackpads report pixels; roughly one notch per line of text.
            MouseScrollUnit::Pixel => event.y as f64 / 50.0,
        })
        .sum();
    if notches != 0.0 {
        let (x, y) = planet.cursor.unwrap_or_else(|| {
            let (width, height) = planet.view.size();
            (width as f64 / 2.0, height as f64 / 2.0)
        });
        planet.view.zoom(x, y, notches);
    }

    // C claims whatever is under the cursor. This is the only place in the project
    // that turns a keypress into an intent, and it does nothing else: the rules decide
    // whether the claim is legal, one layer down and a whole crate away.
    if keys.just_pressed(KeyCode::KeyC) {
        if let Some(region) = hovered {
            pending.push(Intent::Claim {
                region: RegionId(region as u32),
                player: PlayerId(local_player.0),
            });
        }
    }
    if keys.just_pressed(KeyCode::KeyX) {
        if let Some(region) = hovered {
            pending.push(Intent::Abandon {
                region: RegionId(region as u32),
            });
        }
    }
    if keys.just_pressed(KeyCode::Tab) {
        local_player.0 = local_player.0.wrapping_add(1) % 6;
    }
    if keys.just_pressed(KeyCode::KeyG) {
        *renderer = renderer.other();
    }

    // Arrow keys turn the sphere, the same way dragging does. Scaled by elapsed time so
    // the speed does not depend on the frame rate, and held rather than tapped.
    let mut turn = (0.0, 0.0);
    for (key, dx, dy) in [
        (KeyCode::ArrowLeft, -1.0, 0.0),
        (KeyCode::ArrowRight, 1.0, 0.0),
        (KeyCode::ArrowUp, 0.0, -1.0),
        (KeyCode::ArrowDown, 0.0, 1.0),
    ] {
        if keys.pressed(key) {
            turn.0 += dx;
            turn.1 += dy;
        }
    }
    if turn != (0.0, 0.0) {
        // Shift turns faster, matching what it does to the region count.
        let hurry = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            3.0
        } else {
            1.0
        };
        let pixels = KEY_TURN_PIXELS_PER_SECOND * hurry * time.delta_secs() as f64;
        // Dragging moves the surface with the pointer, so an arrow press moves the
        // surface the other way: pressing right should look right, not drag right.
        planet.view.drag(-turn.0 * pixels, -turn.1 * pixels);
    }

    let step = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        COARSE_REGION_STEP
    } else {
        1
    };

    for (key, command) in [
        (KeyCode::KeyL, Command::ToggleLabels),
        (KeyCode::KeyB, Command::ToggleBorders),
        (KeyCode::KeyD, Command::ToggleDimming),
        (KeyCode::KeyP, Command::ToggleProjection),
        (KeyCode::KeyS, Command::ToggleSource),
        (KeyCode::KeyR, Command::NextSeed),
        (KeyCode::Digit0, Command::ResetView),
        (KeyCode::Numpad0, Command::ResetView),
        (KeyCode::Equal, Command::MoreRegions(step)),
        (KeyCode::NumpadAdd, Command::MoreRegions(step)),
        (KeyCode::Minus, Command::FewerRegions(step)),
        (KeyCode::NumpadSubtract, Command::FewerRegions(step)),
    ] {
        if keys.just_pressed(key) {
            planet.view.apply(command);
        }
    }
}

/// Which player the keyboard is acting as. A stand-in until there is a real notion of
/// whose turn it is.
#[derive(Resource, Default)]
pub struct LocalPlayer(pub u16);

/// Copies ownership out of the ECS and into the view model, once a frame.
///
/// Strictly one-way. The view model is a projection of the model; nothing it holds ever
/// travels back. Gathering is keyed by `RegionId`, so query order cannot show through.
fn read_ownership(
    mut planet: ResMut<Planet>,
    topology: Option<Res<WorldTopology>>,
    regions: Query<(&Region, Option<&Owner>)>,
) {
    let Some(topology) = topology else {
        return;
    };
    let mut owners = vec![None; topology.0.region_count()];
    for (region, owner) in regions.iter() {
        if let Some(slot) = owners.get_mut(region.0.index()) {
            *slot = owner.map(|owner| owner.0.0);
        }
    }
    if planet.view.owners() != owners.as_slice() {
        planet.view.set_owners(owners);
    }
}

/// Hands the shader everything it needs, once a frame.
fn feed_shader(
    planet: Res<Planet>,
    renderer: Res<Renderer>,
    surface: Option<Res<PlanetSurface>>,
    mut materials: ResMut<Assets<gpu::PlanetMaterial>>,
    mut visibility: Query<&mut Visibility, With<Mesh2d>>,
) {
    let Some(surface) = surface else {
        return;
    };
    for mut visible in visibility.iter_mut() {
        *visible = if *renderer == Renderer::Gpu {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    if *renderer != Renderer::Gpu {
        return;
    }
    if let Some(mut material) = materials.get_mut(&surface.material) {
        let hovered = planet.view.hovered(planet.cursor);
        material.planet = gpu::fill_uniform(&planet.view, hovered);
    }
}

fn present(
    mut planet: ResMut<Planet>,
    renderer: Res<Renderer>,
    screen: Option<Res<ScreenImage>>,
    mut images: ResMut<Assets<Image>>,
) {
    let Some(screen) = screen else {
        return;
    };
    let Some(mut image) = images.get_mut(&screen.0) else {
        return;
    };

    planet.fit_buffers();
    let cursor = planet.cursor;
    let overlay_only = *renderer == Renderer::Gpu;
    let Planet {
        view,
        pixels,
        bytes,
        ..
    } = &mut *planet;

    if overlay_only {
        // The shader drew the sphere; this layer only carries what sits on top.
        view.draw_overlay(pixels, cursor);
    } else {
        view.draw(pixels, cursor);
    }
    for (target, &pixel) in bytes.chunks_exact_mut(4).zip(pixels.iter()) {
        target[0] = (pixel >> 16) as u8;
        target[1] = (pixel >> 8) as u8;
        target[2] = pixel as u8;
        target[3] = if pixel == planet_render::raster::TRANSPARENT {
            0x00
        } else {
            0xFF
        };
    }

    if let Some(data) = image.data.as_mut() {
        if data.len() == bytes.len() {
            data.copy_from_slice(bytes);
        }
    }
}

fn quit_on_escape(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
