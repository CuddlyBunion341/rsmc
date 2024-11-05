use crate::prelude::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum TextureName {
    Air,
    Stone,
    Dirt,
    GrassTop,
    GrassSide,
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

#[derive(Resource)]
pub struct TextureManager {
    textures: HashMap<TextureName, TextureUV>,
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureManager {
    pub fn new() -> Self {
        let mut textures = HashMap::new();
        textures.insert(TextureName::Air, [-1.0, -1.0]);
        textures.insert(TextureName::Stone, [0.0, 0.0]);
        textures.insert(TextureName::Dirt, [0.25, 0.0]);
        textures.insert(TextureName::GrassTop, [0.5, 0.0]);
        textures.insert(TextureName::GrassSide, [0.75, 0.0]);
        textures.insert(TextureName::Bedrock, [0.0, 0.25]);
        textures.insert(TextureName::RedSand, [0.25, 0.25]);
        textures.insert(TextureName::BrownTerracotta, [0.5, 0.25]);
        textures.insert(TextureName::CyanTerracotta, [0.75, 0.25]);
        textures.insert(TextureName::GrayTerracotta, [0.0, 0.5]);
        textures.insert(TextureName::LightGrayTerracotta, [0.25, 0.5]);
        textures.insert(TextureName::OrangeTerracotta, [0.5, 0.5]);
        textures.insert(TextureName::RedTerracotta, [0.75, 0.5]);
        textures.insert(TextureName::Terracotta, [0.0, 0.75]);
        textures.insert(TextureName::YellowTerracotta, [0.25, 0.75]);
        Self { textures }
    }

    pub fn get_texture_uv(&self, name: TextureName) -> Option<&TextureUV> {
        self.textures.get(&name)
    }
}

pub struct Block {
    pub id: BlockId,
    pub texture_names: [TextureName; 6],
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
    add_block!(BlockId::Air, [TextureName::Air; 6], false),
    add_block!(
        BlockId::Grass,
        [
            TextureName::GrassTop,
            TextureName::Dirt,
            TextureName::GrassSide,
            TextureName::GrassSide,
            TextureName::GrassSide,
            TextureName::GrassSide,
        ],
        true
    ),
    add_block!(BlockId::Dirt, [TextureName::Dirt; 6], true),
    add_block!(BlockId::Stone, [TextureName::Stone; 6], true),
    add_block!(BlockId::Bedrock, [TextureName::Bedrock; 6], true),
    add_block!(BlockId::RedSand, [TextureName::RedSand; 6], true),
    add_block!(
        BlockId::BrownTerracotta,
        [TextureName::BrownTerracotta; 6],
        true
    ),
    add_block!(
        BlockId::CyanTerracotta,
        [TextureName::CyanTerracotta; 6],
        true
    ),
    add_block!(
        BlockId::GrayTerracotta,
        [TextureName::GrayTerracotta; 6],
        true
    ),
    add_block!(
        BlockId::LightGrayTerracotta,
        [TextureName::LightGrayTerracotta; 6],
        true
    ),
    add_block!(
        BlockId::OrangeTerracotta,
        [TextureName::OrangeTerracotta; 6],
        true
    ),
    add_block!(
        BlockId::RedTerracotta,
        [TextureName::RedTerracotta; 6],
        true
    ),
    add_block!(BlockId::Terracotta, [TextureName::Terracotta; 6], true),
    add_block!(
        BlockId::YellowTerracotta,
        [TextureName::YellowTerracotta; 6],
        true
    ),
];

type TextureUV = [f32; 2];

impl Block {
    pub fn get_block_face_uvs(
        block_id: BlockId,
        face: CubeFace,
        texture_manager: &TextureManager,
    ) -> Option<[f32; 2]> {
        let block = &BLOCKS[block_id as usize];
        let texture_name = block.texture_names[face as usize];
        texture_manager.get_texture_uv(texture_name).copied()
    }
}
