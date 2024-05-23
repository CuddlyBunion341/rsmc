use std::collections::HashMap;

use bevy::{ecs::system::Resource, math::Vec3};

use crate::chunk::Chunk;

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

    pub fn instantiate_chunks(position: Vec3, render_distance: i32) -> Vec<Chunk> {
        let mut chunks: Vec<Chunk> = Vec::new();

        for x in 0..render_distance {
            for z in 0..render_distance {
                let chunk_position = Vec3::new(
                    (x - render_distance / 2) as f32 + position.x,
                    0.0,
                    (z - render_distance / 2) as f32 + position.z,
                );
                let chunk = Chunk::new(chunk_position);
                chunks.push(chunk);
            }
        }

        chunks
    }

    pub fn insert_chunks(&mut self, chunks: Vec<Chunk>) {
        for chunk in chunks {
            self.chunks
                .insert(Self::position_to_key(chunk.position), chunk);
        }
    }

    pub fn position_to_key(position: Vec3) -> [i32; 3] {
        [position.x as i32, position.y as i32, position.z as i32]
    }

    pub fn set_chunk(&mut self, position: Vec3, chunk: Chunk) {
        let Vec3 { x, y, z } = position;

        self.chunks.insert([x as i32, y as i32, z as i32], chunk);
    }

    pub fn get_chunk(&mut self, position: Vec3) -> Option<&mut Chunk> {
        let Vec3 { x, y, z } = position.floor();

        self.chunks.get_mut(&[x as i32, y as i32, z as i32])
    }
}
