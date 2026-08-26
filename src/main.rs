use bevy::{prelude::*,};
mod engine;
use engine::province_manager;
use engine::province_manager::*;
use engine::player;
use engine::player::*;
use engine::time;
mod render;
use render::polygon_render;
use render::camera;
use render::text_display;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_scene, setup_instructions, camera::setup_camera, time::setup_time))
        .add_systems(Update, ((camera::move_player, camera::update_camera).chain(),province_manager::province_click_system, time::update_time, player::player_inputs))
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

    let points2 = vec![
        Vec2::new(100.0, 200.0),
        Vec2::new(200.0, 100.0),
        Vec2::new(100.0, 0.0),
        Vec2::new(0.0, 100.0),
    ];

    commands.spawn((
        Transform::default(),
        Visibility::default(),
        children![
            (
                Mesh2d(meshes.add(polygon_render::create_polygon_mesh(
                    points.clone()
                )),),
                MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
            ),
            (
                Mesh2d(meshes.add(polygon_render::create_outline_mesh(
                    &points.clone(),
                    4.
                ))),
                MeshMaterial2d(materials.add(Color::BLACK)),
            ),
        ],
        Province {
            points: points.clone(),
            id: "a".to_string(),
        },
    ));

    commands.spawn((
        Transform::default(),
        Visibility::default(),
        children![
            (
                Mesh2d(meshes.add(polygon_render::create_polygon_mesh(
                    points2.clone()
                )),),
                MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
            ),
            (
                Mesh2d(meshes.add(polygon_render::create_outline_mesh(
                    &points2.clone(),
                    4.
                ))),
                MeshMaterial2d(materials.add(Color::BLACK)),
            ),
        ],
        Province {
            points: points2.clone(),
            id: "b".to_string(),
        },
    ));

    // Player
    commands.spawn((
        Player,
        Transform::from_xyz(0., 0., 2.),
    ));
}

fn setup_instructions(mut commands: Commands) {
    text_display::create_text(&mut commands, "Move with WASD.\nSprint with Shift.".to_string(), "info".to_string(), 12, 12);
}
