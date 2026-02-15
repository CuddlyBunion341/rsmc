pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_HEIGHT: usize = 128;
pub const PADDED_CHUNK_SIZE: usize = CHUNK_SIZE + 2;
pub const CHUNK_LENGTH: usize = PADDED_CHUNK_SIZE * CHUNK_HEIGHT * PADDED_CHUNK_SIZE;
