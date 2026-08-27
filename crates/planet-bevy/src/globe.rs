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
use sphere_tessellation::Direction;

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

/// The spike marking each end of the planet's axis.
///
/// Small enough not to cover the territories around it, long enough to be unmistakable
/// at a glance and still recognisable edge-on at the silhouette. The base is set below
/// the surface so no gap can open between the spike and the panels beneath it.
const POLE_MARKER_RADIUS: f32 = 0.045;
const POLE_MARKER_HEIGHT: f32 = 0.19;
const POLE_MARKER_BASE: f32 = 0.94;

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

            // The poles, marked at both ends of the axis. These are children like the
            // panels are, and positioned in the same model space, so whatever turns the
            // world turns them with it and they stay pinned where they belong.
            let spike = meshes.add(Cone {
                radius: POLE_MARKER_RADIUS,
                height: POLE_MARKER_HEIGHT,
            });
            let marker = materials.add(StandardMaterial {
                base_color: Color::srgb(0.94, 0.95, 0.98),
                perceptual_roughness: 0.35,
                ..default()
            });
            for pole in Direction::poles() {
                let outward = to_view(pole);
                globe.spawn((
                    Mesh3d(spike.clone()),
                    MeshMaterial3d(marker.clone()),
                    Transform {
                        translation: outward * (POLE_MARKER_BASE + POLE_MARKER_HEIGHT / 2.0),
                        // A cone is built standing on +y, so point that at the pole.
                        rotation: Quat::from_rotation_arc(Vec3::Y, outward),
                        ..default()
                    },
                ));
            }
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

/// A model direction in the engine's coordinates.
fn to_view(direction: Direction) -> Vec3 {
    let vector = direction.vector();
    Vec3::new(vector.x as f32, vector.y as f32, vector.z as f32)
}

/// Stands the planet up, so its axis runs down the screen rather than out of it.
///
/// The model's axis is `+z` - see [`Direction::NORTH_POLE`] - while the engine's up is
/// `+y`, so without this the north pole would point at the viewer and the world would not
/// read as a globe at all. Derived from the constant rather than written out as a quarter
/// turn, so that it cannot disagree with the model about which way is north.
fn upright() -> Quat {
    Quat::from_rotation_arc(to_view(Direction::NORTH_POLE), Vec3::Y)
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
            // Both signs are positive because the surface follows the pointer: drag right
            // and the face you are looking at goes right with it. That is the convention
            // the flat view already uses - `GlobeView::drag` turns by `+dx` about the up
            // axis and `+dy` about the across axis - and the two views have to agree, or
            // the same gesture means opposite things depending on which one is up.
            let step = event.position - previous;
            orbit.yaw += step.x * DRAG_SENSITIVITY;
            orbit.pitch =
                (orbit.pitch + step.y * DRAG_SENSITIVITY).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        }
        drag.0 = Some(event.position);
    }
}

fn keys_to_turn(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut orbit: ResMut<Orbit>) {
    // An arrow key is a drag in that direction, which is how the flat view defines them
    // too: right is `+dx`, down is `+dy`. So the signs here have to match `drag_to_turn`
    // exactly, or holding an arrow would turn the world the other way from dragging it.
    let held = |key| keys.pressed(key) as i32 as f32;
    let turn = held(KeyCode::ArrowRight) - held(KeyCode::ArrowLeft);
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
        // Spin first, then tilt: `XYZ` composes as `pitch * yaw`, so yaw is applied to the
        // upright planet and therefore turns it about its own axis. Tilting first - which
        // is what the previous `YXZ` did - would have swung the axis away from vertical
        // and left the poles wandering in circles as you turned, instead of staying put at
        // the top and bottom where they belong.
        transform.rotation =
            Quat::from_euler(EulerRot::XYZ, orbit.pitch, orbit.yaw, 0.0) * upright();
    }
    for mut transform in &mut cameras {
        *transform = Transform::from_xyz(0.0, 0.0, orbit.distance).looking_at(Vec3::ZERO, Vec3::Y);
    }
}
