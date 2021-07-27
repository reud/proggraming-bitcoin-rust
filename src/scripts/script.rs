use crate::helper::helper::{encode_varint, read_varint};
use crate::scripts::element::Element;
use num_bigint::{BigInt, Sign};
use std::io::{Cursor, Read};
use std::ops::Add;

pub enum Cmd {
    OperationCode(u8),
    Element(Vec<u8>),
}

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
}

impl Add for Script {
    type Output = Script;

    fn add(self, mut rhs: Script) -> Self::Output {
        let mut cmds = self.cmds;
        cmds.append(&mut rhs.cmds);
        new_script(cmds)
    }
}
