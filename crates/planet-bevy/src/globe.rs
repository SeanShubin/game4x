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
//! | The roll for any point is fixed, and nothing the user does changes it | [`globe_transform`], and [`planet_presentation::Fingers::moved`] discarding a twist |
//! | Zoom in and out | [`wheel_to_zoom`], [`touch_to_turn`] |
//! | Reset the view to a default | [`reset_view`] |
//! | A territory's id displayed on the sphere | [`place_labels`] |
//! | The poles are visible | [`build_globe`] |
//!
//! Which device does which is not in `spec/planet.md` any more - it is a binding, and
//! `releases/first-release.md` -> Controls holds the ones this release names. What
//! `spec/interface.md` fixes instead is that *how a thing is presented, and how the user
//! acts on it, may follow the platform it runs on*, while what the user can do stays the
//! same. So a drag, an arrow key and a finger all arrive at
//! [`planet_presentation::Orbit::drag`], and none of
//! them is a different feature from the others.

use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::MouseWheel;
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::window::CursorMoved;

use planet_model::PlanetSize;
use planet_presentation::{Gesture, RESTING_DISTANCE, Step};
use planet_render::{Params, World, WorldSpec, mesh};
use sphere_tessellation::Direction;

/// Radians per second while an arrow key is held.
const KEY_SPEED: f32 = 1.1;
/// Fraction of the distance a wheel notch closes. Gentle on purpose - the first
/// prototype's zoom was reported as far too sensitive.
const ZOOM_PER_NOTCH: f32 = 0.09;

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
    /// Whether the globe draws whichever game the front end is holding.
    ///
    /// The application says yes: the planet on screen is the planet being played, and it
    /// follows by watching a counter. A prototype says no - there is no game behind it, and
    /// it drives [`Planet`] itself. Everything else is the same either way, which is why
    /// this is a flag rather than a second plugin.
    follows_the_game: bool,
}

impl GlobePlugin {
    /// The globe as the application uses it, drawing the one game.
    pub fn new(spec: WorldSpec) -> Self {
        Self {
            spec,
            follows_the_game: true,
        }
    }

    /// The globe on its own, for a composition root with no game behind it.
    ///
    /// Nothing added here reaches `game-front`: no counter is watched, no line is typed,
    /// and the keys that start a new game or change the drawing are absent.
    ///
    /// A prototype does not have to touch the game, but the fact that it *could* is what
    /// keeps these boundaries honest. A prototype about polyhedra that had to link the
    /// command language in order to draw a sphere would mean the layering was a diagram
    /// rather than a fact.
    pub fn detached(spec: WorldSpec) -> Self {
        Self {
            spec,
            follows_the_game: false,
        }
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
            .insert_resource(FollowsTheGame(self.follows_the_game))
            .insert_resource(ShowIds::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
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

        // The only three systems that know a game exists. A detached globe has none of
        // them, and so reaches none of the front end either.
        if self.follows_the_game {
            app.add_systems(
                Update,
                (keys_to_choose_size, keys_to_change_drawing, follow_the_game)
                    .chain()
                    .before(build_globe),
            );
        }
    }
}

/// Which planet is being looked at. Changing the size rebuilds the world.
#[derive(Resource, Clone, Copy)]
pub struct Planet {
    regions: usize,
    base: WorldSpec,
}

impl Planet {
    /// A count of regions, rather than one of the game's five named sizes.
    ///
    /// The renderer never cared what a planet was called: it draws a solid with that many
    /// faces. Holding the name here meant the globe could draw only the counts the *game*
    /// has - a fact about the game leaking into the thing that draws it, and the reason a
    /// prototype comparing Goldberg solids had nowhere to ask for the other five.
    fn opening_on(base: WorldSpec) -> Self {
        Self {
            regions: base.params.region_count,
            base,
        }
    }

    /// Draws a planet of this many regions.
    ///
    /// Public so a composition root with no game behind it can drive the globe directly -
    /// see [`GlobePlugin::detached`].
    pub fn show(&mut self, regions: usize) {
        self.regions = regions;
    }

    /// How many regions are being drawn.
    pub fn regions(&self) -> usize {
        self.regions
    }

    fn spec(&self) -> WorldSpec {
        WorldSpec {
            params: Params {
                region_count: self.regions,
                ..self.base.params
            },
            ..self.base
        }
    }
}

/// Where the viewer is, in the only three numbers that matter. The ball turns rather
/// than the camera, so that the light stays put and the terminator does not swing about
/// while you are trying to look at something.
/// Where the viewer is, in a form Bevy can hold as a resource.
///
/// The angles, the limits and the arithmetic are [`planet_presentation::Orbit`]; this adds
/// only the ability to be stored in a world and read by a system. Everything reachable
/// through it is reachable through the deref, so a caller writes `orbit.yaw` and
/// `orbit.drag(step)` exactly as before.
#[derive(Resource, Clone, Copy, Default, Deref, DerefMut)]
pub struct Orbit(pub planet_presentation::Orbit);

/// Where the pointer was on the previous frame, while a drag is in progress.
///
/// Deltas are measured from the cursor's reported position rather than taken from
/// `MouseMotion`. `MouseMotion` is a *device level* signal, and on the web it depends on
/// `movementX`, which is not dependable outside pointer lock - so the flat projection's
/// habit of reading it would have made drag-to-turn a native-only feature. The cursor's
/// position is reported the same way everywhere.
#[derive(Resource, Default)]
struct Drag(Option<Vec2>);

/// The fingers on the glass, in a form Bevy can hold as a resource.
///
/// What a hand is asking for is [`planet_presentation::Fingers`]. This adds storage.
#[derive(Resource, Default, Deref, DerefMut)]
struct Fingers(planet_presentation::Fingers);

/// A point on the glass, in the policy's own units rather than the engine's.
///
/// The one place the two vocabularies meet. Past this line there is no `Vec2`.
fn step(at: Vec2) -> Step {
    Step::new(at.x, at.y)
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
    follows: Res<FollowsTheGame>,
    ids: Res<ShowIds>,
    previous: Query<Entity, BuiltForThisPlanet>,
    mut hud: Query<&mut Text, With<Hud>>,
) {
    if *built && !planet.is_changed() && !drawing.is_changed() && !ids.is_changed() {
        return;
    }
    *built = true;
    for entity in &previous {
        commands.entity(entity).despawn();
    }

    // The same seeds the model's territories were built from, so panel `n` and territory
    // `n` are the same ground by construction. `spec/planet.md` allows only Goldberg
    // counts, so the fallback is for a viewer asking for something the game cannot have.
    let world = World::canonical(planet.regions()).unwrap_or_else(|| World::build(planet.spec()));
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
    for (region, span) in panels
        .regions
        .iter()
        .enumerate()
        .filter(|_| !realistic && ids.0)
    {
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
        *text = Text::new(summary(&world, &panels, *drawing, follows.0));
    }

    *drawn = Drawn {
        drawing: *drawing,
        regions: panels.regions.len(),
        vertices: panels.vertex_count(),
        triangles: panels.triangle_count(),
        // Territory ids, plus the two pole letters, and neither is drawn realistically.
        labels: if realistic || !ids.0 {
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
    let ink = planet_presentation::readable_on(panel);
    Color::srgb(ink[0], ink[1], ink[2])
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

/// The heads-up line, with the drawing named only when a key would change it.
fn summary(
    world: &World,
    panels: &mesh::PlanetMesh,
    drawing: Drawing,
    follows_the_game: bool,
) -> String {
    let other = follows_the_game.then(|| drawing.other().name());
    planet_presentation::summary(world, panels, other)
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
            orbit.drag(step(event.position) - step(previous));
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
            TouchPhase::Started => fingers.began(touch.id, step(touch.position)),
            TouchPhase::Moved => match fingers.moved(touch.id, step(touch.position)) {
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
/// a count that only ever goes up. `spec/interface.md` says actions like this *never
/// require a gesture or a key the platform may lack*, and a tablet lacks every key - so
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

/// Whether this globe follows a game, so the readout can list the bindings that exist.
#[derive(Resource, Clone, Copy)]
struct FollowsTheGame(bool);

/// Whether a territory's id is written on the sphere.
///
/// `spec/planet.md` puts the id in the practical drawing, and the application always shows
/// it. It is a resource so that a prototype can turn it off, and a prototype is allowed to:
/// `docs/prototypes/README.md` lets one take a shortcut the game may not, provided the
/// document says which and why.
///
/// The why is measurable. Every id is a `Text` node projected through the camera every
/// frame by [`place_labels`], so the cost is one node per region per frame - fine at
/// ninety-two, and five hundred nodes of unreadable four-point type at five hundred. A
/// prototype comparing the *shapes* of solids does not need any of them.
#[derive(Resource, Clone, Copy)]
pub struct ShowIds(pub bool);

impl Default for ShowIds {
    fn default() -> Self {
        Self(true)
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
    let Some(count) = game_front::shell::territory_count() else {
        return;
    };
    // Only write when it would change something. Touching a `ResMut` marks it changed,
    // and `build_globe` rebuilds the whole world when it sees that.
    if planet.regions() != count {
        planet.show(count);
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

    use planet_presentation::RESTING_PITCH;

    /// A viewer at these angles, at the resting distance. The geometry tests care about
    /// where the poles land and never about how far away they are.
    fn at(yaw: f32, pitch: f32) -> Orbit {
        Orbit(planet_presentation::Orbit {
            yaw,
            pitch,
            distance: RESTING_DISTANCE,
        })
    }

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
                let pole = globe_transform(&at(yaw, pitch)).rotation * north();
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
            let yaw = step as f32 * std::f32::consts::TAU / 16.0;
            let pole = globe_transform(&at(yaw, 0.0)).rotation * north();
            assert!(pole.abs_diff_eq(Vec3::Y, 1e-5), "pole moved to {pole}");
        }
    }

    /// Tilting is what brings a pole into view, so it had better do that.
    #[test]
    fn tilting_leans_the_north_pole_toward_the_viewer() {
        let level = globe_transform(&at(0.0, 0.0)).rotation * north();
        let tilted = globe_transform(&at(0.0, RESTING_PITCH)).rotation * north();
        // The camera looks down -z from +z, so leaning toward it means gaining z.
        assert!(tilted.z > level.z, "{tilted} is no closer than {level}");
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

    /// The planet drawn is the count that was asked for.
    #[test]
    fn the_globe_draws_the_count_it_was_asked_for() {
        for size in PlanetSize::ALL {
            let planet = Planet::opening_on(spec_of(size.territory_count()));
            assert_eq!(planet.regions(), size.territory_count());
            assert_eq!(planet.spec().params.region_count, size.territory_count());
        }
    }

    /// A count that is not one of the game's five sizes is drawn as asked.
    ///
    /// It used to be rounded down to the largest named size, so asking for 122 drew 92 and
    /// said nothing. That was the game's vocabulary deciding what the renderer could draw,
    /// and it is why `goldberg-view` could not exist: half the solids it compares are
    /// counts the game has no word for.
    #[test]
    fn a_count_the_game_has_no_name_for_is_drawn_anyway() {
        for count in [122, 132, 162, 192, 212] {
            let planet = Planet::opening_on(spec_of(count));
            assert_eq!(planet.regions(), count);
            assert_eq!(planet.spec().params.region_count, count);
        }
    }

    /// The globe can be told to draw a different planet, which is how a composition root
    /// with no game behind it drives it.
    #[test]
    fn a_globe_can_be_told_which_planet_to_draw() {
        let mut planet = Planet::opening_on(spec_of(12));
        planet.show(212);
        assert_eq!(planet.regions(), 212);
        assert_eq!(planet.spec().params.region_count, 212);
    }

    fn spec_of(regions: usize) -> WorldSpec {
        WorldSpec {
            params: Params {
                region_count: regions,
                ..Params::default()
            },
            soccer: false,
        }
    }
}
