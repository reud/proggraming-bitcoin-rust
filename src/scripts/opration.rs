use crate::scripts::element::{Element, new_element, new_element_from_bytes};
use crate::scripts::stack::Stack;
use num_bigint::{BigUint, BigInt};
use num_traits::{Zero, Signed, ToPrimitive};

#[allow(dead_code)]
pub struct Operations {
}

fn encode_num(mut num: BigInt) -> Element {
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

fn decode_num(mut element: Element) -> BigInt {
    if element.is_empty() {
        return BigInt::zero();
    }
    let mut big_endian = element.reverse();
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

impl Operations {
    #[allow(dead_code)]
    pub fn code_functions(code: u8) -> Option<fn(&mut Stack<Element>) -> bool> {
        return match code {
            0x76 => Some(Operations::op_dup),
            0xaa => Some(Operations::op_hash256),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn op_dup(stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }
        let top = stack.top().unwrap();
        stack.push(top);
        return true;
    }
    #[allow(dead_code)]
    pub fn op_hash256(stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }
        let top = stack.pop().unwrap();
        stack.push(top.hash256());
        return true;
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use num_bigint::BigInt;
    use crate::scripts::opration::{encode_num, decode_num};


    #[test]
    fn test_encode_and_decode_positive() {

        let start = 0;
        let end = 20;
        let pow_range = 25u32;

        for n in start..=end {
            for p in 0..=pow_range {
                let n = BigInt::from(n);
                let v = n.clone().pow(p);
                let encoded = encode_num(v.clone());
                let decoded = decode_num(encoded);
                assert_eq!(v, decoded, "in n = {:?}, p  = {:?}  \nassertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`)", n, p, v, decoded);
            }
        }

    }
}