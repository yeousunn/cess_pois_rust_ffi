use gmp::mpz::Mpz;
use std::os::raw::{
    c_int, c_uchar, c_long
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

#[repr(C)]
pub struct ExpandersC {
    pub k: i64,
    pub n: i64,
    pub d: i64,
    pub size: i64,
}

#[repr(C)]
pub struct ProverNodeC {
    pub id: *mut u8,
    pub commits_buf: *mut *mut CommitC,
    pub buf_size: i32,
    pub acc: *mut u8,
    pub count: i64,
}

#[repr(C)]
pub struct RsaKeyC {
    pub n: Mpz,
    pub g: Mpz,
}
// pub struct RsaKeyC {
//     pub n: i64,
//     pub g: i64,
// }

#[repr(C)]
pub struct VerifierC {
    pub key: RsaKeyC,
    pub expanders: ExpandersC,
    pub nodes: *mut *mut ProverNodeC,
    pub nodes_count: i32,
}

#[repr(C)]
pub struct MhtProofC {
    index: NodeType,
    label: *mut c_uchar,
    paths: *mut *mut c_uchar,
    locs: *mut c_uchar,
}

#[repr(C)]
pub struct CommitProofC {
    node: *mut MhtProofC,
    parents: *mut *mut MhtProofC,
    parents_count: c_int,
}

#[repr(C)]
pub struct MyByte {
    pub b: *mut u8,
    pub length: c_long,
}
