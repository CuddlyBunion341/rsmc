use std::collections::HashSet;

use crate::prelude::*;

#[derive(Resource)]
pub struct ChunkManager {
    pub chunks: HashMap<[i32; 3], terrain_util::Chunk>,
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn instantiate_chunks(position: Vec3, render_distance: i32) -> Vec<terrain_util::Chunk> {
        let mut chunks: Vec<terrain_util::Chunk> = Vec::new();

        for x in 0..render_distance {
            for y in 0..render_distance {
                for z in 0..render_distance {
                    let chunk_position = Vec3::new(
                        (x - render_distance / 2) as f32 + position.x,
                        (y - render_distance / 2) as f32 + position.y,
                        (z - render_distance / 2) as f32 + position.z,
                    );
                    let chunk = terrain_util::Chunk::new(chunk_position);
                    chunks.push(chunk);
                }
            }
        }

        chunks
    }

    pub fn insert_chunks(&mut self, chunks: Vec<terrain_util::Chunk>) {
        for chunk in chunks {
            self.chunks
                .insert(Self::position_to_key(chunk.position), chunk);
        }
    }

    pub fn position_to_key(position: Vec3) -> [i32; 3] {
        [position.x as i32, position.y as i32, position.z as i32]
    }

    pub fn set_chunk(&mut self, position: Vec3, chunk: terrain_util::Chunk) {
        let Vec3 { x, y, z } = position;

        self.chunks.insert([x as i32, y as i32, z as i32], chunk);
    }

    pub fn get_chunk(&mut self, position: Vec3) -> Option<&mut terrain_util::Chunk> {
        let Vec3 { x, y, z } = position.floor();

        self.chunks.get_mut(&[x as i32, y as i32, z as i32])
    }

    pub fn set_block(&mut self, position: Vec3, block: BlockId) -> Vec<Vec3> {
        let chunk_positions = Self::get_related_chunk_positions(position);
        let mut positions: Vec<Vec3> = Vec::new();

        for chunk_position in chunk_positions {
            match self.get_chunk(chunk_position) {
                Some(chunk) => {
                    let local_position = (position - chunk_position).floor();
                    chunk.set(
                        local_position.x as usize,
                        local_position.y as usize,
                        local_position.z as usize,
                        block,
                    );
                    positions.push(chunk_position);
                }
                None => {
                    println!("No chunk found for block at {:?}", position);
                }
            }
        }

        positions
    }

    fn get_related_chunk_positions(block_position: Vec3) -> Vec<Vec3> {
        let mut chunk_positions: Vec<Vec3> = Vec::new();

        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    let x = dx as f32;
                    let y = dy as f32;
                    let z = dz as f32;
                    let chunk_position =
                        ((block_position + Vec3::new(x, y, z)) / CHUNK_SIZE as f32).floor();
                    if chunk_positions
                        .iter()
                        .find(|&pos| {
                            pos.x == chunk_position.x
                                && pos.y == chunk_position.y
                                && pos.z == chunk_position.z
                        })
                        .is_none()
                    {
                        chunk_positions.push(chunk_position);
                    }
                }
            }
        }

        println!("Chunk positions: {:?}", chunk_positions);

        chunk_positions
    }

    pub fn get_block(&mut self, position: Vec3) -> Option<BlockId> {
        match self.chunk_from_selection(position) {
            Some(chunk) => {
                let chunk_position = Vec3::new(
                    chunk.position[0] * CHUNK_SIZE as f32,
                    chunk.position[1] * CHUNK_SIZE as f32,
                    chunk.position[2] * CHUNK_SIZE as f32,
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

    fn chunk_from_selection(&mut self, position: Vec3) -> Option<&mut terrain_util::Chunk> {
        let chunk_position = position / CHUNK_SIZE as f32;
        self.get_chunk(chunk_position)
    }
}

#[cfg(test)]
#[test]
fn test_get_related_chunk_positions() {
    let block_position = Vec3::new(0.0, 0.0, 0.0);
    let chunk_positions = ChunkManager::get_related_chunk_positions(block_position);

    assert_eq!(chunk_positions.len(), 8);
    println!("Running test_get_related_chunk_positions");

    let block_position = Vec3::new(30.0, 30.0, 30.0);
    let chunk_positions = ChunkManager::get_related_chunk_positions(block_position);
    assert_eq!(chunk_positions.len(), 1);
}
