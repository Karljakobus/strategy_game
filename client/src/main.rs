use bevy::prelude::*;
use rand::prelude::*;
mod core;
mod render;
use render::text_display;
use std::process::Command;

use crate::core::player::PlayerPlugin;
use render::main_menu::MainMenuPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameSet;

#[derive(Resource, Debug)]
pub struct Rnd(pub StdRng);

fn main() {
    let mut rng = rand::rng();
    let seed: u64 = rng.random();
    let rnd = Rnd(StdRng::seed_from_u64(seed as u64));

    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .insert_resource(rnd)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {

        app.add_plugins((
            TextPlugin,
            PlayerPlugin,
            core::net::NetworkPlugin,
        ));

        // Alles, was zum Game gehört, läuft nur in InGame
        app.configure_sets(
            Update,
            GameSet.run_if(in_state(GameState::InGame)),
        );

        // Dinge, die beim Betreten des Spiels passieren
        app.add_systems(
            OnEnter(GameState::InGame),
            setup_scene,
        );

        // Dinge, die beim Verlassen des Spiels passieren
        app.add_systems(
            OnExit(GameState::InGame),
            cleanup_scene,
        );
    }
}

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (setup_instructions));
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("!!! setup_scene");

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));
}

fn setup_instructions(mut commands: Commands) {
    println!("!!! setup_instructions");

    text_display::create_text(
        &mut commands,
        "Move with WASD.\nSprint with Shift.".to_string(),
        "info".to_string(),
        12,
        12,
    );

    text_display::create_text_ur( &mut commands, "date: --".to_string(), "time".to_string(), 12, 12, );
    text_display::create_text_ur( &mut commands, "speed: --".to_string(), "speed".to_string(), 12, 32, );
}

fn cleanup_scene() {
    
}

pub fn start_server() {
    let project_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Projektverzeichnis konnte nicht gefunden werden");

    let server_manifest = project_dir.join("server/Cargo.toml");

    println!("Baue Server: {:?}", server_manifest);

    let build = Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            server_manifest.to_str().unwrap(),
        ])
        .status()
        .expect("Cargo konnte nicht gestartet werden");

    if !build.success() {
        panic!("Server konnte nicht gebaut werden!");
    }

    println!("Server erfolgreich gebaut!");

    let server_exe = project_dir.join("target/debug/server");

    println!("Starte Server: {:?}", server_exe);

    Command::new(server_exe)
        .spawn()
        .expect("Server konnte nicht gestartet werden");
}