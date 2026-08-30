use bevy::{prelude::*,};
use bevy_replicon::shared::message::client_message;
use shared::ClientMessage;

use crate::{GameSet, GameState, render::camera};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (camera::setup_camera, setup_player),);
        app.add_systems(
            Update,
            ((camera::move_player, camera::update_camera).chain(),player_inputs).in_set(GameSet),
        );
    }
}

#[derive(Component)]
pub struct Player;

pub fn setup_player(
    mut commands: Commands,
) {
    println!("!!! setup_player");

    // Player
    commands.spawn((Player, Transform::from_xyz(0., 0., 2.)));
}


pub fn player_inputs(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut client_messages: MessageWriter<ClientMessage>
) {
    if kb_input.just_pressed(KeyCode::Space) {
        client_messages.write(
            ClientMessage { msg: ("time change".to_string()) },
        );
    }

    if kb_input.just_pressed(KeyCode::Digit1) {
        client_messages.write(
            ClientMessage { msg: ("time speed 1".to_string()) },
        );
    }

    if kb_input.just_pressed(KeyCode::Digit2) {
        client_messages.write(
            ClientMessage { msg: ("time speed 2".to_string()) },
        );
    }

    if kb_input.just_pressed(KeyCode::Digit3) {
        client_messages.write(
            ClientMessage { msg: ("time speed 3".to_string()) },
        );
    }

    if kb_input.just_pressed(KeyCode::Digit4) {
        client_messages.write(
            ClientMessage { msg: ("time speed 4".to_string()) },
        );
    }

    if kb_input.just_pressed(KeyCode::Digit5) {
        client_messages.write(
            ClientMessage { msg: ("time speed 5".to_string()) },
        );
    }
}
