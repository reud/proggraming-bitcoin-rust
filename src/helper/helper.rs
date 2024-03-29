use crypto::digest::Digest as D;
use crypto::sha1::Sha1;
use num_bigint::BigUint;
use ripemd160::{Digest, Ripemd160};
use std::io::{Cursor, Read};
use std::num::ParseIntError;

#[allow(dead_code)]
pub fn hash256(v: Vec<u8>) -> Vec<u8> {
    let sha256r1 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*v);
    let sha256r2 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*sha256r1);
    return sha256r2;
}

#[allow(dead_code)]
pub fn sha256(v: Vec<u8>) -> Vec<u8> {
    crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*v)
}

#[allow(dead_code)]
pub fn hash160(v: Vec<u8>) -> Vec<u8> {
    let sha256 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*v);
    ripemd160(sha256)
}

#[allow(dead_code)]
pub fn ripemd160(v: Vec<u8>) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    Digest::update(&mut hasher, v);
    hasher.finalize().to_vec()
}

#[allow(dead_code)]
pub fn sha1(v: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.input(&*v);
    let mut ret: Vec<u8> = vec![];
    hasher.result(&mut *ret);
    return ret;
}

// for test
#[allow(dead_code)]
pub fn u8vec_to_str(v: Vec<u8>) -> String {
    let mut ret = "".to_string();
    for x in v {
        ret += &*format!("{:02x}", x);
    }
    return ret;
}

#[allow(dead_code)]
pub fn vector_as_u8_4_array(vector: Vec<u8>) -> [u8; 4] {
    let mut arr = [0u8; 4];
    for (place, element) in arr.iter_mut().zip(vector.iter()) {
        *place = *element;
    }
    arr
}
#[allow(dead_code)]
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[allow(dead_code)]
pub fn read_varint(c: &mut Cursor<Vec<u8>>) -> u64 {
    let mut i = [0u8];

    if c.read(&mut i).is_err() {
        panic!("failed to read varint")
    }

    let i = i[0];
    if i == 0xfd {
        let mut bytes = [0u8; 2];
        if c.read(&mut bytes).is_err() {
            panic!("failed to read bytes")
        }
        return u16::from_le_bytes(bytes) as u64;
    }
    if i == 0xfe {
        let mut bytes = [0u8; 4];
        if c.read(&mut bytes).is_err() {
            panic!("failed to read bytes")
        }
        return u32::from_le_bytes(bytes) as u64;
    }
    if i == 0xff {
        let mut bytes = [0u8; 8];
        if c.read(&mut bytes).is_err() {
            panic!("failed to read bytes")
        }
        return u64::from_le_bytes(bytes);
    }
    return i as u64;
}

#[allow(dead_code)]
pub fn encode_varint(i: u128) -> Vec<u8> {
    if i < 0xfd {
        return vec![i as u8];
    }
    if i < 0x10000 {
        let mut v = vec![0xfdu8];
        for x in (i as u16).to_le_bytes().iter() {
            v.push(*x);
        }
        assert_eq!(v.len(), 3);
        return v;
    }
    if i < 0x100000000 {
        let mut v = vec![0xfeu8];
        for x in (i as u32).to_le_bytes().iter() {
            v.push(*x);
        }
        assert_eq!(v.len(), 5);
        return v;
    }
    if i < 0x10000000000000000 {
        let mut v = vec![0xffu8];
        for x in i.to_le_bytes().iter() {
            v.push(*x);
        }
        assert_eq!(v.len(), 9);
        return v;
    }
    panic!("integer too large: {}", i);
}

#[allow(dead_code)]
pub fn biguint_to_32_bytes_be(num: BigUint) -> [u8; 32] {
    let mut r = biguint_to_32_bytes_le(num);
    r.reverse();
    return r;
}

#[allow(dead_code)]
pub fn biguint_to_32_bytes_le(num: BigUint) -> [u8; 32] {
    let mut ret = [0u8; 32];
    let bin = num.to_bytes_le();
    if bin.len() > 32 {
        return ret;
    }
    let mut i = 0;
    while i < bin.len() {
        ret[i] = bin[i];
        i += 1;
    }
    return ret;
}

// ref: https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
pub fn str_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn lstip_bytes(bytes: Vec<u8>, byte: u8) -> Vec<u8> {
    for (i, v) in bytes.iter().enumerate() {
        if *v != byte {
            return bytes[i..].to_vec();
        }
    }
    return vec![];
}
