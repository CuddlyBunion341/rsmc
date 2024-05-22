use crate::mesher::CubeFace;

pub struct Block<'a> {
    pub name: &'a str,
    pub texture_names: [&'a str; 6],
    pub is_solid: bool,
}

pub const BLOCKS: [Block; 5] = [
    Block {
        name: "air",
        texture_names: ["air", "air", "air", "air", "air", "air"],
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
        texture_names: ["dirt", "dirt", "dirt", "dirt", "dirt", "dirt"],
        is_solid: true,
    },
    Block {
        name: "stone",
        texture_names: ["stone", "stone", "stone", "stone", "stone", "stone"],
        is_solid: true,
    },
    Block {
        name: "bedrock",
        texture_names: [
            "bedrock", "bedrock", "bedrock", "bedrock", "bedrock", "bedrock",
        ],
        is_solid: true,
    },
];

pub static AIR: u8 = 0;
pub static GRASS: u8 = 1;
pub static DIRT: u8 = 2;
pub static STONE: u8 = 3;
pub static BEDROCK: u8 = 4;

pub fn get_texture_uvs(texture_name: &str) -> Option<[f32; 2]> {
    match texture_name {
        "grass_top" => Some([0.5, 0.0]),
        "dirt" => Some([0.25, 0.0]),
        "grass_side" => Some([0.75, 0.0]),
        "stone" => Some([0.0, 0.0]),
        "bedrock" => Some([0.0, 0.25]),
        _ => None,
    }
}

pub fn get_block_face_uvs(block_id: u8, face: CubeFace) -> Option<[f32; 2]> {
    let block = &BLOCKS[block_id as usize];
    let texture_name = block.texture_names[face as usize];
    get_texture_uvs(texture_name)
}
