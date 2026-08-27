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
//!
//! What `spec/planet.md` asks of a presentation, and where each of them lives:
//!
//! | Requirement | Here |
//! | --- | --- |
//! | Presented as a three-dimensional sphere | [`build_globe`] |
//! | Rotate to be above any point | [`drag_to_turn`], [`keys_to_turn`] |
//! | Rotation bound to the arrow keys | [`keys_to_turn`] |
//! | Dragging rotates the planet | [`drag_to_turn`] |
//! | The roll for any point is fixed | [`globe_transform`] |
//! | Zoom in and out | [`wheel_to_zoom`] |
//! | Reset the view to a default | [`reset_view`] |
//! | A territory's id displayed on the sphere | [`place_labels`] |
//! | The poles are visible | [`build_globe`] |

use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::MouseWheel;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::window::CursorMoved;

use planet_model::PlanetSize;
use planet_render::{Params, World, WorldSpec, mesh};
use sphere_tessellation::Direction;

/// How far back the camera sits at rest, in sphere radii.
///
/// Far enough that there is sky above the north pole. At 3.1 the planet very nearly
/// filled the window, and the pole's letter - which floats above the spike, further out
/// than any part of the planet - projected past the top edge and was simply not on the
/// screen. Anything drawn beyond the surface needs room beyond the surface to be drawn in.
const RESTING_DISTANCE: f32 = 3.45;
const CLOSEST: f32 = 1.35;
const FURTHEST: f32 = 9.0;

/// The tilt the view opens on, and returns to when reset. Enough to show that the world
/// is a ball rather than a disc, and to bring the north pole into view without hiding it.
const RESTING_PITCH: f32 = 0.35;

/// Radians of turn per pixel of drag. Slow enough to place a region deliberately.
const DRAG_SENSITIVITY: f32 = 0.006;
/// Radians per second while an arrow key is held.
const KEY_SPEED: f32 = 1.1;
/// Fraction of the distance a wheel notch closes. Gentle on purpose - the first
/// prototype's zoom was reported as far too sensitive.
const ZOOM_PER_NOTCH: f32 = 0.09;

/// How far the pitch may travel before it would tip past the pole and invert the world.
///
/// The margin is a hundredth of a radian, a little over half a degree, and it exists
/// because the axis degenerates exactly at the pole: yaw and roll stop being separable
/// there. You can still get near enough to be looking straight down at a pole.
const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;

/// The radius of the dark ball the panels float on, filling the seams between them.
const UNDERSIDE: f32 = 0.965;

/// The spike marking each end of the planet's axis.
const POLE_MARKER_RADIUS: f32 = 0.045;
const POLE_MARKER_HEIGHT: f32 = 0.19;
const POLE_MARKER_BASE: f32 = 0.94;

/// Where a pole's letter floats: just clear of the spike's tip, rather than on top of it
/// or so far past it that it leaves the window.
const POLE_LABEL_HEIGHT: f32 = POLE_MARKER_BASE + POLE_MARKER_HEIGHT + 0.04;

/// The box each label is centred in. Nothing is drawn in it; it exists so that the text
/// inside can be centred on a point rather than beginning at it.
const LABEL_BOX: f32 = 40.0;

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
        app.insert_resource(Planet::opening_on(self.spec))
            .insert_resource(Orbit::default())
            .insert_resource(Drag::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    choose_size,
                    build_globe,
                    drag_to_turn,
                    keys_to_turn,
                    wheel_to_zoom,
                    reset_view,
                    apply_orbit,
                    place_labels,
                )
                    .chain(),
            );
    }
}

/// Which planet is being looked at. Changing the size rebuilds the world.
#[derive(Resource, Clone, Copy)]
struct Planet {
    size: PlanetSize,
    base: WorldSpec,
}

impl Planet {
    /// Takes the size from the requested territory count, so the application states what
    /// it wants in one place. A count that is not one of the five sizes falls back to the
    /// largest rather than failing - the viewer is still useful at any Goldberg count.
    fn opening_on(base: WorldSpec) -> Self {
        let size =
            PlanetSize::with_territory_count(base.params.region_count).unwrap_or(PlanetSize::Huge);
        Self { size, base }
    }

    fn spec(&self) -> WorldSpec {
        WorldSpec {
            params: Params {
                region_count: self.size.territory_count(),
                ..self.base.params
            },
            ..self.base
        }
    }
}

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
            pitch: RESTING_PITCH,
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

/// Something written on the sphere: a territory's id, or a pole's letter.
///
/// The anchor is in the planet's own space, so it travels with the world when that turns
/// and this never has to know how the world is currently oriented.
#[derive(Component)]
struct Label {
    anchor: Vec3,
}

/// The readout in the corner.
#[derive(Component)]
struct Hud;

/// Everything belonging to the planet currently on screen, which a rebuild clears away
/// first. The globe and its children are one tree; the labels are separate, because they
/// are screen-space interface rather than parts of the world.
type BuiltForThisPlanet = Or<(With<Globe>, With<Label>)>;

fn setup(mut commands: Commands, orbit: Res<Orbit>) {
    // Ambient light rides on the camera in Bevy 0.19 rather than being a global
    // resource. Enough of it that the side facing away from the key light is still
    // readable - this is a map before it is a photograph.
    commands.spawn((
        Camera3d::default(),
        camera_at(orbit.distance),
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
        Hud,
        Text::new(String::new()),
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

/// Builds the world and everything drawn on it, and rebuilds when the size changes.
// A system's parameters are its dependency list, which Bevy reads to schedule it.
// Shortening it would mean hiding a dependency, not removing one.
#[allow(clippy::too_many_arguments)]
fn build_globe(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planet: Res<Planet>,
    orbit: Res<Orbit>,
    mut built: Local<bool>,
    previous: Query<Entity, BuiltForThisPlanet>,
    mut hud: Query<&mut Text, With<Hud>>,
) {
    if *built && !planet.is_changed() {
        return;
    }
    *built = true;
    for entity in &previous {
        commands.entity(entity).despawn();
    }

    let world = World::build(planet.spec());
    let solid =
        sphere_tessellation::solid(&world.tessellation.seeds, &world.tessellation.neighbours);
    let panels = mesh::build(&solid, &world.coloring);

    // Vertex colours carry the region colouring, so one material serves the whole world
    // however many regions it has.
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
    let spike = meshes.add(Cone {
        radius: POLE_MARKER_RADIUS,
        height: POLE_MARKER_HEIGHT,
    });

    commands
        .spawn((Globe, globe_transform(&orbit), Visibility::default()))
        .with_children(|globe| {
            globe.spawn((
                Mesh3d(meshes.add(to_bevy_mesh(&panels))),
                MeshMaterial3d(panel_material),
            ));
            globe.spawn((
                Mesh3d(meshes.add(Sphere::new(UNDERSIDE).mesh().ico(4).unwrap())),
                MeshMaterial3d(underside),
            ));

            // The poles, marked at both ends of the axis. North and south get different
            // colours as well as different letters, so a glance at the spike alone says
            // which end you are looking at - a marker that only says "this is a pole"
            // leaves the more useful question unanswered.
            for (pole, colour) in poles() {
                let outward = to_view(pole);
                globe.spawn((
                    Mesh3d(spike.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: colour,
                        perceptual_roughness: 0.35,
                        ..default()
                    })),
                    Transform {
                        translation: outward * (POLE_MARKER_BASE + POLE_MARKER_HEIGHT / 2.0),
                        // A cone is built standing on +y, so point that at the pole.
                        rotation: Quat::from_rotation_arc(Vec3::Y, outward),
                        ..default()
                    },
                ));
            }
        });

    // A territory's id, written at the centre of its panel. The hub vertex of the panel's
    // triangle fan is exactly that centre, so the label lands where the panel is drawn
    // rather than where its seed happens to sit.
    for (region, span) in panels.regions.iter().enumerate() {
        let hub = panels.positions[span.first_vertex as usize];
        spawn_label(
            &mut commands,
            Vec3::from_array(hub),
            &planet_model::RegionId(region as u32).number().to_string(),
            readable_on(panels.colors[span.first_vertex as usize]),
            11.0,
        );
    }
    for (pole, colour) in poles() {
        let letter = if pole == Direction::NORTH_POLE {
            "N"
        } else {
            "S"
        };
        spawn_label(
            &mut commands,
            to_view(pole) * POLE_LABEL_HEIGHT,
            letter,
            colour,
            17.0,
        );
    }

    if let Ok(mut text) = hud.single_mut() {
        *text = Text::new(summary(planet.size, &world, &panels));
    }
}

/// Ink that can be read against a panel of the given colour.
///
/// The palette runs from pale sand to near-black navy, so any single ink is legible on
/// some territories and invisible on others - which was exactly what happened to the ids
/// on the darker panels. Relative luminance decides between the two. The panel colours
/// are already linear, which is the space those coefficients are defined in.
fn readable_on(panel: [f32; 4]) -> Color {
    let luminance = 0.2126 * panel[0] + 0.7152 * panel[1] + 0.0722 * panel[2];
    if luminance > 0.18 {
        Color::srgb(0.05, 0.06, 0.08)
    } else {
        Color::srgb(0.93, 0.95, 0.98)
    }
}

/// The two poles, and the colour each is marked in.
fn poles() -> [(Direction, Color); 2] {
    [
        // Warm for north, cool for south. Neither is a territory colour, so a pole can
        // never be mistaken for one.
        (Direction::NORTH_POLE, Color::srgb(0.99, 0.74, 0.28)),
        (Direction::SOUTH_POLE, Color::srgb(0.42, 0.79, 0.99)),
    ]
}

fn spawn_label(commands: &mut Commands, anchor: Vec3, text: &str, colour: Color, size: f32) {
    commands
        .spawn((
            Label { anchor },
            // The box is what centres the text on the anchor. Without it the text would
            // begin at the point instead of sitting over it.
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(LABEL_BOX),
                height: Val::Px(LABEL_BOX),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Visibility::Hidden,
        ))
        .with_children(|label| {
            label.spawn((
                Text::new(text.to_string()),
                TextFont {
                    font_size: FontSize::Px(size),
                    ..default()
                },
                TextColor(colour),
            ));
        });
}

fn summary(size: PlanetSize, world: &World, panels: &mesh::PlanetMesh) -> String {
    let regions = world.tessellation.region_count();
    let shape = match sphere_tessellation::goldberg::arrangements_up_to(regions)
        .into_iter()
        .find(|&(m, n)| sphere_tessellation::goldberg::region_count(m, n) == regions)
    {
        Some((m, n)) => format!("GP({m},{n})"),
        None => "no Goldberg solid at this count".to_string(),
    };
    format!(
        "{} - {regions} territories - {shape}\n{} - {} triangles\n\
         drag or arrows to turn, wheel to zoom, R to reset\n1-5 to change planet size",
        size.name(),
        world.degree_summary(),
        panels.triangle_count()
    )
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

/// Spin first, then tilt: `XYZ` composes as `pitch * yaw`, so yaw is applied to the
/// upright planet and therefore turns it about its own axis. Tilting first would swing
/// the axis away from vertical and leave the poles wandering in circles as you turned,
/// instead of staying at the top and bottom where they belong. This is also what makes
/// the roll for any point on the planet fixed.
fn globe_transform(orbit: &Orbit) -> Transform {
    Transform::from_rotation(
        Quat::from_euler(EulerRot::XYZ, orbit.pitch, orbit.yaw, 0.0) * upright(),
    )
}

fn camera_at(distance: f32) -> Transform {
    Transform::from_xyz(0.0, 0.0, distance).looking_at(Vec3::ZERO, Vec3::Y)
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

/// Puts the view back where it started. A sphere is easy to get lost on - there is no
/// edge to bump into and nothing to say which way up you have ended up - so there has to
/// be a way back that is not dragging until it looks about right.
fn reset_view(keys: Res<ButtonInput<KeyCode>>, mut orbit: ResMut<Orbit>) {
    if keys.just_pressed(KeyCode::KeyR) {
        *orbit = Orbit::default();
    }
}

/// Number keys pick a planet size, smallest to largest.
fn choose_size(keys: Res<ButtonInput<KeyCode>>, mut planet: ResMut<Planet>) {
    const DIGITS: [KeyCode; 5] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
    ];
    for (digit, size) in DIGITS.into_iter().zip(PlanetSize::ALL) {
        // Only write when it would actually change, so the world is not rebuilt in answer
        // to a keypress asking for the size it already is.
        if keys.just_pressed(digit) && planet.size != size {
            planet.size = size;
        }
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
        *transform = globe_transform(&orbit);
    }
    for mut transform in &mut cameras {
        *transform = camera_at(orbit.distance);
    }
}

/// Puts each label over the point it belongs to, and hides the ones round the back.
///
/// The labels are flat text on the screen rather than geometry on the sphere, so they
/// stay the right way up and readable at any angle. The cost is that each has to be
/// positioned every frame, by projecting its anchor through the camera.
fn place_labels(
    cameras: Query<(&Camera, &GlobalTransform)>,
    globes: Query<&GlobalTransform, With<Globe>>,
    mut labels: Query<(&Label, &mut Node, &mut Visibility)>,
) {
    let (Ok((camera, camera_at)), Ok(globe)) = (cameras.single(), globes.single()) else {
        return;
    };
    let eye = camera_at.translation();

    for (label, mut node, mut visibility) in &mut labels {
        let world = globe.transform_point(label.anchor);
        // Whether a label is hidden is a question about the *planet*, so the test is done
        // against the point on the surface below it rather than the label's own position.
        // A pole's letter floats above its spike, and testing it where it actually is
        // asks whether it lies on the visible cap of a larger sphere - a stricter question
        // with a different answer. It hid both poles.
        let surface = world.normalize_or_zero();
        let facing = surface.dot(eye - surface) > 0.0;
        let placed = facing
            .then(|| camera.world_to_viewport(camera_at, world).ok())
            .flatten()
            .filter(|screen| {
                // Off the edge of the window is as good as hidden, and saying so keeps
                // stray nodes out of the layout rather than parked just out of sight.
                let size = camera.logical_viewport_size().unwrap_or(Vec2::ZERO);
                screen.x >= 0.0 && screen.y >= 0.0 && screen.x <= size.x && screen.y <= size.y
            });
        match placed {
            Some(screen) => {
                node.left = Val::Px(screen.x - LABEL_BOX / 2.0);
                node.top = Val::Px(screen.y - LABEL_BOX / 2.0);
                *visibility = Visibility::Inherited;
            }
            None => *visibility = Visibility::Hidden,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn north() -> Vec3 {
        to_view(Direction::NORTH_POLE)
    }

    /// The whole reason for `upright`: the model calls `+z` north and the engine calls
    /// `+y` up, and something has to reconcile them.
    #[test]
    fn standing_the_planet_up_puts_north_at_the_top() {
        let stood = upright() * north();
        assert!(stood.abs_diff_eq(Vec3::Y, 1e-6), "north went to {stood}");
        let south = upright() * to_view(Direction::SOUTH_POLE);
        assert!(
            south.abs_diff_eq(Vec3::NEG_Y, 1e-6),
            "south went to {south}"
        );
    }

    /// The regression test for a bug that shipped once.
    ///
    /// Turning the world must spin it about its own axis, which means the poles stay put
    /// on the screen's vertical however far you turn. The rotation used to be composed
    /// tilt-first, which spun the *tilted* planet about the screen's up instead, and the
    /// poles wandered in circles. Nothing revealed it until there was a marker at the
    /// pole to watch, so it is pinned down here rather than left to be noticed again.
    #[test]
    fn turning_the_world_never_moves_the_poles_sideways() {
        for pitch in [-1.2, -0.4, 0.0, RESTING_PITCH, 1.2] {
            for step in 0..24 {
                let yaw = step as f32 * std::f32::consts::TAU / 24.0;
                let orbit = Orbit {
                    yaw,
                    pitch,
                    distance: RESTING_DISTANCE,
                };
                let pole = globe_transform(&orbit).rotation * north();
                assert!(
                    pole.x.abs() < 1e-5,
                    "at yaw {yaw} pitch {pitch} the pole slid to x = {}",
                    pole.x
                );
                // And it stays a pole: still on the sphere, still opposite its twin.
                assert!((pole.length() - 1.0).abs() < 1e-5);
            }
        }
    }

    /// With no tilt at all, spinning must not move the poles whatsoever - they are on the
    /// axis being spun about.
    #[test]
    fn with_no_tilt_the_poles_are_completely_still() {
        for step in 0..16 {
            let orbit = Orbit {
                yaw: step as f32 * std::f32::consts::TAU / 16.0,
                pitch: 0.0,
                distance: RESTING_DISTANCE,
            };
            let pole = globe_transform(&orbit).rotation * north();
            assert!(pole.abs_diff_eq(Vec3::Y, 1e-5), "pole moved to {pole}");
        }
    }

    /// Tilting is what brings a pole into view, so it had better do that.
    #[test]
    fn tilting_leans_the_north_pole_toward_the_viewer() {
        let level = globe_transform(&Orbit {
            yaw: 0.0,
            pitch: 0.0,
            distance: RESTING_DISTANCE,
        })
        .rotation
            * north();
        let tilted = globe_transform(&Orbit {
            yaw: 0.0,
            pitch: RESTING_PITCH,
            distance: RESTING_DISTANCE,
        })
        .rotation
            * north();
        // The camera looks down -z from +z, so leaning toward it means gaining z.
        assert!(tilted.z > level.z, "{tilted} is no closer than {level}");
    }

    /// A pole's letter has to clear the spike it belongs to, or it would be drawn inside
    /// it. It also has to stay near enough that the camera can frame both.
    #[test]
    fn a_poles_letter_clears_its_spike() {
        let tip = POLE_MARKER_BASE + POLE_MARKER_HEIGHT;
        assert!(
            POLE_LABEL_HEIGHT >= tip,
            "the letter would be inside the spike"
        );
        assert!(
            POLE_LABEL_HEIGHT < RESTING_DISTANCE / 2.0,
            "the letter is so far out the camera cannot frame it"
        );
    }

    /// Ink has to change with the panel under it, or the ids vanish on half the palette.
    #[test]
    fn ids_are_inked_against_the_panel_they_sit_on() {
        let dark = readable_on([0.01, 0.02, 0.06, 1.0]);
        let light = readable_on([0.75, 0.72, 0.5, 1.0]);
        assert_ne!(dark, light, "one ink cannot serve both");
        // Every colour in the real palette must get an ink that contrasts with it.
        for packed in planet_render::palette::REGION_COLORS {
            let panel = linear_of(packed);
            let luminance = 0.2126 * panel[0] + 0.7152 * panel[1] + 0.0722 * panel[2];
            let ink = readable_on(panel);
            let inked_light = ink == Color::srgb(0.93, 0.95, 0.98);
            assert_eq!(
                inked_light,
                luminance <= 0.18,
                "colour {packed:#08x} at luminance {luminance} got the wrong ink"
            );
        }
    }

    /// Mirrors the conversion in `planet_render::mesh`, for the palette check above.
    fn linear_of(packed: u32) -> [f32; 4] {
        let channel = |shift: u32| {
            let encoded = ((packed >> shift) & 0xFF) as f32 / 255.0;
            if encoded <= 0.04045 {
                encoded / 12.92
            } else {
                ((encoded + 0.055) / 1.055).powf(2.4)
            }
        };
        [channel(16), channel(8), channel(0), 1.0]
    }

    /// The size a planet opens on comes from the territory count asked for.
    #[test]
    fn the_opening_size_follows_the_requested_count() {
        for size in PlanetSize::ALL {
            let spec = WorldSpec {
                params: Params {
                    region_count: size.territory_count(),
                    ..Params::default()
                },
                soccer: false,
            };
            let planet = Planet::opening_on(spec);
            assert_eq!(planet.size, size);
            assert_eq!(planet.spec().params.region_count, size.territory_count());
        }
    }

    /// A count that is not one of the five sizes still has to produce a usable planet.
    #[test]
    fn an_unlisted_count_falls_back_rather_than_failing() {
        let spec = WorldSpec {
            params: Params {
                region_count: 122,
                ..Params::default()
            },
            soccer: false,
        };
        let planet = Planet::opening_on(spec);
        assert_eq!(planet.size, PlanetSize::Huge);
        assert_eq!(planet.spec().params.region_count, 92);
    }
}
