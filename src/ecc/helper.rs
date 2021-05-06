use ripemd160::{Ripemd160, Digest};

// sha256 -> sha256 -> ripemd160 (used by bitcoin address)
pub fn hash160(v: Vec<u8>) -> Vec<u8> {
    let sha256 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*v);
    let mut hasher = Ripemd160::new();
    Digest::update(&mut hasher,sha256);
    hasher.finalize().to_vec()
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