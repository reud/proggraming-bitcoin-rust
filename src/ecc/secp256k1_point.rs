use crate::ecc::secp256k1_field::{Secp256k1Element, new_secp256k1element_from_i32, new_secp256k1element};
use std::fmt::{Display, Formatter};
use std::fmt;
use num_bigint::{BigUint };
use num_traits::{FromPrimitive, Num, Zero, One};
use crate::ecc::secp256k1_curve::{Secp256k1Curve, new_secp256k1curve};
use std::ops::{Add, Rem, Div};

use crate::ecc::secp256k1_scalar_element::Secp256k1ScalarElement;
use crate::ecc::secp256k1_signature::Secp256k1Signature;
use crate::ecc::helper::hash160;
use crate::ecc::encode::encode_base58_checksum;


#[derive(Debug, Clone)]
pub struct Secp256k1Point {
    pub(crate) x: Secp256k1Element,
    pub(crate) y: Secp256k1Element,
    is_infinity: bool,
    curve: Secp256k1Curve
}

impl PartialEq for Secp256k1Point {
    fn eq(&self, other: &Self) -> bool {
        if self.is_infinity || other.is_infinity {
            if self.is_infinity && other.is_infinity {
                return true;
            }
            return false;
        }
        return self.x == other.x && self.y == other.y;
    }
}

impl Secp256k1Point {

    #[allow(dead_code)]
    pub fn compressed_address(self, test_net: bool) -> String {
        let prefix:u8 = if test_net { 0x6f } else { 0x00 };
        let mut h160 = self.hash160_by_compressed_sec();
        h160.insert(0,prefix);
        return encode_base58_checksum(h160);
    }

    #[allow(dead_code)]
    pub fn uncompressed_address(self, test_net: bool) -> String {
        let prefix:u8 = if test_net { 0x6f } else { 0x00 };
        let mut h160 = self.hash160_by_uncompressed_sec();
        h160.insert(0,prefix);
        return encode_base58_checksum(h160);
    }

    #[allow(dead_code)]
    pub fn hash160_by_compressed_sec(self) -> Vec<u8> {
        return hash160(self.compressed_sec());
    }

    #[allow(dead_code)]
    pub fn hash160_by_uncompressed_sec(self) -> Vec<u8> {
        return hash160(self.uncompressed_sec());
    }

    #[allow(dead_code)]
    fn parse_uncompressed_sec(v: Vec<u8>) -> Secp256k1Point {
        if v.len() != 65 {
            panic!("bad length: expect　65 but got {}",v.len())
        }
        let cons = v.split_first().unwrap();
        let pos_vec = cons.1;
        let xy_vec: Vec<Vec<u8>> = pos_vec.chunks(2).map(|x| x.to_vec()).collect();
        if xy_vec.len() != 2 {
            panic!("unknown error occuered(execute chunks(32),but cant get x,y positon)");
        }

        let x = BigUint::from_bytes_be(&*xy_vec[0]);
        let y = BigUint::from_bytes_be(&*xy_vec[1]);

        return new_secp256k1point_from_big_uint(x,y);
    }

    #[allow(dead_code)]
    fn parse_compressed_sec(v: Vec<u8>) -> Secp256k1Point {
        if v.len() != 33 {
            panic!("bad length: expect　33 but got {}",v.len())
        }
        let cons = v.split_first().unwrap();
        let marker = *cons.0;
        let x = BigUint::from_bytes_be(cons.1);
        let curve = new_secp256k1curve();
        let y_square = curve.rhs(x.clone());
        let y_square = new_secp256k1element(y_square);
        let y_1 = y_square.sqrt();
        let y_2 = Secp256k1Element::prime() - y_1.clone();
        if marker == 2u8 { // y is even
            if y_1.clone().rem(new_secp256k1element_from_i32(2)).num == BigUint::zero() {
                return new_secp256k1point_from_element(new_secp256k1element(x),y_1);
            }
            return new_secp256k1point_from_element(new_secp256k1element(x),y_2);
        }
        if marker == 3u8 { // y is odd
            if y_1.clone().rem(new_secp256k1element_from_i32(2)).num == BigUint::zero() {
                return new_secp256k1point_from_element(new_secp256k1element(x),y_2);
            }
            return new_secp256k1point_from_element(new_secp256k1element(x),y_1);
        }
        panic!("unrecognized marker: {} marker can be only (2,3) ",marker);
    }

    fn inner_mul(self, f: Secp256k1Point, v: BigUint) -> Secp256k1Point {
        if v == BigUint::zero() {
            return new_secp256k1point_infinity();
        }
        if v.clone().rem(2u64) == BigUint::zero() {
            let half_res = self.inner_mul(f, v.clone().div(2u32));
            let half_res2 = half_res.clone();
            return half_res + half_res2;
        }
        let cf = f.clone();
        f.clone() + f.clone().inner_mul(cf, v - BigUint::one())
    }
    #[allow(dead_code)]
    pub fn mul_from_u64(self, v: u64) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, BigUint::from(v))
    }
    #[allow(dead_code)]
    pub fn mul_from_u32(self, v: u32) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, BigUint::from(v))
    }
    #[allow(dead_code)]
    pub fn mul_from_i32(self, v: i32) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, BigUint::from_i32(v).unwrap())
    }
    #[allow(dead_code)]
    pub fn mul_from_big_uint(self, v: BigUint) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, v)
    }
    #[allow(dead_code)]
    pub fn mul_from_sec256k1scalar_element(self, v: Secp256k1ScalarElement) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, v.num)
    }
    #[allow(dead_code)]
    pub fn verify(self,z: Secp256k1ScalarElement, sig: Secp256k1Signature) -> bool {
        let u = z.clone()/sig.s.clone();
        let v= sig.r.clone()/sig.s.clone();
        let g = new_secp256k1point_g();
        let r_point = g.mul_from_sec256k1scalar_element(u) + self.clone().mul_from_sec256k1scalar_element(v);
        return r_point.x.num == sig.r.num;
    }

    #[allow(dead_code)]
    pub fn uncompressed_sec_str(self) -> String {
        let mut ret = "04".to_string(); // prefix
        for v in self.x.to_32_bytes_be().unwrap() {
            ret += &*format!("{:02x}", v);
        }
        for v in self.y.to_32_bytes_be().unwrap() {
            ret += &*format!("{:02x}", v);
        }

        return ret.to_string();
    }

    #[allow(dead_code)]
    pub fn uncompressed_sec(self) -> Vec<u8> {
        let mut vec = vec![4u8];
        // fixme: vecがマージできそうな構造にする。
        for v in self.x.to_32_bytes_be().unwrap() {
            vec.push(v);
        }
        for v in self.y.to_32_bytes_be().unwrap() {
            vec.push(v);
        }
        vec
    }

    #[allow(dead_code)]
    pub fn compressed_sec_str(self) -> String {
        let is_even = self.y.rem(new_secp256k1element_from_i32(2)) == new_secp256k1element_from_i32(0);
        let mut ret: String = "".to_string();
        if is_even {
            ret += "02";
        } else {
            ret += "03";
        }
        for v in self.x.to_32_bytes_be().unwrap() {
            ret += &*format!("{:02x}", v);
        }
        return ret;
    }

    #[allow(dead_code)]
    pub fn compressed_sec(self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![];
        let is_even = self.y.rem(new_secp256k1element_from_i32(2)) == new_secp256k1element_from_i32(0);
        if is_even {
            v.push(2u8);
        } else {
            v.push(3u8);
        }
        for x in self.x.to_32_bytes_be().unwrap() {
            v.push(x);
        }
        v
    }
}

impl Display for Secp256k1Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_infinity {
            return write!(f,"(無限遠点)");
        }
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for Secp256k1Point {
    type Output = Secp256k1Point;
    fn add(self, rhs: Self) -> Self::Output {

        if self.is_infinity {
            return rhs;
        }

        if rhs.is_infinity {
            return self;
        }

        // 楕円曲線の座標が一致する時は接点の傾きを利用する。
        if self == rhs {
            let f0 =new_secp256k1element_from_i32(0);
            let f1 = new_secp256k1element_from_i32(1);
            let f2 = new_secp256k1element_from_i32(2);
            let f3 = new_secp256k1element_from_i32(3);
            if self.y == f0 && rhs.y == f0 {
                return new_secp256k1point_infinity();
            }

            let b = new_secp256k1element(self.curve.rhs(f0.num));
            let a = new_secp256k1element(self.curve.rhs(f1.clone().num)) - f1 - b;
            let x = self.x.clone();
            let y = self.y.clone();
            let s = (f3 * x.clone() * x.clone() + a) / (f2.clone() * y.clone());
            let x3 = s.clone() * s.clone() - (f2.clone() * x.clone());
            let y3 = s.clone() * (x.clone() - x3.clone()) - y.clone();
            let p = new_secp256k1point_from_element(x3.clone(),y3.clone());
            return p;
        }

        // 加法逆元の場合、無限遠点を返す
        let f0 = new_secp256k1element_from_i32(0);
        if self.x.clone() == rhs.x.clone() && (self.y.clone() + rhs.y.clone()) == f0 {
            return new_secp256k1point_infinity();
        }

        let s = (rhs.y.clone() - self.y.clone()) / (rhs.x.clone() - self.x.clone());
        let x3 = s.clone() * s.clone() - self.x.clone() - rhs.x.clone();
        let y3 = s.clone() * (self.x.clone() - x3.clone()) - self.y.clone();

        return new_secp256k1point_from_element(x3.clone(),y3);
    }
}


#[allow(dead_code)]
fn new_secp256k1point_from_i64(x: i64,y:i64) -> Option<Secp256k1Point> {
    let xe = BigUint::from_i64(x);
    if xe.is_none() {
        return None;
    }
    let x = new_secp256k1element(xe.unwrap());

    let ye = BigUint::from_i64(y);
    if ye.is_none() {
        return None;
    }
    let y = new_secp256k1element(ye.unwrap());
    return Some(Secp256k1Point{
        x,
        y,
        is_infinity: false,
        curve: new_secp256k1curve()
    })
}

#[allow(dead_code)]
fn new_secp256k1point_from_i32(x: i32,y:i32) -> Option<Secp256k1Point> {
    let xe = BigUint::from_i32(x);
    if xe.is_none() {
        return None;
    }
    let x = new_secp256k1element(xe.unwrap());

    let ye = BigUint::from_i32(y);
    if ye.is_none() {
        return None;
    }
    let y = new_secp256k1element(ye.unwrap());
    return Some(Secp256k1Point{
        x,
        y,
        is_infinity: false,
        curve: new_secp256k1curve()
    })
}

fn new_secp256k1point_from_big_uint(x: BigUint,y: BigUint) -> Secp256k1Point {
    let x = new_secp256k1element(x);
    let y = new_secp256k1element(y);
    return Secp256k1Point {
        x,
        y,
        is_infinity: false,
        curve: new_secp256k1curve()
    }
}


fn new_secp256k1point_from_element(x: Secp256k1Element,y: Secp256k1Element) -> Secp256k1Point {
    return Secp256k1Point {
        x,
        y,
        is_infinity: false,
        curve: new_secp256k1curve()
    }
}

fn new_secp256k1point_infinity() -> Secp256k1Point {
    return Secp256k1Point {
        x: new_secp256k1element(BigUint::from(1u64)),
        y: new_secp256k1element(BigUint::from(1u64)),
        is_infinity: true,
        curve: new_secp256k1curve()
    }
}

pub fn new_secp256k1point_g() -> Secp256k1Point {
    let x = BigUint::from_str_radix("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",16).unwrap();
    let y = BigUint::from_str_radix("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",16).unwrap();
    return new_secp256k1point_from_big_uint(x,y);
}

fn new_secp256k1point_from_hex_str(x: &str,y: &str) -> Option<Secp256k1Point> {
    let x = BigUint::from_str_radix(x,16);
    if x.is_err() {
        return None;
    }
    let y = BigUint::from_str_radix(y,16);
    if y.is_err() {
        return None;
    }
    Some(new_secp256k1point_from_big_uint(x.unwrap(),y.unwrap()))
}



#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::ecc::secp256k1_scalar_element::{new_secp256k1scalarelement_from_hex_str, new_secp256k1scalarelement_from_i32};
    use crate::ecc::secp256k1_signature::new_secp256k1signature;
    use crate::ecc::secp256k1_privatekey::{new_secp_256k1privatekey_from_i32, new_secp_256k1privatekey};

    #[test]
    fn test_base_point() {
        // ベースポイントGx,Gyに位数nを掛けたら無限遠点が帰ること
        let base = new_secp256k1point_g();
        let n = BigUint::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();
        assert_eq!(new_secp256k1point_infinity(),base.mul_from_big_uint(n));
    }

    #[test]
    fn test_signature_practice_p69q6() {

        {
            // signature 1
            let px = "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c";
            let py = "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
            let p = new_secp256k1point_from_hex_str(px,py).unwrap();
            let z = new_secp256k1scalarelement_from_hex_str("ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60").unwrap();
            let r = new_secp256k1scalarelement_from_hex_str("ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395").unwrap();
            let s = new_secp256k1scalarelement_from_hex_str("68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4").unwrap();

            let u = z.clone()/s.clone();
            let v= r.clone()/s.clone();
            let g = new_secp256k1point_g();
            let ug = g.clone().mul_from_sec256k1scalar_element(u.clone());
            let vp = p.clone().mul_from_sec256k1scalar_element(v.clone());
            let r_point = ug.clone() + vp.clone();
            // Rxとrが一致していれば署名は有効
            assert_eq!(r_point.x.num,r.num);
        }
        {
            // signature 1
            let px = "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c";
            let py = "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
            let p = new_secp256k1point_from_hex_str(px,py).unwrap();
            let z = new_secp256k1scalarelement_from_hex_str("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap();
            let r = new_secp256k1scalarelement_from_hex_str("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap();
            let s = new_secp256k1scalarelement_from_hex_str("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap();

            let u = z.clone()/s.clone();
            let v= r.clone()/s.clone();
            let g = new_secp256k1point_g();
            let r_point = g.mul_from_sec256k1scalar_element(u) + p.mul_from_sec256k1scalar_element(v);
            // Rxとrが一致していれば署名は有効
            assert_eq!(r_point.x.num,r.num);
        }
        {
            // signature 1
            let px = "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c";
            let py = "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
            let p = new_secp256k1point_from_hex_str(px,py).unwrap();
            let z = new_secp256k1scalarelement_from_hex_str("ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60").unwrap();
            let r = new_secp256k1scalarelement_from_hex_str("ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395").unwrap();
            let s = new_secp256k1scalarelement_from_hex_str("68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4").unwrap();
            let sig = new_secp256k1signature(r,s);
            assert!(p.verify(z,sig));
        }
        {
            // signature 1
            let px = "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c";
            let py = "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
            let p = new_secp256k1point_from_hex_str(px,py).unwrap();
            let z = new_secp256k1scalarelement_from_hex_str("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap();
            let r = new_secp256k1scalarelement_from_hex_str("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap();
            let s = new_secp256k1scalarelement_from_hex_str("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap();
            let sig = new_secp256k1signature(r,s);
            assert!(p.verify(z,sig));
        }
    }

    #[test]
    fn test_sec_p77q1() {
        {
            let g = new_secp256k1point_g();
            let g = g.mul_from_i32(5000);
            assert_eq!(g.uncompressed_sec_str(),"04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10");
        }
        {
            let g = new_secp256k1point_g();
            let bi = new_secp256k1scalarelement_from_i32(2018);
            let g = g.mul_from_sec256k1scalar_element(bi.pow(BigUint::from(5u8)));
            assert_eq!(g.uncompressed_sec_str(),"04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06");
        }
        {
            let g = new_secp256k1point_g();
            let bi = BigUint::from_str_radix("deadbeef12345",16).unwrap();
            let g = g.mul_from_big_uint(bi);
            assert_eq!(g.uncompressed_sec_str(),"04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd59961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121");
        }
    }

    #[test]
    fn test_sec_p81q2() {
        {
            let g = new_secp256k1point_g();
            let g = g.mul_from_i32(5001);
            assert_eq!(g.compressed_sec_str(),"0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1");
        }
        {
            let g = new_secp256k1point_g();
            let bi = new_secp256k1scalarelement_from_i32(2019);
            let g = g.mul_from_sec256k1scalar_element(bi.pow(BigUint::from(5u8)));
            assert_eq!(g.compressed_sec_str(),"02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701");
        }
        {
            let g = new_secp256k1point_g();
            let bi = BigUint::from_str_radix("deadbeef54321",16).unwrap();
            let g = g.mul_from_big_uint(bi);
            assert_eq!(g.compressed_sec_str(),"0296be5b1292f6c856b3c5654e886fc13511462059089cdf9c479623bfcbe77690");
        }
    }

    #[test]
    fn test_address_p86() {
        {
            let private = new_secp_256k1privatekey_from_i32(5002);
            let addr = private.point.uncompressed_address(true);
            assert_eq!("mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA",addr);
        }
        {
            let se = new_secp256k1scalarelement_from_i32(2020);
            let private = new_secp_256k1privatekey(se.pow(BigUint::from(5u8)));
            let addr = private.point.compressed_address(true);
            assert_eq!("mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH",addr)
        }
        {
            let se = new_secp256k1scalarelement_from_hex_str("12345deadbeef").unwrap();
            let private = new_secp_256k1privatekey(se);
            let addr = private.point.compressed_address(false);
            assert_eq!("1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1",addr);
        }
    }
}
