mod c_types;
mod ffi;
mod types;
mod utils;
mod examples;

#[cfg(test)]
mod tests {
    use crate::{
        ffi::{call_generate_commit_challenge, call_perform_pois, call_initialize_pois_artifacts, call_get_commits, call_get_commit_proof_and_acc_proof},
        utils::rsa_keygen, examples::{call_return_array_of_array, call_return_an_array},
    };

    #[test]
    fn test_perform_pois() {
        let rsa_key = rsa_keygen(2048);
        let key_n = rsa_key.n;
        let key_g = rsa_key.g;

        let k: i64 = 7;
        let n: i64 = 1024 * 1024 * 4;
        let d: i64 = 64;
        call_perform_pois(key_n, key_g, k, n, d);

        // call_get_byte_array()
        // call_get_byte_array_as_struct()
        // call_get_byte_array_as_struct_array();
        // call_get_byte_array_of_array();
    }

    #[test]
    fn test_initialize_pois_artifacts() {
        let rsa_key = rsa_keygen(2048);
        let key_n = rsa_key.n;
        let key_g = rsa_key.g;

        let k: i64 = 7;
        let n: i64 = 512;
        let d: i64 = 64;
        let generated_count = call_initialize_pois_artifacts(key_n, key_g, k, n, d);

        println!("generated_count: {}", generated_count);
    }

    #[test]
    fn test_get_commits(){
        let generated_count = 4;
        let rsa_key = rsa_keygen(2048);
        let key_n = rsa_key.n;
        let key_g = rsa_key.g;

        let k: i64 = 7;
        let n: i64 = 512;
        let d: i64 = 64;

        let commits = call_get_commits(generated_count, key_n, key_g, k, n, d);

        println!("commits: {:?}", commits);
    }

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

        let mut commits = call_get_commits(generated_count, key_n.clone(), key_g.clone(), k, n, d);
        let chal = call_generate_commit_challenge(generated_count, &mut commits, key_n.clone(), key_g.clone(), k, n, d);
        // println!("Rust generatedChals: {:?}", chal);

        // Test call_get_commit_proof_and_acc_proof
        call_get_commit_proof_and_acc_proof(generated_count, chal, key_n.clone(), key_g.clone(), k, n, d);

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
