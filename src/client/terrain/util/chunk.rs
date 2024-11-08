use renet::Bytes;
use serde::{Serialize, Deserialize};
use super::buffer_serializer::{serialize_buffer, deserialize_buffer};
use serde::ser::SerializeStruct;
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

impl Serialize for Chunk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data_as_i32: Vec<i32> = self.data.iter().map(|block_id| *block_id as i32).collect();
        let serialized_data = serialize_buffer(data_as_i32);
        let mut state = serializer.serialize_struct("Chunk", 2)?;
        state.serialize_field("data", &serialized_data)?;
        state.serialize_field("position", &self.position)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Chunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ChunkData {
            data: Vec<Bytes>,
            position: Vec3,
        }

        let ChunkData { data, position } = ChunkData::deserialize(deserializer)?;
        let deserialized_data = deserialize_buffer(data);
        let data_as_block_id: [BlockId; CHUNK_LENGTH] = deserialized_data
            .into_iter()
            .map(|i| BlockId::from(i))
            .collect::<Vec<BlockId>>()
            .try_into()
            .map_err(|_| serde::de::Error::custom("Failed to convert data to BlockId array"))?;
        
        Ok(Chunk {
            data: data_as_block_id,
            position,
        })
    }
}
