use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::ops::Add;
use std::time::SystemTime;
use crate::{GameSet, GameState};
use crate::render::text_display::{self, TextSystem};

use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    RenetChannelsExt, RenetClient, RepliconRenetPlugins,
    netcode::{ClientAuthentication, NetcodeClientTransport},
    renet::ConnectionConfig,
};
use shared::{ServerMessage, ClientMessage, ProtocolPlugin};

const PORT: u16 = 5000;
const PROTOCOL_ID: u64 = 0x11223344;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RepliconPlugins, RepliconRenetPlugins, ProtocolPlugin))
            .add_systems(OnEnter(GameState::InGame), connect)
            .add_systems(OnEnter(ClientState::Connected),request_gamestate,)
            .add_systems(PreUpdate, receive_message.after(ClientSystems::Receive));
            //.add_systems(Update, send_ping.run_if(in_state(ClientState::Connected)));
    }
}

fn connect(mut commands: Commands, channels: Res<RepliconChannels>) {
    let connection_config = ConnectionConfig {
        server_channels_config: channels.server_configs(),
        client_channels_config: channels.client_configs(),
        ..Default::default()
    };

    let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), PORT);
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let authentication = ClientAuthentication::Unsecure {
        client_id: current_time.as_millis() as u64,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    commands.insert_resource(RenetClient::new(connection_config));
    commands.insert_resource(
        NetcodeClientTransport::new(current_time, authentication, socket).unwrap(),
    );

    println!("Verbinde zu {server_addr}");
}

fn request_gamestate(
    mut client_messages: MessageWriter<ClientMessage>
) {
    client_messages.write(
        ClientMessage { msg: ("request time".to_string()) },
    );
}

fn receive_message(
    mut messages: MessageReader<ServerMessage>,
    mut commands: Commands,
    mut displays: Query<(&TextSystem, &mut Text)>,
) {
    for message in messages.read() {
        println!("Message vom Server '{}'", message.msg);
        let args: Vec<&str> = message.msg.split(' ').collect();
        match args.first().copied().unwrap_or("") {
            "time" => {
                match args.get(1).copied().unwrap_or("") {
                    "set" => {
                        text_display::modify_display(
                            "date: ".to_string().add(&args.get(2).unwrap_or(&"").to_string().add(&".".to_string().add(&args.get(3).unwrap_or(&"").to_string().add(&".".to_string().add(&args.get(4).unwrap_or(&"").to_string().add(&".".to_string().add(&args.get(5).unwrap_or(&"").to_string()))))))),
                            &mut displays,
                            "time".to_string(),
                        );
                    }
                    
                    "paused" => {
                        text_display::modify_display(
                            "speed: paused".to_string(),
                            &mut displays,
                            "speed".to_string(),
                        );
                    }
                    
                    "speed" => {
                        let speed = args.get(2).unwrap_or(&"").to_string();
                        text_display::modify_display(
                            format!("speed: {}", speed),
                            &mut displays,
                            "speed".to_string(),
                        );
                    }
                    
                    
                    _ => {}
                }
            }
            _ => {}
        }
    }
}