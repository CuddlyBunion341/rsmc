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

pub static BLOCKS: [Block; 10] = [
    add_block!(BlockId::Air, false),
    add_block!(BlockId::Grass, true),
    add_block!(BlockId::Dirt, true),
    add_block!(BlockId::Stone, true),
    add_block!(BlockId::CobbleStone, true),
    add_block!(BlockId::Bedrock, true),
    add_block!(BlockId::IronOre, true),
    add_block!(BlockId::CoalOre, true),
    add_block!(BlockId::OakLeaves, true),
    add_block!(BlockId::OakLog, true),
];
