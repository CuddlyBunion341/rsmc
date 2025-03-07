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
    Tallgrass
}

pub enum MeshRepresentation {
    None,
    Cube([TextureName; 6]),
    Cross([TextureName; 2])
}

pub struct BlockProperties {
    has_collider: bool,
    mesh_representation: MeshRepresentation
}

impl BlockProperties {
    pub fn new(has_collider: bool, mesh_representation: MeshRepresentation) {
        BlockProperties {has_collider, mesh_representation}
    } 
} 

impl BlockId {
    use MeshRepresentation::*;

    pub fn block_properties(&self) -> BlockProperties {
        let touple = match self {
            BlockId::Air => (true, None()),
            BlockId::Grass => (true, Cube([GrassTop, Dirt, GrassSide, GrassSide, GrassSide, GrassSide])),
            BlockId::Dirt => (true, Cube([Dirt; 6])),
            BlockId::Stone => (true, Cube([Stone; 6])),
            BlockId::CobbleStone => (true, Cube([CobbleStone; 6])),
            BlockId::Bedrock => (true, Cube([Bedrock; 6])),
            BlockId::IronOre => (true, Cube([IronOre; 6])),
            BlockId::CoalOre => (true, Cube([CoalOre; 6])),
            BlockId::OakLeaves => (true, Cube([OakLeaves; 6])),
            BlockId::OakLog => (true, Cube([OakLogTop, OakLogTop, OakLogSide, OakLogSide, OakLogSide, OakLogSide])),
            BlockId::Tallgrass => (true, Cross([Tallgrass, Tallgrass])),
        };

        BlockProperties {
            has_collider: touple.0,
            mesh_representation: touple.1
        }
    }

    pub fn collect_all_texture_names() {
      // TODO: implement
    }
}

use bevy_inspector_egui::egui::panel::TopBottomSide;
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

        Self::get_texture_coordinates()
            .iter()
            .for_each(|(texture_name, (u, v))| {
                if *texture_name != Air {
                    // exclude Air, it is special and used as Placeholder
                    textures.insert(*texture_name, [*u, *v]);
                }
            });

        Self { textures }
    }

    fn get_texture_coordinates() -> Vec<(TextureName, (f32, f32))> {
        const ATLAS_WIDTH: usize = 4;
        const ATLAS_HEIGHT: usize = 4;

        let textures: [[TextureName; ATLAS_WIDTH]; ATLAS_HEIGHT] = [
            [Stone, CobbleStone, GrassTop, OakLeaves],
            [IronOre, Sand, GrassSide, OakLogTop],
            [CoalOre, Bedrock, Dirt, OakLogSide],
            [Tallgrass, Air, Air, Air],
        ];

        let mut texture_positions = Vec::new();

        for x in 0..ATLAS_WIDTH {
            for y in 0..ATLAS_HEIGHT {
                texture_positions.push((
                    *textures.get(y).unwrap().get(x).unwrap(),
                    (1.0 / 4.0 * (x as f32), 1.0 / 4.0 * (y as f32)),
                ))
            }
        }

        texture_positions
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

pub static BLOCKS: [Block; 11] = [
    (Air, [Air; 6], false),
    (Grass, [GrassTop, Dirt, GrassSide, GrassSide, GrassSide, GrassSide], true),
    (Dirt, [Dirt; 6], true),
    (Stone, [Stone; 6], true),
    (CobbleStone, [CobbleStone; 6], true),
    (Bedrock, [Bedrock; 6], true),
    (IronOre, [IronOre; 6], true),
    (CoalOre, [CoalOre; 6], true),
    (OakLeaves, [OakLeaves; 6], true),
    (OakLog, [OakLogTop, OakLogTop, OakLogSide, OakLogSide, OakLogSide, OakLogSide], true),
    (Tallgrass, [Tallgrass; 6], true)
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
