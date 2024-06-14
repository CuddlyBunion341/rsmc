use crate::mesher::CubeFace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockId {
    Air,
    Grass,
    Dirt,
    Stone,
    Bedrock,
    RedSand,
    BrownTerracotta,
    CyanTerracotta,
    GrayTerracotta,
    LightGrayTerracotta,
    OrangeTerracotta,
    RedTerracotta,
    Terracotta,
    YellowTerracotta,
}

pub struct Block {
    pub id: BlockId,
    pub texture_names: [&'static str; 6],
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

pub static BLOCKS: [Block; 15] = [
    add_block!(BlockId::Air, ["air"; 6], false),
    add_block!(BlockId::Air, ["air"; 6], false),
    add_block!(
        BlockId::Grass,
        [
            "grass_top",
            "dirt",
            "grass_side",
            "grass_side",
            "grass_side",
            "grass_side",
        ],
        true
    ),
    add_block!(BlockId::Dirt, ["dirt"; 6], true),
    add_block!(BlockId::Stone, ["stone"; 6], true),
    add_block!(BlockId::Bedrock, ["bedrock"; 6], true),
    add_block!(BlockId::RedSand, ["red_sand"; 6], true),
    add_block!(BlockId::BrownTerracotta, ["brown_terracotta"; 6], true),
    add_block!(BlockId::CyanTerracotta, ["cyan_terracotta"; 6], true),
    add_block!(BlockId::GrayTerracotta, ["gray_terracotta"; 6], true),
    add_block!(
        BlockId::LightGrayTerracotta,
        ["light_gray_terracotta"; 6],
        true
    ),
    add_block!(BlockId::OrangeTerracotta, ["orange_terracotta"; 6], true),
    add_block!(BlockId::RedTerracotta, ["red_terracotta"; 6], true),
    add_block!(BlockId::Terracotta, ["terracotta"; 6], true),
    add_block!(BlockId::YellowTerracotta, ["yellow_terracotta"; 6], true),
];

impl Block {
    pub fn get_texture_uvs(texture_name: &str) -> Option<[f32; 2]> {
        match texture_name {
            "stone" => Some([0.0, 0.0]),
            "dirt" => Some([0.25, 0.0]),
            "grass_top" => Some([0.5, 0.0]),
            "grass_side" => Some([0.75, 0.0]),
            "bedrock" => Some([0.0, 0.25]),
            "red_sand" => Some([0.25, 0.25]),
            "brown_terracotta" => Some([0.5, 0.25]),
            "cyan_terracotta" => Some([0.75, 0.25]),
            "gray_terracotta" => Some([0.0, 0.5]),
            "light_gray_terracotta" => Some([0.25, 0.5]),
            "orange_terracotta" => Some([0.5, 0.5]),
            "red_terracotta" => Some([0.75, 0.5]),
            "terracotta" => Some([0.0, 0.75]),
            "yellow_terracotta" => Some([0.25, 0.75]),
            _ => None,
        }
    }

    pub fn get_block_face_uvs(block_id: BlockId, face: CubeFace) -> Option<[f32; 2]> {
        let block = &BLOCKS[block_id as usize];
        let texture_name = block.texture_names[face as usize];
        Self::get_texture_uvs(texture_name)
    }
}
