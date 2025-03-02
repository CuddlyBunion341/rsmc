use crate::prelude::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum TextureName {
    Air,
    Stone,
    CobbleStone,
    Dirt,
    Sand,
    GrassTop,
    GrassSide,
    IronOre,
    CoalOre,
    Bedrock,
    OakLeaves,
    OakLogTop,
    OakLogSide,
}

use TextureName::*;

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

        Self::get_texture_coordinates().iter().for_each(|(texture_name, (u, v))| {
            if texture_name.clone() != Air { // exclude Air, it is special and used as Placeholder
                textures.insert(*texture_name, [*u, *v]);
            }
        });

        Self { textures }
    }

    fn get_texture_coordinates() -> Vec<(TextureName, (f32, f32))> {
        let textures: [[TextureName; 4]; 4] = [
            [ Stone, CobbleStone, GrassTop, OakLeaves ],
            [ IronOre, Sand, GrassSide, OakLogTop ],
            [ CoalOre, Bedrock, Dirt, OakLogSide ],
            [ Air, Air, Air, Air ]
        ];

        let items: Vec<(TextureName, (f32, f32))> = textures.iter().enumerate().flat_map(|(row_index, row)| {
            row.iter().enumerate().map(|(col_index, texture)| {
                (
                    texture,
                    (
                        1.0 / 4.0 * (col_index as f32),
                        1.0 / 4.0 * (row_index as f32),
                    )
                )
            }).collect()
        }).collect();

        items
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

pub static BLOCKS: [Block; 10] = [
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
    add_block!(BlockId::CobbleStone, [TextureName::CobbleStone; 6], true),
    add_block!(BlockId::Bedrock, [TextureName::Bedrock; 6], true),
    add_block!(BlockId::IronOre, [TextureName::IronOre; 6], true),
    add_block!(BlockId::CoalOre, [TextureName::CoalOre; 6], true),
    add_block!(
        BlockId::OakLeaves,
        [TextureName::OakLeaves; 6],
        true
    ),
    add_block!(
        BlockId::OakLog,
        [
        TextureName::OakLogTop,
        TextureName::OakLogTop,
        TextureName::OakLogSide,
        TextureName::OakLogSide,
        TextureName::OakLogSide,
        TextureName::OakLogSide,
        ],
        true
    )
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
