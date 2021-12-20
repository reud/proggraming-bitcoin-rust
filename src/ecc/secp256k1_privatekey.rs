use crate::ecc::encode::encode_base58_checksum;
use crate::ecc::secp256k1_point::{new_secp256k1point_g, Secp256k1Point};
use crate::ecc::secp256k1_scalar_element::{
    new_secp256k1scalarelement, new_secp256k1scalarelement_from_i32, Secp256k1ScalarElement,
};
use crate::ecc::secp256k1_signature::{new_secp256k1signature, Secp256k1Signature};
use num_bigint::{BigUint, RandBigInt};
use num_traits::{FromPrimitive, Num, One};
use rand::thread_rng;

// 楕円曲線上の加算に対しての有限体の位数はこの値となる。
fn prime() -> BigUint {
    return BigUint::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    )
    .unwrap();
}

#[derive(Debug, Clone)]
pub struct Secp256k1PrivateKey {
    // 秘密鍵
    secret: Secp256k1ScalarElement,
    // 公開鍵(秘密鍵から自動で算出される)
    pub(crate) point: Secp256k1Point,
}

#[allow(dead_code)]
pub fn new_secp_256k1privatekey(secret: Secp256k1ScalarElement) -> Secp256k1PrivateKey {
    return Secp256k1PrivateKey {
        secret: secret.clone(),
        point: new_secp256k1point_g().mul_from_sec256k1scalar_element(secret),
    };
}

#[allow(dead_code)]
pub fn new_secp_256k1privatekey_from_biguint(secret: BigUint) -> Secp256k1PrivateKey {
    return Secp256k1PrivateKey {
        secret: new_secp256k1scalarelement(secret.clone()),
        point: new_secp256k1point_g().mul_from_big_uint(secret),
    };
}

#[allow(dead_code)]
pub fn new_secp_256k1privatekey_from_i32(secret: i32) -> Secp256k1PrivateKey {
    return Secp256k1PrivateKey {
        secret: new_secp256k1scalarelement_from_i32(secret),
        point: new_secp256k1point_g().mul_from_i32(secret),
    };
}
impl Secp256k1PrivateKey {
    #[allow(dead_code)]
    pub fn hex(self) -> String {
        self.secret.num.to_str_radix(16)
    }
    #[allow(dead_code)]
    // zは署名先のハッシュ(p.66)だったりメッセージだったり
    pub fn sign(self, z: Secp256k1ScalarElement) -> Secp256k1Signature {
        let mut generator = thread_rng();
        let k = generator.gen_biguint_below(&(prime() - BigUint::one()));
        let k = new_secp256k1scalarelement(k.clone());
        let r = new_secp256k1point_g()
            .mul_from_sec256k1scalar_element(k.clone())
            .x;
        let r = new_secp256k1scalarelement(r.num);
        let mut s = (z + (r.clone() * self.secret)) / k;
        if s.num > new_secp256k1scalarelement(prime() / BigUint::from_u8(2u8).unwrap()).num {
            s = new_secp256k1scalarelement(prime()) - s;
        }
        return new_secp256k1signature(r, s);
    }
    #[allow(dead_code)]
    pub fn wif(self, compressed: bool, testnet: bool) -> String {
        let mut result = self.secret.to_32_bytes_be().unwrap();
        if testnet {
            result.insert(0, 0xefu8);
        } else {
            result.insert(0, 0x80u8);
        }
        if compressed {
            result.push(1u8);
        }
        return encode_base58_checksum(result);
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::ecc::secp256k1_scalar_element::new_secp256k1scalarelement_from_hex_str;

    #[test]
    fn test_sel() {
        // todo: python3のP76実行結果を比較しつつ Biguint to 32 bytes bigendianを実装する。
        let a = BigUint::from(500u64);
        for v in a.to_bytes_be() {
            print!("{} ", v);
        }
    }

    #[test]
    fn test_wif_p87q6() {
        {
            let el = new_secp256k1scalarelement_from_i32(5003);
            let pk = new_secp_256k1privatekey(el);
            let result = pk.wif(true, true);
            assert_eq!(
                result,
                "cMahea7zqjxrtgAbB7LSGbcQUr1uX1ojuat9jZodMN8rFTv2sfUK"
            )
        }
        {
            let el = new_secp256k1scalarelement_from_i32(2021).pow(BigUint::from(5u8));
            let pk = new_secp_256k1privatekey(el);
            let result = pk.wif(false, true);
            assert_eq!(
                result,
                "91avARGdfge8E4tZfYLoxeJ5sGBdNJQH4kvjpWAxgzczjbCwxic"
            )
        }
        {
            let el = new_secp256k1scalarelement_from_hex_str("54321deadbeef").unwrap();
            let pk = new_secp_256k1privatekey(el);
            let result = pk.wif(true, false);
            assert_eq!(
                result,
                "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgiuQJv1h8Ytr2S53a"
            )
        }
    }
}
