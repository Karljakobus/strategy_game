//! This example showcases a 2D top-down camera with smooth player tracking.
//!
//! ## Controls
//!
//! | Key Binding          | Action        |
//! |:---------------------|:--------------|
//! | `W`                  | Move up       |
//! | `S`                  | Move down     |
//! | `A`                  | Move left     |
//! | `D`                  | Move right    |

use bevy::{post_process::bloom::Bloom, prelude::*};

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 200.;

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 10.;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Province;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
        .add_systems(Update, (move_player, update_camera).chain())
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // World where we move the player
    let points = vec![
        Vec2::new(0.0, 100.0),
        Vec2::new(100.0, 0.0),
        Vec2::new(0.0, -100.0),
        Vec2::new(-100.0, 0.0),
    ];

    commands.spawn((
        Transform::default(),
        Visibility::default(),
        children![
            (
                Mesh2d(meshes.add(create_polygon_mesh(
                    points.clone()
                )),),
                MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
            ),
            (
                Mesh2d(meshes.add(create_outline_mesh(
                    &points.clone(),
                    4.
                ))),
                MeshMaterial2d(materials.add(Color::BLACK)),
            ),
        ],
        Province,
    ));

    // Player
    commands.spawn((
        Player,
        Transform::from_xyz(0., 0., 2.),
    ));
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new("Move the light with WASD.\nSprint with Shift."),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

/// Update the player position with keyboard inputs.
/// Note that the approach used here is for demonstration purposes only,
/// as the point of this example is to showcase the camera tracking feature.
///
/// A more robust solution for player movement can be found in `examples/movement/physics_in_fixed_timestep.rs`.
fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;
    let mut sprint = 1.;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    if kb_input.pressed(KeyCode::ShiftLeft) || kb_input.pressed(KeyCode::ShiftRight) {
        sprint = 3.;
    }

    // Progressively update the player's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs() * sprint;
    player.translation += move_delta.extend(0.);
}

fn create_polygon_mesh(
    vertices: Vec<Vec2>
) -> Mesh {
    let mut indices = Vec::new();

    for i in 1..vertices.len()-1 {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i+1) as u32);
    }

    let positions: Vec<[f32; 3]> = vertices
        .iter()
        .map(|v| [v.x, v.y, 0.0])
        .collect();

    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}

fn create_outline_mesh(points: &[Vec2], width: f32) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let half_width = width / 2.0;

    for i in 0..points.len() {
        let a = points[i];

        // Beim letzten Punkt wieder zum ersten Punkt
        let b = points[(i + 1) % points.len()];

        // Richtung der Kante
        let direction = (b - a).normalize();

        // Senkrechter Vektor
        let normal = Vec2::new(-direction.y, direction.x);

        // Die vier Ecken des Rechtecks
        let v0 = a + normal * half_width;
        let v1 = a - normal * half_width;
        let v2 = b - normal * half_width;
        let v3 = b + normal * half_width;

        let start = vertices.len() as u32;

        vertices.push([v0.x, v0.y, 0.0]);
        vertices.push([v1.x, v1.y, 0.0]);
        vertices.push([v2.x, v2.y, 0.0]);
        vertices.push([v3.x, v3.y, 0.0]);

        // Zwei Dreiecke
        indices.extend_from_slice(&[
            start,
            start + 1,
            start + 2,

            start,
            start + 2,
            start + 3,
        ]);
    }

    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(
        bevy::mesh::Indices::U32(indices),
    )
}