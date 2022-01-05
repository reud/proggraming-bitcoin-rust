use crate::ecc::encode;
use crate::ecc::secp256k1_scalar_element::{
    new_secp256k1scalarelement, new_secp256k1scalarelement_from_u64,
};
use crate::helper::helper::hash256;
use num_bigint::BigUint;
use num_traits::Zero;
use std::ops::{Add, Mul};

// FIXME: 遅い！
pub fn char_to_base58_index(c: char) -> Result<u8, &'static str> {
    for (i, val) in encode::BASE58_ALPHABET.chars().enumerate() {
        if c == val {
            return Ok(i as u8);
        }
    }
    return Err("failed to find a character in base58");
}

pub fn address_decode_base58(addr: String) -> Result<Vec<u8>, String> {
    let mut v = new_secp256k1scalarelement(BigUint::zero());
    let _58 = new_secp256k1scalarelement(BigUint::from(58u32));
    for c in addr.chars() {
        v = v.mul(_58.clone());
        let idx = char_to_base58_index(c);
        match idx {
            Ok(idx) => {
                v = v.add(new_secp256k1scalarelement_from_u64(idx as u64));
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
    let combined = v.to_n_bytes_be(25);
    return match combined {
        None => Err("failed to cast bytes (at Secp256k1ScalarElement::to_n_bytes_be)".to_string()),
        Some(combined) => {
            let checksum = combined[combined.len() - 4..].to_vec();
            let striped = combined[..combined.len() - 4].to_vec();
            if hash256(striped.clone())[..4] != checksum {
                return Err(format!(
                    "bad address: {:?} \nvs \n{:?}",
                    checksum.to_vec(),
                    hash256(striped.clone())[..4].to_vec(),
                ));
            }
            Ok(combined[1..combined.len() - 4].to_owned()) // hash160
        }
    };
}
