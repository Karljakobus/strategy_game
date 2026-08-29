use bevy::camera::CameraPlugin;
use bevy::prelude::*;
use bevy::time::TimePlugin;
use rand::prelude::*;
use std::ops::Add;
mod core;
use core::player;
use core::player::*;
use core::province_manager;
use core::province_manager::*;
use core::time;
mod render;
use render::camera;
use render::polygon_render;
use render::text_display;

use crate::core::player::PlayerPlugin;
use crate::core::time::TickPlugin;

const TILE_SIZE: i32 = 100;

#[derive(Resource, Debug)]
pub struct Seed(pub i64, pub StdRng);

fn main() {
    let mut rng = rand::rng();
    let seed_num = rng.random();
    let seed = Seed(seed_num, StdRng::seed_from_u64(seed_num as u64));

    let render = true;

    if (render) {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(TextPlugin)
            //.add_plugins(PlayerPlugin)
            .add_plugins(TickPlugin)
            .insert_resource(seed)
            //.add_systems(Startup, (setup_scene))
            .run();
    } else {
        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(TickPlugin)
            .insert_resource(seed)
            .run();
    }
}

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_instructions, get_seed));
    }
}
/*
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // World where we move the player
    for i in 0..10 {
        for j in 0..10 {
            let points = vec![
                Vec2::new((i * TILE_SIZE) as f32, (j * TILE_SIZE) as f32),
                Vec2::new((i * TILE_SIZE + TILE_SIZE) as f32, (j * TILE_SIZE) as f32),
                Vec2::new(
                    (i * TILE_SIZE + TILE_SIZE) as f32,
                    (j * TILE_SIZE + TILE_SIZE) as f32,
                ),
                Vec2::new((i * TILE_SIZE) as f32, (j * TILE_SIZE + TILE_SIZE) as f32),
            ];

            province_manager::spawn_province(
                &mut commands,
                &mut meshes,
                &mut materials,
                points,
                i.to_string().add(&j.to_string()),
            );
        }
    }
}
 */
fn setup_instructions(mut commands: Commands) {
    text_display::create_text(
        &mut commands,
        "Move with WASD.\nSprint with Shift.".to_string(),
        "info".to_string(),
        12,
        12,
    );
}

fn get_seed(seed: Res<crate::Seed>) {
    println!("Global seed: {}", seed.0);
}
