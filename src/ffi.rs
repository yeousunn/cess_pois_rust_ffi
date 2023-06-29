use libc::c_longlong;
use libloading::Symbol;
use num_bigint_dig::BigUint;

use crate::{
    c_types::CommitC,
    types::Commit,
    utils::{c_pointer_to_i64_array_of_array, load_library, rust_commit_array_to_commit_c_array, commit_c_array_to_rust_commit_array, array_of_array_to_c_ptr},
};
use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_long},
    time::Instant, ptr,
};

// CreateChallengeFunc(generated_count, CommitC, length, key_n, key_g, k, n, d) -> [][]i64, []i64, i64
type GenerateCommitChallengeFunc = unsafe extern "C" fn(
    c_long,
    *mut CommitC,
    c_long,
    *mut c_char,
    *mut c_char,
    c_long,
    c_long,
    c_long,
    *mut c_char,
    c_int
) -> (*mut *mut i64, *mut i32, i32);

pub fn call_generate_commit_challenge(
    generated_count: i64,
    commits: &mut [Commit],
    key_n: BigUint,
    key_g: BigUint,
    k: i64,
    n: i64,
    d: i64,
    id: &str,
) -> Vec<Vec<i64>> {
    let lib = load_library();

    unsafe {
        // Retrieve the symbol for the CreateCommitChallenge function
        let generate_commit_challenge: Symbol<GenerateCommitChallengeFunc> = lib
            .get(b"GenerateCommitChallenge")
            .expect("Failed to retrieve symbol");

        let n_str = key_n.to_string();
        let n_cstring = CString::new(n_str).expect("CString conversion failed");

        let g_str = key_g.to_string();
        let g_cstring = CString::new(g_str).expect("CString conversion failed");

        let id_length = id.len() as i32;
        let id_ptr = id.as_ptr() as *mut c_char;
        
        let mut commits_c = rust_commit_array_to_commit_c_array(commits);
        // Call the CreateCommitChallenge function
        let (c_arrays, c_lengths, main_array_length) = generate_commit_challenge(
            generated_count,
            commits_c.as_mut_ptr(),
            commits_c.len() as c_long,
            n_cstring.into_raw() as *mut c_char,
            g_cstring.into_raw() as *mut c_char,
            k,
            n,
            d,
            id_ptr,
            id_length,
        );

        let challenge = c_pointer_to_i64_array_of_array(c_arrays, c_lengths, main_array_length);

        challenge
    }
}
