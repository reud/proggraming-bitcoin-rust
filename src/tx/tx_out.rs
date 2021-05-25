use std::io::{Cursor, Read};
use crate::tx::helper::{read_varint, u8vec_to_str};
use std::fmt::{Display, Formatter};
use std::fmt;


#[derive(Debug,Clone)]
pub struct TxOut {
    pub(crate) amount: u64,
    pub(crate) script_pub_key: Vec<u8>
}

impl TxOut {
    pub fn serialize(mut self) -> Vec<u8> {
        let mut v = vec![];
        for x in self.amount.to_le_bytes() {
            v.push(x);
        }
        v.append(&mut self.script_pub_key);
        return v;
    }

    pub fn parse(c: &mut Cursor<Vec<u8>>) -> TxOut {
        let mut amount = [0u8; 8];
        if c.read(&mut amount).is_err() {
            panic!("failed to read amount")
        }
        let amount = u64::from_le_bytes(amount);

        let script_pub_key_sz = read_varint(c);
        let mut script_pub_key = vec![0u8; script_pub_key_sz as usize];
        if c.read(&mut script_pub_key).is_err() {
            panic!("failed to read script_pub_key")
        }

        return TxOut {
            amount,
            script_pub_key
        }
    }
}

impl Display for TxOut {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"  amount: {}\n  script_pub_key: {}",
               self.amount,
               u8vec_to_str(self.clone().script_pub_key))
    }
}