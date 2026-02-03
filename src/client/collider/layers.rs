use crate::prelude::*;

#[derive(PhysicsLayer, Default, Clone, Copy)]
pub enum GameLayer {
    #[default]
    Player,
    Terrain,
}
