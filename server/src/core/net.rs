use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::time::SystemTime;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    RenetChannelsExt, RepliconRenetPlugins,RenetServer,
    netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig},
};
use shared::{ClientMessage, ServerMessage, ProtocolPlugin};

use crate::core::time::{WorldTime, send_date, send_speed};

const PORT: u16 = 5000;
const PROTOCOL_ID: u64 = 0x11223344;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RepliconPlugins, RepliconRenetPlugins, ProtocolPlugin))
            .add_systems(Startup, start_server)
            .add_systems(PreUpdate, receive_message.after(ServerSystems::Receive));
    }
}

fn start_server(mut commands: Commands, channels: Res<RepliconChannels>) {
    let connection_config = ConnectionConfig {
        server_channels_config: channels.server_configs(),
        client_channels_config: channels.client_configs(),
        ..Default::default()
    };

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), PORT);
    let socket = UdpSocket::bind(addr).expect("Port belegt?");

    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 10,
        protocol_id: PROTOCOL_ID,
        authentication: ServerAuthentication::Unsecure,
        public_addresses: vec![addr],
    };

    commands.insert_resource(RenetServer::new(connection_config));
    commands.insert_resource(
        NetcodeServerTransport::new(server_config, socket).unwrap(),
    );

    println!("Server lauscht auf {addr}");
}

fn receive_message(
    mut messages: MessageReader<FromClient<ClientMessage>>,
    mut server_messages: MessageWriter<ToClients<ServerMessage>>,
    mut commands: Commands,
    mut timer: Single<&mut WorldTime>,
) {
    for message in messages.read() {
        println!("Message vom Client '{}'", message.msg);
        let args: Vec<&str> = message.msg.split(' ').collect();
        println!("{} : {}", args.get(0).copied().unwrap_or(""), args.get(1).copied().unwrap_or(""));
        match args.get(0).copied().unwrap_or("") {
            "time" => {
                match args.get(1).copied().unwrap_or("") {
                    "change" => {
                        if (timer.paused) {
                            timer.paused = false;
                        } else {
                            timer.paused = true;
                        }
                    }
                    "speed" => {
                        timer.paused = false;
                        timer.speed = args
                            .get(2)
                            .copied()
                            .unwrap_or("")
                            .parse::<i16>()
                            .unwrap_or(timer.speed);
                    }
                    _ => {}
                }
            }
            "request" => {
                match args.get(1).copied().unwrap_or("") {
                    "time" => {
                        send_date(&mut timer, &mut server_messages);
                        send_speed(&mut timer, &mut server_messages);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}