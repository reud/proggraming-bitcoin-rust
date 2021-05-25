

#[allow(dead_code)]
pub fn hash256(v: Vec<u8>) -> Vec<u8> {
    let sha256r1 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*v);
    let sha256r2 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*sha256r1);
    return sha256r2;
}