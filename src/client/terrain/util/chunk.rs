use crate::prelude::*;

pub const CHUNK_SIZE: i32 = 32;
pub const PADDED_CHUNK_SIZE: i32 = CHUNK_SIZE + 2;
pub const PADDED_CHUNK_USIZE: usize = PADDED_CHUNK_SIZE as usize;
pub const CHUNK_LENGTH: usize =
    (PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE) as usize;

pub struct Chunk {
    pub data: [BlockId; CHUNK_LENGTH],
    pub position: Vec3,
}

impl Chunk {
    pub fn new(position: Vec3) -> Self {
        Self {
            data: [BlockId::Air; CHUNK_LENGTH],
            position,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> BlockId {
        self.data[Self::index(x + 1, y + 1, z + 1)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: BlockId) {
        self.data[Self::index(x + 1, y + 1, z + 1)] = value;
    }

    #[rustfmt::skip]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
      if (x >= PADDED_CHUNK_SIZE as usize) || (y >= PADDED_CHUNK_SIZE as usize) || (z >= PADDED_CHUNK_SIZE as usize) {
        panic!("Index out of bounds: ({}, {}, {})", x, y, z);
      }
        x + PADDED_CHUNK_USIZE * (y + PADDED_CHUNK_USIZE * z)
    }

    pub fn key_eq_pos(key: [i32; 3], position: Vec3) -> bool {
        position.x as i32 == key[0] && position.y as i32 == key[1] && position.z as i32 == key[2]
    }
}
