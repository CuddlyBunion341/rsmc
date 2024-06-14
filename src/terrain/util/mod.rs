pub mod blocks;
pub mod chunk;
pub mod generator;
pub mod mesher;

use bevy::app::{App, Plugin};
use blocks::*;
use chunk::*;
use generator::*;
use mesher::*;


pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.ad_systems()

    }
} 
