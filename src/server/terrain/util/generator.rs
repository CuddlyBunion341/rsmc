use terrain_resources::{Generator, NoiseFunctionParams, TerrainGeneratorParams};

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

        let terrain_height = self.determine_terrain_height(position);

        if (position.y as f64) < terrain_height {
            return BlockId::Stone;
        }

        // let terrain_height = self.sample_2d(
        //     Vec2 {
        //         x: position.x,
        //         y: position.z,
        //     },
        //     default_params,
        // );

        // if (position.y as f64) < terrain_height + 20.0 {
        //     let max_slope = self.calculate_max_slope(position, default_params);
        //     if max_slope > 4.0 {
        //         return BlockId::Stone;
        //     } else if max_slope > 1.0 {
        //         return BlockId::Dirt;
        //     } else {
        //         return BlockId::Grass;
        //     }
        // }

        BlockId::Air
    }

    fn determine_terrain_height(&self, position: Vec3) -> f64 {
        let noise_value = self.sample_2d_normalized(
            Vec2 {
                x: position.x,
                y: position.z,
            },
            &self.params.height_params,
        );

        self.spline_lerp(noise_value)
    }

    fn spline_lerp(&self, x: f64) -> f64 {
        // x is the noise function value
        // y is the mapped terrain height

        let x: f32 = x as f32;

        assert!(self.params.splines.len() > 2);

        let min_x = self.params.splines[0].x;
        let max_x = self.params.splines[self.params.splines.len() - 1].x;

        assert!(min_x == 0.0);
        assert!(max_x == 1.0);

        for i in 0..self.params.splines.len() - 1 {
            let current = self.params.splines[i];
            let next = self.params.splines[i + 1];

            if x >= current.x && x <= next.x {
                return self.lerp(current, x, next);
            }
        }

        panic!("Could not find matching spline points for x value {}", x);
    }

    fn lerp(&self, point0: Vec2, x: f32, point1: Vec2) -> f64 {
        ((point0.y * (point1.x - x) + point1.y * (x - point0.x)) / (point1.x - point0.x)) as f64
    }

    fn get_sample_positions(&self, position: Vec3, epsilon: f32) -> [Vec2; 5] {
        let mut positions = [Vec2::ZERO; 5];

        let Vec3 { x, y: _, z: y } = position;

        positions[0] = Vec2 { x, y };
        positions[1] = Vec2 { x: x + epsilon, y };
        positions[2] = Vec2 { x: x - epsilon, y };
        positions[3] = Vec2 { x, y: y + epsilon };
        positions[4] = Vec2 { x, y: y - epsilon };

        positions
    }

    fn get_terrain_samples(&self, position: Vec3, params: &NoiseFunctionParams) -> [f64; 5] {
        let sample_positions = self.get_sample_positions(position, 0.001);

        sample_positions.map(|sample_position| self.sample_2d(sample_position, params).abs())
    }

    fn calculate_max_slope(&self, position: Vec3, params: &NoiseFunctionParams) -> f64 {
        let samples = self.get_terrain_samples(position, params);

        let mut max = 0.0;

        let main_sample = samples[0];

        for sample in samples.iter().skip(1) {
            if main_sample - sample > max {
                max = main_sample - sample;
            }
        }

        max
    }

    pub fn sample_2d_normalized(&self, position: Vec2, params: &NoiseFunctionParams) -> f64 {
        self.perlin.get([
            position.x as f64 * params.frequency,
            position.y as f64 * params.frequency,
        ]) / 2.0
            + 0.5
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
                Vec2 {
                    x: position.x * params.lacuranity as f32,
                    y: position.y * params.lacuranity as f32,
                },
                &NoiseFunctionParams {
                    octaves: params.octaves - 1,
                    height: params.height + sample,
                    lacuranity: params.lacuranity,
                    frequency: params.frequency,
                    amplitude: params.amplitude,
                    persistence: params.persistence,
                },
            ) * params.persistence
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
}
