use crate::helper::helper::{biguint_to_32_bytes_le, read_varint, u8vec_to_str};
use crate::scripts::script::Script;
use crate::tx::tx::Tx;
use crate::tx::tx_fetcher::TxFetcher;
use num_bigint::BigUint;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read};
#[derive(Debug, Clone)]
pub struct TxIn {
    pub(crate) prev_transaction_id: BigUint,
    pub(crate) prev_transaction_index: u32,
    pub(crate) script_sig: Script,
    sequence: u32,
}

impl TxIn {
    pub fn fetch_tx(&self, testnet: bool) -> Tx {
        return TxFetcher::fetch(self.clone().prev_transaction_id, testnet);
    }

    pub fn value(&self, testnet: bool) -> u64 {
        let index: usize = self.prev_transaction_index as usize;
        let tx = self.fetch_tx(testnet);
        return tx.tx_outs[index].amount;
    }

    pub fn serialize(mut self) -> Vec<u8> {
        let mut v = vec![];
        let prev_transaction_id = biguint_to_32_bytes_le(self.prev_transaction_id);
        for x in prev_transaction_id.iter() {
            v.push(*x);
        }
        for x in self.prev_transaction_index.to_le_bytes().iter() {
            v.push(*x);
        }
        for x in self.script_sig.serialize().iter() {
            v.push(*x);
        }
        let sequence = self.sequence.to_le_bytes();
        for x in sequence.iter() {
            v.push(*x);
        }
        return v;
    }
    pub fn parse(c: &mut Cursor<Vec<u8>>) -> TxIn {
        let mut prev_transaction_id = [0u8; 32];
        if c.read(&mut prev_transaction_id).is_err() {
            panic!("failed to read prev_transaction_id")
        }
        let prev_transaction_id = BigUint::from_bytes_le(&prev_transaction_id);

        let mut prev_transaction_index = [0u8; 4];
        if c.read(&mut prev_transaction_index).is_err() {
            panic!("failed to read prev_transaction_index")
        }

        let prev_transaction_index = u32::from_le_bytes(prev_transaction_index);

        let mut script_sig = Script::parse(c);

        let mut sequence = [0u8; 4];
        if c.read(&mut sequence).is_err() {
            panic!("failed to read sequence")
        }

        let sequence = u32::from_le_bytes(sequence);

        return TxIn {
            prev_transaction_id,
            prev_transaction_index,
            script_sig,
            sequence,
        };
    }
}

impl Display for TxIn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            self.prev_transaction_id.to_str_radix(16),
            self.prev_transaction_index
        )
    }
}
