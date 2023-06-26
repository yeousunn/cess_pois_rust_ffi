use crate::types::RsaKey;
use libloading::Library;
use num_bigint_dig::{BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::One;
use rsa::traits::PublicKeyParts;
use rsa::RsaPrivateKey;

pub fn load_library() -> Library {
    unsafe { Library::new("cgo/main.so").expect("Failed to load the dynamic library") }
}

pub fn c_pointer_to_i64_array_of_array(
    c_arrays: *const *const i64,
    c_lengths: *const i64,
    main_array_length: i64,
) -> Vec<Vec<i64>> {
    let mut arr_of_arr: Vec<Vec<i64>> = Vec::new();
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
