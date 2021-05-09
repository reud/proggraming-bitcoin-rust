use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Sub, Add, Mul, Rem, Div};
use num_bigint::BigUint;
use num_traits::{One, FromPrimitive, Zero, Num};

// 楕円曲線上の加算に対しての有限体の位数はこの値となる。
fn prime() -> BigUint {
    return BigUint::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",16).unwrap()
}

// Debugの自動実装
#[derive(Debug, Clone)]
pub struct Secp256k1ScalarElement {
    pub num: BigUint,
}

impl PartialEq for Secp256k1ScalarElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num;
    }
}

impl Add for Secp256k1ScalarElement {
    type Output = Secp256k1ScalarElement;

    // 左側のprimeに依存させる。
    fn add(self, rhs: Secp256k1ScalarElement) -> Secp256k1ScalarElement {
        Self::Output{
            num: (self.num+rhs.num) % prime()
        }
    }
}

impl Sub for Secp256k1ScalarElement {
    type Output = Secp256k1ScalarElement;

    fn sub(self, rhs: Secp256k1ScalarElement) -> Secp256k1ScalarElement {
        Self::Output{
            num: (self.num  + prime() - rhs.num) % prime()
        }
    }
}

impl Mul for Secp256k1ScalarElement {
    type Output = Secp256k1ScalarElement;

    fn mul(self, rhs: Secp256k1ScalarElement) -> Secp256k1ScalarElement {
        Self::Output{
            num: (self.num * rhs.num) % prime()
        }
    }
}

impl Rem for Secp256k1ScalarElement {
    type Output = Secp256k1ScalarElement;

    fn rem(self, rhs: Secp256k1ScalarElement) -> Secp256k1ScalarElement {
        Self::Output{
            num: (self.num % rhs.num) % prime()
        }
    }
}


// TODO: どうにかして実装したい。
impl Secp256k1ScalarElement {
    fn inner_pow(self,f: Secp256k1ScalarElement,exp: BigUint) -> Secp256k1ScalarElement {
        if exp.clone() == BigUint::zero() {
            return Secp256k1ScalarElement{
                num: One::one(),
            }
        }
        if exp.clone() % BigUint::from(2u32) == BigUint::zero() {
            return self.inner_pow(Secp256k1ScalarElement{
                num: (f.num.clone() * f.num.clone()) % prime(),
            },exp.clone() / BigUint::from(2u32));
        }
        f.clone() * f.clone().inner_pow(Secp256k1ScalarElement{
            num: (f.num.clone() * f.num.clone()) % prime(),
        },(exp.clone()-BigUint::one())/BigUint::from(2u32))
    }
    // rem_euclidを使って負数でもよしなに整数値に変更する。
    pub fn pow(self, exp: BigUint) -> Secp256k1ScalarElement {
        self.clone().inner_pow(self, exp % (prime() - BigUint::one()))
    }

    // フェルマーの小定理からインバースを実装する。 位数が素数で無い場合は正しく動作しない
    pub fn inv(self) -> Secp256k1ScalarElement {
        return self.pow(prime() - BigUint::from(2u32))
    }

    #[inline]
    pub fn to_32_bytes_be(&self) -> Option<Vec<u8>> {
        let mut bin = self.num.to_bytes_be();
        if bin.len() > 32 {
            return None;
        }
        // FIXME: より効率的に
        while bin.len() != 32 {
            bin.insert(0,0u8);
        }
        Some(bin)
    }
}

impl Div for Secp256k1ScalarElement {
    type Output = Secp256k1ScalarElement;

    fn div(self, rhs: Self) -> Self::Output {
        return self * rhs.inv();
    }
}

impl Display for Secp256k1ScalarElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.num)
    }
}

pub fn new_secp256k1scalarelement(num: BigUint) -> Secp256k1ScalarElement {
    Secp256k1ScalarElement{
        num: num % prime()
    }
}

pub fn new_secp256k1scalarelement_from_i32(num: i32) -> Secp256k1ScalarElement {
    Secp256k1ScalarElement{
        num: BigUint::from_i32(num).unwrap(),
    }
}

pub fn new_secp256k1scalarelement_from_u64(num: u64) -> Secp256k1ScalarElement {
    Secp256k1ScalarElement{
        num: BigUint::from_u64(num).unwrap(),
    }
}

pub fn new_secp256k1scalarelement_from_hex_str(hex: &str) -> Option<Secp256k1ScalarElement> {
    let hex = BigUint::from_str_radix(hex,16);
    if hex.is_err() {
        return None;
    }
    Some(new_secp256k1scalarelement(hex.unwrap()))
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn test_secp256k1_field() {
        {
            let a = new_secp256k1scalarelement(BigUint::from(7u32));
            let b = new_secp256k1scalarelement(BigUint::from(12u32));
            let c = new_secp256k1scalarelement(BigUint::from(19u32));
            let d = new_secp256k1scalarelement(BigUint::from(5u32));
            assert_ne!(a,b);
            assert_eq!((a.clone() + b.clone()),c.clone());
            assert_eq!((c.clone() % a.clone()),d.clone());
        }
        {
            let a = new_secp256k1scalarelement(prime() - BigUint::one());
            let b = new_secp256k1scalarelement(BigUint::from(12u32));

            println!("{}",a == b);
            println!("{}",a);
            println!("a + b = {}",a.clone() + b.clone());
            println!("a % b = {}",a.clone() % b.clone());
        }
    }
}

