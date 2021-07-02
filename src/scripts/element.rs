use std::ops::{Sub, Add, Mul, Rem, Div, BitAnd, BitOr};
use num_bigint::{BigInt, BigUint};
use num_traits::{Zero, Signed, ToPrimitive};
use crate::helper::helper::{hash256, hash160, sha1, sha256, ripemd160};

#[derive(Debug,Clone,Ord, PartialOrd, Eq, PartialEq)]
pub struct Element {
    pub(crate) inner_data: Vec<u8>
}

impl Element {
    #[allow(dead_code)]
    pub fn hash256(self) -> Element {
        return Element {
            inner_data: hash256(self.inner_data)
        }
    }

    #[allow(dead_code)]
    pub fn hash160(self) -> Element {
        return Element {
            inner_data: hash160(self.inner_data)
        }
    }

    #[allow(dead_code)]
    pub fn ripemd160(self) -> Element {
        Element {
            inner_data: ripemd160(self.inner_data)
        }
    }

    #[allow(dead_code)]
    pub fn sha1(self) -> Element {
        Element {
            inner_data: sha1(self.inner_data)
        }
    }

    #[allow(dead_code)]
    pub fn sha256(self) -> Element {
        Element {
            inner_data: sha256(self.inner_data)
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        return self.inner_data.is_empty()
    }
    // 新たなオブジェクトを生成する。
    #[allow(dead_code)]
    pub fn reverse(&self) -> Element {
        let mut result = self.clone();
        result.inner_data.reverse();
        return result
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.inner_data.len()
    }

    fn encode_num(num: BigInt) -> Element {
        if num == BigInt::zero() {
            return new_element();
        }
        let mut v:Vec<u8> = vec![];
        let mut abs_num = num.abs().to_biguint().unwrap();
        let negative = num < BigInt::zero();

        while abs_num.clone() != BigUint::zero() {
            let and = abs_num.clone() & BigUint::from(0xff as u8);
            v.push(and.to_u8().unwrap());
            abs_num >>= 8;
        }

        let last_idx = v.len()-1;
        if (v[last_idx] & 0x80) != 0 {
            if negative {
                v.push(0x80);
            } else {
                v.push(0);
            }
        }
        else if negative {
            v[last_idx] |= 0x80
        }

        return new_element_from_bytes(v);
    }

    fn decode_num(element: Element) -> BigInt {
        if element.is_empty() {
            return BigInt::zero();
        }
        let big_endian = element.reverse();
        let mut negative = false;
        let mut result = BigInt::zero();

        if (big_endian.inner_data[0] & 0x80) != 0 {
            negative = true;
            let and = big_endian.inner_data[0] & 0x7f;
            result += BigInt::from(and);
        } else {
            result += BigInt::from(big_endian.inner_data[0]);
        }

        let mut i = 1;
        while i < big_endian.inner_data.len() {
            result <<= 8;
            result += BigInt::from(big_endian.inner_data[i]);
            i += 1;
        }
        if negative {
            return -result
        }
        return result;
    }

}
pub fn new_element() -> Element {
    Element {
        inner_data: vec![]
    }
}

pub fn new_element_from_bytes(bytes: Vec<u8>) -> Element {
    Element {
        inner_data: bytes
    }
}

impl Add for Element {
    type Output = Element;

    fn add(self, rhs: Self) -> Self::Output {
        let l = Element::decode_num(self);
        let r = Element::decode_num(rhs);
        let el = Element::encode_num(l + r);
        return el;
    }
}

impl Sub for Element {
    type Output = Element;

    fn sub(self, rhs: Self) -> Self::Output {
        let l = Element::decode_num(self);
        let r = Element::decode_num(rhs);
        let el = Element::encode_num(l - r);
        return el;
    }
}

impl Mul for Element {
    type Output = Element;

    fn mul(self, rhs: Self) -> Self::Output {
        let l = Element::decode_num(self);
        let r = Element::decode_num(rhs);
        let el = Element::encode_num(l * r);
        return el;
    }
}

impl Rem for Element {
    type Output = Element;

    fn rem(self, rhs: Self) -> Self::Output {
        let l = Element::decode_num(self);
        let r = Element::decode_num(rhs);
        let el = Element::encode_num(l % r);
        return el;
    }
}

impl Div for Element {
    type Output = Element;

    fn div(self, rhs: Self) -> Self::Output {
        let l = Element::decode_num(self);
        let r = Element::decode_num(rhs);
        let el = Element::encode_num(l / r);
        return el;
    }
}

impl BitAnd for Element {
    type Output = Element;

    fn bitand(self, rhs: Self) -> Self::Output {
        let l = Element::decode_num(self);
        let r = Element::decode_num(rhs);
        let el = Element::encode_num(l & r);
        return el;
    }
}

impl  BitOr for Element {
    type Output = Element;

    fn bitor(self, rhs: Self) -> Self::Output {
        let l = Element::decode_num(self);
        let r = Element::decode_num(rhs);
        let el = Element::encode_num(l | r);
        return el;
    }
}