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
                    ]) * 20.0;

                    let height = height.floor() as i32;
                    if (y + chunk_origin.y as i32) < height {
                        chunk.set(x as usize, y as usize, z as usize, 1);
                    }
                }
            }
        }
    }
}
