use libloading::Symbol;
use num_bigint_dig::BigUint;

use crate::{
    c_types::CommitC,
    types::Commit,
    utils::{c_pointer_to_i64_array_of_array, load_library, commit_to_commit_c},
};
use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_long},
    time::Instant,
};

type PerformPoisFunc = unsafe extern "C" fn(*mut c_char, *mut c_char, c_long, c_long, c_long);

// CreateChallengeFunc(CommitC, length, key_n, key_g, k, n, d) -> [][]i64, []i64, i64
type CreateChallengeFunc = unsafe extern "C" fn(
    *mut CommitC,
    c_long,
    *mut c_char,
    *mut c_char,
    c_long,
    c_long,
    c_long,
) -> (*const *const i64, *const i64, i64);

pub fn call_perform_pois(key_n: BigUint, key_g: BigUint, k: i64, n: i64, d: i64) {
    // Load the Go dynamic library
    let lib = load_library();
    unsafe {
        // Get the symbol for the PerformPois function
        let perform_pois: Symbol<PerformPoisFunc> =
            lib.get(b"PerformPois").expect("Failed to get symbol");

        let n_str = key_n.to_string();
        let n_cstring = CString::new(n_str).expect("CString conversion failed");

        let g_str = key_g.to_string();
        let g_cstring = CString::new(g_str).expect("CString conversion failed");

        // Start the timer
        let start_time = Instant::now();
        perform_pois(
            n_cstring.into_raw() as *mut c_char,
            g_cstring.into_raw() as *mut c_char,
            k,
            n,
            d,
        );
        // Stop the timer and calculate the elapsed time
        let elapsed_time = start_time.elapsed();

        // Print the elapsed time
        println!("Total execution time: {:?}", elapsed_time);
    }
}

pub fn call_create_challenge(
    commits: &mut [Commit],
    key_n: BigUint,
    key_g: BigUint,
    k: i64,
    n: i64,
    d: i64,
) -> Vec<Vec<i64>> {
    let lib = load_library();

    unsafe {
        // Retrieve the symbol for the CreateChallenge function
        let create_challenge: Symbol<CreateChallengeFunc> = lib
            .get(b"CreateChallenge")
            .expect("Failed to retrieve symbol");

        let n_str = key_n.to_string();
        let n_cstring = CString::new(n_str).expect("CString conversion failed");

        let g_str = key_g.to_string();
        let g_cstring = CString::new(g_str).expect("CString conversion failed");

        let mut commits_c = commit_to_commit_c(commits);
        // Call the CreateChallenge function
        let (c_arrays, c_lengths, main_array_length) = create_challenge(
            commits_c.as_mut_ptr(),
            commits_c.len() as c_long,
            n_cstring.into_raw() as *mut c_char,
            g_cstring.into_raw() as *mut c_char,
            k,
            n,
            d,
        );

        let challenge = c_pointer_to_i64_array_of_array(c_arrays, c_lengths, main_array_length);

        challenge
    }
}
