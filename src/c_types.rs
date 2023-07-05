use std::os::raw::{
    c_int, c_long, c_char, c_uchar, 
};

pub type NodeType = c_int;

// If pass as []CommitC
// Remember to pass length of the array []CommitC
#[repr(C)]
pub struct CommitC {
    pub file_index: i64,
    // roots [][] byte
    pub roots: *mut *mut u8,

    // Length of root [][]byte
    pub roots_length: i32,

    // Length of sub element of root [][]byte
    pub sub_roots_lengths: *mut i32,
}

// #[repr(C)]
// pub struct ExpandersC {
//     pub k: i64,
//     pub n: i64,
//     pub d: i64,
//     pub size: i64,
// }

// #[repr(C)]
// pub struct ProverNodeC {
//     pub id: *mut u8,
//     pub commits_buf: *mut *mut CommitC,
//     pub buf_size: i32,
//     pub acc: *mut u8,
//     pub count: i64,
// }

// #[repr(C)]
// pub struct RsaKeyC {
//     pub n: Mpz,
//     pub g: Mpz,
// }
// // pub struct RsaKeyC {
// //     pub n: i64,
// //     pub g: i64,
// // }

// #[repr(C)]
// pub struct VerifierC {
//     pub key: RsaKeyC,
//     pub expanders: ExpandersC,
//     pub nodes: *mut *mut ProverNodeC,
//     pub nodes_count: i32,
// }

#[derive(Debug)]
#[repr(C)]
pub struct MhtProofC {
    pub index: NodeType,
    pub label: *mut c_uchar,
    pub label_length: c_int,

    pub paths: *mut *mut c_uchar,
    pub sub_paths_lengths: *mut c_int,
    pub path_length: c_int,

    pub locs: *mut c_uchar,
    pub locs_length: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct CommitProofC {
    pub node: *mut MhtProofC,

    // This is array of pointers
    // []*MhtProof so it is single dimension
    // and require only the length of array when passing to C
    pub parents: *mut *mut MhtProofC,
    pub parents_length: c_int,
}

#[repr(C)]
pub struct MyByte {
    pub b: *mut u8,
    pub length: c_long,
}

#[repr(C)]
pub struct CommonParam {
    pub key_n: *mut c_char,
    pub key_g: *mut c_char,
    pub k: i64,
    pub n: i64,
    pub d: i64,
}

#[repr(C)]
pub struct ProverID {
    pub id: *mut c_char, // Pointer to ID []byte
    pub length: c_int, // Length of ID []byte
}

// Some types that can be represented by [][]int64 are
// Challenge
// [][]int64
#[repr(C)]
pub struct I64ArrOfArr {
    pub main_array: *mut *mut i64,
    pub sub_array_lengths: *mut i32,
    pub length: i32,
}

// i32ArrOfArr
#[repr(C)]
pub struct I32ArrOfArr {
    pub main_array: *mut *mut i32,
    pub sub_array_length: *mut i32,
    pub length: i32,
}