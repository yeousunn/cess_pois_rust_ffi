use libloading::Symbol;
use num_bigint_dig::BigUint;

use crate::{
    c_types::CommitC,
    types::Commit,
    utils::{c_pointer_to_i32_array_of_array, load_library, commit_to_commit_c, commit_c_array_to_go_commit_array},
};
use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_long},
    time::Instant,
};

type PerformPoisFunc = unsafe extern "C" fn(*mut c_char, *mut c_char, c_long, c_long, c_long);

type InitializePoisArtifactsFunc = unsafe extern "C" fn(*mut c_char, *mut c_char, c_long, c_long, c_long) -> i64;

// GetCommitsFunc(generated_count, key_n, key_g, k, n, d) -> (CommitC, length)
type GetCommitsFunc = unsafe extern "C" fn(c_long, *mut c_char, *mut c_char, c_long, c_long, c_long) -> (*mut CommitC, c_long);

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
) -> (*const *const i32, *const i32, i32);

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

pub fn call_initialize_pois_artifacts(
    key_n: BigUint,
    key_g: BigUint,
    k: i64,
    n: i64,
    d: i64,
) -> i64 {
    let lib = load_library();
    unsafe {
        // Retrieve the symbol for the InitializePoisArtifacts function
        let initialize_pois_artifacts: Symbol<InitializePoisArtifactsFunc> = lib
            .get(b"InitializePoisArtifacts")
            .expect("Failed to retrieve symbol");
        
        let n_str = key_n.to_string();
        let n_cstring = CString::new(n_str).expect("CString conversion failed");

        let g_str = key_g.to_string();
        let g_cstring = CString::new(g_str).expect("CString conversion failed");

        let generated_count = initialize_pois_artifacts(
            n_cstring.into_raw() as *mut c_char,
            g_cstring.into_raw() as *mut c_char,
            k,
            n,
            d,
        );
        generated_count
    }
}

pub fn call_get_commits(
    generated_count: i64,
    key_n: BigUint,
    key_g: BigUint,
    k: i64,
    n: i64,
    d: i64,
) -> Vec<Commit> {
    let lib = load_library();

    unsafe {
        // Retrieve the symbol for the GetCommits function
        let get_commits: Symbol<GetCommitsFunc> = lib
            .get(b"GetCommits")
            .expect("Failed to retrieve symbol");

        let n_str = key_n.to_string();
        let n_cstring = CString::new(n_str).expect("CString conversion failed");

        let g_str = key_g.to_string();
        let g_cstring = CString::new(g_str).expect("CString conversion failed");

        let (commit_c, length) = get_commits(
            generated_count, 
            n_cstring.into_raw() as *mut c_char,
            g_cstring.into_raw() as *mut c_char,
            k,
            n,
            d,
        );

        let commit = commit_c_array_to_go_commit_array(commit_c, length);
    
        commit
    }
}

pub fn call_generate_commit_challenge(
    generated_count: i64,
    commits: &mut [Commit],
    key_n: BigUint,
    key_g: BigUint,
    k: i64,
    n: i64,
    d: i64,
) -> Vec<Vec<i32>> {
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

        let mut commits_c = commit_to_commit_c(commits);
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
        );

        let challenge = c_pointer_to_i32_array_of_array(c_arrays, c_lengths, main_array_length);

        challenge
    }
}
