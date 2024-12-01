use std::time::Duration;

pub mod systems;

use crate::prelude::*;

const SERVER_ADDR: &str = "127.0.0.1:5000";

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetServerPlugin);

        let channel_config_unreliable = ChannelConfig {
            channel_id: 0,
            max_memory_usage_bytes: 1000 * 1024 * 1024 * 1024,
            send_type: SendType::Unreliable,
        };

        let channel_config_reliable_ordered = ChannelConfig {
            channel_id: 1,
            max_memory_usage_bytes: 1000 * 1024 * 1024 * 1024,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_millis(300),
            },
        };

        let channel_config_reliable_unordered = ChannelConfig {
            channel_id: 2,
            max_memory_usage_bytes: 1000 * 1024 * 1024 * 1024,
            send_type: SendType::ReliableUnordered {
                resend_time: Duration::from_millis(300),
            },
        };

        let server = RenetServer::new(ConnectionConfig {
            server_channels_config: Vec::from([
                channel_config_unreliable,
                channel_config_reliable_ordered,
                channel_config_reliable_unordered,
            ]),
            ..Default::default()
        });

        app.insert_resource(server);

        app.add_plugins(NetcodeServerPlugin);
        let server_addr = SERVER_ADDR.parse().unwrap();
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

        app.add_systems(Update, networking_systems::receive_message_system);
        app.add_systems(Update, networking_systems::handle_events_system);
    }
}
