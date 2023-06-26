use num_bigint_dig::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Commit {
    pub file_index: i64,
    pub roots: Vec<Vec<u8>>,
}

pub struct RsaKey {
    pub n: BigUint,
    pub g: BigUint,
}
