pub const BLOCK_SIZE: usize = 4096;
pub const HASH_SIZE: usize = 32;
pub const HASHES_PER_BRANCH_BLOCK: usize = BLOCK_SIZE / HASH_SIZE;