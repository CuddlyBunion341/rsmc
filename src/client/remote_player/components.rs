use crate::prelude::*;

#[derive(Component)]
pub struct RemotePlayer {
    pub client_id: ClientId,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct RemotePlayerGizmos;
