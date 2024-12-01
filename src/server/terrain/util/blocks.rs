use crate::prelude::*;

pub struct Block {
    pub id: BlockId,
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
    add_block!(BlockId::Air, false),
    add_block!(BlockId::Grass, true),
    add_block!(BlockId::Dirt, true),
    add_block!(BlockId::Stone, true),
    add_block!(BlockId::Bedrock, true),
    add_block!(BlockId::RedSand, true),
    add_block!(BlockId::BrownTerracotta, true),
    add_block!(BlockId::CyanTerracotta, true),
    add_block!(BlockId::GrayTerracotta, true),
    add_block!(BlockId::LightGrayTerracotta, true),
    add_block!(BlockId::OrangeTerracotta, true),
    add_block!(BlockId::RedTerracotta, true),
    add_block!(BlockId::Terracotta, true),
    add_block!(BlockId::YellowTerracotta, true),
];
