use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Message, Serialize, Deserialize)]
pub struct ClientMessage {
    pub msg: String
}

#[derive(Message, Serialize, Deserialize)]
pub struct ServerMessage {
    pub msg: String
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_client_message::<ClientMessage>(Channel::Ordered)
           .add_server_message::<ServerMessage>(Channel::Ordered);
    }
}