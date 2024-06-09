use std::{net::UdpSocket, time::SystemTime};

use bevy::{
    app::{App, Update}, ecs::{
        event::EventReader,
        system::{Res, ResMut, Resource},
    }, log::{info}, utils::HashMap, MinimalPlugins
};
use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};

#[derive(Resource)]
pub struct PlayerPositions(HashMap<ClientId, [f32; 3]>);

pub fn main() {
    let mut app = App::new();

    app.add_plugins(RenetServerPlugin);
    app.add_plugins(MinimalPlugins);

    let server = RenetServer::new(ConnectionConfig::default());
    app.insert_resource(server);

    // Transport layer setup
    app.add_plugins(NetcodeServerPlugin);
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(transport);
    app.insert_resource(PlayerPositions(HashMap::default()));

    app.add_systems(Update, send_message_system);
    app.add_systems(Update, receive_message_system);
    app.add_systems(Update, handle_events_system);
    app.run();
}


// Systems

fn send_message_system(mut server: ResMut<RenetServer>, player_positions: Res<PlayerPositions>) {
    let channel_id = 0;
    let payload = bincode::serialize(&player_positions.0).unwrap();
    let message = server.broadcast_message(DefaultChannel::ReliableOrdered, payload);
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let server_message: [f32; 3] = bincode::deserialize(&message).unwrap();

            info!(
                "Received message from client {}: {};{};{}",
                client_id, server_message[0], server_message[1], server_message[2]
            )
        }
    }
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}
