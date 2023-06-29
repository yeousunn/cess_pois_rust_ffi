mod c_types;
mod ffi;
mod types;
mod utils;
mod examples;

#[cfg(test)]
mod tests {
    use crate::{
        ffi::{call_generate_commit_challenge},
        utils::rsa_keygen, examples::{call_return_array_of_array, call_return_an_array}, types::Commit,
    };

    #[test]
    fn test_generate_commit_challenge() {
        let generated_count = 4;
        let rsa_key = rsa_keygen(2048);
        let key_n = rsa_key.n;
        let key_g = rsa_key.g;

        let k: i64 = 7;
        // let n: i64 = 1024 * 1024 * 4;
        let n: i64 = 512;
        let d: i64 = 64;

        let mut commits = vec![
            Commit {
                file_index: 6,
                roots: vec![vec![1, 2, 3], vec![4, 5, 6]],
            },
            Commit {
                file_index: 3,
                roots: vec![vec![7, 8, 9], vec![10, 11, 12, 13]],
            },
        ];
        
        let id = "test miner id";

        let chal = call_generate_commit_challenge(generated_count, &mut commits, key_n.clone(), key_g.clone(), k, n, d, id);
        println!("Rust generatedChals: {:?}", chal);

    }

    #[test]
    fn test_call_return_an_array() {
        call_return_an_array()
    }

    #[test]
    fn test_call_return_array_of_array() {
        call_return_array_of_array()
    }
}
