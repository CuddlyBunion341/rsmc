use bevy::log::info;
use rand;
use bevy::app::{App, Update};
use bevy::DefaultPlugins;
use bevy_renet::transport::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;
use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use std::net::{SocketAddrV4, UdpSocket};
use renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy::ecs::system::ResMut;
use bevy::utils::{ SystemTime};

fn main() {
    let mut app = App::new();
    app.add_plugins(RenetClientPlugin);

    let client = RenetClient::new(ConnectionConfig::default());
    app.insert_resource(client);

    // Setup the transport layer
    app.add_plugins(NetcodeClientPlugin);
    app.add_plugins(DefaultPlugins);

    let random_int = rand::random::<u64>();

    let authentication = ClientAuthentication::Unsecure {
        server_addr: std::net::SocketAddr::V4(SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 5000)),
        client_id: random_int,
        user_data: None,
        protocol_id: 0,
    };
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.insert_resource(transport);

    app.add_systems(Update, send_message_system);
    app.add_systems(Update, receive_message_system);

    info!("Starting client {}", random_int);

    app.run();
}

// Systems

fn send_message_system(mut client: ResMut<RenetClient>) {
    // Send a text message to the server
    client.send_message(DefaultChannel::ReliableOrdered, "server message");
}

fn receive_message_system(mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // Handle received message
    }
}
