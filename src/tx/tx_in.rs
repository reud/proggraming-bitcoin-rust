use num_bigint::BigUint;
use std::io::{Cursor, Read};
use crate::tx::helper::read_varint;
use std::fmt::{Formatter, Display};
use std::fmt;

#[derive(Debug,Clone)]
pub struct TxIn {
    pub(crate) prev_transaction_id: BigUint,
    pub(crate) prev_transaction_index: u32,
    pub(crate) script_sig: Vec<u8>,
    sequence: u32,
}

impl TxIn {
    pub fn parse(c: &mut Cursor<Vec<u8>>) -> TxIn {
        let mut prev_transaction_id = [0u8;32];
        c.read(&mut prev_transaction_id);
        let prev_transaction_id = BigUint::from_bytes_le(&prev_transaction_id);

        let mut prev_transaction_index = [0u8; 4];
        c.read(&mut prev_transaction_index);
        let  prev_transaction_index = u32::from_le_bytes(prev_transaction_index);

        let script_sig_sz = read_varint(c);

        println!("script_sz: {}",script_sig_sz );
        let mut script_sig = vec![0u8; script_sig_sz as usize];
        c.read(&mut *script_sig);
        println!("script_sig: {:x?}", &script_sig[..]);

        let mut sequence = [0u8; 4];
        c.read(&mut sequence);
        let sequence = u32::from_le_bytes(sequence);

        return TxIn {
            prev_transaction_id,
            prev_transaction_index,
            script_sig,
            sequence
        }
    }


}

impl Display for TxIn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}:{}", self.prev_transaction_id, self.prev_transaction_index)
    }
}