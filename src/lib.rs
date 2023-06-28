mod c_types;
mod ffi;
mod types;
mod utils;

use crate::utils::c_pointer_to_i32_array_of_array;
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
type GetArrayOfArrayFunc = extern "C" fn() -> (*const *const i32, *const i32, i32);

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

        let array = c_pointer_to_i32_array_of_array(c_arrays, c_lengths, main_array_length);

        println!("Array {:?}", array);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ffi::{call_generate_commit_challenge, call_perform_pois, call_initialize_pois_artifacts, call_get_commits},
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

    // #[test]
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
        let rsa_key = rsa_keygen(2048);
        let key_n = rsa_key.n;
        let key_g = rsa_key.g;

        let k: i64 = 7;
        let n: i64 = 512;
        let d: i64 = 64;

        let commits = call_get_commits(4, key_n, key_g, k, n, d);

        println!("commits: {:?}", commits);
    }

    #[test]
    fn test_generate_commit_challenge() {
        let rsa_key = rsa_keygen(2048);
        let key_n = rsa_key.n;
        let key_g = rsa_key.g;

        let k: i64 = 7;
        // let n: i64 = 1024 * 1024 * 4;
        let n: i64 = 512;
        let d: i64 = 64;

        // let mut commits = vec![
        //     Commit {
        //         file_index: 1,
        //         roots: vec![vec![121,162,193,172,131,126,159,48,88,75,205,99,92,127,0,67,182,10,57,59,165,158,155,154,50,245,79,198,188,75,149,42,240,235,210,88,46,87,252,219,178,87,25,169,150,245,111,9,236,202,250,117,237,137,250,174,10,53,36,186,141,185,38,72],vec![253,249,38,156,39,218,16,30,250,239,181,37,146,139,79,148,60,152,117,134,86,242,72,254,183,253,252,41,237,183,200,248,142,76,248,54,106,53,194,229,35,179,19,45,12,117,121,154,184,153,116,215,140,224,148,46,207,27,57,175,192,175,144,208],vec![20,89,242,210,94,5,99,173,102,192,231,103,216,27,142,76,67,75,99,85,37,202,205,45,169,85,107,6,118,134,148,133,75,167,238,136,55,236,241,252,95,186,90,174,226,142,251,116,13,174,223,5,163,50,71,27,106,195,190,9,168,97,249,226],vec![35,18,62,124,92,207,158,238,178,87,216,84,99,116,215,248,246,35,47,215,181,29,76,3,123,37,171,0,188,161,178,194,121,43,178,170,21,47,126,230,187,36,137,31,210,118,176,21,237,183,243,24,113,119,7,219,89,176,69,149,48,209,153,236],vec![139,17,48,193,199,128,50,68,102,212,103,144,245,142,134,28,178,2,12,115,142,125,149,186,81,102,215,179,54,129,1,154,10,65,55,214,39,36,247,65,54,123,59,221,27,108,240,112,120,189,202,239,186,121,195,154,194,223,172,220,151,96,83,137],vec![33,175,7,86,99,248,38,207,157,153,185,108,43,1,33,21,89,68,156,231,132,210,113,221,218,41,15,61,207,113,209,62,243,194,179,104,224,233,17,80,248,155,73,24,54,130,44,73,202,128,209,194,150,23,12,244,221,238,107,55,41,74,15,67],vec![49,94,249,194,50,189,113,198,237,207,165,27,96,83,24,123,54,121,186,124,239,203,247,238,126,24,71,83,217,143,2,85,219,34,124,241,81,250,134,92,242,127,121,233,235,172,236,101,98,73,240,49,42,224,89,178,44,237,42,129,201,195,24,114],vec![135,47,95,47,34,180,178,166,188,13,212,157,39,119,52,136,198,121,70,230,241,117,188,108,163,39,214,237,214,7,96,184,254,72,37,243,168,52,236,35,170,187,25,34,41,150,252,183,53,12,66,27,182,247,20,91,189,162,177,71,200,79,244,199],vec![247,251,186,241,239,161,178,170,188,8,153,214,150,154,39,85,66,65,192,128,157,169,69,23,9,25,152,186,93,22,189,222,178,204,207,188,166,59,129,36,234,180,250,101,95,236,83,181,100,6,130,212,207,194,62,64,79,81,1,90,155,30,94,207]],
        //     },
        //     Commit {
        //         file_index: 2,
        //         roots: vec![vec![193,182,183,108,113,176,54,247,68,104,37,161,22,159,132,111,160,112,246,77,240,175,90,62,168,194,92,145,191,215,253,126,215,45,56,74,149,27,83,174,59,84,101,207,216,192,245,21,143,25,108,220,100,19,87,170,233,23,107,106,109,124,151,37],vec![150,178,155,192,41,106,74,152,128,237,99,216,27,12,100,167,135,137,120,7,80,117,24,115,102,148,239,157,32,207,153,247,68,135,135,223,236,59,229,2,210,198,216,65,187,35,199,52,169,196,232,8,207,163,245,208,199,225,151,13,145,253,166,166],vec![25,193,152,104,229,142,140,104,6,125,218,233,133,145,221,171,183,234,181,226,109,244,203,113,225,62,87,61,109,228,58,237,0,37,230,205,235,31,224,121,170,118,152,38,162,64,176,204,182,137,248,175,61,205,237,48,50,215,101,207,96,197,228,16],vec![186,182,232,29,162,88,37,110,89,36,85,90,183,99,82,23,153,9,11,42,119,169,139,126,83,7,151,123,99,176,132,51,62,1,28,220,58,93,16,9,134,118,40,171,110,184,55,141,192,207,118,200,192,216,47,95,182,131,118,237,173,54,171,82],vec![69,160,217,144,228,181,72,125,175,69,73,225,176,254,59,45,140,151,161,243,98,128,134,27,186,172,154,103,72,168,9,54,138,14,103,42,110,6,46,50,229,8,177,60,26,236,35,100,159,139,255,19,102,82,52,52,199,91,168,11,233,65,61,53],vec![130,166,23,102,202,185,129,189,246,217,29,49,178,125,250,132,156,115,31,117,14,163,227,22,153,246,98,228,144,10,176,195,166,178,251,129,155,132,252,162,72,90,64,50,11,153,97,53,207,183,161,118,73,19,191,183,222,53,41,35,42,20,232,191],vec![98,235,150,81,192,207,220,116,251,122,198,1,68,104,154,17,21,149,128,129,147,142,218,154,195,213,43,21,26,195,115,132,3,101,24,173,176,157,243,102,253,172,70,109,124,82,116,59,151,61,164,16,173,156,203,181,161,66,38,220,117,93,206,203],vec![134,69,10,158,55,60,43,19,204,35,222,225,149,78,28,15,239,121,219,231,40,231,181,42,43,255,2,242,60,114,168,6,60,21,137,61,10,107,166,0,16,173,92,17,84,56,254,234,201,143,73,69,171,234,23,50,85,242,54,135,97,23,87,33],vec![97,212,83,142,109,121,230,151,212,59,191,123,60,249,57,159,55,108,110,14,150,141,72,122,123,55,63,107,191,226,157,146,23,45,26,197,239,13,175,25,210,130,206,125,61,141,125,66,218,135,231,93,63,2,49,59,108,31,48,166,120,207,230,236]],
        //     },
        //     Commit {
        //         file_index: 3,
        //         roots: vec![vec![226,14,194,165,196,34,103,34,79,87,185,233,13,54,203,57,3,128,81,35,146,185,72,119,160,61,164,7,130,57,5,72,243,75,1,159,169,136,68,90,111,44,164,26,148,172,254,113,88,240,249,194,100,249,58,164,215,203,150,85,114,172,112,81],vec![62,6,115,121,0,112,186,154,69,189,146,228,115,242,88,38,243,10,74,0,53,45,108,245,113,132,252,168,127,82,196,200,244,150,228,142,245,159,115,103,219,123,68,59,29,208,254,55,14,65,204,16,110,7,145,146,172,164,168,94,234,180,46,170],vec![76,230,83,49,137,203,84,171,88,0,56,66,149,116,216,233,114,76,68,22,80,136,38,163,104,115,41,203,97,135,28,60,183,10,98,153,92,221,1,176,92,120,155,217,62,153,186,64,209,86,101,149,214,21,81,93,207,226,82,153,191,208,208,25],vec![205,46,139,12,162,95,227,197,101,139,138,112,0,104,65,89,61,72,192,54,236,141,75,65,118,54,191,165,15,46,121,12,102,11,214,111,162,165,161,4,184,102,100,234,247,95,212,67,217,57,74,119,110,217,29,212,222,165,234,160,19,57,179,76],vec![147,24,176,70,162,159,48,31,250,218,208,182,170,44,107,54,111,82,217,113,130,227,128,68,155,223,133,121,112,119,115,171,15,10,202,228,249,106,235,236,127,187,24,236,49,178,44,164,117,66,167,181,93,160,53,9,140,198,182,252,81,239,61,137],vec![94,200,11,213,219,226,182,115,250,134,229,91,74,239,69,245,215,212,56,143,140,233,33,138,117,248,13,222,56,170,57,221,243,140,85,137,193,73,101,241,99,28,246,228,239,144,235,75,14,145,215,196,21,218,117,124,94,55,134,29,223,140,131,135],vec![93,155,150,26,79,233,197,98,222,45,125,3,53,168,225,174,112,84,157,141,24,218,200,219,254,96,154,174,228,12,61,243,144,85,35,0,250,33,11,77,235,138,107,0,20,171,30,79,25,169,20,121,159,189,155,143,182,238,208,196,54,138,192,106],vec![105,246,24,7,144,244,51,186,164,115,54,137,48,39,3,65,243,33,38,179,45,214,125,41,74,230,132,183,114,170,68,239,87,59,225,100,87,220,89,3,194,130,181,139,104,204,235,124,116,221,29,185,136,204,204,172,249,161,199,147,178,67,217,174],vec![148,238,184,112,190,229,233,123,43,35,37,146,75,68,230,210,135,193,101,187,125,176,208,123,84,46,152,171,74,180,133,192,135,42,210,208,50,222,122,89,181,71,108,31,180,225,41,190,187,41,31,115,185,17,18,129,230,230,179,28,0,159,157,231]],
        //     },
        //     Commit {
        //         file_index: 4,
        //         roots: vec![vec![140,214,205,20,172,245,37,232,211,18,118,81,153,71,163,19,218,4,250,161,71,183,29,6,23,207,157,239,143,203,141,232,234,125,34,150,181,93,148,151,157,126,62,228,139,186,251,82,175,196,91,226,0,103,136,123,251,109,106,78,189,240,99,130],vec![134,230,148,135,120,160,109,226,45,134,115,153,216,28,5,175,207,223,39,77,252,224,210,40,176,202,61,201,91,145,199,113,178,6,39,239,12,178,142,155,2,181,255,58,180,49,149,142,174,47,64,43,185,88,152,206,99,178,205,36,32,77,241,151],vec![241,252,28,113,69,153,159,97,50,158,25,226,100,163,64,3,81,93,22,77,162,184,78,181,231,187,228,86,182,21,202,110,130,3,0,81,166,200,41,127,161,139,43,120,60,167,212,28,87,216,112,70,231,83,138,194,220,29,247,142,44,89,79,78],vec![177,139,148,15,101,201,79,71,1,131,200,95,209,111,82,130,131,251,215,147,145,131,251,135,201,70,192,93,166,226,52,88,82,205,13,133,233,62,210,106,128,255,110,148,189,188,157,96,82,132,56,119,31,24,196,222,70,217,213,25,14,248,45,226],vec![163,160,248,205,166,117,114,244,116,40,13,178,115,101,88,214,194,206,151,64,194,23,190,158,121,100,175,107,141,57,101,218,96,21,181,18,207,97,5,145,16,140,37,132,130,237,82,56,197,13,129,143,147,83,89,73,103,172,234,194,227,202,156,84],vec![162,156,83,8,109,136,70,57,22,133,5,104,229,120,79,33,132,116,38,27,195,59,147,96,27,228,222,29,3,142,162,120,88,195,176,32,70,245,94,243,230,177,121,206,98,17,236,105,163,235,74,213,138,8,56,103,168,182,222,165,115,96,161,115],vec![15,76,96,245,186,241,156,209,234,126,134,18,242,118,243,228,112,237,69,1,51,228,218,151,107,122,252,213,124,127,138,106,28,107,209,168,113,4,25,53,176,120,116,234,150,251,88,134,163,106,228,220,129,88,131,4,123,217,33,171,41,40,131,138],vec![1,171,137,194,143,4,107,174,16,205,134,156,123,128,11,97,219,170,6,132,34,158,76,221,175,56,161,132,182,153,178,52,65,192,102,172,61,34,137,206,52,41,222,99,27,176,149,110,90,44,36,162,83,206,217,174,2,225,246,19,41,248,201,50],vec![71,73,144,35,36,77,242,109,0,175,14,245,218,181,186,180,115,125,207,121,51,104,210,234,150,59,77,198,120,21,114,50,132,37,6,246,189,52,96,251,168,4,233,89,217,33,41,153,50,210,150,70,175,159,167,142,101,26,38,48,199,210,84,77]],
        //     },
        // ];
        let mut commits = call_get_commits(4, key_n.clone(), key_g.clone(), k, n, d);
        let chal = call_generate_commit_challenge(4, &mut commits, key_n, key_g, k, n, d);

        println!("challenge: {:?}", chal);
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
