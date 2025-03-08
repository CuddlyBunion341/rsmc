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

mod client_block {
    use rsmc::BlockId;

    use super::TextureName;
    use super::TextureName::*;
    use BlockId::*;

    pub enum MeshRepresentation {
        None,
        Cube([TextureName; 6]),
        Cross([TextureName; 2])
    }

    use MeshRepresentation::*;

    pub struct BlockProperties {
        pub has_collider: bool,
        pub mesh_representation: MeshRepresentation
    }

    impl BlockProperties {
        pub fn new(has_collider: bool, mesh_representation: MeshRepresentation) {
            BlockProperties {has_collider, mesh_representation}
        } 
    } 

    pub fn block_properties(block_id: BlockId) -> BlockProperties {
        let touple = match self {
            Air => (true, None()),
            Grass => (true, Cube([GrassTop, Dirt, GrassSide, GrassSide, GrassSide, GrassSide])),
            Dirt => (true, Cube([Dirt; 6])),
            Stone => (true, Cube([Stone; 6])),
            CobbleStone => (true, Cube([CobbleStone; 6])),
            Bedrock => (true, Cube([Bedrock; 6])),
            IronOre => (true, Cube([IronOre; 6])),
            CoalOre => (true, Cube([CoalOre; 6])),
            OakLeaves => (true, Cube([OakLeaves; 6])),
            OakLog => (true, Cube([OakLogTop, OakLogTop, OakLogSide, OakLogSide, OakLogSide, OakLogSide])),
            Tallgrass => (true, Cross([Tallgrass, Tallgrass])),
        };

        BlockProperties {
            has_collider: touple.0,
            mesh_representation: touple.1
        }
    }

    pub fn collect_all_texture_names() -> Vec<TextureName> {
        BlockId::values().iter().map(|block_id| {
            let properties = block_properties(block_id);
            let mesh: MeshRepresentation = properties.mesh_representation;

            match mesh {
                MeshRepresentation::None => vec![],
                MeshRepresentation::Cube(textures) => Vec::from(textures),
                MeshRepresentation::Cross(textures) => Vec::from(textures),            
            }
        }).flatten()
    }
}

use bevy_inspector_egui::egui::panel::TopBottomSide;
use client_block::block_properties;
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

type TextureUV = [f32; 2];

impl Block {
    pub fn get_block_face_uvs(
        block_id: BlockId,
        face: CubeFace,
        texture_manager: &TextureManager,
    ) -> Option<[f32; 2]> {
        let properties = block_properties(block_id);
        let mesh = properties.mesh_representation;

        let texture_option: Option<TextureName> = match mesh {
            client_block::MeshRepresentation::None => None(),
            client_block::MeshRepresentation::Cube(textures) => Some(textures[face as usize]),
            client_block::MeshRepresentation::Cross(textures) => Some(textures[face as usize])
        };

        match texture_option {
            Some(texture_name) => Some(texture_manager.get_texture_uv(texture_name).copied()),
            None() => None()
        }
    }
}
