use crate::mesher::CubeFace;

pub struct Block<'a> {
    pub name: &'a str,
    pub texture_names: [&'a str; 6],
    pub is_solid: bool,
}

pub const BLOCKS: [Block; 14] = [
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
    Block {
        name: "red_sand",
        texture_names: [
            "red_sand", "red_sand", "red_sand", "red_sand", "red_sand", "red_sand",
        ],
        is_solid: true,
    },
    Block {
        name: "brown_terracotta",
        texture_names: [
            "brown_terracotta",
            "brown_terracotta",
            "brown_terracotta",
            "brown_terracotta",
            "brown_terracotta",
            "brown_terracotta",
        ],
        is_solid: true,
    },
    Block {
        name: "cyan_terracotta",
        texture_names: [
            "cyan_terracotta",
            "cyan_terracotta",
            "cyan_terracotta",
            "cyan_terracotta",
            "cyan_terracotta",
            "cyan_terracotta",
        ],
        is_solid: true,
    },
    Block {
        name: "gray_terracotta",
        texture_names: [
            "gray_terracotta",
            "gray_terracotta",
            "gray_terracotta",
            "gray_terracotta",
            "gray_terracotta",
            "gray_terracotta",
        ],
        is_solid: true,
    },
    Block {
        name: "light_gray_terracotta",
        texture_names: [
            "light_gray_terracotta",
            "light_gray_terracotta",
            "light_gray_terracotta",
            "light_gray_terracotta",
            "light_gray_terracotta",
            "light_gray_terracotta",
        ],
        is_solid: true,
    },
    Block {
        name: "orange_terracotta",
        texture_names: [
            "orange_terracotta",
            "orange_terracotta",
            "orange_terracotta",
            "orange_terracotta",
            "orange_terracotta",
            "orange_terracotta",
        ],
        is_solid: true,
    },
    Block {
        name: "red_terracotta",
        texture_names: [
            "red_terracotta",
            "red_terracotta",
            "red_terracotta",
            "red_terracotta",
            "red_terracotta",
            "red_terracotta",
        ],
        is_solid: true,
    },
    Block {
        name: "terracotta",
        texture_names: [
            "terracotta",
            "terracotta",
            "terracotta",
            "terracotta",
            "terracotta",
            "terracotta",
        ],
        is_solid: true,
    },
    Block {
        name: "yellow_terracotta",
        texture_names: [
            "yellow_terracotta",
            "yellow_terracotta",
            "yellow_terracotta",
            "yellow_terracotta",
            "yellow_terracotta",
            "yellow_terracotta",
        ],
        is_solid: true,
    },
];

pub static AIR: u8 = 0;
pub static GRASS: u8 = 1;
pub static DIRT: u8 = 2;
pub static STONE: u8 = 3;
pub static BEDROCK: u8 = 4;
pub static RED_SAND: u8 = 5;
pub static BROWN_TERRACOTTA: u8 = 6;
pub static CYAN_TERRACOTTA: u8 = 7;
pub static GRAY_TERRACOTTA: u8 = 8;
pub static LIGHT_GRAY_TERRACOTTA: u8 = 9;
pub static ORANGE_TERRACOTTA: u8 = 10;
pub static RED_TERRACOTTA: u8 = 11;
pub static TERRACOTTA: u8 = 12;
pub static YELLOW_TERRACOTTA: u8 = 13;

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

pub fn get_block_face_uvs(block_id: u8, face: CubeFace) -> Option<[f32; 2]> {
    let block = &BLOCKS[block_id as usize];
    let texture_name = block.texture_names[face as usize];
    get_texture_uvs(texture_name)
}
