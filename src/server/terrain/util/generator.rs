use terrain_resources::{Generator, TerrainGeneratorParams};

use crate::prelude::*;

impl Generator {
    pub fn new(seed: u32) -> Generator {
        Self::new_with_params(seed, TerrainGeneratorParams::default())
    }

    pub fn new_with_params(seed: u32, params: TerrainGeneratorParams) -> Generator {
        Generator {
            seed,
            perlin: Perlin::new(seed),
            params,
        }
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk) {
        let chunk_origin = chunk.position * CHUNK_SIZE as f32;
        if chunk_origin.y < 0.0 {
            return;
        }

        for x in 0..CHUNK_SIZE + 2 {
            for y in 0..CHUNK_SIZE + 2 {
                for z in 0..CHUNK_SIZE + 2 {
                    let local_position = Vec3::new(x as f32, y as f32, z as f32);
                    let block_position = chunk_origin + local_position;
                    let block = self.generate_block(block_position);
                    chunk.set_unpadded(x, y, z, block);
                }
            }
        }
    }

    fn generate_block(&self, position: Vec3) -> BlockId {
        let default_params = &self.params.height_params;

        let terrain_height = self.sample_2d(
            Vec2 {
                x: position.x,
                y: position.z,
            },
            default_params,
        );

        if (position.y as f64) < terrain_height + 20.0 {
            return BlockId::Stone;
        }

        // density -= position.y as f64 * 0.02;
        //
        // let base_block = BlockId::Stone;

        // if density > 0.7 {
        //     BlockId::Stone
        // } else if density > 0.40 {
        //     BlockId::Dirt
        // } else if density > 0.0 {
        //     if self.generate_block(position + Vec3::new(0.0, 1.0, 0.0)) == BlockId::Air {
        //         BlockId::Grass
        //     } else {
        //         BlockId::Dirt
        //     }
        // } else {
        //     BlockId::Air
        // }

        BlockId::Air
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

    pub fn sample_2d(&self, position: Vec2, params: &NoiseFunctionParams) -> f64 {
        let sample = self.perlin.get([
            position.x as f64 * params.frequency,
            position.y as f64 * params.frequency,
        ]) * params.amplitude;

        if params.octaves == 0 {
            return sample;
        }

        sample
            + self.sample_2d(
                position,
                &NoiseFunctionParams {
                    octaves: params.octaves - 1,
                    height: params.height + sample,
                    lacuranity: params.lacuranity,
                    frequency: params.frequency,
                    amplitude: params.amplitude * params.persistence,
                    persistence: params.persistence * 0.5,
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use terrain_resources::Generator;

    #[test]
    fn test_generate_chunk() {
        let generator = Generator::default();
        let mut chunk = Chunk::new(Vec3::new(0.0, 0.0, 0.0));

        generator.generate_chunk(&mut chunk);

        assert_ne!(chunk.get(0, 0, 0), BlockId::Air);
    }

    #[test]
    fn test_sample_3d() {
        let generator = Generator::default();

        let position = Vec3::new(0.0, 0.0, 0.0);
        let density = generator.sample_3d(position, 4);

        assert!((0.0..=1.0).contains(&density));
    }
}
