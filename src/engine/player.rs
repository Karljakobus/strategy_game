use bevy::{prelude::*,};

use crate::engine::time::WorldTime;

#[derive(Component)]
pub struct Player;

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