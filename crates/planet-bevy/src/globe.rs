//! The world as a solid you can turn in your hands.
//!
//! This is the engine side of [`planet_render::mesh`], and it is deliberately thin: the
//! mesh arrives as plain numbers and this module's whole job is to hand them to Bevy,
//! point a camera at the result, and turn the ball when the pointer moves. No geometry
//! is computed here and no rule is decided here.
//!
//! The flat projection in [`crate::gpu`] answers "what colour is this pixel" in a
//! fragment shader. This does the opposite: it uploads the polygons once and lets the
//! hardware rasterize them. Both draw the same model, and the model cannot tell which
//! one is running.

use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::MouseWheel;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::window::CursorMoved;

use planet_render::{World, WorldSpec, mesh};

/// How far back the camera sits at rest, in sphere radii.
const RESTING_DISTANCE: f32 = 3.1;
const CLOSEST: f32 = 1.35;
const FURTHEST: f32 = 9.0;

/// Radians of turn per pixel of drag. Slow enough to place a region deliberately.
const DRAG_SENSITIVITY: f32 = 0.006;
/// Radians per second while an arrow key is held.
const KEY_SPEED: f32 = 1.1;
/// Fraction of the distance a wheel notch closes. Gentle on purpose - the first
/// prototype's zoom was reported as far too sensitive.
const ZOOM_PER_NOTCH: f32 = 0.09;

/// How far the pitch may travel before it would tip past the pole and invert the world.
const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;

/// The radius of the dark ball the panels float on, filling the seams between them.
const UNDERSIDE: f32 = 0.965;

pub struct GlobePlugin {
    pub spec: WorldSpec,
}

impl GlobePlugin {
    pub fn new(spec: WorldSpec) -> Self {
        Self { spec }
    }
}

impl Plugin for GlobePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Spec(self.spec))
            .insert_resource(Orbit::default())
            .insert_resource(Drag::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (drag_to_turn, keys_to_turn, wheel_to_zoom, apply_orbit).chain(),
            );
    }
}

#[derive(Resource, Clone, Copy)]
struct Spec(WorldSpec);

/// Where the viewer is, in the only three numbers that matter. The ball turns rather
/// than the camera, so that the light stays put and the terminator does not swing about
/// while you are trying to look at something.
#[derive(Resource, Clone, Copy)]
struct Orbit {
    yaw: f32,
    pitch: f32,
    distance: f32,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.35,
            distance: RESTING_DISTANCE,
        }
    }
}

/// Where the pointer was on the previous frame, while a drag is in progress.
///
/// Deltas are measured from the cursor's reported position rather than taken from
/// `MouseMotion`. `MouseMotion` is a *device level* signal, and on the web it depends on
/// `movementX`, which is not dependable outside pointer lock - so the flat projection's
/// habit of reading it would have made drag-to-turn a native-only feature. The cursor's
/// position is reported the same way everywhere.
#[derive(Resource, Default)]
struct Drag(Option<Vec2>);

#[derive(Component)]
struct Globe;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spec: Res<Spec>,
) {
    let world = World::build(spec.0);
    let solid =
        sphere_tessellation::solid(&world.tessellation.seeds, &world.tessellation.neighbours);
    let panels = mesh::build(&solid, &world.coloring);

    // The panels. Vertex colours carry the region colouring, so one material serves the
    // whole world however many regions it has.
    let panel_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.75,
        reflectance: 0.15,
        ..default()
    });

    // The ball underneath. The panels are inset, so without this the seams would be
    // holes straight through to the background.
    let underside = materials.add(StandardMaterial {
        base_color: Color::srgb(0.02, 0.025, 0.035),
        perceptual_roughness: 1.0,
        ..default()
    });

    commands
        .spawn((Globe, Transform::default(), Visibility::default()))
        .with_children(|globe| {
            globe.spawn((
                Mesh3d(meshes.add(to_bevy_mesh(&panels))),
                MeshMaterial3d(panel_material),
            ));
            globe.spawn((
                Mesh3d(meshes.add(Sphere::new(UNDERSIDE).mesh().ico(4).unwrap())),
                MeshMaterial3d(underside),
            ));
        });

    // Ambient light rides on the camera in Bevy 0.19 rather than being a global
    // resource. Enough of it that the side facing away from the key light is still
    // readable - this is a map before it is a photograph.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, RESTING_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
        AmbientLight {
            color: Color::srgb(0.65, 0.72, 0.9),
            brightness: 900.0,
            ..default()
        },
    ));

    // The key light. Shadow maps are off: the only thing that could cast a shadow here
    // is the ball onto itself, and the terminator already reads as roundness.
    commands.spawn((
        DirectionalLight {
            illuminance: 9_000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 6.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Text::new(summary(&world, &panels)),
        TextFont {
            font_size: FontSize::Px(13.0),
            ..default()
        },
        TextColor(Color::srgb(0.85, 0.88, 0.94)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            top: Val::Px(10.0),
            ..default()
        },
    ));
}

fn summary(world: &World, panels: &mesh::PlanetMesh) -> String {
    let regions = world.tessellation.region_count();
    let shape = match sphere_tessellation::goldberg::arrangements_up_to(regions)
        .into_iter()
        .find(|&(m, n)| sphere_tessellation::goldberg::region_count(m, n) == regions)
    {
        Some((m, n)) => format!("GP({m},{n})"),
        None => "no Goldberg solid at this count".to_string(),
    };
    format!(
        "{regions} regions - {shape}\n{} - {} triangles\ndrag to turn, wheel to zoom, arrows to turn",
        world.degree_summary(),
        panels.triangle_count()
    )
}

/// Copies the view model into an engine mesh. The only place these numbers acquire a
/// type that belongs to Bevy.
fn to_bevy_mesh(planet: &mesh::PlanetMesh) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, planet.positions.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, planet.normals.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, planet.colors.clone())
    .with_inserted_indices(Indices::U32(planet.indices.clone()))
}

fn drag_to_turn(
    buttons: Res<ButtonInput<MouseButton>>,
    mut moved: MessageReader<CursorMoved>,
    mut drag: ResMut<Drag>,
    mut orbit: ResMut<Orbit>,
) {
    if !buttons.pressed(MouseButton::Left) {
        moved.clear();
        drag.0 = None;
        return;
    }
    for event in moved.read() {
        // The first position after the button goes down only establishes where the drag
        // started. Turning on it would jerk the world by however far the pointer happened
        // to be from wherever it was last seen.
        if let Some(previous) = drag.0 {
            let step = event.position - previous;
            orbit.yaw -= step.x * DRAG_SENSITIVITY;
            orbit.pitch =
                (orbit.pitch - step.y * DRAG_SENSITIVITY).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        }
        drag.0 = Some(event.position);
    }
}

fn keys_to_turn(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut orbit: ResMut<Orbit>) {
    let held = |key| keys.pressed(key) as i32 as f32;
    let turn = held(KeyCode::ArrowLeft) - held(KeyCode::ArrowRight);
    let tilt = held(KeyCode::ArrowDown) - held(KeyCode::ArrowUp);
    // Touching a `ResMut` at all marks it changed, so leave it alone when no key is down.
    // Otherwise the orbit reads as changed on every frame and the guard in `apply_orbit`
    // never saves anything.
    if turn == 0.0 && tilt == 0.0 {
        return;
    }
    let step = KEY_SPEED * time.delta_secs();
    orbit.yaw += turn * step;
    orbit.pitch = (orbit.pitch + tilt * step).clamp(-PITCH_LIMIT, PITCH_LIMIT);
}

fn wheel_to_zoom(mut wheel: MessageReader<MouseWheel>, mut orbit: ResMut<Orbit>) {
    for notch in wheel.read() {
        // Scale rather than subtract, so a notch covers the same proportion of the
        // remaining distance whether you are close in or far out.
        let factor = (1.0 - ZOOM_PER_NOTCH).powf(notch.y.clamp(-3.0, 3.0));
        orbit.distance = (orbit.distance * factor).clamp(CLOSEST, FURTHEST);
    }
}

fn apply_orbit(
    orbit: Res<Orbit>,
    mut globes: Query<&mut Transform, (With<Globe>, Without<Camera3d>)>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
) {
    if !orbit.is_changed() {
        return;
    }
    for mut transform in &mut globes {
        transform.rotation = Quat::from_euler(EulerRot::YXZ, orbit.yaw, orbit.pitch, 0.0);
    }
    for mut transform in &mut cameras {
        *transform = Transform::from_xyz(0.0, 0.0, orbit.distance).looking_at(Vec3::ZERO, Vec3::Y);
    }
}
