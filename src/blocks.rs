use crate::mesher::CubeFace;

pub struct Block {
    pub name: &'static str,
    pub texture_names: [&'static str; 6],
    pub is_solid: bool,
}

pub const BLOCKS: [Block; 14] = [
    Block {
        name: "air",
        texture_names: ["air"; 6],
        is_solid: false,
    },
    Block {
        name: "grass",
        texture_names: [
            "grass_top",
            "dirt",
            "grass_side",
            "grass_side",
            "grass_side",
            "grass_side",
        ],
        is_solid: true,
    },
    Block {
        name: "dirt",
        texture_names: ["dirt"; 6],
        is_solid: true,
    },
    Block {
        name: "stone",
        texture_names: ["stone"; 6],
        is_solid: true,
    },
    Block {
        name: "bedrock",
        texture_names: ["bedrock"; 6],
        is_solid: true,
    },
    Block {
        name: "red_sand",
        texture_names: ["red_sand"; 6],
        is_solid: true,
    },
    Block {
        name: "brown_terracotta",
        texture_names: ["brown_terracotta"; 6],
        is_solid: true,
    },
    Block {
        name: "cyan_terracotta",
        texture_names: ["cyan_terracotta"; 6],
        is_solid: true,
    },
    Block {
        name: "gray_terracotta",
        texture_names: ["gray_terracotta"; 6],
        is_solid: true,
    },
    Block {
        name: "light_gray_terracotta",
        texture_names: ["light_gray_terracotta"; 6],
        is_solid: true,
    },
    Block {
        name: "orange_terracotta",
        texture_names: ["orange_terracotta"; 6],
        is_solid: true,
    },
    Block {
        name: "red_terracotta",
        texture_names: ["red_terracotta"; 6],
        is_solid: true,
    },
    Block {
        name: "terracotta",
        texture_names: ["terracotta"; 6],
        is_solid: true,
    },
    Block {
        name: "yellow_terracotta",
        texture_names: ["yellow_terracotta"; 6],
        is_solid: true,
    },
];

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
