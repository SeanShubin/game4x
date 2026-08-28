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
//! | Rotate to be above any point | [`drag_to_turn`], [`keys_to_turn`], [`touch_to_turn`] |
//! | The roll for any point is fixed, and nothing the user does changes it | [`globe_transform`], and [`Fingers::moved`] discarding a twist |
//! | Zoom in and out | [`wheel_to_zoom`], [`touch_to_turn`] |
//! | Reset the view to a default | [`reset_view`] |
//! | A territory's id displayed on the sphere | [`place_labels`] |
//! | The poles are visible | [`build_globe`] |
//!
//! Which device does which is not in `spec/planet.md` any more - it is a binding, and
//! `releases/first-release.md` -> Controls holds the ones this release names. What
//! `spec/interface.md` fixes instead is that *how a thing is presented, and how the user
//! acts on it, may follow the platform it runs on*, while what the user can do stays the
//! same. So a drag, an arrow key and a finger all arrive at [`Orbit::drag`], and none of
//! them is a different feature from the others.

use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::MouseWheel;
use bevy::input::touch::{TouchInput, TouchPhase};
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

/// How far below the surface the dark ball sits, as a fraction of how far the panels
/// themselves reach.
///
/// It used to be a fixed 0.965, which held for the larger planets and failed for the
/// small ones: a twelve-territory planet has territories wide enough that each flat panel
/// dips to about 0.93, so the ball came through the middle of every one and left only a
/// star-shaped rim showing. The radius now follows the geometry, so it is correct at
/// every size instead of at most of them.
const UNDERSIDE_BELOW: f32 = 0.985;

/// The spike marking each end of the planet's axis.
const POLE_MARKER_RADIUS: f32 = 0.045;
const POLE_MARKER_HEIGHT: f32 = 0.19;
const POLE_MARKER_BASE: f32 = 0.94;

/// Where a pole's letter floats: just clear of the spike's tip, rather than on top of it
/// or so far past it that it leaves the window.
const POLE_LABEL_HEIGHT: f32 = POLE_MARKER_BASE + POLE_MARKER_HEIGHT + 0.04;

// A pole's letter has to clear the spike it belongs to, or it would be drawn inside it,
// and it has to stay near enough that the camera can frame both. Checked when this
// compiles rather than when it runs, since all three are constants.
const _: () = assert!(POLE_LABEL_HEIGHT >= POLE_MARKER_BASE + POLE_MARKER_HEIGHT);
const _: () = assert!(POLE_LABEL_HEIGHT < RESTING_DISTANCE / 2.0);

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
            .insert_resource(Fingers::default())
            .insert_resource(Followed::default())
            .insert_resource(ResetsSeen::default())
            .insert_resource(Drawing::default())
            .insert_resource(DrawingAsksSeen::default())
            .insert_resource(Drawn::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    keys_to_choose_size,
                    keys_to_change_drawing,
                    follow_the_game,
                    build_globe,
                    drag_to_turn,
                    touch_to_turn,
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
pub struct Orbit {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
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

impl Orbit {
    /// Turn by an angle. The pitch is clamped short of the pole, where the axis
    /// degenerates and yaw and roll stop being separable.
    ///
    /// There is no argument for roll, and that is the point: `spec/planet.md` says the
    /// roll for any point on the planet is fixed and nothing the user does changes it, so
    /// there is nothing for a gesture to reach even if one offered it.
    fn turn(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch = (self.pitch + pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    /// Turn by a step across the glass, in pixels.
    ///
    /// A mouse drag, a held arrow key and a finger are three ways of producing this step
    /// and are not three features. `spec/interface.md`: what the user can do is the same
    /// whichever platform it is, and only how they act on it follows the device.
    fn drag(&mut self, step: Vec2) {
        self.turn(step.x * DRAG_SENSITIVITY, step.y * DRAG_SENSITIVITY);
    }

    /// Move the viewer nearer or further by a factor of the distance remaining, so the
    /// same gesture covers the same proportion whether you are close in or far out.
    fn scale_distance(&mut self, factor: f32) {
        self.distance = (self.distance * factor).clamp(CLOSEST, FURTHEST);
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

/// The smallest separation, in pixels, that a pinch is measured against.
///
/// Two fingers that land almost on top of each other give a ratio with a very small
/// number underneath it, and one further pixel of separation would then read as an
/// enormous zoom. Below this the pair is held but not acted on.
const PINCH_FLOOR: f32 = 24.0;

/// What a change in the fingers on the glass asks the view to do.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Gesture {
    /// Turn the world by a step in pixels - the same step a mouse drag produces, and
    /// handed to the same method.
    Turn(Vec2),
    /// Zoom, by the factor the fingers' separation changed by. Greater than one is
    /// fingers spreading apart.
    Pinch(f32),
}

/// The fingers on the glass, and where each of them was last seen.
///
/// Touch is tracked here rather than read from Bevy's `Touches` resource because
/// `Touches` only refreshes `previous_position` on frames that carried an event: with a
/// finger held still, `Touch::delta` keeps reporting the last movement and the world
/// would drift on. Reading the messages and keeping the previous position here is the
/// same shape as [`Drag`], which reads `CursorMoved` for the same reason.
///
/// At most two are tracked. A third finger is ignored rather than queued, because there
/// is nothing for it to mean.
#[derive(Resource, Default)]
struct Fingers {
    down: Vec<(u64, Vec2)>,
}

impl Fingers {
    /// The separation between the pair, or `None` unless there are exactly two.
    fn gap(&self) -> Option<f32> {
        match self.down.as_slice() {
            [(_, first), (_, second)] => Some(first.distance(*second)),
            _ => None,
        }
    }

    fn began(&mut self, id: u64, at: Vec2) {
        if self.down.iter().any(|(known, _)| *known == id) {
            return;
        }
        if self.down.len() < 2 {
            self.down.push((id, at));
        }
    }

    fn ended(&mut self, id: u64) {
        self.down.retain(|(known, _)| *known != id);
    }

    /// Moves one finger and says what the hand as a whole is now asking for.
    ///
    /// One finger turns the world. Two zoom it, and **only** zoom it: the angle between
    /// them is computed nowhere, so a two-finger twist - which every gesture library
    /// offers for free - cannot reach anything. `spec/planet.md` fixes the roll for any
    /// point on the planet and says nothing the user does changes it, so a twist has to
    /// be discarded on purpose rather than left to be wired up by accident.
    fn moved(&mut self, id: u64, to: Vec2) -> Option<Gesture> {
        let index = self.down.iter().position(|(known, _)| *known == id)?;
        let separated = self.gap();
        let from = std::mem::replace(&mut self.down[index].1, to);
        match (separated, self.gap()) {
            (Some(before), Some(after)) if before >= PINCH_FLOOR => {
                Some(Gesture::Pinch(after / before))
            }
            (None, None) => Some(Gesture::Turn(to - from)),
            _ => None,
        }
    }
}

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
    drawing: Res<Drawing>,
    mut built: Local<bool>,
    mut drawn: ResMut<Drawn>,
    previous: Query<Entity, BuiltForThisPlanet>,
    mut hud: Query<&mut Text, With<Hud>>,
) {
    if *built && !planet.is_changed() && !drawing.is_changed() {
        return;
    }
    *built = true;
    for entity in &previous {
        commands.entity(entity).despawn();
    }

    let world = World::build(planet.spec());
    let solid =
        sphere_tessellation::solid(&world.tessellation.seeds, &world.tessellation.neighbours);
    let realistic = *drawing == Drawing::Realistic;
    // The two drawings are two meshes, and the only thing they have in common is the solid
    // they were built from. `spec/planet.md` says they share the camera and nothing else,
    // and this is what that looks like in code: no flag threaded through one builder, two
    // builders that happen to produce the same type.
    let panels = if realistic {
        planet_render::realistic::build(&solid, planet_terrain::WORLD_SEED)
    } else {
        mesh::build(&solid, &world.coloring)
    };

    // Vertex colours carry the region colouring, so one material serves the whole world
    // however many regions it has.
    let panel_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        // Ground is rougher than a painted panel, and a shinier planet reads as plastic.
        perceptual_roughness: if realistic { 0.92 } else { 0.75 },
        reflectance: if realistic { 0.06 } else { 0.15 },
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
            // The ball underneath fills the grooves between inset panels. The realistic
            // drawing has no grooves - its ground meets exactly - so there is nothing to
            // fill, and a sphere sitting just under displaced terrain would poke through
            // every valley.
            if !realistic {
                globe.spawn((
                    Mesh3d(
                        meshes.add(
                            Sphere::new(panels.deepest() * UNDERSIDE_BELOW)
                                .mesh()
                                .ico(4)
                                .unwrap(),
                        ),
                    ),
                    MeshMaterial3d(underside),
                ));
            }

            // The poles, marked at both ends of the axis. North and south get different
            // colours as well as different letters, so a glance at the spike alone says
            // which end you are looking at - a marker that only says "this is a pole"
            // leaves the more useful question unanswered.
            //
            // `spec/planet.md` puts the poles *in the practical drawing*. A spike through
            // the ice cap is exactly the kind of thing that stops a realistic view looking
            // realistic, which is why the specification moved that line under the drawing
            // it belongs to.
            for (pole, colour) in poles().into_iter().filter(|_| !realistic) {
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
    //
    // `spec/planet.md`: *a territory's id is displayed on the sphere in the practical
    // drawing.* An id floating over terrain is the single thing that most stops a
    // realistic view looking realistic, which is why that line names a drawing now.
    for (region, span) in panels.regions.iter().enumerate().filter(|_| !realistic) {
        let hub = panels.positions[span.first_vertex as usize];
        spawn_label(
            &mut commands,
            Vec3::from_array(hub),
            &planet_model::RegionId(region as u32).number().to_string(),
            readable_on(panels.colors[span.first_vertex as usize]),
            11.0,
        );
    }
    for (pole, colour) in poles().into_iter().filter(|_| !realistic) {
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
        *text = Text::new(summary(planet.size, &world, &panels, *drawing));
    }

    *drawn = Drawn {
        drawing: *drawing,
        regions: panels.regions.len(),
        vertices: panels.vertex_count(),
        triangles: panels.triangle_count(),
        // Territory ids, plus the two pole letters, and neither is drawn realistically.
        labels: if realistic {
            0
        } else {
            panels.regions.len() + 2
        },
    };
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

fn summary(size: PlanetSize, world: &World, panels: &mesh::PlanetMesh, drawing: Drawing) -> String {
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
         drag, a finger or the arrows to turn - wheel or pinch to zoom - R to reset\n\
         1-5 start a new game on a planet of that size - T for the {} drawing",
        size.name(),
        world.degree_summary(),
        panels.triangle_count(),
        drawing.other().name()
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
            orbit.drag(event.position - previous);
        }
        drag.0 = Some(event.position);
    }
}

/// Turning and zooming by touch.
///
/// winit routes a touch to `TouchInput` and never to `MouseInput` or `CursorMoved` - its
/// web backend tests `pointer_type != "touch"` before raising a pointer event - so a
/// tablet reaches none of [`drag_to_turn`], [`keys_to_turn`] or [`wheel_to_zoom`], and
/// without this the planet cannot be turned or zoomed at all there. `spec/interface.md`
/// does not allow that: nothing is available in one build and not another, and only the
/// way the user acts on it follows the platform.
fn touch_to_turn(
    mut touches: MessageReader<TouchInput>,
    mut fingers: ResMut<Fingers>,
    mut orbit: ResMut<Orbit>,
) {
    for touch in touches.read() {
        match touch.phase {
            TouchPhase::Started => fingers.began(touch.id, touch.position),
            TouchPhase::Moved => match fingers.moved(touch.id, touch.position) {
                // Fingers spreading apart is a larger ratio and a shorter distance:
                // the world comes toward you.
                Some(Gesture::Pinch(ratio)) => orbit.scale_distance(1.0 / ratio),
                Some(Gesture::Turn(step)) => orbit.drag(step),
                None => {}
            },
            TouchPhase::Ended | TouchPhase::Canceled => fingers.ended(touch.id),
        }
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
    orbit.turn(turn * step, tilt * step);
}

fn wheel_to_zoom(mut wheel: MessageReader<MouseWheel>, mut orbit: ResMut<Orbit>) {
    for notch in wheel.read() {
        // Scale rather than subtract, so a notch covers the same proportion of the
        // remaining distance whether you are close in or far out.
        let factor = (1.0 - ZOOM_PER_NOTCH).powf(notch.y.clamp(-3.0, 3.0));
        orbit.scale_distance(factor);
    }
}

/// Puts the view back where it started. A sphere is easy to get lost on - there is no
/// edge to bump into and nothing to say which way up you have ended up - so there has to
/// be a way back that is not dragging until it looks about right.
///
/// Asked for by the `R` key, or by a control, which is watched the same way the game is:
/// a count that only ever goes up. `spec/interface.md` says an action like this *never
/// requires a gesture or a key the platform may lack*, and a tablet lacks every key - so
/// the key alone would leave the view unresettable on the platform the touch code was
/// written for.
fn reset_view(
    keys: Res<ButtonInput<KeyCode>>,
    mut asked: ResMut<ResetsSeen>,
    mut orbit: ResMut<Orbit>,
) {
    let requested = game_front::shell::resets();
    let by_control = requested != asked.0;
    if by_control {
        asked.0 = requested;
    }
    if by_control || keys.just_pressed(KeyCode::KeyR) {
        *orbit = Orbit::default();
    }
}

/// How many resets the view had been asked for when it last obeyed.
#[derive(Resource, Default)]
struct ResetsSeen(u64);

/// Number keys choose a planet size, smallest to largest.
///
/// **By typing the line, not by writing the size.** `releases/first-release.md`: *choosing
/// a planet size abandons the current game and starts one on a planet of that size*, and
/// the way to say that is `/new <size>`. A key that set [`Planet::size`] directly would
/// let the view hold a planet the model does not have, which is what these keys used to do
/// before the globe followed the game.
///
/// So a key and a typed line take the same path, and the globe learns about the result the
/// same way either way: through [`follow_the_game`], watching the counter.
///
/// It is `/new <size>` rather than `create planet <size>` because the second is available
/// only before `start`, and the shipped build opens on a game already under way - so every
/// size key would have been refused, correctly and uselessly.
fn keys_to_choose_size(keys: Res<ButtonInput<KeyCode>>) {
    for (digit, size) in SIZE_KEYS.into_iter().zip(PlanetSize::ALL) {
        if keys.just_pressed(digit) {
            game_front::shell::with(|console| console.submit(&chooses(size)));
        }
    }
}

/// The digits that choose a size, smallest to largest.
const SIZE_KEYS: [KeyCode; 5] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
];

/// The line a size key types. Exactly what a person would type, because it is the same
/// thing arriving by a different route.
fn chooses(size: PlanetSize) -> String {
    format!("/new {}", size.name())
}

/// Which of the two drawings is on screen.
///
/// `spec/planet.md`: *the planet is drawn either practically or realistically, and the user
/// can change which … the two drawings share the camera and nothing else.* [`Orbit`] is that
/// camera, and it is deliberately untouched by this - switching moves nothing, because
/// there is nothing to move.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Drawing {
    /// Flat colours and a groove at every boundary, so adjacency is legible.
    #[default]
    Practical,
    /// The ground itself, with no boundary drawn anywhere.
    Realistic,
}

impl Drawing {
    fn other(self) -> Self {
        match self {
            Drawing::Practical => Drawing::Realistic,
            Drawing::Realistic => Drawing::Practical,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Drawing::Practical => "practical",
            Drawing::Realistic => "realistic",
        }
    }
}

/// Changes which drawing is on screen.
///
/// `T` for terrain. The release names no binding for this - it names one for rotation,
/// zoom, reset, reaching a surface and choosing a planet size, and this is the capability
/// it has not caught up with - so the key is invented here and the page carries a control
/// beside it, because `spec/interface.md` does not allow a capability to need a key the
/// platform may lack.
fn keys_to_change_drawing(
    keys: Res<ButtonInput<KeyCode>>,
    mut asked: ResMut<DrawingAsksSeen>,
    mut drawing: ResMut<Drawing>,
) {
    // A control asks through a counter, the same shape as the reset one beside it and for
    // the same reason: a button on a page is not on the engine's call stack.
    let requested = game_front::shell::drawing_changes();
    let by_control = requested != asked.0;
    if by_control {
        asked.0 = requested;
    }
    if by_control || keys.just_pressed(KeyCode::KeyT) {
        *drawing = drawing.other();
    }
}

/// How many changes of drawing had been asked for when the last one was obeyed.
#[derive(Resource, Default)]
struct DrawingAsksSeen(u64);

/// What the engine was last handed, for anything outside that needs to check.
///
/// Published rather than measured: once a mesh is uploaded it belongs to the render world
/// and its vertices cannot be read back from `Assets<Mesh>` at all. So the builder says
/// what it built, at the moment it built it, and that is the only account there is.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Drawn {
    pub drawing: Drawing,
    pub regions: usize,
    pub vertices: usize,
    pub triangles: usize,
    /// How many labels are on the sphere. `spec/planet.md` puts a territory's id in the
    /// practical drawing, so in the realistic one this is zero and that is checkable.
    pub labels: usize,
}

/// The last generation the globe was built for.
#[derive(Resource, Default)]
struct Followed(u64);

/// The globe follows the game.
///
/// The one `Session` is outside the engine - on the web it is not even on the same call
/// stack, because the page calls into it - so it cannot hand the new state over when it
/// changes. This watches a counter instead and rebuilds when the number it last saw is
/// not the number it sees now.
///
/// What the globe draws is whichever game the front end is holding, and that changes in
/// two ways: a transition, and `/new <size>` putting a different game there entirely. The
/// counter moves for both, because both leave the same amount to redraw. Number keys used
/// to set the size here directly, which let the view hold a world the model did not have;
/// the view is a projection of the model, so that had to go rather than be kept as a
/// convenience.
fn follow_the_game(mut followed: ResMut<Followed>, mut planet: ResMut<Planet>) {
    let generation = game_front::shell::generation();
    if generation == followed.0 {
        return;
    }
    followed.0 = generation;
    // No planet yet is not an error. A game begins with nothing in it and is designed
    // into existence, so this is what the first few commands of any game look like.
    let Some(size) =
        game_front::shell::territory_count().and_then(PlanetSize::with_territory_count)
    else {
        return;
    };
    // Only write when it would change something. Touching a `ResMut` marks it changed,
    // and `build_globe` rebuilds the whole world when it sees that.
    if planet.size != size {
        planet.size = size;
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

    fn resting() -> Orbit {
        Orbit::default()
    }

    /// A finger is a drag. Not a similar thing that happens to look like one - the same
    /// step, through the same method, to the same result.
    #[test]
    fn a_finger_turns_the_world_exactly_as_a_mouse_drag_does() {
        let step = Vec2::new(37.0, -14.0);

        let mut dragged = resting();
        dragged.drag(step);

        let mut touched = resting();
        let mut fingers = Fingers::default();
        fingers.began(1, Vec2::new(100.0, 100.0));
        let gesture = fingers.moved(1, Vec2::new(100.0, 100.0) + step);
        assert_eq!(gesture, Some(Gesture::Turn(step)));
        touched.drag(step);

        assert_eq!(touched.yaw, dragged.yaw);
        assert_eq!(touched.pitch, dragged.pitch);
    }

    /// The first position after a finger lands only says where it landed. Turning on it
    /// would jerk the world by wherever the finger happened to be, which is the same bug
    /// [`Drag`] avoids for the pointer.
    #[test]
    fn a_finger_landing_turns_nothing() {
        let mut fingers = Fingers::default();
        fingers.began(1, Vec2::new(400.0, 300.0));
        assert_eq!(
            fingers.moved(1, Vec2::new(400.0, 300.0)),
            Some(Gesture::Turn(Vec2::ZERO))
        );
        let mut orbit = resting();
        orbit.drag(Vec2::ZERO);
        assert_eq!(orbit.yaw, resting().yaw);
    }

    /// Fingers spreading apart bring the world closer, and pinching together push it away.
    #[test]
    fn spreading_two_fingers_zooms_in_and_pinching_zooms_out() {
        let mut fingers = Fingers::default();
        fingers.began(1, Vec2::new(300.0, 400.0));
        fingers.began(2, Vec2::new(500.0, 400.0));

        let Some(Gesture::Pinch(ratio)) = fingers.moved(2, Vec2::new(700.0, 400.0)) else {
            panic!("moving one of a pair is a pinch");
        };
        assert!(
            ratio > 1.0,
            "spreading apart is a ratio above one, got {ratio}"
        );
        let mut orbit = resting();
        orbit.scale_distance(1.0 / ratio);
        assert!(
            orbit.distance < RESTING_DISTANCE,
            "spreading did not come closer"
        );

        let Some(Gesture::Pinch(ratio)) = fingers.moved(2, Vec2::new(400.0, 400.0)) else {
            panic!("moving one of a pair is a pinch");
        };
        assert!(
            ratio < 1.0,
            "closing together is a ratio below one, got {ratio}"
        );
        let mut orbit = resting();
        orbit.scale_distance(1.0 / ratio);
        assert!(
            orbit.distance > RESTING_DISTANCE,
            "pinching did not go further out"
        );
    }

    /// `spec/planet.md`: the roll for any point on the planet is fixed, and nothing the
    /// user does changes it.
    ///
    /// A two-finger twist is the gesture that would reach roll, and it comes free with
    /// any pinch. Rotating the pair about its own centre must therefore leave the view
    /// exactly where it was - not approximately, and not only for the yaw.
    #[test]
    fn twisting_two_fingers_changes_nothing() {
        let centre = Vec2::new(400.0, 400.0);
        let arm = 150.0;
        let mut fingers = Fingers::default();
        fingers.began(1, centre + Vec2::new(arm, 0.0));
        fingers.began(2, centre - Vec2::new(arm, 0.0));

        let mut orbit = resting();
        let before = (orbit.yaw, orbit.pitch, orbit.distance);
        for step in 1..=32 {
            let angle = step as f32 * std::f32::consts::TAU / 32.0;
            let offset = Vec2::new(arm * angle.cos(), arm * angle.sin());
            for (id, at) in [(1, centre + offset), (2, centre - offset)] {
                match fingers.moved(id, at) {
                    Some(Gesture::Pinch(ratio)) => orbit.scale_distance(1.0 / ratio),
                    Some(Gesture::Turn(step)) => orbit.drag(step),
                    None => {}
                }
            }
        }
        // The separation never changed, so every ratio was one and nothing moved.
        assert!(
            (orbit.yaw - before.0).abs() < 1e-4,
            "a twist turned the world"
        );
        assert!(
            (orbit.pitch - before.1).abs() < 1e-4,
            "a twist tilted the world"
        );
        assert!(
            (orbit.distance - before.2).abs() < 1e-3,
            "a twist zoomed the world"
        );
    }

    /// A second finger landing mid-drag must not be read as an enormous jump.
    #[test]
    fn a_second_finger_landing_does_not_lurch_the_world() {
        let mut fingers = Fingers::default();
        fingers.began(1, Vec2::new(100.0, 100.0));
        assert!(matches!(
            fingers.moved(1, Vec2::new(140.0, 100.0)),
            Some(Gesture::Turn(_))
        ));
        fingers.began(2, Vec2::new(600.0, 100.0));
        // Now a pair. Moving either one measures separation, never the distance from
        // wherever the other finger happens to be.
        assert!(matches!(
            fingers.moved(1, Vec2::new(150.0, 100.0)),
            Some(Gesture::Pinch(_))
        ));
    }

    /// Two fingers landing on the same spot would divide by something near zero.
    #[test]
    fn a_pinch_that_starts_too_close_together_is_not_acted_on() {
        let mut fingers = Fingers::default();
        fingers.began(1, Vec2::new(400.0, 400.0));
        fingers.began(2, Vec2::new(401.0, 400.0));
        assert_eq!(fingers.moved(2, Vec2::new(460.0, 400.0)), None);
        // Once they are far enough apart it measures normally again.
        assert!(matches!(
            fingers.moved(2, Vec2::new(500.0, 400.0)),
            Some(Gesture::Pinch(_))
        ));
    }

    /// Lifting one of a pair leaves the other turning the world, rather than leaving a
    /// stale finger behind that makes every later move read as a pinch.
    #[test]
    fn lifting_one_of_two_fingers_goes_back_to_turning() {
        let mut fingers = Fingers::default();
        fingers.began(1, Vec2::new(300.0, 400.0));
        fingers.began(2, Vec2::new(500.0, 400.0));
        fingers.ended(2);
        assert_eq!(
            fingers.moved(1, Vec2::new(320.0, 400.0)),
            Some(Gesture::Turn(Vec2::new(20.0, 0.0)))
        );
    }

    /// A third finger is ignored, and ignoring it must not disturb the two that are
    /// already doing something.
    #[test]
    fn a_third_finger_is_ignored() {
        let mut fingers = Fingers::default();
        fingers.began(1, Vec2::new(300.0, 400.0));
        fingers.began(2, Vec2::new(500.0, 400.0));
        fingers.began(3, Vec2::new(700.0, 400.0));
        assert_eq!(fingers.down.len(), 2);
        assert_eq!(fingers.moved(3, Vec2::new(900.0, 400.0)), None);
        assert!(matches!(
            fingers.moved(2, Vec2::new(600.0, 400.0)),
            Some(Gesture::Pinch(_))
        ));
    }

    /// The pitch is clamped whichever device asks for it, or the world inverts at the
    /// pole where yaw and roll stop being separable.
    #[test]
    fn no_device_can_tilt_the_world_past_the_pole() {
        let mut orbit = resting();
        for _ in 0..500 {
            orbit.drag(Vec2::new(0.0, 100.0));
        }
        assert!(orbit.pitch <= PITCH_LIMIT);
        for _ in 0..1000 {
            orbit.drag(Vec2::new(0.0, -100.0));
        }
        assert!(orbit.pitch >= -PITCH_LIMIT);
    }

    /// Zoom is clamped the same way whether it came from a wheel or from a pinch.
    #[test]
    fn no_device_can_zoom_past_the_limits() {
        let mut orbit = resting();
        for _ in 0..200 {
            orbit.scale_distance(0.5);
        }
        assert_eq!(orbit.distance, CLOSEST);
        for _ in 0..200 {
            orbit.scale_distance(2.0);
        }
        assert_eq!(orbit.distance, FURTHEST);
    }

    /// One key per size, in the order the sizes are listed, and each types the line a
    /// person would type. `game-front`'s `starting_over_gives_a_planet_of_the_size_asked_for`
    /// is the other half: that each of those lines does what it says.
    #[test]
    fn the_size_keys_are_one_per_size_smallest_to_largest() {
        assert_eq!(SIZE_KEYS.len(), PlanetSize::ALL.len());
        let typed: Vec<String> = PlanetSize::ALL.into_iter().map(chooses).collect();
        assert_eq!(
            typed,
            [
                "/new tiny",
                "/new small",
                "/new medium",
                "/new large",
                "/new huge",
            ]
        );
        // Ascending, so the digits read the way they look on the keyboard.
        let counts: Vec<usize> = PlanetSize::ALL
            .into_iter()
            .map(PlanetSize::territory_count)
            .collect();
        assert!(
            counts.windows(2).all(|pair| pair[0] < pair[1]),
            "{counts:?}"
        );
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
