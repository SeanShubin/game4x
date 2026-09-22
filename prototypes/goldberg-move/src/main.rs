//! Prototype: laying out a move across a Goldberg planet by clicking.
//!
//! The question and the rule are in `lib.rs`. This file is the composition root: it opens a
//! window, borrows the globe from `planet-bevy`, turns a click into a territory, and hands that
//! territory to [`goldberg_move::plan::Plan`].
//!
//! # What it borrows and what it does not
//!
//! `planet-bevy` gives the sphere, the camera, the drag and the zoom; `sphere-tessellation`
//! gives the solid. It links no game: there is no console, no command language, no biome and no
//! terrain here, which is the same test `goldberg-view` applies to the same boundary.
//!
//! **The two things `planet-bevy` does not offer are picking and recentring**, and both are
//! done here from its public parts rather than by reaching inside it. `Orbit` is a public
//! resource, so recentring is arithmetic on two angles; the globe's transform is
//! `pitch * yaw * upright` and `upright` is derived from `Direction::NORTH_POLE`, which is
//! public - so this can reproduce it exactly and cannot drift from it by guessing.

use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow};

use goldberg_move::board::{Board, Where};
use goldberg_move::plan::{Disk, Plan, Said};
use planet_bevy::globe::{GlobePlugin, Orbit, ShowIds};
use planet_render::{Params, WorldSpec};
use sphere_tessellation::Direction;

/// The third-smallest Goldberg solid: `GP(2, 0)`, forty-two territories.
///
/// Sean, 2026-09-21: *lets do this on the third from smallest goldberg polyhedron.* The order
/// is by face count - `GP(1,0)` has twelve, `GP(1,1)` thirty-two, `GP(2,0)` forty-two - and it
/// is derived below rather than written as `42`, so that the solid and the count cannot
/// disagree.
const ARRANGEMENT: (usize, usize) = (2, 0);

/// Where the disks start. Four, far enough apart that a move between any two is several steps
/// and the ambiguous case is easy to stumble into.
const DISKS: [(Where, Color); 4] = [
    (0, Color::srgb(0.95, 0.35, 0.35)),
    (7, Color::srgb(0.35, 0.75, 0.95)),
    (20, Color::srgb(0.95, 0.80, 0.35)),
    (33, Color::srgb(0.55, 0.90, 0.55)),
];

/// How far above the surface a disk floats, as a multiple of the radius.
const ABOVE: f32 = 1.035;

/// A drag that moves the cursor further than this is turning the globe, not clicking it.
const A_CLICK_IS_STILL: f32 = 4.0;

fn main() {
    let (m, n) = ARRANGEMENT;
    let board = Board::goldberg(m, n);
    let spec = WorldSpec {
        params: Params {
            region_count: board.territories(),
            ..Params::default()
        },
        soccer: false,
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window()))
        .add_plugins(GlobePlugin::new(spec))
        .insert_resource(Table {
            disks: DISKS
                .iter()
                .map(|&(standing, _)| Disk { standing })
                .collect(),
            plan: Plan::Idle,
            board,
            said: "click a disk to pick it up".to_string(),
        })
        .insert_resource(Pressed::default())
        .add_systems(Startup, (ids_on, lay_the_disks_out))
        .add_systems(
            Update,
            (
                a_click_is_a_territory,
                carry_the_markers_with_the_globe,
                draw_the_route,
                say_what_happened,
            )
                .chain(),
        )
        .run();
}

/// The board, the disks and how far the move has got.
#[derive(Resource)]
struct Table {
    board: Board,
    disks: Vec<Disk>,
    plan: Plan,
    said: String,
}

/// Where the button went down, so that turning the globe is not also clicking it.
#[derive(Resource, Default)]
struct Pressed(Option<Vec2>);

/// The root every marker hangs from, which is turned to match the globe each frame.
#[derive(Component)]
struct Markers;

/// One disk, by its index in [`Table::disks`].
#[derive(Component)]
struct OnTheBoard(usize);

/// The status line.
#[derive(Component)]
struct Status;

/// The ids are the whole point here: a route is a list of territories and it has to be
/// readable as one.
fn ids_on(mut ids: ResMut<ShowIds>) {
    ids.0 = true;
}

fn lay_the_disks_out(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    table: Res<Table>,
    orbit: Res<Orbit>,
) {
    let disk = meshes.add(Mesh::from(Cylinder::new(0.085, 0.02)));
    commands
        .spawn((Markers, globe_transform(&orbit), Visibility::default()))
        .with_children(|root| {
            for (at, (_, colour)) in DISKS.iter().enumerate() {
                let centre = facing(&table.board, table.disks[at].standing);
                root.spawn((
                    OnTheBoard(at),
                    Mesh3d(disk.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: *colour,
                        ..default()
                    })),
                    standing_on(centre),
                ));
            }
        });

    commands.spawn((
        Status,
        Text::new(String::new()),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(0.92, 0.94, 0.98)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            bottom: Val::Px(12.0),
            ..default()
        },
    ));
}

/// A press and a release in the same place, on the sphere, is a click on a territory.
///
/// **Picking is a ray against the unit sphere and then the nearest centre**, which is exact
/// rather than approximate: the territories are the Voronoi cells of those centres, so the
/// nearest centre to a point on the sphere *is* the territory that point is in. Nothing here
/// reads the mesh, so it cannot disagree with the adjacency the routes are computed over.
fn a_click_is_a_territory(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut pressed: ResMut<Pressed>,
    mut table: ResMut<Table>,
    mut orbit: ResMut<Orbit>,
) {
    let Ok(window) = windows.single() else { return };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    if buttons.just_pressed(MouseButton::Left) {
        pressed.0 = Some(cursor);
    }
    if !buttons.just_released(MouseButton::Left) {
        return;
    }
    let Some(went_down) = pressed.0.take() else {
        return;
    };
    if went_down.distance(cursor) > A_CLICK_IS_STILL {
        return;
    }

    let Ok((camera, camera_at)) = cameras.single() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_at, cursor) else {
        return;
    };
    let Some(hit) = where_it_meets_the_sphere(ray) else {
        table.said = "that was not the planet".to_string();
        return;
    };

    // Into the globe's own coordinates, where the centres live.
    let model = globe_transform(&orbit).rotation.inverse() * hit;
    let Some(face) = nearest(&table.board, model) else {
        return;
    };

    let Table {
        board, disks, plan, ..
    } = &mut *table;
    let said = plan.clicked(board, disks, face);
    // Recentre on wherever the move has reached, which is what makes a long move possible: the
    // next destination is in front of you rather than round the back.
    if matches!(said, Said::Reached { .. } | Said::Moved { .. }) {
        centre_on(&mut orbit, facing(board, face));
    }
    table.said = words(&said, face);
}

/// Keeps the markers on the globe while it turns.
///
/// The disks are not children of the globe entity, because `planet-bevy` does not offer it -
/// so they hang from a root of their own that is given the same rotation. **Derived from the
/// same two angles by the same formula**, rather than copied from the globe's transform, so
/// there is one statement of where the planet is pointing and not two.
fn carry_the_markers_with_the_globe(
    orbit: Res<Orbit>,
    mut roots: Query<&mut Transform, With<Markers>>,
) {
    if !orbit.is_changed() {
        return;
    }
    for mut transform in &mut roots {
        *transform = globe_transform(&orbit);
    }
}

/// The move so far: a line from territory to territory, and a ring on each one stepped onto.
fn draw_the_route(mut gizmos: Gizmos, table: Res<Table>, orbit: Res<Orbit>) {
    let route = table.plan.route();
    let Some(disk) = table.plan.disk() else {
        return;
    };
    let turn = globe_transform(&orbit).rotation;
    let at = |face: Where| turn * facing(&table.board, face) * ABOVE;

    let mut from = at(table.disks[disk].standing);
    gizmos.circle(
        Isometry3d::new(from, Quat::from_rotation_arc(Vec3::Z, from.normalize())),
        0.11,
        Color::srgb(1.0, 1.0, 1.0),
    );
    for &step in route {
        let to = at(step);
        gizmos.line(from, to, Color::srgb(1.0, 0.95, 0.4));
        gizmos.circle(
            Isometry3d::new(to, Quat::from_rotation_arc(Vec3::Z, to.normalize())),
            0.09,
            Color::srgb(1.0, 0.95, 0.4),
        );
        from = to;
    }
}

fn say_what_happened(
    table: Res<Table>,
    mut lines: Query<&mut Text, With<Status>>,
    mut disks: Query<(&OnTheBoard, &mut Transform)>,
) {
    for (on, mut transform) in &mut disks {
        *transform = standing_on(facing(&table.board, table.disks[on.0].standing));
    }
    for mut line in &mut lines {
        *line = Text::new(table.said.clone());
    }
}

fn words(said: &Said, face: Where) -> String {
    match said {
        Said::Nothing(why) => why.to_string(),
        Said::Selected(disk) => format!("disk {disk} picked up - click a territory to move it"),
        Said::Deselected => "put down - click a disk to pick one up".to_string(),
        Said::Cancelled => "move abandoned".to_string(),
        Said::Reached { steps, .. } => format!(
            "{steps} step{} to territory {face} - click it again to move, or another to go on",
            if *steps == 1 { "" } else { "s" }
        ),
        Said::Refused { ways, steps } => format!(
            "territory {face} is {steps} steps away by {ways} different routes - \
             click one of the territories on the way first"
        ),
        Said::Moved { disk, to } => format!("disk {disk} is on territory {to}"),
    }
}

/// The nearest point of the unit sphere the ray meets, or `None` if it misses.
fn where_it_meets_the_sphere(ray: Ray3d) -> Option<Vec3> {
    let from = ray.origin;
    let along: Vec3 = ray.direction.into();
    let half = from.dot(along);
    let under = half * half - (from.length_squared() - 1.0);
    if under < 0.0 {
        return None;
    }
    let distance = -half - under.sqrt();
    (distance > 0.0).then(|| (from + along * distance).normalize())
}

/// Which territory a direction is in: the nearest centre, which is what a Voronoi cell means.
fn nearest(board: &Board, direction: Vec3) -> Option<Where> {
    board
        .centres
        .iter()
        .enumerate()
        .map(|(at, centre)| (at, to_view(*centre).dot(direction)))
        .max_by(|(_, one), (_, two)| one.total_cmp(two))
        .map(|(at, _)| at as Where)
}

/// Turns the globe so that a direction faces the camera.
///
/// The camera sits on `+z` looking at the origin, so *facing the camera* is the model direction
/// landing on `+z` after `pitch * yaw * upright`. Solving that for the two angles is two
/// `atan2`s and no search: yaw brings the point into the `y-z` plane, and pitch lifts it onto
/// the axis.
fn centre_on(orbit: &mut Orbit, direction: Vec3) {
    let u = upright() * direction;
    orbit.0.yaw = (-u.x).atan2(u.z);
    orbit.0.pitch = u.y.atan2(u.x.hypot(u.z));
}

/// A disk lying flat on the surface at a territory's centre.
fn standing_on(centre: Vec3) -> Transform {
    Transform {
        translation: centre * ABOVE,
        rotation: Quat::from_rotation_arc(Vec3::Y, centre),
        ..default()
    }
}

fn facing(board: &Board, face: Where) -> Vec3 {
    to_view(board.centres[face as usize])
}

fn to_view(vector: sphere_tessellation::Vec3) -> Vec3 {
    Vec3::new(vector.x as f32, vector.y as f32, vector.z as f32)
}

/// The same rotation `planet-bevy` puts on the globe, derived the same way.
fn globe_transform(orbit: &Orbit) -> Transform {
    Transform::from_rotation(
        Quat::from_euler(EulerRot::XYZ, orbit.0.pitch, orbit.0.yaw, 0.0) * upright(),
    )
}

/// Stands the planet up: the model's axis is `+z` and the engine's up is `+y`.
fn upright() -> Quat {
    Quat::from_rotation_arc(to_view(Direction::NORTH_POLE.vector()), Vec3::Y)
}

fn window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "goldberg move - click a disk, then click where it goes".to_string(),
            resolution: (1280, 800).into(),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }
}
