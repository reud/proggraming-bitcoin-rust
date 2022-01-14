use crate::scripts::script::Script;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read};

#[derive(Debug, Clone)]
pub struct TxOut {
    pub(crate) amount: u64,
    pub(crate) script_pub_key: Script,
}

impl TxOut {
    pub fn serialize(self) -> Vec<u8> {
        let mut v = vec![];
        v.append(&mut self.amount.to_le_bytes().to_vec());
        v.append(&mut self.script_pub_key.serialize().to_vec());
        return v;
    }

    pub fn parse(c: &mut Cursor<Vec<u8>>) -> TxOut {
        let mut amount = [0u8; 8];
        if c.read(&mut amount).is_err() {
            panic!("failed to read amount")
        }
        let amount = u64::from_le_bytes(amount);

        let script_pub_key = Script::parse(c);

        return TxOut {
            amount,
            script_pub_key,
        };
    }
}

impl Display for TxOut {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  amount: {}\n  script_pub_key: {}",
            self.amount, self.script_pub_key
        )
    }
}
