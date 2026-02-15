use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::math::IVec2;
use bevy::math::IVec3;
use serde::Deserialize;
use serde::Serialize;

use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

impl std::ops::Add for ChunkPosition {
    type Output = ChunkPosition;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for ChunkPosition {
    type Output = ChunkPosition;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            z: self.z - rhs.z,
        }
    }
}

impl ChunkPosition {
    pub const ZERO: Self = Self { x: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, z: 1 };

    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn as_vec2(self) -> bevy::math::Vec2 {
        bevy::math::Vec2::new(self.x as f32, self.z as f32)
    }

    pub fn as_ivec3(self) -> IVec3 {
        IVec3::new(self.x, 0, self.z)
    }
}

impl std::ops::Index<usize> for ChunkPosition {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.z,
            _ => panic!("Index out of bounds for ChunkPosition: {}", index),
        }
    }
}

impl std::ops::Mul<i32> for ChunkPosition {
    type Output = ChunkPosition;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            z: self.z * rhs,
        }
    }
}

impl From<IVec2> for ChunkPosition {
    fn from(v: IVec2) -> Self {
        Self { x: v.x, z: v.y }
    }
}

impl From<ChunkPosition> for IVec2 {
    fn from(p: ChunkPosition) -> Self {
        IVec2::new(p.x, p.z)
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub data: Box<[BlockId; CHUNK_LENGTH]>,
    pub position: ChunkPosition,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new(ChunkPosition::ZERO)
    }
}

impl Chunk {
    pub fn new(position: ChunkPosition) -> Self {
        Self {
            position,
            data: Box::new([BlockId::Air; CHUNK_LENGTH]),
        }
    }

    pub fn valid_local(x: usize, y: usize, z: usize) -> bool {
        x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE
    }

    pub fn is_within_padded_bounds(x: i32, y: i32, z: i32) -> bool {
        x >= -1
            && y >= 0
            && z >= -1
            && x <= CHUNK_SIZE as i32
            && y < CHUNK_HEIGHT as i32
            && z <= CHUNK_SIZE as i32
    }

    pub fn valid_unpadded(x: usize, y: usize, z: usize) -> bool {
        x < PADDED_CHUNK_SIZE && y < CHUNK_HEIGHT && z < PADDED_CHUNK_SIZE
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> BlockId {
        assert!(Self::is_within_padded_bounds(x, y, z));
        self.get_unpadded((x + 1) as usize, y as usize, (z + 1) as usize)
    }

    pub fn get_safe(&self, x: i32, y: i32, z: i32) -> Option<BlockId> {
        if Self::is_within_padded_bounds(x, y, z) {
            Some(self.get_unpadded((x + 1) as usize, y as usize, (z + 1) as usize))
        } else {
            None
        }
    }

    pub fn get_unpadded(&self, x: usize, y: usize, z: usize) -> BlockId {
        self.data[Self::index(x, y, z)]
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, value: BlockId) {
        assert!(Self::is_within_padded_bounds(x, y, z));
        self.set_unpadded((x + 1) as usize, y as usize, (z + 1) as usize, value);
    }

    pub fn update(&mut self, x: i32, y: i32, z: i32, value: BlockId) {
        self.set(x, y, z, value);

        if !value.supports_grass()
            && Self::is_within_padded_bounds(x, y + 1, z)
            && self.get(x, y + 1, z) == BlockId::Tallgrass
        {
            self.set(x, y + 1, z, BlockId::Air);
        }
    }

    pub fn set_unpadded(&mut self, x: usize, y: usize, z: usize, value: BlockId) {
        self.data[Self::index(x, y, z)] = value;
    }

    #[rustfmt::skip]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
        let n  = PADDED_CHUNK_SIZE;
        let h = CHUNK_HEIGHT;
        assert!(x <  n && y < h && z < n, "Index out of bounds: ({}, {}, {})", x,y,z);

        z + n * (y + h * x)
    }

    pub fn key_eq_pos(key: [i32; 3], position: IVec3) -> bool {
        position.x == key[0] && position.y == key[1] && position.z == key[2]
    }

    pub fn rng_seed(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.position.hash(&mut s);
        s.finish()
    }
}
