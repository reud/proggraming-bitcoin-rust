use num_bigint::BigUint;
use crate::tx::tx::Tx;
use std::io::{Read, Cursor};
use std::num::ParseIntError;

#[derive(Debug,Clone)]
pub struct TxFetcher {

}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

impl TxFetcher {
    pub fn get_url(testnet: bool) -> &'static str {
        return if testnet {
            "http://testnet.programmingbitcoin.com"
        } else {
            "http://mainnet.programmingbitcoin.com"
        }
    }

    pub fn fetch(tx_id: BigUint, testnet: bool, fresh: bool) -> Tx {
        let url = format!("{}/tx/{}.hex",TxFetcher::get_url(testnet),tx_id);
        let response = reqwest::blocking::get(url);
        if response.is_err() {
            panic!(response.err().unwrap())
        }
        let mut body = String::new();
        response.unwrap().read_to_string(&mut body);
        println!("{}",body.is_empty());
        let hex = decode_hex(&*body).unwrap();

        let mut tx;
        if hex[4] == 0x00 {
            let mut s1 = hex[..4].to_vec();
            let mut s2 = hex[6..].to_vec();
            let mut raw: Vec<u8> = vec![];
            raw.append(&mut s1);
            raw.append(&mut s2);
            let mut cursor = Cursor::new(raw.clone());
            tx = Tx::parse(testnet, &mut cursor);
            let mut s: [u8; 4] = Default::default();
            s.copy_from_slice(&raw[(raw.len() - 4) as usize..]);
            tx.lock_time = u32::from_le_bytes(s);
        } else {
            let mut cursor = Cursor::new(hex.clone());
            tx = Tx::parse(testnet, &mut cursor);
        }



        let tx_id_str = tx_id.to_str_radix(16);
        if tx.id() !=tx_id_str {
            panic!("not the same id: {} vs {}", tx.id(),tx_id_str);
        }

        return tx;
    }
}
