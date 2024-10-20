use crate::prelude::*;

pub const CHUNK_SIZE: usize = 32;
pub const PADDED_CHUNK_SIZE: usize = CHUNK_SIZE + 2;
pub const PADDED_CHUNK_USIZE: usize = PADDED_CHUNK_SIZE;
pub const CHUNK_LENGTH: usize = PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE;

#[derive(Clone)]
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
        self.get_unpadded(x + 1, y + 1, z + 1)
    }

    pub fn get_unpadded(&self, x: usize, y: usize, z: usize) -> BlockId {
        self.data[Self::index(x, y, z)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: BlockId) {
        self.set_unpadded(x + 1, y + 1, z + 1, value);
    }

    pub fn set_unpadded(&mut self, x: usize, y: usize, z: usize, value: BlockId) {
        self.data[Self::index(x, y, z)] = value;
    }

    #[rustfmt::skip]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
      if (x >= PADDED_CHUNK_SIZE) || (y >= PADDED_CHUNK_SIZE) || (z >= PADDED_CHUNK_SIZE) {
        panic!("Index out of bounds: ({}, {}, {})", x, y, z);
      }
        x + PADDED_CHUNK_USIZE * (y + PADDED_CHUNK_USIZE * z)
    }

    pub fn key_eq_pos(key: [i32; 3], position: Vec3) -> bool {
        position.x as i32 == key[0] && position.y as i32 == key[1] && position.z as i32 == key[2]
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}
