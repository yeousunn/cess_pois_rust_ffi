use libloading::Symbol;
use crate::{
    c_types::{CommitC, CommonParam, ProverID, CommitProofC, MhtProofC, I64ArrOfArr},
    types::{Commit, CommitProof},
    utils::{c_ptr_to_i64_array_of_array, load_library, rust_commit_array_to_commit_c_array, commit_c_array_to_rust_commit_array, i64_array_of_array_to_c_ptr},
};
use std::{
    os::raw::{c_char, c_int, c_long, c_uchar}, ptr,
};

type GenerateCommitChallengeFunc = unsafe extern "C" fn(
    *mut CommitC, // CommitC Array
    c_int, // Length of CommitC Array
    *mut CommonParam, // CommonParam
    *mut ProverID, // Prover/Miner ID
) -> (*mut *mut i64, *mut i32, i32);

type VerifyCommitAndAccProofsFunc = unsafe extern "C" fn(
    // *mut *mut CommitProofC,
    // c_int, 
    *mut I64ArrOfArr,
    *mut CommonParam, // CommonParam
    *mut ProverID,
);

pub fn call_generate_commit_challenge(
    commits: &mut [Commit],
    common_params: &mut CommonParam,
    id: &str,
) -> Vec<Vec<i64>> {
    let lib = load_library();

    unsafe {
        let generate_commit_challenge: Symbol<GenerateCommitChallengeFunc> = lib
            .get(b"GenerateCommitChallenge")
            .expect("Failed to retrieve symbol");

        let prover_id = &mut ProverID {
            id: id.as_ptr() as *mut c_char,
            length: id.len() as i32,
        };
        
        let mut commits_c = rust_commit_array_to_commit_c_array(commits);
        
        let (c_arrays, c_lengths, main_array_length) = generate_commit_challenge(
            commits_c.as_mut_ptr(),
            commits_c.len() as c_int,
            common_params,
            prover_id,
        );

        let challenge = c_ptr_to_i64_array_of_array(c_arrays, c_lengths, main_array_length);
        challenge
    }
}

pub fn call_verify_commit_and_acc_proofs(
    commit_proof: Vec<Vec<CommitProof>>,
    challenge: Vec<Vec<i64>>,
    common_params: &mut CommonParam,
    id: &str,
) {
    let lib = load_library();

    unsafe{
        let verify_commit_and_acc_proofs: Symbol<VerifyCommitAndAccProofsFunc> = lib
            .get(b"VerifyCommitAndAccProofs")
            .expect("Failed to retrieve symbol");

        let prover_id = &mut ProverID {
            id: id.as_ptr() as *mut c_char,
            length: id.len() as i32,
        };

        let commit_proof_length = commit_proof.len() as c_int;
        
        // TODO:
        // Convert commit_proof: Vec<Vec<CommitProof>>
        // to **CommitProofC

        let mut challenge_c = i64_array_of_array_to_c_ptr(challenge);

        verify_commit_and_acc_proofs(
            // &mut &mut commit_proof_c as *mut *mut CommitProofC,
            // commit_proof_length,
            &mut challenge_c as *mut I64ArrOfArr,
            common_params,
            prover_id
        );
    }
}
