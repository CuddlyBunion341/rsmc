use crate::prelude::*;

pub struct Generator {
    pub seed: u32,
    perlin: Perlin,
}

impl Generator {
    pub fn new(seed: u32) -> Generator {
        Generator {
            seed,
            perlin: Perlin::new(seed),
        }
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk) {
        let chunk_origin = chunk.position * CHUNK_SIZE as f32;
        if chunk_origin.y < 0.0 {
            return;
        }

        for x in -1..=CHUNK_SIZE {
            for y in -1..=CHUNK_SIZE {
                for z in -1..=CHUNK_SIZE {
                    let local_position = Vec3::new(x as f32, y as f32, z as f32);
                    let block_position = chunk_origin + local_position;
                    let block = self.generate_block(block_position);
                    chunk.set(x as usize, y as usize, z as usize, block);
                }
            }
        }
    }

    fn generate_block(&self, position: Vec3) -> BlockId {
        let base_height = -50.0;

        let mut density = self.sample_3d(
            Vec3 {
                x: position.x,
                y: position.y + base_height,
                z: position.z,
            },
            4,
        );
        density -= position.y as f64 * 0.02;
        if density > 0.7 {
            BlockId::Stone
        } else if density > 0.40 {
            BlockId::Dirt
        } else if density > 0.0 {
            BlockId::Grass
        } else {
            BlockId::Air
        }
    }

    fn sample_3d(&self, position: Vec3, octaves: i32) -> f64 {
        let mut density = 0.0;
        let lacuranity = 2.0;
        let mut frequency = 0.04;
        let mut amplitude = 1.0;
        let mut persistence = 0.5;

        for _ in 0..octaves {
            density += self.perlin.get([
                position.x as f64 * frequency,
                position.y as f64 * frequency,
                position.z as f64 * frequency,
            ]) * amplitude;

            amplitude *= persistence;
            frequency *= lacuranity;
            persistence *= 0.5;
        }

        density
    }
}
