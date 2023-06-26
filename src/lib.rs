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

// CreateChallengeFunc(CommitC, length, key_n, key_g, k, n, d) -> [][]i64, []i64, i64
type CreateChallengeFunc = unsafe extern "C" fn(*mut CommitC, c_long, *mut c_char, *mut c_char, c_long, c_long, c_long) -> (*const *const i64, *const i64, i64);




// Dummy function signatures
type GetByteArrayFunc = unsafe extern "C" fn(*mut c_char, c_long);
type GetByteArrayAsStructFunc = unsafe extern "C" fn(*mut MyByte);
type GetByteArrayAsStructArrayFunc = unsafe extern "C" fn(*mut MyByte, c_long);
type GetByteArrayOfArrayFunc = unsafe extern "C" fn(*mut *mut u8, c_int, *mut c_int);

type GetArrayFunc = extern "C" fn() -> (*const c_int, *const c_int);
type FreeArrayFunc = extern "C" fn(*const c_int);
type GetArrayOfArrayFunc = extern "C" fn() -> (*const *const i64, *const i64, i64);

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

pub fn call_return_an_array() {
    // Load the Go shared library
    let lib = load_library();
    unsafe {
        // Get the symbols for the functions
        let get_array: libloading::Symbol<GetArrayFunc> = lib
            .get(b"ReturnAnArray")
            .expect("Failed to retrieve symbol");
        let free_array: libloading::Symbol<FreeArrayFunc> = lib
            .get(b"FreeArray")
            .expect("Failed to retrieve symbol");

        // // Call the Go function
        // let (go_array, go_array2)  = get_array();
        // if !go_array.is_null() {
        //     let array_len = 4; // Assuming the Go array has a length of 4
        //     let slice = std::slice::from_raw_parts(go_array, array_len);
        //     let rust_array = slice.to_vec();
        //     println!("Received array from Go: {:?}", rust_array);

        //     // Free the Go array
        //     free_array(go_array as *const c_int);
        // }

        // if !go_array2.is_null() {
        //     let array_len2 = 4; // Assuming the Go array has a length of 4
        //     let slice = std::slice::from_raw_parts(go_array2, array_len2);
        //     let rust_array = slice.to_vec();
        //     println!("Received array 2 from Go: {:?}", rust_array);

        //     // Free the Go array
        //     free_array(go_array2 as *const c_int);
        // }

         // Call the Go function to get the arrays
        let (arr1_ptr, arr2_ptr) = get_array() ;

        // Convert the C arrays to Rust slices
        let arr1_slice: &[c_int] = std::slice::from_raw_parts(arr1_ptr, 4);
        let arr2_slice: &[c_int] = std::slice::from_raw_parts(arr2_ptr, 4);

        // Convert the slices to Vec<i32>
        let arr1_vec: Vec<i32> = arr1_slice.iter().map(|&val| val as i32).collect();
        let arr2_vec: Vec<i32> = arr2_slice.iter().map(|&val| val as i32).collect();

        // Print the arrays
        println!("Array 1: {:?}", arr1_vec);
        println!("Array 2: {:?}", arr2_vec);
    }
}

fn c_pointer_to_i64_array_of_array(c_arrays: *const *const i64, c_lengths: *const i64, main_array_length: i64) -> Vec<Vec<i64>>{
    let mut arr_of_arr: Vec<Vec<i64>> = Vec::new();
    unsafe{
        let arrays = std::slice::from_raw_parts(c_arrays, main_array_length as usize);
        let lengths = std::slice::from_raw_parts(c_lengths, main_array_length as usize);
    
        for i in 0..main_array_length {
            let sub_array = std::slice::from_raw_parts(arrays[i as usize], lengths[i as usize] as usize);
            arr_of_arr.push(sub_array.to_vec());
        }
    }
    arr_of_arr
}

pub fn call_return_array_of_array(){
    // Load the Go shared library
    let lib = load_library();
    unsafe {
        let get_array_of_array: libloading::Symbol<GetArrayOfArrayFunc> = lib
            .get(b"ReturnArrayofArrays")
            .expect("Failed to retrieve symbol");

        let (c_arrays, c_lengths, main_array_length) = get_array_of_array();

        let array = c_pointer_to_i64_array_of_array(c_arrays, c_lengths, main_array_length);
        
        println!("Array {:?}", array);

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

        let k: i64 = 7;
        let n: i64 = 1024 * 1024 * 4;
        let d: i64 = 64;
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

        let k: i64 = 7;
        let n: i64 = 1024 * 1024 * 4;
        let d: i64 = 64;

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

    #[test]
    fn test_call_return_an_array(){
        call_return_an_array()
    }

    #[test]
    fn test_call_return_array_of_array(){
        call_return_array_of_array()
    }

}
