#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BlockId {
    Air = 0,
    Grass = 1,
    Dirt = 2,
    Stone = 3,
    CobbleStone = 4,
    Bedrock = 5,
    IronOre = 6,
    CoalOre = 7,
    OakLeaves = 8,
    OakLog = 9,
    Tallgrass = 10
}

impl BlockId {
    pub fn from_u8(index: u8) -> BlockId {
        use BlockId::*;
        match index {
            0 => Air,
            1 => Grass,
            2 => Dirt,
            3 => Stone,
            4 => CobbleStone,
            5 => Bedrock,
            6 => IronOre,
            7 => CoalOre,
            8 => OakLeaves,
            9 => OakLog,
            10 => Tallgrass,
            _ => panic!("Invalid block id")
        }
    }
}
