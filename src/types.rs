use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Commit {
    file_index: i64,
    roots: Vec<Vec<u8>>,
}