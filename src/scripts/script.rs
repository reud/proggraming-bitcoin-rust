use crate::ecc::secp256k1_scalar_element::Secp256k1ScalarElement;
use crate::helper::helper::{encode_varint, read_varint, u8vec_to_str};
use crate::scripts::element::{new_element, new_element_from_bytes, Element};
use crate::scripts::operation::{Operation, OperationCodes, Operations};
use crate::scripts::stack::new_stack;
use crate::scripts::stack::Stack;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::helper;
use std::io::{Cursor, Read};
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum Cmd {
    OperationCode(u8),
    Element(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Script {
    cmds: Vec<Cmd>,
}

pub fn new_empty_script() -> Script {
    return Script { cmds: vec![] };
}

pub fn new_script(cmds: Vec<Cmd>) -> Script {
    Script { cmds }
}

impl Script {
    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn parse(c: &mut Cursor<Vec<u8>>) -> Script {
        let length = read_varint(c);
        let mut cmds: Vec<Cmd> = vec![];
        let mut count = 0;
        while count < length {
            let mut one_byte_buf = [0u8];
            let read_result = c.read(&mut one_byte_buf);
            if read_result.is_err() {
                std::panic::panic_any(read_result.err().unwrap());
            }

            count += 1;
            let current_byte = one_byte_buf[0];

            if current_byte >= 1 && current_byte <= 75 {
                let read_len = current_byte;
                let mut buf: Vec<u8> = vec![0u8; read_len as usize];

                let read_result = c.read(&mut buf);
                if read_result.is_err() {
                    std::panic::panic_any(read_result.err().unwrap());
                }
                cmds.push(Cmd::Element(buf));
                count += read_len as u64;
            } else if current_byte == 76 {
                // 1byteだけ読み込んで決めるその値分の読み込み
                let mut buf = [0u8; 1];
                let read_result = c.read(&mut buf);
                if read_result.is_err() {
                    std::panic::panic_any(read_result.err().unwrap());
                }
                // 読み込むデータの長さ
                let data_length = u8::from_le_bytes(buf);
                let mut buf: Vec<u8> = vec![0u8; data_length as usize];
                let read_result = c.read(&mut buf);
                if read_result.is_err() {
                    std::panic::panic_any(read_result.err().unwrap());
                }
                cmds.push(Cmd::Element(buf));
                count += (data_length + 1) as u64;
            } else if current_byte == 77 {
                // 2byteだけ読み込んで決めるその値分の読み込み
                let mut buf = [0u8; 2];
                let read_result = c.read(&mut buf);
                if read_result.is_err() {
                    std::panic::panic_any(read_result.err().unwrap())
                }
                // 読み込むデータの長さ
                let data_length = u16::from_le_bytes(buf);
                let mut buf = vec![0u8; data_length as usize];
                let read_result = c.read(&mut buf);
                if read_result.is_err() {
                    std::panic::panic_any(read_result.err().unwrap());
                }
                cmds.push(Cmd::Element(buf));
                count += (data_length + 2) as u64;
            } else {
                let op_code = current_byte;
                cmds.push(Cmd::OperationCode(op_code));
            }
        }
        if count != length {
            panic!("parsing script failed");
        }
        return new_script(cmds);
    }
    #[allow(dead_code)]
    pub fn serialize(&self) -> Vec<u8> {
        let result = self.raw_serialize();
        let total = result.len();
        let mut ev = encode_varint(total as u128);
        for el in result.iter() {
            ev.push(*el);
        }
        return ev;
    }
    #[allow(dead_code)]
    pub fn raw_serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for cmd in self.cmds.iter() {
            match &*cmd {
                Cmd::OperationCode(code) => {
                    result.push(*code);
                }
                Cmd::Element(v) => {
                    let len = v.len();
                    if len < 75 {
                        let l = len as u8;
                        for b in l.to_le_bytes().iter() {
                            result.push(*b);
                        }
                    } else if len > 75 && len < 0x100 {
                        let l = u8::to_le_bytes(76 as u8);
                        for b in l.to_vec().iter() {
                            result.push(*b);
                        }
                        let l = u8::to_le_bytes(len as u8);
                        for b in l.to_vec().iter() {
                            result.push(*b);
                        }
                    } else if len > 0x100 && len <= 520 {
                        let l = u8::to_le_bytes(77 as u8);
                        for b in l.to_vec().iter() {
                            result.push(*b);
                        }
                        let l = u16::to_le_bytes(len as u16);
                        for b in l.to_vec().iter() {
                            result.push(*b);
                        }
                    } else {
                        panic!("too long an cmd")
                    }
                    for el in v.iter() {
                        result.push(*el);
                    }
                }
            }
        }
        return result;
    }
    #[allow(dead_code)]
    pub fn evaluate(&self, z: Secp256k1ScalarElement) -> bool {
        let mut now_cmds = self.cmds.clone();
        let mut stack: Stack<Element> = new_stack();
        let mut alt_stack: Stack<Element> = new_stack();
        while now_cmds.len() > 0 {
            let cmd = now_cmds.remove(0);
            match cmd {
                Cmd::OperationCode(code) => {
                    let op = Operations::code_functions(code).unwrap();
                    match op {
                        Operation::NormalOperation(op) => {
                            let operation_result = op(&mut stack);
                            if !operation_result {
                                println!("bad operation. code: {}", code);
                                return false;
                            }
                        }
                        Operation::AdditionalStackOperation(op) => {
                            let operation_result = op(&mut stack, &mut alt_stack);
                            if !operation_result {
                                println!("bad operation. code: {}", code);
                                return false;
                            }
                        }
                        Operation::AdditionalItemOperation(op) => {
                            let operation_result = op(&mut stack, &mut now_cmds);
                            if !operation_result {
                                println!("bad operation. code: {}", code);
                                return false;
                            }
                        }
                        Operation::AdditionalScalarElementOperation(_op) => {
                            // let operation_result = op(&mut stack, z.clone());
                            let operation_result = Operations::op_checksig(&mut stack, z.clone());
                            if !operation_result {
                                println!("bad operation. code: {}", code);
                                return false;
                            }
                        }
                    }
                }
                Cmd::Element(bytes) => {
                    stack.push(new_element_from_bytes(bytes));
                }
            }
        }
        if stack.len() == 0 {
            return false;
        }
        if stack.pop().unwrap() == new_element() {
            return false;
        }
        return true;
    }
}

impl Add for Script {
    type Output = Script;

    fn add(self, mut rhs: Script) -> Self::Output {
        let mut cmds = self.cmds;
        cmds.append(&mut rhs.cmds);
        new_script(cmds)
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Script[{}]",
            self.cmds
                .iter()
                .map(|cmd| match cmd {
                    Cmd::OperationCode(code) => {
                        let method = Operations::code_functions(*code).unwrap();
                        format!("{}, ", helper::helper::str_type_of(&method))
                    }
                    Cmd::Element(el) => {
                        format!("Element<{}>, ", u8vec_to_str(el.clone()))
                    }
                })
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use crate::ecc::secp256k1_point::Secp256k1Point;
    use crate::ecc::secp256k1_privatekey::new_secp_256k1privatekey;
    use crate::ecc::secp256k1_scalar_element::{
        new_secp256k1scalarelement_from_hex_str, new_secp256k1scalarelement_from_i32,
    };
    use crate::helper::helper;
    use crate::helper::helper::hash160;
    use crate::scripts::operation::OperationCodes;

    use crate::scripts::script::Cmd::OperationCode;
    use crypto_hash::{digest, hex_digest, Algorithm};

    #[test]
    fn test_p2pk_script() {
        let z = new_secp256k1scalarelement_from_hex_str(
            "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
        )
        .unwrap();

        let s = "04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
        let sec_pubkey = helper::decode_hex(s).unwrap();
        let _p1 = Secp256k1Point::parse(sec_pubkey.clone());

        let sig = helper::decode_hex("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();

        let pubkey_cmds: Vec<Cmd> = vec![
            Cmd::Element(sec_pubkey),
            Cmd::OperationCode(OperationCodes::OpChecksig as u8),
        ];
        let sig_cmds: Vec<Cmd> = vec![Cmd::Element(sig)];

        let pubkey_script = new_script(pubkey_cmds);
        let sig_script = new_script(sig_cmds);

        let combined_script = sig_script + pubkey_script;
        assert_eq!(combined_script.evaluate(z), true);
    }

    #[test]
    fn test_p2pkh_script() {
        // secret
        let e = new_secp256k1scalarelement_from_i32(12345);

        // 秘密鍵
        let pk = new_secp_256k1privatekey(e);
        // 公開鍵の圧縮sec
        let compressed_public_sec = pk.clone().point.compressed_sec();
        let compressed_public_sec_hashed = hash160(compressed_public_sec.clone());
        // 署名先
        let z = digest(Algorithm::SHA256, b"Programming Bitcoin!");
        let z = hex_digest(Algorithm::SHA256, &*z);
        let z = new_secp256k1scalarelement_from_hex_str(&*z).unwrap();
        let sig = pk.clone().sign(z.clone());
        let mut sig = sig.der();
        sig.push(1); // <signature>はDER署名+sighash(01) で表す
        let script_pub_key_cmds: Vec<Cmd> = vec![
            OperationCode(OperationCodes::OpDup as u8),
            OperationCode(OperationCodes::OpHash160 as u8),
            Cmd::Element(compressed_public_sec_hashed), // <hash>
            OperationCode(OperationCodes::OpEqualverify as u8),
            OperationCode(OperationCodes::OpChecksig as u8),
        ];

        let script_sig_cmds: Vec<Cmd> = vec![
            Cmd::Element(sig),
            Cmd::Element(compressed_public_sec.clone()),
        ];

        let pubkey_script = new_script(script_pub_key_cmds);
        let sig_script = new_script(script_sig_cmds);

        let combined_script = sig_script + pubkey_script;
        assert_eq!(combined_script.evaluate(z.clone()), true);
    }

    #[test]
    fn test_p130_script() {
        let pubkey_script = new_script(vec![
            OperationCode(OperationCodes::Op5 as u8),
            OperationCode(OperationCodes::OpAdd as u8),
            OperationCode(OperationCodes::Op9 as u8),
            OperationCode(OperationCodes::OpEqual as u8),
        ]);

        let sig_script = new_script(vec![OperationCode(OperationCodes::Op4 as u8)]);

        let combined_script = sig_script + pubkey_script;
        assert_eq!(
            combined_script.evaluate(new_secp256k1scalarelement_from_i32(0)),
            true
        );
    }

    #[test]
    fn test_p133q3_script() {
        // question
        let pubkey_script = new_script(vec![
            OperationCode(OperationCodes::OpDup as u8),
            OperationCode(OperationCodes::OpDup as u8),
            OperationCode(OperationCodes::OpMul as u8),
            OperationCode(OperationCodes::OpAdd as u8),
            OperationCode(OperationCodes::Op6 as u8),
            OperationCode(OperationCodes::OpEqual as u8),
        ]);
        // answer
        let sig_script = new_script(vec![OperationCode(OperationCodes::Op2 as u8)]);

        let combined_script = sig_script + pubkey_script;
        assert_eq!(
            combined_script.evaluate(new_secp256k1scalarelement_from_i32(0)),
            true
        );
    }
}
