use bevy::prelude::*;
use rand::prelude::*;
mod core;
mod render;
use render::text_display;

use crate::core::player::PlayerPlugin;

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
            .add_plugins(PlayerPlugin)
            .add_plugins(core::net::NetworkPlugin)
            .insert_resource(seed)
            .add_systems(Startup, (setup_scene))
            .run();
    } else {
        App::new()
            .add_plugins(MinimalPlugins)
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

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(( Mesh2d(meshes.add(Rectangle::new(1000., 700.))), MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))), ));
}

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
