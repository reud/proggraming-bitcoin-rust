use ripemd160::{Ripemd160, Digest};
use ripemd160::digest::DynDigest;

// sha256 -> sha256 -> ripemd160 (used by bitcoin address)
pub fn hash160(v: Vec<u8>) -> Vec<u8> {
    let sha256r1 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*v);
    let sha256r2 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*sha256r1);
    let mut hasher = Ripemd160::new();
    hasher.update(sha256r2);
    hasher.finalize().to_vec()
}