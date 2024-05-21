use std::collections::HashMap;

use bevy::{ecs::system::Resource, math::Vec3};

use crate::chunk::{Chunk, CHUNK_SIZE};

#[derive(Resource)]
pub struct ChunkManager {
    pub chunks: HashMap<[i32; 3], Chunk>,
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn instantiate_chunks(&self, position: Vec3, render_distance: i32) -> Vec<Chunk> {
        let mut chunks = Vec::new();

        for x in 0..render_distance {
            for z in 0..render_distance {
                let chunk_position = Vec3 {
                    x: x as f32 + position.x - render_distance as f32 / 2.0,
                    y: 0.0,
                    z: z as f32 + position.z - render_distance as f32 / 2.0,
                };

                let chunk = self.get_chunk(chunk_position);
                if chunk.is_none() {
                    chunks.push(Chunk::new(chunk_position));
                }
            }
        }

        chunks
    }

    pub fn block_to_chunk_position(position: Vec3) -> Vec3 {
        Vec3 {
            x: (position.x / CHUNK_SIZE as f32).floor(),
            y: (position.y / CHUNK_SIZE as f32).floor(),
            z: (position.z / CHUNK_SIZE as f32).floor(),
        }
    }

    pub fn get_chunk(&self, position: Vec3) -> Option<&Chunk> {
        let Vec3 { x, y, z } = position.floor();
        self.chunks.get(&[x as i32, y as i32, z as i32])
    }
}
