use crate::prelude::*;

pub fn spawn_remote_player_system(
    mut commands: Commands,
    mut spawn_events: EventReader<remote_player_events::RemotePlayerSpawnedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in spawn_events.read() {
        let client_id = event.client_id;

        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material,
                ..default()
            },
            remote_player_components::RemotePlayer { client_id },
        ));
    }
}

pub fn despawn_remote_player_system(
    mut commands: Commands,
    mut despawn_events: EventReader<remote_player_events::RemotePlayerDespawnedEvent>,
    query: Query<(Entity, &remote_player_components::RemotePlayer)>,
) {
    for event in despawn_events.read() {
        for (entity, remote_player) in query.iter() {
            if remote_player.client_id == event.client_id {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn update_remote_player_system(
    mut sync_events: EventReader<remote_player_events::RemotePlayerSyncEvent>,
    mut query: Query<(&remote_player_components::RemotePlayer, &mut Transform)>,
) {
    let latest_event = sync_events.read().last();

    if let Some(event) = latest_event {
        for (client_id, player_state) in event.players.iter() {
            for (remote_player, mut transform) in query.iter_mut() {
                if remote_player.client_id == *client_id {
                    transform.translation = player_state.position;
                    transform.rotation = player_state.rotation;
                }
            }
        }
    }
}
