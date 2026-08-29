use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    RenetChannelsExt, RenetClient, RepliconRenetPlugins,
    netcode::{ClientAuthentication, NetcodeClientTransport},
    renet::ConnectionConfig,
};
use shared::{Ping, Pong, ProtocolPlugin};

const PORT: u16 = 5000;
const PROTOCOL_ID: u64 = 0x11223344;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RepliconPlugins, RepliconRenetPlugins, ProtocolPlugin))
            .add_systems(Startup, connect)
            .add_systems(PreUpdate, receive_pong.after(ClientSystems::Receive))
            .add_systems(Update, send_ping.run_if(in_state(ClientState::Connected)));
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

fn send_ping(mut pings: MessageWriter<Ping>) {
    pings.write(Ping);
}

fn receive_pong(mut pongs: MessageReader<Pong>) {
    for _ in pongs.read() {
        println!("Pong vom Server");
    }
}