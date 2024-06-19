use crate::prelude::*;

pub fn spawn_remote_player_system(
    mut commands: Commands,
    mut query: Query<(&remote_player_components::RemotePlayer, &Transform, &Handle<Mesh>)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = asset_server.load("models/character.glb#Mesh0/Primitive0");
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });
    for (_, transform, _) in query.iter_mut() {
        commands.spawn(PbrBundle {
            mesh: mesh_handle.clone(),
            material: material_handle.clone(),
            transform: transform.clone(),
            ..Default::default()
        });
    }
}

pub fn update_remote_player_system(
    mut query: Query<(&remote_player_components::RemotePlayer, &mut Transform)>,
    time: Res<Time>,
) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation.y = (transform.translation.y + time.delta_seconds()) % 10.0;
    }
}
