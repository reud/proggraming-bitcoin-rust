use std::fmt::{Display, Formatter};
use crate::FieldElementError::InitializeError;

struct FieldElement {
    num: u64,
    prime: u64
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num && self.prime == other.prime;
    }
}

// Debugの自動実装
#[derive(Debug)]
enum FieldElementError {
    InitializeError(u64,u64,&str)
}

impl Display for FieldElementError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // matchを利用する。
        use self::FieldElementError::*;
        match self {
            InitializeError(n,p,s) => write!(f, "InitializeError: msg: {} num: {} prime: {}",s,n,p);
        }
    }
}

fn new_field_element(num: u64, prime: u64) -> Result<FieldElement,FieldElementError> {
    if num >= prime || num < 0 {
        return Err(InitializeError(n, p, "bad argument"));
    }
    return returnOK(FieldElement{
        num,
        prime
    });
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
        a = new_field_element(44,57).unwrap();
        b = new_field_element(44,57).unwrap();
        // TODO 加算の追加
        println!("{}",a == b);
    }
}