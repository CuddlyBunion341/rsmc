pub mod systems;

use renet::{ChannelConfig, SendType};

use crate::prelude::*;

const SERVER_ADDR: &str = "127.0.0.1:5000";

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RenetClientPlugin, NetcodeClientPlugin));

        let channel_config_unreliable = ChannelConfig {
            channel_id: 0,
            max_memory_usage_bytes: 1000 * 1024 * 1024,
            send_type: SendType::Unreliable,
        };

        let channel_config_reliable_ordered = ChannelConfig {
            channel_id: 1,
            max_memory_usage_bytes: 1000 * 1024 * 1024,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_millis(300),
            },
        };

        let channel_config_reliable_unordered = ChannelConfig {
            channel_id: 2,
            max_memory_usage_bytes: 1000 * 1024 * 1024,
            send_type: SendType::ReliableUnordered {
                resend_time: Duration::from_millis(300),
            },
        };

        let client = RenetClient::new(ConnectionConfig {
            client_channels_config: Vec::from([
                channel_config_unreliable,
                channel_config_reliable_ordered,
                channel_config_reliable_unordered,
            ]),
            ..Default::default()
        });
        app.insert_resource(client);

        let client_id = rand::random::<u64>();
        let authentication = ClientAuthentication::Unsecure {
            server_addr: SERVER_ADDR.parse().unwrap(),
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

        app.add_systems(Update, networking_systems::receive_message_system);
    }
}
