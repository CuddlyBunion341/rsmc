use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct PlayerState {
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Event)]
pub struct RemotePlayerSpawnedEvent {
    pub client_id: ClientId,
}

#[derive(Event)]
pub struct RemotePlayerDespawnedEvent {
    pub client_id: ClientId,
}

#[derive(Event)]
pub struct RemotePlayerSyncEvent {
    pub players: HashMap<ClientId, PlayerState>,
}
