use crate::{
    blocks::{AIR, DIRT, GRASS, STONE},
    chunk::{Chunk, CHUNK_SIZE},
};
use bevy::math::Vec3;
use noise::{NoiseFn, Perlin};

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
        let frequency = 0.03;

        for x in -1..=CHUNK_SIZE {
            for y in -1..=CHUNK_SIZE {
                for z in -1..=CHUNK_SIZE {
                    let local_position = Vec3::new(x as f32, y as f32, z as f32);

                    let density =
                        self.get_layered_3d_noise(4, chunk_origin + local_position) * 10.0;

                    let density_above = self.get_layered_3d_noise(
                        4,
                        chunk_origin + local_position + Vec3::new(0.0, 1.0, 0.0),
                    ) * 10.0;

                    // let height = height.floor() as i32;
                    // let block = Self::block_from_height(y as i32, height);
                    let block = Self::block_from_density(density, density_above);
                    chunk.set(x as usize, y as usize, z as usize, block);
                }
            }
        }
    }

    fn block_from_density(density: f64, density_above: f64) -> u8 {
        if density > 0.0 && density_above <= 0.0 {
            GRASS
        } else if density > 0.5 {
            STONE
        } else if density > 0.0 {
            DIRT
        } else {
            AIR
        }
    }

    fn get_layered_3d_noise(&self, octaves: i32, position: Vec3) -> f64 {
        let mut density = 0.0;
        let mut frequency = 0.03;
        let mut persistence = 0.5;

        for _ in 0..octaves {
            density += self.get_3d_noise(position * frequency) * persistence;
            frequency *= 2.0;
            persistence *= 0.5;
        }

        density
    }

    fn get_3d_noise(&self, position: Vec3) -> f64 {
        self.perlin
            .get([position.x as f64, position.y as f64, position.z as f64])
    }
}
