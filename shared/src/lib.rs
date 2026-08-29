use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Message, Serialize, Deserialize)]
pub struct Ping;

#[derive(Message, Serialize, Deserialize)]
pub struct Pong;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_client_message::<Ping>(Channel::Ordered)
           .add_server_message::<Pong>(Channel::Ordered);
    }
}