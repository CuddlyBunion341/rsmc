pub const CHUNK_SIZE: usize = 32;
pub const PADDED_CHUNK_SIZE: usize = CHUNK_SIZE + 2;

pub struct Chunk {
    pub data: [u8; PADDED_CHUNK_SIZE],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            data: [0; PADDED_CHUNK_SIZE],
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u8 {
        self.data[Self::index(x + 1, y + 1, z + 1)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: u8) {
        self.data[Self::index(x + 1, y + 1, z + 1)] = value;
    }

    pub fn index(x: usize, y: usize, z: usize) -> usize {
        x * PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE + y * PADDED_CHUNK_SIZE + z
    }
}
