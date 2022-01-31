use crate::tx::tx::Tx;
use num_bigint::BigUint;
use std::fmt::Write;
use std::io::{Cursor, Read};
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy)]
pub struct TxFetcher {}

#[allow(dead_code)]
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
#[allow(dead_code)]
pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

impl TxFetcher {
    #[allow(dead_code)]
    pub fn get_url(testnet: bool) -> &'static str {
        return if testnet {
            "https://blockstream.info/testnet/api"
        } else {
            "https://blockstream.info/api"
        };
    }

    // TODO: cache
    #[allow(dead_code)]
    pub fn fetch(tx_id: BigUint, testnet: bool) -> Tx {
        let tx_id_str = tx_id.clone().to_str_radix(16);
        // もしかしたら64桁詰めいるかもしれないし、いらないかもしれない・・・
        // アクセス先URLでは先頭が0で詰められていたので、0埋め、64桁、右詰めをしている。
        let tx_id_str = format!("{:0>64}", tx_id_str);
        #[cfg(test)]
        println!("hex: {}", tx_id_str);
        let url = format!("{}/tx/{}/hex", TxFetcher::get_url(testnet), tx_id_str);
        #[cfg(test)]
        println!("url: {}", url.clone());
        let response = reqwest::blocking::get(url);
        if response.is_err() {
            panic!("{}", response.err().unwrap())
        }
        let mut body = String::new();

        if response.unwrap().read_to_string(&mut body).is_err() {
            panic!("failed to read_string (body)")
        }

        // body.pop(); // remove newline(\n)
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
        let id = tx.clone().id();
        if id != tx_id_str {
            panic!("not the same id: {} vs {}", id, tx_id_str);
        }

        return tx;
    }
}
