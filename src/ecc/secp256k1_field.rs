use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Sub, Add, Mul, Rem, Div};
use num_bigint::BigUint;
use num_traits::{One, FromPrimitive};

const PRIME: BigUint = BigUint::from_u8(2u8).unwrap().pow(256)
    - BigUint::from_u8(2u8).unwrap().pow(32u32)
    - (BigUint::from(1u8) * 977u32);

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
            num: (self.num+rhs.num) % PRIME
        }
    }
}

impl Sub for Sec256k1Element {
    type Output = Sec256k1Element;

    fn sub(self, rhs: Sec256k1Element) -> Sec256k1Element {
        Self::Output{
            num: (self.num  - rhs.num) % PRIME
        }
    }
}

impl Mul for Sec256k1Element {
    type Output = Sec256k1Element;

    fn mul(self, rhs: Sec256k1Element) -> Sec256k1Element {
        Self::Output{
            num: (self.num * rhs.num) % PRIME
        }
    }
}

impl Rem for Sec256k1Element {
    type Output = Sec256k1Element;

    fn rem(self, rhs: Sec256k1Element) -> Sec256k1Element {
        Self::Output{
            num: (self.num % rhs.num) % PRIME
        }
    }
}


// TODO: どうにかして実装したい。
impl Sec256k1Element {
    fn inner_pow(self,f: Sec256k1Element,exp: u64) -> Sec256k1Element {
        if exp == 0 {
            return Sec256k1Element{
                num: One::one(),
            }
        }
        if exp % 2 == 0 {
            return self.inner_pow(Sec256k1Element{
                num: (f.num.clone() * f.num.clone()) % PRIME,
            },exp / 2);
        }
        f * f.inner_pow(Sec256k1Element{
            num: (f.num.clone() * f.num.clone()) % PRIME,
        },(exp-1)/2)
    }
    // rem_euclidを使って負数でもよしなに整数値に変更する。
    pub fn pow(self, exp: BigUint) -> Sec256k1Element {
        self.inner_pow(self, exp % (PRIME - 1))
    }

    // フェルマーの小定理からインバースを実装する。 位数が素数で無い場合は正しく動作しない
    pub fn inv(self) -> Sec256k1Element {
        return self.pow(PRIME - 2)
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

pub fn new_field_element(num: BigUint) -> Sec256k1Element {
    Sec256k1Element{
        num: num % PRIME
    }
}