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
        for x in -1..CHUNK_SIZE {
            for y in -1..CHUNK_SIZE {
                for z in -1..CHUNK_SIZE {
                    let density = self.perlin.get([
                        x as f64 / CHUNK_SIZE as f64,
                        y as f64 / CHUNK_SIZE as f64,
                        z as f64 / CHUNK_SIZE as f64,
                    ]) * 0.5;
                    let block = Generator::block_from_density(density);
                    chunk.set((x + 1) as usize, (y + 1) as usize, (z + 1) as usize, block)
                }
            }
        }
    }

    pub fn block_from_density(density: f64) -> u8 {
        if density > 0.0 {
            1
        } else {
            0
        }
    }
}
