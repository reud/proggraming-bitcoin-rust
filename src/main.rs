#![feature(test)]

use crate::ecc::decode::address_decode_base58;
use crate::ecc::secp256k1_privatekey::{new_secp_256k1privatekey, Secp256k1PrivateKey};
use crate::ecc::secp256k1_scalar_element::{
    new_secp256k1scalarelement, new_secp256k1scalarelement_from_hex_str,
};
use crate::helper::helper::{decode_hex, hash160, hash256, u8vec_to_str};
use crate::scripts::script::{
    new_script, new_script_p2pkh_locking, new_script_p2pkh_unlocking, Cmd, Script,
};
use crate::tx::tx::Tx;
use crate::tx::tx_fetcher::TxFetcher;
use crate::tx::tx_in::TxIn;
use crate::tx::tx_out::TxOut;
use crypto_hash::{hex_digest, Algorithm};
use num_bigint::BigUint;
use num_traits::Num;
use std::ptr::hash;

mod ecc;
mod helper;
mod scripts;
mod tx;

#[allow(dead_code)]
fn fetch_private_key() -> Secp256k1PrivateKey {
    let secret = hash256(b"reud coin".to_vec());
    let secret = BigUint::from_bytes_le(&*secret);
    let secret = new_secp256k1scalarelement(secret);
    let private_key = new_secp_256k1privatekey(secret);
    return private_key;
}

// P148 練習問題4
#[allow(dead_code)]
fn broadcast_testnet_transaction_test() {
    // address = "mpw1fSjdDKX6Qs2FAi8Q6Qqm7TKS296sDK";
    let private_key = fetch_private_key();

    let my_address = private_key.clone().point.compressed_address(true);
    let target = "mwJn1YPMq7y5F8J3LkC5Hxg9PHyZ5K4cFv";

    let faucet_tx_id = "7eac43503b64db16d791b00d5fbf2067d53b3607418f911bac7ffefe4fea7184";
    let faucet_tx_id = BigUint::from_str_radix(faucet_tx_id, 16).unwrap();
    let tx = TxFetcher::fetch(faucet_tx_id.clone(), true);
    let outs = tx.clone().tx_outs;
    let amount = outs[0].amount;
    println!("your output amount: {}", amount);

    let send_amount = amount / 10 * 6;
    let change_amount = (amount - send_amount) * 8 / 10;
    // 残りはrelay fee

    println!("send amount: {}", send_amount);
    println!("change_amount: {}", change_amount);

    let version = 1;
    let lock_time = 0;

    let my_address = address_decode_base58(my_address.to_string()).unwrap();
    let target = address_decode_base58(target.to_string()).unwrap();

    let change_script = new_script_p2pkh_locking(my_address);
    let target_script = new_script_p2pkh_locking(target);

    let target_out = TxOut {
        amount: send_amount,
        script_pub_key: target_script,
    };
    let change_out = TxOut {
        amount: change_amount,
        script_pub_key: change_script,
    };

    let txin = TxIn::new(faucet_tx_id, 0);

    let mut create_tx = Tx::new(
        version,
        vec![txin.clone()].clone(),
        vec![target_out.clone(), change_out.clone()].clone(),
        lock_time,
        true,
    );

    let z = create_tx.sig_hash(0); // 署名するinputについて呼び出す。今回はinputは一個なのでidx: 0
    println!("z: {}", z);
    let z = new_secp256k1scalarelement(z);
    println!("z: {}", z);
    let sig = private_key.clone().sign(z.clone());
    let mut sig = sig.der();
    sig.push(1); // <signature>はDER署名+sighash(01) で表す
    let sec = private_key.point.compressed_sec();
    let script_sig = new_script(vec![Cmd::Element(sig), Cmd::Element(sec)]);

    create_tx.tx_ins[0].script_sig = script_sig;

    println!("generate: \n{}", create_tx);
    println!("verify result: {}", create_tx.verify());
    println!("tx: \n{}", create_tx.serialize_str());
    // 間違っているので直す。 zってどこから取ってくるんだっけ？
}

fn main() {
    broadcast_testnet_transaction_test();
}
