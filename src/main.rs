#![feature(test)]

use crate::ecc::decode::address_decode_base58;
use crate::ecc::secp256k1_privatekey::{new_secp_256k1privatekey, Secp256k1PrivateKey};
use crate::ecc::secp256k1_scalar_element::{
    new_secp256k1scalarelement, new_secp256k1scalarelement_from_hex_str,
};
use crate::helper::helper::decode_hex;
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

    let my_address = "msvtqXkr2GPAaVDZLdKXaNH1gWk7qtgXX7";
    let target = "mwJn1YPMq7y5F8J3LkC5Hxg9PHyZ5K4cFv";

    let faucet_tx_id = "081634ef66619620a2953e03b53d341fccff2a6f30b7a1a14a3a35c935508546";
    let faucet_tx_id = BigUint::from_str_radix(faucet_tx_id, 16).unwrap();
    let tx = TxFetcher::fetch(faucet_tx_id.clone(), true);
    let outs = tx.clone().tx_outs;
    let amount = outs[1].amount;

    let send_amount = amount / 10 * 6;
    let change_amount = amount - send_amount;

    println!("transaction: \n{}", tx);

    let z = tx.sig_hash(1);
    let z = new_secp256k1scalarelement(z);
    let sig = private_key.clone().sign(z.clone());
    let mut sig = sig.der();
    sig.push(1); // <signature>はDER署名+sighash(01) で表す
    let sec = private_key.point.compressed_sec();
    let script_sig = new_script(vec![Cmd::Element(sig), Cmd::Element(sec)]);

    let mut txin = TxIn::new(faucet_tx_id, 1);
    txin.script_sig = script_sig;

    let version = 1;
    let lock_time = 0;
    let change_amount = 77777;

    let my_address = address_decode_base58(my_address.to_string()).unwrap();
    let target = address_decode_base58(target.to_string()).unwrap();

    let change_script = new_script_p2pkh_locking(my_address);
    let target_script = new_script_p2pkh_locking(target);

    let target_out = TxOut {
        amount: send_amount,
        script_pub_key: change_script,
    };
    let change_out = TxOut {
        amount: change_amount,
        script_pub_key: target_script,
    };
    let create_tx = Tx::new(
        version,
        vec![txin],
        vec![target_out, change_out],
        lock_time,
        true,
    );
    println!("tx: \n{}", create_tx.to_string())
    // 間違っているので直す。 zってどこから取ってくるんだっけ？
}

fn main() {
    broadcast_testnet_transaction_test();
}
