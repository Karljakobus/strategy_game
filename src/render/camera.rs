use bevy::{post_process::bloom::Bloom, prelude::*,};
use crate::engine::player::*;

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 200.;

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 10.;



pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}

/// Update the camera position by tracking the player.
pub fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<crate::engine::player::Player>)>,
    player: Single<&Transform, (With<crate::engine::player::Player>, Without<Camera2d>)>,
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
pub fn move_player(
    mut player: Single<&mut Transform, With<crate::engine::player::Player>>,
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