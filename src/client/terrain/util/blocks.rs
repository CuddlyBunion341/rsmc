use crate::prelude::*;

pub struct Block {
    pub id: BlockId,
    pub texture_names: [&'static TextureUV; 6],
    pub is_solid: bool,
}

macro_rules! add_block {
    ($block_id:expr, $texture_names:expr, $is_solid:expr) => {
        Block {
            id: $block_id,
            texture_names: $texture_names,
            is_solid: $is_solid,
        }
    };
}

pub static BLOCKS: [Block; 14] = [
    add_block!(BlockId::Air, [&AIR_TEXTURE; 6], false),
    add_block!(
        BlockId::Grass,
        [
            &GRASS_TOP,
            &DIRT_TEXTURE,
            &GRASS_SIDE,
            &GRASS_SIDE,
            &GRASS_SIDE,
            &GRASS_SIDE,
        ],
        true
    ),
    add_block!(BlockId::Dirt, [&DIRT_TEXTURE; 6], true),
    add_block!(BlockId::Stone, [&STONE_TEXTURE; 6], true),
    add_block!(BlockId::Bedrock, [&BEDROCK; 6], true),
    add_block!(BlockId::RedSand, [&RED_SAND; 6], true),
    add_block!(BlockId::BrownTerracotta, [&BROWN_TERRACOTTA; 6], true),
    add_block!(BlockId::CyanTerracotta, [&CYAN_TERRACOTTA; 6], true),
    add_block!(BlockId::GrayTerracotta, [&GRAY_TERRACOTTA; 6], true),
    add_block!(
        BlockId::LightGrayTerracotta,
        [&LIGHT_GRAY_TERRACOTTA; 6],
        true
    ),
    add_block!(BlockId::OrangeTerracotta, [&ORANGE_TERRACOTTA; 6], true),
    add_block!(BlockId::RedTerracotta, [&RED_TERRACOTTA; 6], true),
    add_block!(BlockId::Terracotta, [&TERRACOTTA; 6], true),
    add_block!(BlockId::YellowTerracotta, [&YELLOW_TERRACOTTA; 6], true),
];

type TextureUV = [f32; 2];

const AIR_TEXTURE: TextureUV = [-1.0, -1.0];
const STONE_TEXTURE: TextureUV = [0.0, 0.0];
const DIRT_TEXTURE: TextureUV = [0.25, 0.0];
const GRASS_TOP: TextureUV = [0.5, 0.0];
const GRASS_SIDE: TextureUV = [0.75, 0.0];
const BEDROCK: TextureUV = [0.0, 0.25];
const RED_SAND: TextureUV = [0.25, 0.25];
const BROWN_TERRACOTTA: TextureUV = [0.5, 0.25];
const CYAN_TERRACOTTA: TextureUV = [0.75, 0.25];
const GRAY_TERRACOTTA: TextureUV = [0.0, 0.5];
const LIGHT_GRAY_TERRACOTTA: TextureUV = [0.25, 0.5];
const ORANGE_TERRACOTTA: TextureUV = [0.5, 0.5];
const RED_TERRACOTTA: TextureUV = [0.75, 0.5];
const TERRACOTTA: TextureUV = [0.0, 0.75];
const YELLOW_TERRACOTTA: TextureUV = [0.25, 0.75];

impl Block {
    pub fn get_block_face_uvs(block_id: BlockId, face: CubeFace) -> Option<[f32; 2]> {
        let block = &BLOCKS[block_id as usize];
        let texture_uv = block.texture_names[face as usize];
        Some(*texture_uv)
    }
}
