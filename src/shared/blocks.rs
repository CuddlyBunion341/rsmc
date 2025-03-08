#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum BlockId {
    Air,
    Grass,
    Dirt,
    Stone,
    CobbleStone,
    Bedrock,
    IronOre,
    CoalOre,
    OakLeaves,
    OakLog,
    Tallgrass,
}

use serde::{Deserialize, Serialize};
use BlockId::*;

impl From<u8> for BlockId {
    fn from(value: u8) -> Self {
        match value {
            0 => Air,
            1 => Grass,
            2 => Dirt,
            3 => Stone,
            4 => CobbleStone,
            5 => Bedrock,
            6 => IronOre,
            7 => CoalOre,
            8 => OakLeaves,
            9 => OakLog,
            10 => Tallgrass,
            _ => panic!("Invalid block id"),
        }
    }
}

impl Into<u8> for BlockId {
    fn into(self) -> u8 {
        match self {
            Air => 0,
            Grass => 1,
            Dirt => 2,
            Stone => 3,
            CobbleStone => 4,
            Bedrock => 5,
            IronOre => 6,
            CoalOre => 7,
            OakLeaves => 8,
            OakLog => 9,
            Tallgrass => 10,
        }
    }
}

impl BlockId {
    pub fn values() -> [BlockId; 11] {
        [
            Air,
            Grass,
            Dirt,
            Stone,
            CobbleStone,
            Bedrock,
            IronOre,
            CoalOre,
            OakLeaves,
            OakLog,
            Tallgrass,
        ]
    }
}
