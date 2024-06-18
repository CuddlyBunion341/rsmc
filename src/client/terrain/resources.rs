use std::collections::HashMap;

use bevy::{ecs::system::Resource, math::Vec3};

use super::{
    blocks::BlockId,
    chunk::{self, Chunk, CHUNK_SIZE},
};

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
            for y in 0..render_distance {
                for z in 0..render_distance {
                    let chunk_position = Vec3::new(
                        (x - render_distance / 2) as f32 + position.x,
                        (y - render_distance / 2) as f32 + position.y,
                        (z - render_distance / 2) as f32 + position.z,
                    );
                    let chunk = Chunk::new(chunk_position);
                    chunks.push(chunk);
                }
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

    pub fn set_block(&mut self, position: Vec3, block: BlockId) {
        match self.chunk_from_selection(position) {
            Some(chunk) => {
                let chunk_position = Vec3::new(
                    chunk.position[0] * chunk::CHUNK_SIZE as f32,
                    chunk.position[1] * chunk::CHUNK_SIZE as f32,
                    chunk.position[2] * chunk::CHUNK_SIZE as f32,
                );
                let local_position = (position - chunk_position).floor();
                chunk.set(
                    local_position.x as usize,
                    local_position.y as usize,
                    local_position.z as usize,
                    block,
                );
            }
            None => {
                println!("No chunk found");
            }
        }
    }

    pub fn get_block(&mut self, position: Vec3) -> Option<BlockId> {
        match self.chunk_from_selection(position) {
            Some(chunk) => {
                let chunk_position = Vec3::new(
                    chunk.position[0] * chunk::CHUNK_SIZE as f32,
                    chunk.position[1] * chunk::CHUNK_SIZE as f32,
                    chunk.position[2] * chunk::CHUNK_SIZE as f32,
                );
                let local_position = (position - chunk_position).floor();
                Some(chunk.get(
                    local_position.x as usize,
                    local_position.y as usize,
                    local_position.z as usize,
                ))
            }
            None => {
                println!("No chunk found for block at {:?}", position);
                None
            }
        }
    }

    fn chunk_from_selection(
        &mut self,
        position: Vec3,
    ) -> Option<&mut chunk::Chunk> {
        let chunk_position = position / CHUNK_SIZE as f32;
        self.get_chunk(chunk_position)
    }
}
