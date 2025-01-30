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

        let default_params = GeneratorParams {
            octaves: 4,
            height: 0.0,
            lacuranity: 2.0,
            frequency: 1.0 / 60.0,
            amplitude: 10.0,
            persistence: 0.5,
        };

        let terrain_height = self.sample_2d(
            Vec2 {
                x: position.x,
                y: position.z,
            },
            default_params
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

    fn sample_2d(&self, position: Vec2, params: GeneratorParams) -> f64 {
        let sample = self.perlin.get([
            position.x as f64 * params.frequency,
            position.y as f64 * params.frequency,
        ]) * params.amplitude;

        if params.octaves == 0 {
            return sample;
        }

        sample + self.sample_2d(position, GeneratorParams {
            octaves: params.octaves - 1,
            height: params.height + sample,
            lacuranity: params.lacuranity,
            frequency: params.frequency,
            amplitude: params.amplitude * params.persistence,
            persistence: params.persistence * 0.5,
        })
    }
}

struct GeneratorParams {
    octaves: i32,
    height: f64,
    lacuranity: f64,
    frequency: f64,
    amplitude: f64,
    persistence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_new() {
        let seed = 42;
        let generator = Generator::new(seed);
        assert_eq!(generator.seed, seed);
    }

    #[test]
    fn test_generate_chunk() {
        let seed = 42;
        let generator = Generator::new(seed);
        let mut chunk = Chunk::new(Vec3::new(0.0, 0.0, 0.0));

        generator.generate_chunk(&mut chunk);

        assert_ne!(chunk.get(0, 0, 0), BlockId::Air);
    }

    #[test]
    fn test_sample_3d() {
        let seed = 42;
        let generator = Generator::new(seed);

        let position = Vec3::new(0.0, 0.0, 0.0);
        let density = generator.sample_3d(position, 4);

        assert!((0.0..=1.0).contains(&density));
    }
}
