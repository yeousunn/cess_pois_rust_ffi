mod c_types;
mod types;

use libloading::{Library, Symbol};
use num_bigint_dig::{BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::One;
use rsa::RsaPrivateKey;
use rsa::traits::PublicKeyParts;


use c_types::{
    CommitC,
    MyByte, // Dummy Type for testing
};

use std::ffi::CString;
use std::os::raw::{c_long, c_char, c_int};

use std::time::Instant;

type PerformPoisFunc = unsafe extern "C" fn(*mut c_char, *mut c_char, c_long, c_long, c_long);
// CreateChallengeFunc(CommitC, length, key_n, key_g, k, n, d, chal, chal_length)
type CreateChallengeFunc = unsafe extern "C" fn(*mut CommitC, c_long, *mut c_char, *mut c_char, c_long, c_long, c_long, *mut *mut c_long, *mut c_long);




// Dummy function signatures
type GetByteArrayFunc = unsafe extern "C" fn(*mut c_char, c_long);
type GetByteArrayAsStructFunc = unsafe extern "C" fn(*mut MyByte);
type GetByteArrayAsStructArrayFunc = unsafe extern "C" fn(*mut MyByte, c_long);
type GetByteArrayOfArrayFunc = unsafe extern "C" fn(*mut *mut u8, c_int, *mut c_int);

fn load_library() -> Library {
    unsafe {
        Library::new("cgo/main.so")
            .expect("Failed to load the dynamic library")
    }
}

pub fn call_perform_pois(key_n: BigUint, key_g: BigUint, k: i64, n: i64, d: i64){
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
            d
        );
        // Stop the timer and calculate the elapsed time
        let elapsed_time = start_time.elapsed();

        // Print the elapsed time
        println!("Total execution time: {:?}", elapsed_time);

    }
}

pub fn call_get_byte_array() {
    
    let lib = load_library();
    unsafe {
        // Pass the byte array to the C function
        let get_byte_array: Symbol<GetByteArrayFunc> =
            lib.get(b"GetByteArray")
                .expect("Failed to retrieve symbol");

        let data: [u8; 3] = [1, 2, 3];
        let data_ptr = data.as_ptr() as *mut c_char;
        let data_len = data.len() as c_long;
        get_byte_array(data_ptr, data_len);
    }
}

pub fn call_get_byte_array_as_struct() {
    
    let lib = load_library();
    unsafe {
        // Pass the byte array to the C function
        let get_byte_array_as_struct: Symbol<GetByteArrayAsStructFunc> =
            lib.get(b"GetByteArrayAsStruct")
                .expect("Failed to retrieve symbol");

        let data: [u8; 3] = [1, 2, 3];
        let mut my_byte = MyByte {
            b: data.as_ptr() as *mut u8,
            length: data.len() as c_long,
        };
        // let data_ptr = data.as_ptr() as *mut c_char;
        // let data_len = data.len() as c_long;
        get_byte_array_as_struct(&mut my_byte as *mut MyByte);
    }
}

pub fn call_get_byte_array_as_struct_array() {
    
    let lib = load_library();
    unsafe {
        // Pass the byte array to the C function
        let get_byte_array_by_struct: Symbol<GetByteArrayAsStructArrayFunc> =
            lib.get(b"GetByteArrayAsStructArray")
                .expect("Failed to retrieve symbol");

        let data1: [u8; 3] = [1, 2, 3];
        let my_byte1 = MyByte {
            b: data1.as_ptr() as *mut u8,
            length: data1.len() as c_long,
        };
        let data2: [u8; 4] = [5, 6, 7, 8];
        let my_byte2 = MyByte {
            b: data2.as_ptr() as *mut u8,
            length: data2.len() as c_long,
        };

        let mut data = vec![my_byte1, my_byte2];
        // let data_ptr = data.as_ptr() as *mut c_char;
        // let data_len = data.len() as c_long;
        get_byte_array_by_struct(data.as_mut_ptr(), data.len() as c_long);
    }
}

pub fn call_get_byte_array_of_array() {
    let lib = load_library();
    unsafe {
        let get_byte_array_of_array: Symbol<GetByteArrayOfArrayFunc> =
            lib.get(b"GetByteArrayOfArray")
                .expect("Failed to retrieve symbol");

        let data1: [u8; 5] = [1, 2, 3, 4, 5];
        let data2: [u8; 4] = [4, 5, 6, 7];

        let mut sub_data_lengths = vec![data1.len() as c_int, data2.len() as c_int];

        // Create a vector of byte array pointers
        let byte_array1 = data1.as_ptr() as *mut u8;
        let byte_array2 = data2.as_ptr() as *mut u8;
        let mut byte_array_ptrs = vec![byte_array1, byte_array2];

        // Pass the array of pointers to the C function
        let data_length = byte_array_ptrs.len() as c_int;
        get_byte_array_of_array(byte_array_ptrs.as_mut_ptr(), data_length, sub_data_lengths.as_mut_ptr());
    }
}

#[derive(Debug)]
pub struct Commit {
    file_index: i64,
    roots: Vec<Vec<u8>>,
}

pub fn call_create_challenge(commits: &mut [Commit], key_n: BigUint, key_g: BigUint, k: i64, n: i64, d: i64) -> Vec<Vec<i64>> {
    let lib = load_library();

    unsafe {
        // Retrieve the symbol for the CreateChallenge function
        let create_challenge: Symbol<CreateChallengeFunc> =
            lib.get(b"CreateChallenge").expect("Failed to retrieve symbol");

        let n_str = key_n.to_string();
        let n_cstring = CString::new(n_str).expect("CString conversion failed");
        
        let g_str = key_g.to_string();
        let g_cstring = CString::new(g_str).expect("CString conversion failed");
        
        // Prepare the CommitC struct
        let mut commits_c: Vec<CommitC> = Vec::with_capacity(commits.len());

        for commit in commits.iter() {
            let roots_len = commit.roots.len() as i32;
            let mut roots_ptr: *mut *mut u8 = std::ptr::null_mut();
            let mut sub_roots_lengths_ptr: *mut i32 = std::ptr::null_mut();

            if roots_len > 0 {
                roots_ptr = libc::malloc((roots_len as usize) * std::mem::size_of::<*mut u8>()) as *mut *mut u8;
                sub_roots_lengths_ptr = libc::malloc((roots_len as usize) * std::mem::size_of::<i32>()) as *mut i32;

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

        // Prepare the variables to receive the challenges
        let mut chals: *mut *mut c_long = std::ptr::null_mut();
        let mut chals_length: c_long = 0;

        // Call the CreateChallenge function
        create_challenge(
            commits_c.as_mut_ptr(), 
            commits_c.len() as c_long,
            n_cstring.into_raw() as *mut c_char,
            g_cstring.into_raw() as *mut c_char, 
            k,
            n,
            d,
            chals,
            &mut chals_length,
        );

        // Convert the challenges to a Vec<Vec<i64>> in Rust
         let mut result: Vec<Vec<i64>> = Vec::new();
         for i in 0..chals_length {
             let chal_ptr = *chals.offset(i as isize);
             let chal_slice = std::slice::from_raw_parts(chal_ptr, n as usize);
             let chal_vec = chal_slice.iter().copied().collect();
             result.push(chal_vec);
         }

         println!("result: {:?}", result);
 
        // // Free the memory allocated for roots, subRootsLengths, and challenges
        // for commit_c in commits_c.iter() {
        //     for i in 0..commit_c.roots_length {
        //         libc::free(*commit_c.roots.offset(i as isize) as *mut libc::c_void);
        //     }
        //     libc::free(commit_c.roots as *mut libc::c_void);
        //     libc::free(commit_c.sub_roots_lengths as *mut libc::c_void);
        // }
        // for i in 0..chals_length {
        //     libc::free(*chals.offset(i as isize) as *mut libc::c_void);
        // }
        // libc::free(chals as *mut libc::c_void);

        result
    }
}


pub struct RsaKey {
    pub n: BigUint,
    pub g: BigUint,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perform_pois() {
        let rsa_key = rsa_keygen(2048);
        let key_n = rsa_key.n;
        let key_g = rsa_key.g;

        let k: i64 = 7; // Replace with the actual value
        let n: i64 = 1024 * 1024 * 4; // Replace with the actual value
        let d: i64 = 64; // Replace with the actual value
        call_perform_pois(
            key_n,
            key_g,
            k,
            n,
            d
        );

        // call_get_byte_array()
        // call_get_byte_array_as_struct()
        // call_get_byte_array_as_struct_array();
        // call_get_byte_array_of_array();

    }

    #[test]
    fn test_create_challenge(){
        let rsa_key = rsa_keygen(2048);
        let key_n = rsa_key.n;
        let key_g = rsa_key.g;

        let k: i64 = 7; // Replace with the actual value
        let n: i64 = 1024 * 1024; // Replace with the actual value
        let d: i64 = 64; // Replace with the actual value

        let mut commits = vec![
            Commit {
                    file_index: 6,
                    roots: vec![
                        vec![1, 2, 3],
                        vec![4, 5, 6],
                    ],
                },
            Commit {
                file_index: 3,
                roots: vec![
                    vec![7, 8, 9],
                    vec![10, 11, 12, 13],
                ],
            },
        ];
     
        call_create_challenge(
            &mut commits,
            key_n,
            key_g,
            k,
            n,
            d,
        );
    }
}
