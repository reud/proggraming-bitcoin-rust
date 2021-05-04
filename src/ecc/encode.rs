use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};

const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

#[allow(dead_code)]
pub fn encode_base58(v: Vec<u8>) -> String {
    let mut count = 0;
    for x in v.clone() {
        if x == 0u8 {
            count += 1;
        } else {
            break;
        }
    }
    let mut prefix = "".to_string();
    for _ in 1..count {
        prefix += "1";
    }

    let mut result = "".to_string();
    let mut val = BigUint::from_bytes_be(&*v);
    while val > BigUint::zero() {
        let m = val.clone() % &BigUint::from(58u32);
        val = val / BigUint::from(58u32);
        result = (BASE58_ALPHABET.as_bytes()[m.to_u8().unwrap() as usize] as char).to_string() + &*result;
    }
    return prefix + &*result;
}



#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use num_traits::Num;

    #[test]
    fn test_encode_base58() {
        {
            let bi = BigUint::from_str_radix("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",16).unwrap();
            println!("{}",encode_base58(bi.to_bytes_be()))
        }
        {
            let bi = BigUint::from_str_radix("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",16).unwrap();
            println!("{}",encode_base58(bi.to_bytes_be()))
        }
        {
            let bi = BigUint::from_str_radix("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",16).unwrap();
            println!("{}",encode_base58(bi.to_bytes_be()))
        }
    }
}

