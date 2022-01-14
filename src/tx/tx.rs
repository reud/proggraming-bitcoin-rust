use crate::helper::helper::{
    encode_varint, hash256, read_varint, u8vec_to_str, vector_as_u8_4_array,
};
use crate::tx::tx_in::TxIn;
use crate::tx::tx_out::TxOut;

use crate::ecc::secp256k1_scalar_element::new_secp256k1scalarelement;

use crate::ecc::secp256k1_privatekey::Secp256k1PrivateKey;
use crate::ecc::secp256k1_signature::Secp256k1Signature;
use crate::scripts::script::{new_script, Cmd};
use crate::Script;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read};

pub enum Sighash {
    All = 1,
}

#[derive(Debug, Clone)]
pub struct Tx {
    pub(crate) version: u32,
    pub(crate) tx_ins: Vec<TxIn>,
    pub(crate) tx_outs: Vec<TxOut>,
    pub(crate) lock_time: u32,
    pub(crate) testnet: bool,
}

impl Tx {
    pub fn new(
        version: u32,
        tx_ins: Vec<TxIn>,
        tx_outs: Vec<TxOut>,
        lock_time: u32,
        testnet: bool,
    ) -> Tx {
        Tx {
            version,
            tx_ins,
            tx_outs,
            lock_time,
            testnet,
        }
    }

    pub fn verify(&self) -> bool {
        if self.clone().fee() < BigUint::from(0u64) {
            return false;
        }
        for i in 0..self.clone().tx_ins.len() {
            if !self.verify_input(i) {
                return false;
            }
        }
        return true;
    }

    pub fn verify_input(&self, input_idx: usize) -> bool {
        let tx_in = self.tx_ins[input_idx].clone();
        let tx = tx_in.fetch_tx(self.testnet);
        let script_pub_key = tx.clone().tx_outs[tx_in.prev_transaction_index as usize]
            .script_pub_key
            .clone();
        let combined = tx_in.clone().script_sig.clone() + script_pub_key;
        let z = tx.sig_hash(input_idx, self.testnet);
        let z = new_secp256k1scalarelement(z);
        combined.evaluate(z)
    }

    pub fn sign_input(&mut self, input_idx: usize, private_key: Secp256k1PrivateKey) -> bool {
        let z = self.sig_hash(input_idx, self.testnet);
        let z = new_secp256k1scalarelement(z);
        let mut der = private_key.clone().sign(z).der();
        der.append(&mut (Sighash::All as u8).to_le_bytes().to_vec());
        let sig = der;
        let sec = private_key.point.compressed_sec();
        self.tx_ins[input_idx].script_sig = new_script(vec![Cmd::Element(sig), Cmd::Element(sec)]);
        return self.verify_input(input_idx);
    }

    // ref. p139
    // トランザクションの署名ハッシュzを取得する。(署名の検証に利用する)
    // ScriptSigの一部に署名がくっついているので、くっつく前の状態まで復元する
    pub fn sig_hash(&self, input_idx: usize, testnet: bool) -> BigUint {
        let mut result = self.version.to_le_bytes().to_vec();
        result.append(&mut encode_varint(self.clone().tx_ins.len() as u128));

        for (i, tx_in) in self.clone().tx_ins.clone().iter().enumerate() {
            let script_sig = if i == input_idx {
                tx_in.script_pubkey(testnet)
            } else {
                Script { cmds: vec![] }
            };
            result.append(
                &mut TxIn {
                    prev_transaction_id: tx_in.clone().prev_transaction_id,
                    prev_transaction_index: tx_in.clone().prev_transaction_index,
                    script_sig: script_sig,
                    sequence: tx_in.sequence,
                }
                .serialize(),
            );
        }
        result.append(&mut encode_varint((self.clone().tx_outs.len() as u128)));
        for (i, tx_out) in self.clone().tx_outs.clone().iter().enumerate() {
            result.append(&mut tx_out.clone().serialize());
        }
        result.append(&mut self.lock_time.to_le_bytes().to_vec());
        result.append(&mut (Sighash::All as u32).to_le_bytes().to_vec());
        println!("hash mae: {}", u8vec_to_str(result.clone()));
        let h256 = hash256(result);

        return BigUint::from_bytes_be(&*h256);
    }

    pub fn fee(&self) -> BigUint {
        let mut input_sum = BigUint::from(0u8);
        let mut output_sum = BigUint::from(0u8);
        for tx_in in &self.tx_ins {
            input_sum += BigUint::from_u64(tx_in.value(self.testnet)).unwrap();
        }
        for tx_out in &self.tx_outs {
            output_sum += tx_out.amount;
        }
        return input_sum - output_sum;
    }

    fn hash(&self) -> Vec<u8> {
        let mut h = hash256(self.serialize());
        h.reverse();
        return h;
    }

    pub fn id(&self) -> String {
        // 人が読める16進数表記のトランザクションハッシュ
        return u8vec_to_str(self.hash());
    }
    #[allow(dead_code)]
    fn parse_from_vec(testnet: bool, serialization: Vec<u8>) -> Tx {
        let version = vector_as_u8_4_array(serialization[..4].to_vec());
        let version = u32::from_le_bytes(version);
        let tx_ins = vec![];
        let tx_outs = vec![];
        let lock_time = 0u32;
        return Tx {
            version,
            tx_ins,
            tx_outs,
            lock_time,
            testnet,
        };
    }

    pub fn serialize(&self) -> Vec<u8> {
        // version
        let mut result = self.version.to_le_bytes().to_vec();
        result.append(&mut encode_varint(self.clone().tx_ins.len() as u128));

        for tx_in in self.clone().tx_ins.clone().iter() {
            result.append(&mut tx_in.clone().serialize());
        }

        result.append(&mut encode_varint((self.clone().tx_outs.len() as u128)));
        for tx_out in self.clone().tx_outs.clone().iter() {
            result.append(&mut tx_out.clone().serialize());
        }

        result.append(&mut self.lock_time.to_le_bytes().to_vec());
        return result;
    }

    pub fn serialize_str(&self) -> String {
        u8vec_to_str(self.serialize())
    }

    pub fn parse(testnet: bool, c: &mut Cursor<Vec<u8>>) -> Tx {
        let mut version = [0u8; 4];
        let result = c.read(&mut version);
        if result.is_err() {
            panic!("failed to read version")
        }
        let version = u32::from_le_bytes(version);

        let tx_ins_len = read_varint(c);
        let mut tx_ins = vec![];

        for _ in 0..tx_ins_len {
            tx_ins.push(TxIn::parse(c));
        }

        let tx_outs_size = read_varint(c);

        let mut tx_outs = vec![];

        for _ in 0..tx_outs_size {
            tx_outs.push(TxOut::parse(c));
        }

        let mut lock_time = [0u8; 4];
        if c.read(&mut lock_time).is_err() {
            panic!("failed to read lock_time")
        }

        let lock_time = u32::from_le_bytes(lock_time);

        return Tx {
            version,
            tx_ins,
            tx_outs,
            lock_time,
            testnet,
        };
    }
}

impl Display for Tx {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "tx_id: {} \nversion: {}\ntx_ins: \n{}\ntx_outs: \n{}\nlocktime: {}\ntestnet: {}",
            self.clone().id(),
            self.clone().version,
            self.clone()
                .tx_ins
                .into_iter()
                .map(|x| format!(
                    "  id,idx: {}:{}\n  sig: {}\n  serialize: {}",
                    x.prev_transaction_id.to_str_radix(16),
                    x.prev_transaction_index,
                    x.script_sig,
                    u8vec_to_str(x.script_sig.serialize()),
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            self.clone()
                .tx_outs
                .into_iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join("\n"),
            self.lock_time,
            self.testnet
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::helper::helper::decode_hex;

    #[test]
    fn test_parse_p101q5() {
        {
            let s = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e0100"
                .to_owned()
                + "00006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951"
                + "c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0"
                + "da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4"
                + "038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a473044022078"
                + "99531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b84"
                + "61cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba"
                + "1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c35"
                + "6efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da"
                + "6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c3"
                + "4210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49"
                + "abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd"
                + "04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea833"
                + "1ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c"
                + "2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20df"
                + "e7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948"
                + "a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46"
                + "430600";
            let s = decode_hex(&s).unwrap();
            let mut x = Cursor::new(s);
            let tx = Tx::parse(false, &mut x);
            println!("{}", tx);
        }
    }

    #[test]
    fn test_parse_p137() {
        let s = "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303".to_owned()
            + "c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21"
            + "320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615be"
            + "d01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a"
            + "135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001"
            + "976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600";
        let s = decode_hex(&s).unwrap();
        let mut x = Cursor::new(s);
        let tx = Tx::parse(false, &mut x);
        println!("tx1info: \n{}\n ", tx.clone());
        println!("fee: {}", tx.fee())
    }
}
