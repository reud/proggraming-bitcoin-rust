use std::fmt::{Display, Formatter};
use std::fmt;
use crate::tx::helper::{hash256, u8vec_to_str, vector_as_u8_4_array, read_varint};
use std::io::{self, Cursor, Read};
use crate::tx::tx_in::TxIn;
use crate::tx::tx_out::TxOut;

#[derive(Debug,Clone)]
pub struct Tx {
    version: u32,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
    lock_time: u32,
    testnet: bool,
}




impl Tx {
    fn serialize(self) -> Vec<u8> {
        // TBD
        return vec![0u8];
    }

    fn hash(self) -> Vec<u8> {
        return hash256(self.serialize())
    }

    fn id(self) -> String {
        // 人が読める16進数表記のトランザクションハッシュ
        return u8vec_to_str(self.hash());
    }

    fn parse_from_vec(testnet: bool,serialization: Vec<u8>) -> Tx {
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
            testnet
        }
    }

    pub fn parse(testnet: bool, c: &mut Cursor<Vec<u8>>) -> Tx {

        let mut version = [0u8;4];
        c.read(&mut version);
        println!("raw version: {:?}", &version[..]);
        let version = u32::from_le_bytes(version);

        let tx_ins_len = read_varint(c);
        let mut tx_ins = vec![];

        for i in 0..tx_ins_len {
            tx_ins.push(TxIn::parse( c));
        }

        let tx_outs_size = read_varint( c);

        let mut tx_outs = vec![];

        for i in 0..tx_outs_size {
            tx_outs.push(TxOut::parse( c));
        }

        let mut lock_time = [0u8;4];
        c.read(&mut lock_time);
        let lock_time = u32::from_le_bytes(lock_time);
        
        println!("version: {} \ntx_ins_len: {}", version, tx_ins_len);

        return Tx {
            version,
            tx_ins,
            tx_outs,
            lock_time,
            testnet
        }
    }
}

impl Display for Tx {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"tx: {} \nversion: {}\ntx_ins: \n{}\ntx_outs: \n{}\nlocktime: {}\ntestnet: {}"
               ,self.clone().id()
               ,self.clone().version
               ,self.clone().tx_ins.into_iter()
                   .map(|x| format!("  id,idx: {}:{}\n  sig: {}",
                                    u8vec_to_str(x.prev_transaction_id.to_bytes_le()),
                                    x.prev_transaction_index,
                                    u8vec_to_str(x.script_sig)))
                   .collect::<Vec<_>>()
                   .join("\n")
               ,self.clone().tx_outs.into_iter()
                   .map(|x| format!("{}",x))
                   .collect::<Vec<_>>()
                   .join("\n")
               ,self.lock_time
               ,self.testnet)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::tx::helper::decode_hex;

    #[test]
    fn test_parse_p101q5() {
        {
            let s = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e0100".to_owned()
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
            let tx = Tx::parse(false,&mut x);
            println!("{}",tx);
        }
    }
}

