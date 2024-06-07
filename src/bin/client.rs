use bevy::app::{App, Startup, Update};
use bevy::asset::Assets;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, ResMut, Resource};
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ButtonState;
use bevy::log::info;
use bevy::math::primitives::Cuboid;
use bevy::math::Vec3;
use bevy::pbr::{MaterialMeshBundle, StandardMaterial};
use bevy::prelude::default;
use bevy::render::color::Color;
use bevy::render::mesh::Mesh;
use bevy::transform::components::Transform;
use bevy::utils::SystemTime;
use bevy::DefaultPlugins;
use bevy_renet::transport::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;
use rand;
use renet::transport::{ClientAuthentication, NetcodeClientTransport};
use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use std::net::{SocketAddrV4, UdpSocket};

#[derive(Resource)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub forward: bool,
    pub backward: bool,
}

#[derive(Component)]
pub struct MyPlayer;

fn main() {
    let mut app = App::new();
    app.add_plugins(RenetClientPlugin);

    let client = RenetClient::new(ConnectionConfig::default());
    app.insert_resource(client);
    app.insert_resource(PlayerInput {
        left: false,
        right: false,
        forward: false,
        backward: false,
    });

    // Setup the transport layer
    app.add_plugins(NetcodeClientPlugin);
    app.add_plugins(DefaultPlugins);

    let client_id = rand::random::<u64>();

    let authentication = ClientAuthentication::Unsecure {
        server_addr: std::net::SocketAddr::V4(SocketAddrV4::new(
            std::net::Ipv4Addr::new(127, 0, 0, 1),
            5000,
        )),
        client_id,
        user_data: None,
        protocol_id: 0,
    };
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.insert_resource(transport);

    app.add_systems(Update, send_message_system);
    app.add_systems(Update, receive_message_system);
    app.add_systems(Update, handle_keyboard_input);
    app.add_systems(Update, update_player_movement);
    app.add_systems(Startup, setup);

    info!("Starting client {}", client_id);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    let mesh = MaterialMeshBundle {
        mesh: meshes.add(Cuboid::default()),
        transform: mesh_transform,
        material: materials.add(StandardMaterial {
            emissive: Color::rgb(255.0, 255.0, 0.0),
            ..default()
        }),
        ..default()
    };

    commands.spawn((mesh, MyPlayer));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn handle_keyboard_input(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut input: ResMut<PlayerInput>,
) {
    for event in keyboard_events.read() {
        let state = event.state == ButtonState::Pressed;
        match event.key_code {
            KeyCode::KeyW => input.forward = state,
            KeyCode::KeyS => input.backward = state,
            KeyCode::KeyD => input.right = state,
            KeyCode::KeyA => input.left = state,
            _ => {}
        }
    }
}

fn update_player_movement(
    input: ResMut<PlayerInput>,
    mut query: Query<(Entity, &mut Transform), With<MyPlayer>>,
) {
    let (_, mut player_transform) = query.single_mut();

    let deltatime = 0.016;
    let speed = 2.0;

    let movement = deltatime * speed;

    player_transform.translation.x += if input.right { movement } else { 0.0 };
    player_transform.translation.x -= if input.left { movement } else { 0.0 };
    player_transform.translation.z += if input.forward { movement } else { 0.0 };
    player_transform.translation.z -= if input.backward { movement } else { 0.0 };
}

// Systems

fn send_message_system(mut client: ResMut<RenetClient>, query: Query<&Transform, With<MyPlayer>>) {
    let position_vector = query.single().translation;
    let position_array = [position_vector.x, position_vector.y, position_vector.z];

    client.send_message(
        DefaultChannel::ReliableOrdered,
        bincode::serialize(&position_array).unwrap(),
    );
}

fn receive_message_system(mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // Handle received message
    }
}
