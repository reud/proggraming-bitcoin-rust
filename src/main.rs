#![feature(test)]

use crate::ecc::secp256k1_privatekey::{new_secp_256k1privatekey, Secp256k1PrivateKey};
use crate::ecc::secp256k1_scalar_element::new_secp256k1scalarelement_from_hex_str;
use crate::helper::helper::decode_hex;
use crate::tx::tx_fetcher::TxFetcher;
use crypto_hash::{hex_digest, Algorithm};
use num_bigint::BigUint;
use num_traits::Num;

mod ecc;
mod helper;
mod scripts;
mod tx;

#[allow(dead_code)]
fn fetch_private_key() -> Secp256k1PrivateKey {
    let secret = hex_digest(Algorithm::SHA256, b"reud coin");
    let secret = new_secp256k1scalarelement_from_hex_str(&secret).unwrap();
    let private_key = new_secp_256k1privatekey(secret);
    return private_key;
}

// P148 練習問題4
#[allow(dead_code)]
fn broadcast_testnet_transaction_test() {
    // address = "msvtqXkr2GPAaVDZLdKXaNH1gWk7qtgXX7";
    let private_key = fetch_private_key();
    let target = "mwJn1YPMq7y5F8J3LkC5Hxg9PHyZ5K4cFv";

    let faucet_tx_id = "081634ef66619620a2953e03b53d341fccff2a6f30b7a1a14a3a35c935508546";
    let faucet_tx_id = BigUint::from_str_radix(faucet_tx_id, 16).unwrap();
    let tx = TxFetcher::fetch(faucet_tx_id, true);
    println!("{}", tx);
}

fn main() {
    broadcast_testnet_transaction_test();
}
