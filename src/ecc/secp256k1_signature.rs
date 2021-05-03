use std::fmt::{Display, Formatter};
use std::fmt;
use crate::ecc::secp256k1_scalar_element::{Secp256k1ScalarElement, new_secp256k1scalarelement_from_hex_str};

pub struct Secp256k1Signature {
    pub(crate) r: Secp256k1ScalarElement,
    pub(crate) s: Secp256k1ScalarElement
}

impl Display for Secp256k1Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"Signature({},{})",self.r,self.s)
    }
}

#[allow(dead_code)]
pub fn new_secp256k1signature_from_str(r: &str,s: &str) -> Option<Secp256k1Signature> {
    let r = new_secp256k1scalarelement_from_hex_str(r);
    let s = new_secp256k1scalarelement_from_hex_str(s);
    if r.is_none() || s.is_none(){
        return None;
    }
    return Some(Secp256k1Signature{
        r: r.unwrap(),
        s: s.unwrap()
    })
}

pub fn new_secp256k1signature(r: Secp256k1ScalarElement,s: Secp256k1ScalarElement) -> Secp256k1Signature {
    return Secp256k1Signature{
        r,
        s
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crypto_hash::{hex_digest, Algorithm, digest};
    use crate::ecc::secp256k1_point::new_secp256k1point_g;
    use crate::ecc::secp256k1_scalar_element::{new_secp256k1scalarelement, new_secp256k1scalarelement_from_i32, new_secp256k1scalarelement_from_u64};

    #[test]
    fn test_signature_practice_p71q7() {
        let e = new_secp256k1scalarelement_from_i32(12345);
        let z= digest(Algorithm::SHA256, b"Programming Bitcoin!");
        let z= hex_digest(Algorithm::SHA256, &*z);
        let z = new_secp256k1scalarelement_from_hex_str(&*z).unwrap();
        let k = new_secp256k1scalarelement_from_u64(1234567890u64);
        let g = new_secp256k1point_g().mul_from_sec256k1scalar_element(k.clone());
        let r = new_secp256k1scalarelement(g.clone().x.num);
        let s = (z.clone()+(r.clone()*e)) / k.clone();
        println!("z: {}",z.num.to_str_radix(16));
        assert_eq!("969f6056aa26f7d2795fd013fe88868d09c9f6aed96965016e1936ae47060d48",z.num.to_str_radix(16));
        println!("r: {}",r.num.to_str_radix(16));
        assert_eq!("2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22",r.num.to_str_radix(16));
        println!("s: {}",s.num.to_str_radix(16));
        assert_eq!("1dbc63bfef4416705e602a7b564161167076d8b20990a0f26f316cff2cb0bc1a",s.num.to_str_radix(16));
    }
}