use crate::scripts::element::{Element, new_element, new_element_from_bytes};
use crate::scripts::stack::Stack;
use num_bigint::{BigUint, BigInt};
use num_traits::{Zero, Signed, ToPrimitive, One};

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
    pub fn op_0(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::zero()));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_1negate(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(-1)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_1(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(1)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_2(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(2)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_3(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(3)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_4(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(4)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_5(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(5)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_6(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(6)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_7(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(7)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_8(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(8)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_9(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(9)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_10(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(10)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_11(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(11)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_12(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(12)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_13(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(13)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_14(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(14)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_15(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(15)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_16(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(16)));
        return true;
    }
    #[allow(dead_code)]
    pub fn op_if(stack: &mut Stack<Element>, items: &mut Vec<u8>) -> bool {
        if stack.is_empty() {
            return false;
        }
        let mut true_items: Vec<u8> = vec![];
        let mut false_items:Vec<u8> = vec![];
        let mut found = false;
        let mut num_endifs_needed = 1;
        let mut is_true_items = true;

        while items.len() > 0 {
            let item = items.remove(0);
            if item == 99 || item == 100 {
                num_endifs_needed += 1;
                if is_true_items {
                    true_items.push(item);
                } else {
                    false_items.push(item);
                }

            } else if num_endifs_needed == 1 && item == 103 {
                is_true_items = false;
            } else if item == 104 {
                if num_endifs_needed == 1 {
                    found = true;
                    break
                } else {
                    num_endifs_needed -= 1;
                    if is_true_items {
                        true_items.push(item);
                    } else {
                        false_items.push(item);
                    }
                }
            } else {
                if is_true_items {
                    true_items.push(item);
                } else {
                    false_items.push(item);
                }
            }
        }

        if ! found {
            return false;
        }
        element = stack.pop().unwrap();
        items.clear();
        if decode_num(element) == BigInt::zero() {
            for v in false_items.into_iter() {
                items.push(v);
            }
        } else {
            for v in true_items.into_iter() {
                items.push(v);
            }
        }

        return true;
    }

    #[allow(dead_code)]
    pub fn op_notif(stack: &mut Stack<Element>, items: &mut Vec<u8>) -> bool {
        if stack.is_empty() {
            return false;
        }
        let mut true_items: Vec<u8> = vec![];
        let mut false_items:Vec<u8> = vec![];
        let mut found = false;
        let mut num_endifs_needed = 1;
        let mut is_true_items = true;

        while items.len() > 0 {
            let item = items.remove(0);
            if item == 99 || item == 100 {
                num_endifs_needed += 1;
                if is_true_items {
                    true_items.push(item);
                } else {
                    false_items.push(item);
                }

            } else if num_endifs_needed == 1 && item == 103 {
                is_true_items = false;
            } else if item == 104 {
                if num_endifs_needed == 1 {
                    found = true;
                    break
                } else {
                    num_endifs_needed -= 1;
                    if is_true_items {
                        true_items.push(item);
                    } else {
                        false_items.push(item);
                    }
                }
            } else {
                if is_true_items {
                    true_items.push(item);
                } else {
                    false_items.push(item);
                }
            }
        }

        if ! found {
            return false;
        }
        element = stack.pop().unwrap();
        items.clear();
        if decode_num(element) == BigInt::zero() {
            for v in true_items.into_iter() {
                items.push(v);
            }
        } else {
            for v in false_items.into_iter() {
                items.push(v);
            }
        }

        return true;
    }

    #[allow(dead_code)]
    pub fn op_verify(stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }
        let element = stack.pop().unwrap();
        if decode_num(element) == BigInt::zero() {
            return false;
        }
        return true;
    }

    #[allow(dead_code)]
    pub fn op_return(stack: &mut Stack<Element>) -> bool {
        return false;
    }

    #[allow(dead_code)]
    pub fn op_toaltstack(stack: &mut Stack<Element>, alt_stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }
        alt_stack.push(stack.pop().unwrap());
        return true;
    }

    #[allow(dead_code)]
    pub fn op_fromaltstack(stack: &mut Stack<Element>, alt_stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }
        stack.push(alt_stack.pop().unwrap());
        return true;
    }

    #[allow(dead_code)]
    pub fn op_2drop(stack: &mut Stack<Element>) -> bool {
        if stack.len() < 2 {
            return false;
        }
        stack.pop().unwrap();
        stack.pop().unwrap();
        return true;
    }

    #[allow(dead_code)]
    pub fn op_2dup(stack: &mut Stack<Element>) -> bool {
        if stack.len() < 2 {
            return false;
        }
        let one = stack.pop().unwrap();
        let two = stack.pop().unwrap();

        stack.push(two.clone());
        stack.push(one.clone());
        stack.push(two.clone());
        stack.push(one.clone());
        return true;
    }

    #[allow(dead_code)]
    pub fn op_3dup(stack: &mut Stack<Element>) -> bool {
        if stack.len() < 3 {
            return false;
        }

        let one = stack.pop().unwrap();
        let two = stack.pop().unwrap();
        let three = stack.pop().unwrap();

        stack.push(three.clone());
        stack.push(two.clone());
        stack.push(one.clone());
        stack.push(three.clone());
        stack.push(two.clone());
        stack.push(one.clone());
        return true;
    }

    #[allow(dead_code)]
    pub fn op_2over(stack: &mut Stack<Element>) -> bool {
        if stack.len() < 4 {
            return false;
        }
        let one = stack.pop().unwrap();
        let two = stack.pop().unwrap();
        let three = stack.pop().unwrap();
        let four = stack.pop().unwrap();

        stack.push(four.clone());
        stack.push(three.clone());
        stack.push(two.clone());
        stack.push(one.clone());
        stack.push(four.clone());
        stack.push(three.clone());
        return true;
    }

    #[allow(dead_code)]
    pub fn op_2rot(stack: &mut Stack<Element>) -> bool {
        if stack.len() < 6 {
            return false;
        }
        let one = stack.pop().unwrap();
        let two = stack.pop().unwrap();
        let three = stack.pop().unwrap();
        let four = stack.pop().unwrap();
        let five = stack.pop().unwrap();
        let six = stack.pop().unwrap();

        stack.push(four.clone());
        stack.push(three.clone());
        stack.push(two.clone());
        stack.push(one.clone());
        stack.push(five.clone());
        stack.push(six.clone());
        return true;
    }

    #[allow(dead_code)]
    pub fn op_2swap(stack: &mut Stack<Element>) -> bool {
        if stack.len() < 4 {
            return false;
        }
        let one = stack.pop().unwrap();
        let two = stack.pop().unwrap();
        let three = stack.pop().unwrap();
        let four = stack.pop().unwrap();
        stack.push(two);
        stack.push(one);
        stack.push(four);
        stack.push(three);
        return true;
    }

    #[allow(dead_code)]
    pub fn op_ifdup(stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }
        let el = stack.top().unwrap();
        if decode_num(el.clone()) != BigInt::zero() {
            stack.push(el);
        }
        return true;
    }

    #[allow(dead_code)]
    pub fn op_depth(stack: &mut Stack<Element>) -> bool {
        stack.push(encode_num(BigInt::from(stack.len())));
        return true;
    }

    #[allow(dead_code)]
    pub fn op_drop(stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }
        let _ = stack.pop();
        return true;
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
    pub fn op_nip(stack: &mut Stack<Element>) -> bool {
        if stack.len() < 2 {
            return false;
        }
        let one = stack.pop().unwrap();
        let _ = stack.pop().unwrap();
        stack.push(one);
        return true;
    }

    #[allow(dead_code)]
    pub fn op_over(stack: &mut Stack<Element>) -> bool {
        if stack.len() < 2 {
            return false;
        }
        let one = stack.pop().unwrap();
        let two = stack.pop().unwrap();
        stack.push(two.clone());
        stack.push(one);
        stack.push(two);
        return true;
    }

    #[allow(dead_code)]
    pub fn op_pick(stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }

        let n = decode_num(stack.pop().unwrap());
        let bi = BigInt::from(stack.len());
        if bi < (n + BigInt::one()) {
            return false;
        }
        let last = stack.len() - 1;
        let index = (last as u64) - n.to_u64().unwrap();
        stack.push(stack.get(index as usize).unwrap());
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