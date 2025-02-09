use terrain_resources::{Generator, NoiseFunctionParams, TerrainGeneratorParams};

use crate::prelude::*;

macro_rules! for_each_chunk_coordinate {
    ($chunk:expr, $body:expr) => {
        for x in 0..CHUNK_SIZE + 2 {
            for y in 0..CHUNK_SIZE + 2 {
                for z in 0..CHUNK_SIZE + 2 {
                    #[cfg(feature = "skip_chunk_padding")]
                    if x == 0
                        || x == CHUNK_SIZE + 1
                            || y == 0
                            || y == CHUNK_SIZE + 1
                            || z == 0
                            || z == CHUNK_SIZE + 1
                    {
                        continue;
                    }

                    let chunk_origin = $chunk.position * CHUNK_SIZE as f32;
                    let local_position = Vec3::new(x as f32, y as f32, z as f32);
                    let block_position = chunk_origin + local_position;

                    $body(x, y, z, block_position);
                }
            }
        }
    };
}


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
        if chunk.position.y < 0.0 {
            return;
        }

        for_each_chunk_coordinate!(chunk, |x, y, z, block_position| {
            let block = self.generate_block(block_position);
            chunk.set_unpadded(x, y, z, block);
        });

        for_each_chunk_coordinate!(chunk, |x, y, z, _| {
            let block = chunk.get_unpadded(x, y, z);

            if block == BlockId::Stone {

                let x = x as usize;
                let y = y as usize;
                let z = z as usize;

                if Chunk::valid_unpadded(x,y + 1,z) {
                    let top_block = chunk.get_unpadded(x, y + 1, z);
                    if top_block == BlockId::Air {
                        chunk.set_unpadded(x, y, z, BlockId::Grass);
                    }
                }
            }
        });
    }

    fn generate_block(&self, position: Vec3) -> BlockId {
        let terrain_height = self.determine_terrain_height(position);
        let terrain_density = self.determine_terrain_density(position);

        if (position.y as f64) < terrain_height {
            return BlockId::Stone;
        }

        if terrain_density > 0.0 {
            return BlockId::Stone;
        }

        BlockId::Air
    }

    fn determine_terrain_height(&self, position: Vec3) -> f64 {
        let noise_value = self
            .sample_2d(
                Vec2 {
                    x: position.x,
                    y: position.z,
                },
                &self.params.height_params,
            )
            .abs();

        self.spline_lerp(noise_value)
    }

    fn determine_terrain_density(&self, position: Vec3) -> f64 {
        self.sample_3d(position, &self.params.density_params)
    }

    pub fn normalized_spline_terrain_sample(&self, position: Vec2) -> f64 {
        let noise_value = self.sample_2d(position, &self.params.height_params);

        let min_height = self.params.splines[0].y as f64;
        let max_height = self.params.splines[self.params.splines.len() - 1].y as f64;

        let splined_value = self.spline_lerp(noise_value);

        (splined_value - min_height) / (max_height - min_height)
    }

    fn spline_lerp(&self, x: f64) -> f64 {
        let x: f32 = x as f32;

        assert!(self.params.splines.len() >= 2);

        let min_x = self.params.splines[0].x;
        let max_x = self.params.splines[self.params.splines.len() - 1].x;

        assert!(min_x == -1.0);
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

    pub fn sample_2d(&self, position: Vec2, params: &NoiseFunctionParams) -> f64 {
        let mut sample = 0.0;
        let mut frequency = params.frequency;
        let mut weight = 1.0;
        let mut weight_sum = 0.0;

        for _ in 0..params.octaves {
            let new_sample = self
                .perlin
                .get([position.x as f64 * frequency, position.y as f64 * frequency]);

            frequency *= params.lacuranity;
            sample += new_sample * weight;
            weight_sum += weight;
            weight *= params.persistence;
        }

        sample / weight_sum
    }

    pub fn sample_3d(&self, position: Vec3, params: &NoiseFunctionParams) -> f64 {
        let mut sample = 0.0;
        let mut frequency = params.frequency;
        let mut weight = 1.0;
        let mut weight_sum = 0.0;

        for _ in 0..params.octaves {
            let new_sample = self.perlin.get([
                position.x as f64 * frequency,
                position.y as f64 * frequency,
                position.z as f64 * frequency,
            ]);

            frequency *= params.lacuranity;
            sample += new_sample * weight;
            weight_sum += weight;
            weight *= params.persistence;
        }

        sample / weight_sum
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
