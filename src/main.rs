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
use crate::tx::tx::{Sighash, Tx};
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

    let faucet_tx_id = "ae89b93ddf943d5b692181980877c29d248519210a92f4a12c607743e9c6ee31";
    let faucet_tx_id = BigUint::from_str_radix(faucet_tx_id, 16).unwrap();

    let send_amount = 4000;
    let change_amount = 4000;

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
        vec![change_out.clone(), target_out.clone()].clone(),
        lock_time,
        true,
    );

    let z = create_tx.sig_hash(0, true); // 署名するinputについて呼び出す。今回はinputは一個なのでidx: 0
    let z = new_secp256k1scalarelement(z);
    let sig = private_key.clone().sign(z.clone());
    let mut sig = sig.der();
    // <signature>はDER署名+sighash(01) で表す
    sig.append(&mut (Sighash::All as u8).to_be_bytes().to_vec());
    let sec = private_key.clone().point.compressed_sec();
    let script_sig = new_script(vec![Cmd::Element(sig), Cmd::Element(sec)]);

    create_tx.tx_ins[0].script_sig = script_sig.clone();
    println!("create_tx: \n {}", create_tx);
    println!("create_tx_serialized: \n{}", create_tx.serialize_str());
    // private_key
    println!("private_key point: \n {}", private_key.clone().point);
}

// P148 練習問題5
// インプットが2つあるトランザクションの作成
#[allow(dead_code)]
fn broadcast_testnet_transaction_two_input_test() {
    let private_key = fetch_private_key();

    let my_address = "mpw1fSjdDKX6Qs2FAi8Q6Qqm7TKS296sDK".to_string();
    let my_address = address_decode_base58(my_address.to_string()).unwrap();

    let target_address = "mwJn1YPMq7y5F8J3LkC5Hxg9PHyZ5K4cFv".to_string();
    let target_address = address_decode_base58(target_address.to_string()).unwrap();

    let change_script = new_script_p2pkh_locking(my_address);
    let target_script = new_script_p2pkh_locking(target_address);

    let send_amount = 50000;
    let change_amount = 20000;

    let change_out = TxOut{ amount: change_amount, script_pub_key: change_script };
    let target_out = TxOut{ amount: send_amount, script_pub_key: target_script };

    let input_tx1 = "ec9995c4605aa1f30d53d8baa904e09fb3aab26c43d28d8d2477644a5707eec9";
    let input_tx1 = BigUint::from_str_radix(input_tx1,16).unwrap();
    let input_tx1_idx = 1;

    let txin1 = TxIn::new(input_tx1,input_tx1_idx);

    let input_tx2 = "2e0baaaf9285532a5bb4aa4ab0ec758d4a47a4146b625c390f44de5e98328f17";
    let input_tx2 = BigUint::from_str_radix(input_tx2,16).unwrap();
    let input_tx2_idx = 0;

    let txin2 = TxIn::new(input_tx2,input_tx2_idx);

    let version = 1;
    let lock_time = 0;
    let mut create_tx = Tx::new(
        version,
        vec![txin1,txin2],
        vec![change_out,target_out],
        lock_time,
        true,
    );

    let sec = private_key.clone().point.compressed_sec();

    // zの値はどっちもあってそう。kの値を固定してみてトランザクションのチェックを行う
    let z1 = create_tx.sig_hash(0,true);
    let z1 = new_secp256k1scalarelement(z1);
    let sig1 = private_key.clone().sign(z1);
    let mut sig1 = sig1.der();

    sig1.append(&mut (Sighash::All as u8).to_be_bytes().to_vec());
    let script_sig1 = new_script(vec![Cmd::Element(sig1),Cmd::Element(sec.clone())]);
    create_tx.tx_ins[0].script_sig = script_sig1.clone();

    let z2 = create_tx.sig_hash(1,true);
    let z2 = new_secp256k1scalarelement(z2);
    let sig2 = private_key.clone().sign(z2);
    let mut sig2 = sig2.der();

    sig2.append(&mut (Sighash::All as u8).to_be_bytes().to_vec());

    let script_sig2 = new_script(vec![Cmd::Element(sig2),Cmd::Element(sec.clone())]);
    create_tx.tx_ins[1].script_sig = script_sig2.clone();
    println!("create_tx: {}",create_tx.serialize_str());
    if !create_tx.verify() {
        panic!("create_tx.verify() failed");
    }

}


fn main() {
    broadcast_testnet_transaction_two_input_test();
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;
    #[test]
    fn generate_testnet_transaction_two_input() {
        let expected_tx_str_raw = "0100000002c9ee07574a6477248d8dd2436cb2aab39fe004a9bad8530df3a15a60c49599ec010000006a47304402207af5f77e06684c937b513e823518cd545d9cf057be0cf89d015af055c6a4d22302204f18c6a906628a70fe3070a804b878637faab098d2d38474140913518a328ce8012103764e263fa94bb5c54a4898aeb3e22bc025b6c9617f05bda41c9351a874d472ccffffffff178f32985ede440f395c626b14a4474a8d75ecb04aaab45b2a538592afaa0b2e000000006a47304402207af5f77e06684c937b513e823518cd545d9cf057be0cf89d015af055c6a4d223022020f25559aabe32c2b9a20c8d6e82f2db406f0ad628cb69bb457cbbef164e4a0c012103764e263fa94bb5c54a4898aeb3e22bc025b6c9617f05bda41c9351a874d472ccffffffff02204e0000000000001976a9146745ed61a219bc660b8ba7dd7abf2aa3002bd0c688ac50c30000000000001976a914ad346f8eb57dee9a37981716e498120ae80e44f788ac00000000";
        let expected_tx_str = decode_hex(expected_tx_str_raw.clone()).unwrap();
        let mut cursor = Cursor::new(expected_tx_str);
        let expected_tx = Tx::parse(true,&mut cursor);
        // input_tx1,input_idx1 = "ec9995c4605aa1f30d53d8baa904e09fb3aab26c43d28d8d2477644a5707eec9", 1
        // input_tx2,input_idx2 = "2e0baaaf9285532a5bb4aa4ab0ec758d4a47a4146b625c390f44de5e98328f17", 0
        let private_key = fetch_private_key();

        let my_address = "mpw1fSjdDKX6Qs2FAi8Q6Qqm7TKS296sDK".to_string();
        let my_address = address_decode_base58(my_address.to_string()).unwrap();

        let target_address = "mwJn1YPMq7y5F8J3LkC5Hxg9PHyZ5K4cFv".to_string();
        let target_address = address_decode_base58(target_address.to_string()).unwrap();

        let change_script = new_script_p2pkh_locking(my_address);
        let target_script = new_script_p2pkh_locking(target_address);

        let send_amount = 50000;
        let change_amount = 20000;

        let change_out = TxOut{ amount: change_amount, script_pub_key: change_script };
        let target_out = TxOut{ amount: send_amount, script_pub_key: target_script };

        let input_tx1 = "ec9995c4605aa1f30d53d8baa904e09fb3aab26c43d28d8d2477644a5707eec9";
        let input_tx1 = BigUint::from_str_radix(input_tx1,16).unwrap();
        let input_tx1_idx = 1;

        let txin1 = TxIn::new(input_tx1,input_tx1_idx);

        let input_tx2 = "2e0baaaf9285532a5bb4aa4ab0ec758d4a47a4146b625c390f44de5e98328f17";
        let input_tx2 = BigUint::from_str_radix(input_tx2,16).unwrap();
        let input_tx2_idx = 0;

        let txin2 = TxIn::new(input_tx2,input_tx2_idx);

        let version = 1;
        let lock_time = 0;
        let mut create_tx = Tx::new(
            version,
            vec![txin1,txin2],
            vec![change_out,target_out],
            lock_time,
            true,
        );

        let sec = private_key.clone().point.compressed_sec();

        // zの値はどっちもあってそう。kの値を固定してみてトランザクションのチェックを行う
        let z1 = create_tx.sig_hash(0,true);
        let z1 = new_secp256k1scalarelement(z1);
        println!("z_val 0: {}",z1.clone());
        let sig1 = private_key.clone().sign(z1);
        let mut sig1 = sig1.der();

        sig1.append(&mut (Sighash::All as u8).to_be_bytes().to_vec());
        println!("script_sig 0: {}",u8vec_to_str(sig1.clone()));
        let script_sig1 = new_script(vec![Cmd::Element(sig1),Cmd::Element(sec.clone())]);
        create_tx.tx_ins[0].script_sig = script_sig1.clone();

        let z2 = create_tx.sig_hash(1,true);
        let z2 = new_secp256k1scalarelement(z2);
        println!("z_val 1: {}",z2.clone());
        let sig2 = private_key.clone().sign(z2);
        let mut sig2 = sig2.der();

        sig2.append(&mut (Sighash::All as u8).to_be_bytes().to_vec());
        println!("script_sig 1: {}",u8vec_to_str(sig2.clone()));

        let script_sig2 = new_script(vec![Cmd::Element(sig2),Cmd::Element(sec.clone())]);
        create_tx.tx_ins[1].script_sig = script_sig2.clone();

        match  create_tx.test_match_tx(expected_tx){
            Ok(_) => {}
            Err(m) => {
                panic!("[UNMATCH] {}",m);
            }
        };

        assert_eq!(create_tx.clone().serialize_str(),expected_tx_str_raw);

        if !create_tx.verify() {
            panic!("create_tx.verify() failed");
        }
    }

}