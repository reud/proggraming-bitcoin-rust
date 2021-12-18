use crate::ecc::secp256k1_scalar_element::{
    new_secp256k1scalarelement, new_secp256k1scalarelement_from_hex_str, Secp256k1ScalarElement,
};
use crate::helper::helper::read_varint;
use num_bigint::{BigInt, BigUint};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read};

pub struct Secp256k1Signature {
    pub(crate) r: Secp256k1ScalarElement,
    pub(crate) s: Secp256k1ScalarElement,
}

impl Display for Secp256k1Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Signature({},{})", self.r, self.s)
    }
}

impl Secp256k1Signature {
    #[allow(dead_code)]
    fn der_str(self) -> String {
        let mut ret = "".to_string();
        for v in self.der() {
            ret += &*format!("{:02x}", v);
        }
        return ret;
    }

    #[allow(dead_code)]
    pub fn parse(der_signature: Vec<u8>) -> Secp256k1Signature {
        let mut s = Cursor::new(der_signature.clone());

        let mut bytes = [0u8; 1];
        s.read(&mut bytes);
        let compound = bytes[0];

        if compound != 0x30 {
            panic!("Bad Signature")
        }

        s.read(&mut bytes);
        let length = bytes[0];
        if (length + 2) as usize != der_signature.len() {
            panic!("Bad Signature Length")
        }

        s.read(&mut bytes);
        let marker = bytes[0];
        if marker != 0x02 {
            panic!("Bad Signature")
        }

        s.read(&mut bytes);
        let rlength = bytes[0];

        let mut sret: Vec<u8> = vec![0u8; rlength as usize];
        s.read(&mut *sret);
        let r = BigUint::from_bytes_be(&*sret);
        let r = new_secp256k1scalarelement(r);

        s.read(&mut bytes);
        let marker = bytes[0];
        if marker != 0x02 {
            panic!("Bad Signature")
        }

        s.read(&mut bytes);
        let slength = bytes[0];

        let mut sret: Vec<u8> = vec![0u8; slength as usize];
        s.read(&mut *sret);
        let s = BigUint::from_bytes_be(&*sret);
        let s = new_secp256k1scalarelement(s);
        return new_secp256k1signature(r, s);
    }

    #[allow(dead_code)]
    fn der(self) -> Vec<u8> {
        let prefix_marker = 0x30u8;
        let r_bytes = self.r.num.to_bytes_be();
        let s_bytes = self.s.num.to_bytes_be();
        let mut r_sel: Vec<u8> = vec![];
        let mut s_sel: Vec<u8> = vec![];
        if r_bytes[0] >= 0x80u8 {
            r_sel.push(0u8);
        }
        if s_bytes[0] >= 0x80u8 {
            s_sel.push(0u8);
        }

        for rnum in r_bytes {
            r_sel.push(rnum);
        }
        r_sel.insert(0, r_sel.len() as u8); // size
        r_sel.insert(0, 2u8); // marker
        for snum in s_bytes {
            s_sel.push(snum);
        }
        s_sel.insert(0, s_sel.len() as u8);
        s_sel.insert(0, 2u8); //marker

        let size = (r_sel.len() + s_sel.len()) as u8;

        let mut ret = vec![prefix_marker];
        ret.push(size);

        for v in r_sel {
            ret.push(v);
        }

        for v in s_sel {
            ret.push(v);
        }

        return ret;
    }
}

#[allow(dead_code)]
pub fn new_secp256k1signature_from_str(r: &str, s: &str) -> Option<Secp256k1Signature> {
    let r = new_secp256k1scalarelement_from_hex_str(r);
    let s = new_secp256k1scalarelement_from_hex_str(s);
    if r.is_none() || s.is_none() {
        return None;
    }
    return Some(Secp256k1Signature {
        r: r.unwrap(),
        s: s.unwrap(),
    });
}

pub fn new_secp256k1signature(
    r: Secp256k1ScalarElement,
    s: Secp256k1ScalarElement,
) -> Secp256k1Signature {
    return Secp256k1Signature { r, s };
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::ecc::secp256k1_point::new_secp256k1point_g;
    use crate::ecc::secp256k1_scalar_element::{
        new_secp256k1scalarelement, new_secp256k1scalarelement_from_i32,
        new_secp256k1scalarelement_from_u64,
    };
    use crypto_hash::{digest, hex_digest, Algorithm};

    #[test]
    fn test_signature_practice_p71q7() {
        let e = new_secp256k1scalarelement_from_i32(12345);
        let z = digest(Algorithm::SHA256, b"Programming Bitcoin!");
        let z = hex_digest(Algorithm::SHA256, &*z);
        let z = new_secp256k1scalarelement_from_hex_str(&*z).unwrap();
        let k = new_secp256k1scalarelement_from_u64(1234567890u64);
        let g = new_secp256k1point_g().mul_from_sec256k1scalar_element(k.clone());
        let r = new_secp256k1scalarelement(g.clone().x.num);
        let s = (z.clone() + (r.clone() * e)) / k.clone();
        println!("z: {}", z.num.to_str_radix(16));
        assert_eq!(
            "969f6056aa26f7d2795fd013fe88868d09c9f6aed96965016e1936ae47060d48",
            z.num.to_str_radix(16)
        );
        println!("r: {}", r.num.to_str_radix(16));
        assert_eq!(
            "2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22",
            r.num.to_str_radix(16)
        );
        println!("s: {}", s.num.to_str_radix(16));
        assert_eq!(
            "1dbc63bfef4416705e602a7b564161167076d8b20990a0f26f316cff2cb0bc1a",
            s.num.to_str_radix(16)
        );
    }

    #[test]
    fn test_der_format_p83q3() {
        let r = new_secp256k1scalarelement_from_hex_str(
            "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
        )
        .unwrap();
        let s = new_secp256k1scalarelement_from_hex_str(
            "8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
        )
        .unwrap();
        let sig = new_secp256k1signature(r, s);
        println!("sig: {}", sig.der_str());
    }
}
