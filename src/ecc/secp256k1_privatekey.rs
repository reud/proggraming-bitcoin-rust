use crate::ecc::secp256k1_scalar_element::{Secp256k1ScalarElement, new_secp256k1scalarelement};
use crate::ecc::secp256k1_point::{Secp256k1Point, new_secp256k1point_g};
use crate::ecc::secp256k1_signature::{Secp256k1Signature, new_secp256k1signature};
use rand::{thread_rng};
use num_bigint::{RandBigInt, BigUint};
use num_traits::{One, FromPrimitive, Num};

// 楕円曲線上の加算に対しての有限体の位数はこの値となる。
fn prime() -> BigUint {
    return BigUint::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",16).unwrap()
}

#[derive(Debug, Clone)]
pub struct Secp256k1PrivateKey {
    secret: Secp256k1ScalarElement,
    point: Secp256k1Point
}

#[allow(dead_code)]
pub fn new_secp_256k1privatekey(secret: Secp256k1ScalarElement) -> Secp256k1PrivateKey {
    return Secp256k1PrivateKey {
        secret: secret.clone(),
        point: new_secp256k1point_g().mul_from_sec256k1scalar_element(secret)
    }
}

impl Secp256k1PrivateKey {
    #[allow(dead_code)]
    pub fn hex(self) -> String {
        self.secret.num.to_str_radix(16)
    }
    #[allow(dead_code)]
    pub fn sign(self,z: Secp256k1ScalarElement) -> Secp256k1Signature {
        let mut generator = thread_rng();
        let k = generator.gen_biguint_below(&(prime() - BigUint::one()));
        let k = new_secp256k1scalarelement(k.clone());
        let r = new_secp256k1point_g().mul_from_sec256k1scalar_element(k.clone()).x;
        let r = new_secp256k1scalarelement(r.num);
        let mut s = (z + (r.clone() * self.secret)) / k;
        if s.num > new_secp256k1scalarelement(prime() / BigUint::from_u8(2u8).unwrap()).num {
            s = new_secp256k1scalarelement(prime()) - s;
        }
        return new_secp256k1signature(r,s);
    }

}


#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn test_sel() {
        // todo: python3のP76実行結果を比較しつつ Biguint to 32 bytes bigendianを実装する。
        let a = BigUint::from(500u64);
        for v in a.to_bytes_be() {
            print!("{} ",v);
        }
    }
}