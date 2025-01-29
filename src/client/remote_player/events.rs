use crate::prelude::*;

#[derive(Event)]
pub struct RemotePlayerSpawnedEvent {
    pub client_id: ClientId,
    pub position: Vec3,
}

#[derive(Event)]
pub struct RemotePlayerDespawnedEvent {
    pub client_id: ClientId,
}

#[derive(Event)]
pub struct RemotePlayerSyncEvent {
    pub players: HashMap<ClientId, PlayerState>,
}
