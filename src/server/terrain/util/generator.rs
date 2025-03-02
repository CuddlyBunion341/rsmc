use terrain_resources::{Generator, NoiseFunctionParams, TerrainGeneratorParams};

use rand::prelude::*;

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
                    let world_position = chunk_origin + local_position;

                    $body(x, y, z, world_position);
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

        for_each_chunk_coordinate!(chunk, |x, y, z, world_position| {
            let block = self.generate_block(world_position);
            chunk.set_unpadded(x, y, z, block);
        });

        for_each_chunk_coordinate!(chunk, |x, y, z, _| {
            let pos = Vec3 {
                x: x as f32,
                y: y as f32,
                z: z as f32,
            };

            let block = self.decorate_block(chunk, pos);
            chunk.set_unpadded(x, y, z, block);
        });

        for _ in 0..150 {
            self.attempt_spawn_tree(chunk);
        }
    }

    fn attempt_spawn_tree(&self, chunk: &mut Chunk) {
        let proposal = Self::propose_tree_blocks();

        struct Bounds {
            min: Vec3,
            max: Vec3,
        }

        let proposal_bounds = proposal.iter().fold(
            Bounds {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            },
            |bounds, (relative_pos, _block_id)| Bounds {
                min: Vec3 {
                    x: bounds.min.x.min(relative_pos.x),
                    y: bounds.min.y.min(relative_pos.y),
                    z: bounds.min.z.min(relative_pos.z),
                },
                max: Vec3 {
                    x: bounds.max.x.max(relative_pos.x),
                    y: bounds.max.y.max(relative_pos.y),
                    z: bounds.max.z.max(relative_pos.z),
                },
            },
        );

        let sapling_x: usize = rand::random_range(
            proposal_bounds.min.x.abs()..(CHUNK_SIZE as f32 - proposal_bounds.max.x),
        ) as usize;
        let sapling_y: usize = rand::random_range(
            proposal_bounds.min.y.abs()..(CHUNK_SIZE as f32 - proposal_bounds.max.y),
        ) as usize;
        let sapling_z: usize = rand::random_range(
            proposal_bounds.min.z.abs()..(CHUNK_SIZE as f32 - proposal_bounds.max.z),
        ) as usize;

        if chunk.get(sapling_x, sapling_y, sapling_z) != BlockId::Grass {
            return;
        }

        let proposal_valid = proposal.iter().all(|(relative_pos, _block)| {
            let Vec3 { x, y, z } = relative_pos;
            Chunk::valid_padded(
                (sapling_x as f32 + *x) as usize,
                (sapling_y as f32 + *y) as usize,
                (sapling_z as f32 + *z) as usize,
            )
        });

        if !proposal_valid {
            return;
        }

        proposal.iter().for_each(|(relative_pos, block_id)| {
            let Vec3 { x, y, z } = relative_pos;
            chunk.set(
                (sapling_x as f32 + *x) as usize,
                (sapling_y as f32 + *y) as usize,
                (sapling_z as f32 + *z) as usize,
                *block_id,
            );
        });
    }

    fn propose_tree_blocks() -> Vec<(Vec3, BlockId)> {
        let mut blocks = Vec::new();

        let min_tree_stump_height = 5;
        let max_tree_stump_height = 12;

        let tree_stump_height = rand::random_range(min_tree_stump_height..max_tree_stump_height);

        let bush_radius: i32 = rand::random_range(3..5);

        for dx in -bush_radius..bush_radius {
            for dz in -bush_radius..bush_radius {
                for dy in -bush_radius..bush_radius {
                    let distance_from_center = dx * dx + dy * dy + dz * dz;
                    if distance_from_center < bush_radius * bush_radius {
                        blocks.push((
                            Vec3 {
                                x: dx as f32,
                                y: (tree_stump_height + dy) as f32,
                                z: dz as f32,
                            },
                            BlockId::OakLeaves,
                        ));
                    }
                }
            }
        }

        for dy in 1..tree_stump_height {
            blocks.push((
                Vec3 {
                    x: 0.0,
                    y: dy as f32,
                    z: 0.0,
                },
                BlockId::OakLog,
            ));
        }

        blocks
    }

    fn decorate_block(&self, chunk: &Chunk, position: Vec3) -> BlockId {
        let x = position.x as usize;
        let y = position.y as usize;
        let z = position.z as usize;

        let block = chunk.get_unpadded(x, y, z);
        if block == BlockId::Air {
            return block;
        }

        let mut depth_below_nearest_air = 0;
        let depth_check = 3;

        for delta_height in 0..depth_check {
            if !Chunk::valid_unpadded(x, y + delta_height, z) {
                break;
            }

            let block = chunk.get_unpadded(x, y + delta_height, z);

            if block == BlockId::Air {
                break;
            }

            depth_below_nearest_air += 1;
        }

        match depth_below_nearest_air {
            0_i32..=1_i32 => BlockId::Grass,
            2..3 => BlockId::Dirt,
            _ => BlockId::Stone,
        }
    }

    fn generate_block(&self, position: Vec3) -> BlockId {
        if self.is_inside_cave(position) {
            return BlockId::Air;
        }

        if (position.y as f64) < self.determine_terrain_height(position) {
            return BlockId::Stone;
        }

        if self.determine_terrain_density(position) > 0.0 {
            return BlockId::Stone;
        }

        BlockId::Air
    }

    fn is_inside_cave(&self, position: Vec3) -> bool {
        let density = self.sample_3d(position, &self.params.cave.noise);

        let upper_bound = self.params.cave.base_value - self.params.cave.threshold;
        let lower_bound = self.params.cave.base_value + self.params.cave.threshold;

        lower_bound <= density && density >= upper_bound
    }

    fn determine_terrain_height(&self, position: Vec3) -> f64 {
        let noise_value = self
            .sample_2d(
                Vec2 {
                    x: position.x,
                    y: position.z,
                },
                &self.params.height.noise,
            )
            .abs();

        self.spline_lerp(noise_value)
    }

    fn determine_terrain_density(&self, position: Vec3) -> f64 {
        let density = self.sample_3d(position, &self.params.density.noise);
        let density_falloff = (position.y as f64 + self.params.density.height_offset)
            * self.params.density.squash_factor;

        density - density_falloff
    }

    pub fn normalized_spline_terrain_sample(&self, position: Vec2) -> f64 {
        let noise_value = self.sample_2d(position, &self.params.height.noise);

        let min_height = self.params.height.splines[0].y as f64;
        let max_height = self.params.height.splines[self.params.height.splines.len() - 1].y as f64;

        let splined_value = self.spline_lerp(noise_value);

        (splined_value - min_height) / (max_height - min_height)
    }

    fn spline_lerp(&self, x: f64) -> f64 {
        let x: f32 = x as f32;

        assert!(self.params.height.splines.len() >= 2);

        let min_x = self.params.height.splines[0].x;
        let max_x = self.params.height.splines[self.params.height.splines.len() - 1].x;

        assert!(min_x == -1.0);
        assert!(max_x == 1.0);

        for i in 0..self.params.height.splines.len() - 1 {
            let current = self.params.height.splines[i];
            let next = self.params.height.splines[i + 1];

            if x >= current.x && x <= next.x {
                return self.lerp(current, x, next);
            }
        }

        panic!("Could not find matching spline points for x value {}", x);
    }

    fn lerp(&self, point0: Vec2, x: f32, point1: Vec2) -> f64 {
        ((point0.y * (point1.x - x) + point1.y * (x - point0.x)) / (point1.x - point0.x)) as f64
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
