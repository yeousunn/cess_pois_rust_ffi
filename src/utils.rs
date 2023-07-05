use std::ffi::CString;
use std::os::raw::c_char;
use std::{slice, ptr};
use std::mem::forget;

use crate::c_types::{CommitC, CommonParam, I64ArrOfArr};
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

pub fn init_common_params(rsa_key: RsaKey, k: i64, n: i64, d: i64) -> CommonParam {
    let key_n = rsa_key.n;
    let key_g = rsa_key.g;

    let n_str = key_n.to_string();
    let n_cstring = CString::new(n_str).expect("CString conversion failed");

    let g_str = key_g.to_string();
    let g_cstring = CString::new(g_str).expect("CString conversion failed");

    CommonParam {
        key_n: n_cstring.into_raw() as *mut c_char,
        key_g: g_cstring.into_raw() as *mut c_char,
        k,
        n,
        d,
    }
}

pub fn c_ptr_to_i64_array_of_array(
    main_array: *mut *mut i64,
    sub_array_lengths: *const i32,
    main_array_length: i32,
) -> Vec<Vec<i64>> {
    let mut arr_of_arr: Vec<Vec<i64>> = Vec::new();
    unsafe {
        let arrays = std::slice::from_raw_parts(main_array, main_array_length as usize);
        let lengths = std::slice::from_raw_parts(sub_array_lengths, main_array_length as usize);

        for i in 0..main_array_length {
            let sub_array =
                std::slice::from_raw_parts(arrays[i as usize], lengths[i as usize] as usize);
            arr_of_arr.push(sub_array.to_vec());
        }
    }
    arr_of_arr
}

pub fn i64_array_of_array_to_c_ptr(arr: Vec<Vec<i64>>) -> I64ArrOfArr { // (*mut *mut i64, *const i32, i32) {
    let (main_array_ptr, lengths_ptr, length) = {
        let length = arr.len() as i32;
        let mut lengths: Vec<i32> = arr.iter().map(|sub_array| sub_array.len() as i32).collect();
        let mut main_array: Vec<*mut i64> = Vec::new();
        let mut sub_arrays: Vec<Vec<i64>> = Vec::new();

        for sub_array in arr {
            let mut sub_array_ptr: *mut i64 = ptr::null_mut();
            if !sub_array.is_empty() {
                sub_arrays.push(sub_array);
                sub_array_ptr = sub_arrays.last_mut().unwrap().as_mut_ptr();
            }
            main_array.push(sub_array_ptr);
        }

        let lengths_ptr = lengths.as_mut_ptr();
        let main_array_ptr = main_array.as_mut_ptr();

        // Prevent the vectors from being deallocated when going out of scope
        std::mem::forget(sub_arrays);
        std::mem::forget(lengths);
        std::mem::forget(main_array);

        (main_array_ptr, lengths_ptr, length)
    };

    I64ArrOfArr {
        main_array: main_array_ptr,
        sub_array_lengths: lengths_ptr,
        length,
    }
}

pub fn rust_commit_array_to_commit_c_array(commits: &mut [Commit],) -> Vec<CommitC> {
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

pub fn commit_c_array_to_rust_commit_array(commits: *const CommitC, length: i64) -> Vec<Commit> {
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



