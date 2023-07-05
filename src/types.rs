use num_bigint_dig::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Commit {
    pub file_index: i64,
    pub roots: Vec<Vec<u8>>,
}

pub struct MhtProof {
    pub index: i32,
    pub label: Vec<u8>,
    pub paths: Vec<Vec<u8>>,
    pub locs: Vec<u8>,
}

pub struct CommitProof {
    pub node: Option<MhtProof>,
    pub parents: Vec<MhtProof>,
}

pub struct RsaKey {
    pub n: BigUint,
    pub g: BigUint,
}
