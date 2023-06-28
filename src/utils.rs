use std::slice;

use crate::c_types::CommitC;
use crate::types::{RsaKey, Commit};
use libloading::Library;
use num_bigint_dig::{BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::One;
use rsa::traits::PublicKeyParts;
use rsa::RsaPrivateKey;

pub fn load_library() -> Library {
    unsafe { Library::new("cgo/main.so").expect("Failed to load the dynamic library") }
}

pub fn rsa_keygen(lambda: usize) -> RsaKey {
    let mut rng = rand::thread_rng();
    let pk = RsaPrivateKey::new(&mut rng, lambda).expect("Failed to generate RSA key");

    let n = pk.n();
    let mut f: BigUint;
    let g: BigUint;

    loop {
        f = rng.gen_biguint(lambda);
        if f.gcd(n) == BigUint::one() {
            break;
        }
    }

    g = f.modpow(&BigUint::from(2u32), &n.clone());

    RsaKey {
        n: n.clone(),
        g: g.clone(),
    }
}

pub fn c_pointer_to_i32_array_of_array(
    c_arrays: *const *const i32,
    c_lengths: *const i32,
    main_array_length: i32,
) -> Vec<Vec<i32>> {
    let mut arr_of_arr: Vec<Vec<i32>> = Vec::new();
    unsafe {
        let arrays = std::slice::from_raw_parts(c_arrays, main_array_length as usize);
        let lengths = std::slice::from_raw_parts(c_lengths, main_array_length as usize);

        for i in 0..main_array_length {
            let sub_array =
                std::slice::from_raw_parts(arrays[i as usize], lengths[i as usize] as usize);
            arr_of_arr.push(sub_array.to_vec());
        }
    }
    arr_of_arr
}

pub fn commit_to_commit_c(commits: &mut [Commit],) -> Vec<CommitC> {
    unsafe {
        // Prepare the CommitC struct
        let mut commits_c: Vec<CommitC> = Vec::with_capacity(commits.len());

        for commit in commits.iter() {
            let roots_len = commit.roots.len() as i32;
            let mut roots_ptr: *mut *mut u8 = std::ptr::null_mut();
            let mut sub_roots_lengths_ptr: *mut i32 = std::ptr::null_mut();

            if roots_len > 0 {
                roots_ptr = libc::malloc((roots_len as usize) * std::mem::size_of::<*mut u8>())
                    as *mut *mut u8;
                sub_roots_lengths_ptr =
                    libc::malloc((roots_len as usize) * std::mem::size_of::<i32>()) as *mut i32;

                for (i, root) in commit.roots.iter().enumerate() {
                    let root_len = root.len();
                    let root_ptr = libc::malloc(root_len) as *mut u8;
                    std::ptr::copy_nonoverlapping(root.as_ptr(), root_ptr, root_len);

                    *roots_ptr.offset(i as isize) = root_ptr;
                    *sub_roots_lengths_ptr.offset(i as isize) = root_len as i32;
                }
            }

            let commit_c = CommitC {
                file_index: commit.file_index,
                roots: roots_ptr,
                roots_length: roots_len,
                sub_roots_lengths: sub_roots_lengths_ptr,
            };

            commits_c.push(commit_c);
        }
        commits_c
    }
}

pub fn commit_c_array_to_go_commit_array(commits: *const CommitC, length: i64) -> Vec<Commit> {
    let data_slice = unsafe { slice::from_raw_parts(commits, length as usize) };

    let mut go_commits = Vec::with_capacity(length as usize);

    for i in 0..length {
        let c = &data_slice[i as usize];

        let roots_array_slice = unsafe {
            slice::from_raw_parts(c.roots, c.roots_length as usize)
        };
        let roots_lengths_slice = unsafe {
            slice::from_raw_parts(c.sub_roots_lengths, c.roots_length as usize)
        };

        let mut roots = Vec::with_capacity(c.roots_length as usize);

        for j in 0..c.roots_length {
            let byte_array_ptr = roots_array_slice[j as usize];
            let byte_array_len = roots_lengths_slice[j as usize];

            let byte_slice = unsafe {
                slice::from_raw_parts(byte_array_ptr, byte_array_len as usize)
            };

            let new_byte_array = byte_slice.to_vec();
            roots.push(new_byte_array);
        }

        let go_commit = Commit {
            file_index: c.file_index,
            roots,
        };

        go_commits.push(go_commit);
    }

    go_commits
}
