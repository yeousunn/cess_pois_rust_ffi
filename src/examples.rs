
use crate::{c_types::MyByte, utils::c_pointer_to_i32_array_of_array};
use std::os::raw::{c_char, c_int, c_long};
use libloading::Symbol;

use crate::utils::load_library; // Dummy Type for testing
type GetByteArrayFunc = unsafe extern "C" fn(*mut c_char, c_long);
type GetByteArrayAsStructFunc = unsafe extern "C" fn(*mut MyByte);
type GetByteArrayAsStructArrayFunc = unsafe extern "C" fn(*mut MyByte, c_long);

/* [][]byte as parameter 
* The [][]byte array need three parameters
*   1. pointer to main array
*   2. length of the main array
*   3. pointer to array containing the length of each sub array
 */ 
type GetByteArrayOfArrayFunc = unsafe extern "C" fn(*mut *mut u8, c_int, *mut c_int);

type GetArrayFunc = extern "C" fn() -> (*mut c_int, *mut c_int);
type FreeArrayFunc = extern "C" fn(*mut c_int);
type GetArrayOfArrayFunc = extern "C" fn() -> (*mut *mut i32, *mut i32, i32);

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