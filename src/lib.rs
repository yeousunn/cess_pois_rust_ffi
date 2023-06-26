mod c_types;
mod ffi;
mod types;
mod utils;

use crate::utils::c_pointer_to_i64_array_of_array;
use c_types::MyByte; // Dummy Type for testing
use libloading::Symbol;
use std::os::raw::{c_char, c_int, c_long};
use utils::load_library;

// Dummy function signatures
type GetByteArrayFunc = unsafe extern "C" fn(*mut c_char, c_long);
type GetByteArrayAsStructFunc = unsafe extern "C" fn(*mut MyByte);
type GetByteArrayAsStructArrayFunc = unsafe extern "C" fn(*mut MyByte, c_long);
type GetByteArrayOfArrayFunc = unsafe extern "C" fn(*mut *mut u8, c_int, *mut c_int);

type GetArrayFunc = extern "C" fn() -> (*const c_int, *const c_int);
type FreeArrayFunc = extern "C" fn(*const c_int);
type GetArrayOfArrayFunc = extern "C" fn() -> (*const *const i64, *const i64, i64);

pub fn call_get_byte_array() {
    let lib = load_library();
    unsafe {
        // Pass the byte array to the C function
        let get_byte_array: Symbol<GetByteArrayFunc> =
            lib.get(b"GetByteArray").expect("Failed to retrieve symbol");

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
        let get_byte_array_as_struct: Symbol<GetByteArrayAsStructFunc> = lib
            .get(b"GetByteArrayAsStruct")
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
        let get_byte_array_by_struct: Symbol<GetByteArrayAsStructArrayFunc> = lib
            .get(b"GetByteArrayAsStructArray")
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
        let get_byte_array_of_array: Symbol<GetByteArrayOfArrayFunc> = lib
            .get(b"GetByteArrayOfArray")
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
        get_byte_array_of_array(
            byte_array_ptrs.as_mut_ptr(),
            data_length,
            sub_data_lengths.as_mut_ptr(),
        );
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
        let free_array: libloading::Symbol<FreeArrayFunc> =
            lib.get(b"FreeArray").expect("Failed to retrieve symbol");

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
        let (arr1_ptr, arr2_ptr) = get_array();

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

pub fn call_return_array_of_array() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ffi::{call_create_challenge, call_perform_pois},
        types::Commit,
        utils::rsa_keygen,
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
    fn test_create_challenge() {
        let rsa_key = rsa_keygen(2048);
        let key_n = rsa_key.n;
        let key_g = rsa_key.g;

        let k: i64 = 7;
        let n: i64 = 1024 * 1024 * 4;
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

        call_create_challenge(&mut commits, key_n, key_g, k, n, d);
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
