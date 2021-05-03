use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Sub, Add, Mul, Rem, Div};
use num_bigint::BigUint;
use num_traits::{One, FromPrimitive, Zero, Num};

fn prime() -> BigUint {
    return BigUint::from(2u32).pow(256)
        - BigUint::from_u8(2u8).unwrap().pow(32u32)
        - (BigUint::from(1u8) * 977u32)
}


// Sec256k1Elementと共通化したい・・・
// Debugの自動実装
#[derive(Debug, Clone)]
pub struct Sec256k1Element {
    pub num: BigUint,
}

// P.5 練習問題1
impl PartialEq for Sec256k1Element {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num;
    }
}

impl Add for Sec256k1Element {
    type Output = Sec256k1Element;

    // 左側のprimeに依存させる。
    fn add(self, rhs: Sec256k1Element) -> Sec256k1Element {
        Self::Output{
            num: (self.num+rhs.num) % prime()
        }
    }
}

impl Sub for Sec256k1Element {
    type Output = Sec256k1Element;

    fn sub(self, rhs: Sec256k1Element) -> Sec256k1Element {
        Self::Output{
            num: (self.num  + prime() - rhs.num) % prime()
        }
    }
}

impl Mul for Sec256k1Element {
    type Output = Sec256k1Element;

    fn mul(self, rhs: Sec256k1Element) -> Sec256k1Element {
        Self::Output{
            num: (self.num * rhs.num) % prime()
        }
    }
}

impl Rem for Sec256k1Element {
    type Output = Sec256k1Element;

    fn rem(self, rhs: Sec256k1Element) -> Sec256k1Element {
        Self::Output{
            num: (self.num % rhs.num) % prime()
        }
    }
}


// TODO: どうにかして実装したい。
impl Sec256k1Element {
    fn inner_pow(self,f: Sec256k1Element,exp: BigUint) -> Sec256k1Element {
        if exp.clone() == BigUint::zero() {
            return Sec256k1Element{
                num: One::one(),
            }
        }
        if exp.clone() % BigUint::from(2u32) == BigUint::zero() {
            return self.inner_pow(Sec256k1Element{
                num: (f.num.clone() * f.num.clone()) % prime(),
            },exp.clone() / BigUint::from(2u32));
        }
        f.clone() * f.clone().inner_pow(Sec256k1Element{
            num: (f.num.clone() * f.num.clone()) % prime(),
        },(exp.clone()-BigUint::one())/BigUint::from(2u32))
    }
    // rem_euclidを使って負数でもよしなに整数値に変更する。
    pub fn pow(self, exp: BigUint) -> Sec256k1Element {
        self.clone().inner_pow(self, exp % (prime() - BigUint::one()))
    }

    // フェルマーの小定理からインバースを実装する。 位数が素数で無い場合は正しく動作しない
    pub fn inv(self) -> Sec256k1Element {
        return self.pow(prime() - BigUint::from(2u32))
    }
}

impl Div for Sec256k1Element {
    type Output = Sec256k1Element;

    fn div(self, rhs: Self) -> Self::Output {
        return self * rhs.inv();
    }
}

impl Display for Sec256k1Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.num)
    }
}

#[allow(dead_code)]
pub fn new_secp256k1element(num: BigUint) -> Sec256k1Element {
    Sec256k1Element{
        num: num % prime()
    }
}
#[allow(dead_code)]
pub fn new_secp256k1element_from_i32(num: i32) -> Sec256k1Element {
    Sec256k1Element{
        num: BigUint::from_i32(num).unwrap(),
    }
}

#[allow(dead_code)]
pub fn new_secp256k1element_from_hex_str(hex: &str) -> Option<Sec256k1Element> {
    let hex = BigUint::from_str_radix(hex,16);
    if hex.is_err() {
        return None;
    }
    Some(new_secp256k1element(hex.unwrap()))
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn test_secp256k1_field() {
        {
            let a = new_secp256k1element(BigUint::from(7u32));
            let b = new_secp256k1element(BigUint::from(12u32));
            let c = new_secp256k1element(BigUint::from(19u32));
            let d = new_secp256k1element(BigUint::from(5u32));
            assert_ne!(a,b);
            assert_eq!((a.clone() + b.clone()),c.clone());
            assert_eq!((c.clone() % a.clone()),d.clone());
        }
        {
            let a = new_secp256k1element(prime() - BigUint::one());
            let b = new_secp256k1element(BigUint::from(12u32));

            println!("{}",a == b);
            println!("{}",a);
            println!("a + b = {}",a.clone() + b.clone());
            println!("a % b = {}",a.clone() % b.clone());
        }
    }
}

