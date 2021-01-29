use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::Add;
use std::process::Output;

// Debugの自動実装
#[derive(Debug)]
struct FieldElement {
    num: u64,
    prime: u64
}

// P.5 練習問題1
impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num && self.prime == other.prime;
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    // 左側のprimeに依存させる。
    fn add(self, rhs: FieldElement) -> FieldElement {
        Self::Output{
            prime: self.prime,
            num: (self.num+rhs.num).rem_euclid(self.prime)
        }
    }

}

impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.num)
    }
}

fn new_field_element(num: i64, prime: u64) -> FieldElement {
    FieldElement{
        num: num.rem_euclid(prime as i64) as u64,
        prime
    }
}

fn main() {
    {
        let a = new_field_element(7,13);
        let b = new_field_element(6,13);

        println!("{}",a == b);
        println!("{}",a != b);

        println!("{}",a == a);
        println!("{}",a != a);
    }


    // P.9 練習問題2
    {
        {
            let a = new_field_element(44,57);
            let b = new_field_element(33,57);
            println!("{}",a + b);
        }
        {
            let a = new_field_element(9,57);
            let b = new_field_element(-29,57);
            println!("{}", a + b);
        }
        {
            let a = new_field_element(17,57);
            let b = new_field_element(42,57);
            let c = new_field_element(49,57);
            println!("{}", a + b + c);
        }
        {
            let a = new_field_element(52,57);
            let b = new_field_element(-30,57);
            let c = new_field_element(-38,57);
            println!("{}", a + b + c);
        }
    }
}