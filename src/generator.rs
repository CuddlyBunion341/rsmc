use crate::chunk::{Chunk, CHUNK_SIZE};
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
                    let height = self.perlin.get([
                        (x as f64 + chunk_origin.x as f64) * frequency,
                        (z as f64 + chunk_origin.z as f64) * frequency,
                    ]) * 10.0
                        + 5.0;

                    let height = height.floor() as i32;
                    let block = Self::block_from_height(y as i32, height);
                    chunk.set(x as usize, y as usize, z as usize, block);
                }
            }
        }
    }

    fn block_from_height(y: i32, height: i32) -> u8 {
        if y == height {
            1
        } else if y < height && y > height - 4 {
            2
        } else if y < height {
            3
        } else {
            0
        }
    }
}
