use bevy::math::IVec2;
use bevy::math::IVec3;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

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

    pub fn to_world_position(self) -> IVec3 {
        IVec3::new(self.x * CHUNK_SIZE as i32, 0, self.z * CHUNK_SIZE as i32)
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
