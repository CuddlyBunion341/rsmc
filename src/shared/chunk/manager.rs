use std::collections::HashMap;

use bevy::{log::info, math::IVec2, math::IVec3, prelude::Resource};

use crate::*;

#[derive(Resource)]
pub struct ChunkManager {
    pub chunks: HashMap<IVec2, Chunk>,
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

    pub fn with_chunks(chunks: Vec<Chunk>) -> Self {
        let chunks: HashMap<IVec2, Chunk> = chunks
            .into_iter()
            .map(|chunk| (chunk.position, chunk))
            .collect();

        Self { chunks }
    }

    pub fn instantiate_chunks(position: IVec2, render_distance: IVec2) -> Vec<Chunk> {
        let render_distance_x = render_distance[0];
        let render_distance_z = render_distance[1];

        let mut chunks: Vec<Chunk> = Vec::new();

        for x in -render_distance_x..render_distance_x {
            for z in -render_distance_z..render_distance_z {
                let chunk_position = IVec2::new(x + position[0], z + position[1]);
                let chunk = Chunk::new(chunk_position);
                chunks.push(chunk);
            }
        }

        chunks
    }

    pub fn sorted_new_chunk_positions(&self, origin: IVec2, distance: IVec2) -> Vec<IVec2> {
        let all_positions = Self::get_sorted_chunk_positions_in_range(origin, distance);
        all_positions
            .into_iter()
            .filter(|position| self.get_chunk(position).is_none())
            .collect()
    }

    pub fn get_sorted_chunk_positions_in_range(origin: IVec2, distance: IVec2) -> Vec<IVec2> {
        let distance_x = distance[0];
        let distance_z = distance[1];

        let mut positions: Vec<IVec2> =
            Vec::with_capacity(((distance_x * 2 + 1) * (distance_z * 2 + 1)) as usize);

        for x in -distance_x..=distance_x {
            for z in -distance_z..=distance_z {
                let chunk_position = IVec2::new(x + origin[0], z + origin[1]);
                positions.push(chunk_position);
            }
        }

        positions.sort_by(|a, b| {
            (a - origin)
                .length_squared()
                .cmp(&(b - origin).length_squared())
        });

        positions
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(chunk.position, chunk);
    }

    pub fn insert_chunks(&mut self, chunks: Vec<Chunk>) {
        for chunk in chunks {
            self.insert_chunk(chunk);
        }
    }

    pub fn set_chunk(&mut self, position: IVec2, chunk: Chunk) {
        self.chunks.insert(position, chunk);
    }

    pub fn get_chunk(&self, position: &IVec2) -> Option<&Chunk> {
        self.chunks.get(position)
    }

    pub fn has_chunk(&self, position: &IVec2) -> bool {
        self.chunks.contains_key(position)
    }

    pub fn get_chunk_mut(&mut self, position: &IVec2) -> Option<&mut Chunk> {
        self.chunks.get_mut(position)
    }

    pub fn inside_world(position: &IVec3) -> bool {
        position.y >= 0 && position.y < CHUNK_HEIGHT as i32
    }

    pub fn update_block(&mut self, position: IVec3, block: BlockId) -> Vec<IVec2> {
        Self::chunk_positions_containing_world_pos(position)
            .iter()
            .flat_map(|chunk_position| {
                let chunk_option =
                    self.get_chunk_mut(&IVec2::new(chunk_position[0], chunk_position[1]));
                match chunk_option {
                    Some(chunk) => {
                        let chunk_origin = *chunk_position * CHUNK_SIZE as i32;
                        let local_position =
                            position - IVec3::new(chunk_origin[0], 0, chunk_origin[1]);

                        info!("Performing local update at {:?}", local_position);

                        assert!(local_position.x >= -1 && local_position.x <= CHUNK_SIZE as i32);
                        assert!(local_position.y >= 0 && local_position.y < CHUNK_HEIGHT as i32);
                        assert!(local_position.z >= -1 && local_position.z <= CHUNK_SIZE as i32);

                        chunk.update(local_position.x, local_position.y, local_position.z, block);

                        Some(*chunk_position)
                    }
                    None => {
                        // FIXME: we should do something about updates in unloaded chunks..
                        None
                    }
                }
            })
            .collect()
    }

    pub fn get_block(&self, position: IVec3) -> Option<BlockId> {
        match self.chunk_at_position(position) {
            Some(chunk) => {
                let chunk_position = IVec3::new(
                    chunk.position[0] * CHUNK_SIZE as i32,
                    0,
                    chunk.position[1] * CHUNK_SIZE as i32,
                );
                let local_position = position - chunk_position;
                chunk.get_safe(local_position.x, local_position.y, local_position.z)
            }
            None => {
                // println!("No chunk found for block at {:?}", position);
                None
            }
        }
    }

    fn chunk_positions_containing_world_pos(position: IVec3) -> Vec<IVec2> {
        fn axis_chunks(world: i32) -> Vec<i32> {
            let size = CHUNK_SIZE as i32;
            let base = world.div_euclid(size);

            let candidates = [base - 1, base, base + 1];

            candidates
                .into_iter()
                .filter(|&chunk| {
                    let start = chunk * size - 1;
                    let end = chunk * size + size;
                    world >= start && world <= end
                })
                .collect()
        }

        let xs = axis_chunks(position.x);
        let zs = axis_chunks(position.z);

        let mut out = Vec::new();

        for x in xs {
            for z in &zs {
                out.push(IVec2::new(x, *z));
            }
        }

        out
    }

    pub fn world_position_to_chunk_position(world_position: IVec3) -> IVec2 {
        IVec2::new(
            world_position.x.div_euclid(CHUNK_SIZE as i32),
            world_position.z.div_euclid(CHUNK_SIZE as i32),
        )
    }

    fn chunk_at_position(&self, world_position: IVec3) -> Option<&Chunk> {
        let chunk_position = Self::world_position_to_chunk_position(world_position);
        self.get_chunk(&chunk_position)
    }

    pub fn get_all_chunk_positions(&self) -> Vec<IVec2> {
        self.chunks.keys().copied().collect()
    }

    pub fn all_chunks(&self) -> Vec<&Chunk> {
        self.chunks.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // For performance reasons chunk copy one block on each side form their neighbor.
    // When updating a block, multiple chunks may be involved.
    // With chunk width = 30:
    //
    // Example chunk x = 0:
    //  -1    0    1
    // [-1][0,29][30]
    //
    //   -2     -1    0
    // [-31][-30, -1][0]
    //
    // The position 0 is included in the padding of chunk -1, as well as in the main area of chuk 0
    #[test]
    fn test_chunk_positions_containing_world_pos() {
        assert_eq!(
            ChunkManager::chunk_positions_containing_world_pos(IVec3::ZERO),
            vec![
                IVec2::new(-1, -1),
                IVec2::new(-1, 0),
                IVec2::new(0, -1),
                IVec2::new(0, 0)
            ]
        );

        assert_eq!(
            ChunkManager::chunk_positions_containing_world_pos(IVec3::ONE),
            vec![IVec2::new(0, 0),]
        );

        assert_eq!(
            ChunkManager::chunk_positions_containing_world_pos(IVec3::new(0, 1, 1)),
            vec![IVec2::new(-1, 0), IVec2::new(0, 0),]
        );

        assert_eq!(
            ChunkManager::chunk_positions_containing_world_pos(IVec3::new(CHUNK_SIZE as i32, 1, 1)),
            vec![IVec2::new(0, 0), IVec2::new(1, 0),]
        );
    }

    #[test]
    fn test_chunk_manager_new() {
        let chunk_manager = ChunkManager::new();
        assert!(chunk_manager.chunks.is_empty());
    }

    #[test]
    fn test_instantiate_chunks() {
        let position = IVec2::new(0, 0);

        let width = 2;
        let depth = 4;

        let render_distance = IVec2::new(width, depth);

        let chunks = ChunkManager::instantiate_chunks(position, render_distance);
        assert_eq!(chunks.len(), (2 * width * 2 * depth) as usize,);
    }

    #[test]
    fn test_insert_chunks() {
        let mut chunk_manager = ChunkManager::new();
        let position = IVec2::new(0, 0);
        let render_distance = 2;
        let chunks = ChunkManager::instantiate_chunks(
            position,
            IVec2::new(render_distance, render_distance),
        );

        let render_diameter = render_distance * 2;

        chunk_manager.insert_chunks(chunks);
        assert_eq!(
            chunk_manager.chunks.len(),
            (render_diameter * render_diameter) as usize
        );
    }

    #[test]
    fn test_set_and_get_chunk_mut() {
        let mut chunk_manager = ChunkManager::new();
        let position = IVec2::ZERO;
        let chunk = Chunk::new(position);

        chunk_manager.set_chunk(position, chunk.clone());
        let retrieved_chunk = chunk_manager.get_chunk_mut(&position).unwrap();
        assert_eq!(retrieved_chunk.position, chunk.position);
    }

    #[test]
    fn test_set_and_get_block() {
        let mut chunk_manager = ChunkManager::new();
        let position = IVec2::ZERO;
        let chunk = Chunk::new(position);

        chunk_manager.set_chunk(position, chunk);
        let block_position = IVec3::ONE;
        let block_id = BlockId::Stone;

        chunk_manager.update_block(block_position, block_id);
        let retrieved_block = chunk_manager.get_block(block_position).unwrap();
        assert_eq!(retrieved_block, block_id);
    }

    #[test]
    fn test_get_all_chunk_positions() {
        let mut chunk_manager = ChunkManager::new();
        chunk_manager.set_chunk(IVec2::new(0, 0), Chunk::default());
        chunk_manager.set_chunk(IVec2::new(2, 0), Chunk::default());
        chunk_manager.set_chunk(IVec2::new(1, 3), Chunk::default());

        let retrieved_chunk_positions = chunk_manager.get_all_chunk_positions();
        assert_eq!(retrieved_chunk_positions.len(), 3);
    }

    #[test]
    #[rustfmt::skip]
    fn test_tallgrass_update() {
        let mut chunk_manager = ChunkManager::new();
        let chunk_position = IVec2::ZERO;
        let chunk = Chunk::new(chunk_position);
        chunk_manager.set_chunk(chunk_position, chunk);

        let grass_position = IVec3::new(0, 0, 0);
        let tallgrass_position = IVec3::new(0, 1, 0);

        chunk_manager.update_block(grass_position, BlockId::Grass);
        assert_eq!(chunk_manager.get_block(grass_position).unwrap(), BlockId::Grass);
        chunk_manager.update_block(tallgrass_position, BlockId::Tallgrass);
        assert_eq!(chunk_manager.get_block(tallgrass_position).unwrap(), BlockId::Tallgrass);

        chunk_manager.update_block(grass_position, BlockId::Dirt);
        assert_eq!(chunk_manager.get_block(grass_position).unwrap(), BlockId::Dirt);
        assert_eq!(chunk_manager.get_block(tallgrass_position).unwrap(), BlockId::Tallgrass);

        chunk_manager.update_block(grass_position, BlockId::Air);
        assert_eq!(chunk_manager.get_block(grass_position).unwrap(), BlockId::Air);
        assert_eq!(chunk_manager.get_block(tallgrass_position).unwrap(), BlockId::Air);
    }
}
