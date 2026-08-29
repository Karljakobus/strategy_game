use bevy::{prelude::*,};

use crate::core::province_manager;
use crate::core::time::WorldTime;
use crate::render::camera;

pub struct PlayerPlugin;
/*
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (camera::setup_camera, setup_player));
        app.add_systems(
            Update,
            ((camera::move_player, camera::update_camera).chain(), player_inputs, province_manager::province_click_system,)
        );
    }
}

#[derive(Component)]
pub struct Player;

pub fn setup_player(
    mut commands: Commands,
) {
    // Player
    commands.spawn((Player, Transform::from_xyz(0., 0., 2.)));
}

pub fn player_inputs(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut timer: Single<&mut WorldTime>,
) {
    if kb_input.just_pressed(KeyCode::Space) {
        if (timer.paused) {
            timer.paused = false;
        } else {
            timer.paused = true;
        }
    }

    if kb_input.just_pressed(KeyCode::Digit1) {
        timer.paused = false;
        timer.speed = 1;
    }

    if kb_input.just_pressed(KeyCode::Digit2) {
        timer.paused = false;
        timer.speed = 2;
    }

    if kb_input.just_pressed(KeyCode::Digit3) {
        timer.paused = false;
        timer.speed = 3;
    }

    if kb_input.just_pressed(KeyCode::Digit4) {
        timer.paused = false;
        timer.speed = 4;
    }

    if kb_input.just_pressed(KeyCode::Digit5) {
        timer.paused = false;
        timer.speed = 5;
    }
}
*/