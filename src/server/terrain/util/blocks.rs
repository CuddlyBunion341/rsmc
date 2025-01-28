use crate::prelude::*;

pub struct Block {
    pub id: lib::BlockId,
    pub is_solid: bool,
}

macro_rules! add_block {
    ($block_id:expr, $is_solid:expr) => {
        Block {
            id: $block_id,
            is_solid: $is_solid,
        }
    };
}

pub static BLOCKS: [Block; 14] = [
    add_block!(lib::BlockId::Air, false),
    add_block!(lib::BlockId::Grass, true),
    add_block!(lib::BlockId::Dirt, true),
    add_block!(lib::BlockId::Stone, true),
    add_block!(lib::BlockId::Bedrock, true),
    add_block!(lib::BlockId::RedSand, true),
    add_block!(lib::BlockId::BrownTerracotta, true),
    add_block!(lib::BlockId::CyanTerracotta, true),
    add_block!(lib::BlockId::GrayTerracotta, true),
    add_block!(lib::BlockId::LightGrayTerracotta, true),
    add_block!(lib::BlockId::OrangeTerracotta, true),
    add_block!(lib::BlockId::RedTerracotta, true),
    add_block!(lib::BlockId::Terracotta, true),
    add_block!(lib::BlockId::YellowTerracotta, true),
];
