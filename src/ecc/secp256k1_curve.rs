use num_bigint::BigUint;

#[derive(Debug, Clone,Copy)]
pub struct Secp256k1Curve {
 }

impl Secp256k1Curve {
    pub fn lhs(self, y: BigUint) -> BigUint {
        y.clone() * y
    }

    pub fn rhs(self, x: BigUint) -> BigUint {
        x.clone() * x.clone() * x + BigUint::from(7u64)
    }
}

pub fn new_secp256k1curve() -> Secp256k1Curve {
    return Secp256k1Curve{}
}
